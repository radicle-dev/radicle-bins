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

    radicle-seed-node [[--track-peer <track-peer>] | [--track-urn <track-urn>]]
                      [--peer-listen <peer-listen>] [--http-listen <http-listen>]
                      [--log <log>] [--root <root>] [--name <name>]
                      [--description <description>] [--public-addr <public-addr>]

    To run the seed node, a PKCS8 encoded secret key **must always** be
    provided in STDIN.

    OPTIONS

      --track-peer      track the specified peer only
      --track-urn       track the specified URN only
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


    EXAMPLES

    Start a seed node that tracks and replicates specific peers

        $ radicle-seed-node --root ~/.radicle-seed \
                            --track-peer hyymwdkgymeupidbgwfb16wp5fg1ojz3ias8c8ijtdeecjo6yxtw3g \
                            --track-peer hydijmyip398ihqejgpouwhfszdmd45dkh7xwd9ewtjmzwp9tb855a \
                            --track-peer hybra8u45w7ahr195sqcw136twqrjg3nknbzxhyd1pncwsr3pwnkc1 \
                            < ~/.radicle-seed/secret.key

        INFO radicle_seed: Initializing tracker with 3 peers..

    Start a seed node that tracks and replicates specific URNs

        $ radicle-seed-node --root ~/.radicle-seed \
                            --track-urn rad:git:hwd1yrebfxd5fu79qh4zejg4kf1xohfg54iqyssf7guds6cp6hkug4iqsmc \
                            --track-urn rad:git:hwd1yref9i4h9certox1dpb5nfruk9gfyyjnjodg63oqak7d3pa6bpy6bmc \
                            < ~/.radicle-seed/secret.key

        Nov 11 18:03:25.650  INFO radicle_seed: Initializing tracker with 2 URNs..

    Start a seed node that tracks and replicates any peer that connects. All
    URNs that any peer gossips will be replicated.

        $ radicle-seed-node --root ~/.radicle-seed \
                            --public-addr "superseed.monadic.xyz:4444" \
                            --name "Radicle seed node" \
                            --description "<p>The dashboard for Radicle's first public seed node.</p>" \
                            < ~/.radicle-seed/secret.key

        Nov 11 18:08:13.660  INFO radicle_seed: Initializing tracker to track everything..
