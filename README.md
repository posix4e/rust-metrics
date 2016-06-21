# rust-metrics
[![Linux Status](https://travis-ci.org/posix4e/rust-metrics.svg?branch=master)](https://travis-ci.org/posix4e/rust-metrics)


Metrics are things you can use to safely & directly store metrics with little overhead. Metrics
can be attached to a registry and that registry can be collected across a system. This registry
also provides reporting services. Current reporters include. Contact us on #rust-metrics on mozilla irc.

- Prometheus
- Graphite/Carbon/Whisper
- Console/Syslog/Journald (via stdout)

```rust
fn make_a_bunch_of_metrics_store_them_and_start_sending_them_at_a_regular_interval_to_graphite_or_carbon() {
     let m = StdMeter::new();
     m.mark(100);
    
     let mut c: StdCounter = StdCounter::new();
     c.inc();
    
     let mut g: StdGauge = StdGauge { value: 0f64 };
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
"metrics" = "0.1.1"
```

And add this to your crate root:

```rust
extern crate metrics
```

## TBD

- [ ] C ffi
- [ ] js ffi
- [ ] Prometheus enabled
- [ ] Tested in Production

## License

`rust-metrics` is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT) for details.

Copyright (c) 2016 Alex Newman.
