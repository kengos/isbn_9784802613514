FROM rust:1.71-slim-bookworm

RUN set -eux; \
  apt-get update; \
  apt-get install -y --no-install-recommends \
  python3 \
  curl \
  ; \
  rm -rf /var/lib/apt/lists/* ;

RUN rustup component add rustfmt;
WORKDIR /app
EXPOSE 8888
