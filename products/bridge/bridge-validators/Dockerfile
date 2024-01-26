FROM rust:1.75.0-slim-bullseye as builder

WORKDIR /bridge

RUN apt-get update -y && \
    apt-get upgrade -y && \
    apt-get autoremove && \
    mkdir build

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/target \
    cargo build && \
    mv /bridge/target/debug/bridge-validator /bridge/build/

FROM debian:bullseye-slim

COPY --from=builder /bridge/build/bridge-validator /bridge-validator
