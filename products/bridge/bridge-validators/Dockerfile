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

FROM gcr.io/distroless/cc-debian11@sha256:ef6b28ea043173c428d9a8b1e673896466d1846112e46267348d73073b18f578

COPY --from=builder /bridge/build/bridge-validator /bridge-validator
