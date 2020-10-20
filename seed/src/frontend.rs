use std::net;

use radicle_seed::Event;

pub async fn run<A: Into<net::SocketAddr>>(
    addr: A,
    _events: futures::channel::mpsc::Receiver<Event>,
) {
    let public = warp::fs::dir("ui/public");

    warp::serve(public).run(addr).await
}
