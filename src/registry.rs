use std::collections::HashMap;
use metric::Metric;
use reporter::Reporter;

pub trait Registry<'a>: Send + Sync {
    fn add_scheduled_reporter(&mut self, reporter: Box<Reporter>);
    fn get(&'a self, name: &'a str) -> &'a Metric;
    fn get_metrics_names(&self) -> Vec<&str>;
    fn insert<T: Metric + 'a>(&mut self, name: &'a str, metric: T);
}

pub struct StdRegistry<'a> {
    metrics: HashMap<&'a str, Box<Metric+ 'a>>,
    reporter: HashMap<&'a str, Box<Reporter>>
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
}

impl<'a> StdRegistry<'a> {
    #[allow(dead_code)]
    pub fn new() -> StdRegistry<'a> {
        StdRegistry { metrics: HashMap::new(), reporter: HashMap::new() }
    }
}

#[cfg(test)]
mod test {
    use meter::{Meter, StdMeter};
    use counter::{Counter, StdCounter};
    use gauge::{Gauge, StdGauge};
    use registry::{Registry, StdRegistry};
    use histogram::*;

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
        let mut g: StdGauge = StdGauge { value: 0f64 };
        g.update(1.2);
        r.insert("gauge1", g);
    }

    #[test]
    fn counter() {
        let mut r = StdRegistry::new();
        let mut c: StdCounter = StdCounter::new();
        c.inc(1);
        r.insert("counter1", c);
    }

    #[test]
    fn histogram() {
        let mut r = StdRegistry::new();
        let mut h = Histogram::new(
    HistogramConfig{
        max_memory: 0,
        max_value: 1000000,
        precision: 3,
        }).unwrap();
        h.record(1, 1);
        r.insert("histogram", h);

        let mut r = StdRegistry::new();

    }
}
