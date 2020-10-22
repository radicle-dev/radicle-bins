# Radicle Seed Node

*Seed node daemon.*

## Usage

Build the UI:

    cd ui
    yarn
    yarn build

Start the node:

    cargo run < secret-key

You'll need a valid Ed25519 private key to start the node. The key is provided
to the node via standard input.
