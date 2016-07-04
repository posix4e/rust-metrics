FROM ubuntu:xenial
RUN apt-get update && apt-get install curl git perl bash file sudo build-essential vim libssl-dev protobuf-compiler -y
RUN curl -sf https://static.rust-lang.org/rustup.sh -o rustup.sh
RUN chmod +x rustup.sh
RUN ./rustup.sh
# This keeps an immutable cached environment

RUN cargo install protobuf
COPY Cargo.toml /rust-metrics/

WORKDIR /rust-metrics/
# Cache rust package list
### Just for rust package cacheing!
RUN mkdir -p src; touch src/lib.rs
RUN cargo build
RUN rm -rf src
WORKDIR /

WORKDIR /rust-metrics
# Generate protobuf
COPY bin/ /rust-metrics/bin/
COPY metrics.proto  /rust-metrics/metrics.proto
RUN ./bin/generate_pb
WORKDIR /

# Actually move the source in place
COPY src/ /rust-metrics/src/
RUN touch /rust-metrics/src/*

WORKDIR /rust-metrics/
RUN cargo build --verbose --features "prometheus"
COPY examples/ /rust-metrics/examples/
COPY bin/ /rust-metrics/bin/
RUN cargo test --features "prometheus"

ENTRYPOINT env PATH=$PATH:/rust-metrics/bin/ /bin/bash
