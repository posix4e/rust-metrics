// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// An example of sending data to a Prometheus server with a local webserver

extern crate iron;
extern crate metrics;
extern crate histogram;


use iron::prelude::*;
use iron::status;
use metrics::metrics::{Counter, Gauge, Meter, Metric, StdCounter, StdGauge, StdMeter};
use std::collections::HashMap;
use histogram::Histogram;

#[cfg(not(feature = "prometheus"))]
fn main() {}
#[cfg(feature = "prometheus")]
fn main() {
    use metrics::reporter::PrometheusReporter;
    println!("WebServer Starting");
    extern crate hyper;
    let m = StdMeter::new();
    m.mark(100);

    let c = StdCounter::new();
    c.inc();

    let g = StdGauge::new();
    g.set(32);

    let mut h = Histogram::configure()
        .max_value(100)
        .precision(1)
        .build()
        .unwrap();

    h.increment_by(1, 1).unwrap();

    let mut labels = HashMap::new();
    labels.insert(String::from("test"), String::from("test"));
    let mut reporter =
        PrometheusReporter::new("test", "0.0.0.0:8080");
    reporter.start(1024);
    reporter.add("meter1", Metric::Meter(m.clone()), labels.clone());
    reporter.add("counter1", Metric::Counter(c.clone()), labels.clone());
    reporter.add("gauge1", Metric::Gauge(g.clone()), labels.clone());
    reporter.add("histogram", Metric::Histogram(h), labels.clone());
    Iron::new(|_: &mut Request| Ok(Response::with(status::NotFound)))
        .http("0.0.0.0:3000")
        .unwrap();
    println!("WebServer Running");
}
