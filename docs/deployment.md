
## Requirements

This requires the [Google SDK]().

## Build Process

CI builds the binaries and places them in a `dist` folder at the final step of
the [`pipeline.yaml`](../.buildkite/pipeline.yaml). The contents of the `dist`
folder are:

```
tree dist
dist
├── assets.tar.gz
├── radicle-keyutil
├── radicle-seed-node
├── seed.service
└── setup.sh
```

Each of these will be uploaded the `radicle-bins` bucket in Google Cloud Storage
with the tag version that we are releasing:
`https://storage.googleapis.com/builds.radicle.xyz/radicle-bins/v${VERSION}/dist/*`.

## Creating a New Instance

To create a new instance we use the [`create-instance`](../infra/gcloud/create-instance) script as follows:

```
$ VERSION=X.X.X ./infra/gcloud/create-instance
```

This will create the instance for the version provided. The instance name will
be `seedling-v${VERSION}` and will run the
[`setup.sh`](../infra/scripts/setup.sh) script.

## Setup DNS

TODO(finto): We'll have to set a static IP and link it to a DNS record after the
instance is created. Those steps are unclear to me right now.
