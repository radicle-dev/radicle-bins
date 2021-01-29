FROM debian:buster-slim
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        git \
        ; \
# install npm
RUN curl -sfLS https://deb.nodesource.com/setup_14.x | bash -
RUN apt-get -y update
RUN apt-get -y --no-install-recommends install build-essential nodejs

# install yarn
RUN curl -sfLS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -
RUN echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list
RUN apt-get -y update
RUN apt-get -y --no-install-recommends install yarn

RUN apt-get autoremove; \
    rm -rf /var/lib/apt/lists/*

WORKDIR /
ADD radicle-seed-node .
ENTRYPOINT ["./radicle-seed-node"]
