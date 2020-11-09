# Radicle Seed Node

*Seed node daemon.*

## Build and run from source

Build the UI:

    (cd ui && yarn && yarn build)

Generate a secret key:

    mkdir -p ~/.radicle-seed
    cargo run -p radicle-keyutil -- --filename ~/.radicle-seed/secret.key

Start the node:

    cargo run --bin radicle-seed-node -- \
      --root ~/.radicle-seed \
      --name "seedling" \
      < ~/.radicle-seed/secret.key

To see the seed dashboard, point your browser to http://127.0.0.1:8888.

## Usage

    radicle-seed-node [options]

    Radicle Seed. To run the node a PKCS8 encoded secret key must be provided
    as STDIN.

    Options:
      --track-peers     track the specified peers only
      --track-urns      track the specified URNs only
      --peer-listen     listen on the following address for peer connections
      --http-listen     listen on the following address for HTTP connections
                        (default: 127.0.0.1:8888)
      --log             log level (default: info)
      --root            radicle root path, for key and git storage
      --name            name of this seed, displayed to users
      --description     description of this seed, displayed to users as HTML
      --public-addr     public address of this seed node, eg.
                        'seedling.radicle.xyz:12345'
      --help            display usage information
