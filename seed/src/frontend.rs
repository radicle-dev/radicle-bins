use std::net;

use futures::StreamExt as _;
use librad::{peer::PeerId, uri::RadUrn};
use serde::ser::SerializeStruct;
use serde::Serialize;
use warp::Filter as _;

use radicle_seed as seed;

use std::collections::HashMap;
use std::convert::{Infallible, TryFrom};
use std::sync::Arc;
use tokio::sync::Mutex;

use futures::channel::mpsc as chan;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Event {
    PeerConnected(Peer),
    PeerDisconnected(PeerId),
    ProjectTracked(Project),
    Snapshot { projects: Vec<Project> },
}

impl TryFrom<seed::Event> for Event {
    type Error = ();

    fn try_from(other: seed::Event) -> Result<Self, ()> {
        match other {
            seed::Event::PeerConnected { peer_id, urn, name } => {
                Ok(Event::PeerConnected(Peer { peer_id, urn, name }))
            }
            seed::Event::PeerDisconnected(id) => Ok(Event::PeerDisconnected(id)),
            seed::Event::ProjectTracked(proj, _) => Ok(Event::ProjectTracked(Project(proj))),
            seed::Event::Listening(_) => Err(()),
        }
    }
}

#[derive(Debug)]
struct State {
    projects: HashMap<RadUrn, seed::Project>,
    peers: HashMap<PeerId, Peer>,
    subs: Vec<tokio::sync::mpsc::UnboundedSender<Event>>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Peer {
    pub peer_id: PeerId,
    pub urn: Option<RadUrn>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project(#[serde(serialize_with = "serialize_project")] pub seed::Project);

fn serialize_project<S>(proj: &seed::Project, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut state = s.serialize_struct("Project", 4)?;

    state.serialize_field("urn", &proj.urn.to_string())?;
    state.serialize_field("name", &proj.name)?;
    state.serialize_field("description", &proj.description)?;
    state.serialize_field("maintainers", &proj.maintainers)?;
    state.end()
}

async fn fanout(state: Arc<Mutex<State>>, mut events: chan::Receiver<seed::Event>) {
    while let Some(e) = events.next().await {
        tracing::debug!("{:?}", e);

        let mut state = state.lock().await;
        if let Ok(event) = Event::try_from(e.clone()) {
            state
                .subs
                .retain(move |sub| sub.send(event.clone()).is_ok());
        }

        match e {
            seed::Event::ProjectTracked(proj, _) => {
                state.projects.insert(proj.urn.clone(), proj.clone());
            }
            seed::Event::PeerConnected { peer_id, urn, name } => {
                state.peers.insert(peer_id, Peer { peer_id, urn, name });
            }
            seed::Event::PeerDisconnected(peer_id) => {
                state.peers.remove(&peer_id);
            }
            seed::Event::Listening(_) => {}
        }
    }
}

pub async fn run<A: Into<net::SocketAddr>>(
    addr: A,
    mut handle: seed::NodeHandle,
    events: chan::Receiver<seed::Event>,
) {
    let public = warp::fs::dir("ui/public");
    let projects = handle.get_projects().await.unwrap();
    let state = Arc::new(Mutex::new(State {
        projects: projects.into_iter().map(|p| (p.urn.clone(), p)).collect(),
        peers: HashMap::new(),
        subs: Vec::new(),
    }));

    tokio::task::spawn(fanout(state.clone(), events));

    let app = warp::path("events")
        .and(warp::get())
        .map(move || state.clone())
        .and_then(handler);

    warp::serve(app.or(public)).run(addr).await;
}

async fn handler(state: Arc<Mutex<State>>) -> Result<impl warp::Reply, warp::Rejection> {
    let receiver = {
        let mut state = state.lock().await;
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let projects = state
            .projects
            .values()
            .map(|p| Project(p.clone()))
            .collect::<Vec<_>>();

        tx.send(Event::Snapshot { projects }).unwrap();
        state.subs.push(tx);

        rx
    };

    Ok(warp::sse::reply(warp::sse::keep_alive().stream(
        receiver.map(|e| Ok::<_, Infallible>(warp::sse::json(e))),
    )))
}
