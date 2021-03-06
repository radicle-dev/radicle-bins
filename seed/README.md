# Radicle Seed Node

*Seed node daemon.*


## Radicle Upstream Compatability

As we are working on this infrastructure there will be some compatability
issues when it comes to `radicle-bins` and `radicle-upstream` which both rely
on `radicle-link`.

⚠️ If you are on Upstream `v0.1.x` then use the following commit
[`f1462b9`][ha]. ⚠️


## Working on the UI

To run the UI with dummy data:

1. `cd radicle-bins/seed/ui`
2. `yarn && yarn dev`
3. Open http://localhost:5000 in your browser.


## Build and run from the seed node from source

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

    radicle-seed-node [--network <network>]
                      [--peer-listen <peer-listen>]
                      [--advertised-address <advertised-address>]
                      [--http-listen <http-listen>]
                      [--log <log>]
                      [--root <root>]
                      [--assets-path <assets-path>]
                      [--name <name>]
                      [--description <description>]
                      [--homepage <homepage>]
                      [--logo-url <logo-url>]
                      [--public-addr <public-addr>]
                      [--bootstrap <bootstrap>]
                      [--featured-projects <featured-projects>]
                      [--user-size <user-size>]
                      [--protocol-size <protocol-size>]
                      [--membership-max-active <membership-max-active>]
                      [--membership-max-passive <membership-max-passive>]
                      [<command>] [<args>]


    To run the seed node, a secret key in the format produced by
    `radicle-keyutil` **must always** be provided in STDIN.

    Options:

      --network         join this radicle link network by name
                        leave empty to join the mainnet

      --peer-listen     listen on the following address for peer connections

      --advertised-address
                        advertised address for peer connections,
                        eg. 1.2.3.4:12345,2.3.4.5:12345

      --http-listen     listen on the following address for HTTP connections,
                        default: 127.0.0.1:8888

      --log             log level, default: info
      --root            radicle root path, for key and git storage
      --assets-path     path to UI assets directory
      --name            name of this seed, displayed to users
      --description     description of this seed, displayed to users as HTML
      --homepage        homepage of this seed, displayed to users as a URL
      --logo-url        logo url of this seed, displayed to users as an image

      --public-addr     public address of this seed node,
                        eg. 'seedling.radicle.xyz:12345'

      --bootstrap       list of bootstrap peers,
                        eg. 'f00...@seed1.example.com:12345,bad...@seed2.example.com:12345'

      --featured-projects
                        list of featured projects,
                        eg. 'rad:git:abcd1,rad:git:defg2,...'

      --user-size       number of [`librad::git::storage::Storage`] instancess
                        to pool for consumers

      --protocol-size   number of [`librad::git::storage::Storage`] instancess
                        to pool for the protocol

      --membership-max-active
                        max number of active members to set in
                        [`membership::Params`]

      --membership-max-passive
                        max number of passive members to set in
                        [`membership::Params`]

      --help            display usage information

    Commands:

      track-urns        A set of URNs to track
      track-peers       A set of peers to track


    Examples:

      Start a seed node that tracks and replicates specific peers

        $ radicle-seed-node --root ~/.radicle-seed track-peers \
                            --peer hyymwdkgymeupidbgwfb16wp5fg1ojz3ias8c8ijtdeecjo6yxtw3g \
                            --peer hydijmyip398ihqejgpouwhfszdmd45dkh7xwd9ewtjmzwp9tb855a \
                            --peer hybra8u45w7ahr195sqcw136twqrjg3nknbzxhyd1pncwsr3pwnkc1 \
                            < ~/.radicle-seed/secret.key

        INFO radicle_seed: Initializing tracker with 3 peers..

      Start a seed node that tracks and replicates specific URNs

        $ radicle-seed-node --root ~/.radicle-seed track-urns \
                            --urn rad:git:hwd1yrebfxd5fu79qh4zejg4kf1xohfg54iqyssf7guds6cp6hkug4iqsmc \
                            --urn rad:git:hwd1yref9i4h9certox1dpb5nfruk9gfyyjnjodg63oqak7d3pa6bpy6bmc \
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


## Systemd integration

To run the seed daemon via systemd, you can use the provided templates in
[systemd](./systemd). To set it up adjust the settings in the templates and
copy them to their respective locations:

    cp systemd/radicle-seed.example /etc/default/radicle-seed
    cp systemd/radicle-seed.service.example /etc/systemd/system/radicle-seed.service

To start or stop the service run:

    systemctl start radicle-seed
    systemctl stop radicle-seed

And to start the seed node on system start do:

    systemctl enable radicle-seed



[ha]: https://github.com/radicle-dev/radicle-bins/commit/f1462b92a06ef65ec4b65201e9801473a41b4ee3