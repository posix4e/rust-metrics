FROM ubuntu:xenial
RUN apt-get update && apt-get install curl git perl bash file sudo build-essential vim -y
RUN curl -sf https://static.rust-lang.org/rustup.sh -o rustup.sh
RUN chmod +x rustup.sh; ./rustup.sh --channel=nightly
COPY Cargo.toml /rust-metrics/
COPY src/ /rust-metrics/src/
WORKDIR /rust-metrics/
RUN find /rust-metrics
RUN cargo test
ENTRYPOINT /bin/bash
