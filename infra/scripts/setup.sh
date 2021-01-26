#!/bin/sh
set -eou pipefail

ROOT=/state
ARTIFACT_URL="https://storage.googleapis.com/builds.radicle.xyz/radicle-bins/master/${COMMIT}"

# Download the binaries
curl ARTIFACT_URL/dist/radicle-keyutil -o /usr/bin/radicle-keyutil
curl ARTIFACT_URL/dist/radicle-seed-node -o /usr/bin/radicle-seed-node

# Download the systemd service unit
curl ARTIFACT_URL/dist/seed.service -o /etc/systemd/system/seed.service

# Download and unpack the assets
curl ARTIFACT_URL/dist/assets.tar.gz -o $ROOT/assets.tar.gz
tar -zxvf $ROOT/assets.tar.gz

systemctl enable seed
systemctl start seed
