# Requires alpine to get openssl and build
FROM rust:1.71.0-bullseye as builder

WORKDIR /pdt

RUN apt-get update -y && \
  apt-get upgrade -y && \
  apt-get install -y protobuf-compiler build-essential cmake

RUN mkdir build

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build && \
  mv /pdt/target/debug/pdt /pdt/build/

FROM gcr.io/distroless/cc-debian11:latest

COPY --from=builder /pdt/build/pdt /pdt
