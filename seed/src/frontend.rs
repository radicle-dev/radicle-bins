use std::net;
use std::time;

use futures::StreamExt as _;
use librad::{peer::PeerId, uri, uri::RadUrn};
use serde::Serialize;
use warp::Filter as _;

use radicle_avatar as avatar;
use radicle_seed as seed;

use avatar::Avatar;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;

use futures::channel::mpsc as chan;

/// Maximum number of disconnected peers to keep around in the state.
const MAX_DISCONNECTED_PEERS: usize = 32;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    name: Option<String>,
    peer_id: PeerId,
    description: Option<String>,
    peers: usize,
    projects: usize,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Event {
    PeerConnected(Peer),
    #[serde(rename_all = "camelCase")]
    PeerDisconnected {
        peer_id: PeerId,
    },
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
    peer_id: PeerId,
    projects: HashMap<RadUrn, seed::Project>,
    peers: HashMap<PeerId, Peer>,
    subs: Vec<tokio::sync::mpsc::UnboundedSender<Event>>,
}

impl State {
    fn info(&self) -> Info {
        Info {
            name: self.name.clone(),
            peer_id: self.peer_id.clone(),
            description: None,
            projects: self.projects.len(),
            peers: self.peers.values().filter(|p| p.is_connected()).count(),
        }
    }

    fn broadcast(&mut self, event: Event) {
        self.subs.retain(|sub| sub.send(event.clone()).is_ok());
    }

    fn project_tracked(&mut self, mut proj: seed::Project) {
        // We don't want any path in this URN, just the project id.
        proj.urn = RadUrn::new(proj.urn.id, proj.urn.proto, uri::Path::default());

        if self
            .projects
            .insert(proj.urn.clone(), proj.clone())
            .is_none()
        {
            self.broadcast(Event::ProjectTracked(Project::from(proj.clone())));
        }
    }

    fn peer_connected(&mut self, peer_id: PeerId, urn: Option<RadUrn>, name: Option<String>) {
        match self.peers.entry(peer_id) {
            Entry::Vacant(entry) => {
                let user = urn.map(|u| User::from(u.clone()).with_name(name));
                let peer = Peer {
                    peer_id,
                    user,
                    state: PeerState::new(),
                };
                entry.insert(peer.clone());

                self.broadcast(Event::PeerConnected(peer.clone()));
            }
            Entry::Occupied(mut entry) => {
                let peer = entry.get_mut();

                match &mut peer.state {
                    PeerState::Connected { connections } => {
                        *connections += 1;
                    }
                    PeerState::Disconnected { .. } => {
                        peer.state = PeerState::new();
                    }
                }
            }
        }
    }

    fn peer_disconnected(&mut self, peer_id: PeerId) {
        if let Entry::Occupied(mut entry) = self.peers.entry(peer_id) {
            let peer = entry.get_mut();

            match &mut peer.state {
                PeerState::Connected { connections } => {
                    if *connections > 1 {
                        *connections -= 1;
                    } else {
                        peer.state = PeerState::Disconnected {
                            since: time::SystemTime::now(),
                        };
                        self.broadcast(Event::PeerDisconnected { peer_id });
                    }
                }
                PeerState::Disconnected { .. } => {}
            }
        }

        // Remove oldest disconnected peer if necessary.
        if self
            .peers
            .values()
            .filter(|p| matches!(p.state, PeerState::Disconnected {..}))
            .count()
            > MAX_DISCONNECTED_PEERS
        {
            if let Some((oldest, _)) = self
                .peers
                .iter()
                .flat_map(|(id, p)| {
                    if let PeerState::Disconnected { since } = p.state {
                        Some((*id, since))
                    } else {
                        None
                    }
                })
                .min_by(|(_, a), (_, b)| a.cmp(b))
            {
                self.peers.remove(&oldest);
            }
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
    Connected { connections: usize },
    #[serde(rename_all = "camelCase")]
    Disconnected { since: time::SystemTime },
}

impl PeerState {
    fn new() -> Self {
        Self::Connected { connections: 1 }
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    urn: RadUrn,
    avatar: Avatar,
    name: Option<String>,
}

impl From<RadUrn> for User {
    fn from(urn: RadUrn) -> Self {
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
    pub urn: RadUrn,
    pub name: String,
    pub description: Option<String>,
    pub maintainers: Vec<User>,
}

impl From<seed::Project> for Project {
    fn from(other: seed::Project) -> Self {
        Self {
            urn: other.urn,
            name: other.name,
            description: other.description,
            maintainers: other.maintainers.into_iter().map(User::from).collect(),
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
            }
            seed::Event::PeerConnected { peer_id, urn, name } => {
                state.peer_connected(peer_id, urn, name);
            }
            seed::Event::PeerDisconnected(peer_id) => {
                state.peer_disconnected(peer_id);
            }
            seed::Event::Listening(_) => {}
        };
    }
}

pub async fn run<A: Into<net::SocketAddr>>(
    name: Option<String>,
    addr: A,
    peer_id: PeerId,
    mut handle: seed::NodeHandle,
    events: chan::Receiver<seed::Event>,
) {
    let public = warp::fs::dir("ui/public");
    let projects = handle.get_projects().await.unwrap();
    let state = Arc::new(Mutex::new(State {
        name,
        peer_id,
        projects: projects.into_iter().map(|p| (p.urn.clone(), p)).collect(),
        peers: HashMap::new(),
        subs: Vec::new(),
    }));

    tokio::task::spawn(fanout(state.clone(), events));

    let projects = warp::path("projects")
        .map({
            let state = state.clone();
            move || state.clone()
        })
        .and_then(projects_handler);

    let peers = warp::path("peers")
        .map({
            let state = state.clone();
            move || state.clone()
        })
        .and_then(peers_handler);

    let app = warp::path("events")
        .and(warp::get())
        .map(move || state.clone())
        .and_then(events_handler);

    warp::serve(app.or(public).or(projects).or(peers))
        .run(addr)
        .await;
}

async fn peers_handler(state: Arc<Mutex<State>>) -> Result<impl warp::Reply, warp::Rejection> {
    let state = state.lock().await;
    let peers = state.peers.clone();

    Ok(warp::reply::json(
        &peers.values().cloned().collect::<Vec<_>>(),
    ))
}

async fn projects_handler(state: Arc<Mutex<State>>) -> Result<impl warp::Reply, warp::Rejection> {
    let state = state.lock().await;
    let projs = state.projects.clone();

    Ok(warp::reply::json(
        &projs
            .values()
            .cloned()
            .map(|p| Project::from(p))
            .collect::<Vec<_>>(),
    ))
}

async fn events_handler(state: Arc<Mutex<State>>) -> Result<impl warp::Reply, warp::Rejection> {
    let receiver = {
        let mut state = state.lock().await;
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let projects = state
            .projects
            .values()
            .map(|p| Project::from(p.clone()))
            .collect();
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
        receiver.map(|e| Ok::<_, Infallible>(warp::sse::json(e))),
    )))
}
