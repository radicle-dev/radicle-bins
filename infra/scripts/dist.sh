#!/bin/bash

set -eou pipefail

if [ -d dist ]; then
    rm -rf dist
fi

mkdir dist
tar -zcvf dist/assets.tar.gz ./seed/ui/public
cp target/release/radicle-keyutil dist
cp target/release/radicle-seed-node dist
cp infra/systemd/seed.service dist
cp infra/scripts/setup.sh dist
