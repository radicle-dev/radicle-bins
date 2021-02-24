// This file is part of radicle-bins
// <https://github.com/radicle-dev/radicle-bins>
//
// Copyright (C) 2019-2020 The Radicle Team <dev@radicle.xyz>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 3 or
// later as published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::{
    collections::{hash_map::Entry, HashMap},
    convert::Infallible,
    net,
    path::PathBuf,
    sync::Arc,
    time::{self, SystemTime},
};

use futures::{channel::mpsc as chan, StreamExt as _};
use serde::Serialize;
use tokio::sync::Mutex;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::Filter as _;

use avatar::Avatar;
use librad::{git::Urn, peer::PeerId};
use radicle_avatar as avatar;
use radicle_seed as seed;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    name: Option<String>,
    peer_id: PeerId,
    public_addr: Option<String>,
    description: Option<String>,
    peers: usize,
    projects: usize,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Event {
    ProjectTracked(Project),
    #[serde(rename_all = "camelCase")]
    Snapshot {
        projects: Vec<Project>,
        peers: Vec<Peer>,
        info: Info,
    },
}

#[derive(Debug)]
struct State {
    name: Option<String>,
    description: Option<String>,
    public_addr: Option<String>,
    peer_id: PeerId,
    projects: HashMap<Urn, Project>,
    peers: HashMap<PeerId, Peer>,
    subs: Vec<tokio::sync::mpsc::UnboundedSender<Event>>,
}

impl State {
    fn info(&self) -> Info {
        Info {
            name: self.name.clone(),
            public_addr: self.public_addr.clone(),
            peer_id: self.peer_id,
            description: self.description.clone(),
            projects: self.projects.len(),
            peers: self.peers.values().filter(|p| p.is_connected()).count(),
        }
    }

    fn broadcast(&mut self, event: Event) {
        self.subs.retain(|sub| sub.send(event.clone()).is_ok());
    }

    fn project_tracked(&mut self, mut proj: seed::Project) {
        // We don't want any path in this URN, just the project id.
        proj.urn = proj.urn.with_path(None);

        let tracked = time::SystemTime::now();

        match self.projects.entry(proj.urn.clone()) {
            Entry::Vacant(v) => {
                let proj = Project::from((proj, Some(tracked)));
                v.insert(proj.clone());
                self.broadcast(Event::ProjectTracked(proj));
            },
            Entry::Occupied(_) => {},
        }
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Peer {
    pub peer_id: PeerId,
    pub user: Option<User>,
    pub state: PeerState,
}

impl Peer {
    fn is_connected(&self) -> bool {
        matches!(self.state, PeerState::Connected {..})
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum PeerState {
    #[serde(rename_all = "camelCase")]
    Connected,
    #[serde(rename_all = "camelCase")]
    Disconnected { since: time::SystemTime },
}

impl PeerState {
    fn new() -> Self {
        Self::Connected
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    urn: Urn,
    avatar: Avatar,
    name: Option<String>,
}

impl From<Urn> for User {
    fn from(urn: Urn) -> Self {
        let avatar = Avatar::from(&urn.to_string(), avatar::Usage::Identity);

        Self {
            avatar,
            urn,
            name: None,
        }
    }
}

impl User {
    fn with_name(mut self, name: Option<String>) -> Self {
        self.name = name;
        self
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub urn: Urn,
    pub name: String,
    pub description: Option<String>,
    pub maintainers: Vec<User>,
    pub tracked: Option<SystemTime>,
}

impl From<(seed::Project, Option<SystemTime>)> for Project {
    fn from((other, tracked): (seed::Project, Option<SystemTime>)) -> Self {
        Self {
            urn: other.urn,
            name: other.name,
            description: other.description,
            maintainers: other.maintainers.into_iter().map(User::from).collect(),
            tracked,
        }
    }
}

async fn fanout(state: Arc<Mutex<State>>, mut events: chan::Receiver<seed::Event>) {
    while let Some(e) = events.next().await {
        tracing::info!("{:?}", e);

        let mut state = state.lock().await;

        match e {
            seed::Event::ProjectTracked(proj, _) => {
                state.project_tracked(proj);
            },
            // FIXME: potential to report the status of the seed
            seed::Event::Disconnected => {},
            seed::Event::Listening(_) => {},
        };
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn run<A: Into<net::SocketAddr>>(
    name: Option<String>,
    description: Option<String>,
    addr: A,
    public_addr: Option<String>,
    assets_path: PathBuf,
    peer_id: PeerId,
    mut handle: seed::NodeHandle,
    events: chan::Receiver<seed::Event>,
) {
    let public = warp::fs::dir(assets_path);
    let projects = handle.get_projects().await.unwrap();
    let state = Arc::new(Mutex::new(State {
        name,
        description,
        peer_id,
        public_addr,
        projects: projects
            .into_iter()
            .map(|p| (p.urn.clone(), Project::from((p, None))))
            .collect(),
        peers: HashMap::new(),
        subs: Vec::new(),
    }));

    tokio::task::spawn(fanout(state.clone(), events));

    let handle = Arc::new(Mutex::new(handle));
    let projects = warp::path("projects")
        .map({
            let state = state.clone();
            move || state.clone()
        })
        .and_then(projects_handler);

    let peers = warp::path("peers")
        .map(move || handle.clone())
        .and_then(peers_handler);

    let app = warp::path("events")
        .and(warp::get())
        .map(move || state.clone())
        .and_then(events_handler);

    warp::serve(app.or(public).or(projects).or(peers))
        .run(addr)
        .await;
}

async fn peers_handler(
    handle: Arc<Mutex<seed::NodeHandle>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut handle = handle.lock().await;
    let peers = handle.get_peers().await.expect("failed to get peer list");

    Ok(warp::reply::json(&peers))
}

async fn projects_handler(state: Arc<Mutex<State>>) -> Result<impl warp::Reply, warp::Rejection> {
    let state = state.lock().await;
    let projs = state.projects.clone();

    Ok(warp::reply::json(
        &projs
            .values()
            .cloned()
            .map(Project::from)
            .collect::<Vec<_>>(),
    ))
}

async fn events_handler(state: Arc<Mutex<State>>) -> Result<impl warp::Reply, warp::Rejection> {
    let receiver = {
        let mut state = state.lock().await;
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let projects = state.projects.values().cloned().collect();
        let peers = state.peers.values().cloned().collect();
        let info = state.info();

        tx.send(Event::Snapshot {
            projects,
            peers,
            info,
        })
        .unwrap();
        state.subs.push(tx);

        rx
    };

    Ok(warp::sse::reply(warp::sse::keep_alive().stream(
        UnboundedReceiverStream::new(receiver).map(|e| Ok::<_, Infallible>(warp::sse::json(e))),
    )))
}
