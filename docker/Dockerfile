FROM rust:latest as builder

WORKDIR /usr/src/rustdesk-server

COPY . .

RUN cargo build --release

FROM debian:stable-slim

WORKDIR /usr/src/rustdesk-server

COPY --from=builder /usr/src/rustdesk-server/target/release .