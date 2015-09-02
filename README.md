# rust-metrics

Metrics collection for Rust.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.metrics]

git = "https://github.com/posix4e/rust-metrics.git"
```

And add this to your crate root:

```rust
extern crate metrics
```

## Features

- [x] Gauges
- [x] Counters
- [x] Meters
- [x] Console Based Reporter
- [ ] Ganglia Reporter
- [ ] Graphite Reporter
- [ ] Gauge should be made generic
- [ ] Histogram support

## License

`rust-metrics` is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT) for details.

Copyright (c) 2015 Alex Newman.
