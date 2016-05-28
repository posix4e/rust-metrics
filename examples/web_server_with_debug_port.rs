extern crate iron;
extern crate metrics;
extern crate histogram;

use iron::prelude::*;
use iron::status;
use metrics::meter::{Meter, StdMeter};
use metrics::counter::{Counter, StdCounter};
use metrics::gauge::{Gauge, StdGauge};
use metrics::registry::{Registry, StdRegistry};
use metrics::prometheus_reporter::PrometheusReporter;
use std::sync::Arc;
use histogram::*;

fn main() {
    println!("WebServer Starting");
    extern crate hyper;
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
    let reporter = PrometheusReporter::new(arc_registry.clone(),
                        "test",
                        "0.0.0.0:8080",
                        "asd.asdf");
    reporter.start();
    Iron::new(|_: &mut Request| {
        Ok(Response::with(status::NotFound))
    }).http("0.0.0.0:3000").unwrap();
    println!("WebServer Running");

}
