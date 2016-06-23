/// An example of sending data to a Prometheus server with a local webserver
extern crate iron;
extern crate metrics;
extern crate histogram;

use iron::prelude::*;
use iron::status;
use metrics::metrics::{Counter, Gauge, Meter, StdCounter, StdGauge, StdMeter};
use metrics::registry::StdRegistry;
use metrics::reporter::PrometheusReporter;
use std::sync::Arc;
use std::collections::HashMap;
use histogram::*;

fn main() {
    println!("WebServer Starting");
    extern crate hyper;
    let m = StdMeter::new();
    m.mark(100);

    let mut c = StdCounter::new();
    c.inc();

    let mut g = StdGauge::default();
    g.set(1.2);

    let mut h = Histogram::configure()
        .max_value(100)
        .precision(1)
        .build()
        .unwrap();

    h.increment_by(1, 1).unwrap();

    let mut labels = HashMap::new();
    labels.insert(String::from("test"), String::from("test"));
    let r = StdRegistry::new_with_labels(labels);
    // r.insert("meter1", m);
    // r.insert("counter1", c);
    // r.insert("gauge1", g);
    // r.insert("histogram", h);

    let arc_registry = Arc::new(r);
    let reporter =
        PrometheusReporter::new(arc_registry.clone(), "test", "0.0.0.0:9090", "asd.asdf");
    reporter.start();
    Iron::new(|_: &mut Request| Ok(Response::with(status::NotFound)))
        .http("0.0.0.0:3000")
        .unwrap();
    println!("WebServer Running");

}
