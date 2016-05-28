# rust-metrics
[![Linux Status](https://travis-ci.org/posix4e/rust-metrics.svg?branch=master)](https://travis-ci.org/posix4e/rust-metrics)

Metrics collection for Rust.

## Usage

Add this to your `Cargo.toml`:

```toml
"metrics" = "0.1.1"
```

And add this to your crate root:

```rust
extern crate metrics
```

## Features

- [ ] C library examples
- [x] Gauges
- [x] Counters
- [x] Meters
- [x] Console Based Reporter
- [x] Create a more basic histogram trait and MetricValue
- [x] Histogram support
- [ ] max,mean,sum,stdev support for the histogram
- [ ] PostgreSQL Reporter
- [ ] https://prometheus.io/docs/instrumenting/writing_clientlibs/
- [x] Graphite Reporter
- [ ] Gauge should be made generic
- [ ] Improved testing (Matchers, for the !server macros in the carbon reporter testing)
- [ ] Tested in Production

## License

`rust-metrics` is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT) for details.

Copyright (c) 2015 Alex Newman.
