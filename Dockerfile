FROM ubuntu:xenial
WORKDIR /rust-metrics/
RUN apt-get update && apt-get install curl git perl bash file sudo build-essential vim libssl-dev protobuf-compiler -y
RUN curl -sf https://static.rust-lang.org/rustup.sh -o rustup.sh
RUN chmod +x rustup.sh
RUN ./rustup.sh
# This keeps an immutable cached environment

RUN cargo install protobuf
COPY Cargo.toml /rust-metrics/
COPY prometheus_reporter/Cargo.toml /rust-metrics/prometheus_reporter/
COPY prometheus_reporter/src /rust-metrics/prometheus_reporter/src/

# Cache rust package list
### Just for rust package cacheing!
RUN mkdir -p src; touch src/lib.rs
RUN cargo test --verbose --features prometheus
RUN rm -rf src

# Actually move the source in place
COPY src/ /rust-metrics/src/
RUN touch /rust-metrics/src/*
RUN cargo build --verbose  --features prometheus

COPY examples/ /rust-metrics/examples/
COPY bin/ /rust-metrics/bin/
RUN cargo test --verbose  --features prometheus

ENTRYPOINT env PATH=$PATH:/rust-metrics/bin/ /bin/bash
