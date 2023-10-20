FROM rust:1.71.0-bullseye as builder

WORKDIR /pdt

RUN apt-get update -y && \
  apt-get upgrade -y && \
  apt-get install -y --no-install-recommends protobuf-compiler=3.12.4-1+deb11u1 build-essential=12.9 cmake=3.18.4-2+deb11u1

RUN mkdir build

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build && \
  mv /pdt/target/debug/pdt /pdt/build/

FROM debian:bullseye-slim

RUN apt-get update -y && \
  apt-get upgrade -y && \
  apt-get install -y --no-install-recommends ca-certificates=20210119 rsync=3.2.3-4+deb11u1 && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/*

COPY --from=builder /pdt/build/pdt /pdt
