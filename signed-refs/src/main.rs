// This file is part of radicle-link
// <https://github.com/radicle-dev/radicle-link>
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

use std::io;

use librad::git::refs::Signed;
use librad::paths::Paths;

use signed_refs::args::{Args, Mode, Sign, Verify};
use signed_refs::signer;

fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();

    match args.mode {
        Mode::Sign(Sign {
            keystore,
            stdin,
            refs,
        }) => {
            if keystore && stdin {
                return Err(anyhow::anyhow!(
                    "Please choose `--keystore` or `--stdin`, but not both"
                ));
            }

            let signer = if keystore {
                let paths = Paths::from_env()?;
                signer::get_signer(paths.keys_dir())?
            } else if stdin {
                signer::from_std_in(io::stdin())?
            } else {
                return Err(anyhow::anyhow!(
                    "Please choose one of `--keystore` or `--stdin`"
                ));
            };

            let signed = refs.sign(&signer)?;
            println!("{}", serde_json::to_string(&signed)?);
        }
        Mode::Verify(Verify { peer_id, signed }) => match Signed::verify(signed, &peer_id) {
            Ok(_) => println!(
                "The signed refs are verified by the provided PeerId `{}`",
                peer_id
            ),
            Err(err) => println!("{} for PeerId `{}`", err, peer_id),
        },
    }

    Ok(())
}
