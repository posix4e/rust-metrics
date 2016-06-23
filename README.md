# rust-metrics
[![Linux Status](https://travis-ci.org/posix4e/rust-metrics.svg?branch=master)](https://travis-ci.org/posix4e/rust-metrics)


Metrics are things you can use to safely & directly store metrics with little overhead. Metrics
can be attached to a registry and that registry can be collected across a system. This registry
also provides reporting services. Current reporters include. Contact us on #rust-metrics on mozilla irc.

- [Prometheus](https://prometheus.io/)
- Graphite/Carbon/Whisper
- Console/Syslog/Journald (via stdout)

```rust
fn make_a_bunch_of_metrics_store_them_and_start_sending_them_at_a_regular_interval_to_graphite_or_carbon() {
     let m = StdMeter::new();
     m.mark(100);

     let mut c = StdCounter::new();
     c.inc();

     let mut g = StdGauge::default();
     g.set(1.2);

     let mut hc = HistogramConfig::new();
     hc.max_value(100).precision(1);
     let mut h = Histogram::configured(hc).unwrap();

     h.record(1, 1);

     let mut r = StdRegistry::new();
     r.insert("meter1", m);
     r.insert("counter1", c);
     r.insert("gauge1", g);
     r.insert("histogram", h);

     let arc_registry = Arc::new(r);
     CarbonReporter::new(arc_registry.clone(),
                         "test",
                         "localhost:0".to_string(),
                         "asd.asdf");
```

## Usage


Add this to your `Cargo.toml`:

```toml
"metrics" = "*"
```

And add this to your crate root:

```rust
extern crate metrics
```
## Provided scripts in bin/

* **build_docker** This builds the default docker image
* **generate_pb** Generates the prometheus protocol buffer code
* **run_docker** This will run the  docker container once it's been built (or download the last one i pushed)
* **start_docker** Use docker_compose  to launch prometheus, carbon/graphite and clients which send them data
* **webserver_with_prometheus** Starts a webserver which runs with a prometheus reporter
* **start_prometheus_example** Use docker-compose to start a prometheus server & hook it up to webserver_with_prometheus
* **webserver_with_carbon** Starts a webserver with a carbon reporter
* **start_carbon_example** Use docker-compose to start graphite/carbon/whisper and hook it up to webserver_with_carbon

## TBD

- [ ] C ffi
- [ ] js ffi
- [ ] Prometheus enabled
- [ ] Make prometheus optional
- [ ] Tested in Production

## Development

To work on this crate without **build_docker**:

- Install protobuf for `protoc` binary:

    On OS X [Homebrew](https://github.com/Homebrew/homebrew) can be used:

    ```
    brew install protobuf
    ```

    On Ubuntu, `protobuf-compiler` package can be installed:

    ```
    apt-get install protobuf-compiler
    ```
- Install the rust `protoc` plugin:

    ```
    cargo install protobuf
    ```

    and make sure the resulting binary in `$HOME/.cargo/bin` is in your path.

- Run this script to generate the protobuf `.rs` files:

    ```
    ./bin/generate_pb
    ```

- Then you should be able to use cargo:

    ```
    cargo build # to build the code
    cargo test # to run the tests
    ```

## License

`rust-metrics` is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT) for details.

Copyright (c) 2016 Alex Newman.
