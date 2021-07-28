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
    collections::HashSet,
    fs::File,
    io::Read,
    net::{self, SocketAddr, ToSocketAddrs},
    path::PathBuf,
    str::FromStr,
};

use nonempty::NonEmpty;
use tokio::sync::mpsc;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use librad::{
    git::{replication, Urn},
    net::{
        peer,
        protocol::{self, membership},
        Network,
    },
    paths,
    peer::PeerId,
    profile,
};
use radicle_seed::{Mode, Node, NodeConfig, Signer};
use radicle_seed_node as seed;

use argh::FromArgs;

#[derive(FromArgs)]
#[argh(
    subcommand,
    name = "track-everything",
    description = "Track everything"
)]
pub struct TrackEverything {}

/// A set of peers to track
#[derive(FromArgs)]
#[argh(subcommand, name = "track-peers")]
pub struct TrackPeers {
    /// track the specified peer only
    #[argh(option, long = "peer")]
    peers: Vec<PeerId>,
}

/// A set of URNs to track
#[derive(FromArgs)]
#[argh(subcommand, name = "track-urns")]
pub struct TrackUrns {
    /// track the specified URN only
    #[argh(option, long = "urn")]
    urns: Vec<Urn>,
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum Subcommand {
    TrackEverything(TrackEverything),
    TrackUrns(TrackUrns),
    TrackPeers(TrackPeers),
}

#[derive(FromArgs)]
/// Radicle Seed.
pub struct Options {
    /// track the specified peer only
    #[argh(subcommand)]
    pub track: Subcommand,

    /// join this radicle link network by name.
    ///
    /// Leave empty to join the mainnet.
    #[argh(option, default = "Network::Main")]
    pub network: Network,

    /// listen on the following address for peer connections
    #[argh(option)]
    pub peer_listen: Option<net::SocketAddr>,

    /// advertised address for peer connections (eg.
    /// 1.2.3.4:12345,2.3.4.5:12345)
    #[argh(option)]
    pub advertised_address: Option<String>,

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

    /// path to UI assets directory
    #[argh(option, default = "PathBuf::from(\"ui/public\")")]
    pub assets_path: PathBuf,

    /// name of this seed, displayed to users
    #[argh(option)]
    pub name: Option<String>,

    /// description of this seed, displayed to users as HTML
    #[argh(option)]
    pub description: Option<String>,

    /// homepage of this seed, displayed to users as a URL
    #[argh(option)]
    pub homepage: Option<String>,

    /// logo url of this seed, displayed to users as an image
    #[argh(option)]
    pub logo_url: Option<String>,

    /// public address of this seed node, eg. 'seedling.radicle.xyz:12345'
    #[argh(option)]
    pub public_addr: Option<String>,

    /// list of bootstrap peers, eg.
    /// 'f00...@seed1.example.com:12345,bad...@seed2.example.com:12345'
    #[argh(option)]
    pub bootstrap: Option<String>,

    /// list of featured projects, eg.
    /// 'rad:git:abcd1,rad:git:defg2,...'
    #[argh(option)]
    pub featured_projects: Option<String>,

    /// number of [`librad::git::storage::Storage`] instancess to pool for
    /// consumers.
    #[argh(option, default = "num_cpus::get_physical()")]
    pub user_size: usize,

    /// number of [`librad::git::storage::Storage`] instancess to pool for the
    /// protocol.
    #[argh(option, default = "num_cpus::get_physical()")]
    pub protocol_size: usize,

    /// max number of active members to set in [`membership::Params`].
    #[argh(option, default = "membership::Params::default().max_active")]
    pub membership_max_active: usize,

    /// max number of passive members to set in [`membership::Params`].
    #[argh(option, default = "membership::Params::default().max_passive")]
    pub membership_max_passive: usize,

    #[argh(
        option,
        default = "String::from(\"-\")",
        description = "path to the secret key file or - for stdin"
    )]
    pub secret_key: String,
}

impl Options {
    pub fn from_env() -> Self {
        argh::from_env()
    }
}

fn parse_peer_address(address: &str) -> SocketAddr {
    address
        .to_socket_addrs()
        .map(|mut a| a.next())
        .expect("peer address could not be parsed")
        .expect("peer address could not be resolved")
}

