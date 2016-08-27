FROM ubuntu:xenial
WORKDIR /prometheus-reporter/
RUN apt-get update && apt-get install curl git perl bash file sudo build-essential vim libssl-dev protobuf-compiler -y
RUN curl -sf https://static.rust-lang.org/rustup.sh -o rustup.sh
RUN chmod +x rustup.sh
RUN ./rustup.sh
# This keeps an immutable cached environment

RUN cargo install protobuf
COPY Cargo.toml /prometheus-reporter/
# Cache rust package list
### Just for rust package cacheing!
RUN mkdir -p src; touch src/lib.rs
RUN cargo build --verbose

# Actually move the source in place
RUN rm -rf src
COPY src src/

RUN RUST_BACKTRACE=1 cargo test --verbose  -- --nocapture

ENTRYPOINT env PATH=$PATH:/prometheus-reporter/bin/ /bin/bash
