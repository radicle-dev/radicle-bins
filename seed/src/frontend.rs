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
    collections::{hash_map::Entry, HashMap, HashSet},
    net,
    path::PathBuf,
    sync::Arc,
    time::{self, SystemTime},
};

use futures::StreamExt as _;
use serde::Serialize;
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::ReceiverStream;
use warp::Filter as _;

use avatar::Avatar;
use librad::{git::Urn, net::protocol::event::downstream, peer::PeerId};
use radicle_avatar as avatar;
use radicle_seed as seed;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    name: Option<String>,
    peer_id: PeerId,
    public_addr: Option<String>,
    description: Option<String>,
    homepage: Option<String>,
    logo_url: Option<String>,
    peers: usize,
    projects: usize,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MembershipInfo {
    active: Vec<PeerId>,
    passive: Vec<PeerId>,
}

impl From<downstream::MembershipInfo> for MembershipInfo {
    fn from(i: downstream::MembershipInfo) -> Self {
        Self {
            active: i.active,
            passive: i.passive,
        }
    }
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
    homepage: Option<String>,
    logo_url: Option<String>,
    public_addr: Option<String>,
    peer_id: PeerId,
    projects: HashMap<Urn, Project>,
    featured_projects: HashSet<Urn>,
    peers: HashMap<PeerId, Peer>,
    subs: Vec<mpsc::Sender<Event>>,
}

impl State {
    fn info(&self) -> Info {
        Info {
            name: self.name.clone(),
            public_addr: self.public_addr.clone(),
            peer_id: self.peer_id,
            description: self.description.clone(),
            homepage: self.homepage.clone(),
            logo_url: self.logo_url.clone(),
            projects: self.projects.len(),
            peers: self.peers.values().filter(|p| p.is_connected()).count(),
        }
    }

    fn broadcast(&mut self, event: Event) {
        self.subs.retain(|sub| match sub.try_send(event.clone()) {
            Ok(_) => true,
            Err(mpsc::error::TrySendError::Full(_)) => {
                tracing::warn!("subscription is full");
                true
            },
            // Remove if the other end is gone.
            Err(mpsc::error::TrySendError::Closed(_)) => false,
        });
    }

