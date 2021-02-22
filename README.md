[![Build status](https://badge.buildkite.com/6e86c41a8281d8f6f5a5537d3f499437c31a807d620157a863.svg?branch=master)](https://buildkite.com/monadic/radicle-bins)

# Radicle Bins

Collection of various executables for the [Radicle](https://radicle.xyz) stack.

## Radicle Upstream Compatability

As we are working on this infrastructure there will be some compatability issues
when it comes to `radicle-bins` and `radicle-upstream` which both rely on `radicle-link`.

If you are on Upstream `< v0.2.0` then use the following commit [`f1462b9`](
https://github.com/radicle-dev/radicle-bins/commit/f1462b92a06ef65ec4b65201e9801473a41b4ee3).

## Build

See the [Dockerfile used for CI](./.docker/build/Dockerfile) for any system /
toolchain dependencies. If you have those, `cargo build` should get you going.

### Docker Images

We currently build and publish a (very simple) docker image for the
`radicle-seed-node` executable as part of our CI builds. You can obtain the
pre-built image like so:

```shell
docker pull gcr.io/opensourcecoin/radicle-seed-node
```

## License

Unless otherwise noted, all source code in this repository is licensed under the
[GPLv3](https://www.gnu.org/licenses/gpl-3.0.txt).
