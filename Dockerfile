FROM ubuntu:xenial
RUN apt-get update && apt-get install curl git perl bash file sudo build-essential vim libssl-dev -y
RUN curl -sf https://static.rust-lang.org/rustup.sh -o rustup.sh
RUN chmod +x rustup.sh
RUN ./rustup.sh --channel=nightly

COPY Cargo.toml /rust-metrics/
WORKDIR /rust-metrics/
# Cache rust package list
RUN cargo install gcc
RUN mkdir -p src; touch src/lib.rs
RUN cargo build
RUN rm -rf src

# So now all dependencies should be cached
COPY src/ /rust-metrics/src/
RUN cargo test
COPY examples/ /rust-metrics/examples/
COPY bin/ /rust-metrics/bin/

ENTRYPOINT env PATH=$PATH:/rust-metrics/bin/ /bin/bash
