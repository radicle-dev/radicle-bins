#!/bin/sh
set -eou pipefail

mkdir dist
tar -zcvf dist/assets.tar.gz ./seed/ui/public
