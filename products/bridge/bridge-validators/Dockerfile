FROM rust:1.75.0-slim-bullseye as builder

RUN apt update -y && \
    apt upgrade -y

RUN apt autoremove

RUN mkdir build

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/target \
    cargo build && \
    mv /target/debug/bridge-validator /build/


FROM gcr.io/distroless/cc-debian11

COPY --from=builder /build/bridge-validator /bridge-validator
