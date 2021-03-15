FROM debian:buster-slim
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        git \
        ; \
    apt-get autoremove; \
    rm -rf /var/lib/apt/lists/*

WORKDIR /
ADD radicle-seed-node .
ENTRYPOINT ["./radicle-seed-node"]
