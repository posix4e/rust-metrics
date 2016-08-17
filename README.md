# rust-metrics
[![Linux Status](https://travis-ci.org/posix4e/rust-metrics.svg?branch=master)](https://travis-ci.org/posix4e/rust-metrics)
[![Kanban](https://img.shields.io/github/issues/posix4e/rust-metrics.svg?label=HuBoard)](https://huboard.com/posix4e/rust-metrics#/?repo=[%22huboard%22])

This is a work in progress. 
Metrics are things you can use to safely & directly store metrics with little overhead. Metrics
can be attached to a registry and that registry can be collected across a system. This registry
also provides reporting services. Current reporters include:

- Graphite/Carbon/Whisper
- Console/Syslog/Journald (via stdout)
- Integration with included [prometheues reporter](prometheus_reporter)

Contact us on #rust-metrics on Mozilla IRC.

```rust
fn make_a_bunch_of_metrics_store_them_and_start_sending_them_at_a_regular_interval_to_graphite_or_carbon() {
     let m = StdMeter::new();
     m.mark(100);

     let mut c = StdCounter::new();
     c.inc();

     let mut g = StdGauge::new();
     g.set(1.2);

     let mut hc = HistogramConfig::new();
     hc.max_value(100).precision(1);
     let mut h = Histogram::configured(hc).unwrap();

     h.record(1, 1);

     let r = CarbonReporter::new("test",
                                 "localhost:0".to_string(),
                                 "asd.asdf");
     r.add("meter1", Metric::Meter(m.clone()));
     r.add("counter1", Metric::Counter(c.clone()));
     r.add("gauge1", Metric::Gauge(g.clone()));
     r.add("histogram", Metric::Histogram(h));

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
* **run_docker** This will run the  docker container once it's been built (or download the last one i pushed)
* **start_docker** Use docker_compose  carbon/graphite and clients which send them data
* **webserver_with_carbon** Starts a webserver with a carbon reporter
* **start_carbon_example** Use docker-compose to start graphite/carbon/whisper and hook it up to webserver_with_carbon


## TBD
- [ ] C ffi
- [ ] js ffi
- [ ] Prometheus Reporter Integration
- [ ] Tested in Production

## Development
This crate includes a Docker designed for development.
To work on this crate without **build_docker**:
We use cargo for development.

## License

`rust-metrics` is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT) for details.

Copyright (c) 2016 Alex Newman.
