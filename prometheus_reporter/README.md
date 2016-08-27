A first class prometheus reporter written in rust. This is still an early work in progress. 
This library is currently bundled with metrics but we intend on breaking it out into its own
project as soon as it is stabilized. Rust metrics uses this library for prometheus integration.
The overarching goal is a library to provide prometheus support to any rust application or library.
We use the model having a seperate system thread for metrics collection.

## Local Development

- Install protobuf for `protoc` binary:

    On OS X [Homebrew](https://github.com/Homebrew/homebrew) can be used:

    ```
    brew install protobuf
    ```

    On Ubuntu, `protobuf-compiler` package can be installed, and a more complete
list of dependencies can be found in the travis file:

    ```
    apt-get install protobuf-compiler
    ```
- Install the rust `protoc` plugin:

    ```
    cargo install protobuf
    ```

    and make sure the resulting binary in `$HOME/.cargo/bin` is in your path.


- Then you should be able to use cargo
 ```
    cargo build # to build the code
    cargo test # to run the tests

## License

`prometheus reporter` is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT) for details.

Copyright (c) 2016 Alex Newman.
