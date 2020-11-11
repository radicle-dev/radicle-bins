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

use std::{net, path::PathBuf, process};

use tracing_subscriber::FmtSubscriber;

use librad::{peer::PeerId, uri::RadUrn};
use radicle_seed::{Mode, Node, NodeConfig, Signer};
use radicle_seed_node as seed;

use argh::FromArgs;

#[derive(FromArgs)]
/// Radicle Seed.
pub struct Options {
    /// track the specified peer only
    #[argh(option)]
    pub track_peer: Vec<PeerId>,

    /// track the specified URN only
    #[argh(option)]
    pub track_urn: Vec<RadUrn>,

    /// listen on the following address for peer connections
    #[argh(option)]
    pub peer_listen: Option<net::SocketAddr>,

    /// listen on the following address for HTTP connections (default:
    /// 127.0.0.1:8888)
    #[argh(option, default = "std::net::SocketAddr::from(([127, 0, 0, 1], 8888))")]
    pub http_listen: net::SocketAddr,

    /// log level (default: info)
    #[argh(option, default = "tracing::Level::INFO")]
    pub log: tracing::Level,

    /// radicle root path, for key and git storage
    #[argh(option)]
    pub root: Option<PathBuf>,

    /// name of this seed, displayed to users
    #[argh(option)]
    pub name: Option<String>,

    /// description of this seed, displayed to users as HTML
    #[argh(option)]
    pub description: Option<String>,

    /// public address of this seed node, eg. 'seedling.radicle.xyz:12345'
    #[argh(option)]
    pub public_addr: Option<String>,
}

impl Options {
    pub fn from_env() -> Self {
        argh::from_env()
    }
}

#[tokio::main]
async fn main() {
    let opts = Options::from_env();
    let subscriber = FmtSubscriber::builder().with_max_level(opts.log).finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting tracing subscriber should succeed");

    let signer = match Signer::new(std::io::stdin()) {
        Ok(signer) => signer,
        Err(err) => panic!("invalid key was supplied to stdin: {}", err),
    };

    if !opts.track_peer.is_empty() && !opts.track_urn.is_empty() {
        println!("--track-peer and --track-urn are mutually exclusive options!");
        process::exit(1);
    }

    let config = NodeConfig {
        listen_addr: opts
            .peer_listen
            .unwrap_or(NodeConfig::default().listen_addr),
        root: opts.root,
        mode: if !opts.track_peer.is_empty() {
            Mode::TrackPeers(opts.track_peer.into_iter().collect())
        } else if !opts.track_urn.is_empty() {
            Mode::TrackUrns(opts.track_urn.into_iter().collect())
        } else {
            Mode::TrackEverything
        },
    };
    let node = Node::new(config, signer).unwrap();
    let handle = node.handle();
    let peer_id = node.peer_id();
    let (tx, rx) = futures::channel::mpsc::channel(1);

    tokio::spawn(seed::frontend::run(
        opts.name,
        opts.description,
        opts.http_listen,
        opts.public_addr,
        peer_id,
        handle,
        rx,
    ));

    node.run(tx).await.unwrap();
}
