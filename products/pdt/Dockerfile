# Requires alpine to get openssl and build
FROM rust:1.71.0-bullseye as builder

RUN apt update -y && \
  apt upgrade -y && \
  apt install -y protobuf-compiler build-essential cmake libssl-dev

RUN mkdir build

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build && \
  mv /target/debug/pdt /build/


FROM gcr.io/distroless/cc-debian11

COPY --from=builder /build/pdt /pdt
