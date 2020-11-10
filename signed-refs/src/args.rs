use argh::FromArgs;

use librad::{
    git::refs::{Refs, Signed, Unverified},
    peer::PeerId,
};

fn deserialize_refs(refs: &str) -> Result<Refs, String> {
    serde_json::from_str(refs).map_err(|err| err.to_string())
}

fn deserialize_signed(signed: &str) -> Result<Signed<Unverified>, String> {
    serde_json::from_str(signed).map_err(|err| err.to_string())
}

/// Whether we want to get the signing key from the keystore or from stdin
#[derive(FromArgs)]
#[argh(subcommand, name = "sign")]
pub struct Sign {
    /// get the signing key via the keystore. Please choose one between `keystore` and `stdin`.
    #[argh(switch)]
    pub keystore: bool,

    /// get the signing key via stdin. Please choose one between `keystore` and `stdin`.
    #[argh(switch)]
    pub stdin: bool,

    /// the [`Refs]` we are signing
    #[argh(option, from_str_fn(deserialize_refs))]
    pub refs: Refs,
}

/// Verify a `Signed` using the provided `PeerId`.
#[derive(FromArgs)]
#[argh(subcommand, name = "verify")]
pub struct Verify {
    /// the `PeerId` we are verifying with
    #[argh(option)]
    pub peer_id: PeerId,

    /// the `Signed` refs we are verifying
    #[argh(option, from_str_fn(deserialize_signed))]
    pub signed: Signed<Unverified>,
}

/// Which mode we are running in.
#[derive(FromArgs)]
#[argh(subcommand)]
pub enum Mode {
    Verify(Verify),
    Sign(Sign),
}

/// Verify `Signed` or sign `Refs`.
#[derive(FromArgs)]
pub struct Args {
    #[argh(subcommand)]
    pub mode: Mode,
}
