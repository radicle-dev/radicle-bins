use std::net;

use futures::StreamExt as _;
use warp::Filter as _;

use radicle_seed::Event;

use std::sync::Arc;
use tokio::sync::Mutex;

use futures::channel::mpsc as chan;

type Subscriptions = Arc<Mutex<Vec<tokio::sync::mpsc::UnboundedSender<Event>>>>;

async fn fanout(subs: Subscriptions, mut events: chan::Receiver<Event>) {
    while let Some(e) = events.next().await {
        tracing::debug!("{:?}", e);

        subs.lock().await.retain(|sub| sub.send(e.clone()).is_ok());
    }
}

pub async fn run<A: Into<net::SocketAddr>>(addr: A, events: chan::Receiver<Event>) {
    let public = warp::fs::dir("ui/public");
    let subs = Arc::new(Mutex::new(Vec::new()));

    tokio::task::spawn(fanout(subs.clone(), events));

    let app = warp::path("events")
        .and(warp::get())
        .map(move || subs.clone())
        .and_then(handler);

    warp::serve(app.or(public)).run(addr).await;
}

async fn handler(subs: Subscriptions) -> Result<impl warp::Reply, warp::Rejection> {
    let receiver = {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        subs.lock().await.push(tx);

        rx
    };

    Ok(warp::sse::reply(
        warp::sse::keep_alive().stream(receiver.filter_map(crate::sse::to_sse)),
    ))
}
