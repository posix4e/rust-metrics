/// An example of sending data to a prometheus server with a local webserver
extern crate iron;
extern crate metrics;
extern crate histogram;

use iron::prelude::*;
use iron::status;
use metrics::metrics::counter::*;
use metrics::metrics::gauge::*;
use metrics::metrics::meter::*;
use metrics::registry::StdRegistry;
use metrics::reporter::prometheus::PrometheusReporter;
use std::sync::Arc;
use std::collections::HashMap;
use histogram::*;

fn main() {
    println!("WebServer Starting");
    extern crate hyper;
    let m = StdMeter::new();
    m.mark(100);

    let mut c: StdCounter = StdCounter::new();
    c.inc();

    let mut g: StdGauge = StdGauge { value: 0.0 };
    g.set(1.2);

    let mut hc = HistogramConfig::new();
    hc.max_value(100).precision(1);
    let mut h = Histogram::configured(hc).unwrap();

    h.record(1, 1);

    let mut labels = HashMap::new();
    labels.insert(String::from("test"), String::from("test"));
    let mut r = StdRegistry::new_with_labels(labels);
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
