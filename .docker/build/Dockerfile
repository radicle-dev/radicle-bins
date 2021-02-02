FROM debian:buster-slim

# System packages
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        file \
        gcc \
        git \
        libc6-dev \
        liblzma-dev \
        libssl-dev \
        make \
        pkg-config \
        ; \
    apt-get autoremove; \
    rm -rf /var/lib/apt/lists/*

# install npm
RUN curl -sfLS https://deb.nodesource.com/setup_14.x | bash -
RUN apt-get -y update
RUN apt-get -y --no-install-recommends install build-essential nodejs

# install yarn
RUN curl -sfLS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -
RUN echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list
RUN apt-get -y update
RUN apt-get -y --no-install-recommends install yarn

# Rust toolchain
# Make sure this is in sync with rust-toolchain!
ENV RUST_VERSION=nightly-2020-11-10 \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUSTUP_HOME=/usr/local/rustup \
    RUSTUP_VERSION=1.22.1 \
    RUSTUP_SHA256=49c96f3f74be82f4752b8bffcf81961dea5e6e94ce1ccba94435f12e871c3bdb

RUN set -eux; \
    curl -LOf "https://static.rust-lang.org/rustup/archive/${RUSTUP_VERSION}/x86_64-unknown-linux-gnu/rustup-init"; \
    echo "${RUSTUP_SHA256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version; \
    rustup component add clippy rustfmt; \
    cargo install cargo-deny; \
    rm -rf /usr/local/cargo/registry; \
    rm /usr/local/cargo/.package-cache;

VOLUME /cache
ENV CARGO_HOME=/cache/cargo
