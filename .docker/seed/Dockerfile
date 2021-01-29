FROM debian:buster-slim
RUN set -eux; \
    curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | sudo apt-key add -; \
    echo "deb https://dl.yarnpkg.com/debian/ stable main" | sudo tee /etc/apt/sources.list.d/yarn.list; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        git \
        yarn \
        ; \
    apt-get autoremove; \
    rm -rf /var/lib/apt/lists/*
WORKDIR /
ADD radicle-seed-node .
ENTRYPOINT ["./radicle-seed-node"]
