FROM rust:latest as build

# Compile
RUN rm -rf /tmp/malted-dev/

COPY . /tmp/malted-dev/
RUN rm -rf /tmp/malted-dev/target/

WORKDIR /tmp/malted-dev/

RUN cargo build --release

# Copy the binary into a new container for a smaller docker image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl-dev

COPY --from=build /tmp/malted-dev/target/release/malted-dev /
COPY --from=build /tmp/malted-dev/static /static

RUN mkdir /tmp/malted-dev

USER root

ENV RUST_LOG=info
ENV RUST_BACKTRACE=full

CMD ["/malted-dev"]
