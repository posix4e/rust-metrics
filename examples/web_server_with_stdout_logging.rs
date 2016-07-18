/// An example of sending data to a Prometheus server with a local webserver

extern crate iron;
extern crate metrics;
extern crate histogram;

use iron::prelude::*;
use iron::status;
use metrics::metrics::{Counter, Gauge, Meter, Metric, StdCounter, StdGauge, StdMeter};
use metrics::reporter::ConsoleReporter;
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

        let mut reporter = ConsoleReporter::new("test");
        reporter.add(Metric::Meter(m.clone()));
        reporter.add(Metric::Counter(c.clone()));
        reporter.add(Metric::Gauge(g.clone()));
        reporter.add(Metric::Histogram(h));
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
