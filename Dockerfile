FROM ubuntu:xenial
RUN apt-get update && apt-get install curl git perl bash file sudo build-essential vim libssl-dev \
 libprotobuf-c0-dev protobuf-compiler libprotobuf-dev libprotoc-dev pkg-config -y
RUN curl -sf https://static.rust-lang.org/rustup.sh -o rustup.sh
RUN chmod +x rustup.sh
RUN ./rustup.sh
# This keeps an immutable cached environment

RUN cargo install protobuf
COPY Cargo.toml /rust-metrics/
COPY protobufs/Cargo.toml protobufs/build.rs /rust-metrics/protobufs/
COPY protobufs/src/ /rust-metrics/protobufs/src/
COPY protobufs/proto/ /rust-metrics/protobufs/proto/

WORKDIR /rust-metrics/
# Cache rust package list
### Just for rust package cacheing!
RUN mkdir -p src; touch src/lib.rs
RUN env RUST_BACKTRACE=1 cargo build --verbose --features "prometheus"
RUN rm -rf src
WORKDIR /

# Actually move the source in place
COPY src/ /rust-metrics/src/
RUN touch /rust-metrics/src/*

WORKDIR /rust-metrics/
RUN env RUST_BACKTRACE=1 cargo build --verbose --features "prometheus"
COPY examples/ /rust-metrics/examples/
COPY bin/ /rust-metrics/bin/
RUN env RUST_BACKTRACE=1 cargo test --features "prometheus"

ENTRYPOINT env PATH=$PATH:/rust-metrics/bin/ /bin/bash
