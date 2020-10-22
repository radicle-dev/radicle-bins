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
        seed::Event::PeerConnected { peer_id, urn, name } => Some(Ok(sse::json(json!({
            "type": "peer-connected",
            "id": peer_id.to_string(),
            "urn": urn.map(|u| u.to_string()),
            "name": name.map(|u| u.to_string()),
        })))),
        seed::Event::PeerDisconnected(id) => Some(Ok(sse::json(json!({
            "type": "peer-disconnected",
            "id": id.to_string()
        })))),
        seed::Event::ProjectTracked {
            urn,
            name,
            description,
            maintainers,
            ..
        } => Some(Ok(sse::json(json!({
            "type": "project-tracked",
            "urn": urn.to_string(),
            "name": name,
            "description": description,
            "maintainers": maintainers
        })))),
    }
}
