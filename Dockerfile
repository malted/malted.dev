# syntax=docker/dockerfile:1
FROM rust:latest AS chef
# Installed from crates.io rather than pulled as a third-party image, so the
# build stage only ever runs the official rust image plus a pinned crate.
RUN cargo install cargo-chef --locked --version 0.1.78
WORKDIR /tmp/malted-dev/

# Work out the dependency graph without needing the source
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build
# Build only the dependencies first. This layer is cached (and restored from
# the GHA cache) until Cargo.toml / Cargo.lock change, so a normal push only
# recompiles our own crate instead of all ~500.
COPY --from=planner /tmp/malted-dev/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

# Copy the binary into a new container for a smaller docker image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates

COPY --from=build /tmp/malted-dev/target/release/malted-dev /

RUN mkdir /tmp/malted-dev

USER root

ENV RUST_LOG=info
ENV RUST_BACKTRACE=full

CMD ["/malted-dev"]
