FROM rust:latest AS reopenconnect-build

RUN mkdir /project
COPY src /project/src
COPY Cargo.toml /project/Cargo.toml
COPY Cargo.lock /project/Cargo.lock
WORKDIR /project
RUN \
    cargo build \
        --release \
        --no-default-features \
        --features with-tokio-tun



FROM debian:latest

RUN \
    apt-get update && \
    apt-get install -y \
        ca-certificates \
        iproute2

COPY --from=reopenconnect-build /project/target/release/reopenconnect /usr/local/sbin/reopenconnect
COPY --from=reopenconnect-build /project/target/release/reopenconnect-strap /usr/local/bin/reopenconnect-strap

ENTRYPOINT /usr/local/sbin/reopenconnect