#!/bin/sh
set -eou pipefail

ROOT=/state
ARTIFACT_URL="https://storage.googleapis.com/builds.radicle.xyz/radicle-bins/v{VERSION}/dist"

# Download the binaries
curl -sfLS ARTIFACT_URL/radicle-keyutil -o /usr/bin/radicle-keyutil
curl -sfLS ARTIFACT_URL/radicle-seed-node -o /usr/bin/radicle-seed-node

# Download the systemd service unit
curl -sfLS ARTIFACT_URL/seed.service -o /etc/systemd/system/seed.service

# Download and unpack the assets
curl -sfLS ARTIFACT_URL/assets.tar.gz -o $ROOT/assets.tar.gz
tar -zxvf $ROOT/assets.tar.gz

systemctl enable seed
systemctl start seed
