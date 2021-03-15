#!/bin/sh
set -eou pipefail

export ROOT=/state
export KEY_FILE=$ROOT/seed.key
export ASSETS=$ROOT/assets
export HTTP_PORT=80
export PEER_PORT=12345
# NOTE: the DNS needs to be changed to point to the specific DNS name
export PUBLIC_ADDR="seedling.radicle.xyz:${PEER_PORT}"

ARTIFACT_URL="https://storage.googleapis.com/builds.radicle.xyz/radicle-bins/v{VERSION}/dist"

# Download the binaries
curl -sfLS ARTIFACT_URL/radicle-keyutil -o /usr/bin/radicle-keyutil
curl -sfLS ARTIFACT_URL/radicle-seed-node -o /usr/bin/radicle-seed-node

# Download the systemd service unit
curl -sfLS ARTIFACT_URL/seed.service -o /etc/systemd/system/seed.service

# Download and unpack the assets
curl -sfLS ARTIFACT_URL/assets.tar.gz -o $ROOT/assets.tar.gz
tar -zxvf $ROOT/assets.tar.gz

curl -sfLS ARTIFACT_URL/run-seed.sh -o /usr/bin/run-seed.sh

apt-get install git

systemctl enable seed
systemctl start seed
