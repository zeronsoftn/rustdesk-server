FROM rust:latest as builder

WORKDIR /usr/src/rustdesk-server

COPY .. .

RUN cargo build --release

FROM debian:stable-slim

WORKDIR /root

COPY --from=builder /usr/src/rustdesk-server/target/release/hbbr /root/hbbr
COPY --from=builder /usr/src/rustdesk-server/target/release/hbbs /root/hbbs