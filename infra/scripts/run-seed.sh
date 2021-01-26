#!/bin/sh
set -eou pipefail

export RUST_BACKTRACE=full

ROOT=/state
KEY_FILE=$ROOT/seed.key
ASSETS=$ROOT/assets

if [ ! -d $ROOT ]; then
    echo "The root directory `${ROOT}` does not exist"
    exit 1
fi

if [ ! -d $ASSETS ]; then
    echo "The assets directory `${ASSETS}` does not exist"
    exit 1
fi

if [ -f "$KEY_FILE" ]; then
    echo "Using existing key file"
else
    echo "Generating new key"
    /usr/bin/radicle-keyutil --filename $KEY_FILE
fi

echo "Starting seed node"

/usr/bin/radicle-seed-node \
	--assets-path $ASSETS \
	--http-listen 0.0.0.0:80 \
	--peer-listen 0.0.0.0:12345 \
	--public-addr "seedling.radicle.xyz:12345" \
	--log info \
	--root $ROOT_DIR \
	--name "Seedling 🌱" \
--description "<p class='typo-text-small' style='margin-bottom: 1rem;'>This is the dashboard for Radicle's seed node. It maintains a real-time view of activity on the Radicle network. You can use it to discover new projects a
nd people to add to your social graph. To find a project or peer, copy it's Radicle ID, and paste it into the search bar of your Upstream app.</p>

<p class='typo-text-small' style='margin-bottom: 1rem'>To add this seed node to your Upstream app, follow the steps in the <a class="typo-link" href="https://docs.radicle.xyz/docs/getting-started">Getting Started</a> guide. To start your own seed node, follow the instructions <a class="typo-link" href="https://docs.radicle.xyz/docs/using-radicle/running-a-seed-node">here</a>.</p>" < $KEY_FILE
