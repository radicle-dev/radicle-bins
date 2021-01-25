#!/bin/sh
set -eou pipefail

# TODO: Grab the binaries from buildkite, but how do we determine the URLs?

export ROOT=/state
export KEY_FILE=$ROOT/seed.key
# TODO: How do we get assets?
export ASSETS=/state

systemctl enable seed
systemctl start seed
