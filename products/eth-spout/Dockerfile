FROM rust:1.72.0-slim-bullseye as builder

RUN mkdir build

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/target \
    cargo build --bin eth-spout --release && \
    mv /target/release/eth-spout /build/


FROM gcr.io/distroless/cc-debian11

COPY --from=builder /build/eth-spout /eth-spout

ENTRYPOINT [ "/eth-spout" ]
