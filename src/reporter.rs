use metric::Metric;
use registry::{Registry, StdRegistry};
use std::thread;
use std::sync::Arc;
use meter::Meter;

pub trait Reporter: Send + Sync {
    fn report(&self, delay_ms: u32);

    fn get_unique_reporter_name(&self) -> &'static str;
}

pub struct ConsoleReporter {
    registry: Arc<StdRegistry<'static>>,
    reporter_name: &'static str
}

impl Reporter for ConsoleReporter {
    fn report(&self, delay_ms: u32) {
        use metric::MetricValue::{Counter, Gauge, Histogram, Meter};
        let registry = self.registry.clone();
        thread::spawn(move || {
                               loop {
                                   for metric_name in &registry.get_metrics_names() {
                                       let metric = registry.get(metric_name);
                                       match metric.export_metric() {
                                           Meter(x) => {
                                               println!("{:?}", x);
                                           }
                                           Gauge(x) => {
                                               println!("{:?}", x);
                                           }
                                           Counter(x) => {
                                               println!("{:?}", x);
                                           }
                                           Histogram(x) => {
                                               println!("histogram{:?}", x);
                                           }
                                       }
                                   }

                                   thread::sleep_ms(delay_ms);
                               }
                           });
    }

    fn get_unique_reporter_name(&self) -> &'static str {
        self.reporter_name
    }
}

impl ConsoleReporter {
    pub fn new(registry: Arc<StdRegistry<'static>>, reporter_name: &'static str) -> ConsoleReporter {
        ConsoleReporter { registry: registry, reporter_name: reporter_name }
    }
    pub fn start(self, delay_ms: u32) {
        self.report(delay_ms);
    }
}

#[cfg(test)]
mod test {

    use meter::{Meter, StdMeter};
    use counter::{Counter, StdCounter};
    use gauge::{Gauge, StdGauge};
    use registry::{Registry, StdRegistry};
    use reporter::{ConsoleReporter, Reporter};
    use std::sync::Arc;
    use std::thread;
    use histogram::*;

    #[test]
    fn meter() {
        let m = StdMeter::new();
        m.mark(100);

        let mut c: StdCounter = StdCounter::new();
        c.inc(1);

        let mut g: StdGauge = StdGauge { value: 0f64 };
        g.update(1.2);

        let mut h = Histogram::new(
    HistogramConfig{
        max_memory: 0,
        max_value: 1000000,
        precision: 3,
}).unwrap();
        h.record(1, 1);


        let mut r = StdRegistry::new();
        r.insert("meter1", m);
        r.insert("counter1", c);
        r.insert("gauge1", g);
        r.insert("histogram", h);

        let arc_registry = Arc::new(r);
        let reporter = ConsoleReporter::new(arc_registry.clone(), "test");
        reporter.start(1);
        g.update(1.4);
        thread::sleep_ms(200);
        println!("poplopit");

    }
}
