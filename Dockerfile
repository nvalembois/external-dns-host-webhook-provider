FROM docker.io/library/rust:1.83.0-alpine3.20 AS build

COPY Cargo.toml Cargo.lock /tmp/
COPY src /tmp/src/

WORKDIR /tmp

RUN set -e && \
  apk add --no-cache musl-dev build-base && \
  cargo build --release

FROM docker.io/library/alpine:3.21.0

COPY --from=build /tmp/target/release/host_webhook_provider /

USER 10000
ENTRYPOINT [ "/host_webhook_provider" ]