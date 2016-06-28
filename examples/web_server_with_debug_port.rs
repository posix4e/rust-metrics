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
use metrics::registry::{Registry, StdRegistry};
use metrics::reporter::PrometheusReporter;
use std::sync::Arc;
use std::collections::HashMap;
use histogram::*;

fn main() {
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
    let mut r = StdRegistry::new_with_labels(labels);
    r.insert("meter1", Metric::Meter(m.clone()));
    r.insert("counter1", Metric::Counter(c.clone()));
    r.insert("gauge1", Metric::Gauge(g.clone()));
    r.insert("histogram", Metric::Histogram(h));

    let arc_registry = Arc::new(r);
    let reporter =
        PrometheusReporter::new(arc_registry.clone(), "test", "0.0.0.0:9090", "asd.asdf");
    reporter.start();
    Iron::new(|_: &mut Request| Ok(Response::with(status::NotFound)))
        .http("0.0.0.0:3000")
        .unwrap();
    println!("WebServer Running");

}
