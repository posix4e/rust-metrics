// A registry to store metrics in

use std::collections::HashMap;
use metrics::metric::Metric;
use reporter::base::Reporter;

// TODO break out any notion of metrics. Instead we should have a notion of a collector.
// A collector should be able to insert metrics, and a registry should not.
pub trait Registry<'a>: Send + Sync {
    fn add_scheduled_reporter(&mut self, reporter: Box<Reporter>);
    fn get(&'a self, name: &'a str) -> &'a Metric;
    fn get_metrics_names(&self) -> Vec<&str>;
    fn insert<T: Metric + 'a>(&mut self, name: &'a str, metric: T);
    fn labels(&self) -> HashMap<String, String>;
}

#[derive(Default)]
pub struct StdRegistry<'a> {
    metrics: HashMap<&'a str, Box<Metric + 'a>>,
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
        &*self.metrics[name]
    }

    fn insert<T: Metric + 'a>(&mut self, name: &'a str, metric: T) {
        let boxed = Box::new(metric);
        self.metrics.insert(name, boxed);
    }

    fn get_metrics_names(&self) -> Vec<&str> {
        self.metrics.keys().cloned().collect()
    }

    fn labels(&self) -> HashMap<String, String> {
        self.labels.clone()
    }
}

impl<'a> StdRegistry<'a> {
    pub fn new_with_labels(labels: HashMap<String, String>) -> StdRegistry<'a> {
        StdRegistry {
            metrics: HashMap::new(),
            reporter: HashMap::new(),
            labels: labels,
        }
    }

    pub fn new() -> StdRegistry<'a> {
        StdRegistry {
            metrics: HashMap::new(),
            reporter: HashMap::new(),
            labels: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use metrics::meter::{Meter, StdMeter};
    use metrics::counter::{Counter, StdCounter};
    use metrics::gauge::{Gauge, StdGauge};
    use registry::{Registry, StdRegistry};
    use histogram::*;


    // TODO add labels tests

    #[test]
    fn meter() {
        let mut r = StdRegistry::new();
        let m = StdMeter::new();
        m.mark(100);
        r.insert("meter1", m);
    }

    #[test]
    fn gauge() {
        let mut r = StdRegistry::new();
        let mut g: StdGauge = StdGauge { value: 0.0 };
        g.set(1.2);
        r.insert("gauge1", g);
    }

    #[test]
    fn counter() {
        let mut r = StdRegistry::new();
        let mut c: StdCounter = StdCounter::new();
        c.add(1.0);
        r.insert("counter1", c);
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
        r.insert("histogram", h);
    }
}
