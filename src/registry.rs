use std::collections::HashMap;

use metric::Metric;
use reporter::Reporter;


pub trait Registry<'a> {
    fn get(&'a self, name: &'a str) -> &'a Metric;
    fn insert<T: Metric + 'a>(&mut self, name: &'a str, metric: T);
    fn add_scheduled_reporter(&mut self, reporter: Box<Reporter>);
}

pub struct StdRegistry<'a> {
    metrics: HashMap<&'a str, Box<Metric+ 'a>>,
    reporter: HashMap<&'a str, Box<Reporter>>,
}

// Specific stuff for registry goes here
impl<'a> Registry<'a> for StdRegistry<'a> {
    fn get(&'a self, name: &'a str) -> &'a Metric {
        &*self.metrics[name]
    }

    fn insert<T: Metric + 'a>(&mut self, name: &'a str, metric: T) {
        let boxed = Box::new(metric);

        self.metrics.insert(name, boxed);
    }

    fn add_scheduled_reporter(&mut self, reporter: Box<Reporter>) {
        let reporter_name = reporter.get_unique_reporter_name();
        self.reporter.insert(reporter_name, reporter);
    }
}

// General StdRegistry
impl<'a> StdRegistry<'a> {
    #[allow(dead_code)]
    pub fn new() -> StdRegistry<'a> {
        StdRegistry { metrics: HashMap::new(), reporter: HashMap::new() }
    }
}

#[cfg(test)]
mod test {
    use meter::StdMeter;
    use registry::{Registry, StdRegistry};

    #[test]
    fn meter() {
        let mut r: StdRegistry = StdRegistry::new();
        let m: StdMeter = StdMeter::new();

        r.insert("foo", m);
        r.get("foo");
    }
}
