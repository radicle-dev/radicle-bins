//! Server-side events for the seed node.

use radicle_seed as seed;
use warp::sse;

use std::convert::Infallible;

use serde_json::json;

pub async fn to_sse(event: seed::Event) -> Option<Result<impl sse::ServerSentEvent, Infallible>> {
    match event {
        seed::Event::Listening(addr) => Some(Ok(sse::json(json!({
            "type": "listening",
            "addr": addr.to_string()
        })))),
        seed::Event::PeerConnected(id) => Some(Ok(sse::json(json!({
            "type": "peer-connected",
            "id": id.to_string()
        })))),
        seed::Event::PeerDisconnected(id) => Some(Ok(sse::json(json!({
            "type": "peer-disconnected",
            "id": id.to_string()
        })))),
        seed::Event::ProjectTracked(urn, _) => Some(Ok(sse::json(json!({
            "type": "project-tracked",
            "urn": urn.to_string(),
        })))),
    }
}