    fn project_tracked(&mut self, mut proj: seed::Project) {
        // We don't want any path in this URN, just the project id.
        proj.urn = proj.urn.with_path(None);

        let tracked = time::SystemTime::now();

        match self.projects.entry(proj.urn.clone()) {
            Entry::Vacant(v) => {
                let featured = self.featured_projects.contains(&proj.urn);
                let proj = Project::from((proj, Some(tracked), featured));
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
        matches!(self.state, PeerState::Connected { .. })
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

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub urn: Urn,
    pub name: String,
    pub description: Option<String>,
    pub maintainers: Vec<User>,
    pub tracked: Option<SystemTime>,
    pub featured: bool,
}

impl From<(seed::Project, Option<SystemTime>, bool)> for Project {
    fn from((other, tracked, featured): (seed::Project, Option<SystemTime>, bool)) -> Self {
        Self {
            urn: other.urn,
            name: other.name,
            description: other.description,
            maintainers: other.maintainers.into_iter().map(User::from).collect(),
            tracked,
            featured,
        }
    }
}

async fn fanout(state: Arc<Mutex<State>>, rx: mpsc::Receiver<seed::Event>) {
    let mut events = ReceiverStream::new(rx);
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

#[derive(Debug)]
struct ServiceUnavailable {
    message: String,
}

impl warp::reject::Reject for ServiceUnavailable {}

#[allow(clippy::too_many_arguments)]
pub async fn run<A: Into<net::SocketAddr>>(
    name: Option<String>,
    description: Option<String>,
    homepage: Option<String>,
    logo_url: Option<String>,
    addr: A,
    public_addr: Option<String>,
    assets_path: PathBuf,
    featured_projects: HashSet<Urn>,
    peer_id: PeerId,
    mut handle: seed::NodeHandle,
    events: mpsc::Receiver<seed::Event>,
) {
    let public = warp::fs::dir(assets_path);
    let projects = handle.get_projects().await.unwrap();
    let state = Arc::new(Mutex::new(State {
        name,
        description,
        homepage,
        logo_url,
        peer_id,
        public_addr,
        projects: projects
            .into_iter()
            .map(|p| {
                let urn = p.urn.clone();
                let p = Project::from((p, None, featured_projects.contains(&urn)));

                (urn, p)
            })
            .collect(),
        featured_projects,
        peers: HashMap::new(),
        subs: Vec::new(),
    }));

    tokio::task::spawn(fanout(state.clone(), events));

    let handle = Arc::new(Mutex::new(handle));

    let membership = {
        let handle = handle.clone();
        warp::path("membership")
            .map(move || handle.clone())
            .and_then(membership_handler)
    };

    let projects = warp::path("projects")
        .and(warp::path::end())
        .map({
            let state = state.clone();
            move || state.clone()
        })
        .and_then(projects_handler);

    let project = warp::path!("projects" / Urn)
        .map({
            let state = state.clone();
            move |urn| (state.clone(), urn)
        })
        .and_then(|(state, urn)| project_handler(state, urn));

    let peers = warp::path("peers")
        .map(move || handle.clone())
        .and_then(peers_handler);

    let info = warp::path("info")
        .map({
            let state = state.clone();
            move || state.clone()
        })
        .and_then(info_handler);

    let app = warp::path("events")
        .and(warp::get())
        .map(move || state.clone())
        .and_then(events_handler);

    warp::serve(
        app.or(public)
            .or(membership)
            .or(project)
            .or(projects)
            .or(peers)
            .or(info)
            .recover(|err: warp::Rejection| async {
                if let Some(err) = err.find::<ServiceUnavailable>() {
                    let json = warp::reply::json(&err.message);
                    Ok(warp::reply::with_status(
                        json,
                        warp::hyper::StatusCode::SERVICE_UNAVAILABLE,
                    ))
                } else {
                    Err(err)
                }
            }),
    )
    .run(addr)
    .await;
}

async fn membership_handler(
    handle: Arc<Mutex<seed::NodeHandle>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut handle = handle.lock().await;
    let info = handle.get_membership().await.map_err(|e| {
        warp::reject::custom(ServiceUnavailable {
            message: e.to_string(),
        })
    })?;

    Ok(warp::reply::json(&MembershipInfo::from(info)))
}

async fn info_handler(state: Arc<Mutex<State>>) -> Result<impl warp::Reply, warp::Rejection> {
    let state = state.lock().await;

    Ok(warp::reply::json(&state.info()))
}

async fn peers_handler(
    handle: Arc<Mutex<seed::NodeHandle>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut handle = handle.lock().await;
    let peers = handle.get_peers().await.map_err(|e| {
        warp::reject::custom(ServiceUnavailable {
            message: e.to_string(),
        })
    })?;
    let peers = peers
        .into_iter()
        .map(|peer_id| Peer {
            peer_id,
            user: None,
            state: PeerState::Connected,
        })
        .collect::<Vec<_>>();

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

async fn project_handler(
    state: Arc<Mutex<State>>,
    urn: Urn,
) -> Result<impl warp::Reply, warp::Rejection> {
    let state = state.lock().await;
    match state.projects.get(&urn) {
        Some(proj) => Ok(warp::reply::json(proj)),
        None => Err(warp::reject()),
    }
}

async fn events_handler(state: Arc<Mutex<State>>) -> Result<impl warp::Reply, warp::Rejection> {
    let receiver = {
        let mut state = state.lock().await;
        // FIXME(xla): Somehow pass this down and have a knob for it.
        // 640 is all you ever need.
        let (tx, rx) = mpsc::channel(640);
        let projects = state.projects.values().cloned().collect();
        let peers = state.peers.values().cloned().collect();
        let info = state.info();

        tx.try_send(Event::Snapshot {
            projects,
            peers,
            info,
        })
        .expect("channel just created, should not be gone or full");
        state.subs.push(tx);

        rx
    };

    Ok(warp::sse::reply(warp::sse::keep_alive().stream(
        ReceiverStream::new(receiver).map(|e| warp::sse::Event::default().json_data(e)),
    )))
}
