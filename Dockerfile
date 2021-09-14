FROM rust:latest AS reopenconnect-build

RUN \
    cargo install reopenconnect

FROM debian:latest

COPY --from=reopenconnect-build /usr/local/cargo/bin/reopenconnect /usr/local/sbin/reopenconnect
COPY --from=reopenconnect-build /usr/local/cargo/bin/reopenconnect-strap /usr/local/bin/reopenconnect-strap

ENTRYPOINT /usr/local/sbin/reopenconnect