fn parse_peer_list(option: String) -> Vec<(PeerId, SocketAddr)> {
    option
        .split(',')
        .map(|entry| entry.splitn(2, '@').collect())
        .into_iter()
        .map(|parts: Vec<&str>| {
            (
                PeerId::from_str(parts[0]).expect("peer id could not be parsed"),
                parse_peer_address(parts[1]),
            )
        })
        .collect()
}

fn parse_address_list(option: String) -> Vec<SocketAddr> {
    option
        .split(',')
        .map(|entry| {
            entry
                .to_socket_addrs()
                .expect("advertised address could not be parsed")
                .next()
                .unwrap()
        })
        .collect()
}

fn parse_urn_list(option: String) -> HashSet<Urn> {
    option
        .split(',')
        .map(|entry| Urn::from_str(entry).unwrap())
        .collect()
}

fn get_secret_key(secret_key_file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut secret_key: Vec<u8> = Default::default();
    if secret_key_file_path == "-" {
        std::io::stdin().read_to_end(&mut secret_key)?;
    } else {
        let mut file = File::open(secret_key_file_path)?;
        file.read_to_end(&mut secret_key)?;
    };
    Ok(secret_key)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Options::from_env();
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env().add_directive(opts.log.into()))
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting tracing subscriber should succeed");

    let secret_key = get_secret_key(&opts.secret_key)?;

    let signer = match Signer::new(&secret_key[..]) {
        Ok(signer) => signer,
        Err(err) => panic!("invalid key was supplied to stdin: {}", err),
    };
    let paths = if let Some(root) = &opts.root {
        paths::Paths::from_root(root).expect("failed to configure paths")
    } else {
        profile::Profile::load()
            .expect("failed to load profile")
            .paths()
            .to_owned()
    };

    let storage = peer::config::Storage {
        protocol: peer::config::ProtocolStorage {
            fetch_slot_wait_timeout: Default::default(),
            pool_size: opts.protocol_size,
        },
        user: peer::config::UserStorage {
            pool_size: opts.user_size,
        },
    };
    let membership = membership::Params {
        max_active: opts.membership_max_active,
        max_passive: opts.membership_max_passive,
        ..membership::Params::default()
    };
    let listen_addr = opts.peer_listen.unwrap_or_else(|| ([0, 0, 0, 0], 0).into());

    let config = NodeConfig {
        bootstrap: opts.bootstrap.map_or_else(Vec::new, parse_peer_list),
        limits: Default::default(),
        mode: match opts.track {
            Subcommand::TrackPeers(TrackPeers { peers, .. }) => {
                Mode::TrackPeers(peers.into_iter().collect())
            },
            Subcommand::TrackUrns(TrackUrns { urns, .. }) => {
                Mode::TrackUrns(urns.into_iter().collect())
            },
            Subcommand::TrackEverything(_) => Mode::TrackEverything,
        },
    };
    let peer_config = peer::Config {
        signer: signer.clone(),
        protocol: protocol::Config {
            fetch: Default::default(),
            paths,
            listen_addr,
            advertised_addrs: opts
                .advertised_address
                .map(|x| NonEmpty::from_vec(parse_address_list(x)).unwrap()),
            membership,
            network: opts.network,
            replication: replication::Config::default(),
            rate_limits: protocol::Quota::default(),
        },
        storage,
    };
    let node = Node::new(config).unwrap();
    let handle = node.handle();
    let peer_id = PeerId::from(signer);
    let (tx, rx) = mpsc::channel(1);
    let featured_projs = opts
        .featured_projects
        .map_or_else(HashSet::new, parse_urn_list);

    let frontend = tokio::spawn(seed::frontend::run(
        opts.name,
        opts.description,
        opts.homepage,
        opts.logo_url,
        opts.http_listen,
        opts.public_addr,
        opts.assets_path,
        featured_projs,
        peer_id,
        handle,
        rx,
    ));
    let node = tokio::spawn(node.run(peer_config, tx));

    tokio::select! {
        res = frontend => res?,
        res = node => res??,
    };

    Ok(())
}
