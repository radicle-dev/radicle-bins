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

use std::{fs, fs::File, io::prelude::*, os::unix::fs::PermissionsExt, path::PathBuf};

use argh::FromArgs;
use librad::keys::SecretKey;

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
