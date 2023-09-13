FROM rust:1.71-slim-bookworm

RUN set -eux; \
  apt-get update; \
  apt-get install -y --no-install-recommends \
  python3-full \
  python3-pip \
  curl \
  libssl-dev \
  pkg-config \
  locales \
  ; \
  rm -rf /var/lib/apt/lists/* ; \
  locale-gen ja_JP.UTF-8 ; \
  localedef -f UTF-8 -i ja_JP ja_JP.UTF-8 ;

RUN rustup component add rustfmt rust-src;
ENV LANG=ja_JP.UTF-8
WORKDIR /app
EXPOSE 8888
