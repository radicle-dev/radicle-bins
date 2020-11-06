use argh::FromArgs;
use librad::keys::SecretKey;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

#[derive(FromArgs)]
/// Utility for generating a private key file for Radicle Seed.
pub struct Options {
    /// filename of the generated key file (default: seed.key)
    #[argh(option)]
    pub filename: Option<PathBuf>,
}

impl Options {
    pub fn from_env() -> Self {
        argh::from_env()
    }
}

fn main() -> std::io::Result<()> {
    let opts = Options::from_env();

    let file_path = opts.filename.unwrap_or_else(|| PathBuf::from("seed.key"));

    if file_path.exists() {
        println!(
            "The file {} already exists, will not overwrite it.",
            file_path.display()
        );
    } else {
        let mut file = File::create(&file_path)?;

        let metadata = file.metadata()?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o600);
        fs::set_permissions(&file_path, permissions)?;

        let secret_key = SecretKey::new();
        file.write_all(&secret_key.as_ref())?;
        println!("Key file generated: {}", file_path.display());
    }

    Ok(())
}
