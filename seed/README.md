# Radicle Seed Node

*Seed node daemon.*

## Usage

Build the UI:

    (cd ui && yarn && yarn build)

Generate a secret key:

    cargo run -p radicle-keyutil -- --filename seed-secret.key

Start the node:

    cargo run --bin radicle-seed-node -- \
      --root ~/.radicle-seed \
      --name "seedling" \
      < ./seed-secret.key

To see the seed dashboard, point your browser to http://127.0.0.1:8888.
