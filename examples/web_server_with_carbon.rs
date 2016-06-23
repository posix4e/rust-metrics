/// An example of sending data to a Prometheus server with a local webserver
extern crate iron;
extern crate metrics;
extern crate histogram;

use iron::prelude::*;
use iron::status;
use metrics::metrics::counter::*;
use metrics::metrics::gauge::*;
use metrics::metrics::meter::*;
use metrics::registry::{Registry, StdRegistry};
use metrics::reporter::CarbonReporter;
use std::sync::Arc;
use histogram::*;
use std::thread;

fn main() {
    println!("WebServer Starting");
    extern crate hyper;
    thread::spawn(|| {
        let m = StdMeter::new();
        m.mark(100);

        let mut c: StdCounter = StdCounter::new();
        c.inc();

        let mut g: StdGauge = StdGauge { value: 0.0 };
        g.set(1.2);

        let mut h = Histogram::configure()
                    .max_value(100)
                    .precision(1)
                    .build()
                    .unwrap();

        h.increment_by(1, 1).unwrap();

        let mut r = StdRegistry::new();
        r.insert("meter1", m);
        r.insert("counter1", c);
        r.insert("gauge1", g);
        r.insert("histogram", h);

        let arc_registry = Arc::new(r);
        let reporter = CarbonReporter::new(arc_registry.clone(),
                                           "test",
                                           "carbon_graphite:2003".to_string(),
                                           "asd.asdf");
        reporter.start(5);
        loop {
            c.inc()
        }
    });
    Iron::new(|_: &mut Request| Ok(Response::with(status::NotFound)))
        .http("0.0.0.0:3000")
        .unwrap();
    println!("WebServer Running");
}
