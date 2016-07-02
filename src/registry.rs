// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A registry to store metrics in

#![allow(missing_docs)]

use std::collections::HashMap;
use metrics::Metric;
use reporter::Reporter;

// TODO break out any notion of metrics. Instead we should have a notion of a collector.
// A collector should be able to insert metrics, and a registry should not.
pub trait Registry<'a>: Send + Sync {
    fn add_scheduled_reporter(&mut self, reporter: Box<Reporter>);
    fn get(&'a self, name: &'a str) -> &'a Metric;
    fn get_metrics_names(&self) -> Vec<&str>;
    fn insert(&mut self, name: &'a str, metric: Metric);
    fn labels(&self) -> HashMap<String, String>;
}

#[derive(Default)]
pub struct StdRegistry<'a> {
    metrics: HashMap<&'a str, Metric>,
    reporter: HashMap<&'a str, Box<Reporter>>,
    labels: HashMap<String, String>,
}

// Specific stuff for registry goes here
impl<'a> Registry<'a> for StdRegistry<'a> {
    fn add_scheduled_reporter(&mut self, reporter: Box<Reporter>) {
        let reporter_name = reporter.get_unique_reporter_name();
        self.reporter.insert(reporter_name, reporter);
    }

    fn get(&'a self, name: &'a str) -> &'a Metric {
        &self.metrics[name]
    }

    fn insert(&mut self, name: &'a str, metric: Metric) {
        self.metrics.insert(name, metric);
    }

    fn get_metrics_names(&self) -> Vec<&str> {
        self.metrics.keys().cloned().collect()
    }

    fn labels(&self) -> HashMap<String, String> {
        self.labels.clone()
    }
}

impl<'a> StdRegistry<'a> {
    pub fn new_with_labels(labels: HashMap<String, String>) -> Self {
        StdRegistry {
            metrics: HashMap::new(),
            reporter: HashMap::new(),
            labels: labels,
        }
    }

    pub fn new() -> Self {
        StdRegistry {
            metrics: HashMap::new(),
            reporter: HashMap::new(),
            labels: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use metrics::{Counter, Gauge, Meter, Metric, StdCounter, StdGauge, StdMeter};
    use registry::{Registry, StdRegistry};
    use histogram::Histogram;


    // TODO add labels tests

    #[test]
    fn meter() {
        let mut r = StdRegistry::new();
        let m = StdMeter::new();
        m.mark(100);
        r.insert("meter1", Metric::Meter(m.clone()));
        m.mark(200);
        if let Metric::Meter(ref meter) = *r.get("meter1") {
            assert_eq!(meter.snapshot().count, 300);
        } else {
            panic!("We didn't get a meter back.");
        }
    }

    #[test]
    fn gauge() {
        let mut r = StdRegistry::new();
        let g = StdGauge::new();
        g.set(3);
        r.insert("gauge1", Metric::Gauge(g.clone()));
        g.set(5);
        if let Metric::Gauge(ref gauge) = *r.get("gauge1") {
            assert_eq!(gauge.snapshot().value, 5);
        } else {
            panic!("We didn't get a gauge back.");
        }
    }

    #[test]
    fn counter() {
        let mut r = StdRegistry::new();
        let c = StdCounter::new();
        c.add(1);
        r.insert("counter1", Metric::Counter(c.clone()));
        c.inc();
        if let Metric::Counter(ref counter) = *r.get("counter1") {
            assert_eq!(counter.snapshot().value, 2);
        } else {
            panic!("We didn't get a counter back.");
        }
    }

    #[test]
    fn histogram() {
        let mut r = StdRegistry::new();
        let mut h = Histogram::configure()
            .max_value(100)
            .precision(1)
            .build()
            .unwrap();
        h.increment_by(1, 1).unwrap();
        r.insert("histogram", Metric::Histogram(h));
    }
}
