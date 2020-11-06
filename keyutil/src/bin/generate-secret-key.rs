use argh::FromArgs;
use librad::keys::SecretKey;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

#[derive(FromArgs)]
/// Utility for generating a private key file for Radicle Seed.
pub struct Options {
    /// filename of the generated key file (default: seed.key)
    #[argh(option)]
    pub filename: Option<String>,
}

impl Options {
    pub fn from_env() -> Self {
        argh::from_env()
    }
}

fn main() -> std::io::Result<()> {
    let opts = Options::from_env();

    let filename = opts.filename.unwrap_or("seed.key".to_string());

    if Path::new(&filename).exists() {
        println!(
            "A file named {} already exists, will not overwrite it.",
            filename
        );
    } else {
        let mut file = File::create(&filename)?;

        let metadata = file.metadata()?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o600);
        fs::set_permissions(&filename, permissions)?;

        let secret_key = SecretKey::new();
        file.write_all(&secret_key.as_ref())?;
        println!("Key file generated: {}", filename);
    }

    Ok(())
}
