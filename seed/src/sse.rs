//! Server-side events for the seed node.

use radicle_seed::Event;
use warp::{sse, Rejection, Reply};

use futures::stream::StreamExt as _;
use std::convert::Infallible;

pub fn seed_events(
    events: futures::channel::mpsc::Receiver<Event>,
) -> Result<impl Reply, Rejection> {
    Ok(sse::reply(
        sse::keep_alive().stream(events.filter_map(to_sse)),
    ))
}

pub async fn to_sse(event: Event) -> Option<Result<impl sse::ServerSentEvent, Infallible>> {
    match event {
        Event::Listening(addr) => Some(Ok((sse::event("listening"), sse::json(addr.to_string())))),
        Event::PeerConnected(id) => Some(Ok((
            sse::event("peer-connected"),
            sse::json(id.to_string()),
        ))),
        Event::PeerDisconnected(id) => Some(Ok((
            sse::event("peer-disconnected"),
            sse::json(id.to_string()),
        ))),
        Event::ProjectTracked(urn, _) => Some(Ok((
            sse::event("project-tracked"),
            sse::json(urn.to_string()),
        ))),
    }
}
