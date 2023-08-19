FROM python:3.11-slim-bookworm

RUN set -eux; \
  apt-get update; \
  apt-get install -y --no-install-recommends \
  rustc \
  ; \
  rm -rf /var/lib/apt/lists/*

WORKDIR /app
EXPOSE 8888
