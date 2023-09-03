FROM rust:1.71-slim-bookworm

RUN set -eux; \
  apt-get update; \
  apt-get install -y --no-install-recommends \
  python3-full \
  python3-pip \
  curl \
  ; \
  rm -rf /var/lib/apt/lists/* ; \
  rustup component add rust-src ;

RUN rustup component add rustfmt;
WORKDIR /app
EXPOSE 8888
