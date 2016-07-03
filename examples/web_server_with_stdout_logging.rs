/// An example of sending data to a Prometheus server with a local webserver
extern crate iron;
extern crate metrics;
extern crate histogram;

use iron::prelude::*;
use iron::status;
use metrics::metrics::{Counter, Gauge, Meter, Metric, StdCounter, StdGauge, StdMeter};
use metrics::registry::{Registry, StdRegistry};
use metrics::reporter::ConsoleReporter;
use std::sync::Arc;
use histogram::*;
use std::thread;

fn main() {
    println!("WebServer Starting");
    extern crate hyper;
    thread::spawn(|| {
        let m = StdMeter::new();
        m.mark(100);

        let c = StdCounter::new();
        c.inc();

        let g = StdGauge::new();
        g.set(1);
        let mut h = Histogram::configure().max_value(100).precision(1).build().unwrap();
        h.increment_by(1, 1).unwrap();


        let mut r = StdRegistry::new();
        r.insert("meter1", Metric::Meter(m.clone()));
        r.insert("counter1", Metric::Counter(c.clone()));
        r.insert("gauge1", Metric::Gauge(g.clone()));
        r.insert("histogram", Metric::Histogram(h));

        let arc_registry = Arc::new(r);
        let reporter = ConsoleReporter::new(arc_registry, "test");
        reporter.start(500);
        loop {
            c.inc()
        }
    });
    Iron::new(|_: &mut Request| Ok(Response::with(status::NotFound)))
        .http("0.0.0.0:3000")
        .unwrap();
    println!("WebServer Running");
}
