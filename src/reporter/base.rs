// Common traits and functionality to reporting.
// Also contains the ConsoleReporter
use registry::{Registry, StdRegistry};
use std::time::Duration;
use std::thread;
use std::sync::Arc;

pub trait Reporter: Send + Sync {
    fn get_unique_reporter_name(&self) -> &'static str;
}

pub struct ConsoleReporter {
    registry: Arc<StdRegistry<'static>>,
    reporter_name: &'static str,
}

impl Reporter for ConsoleReporter {
    fn get_unique_reporter_name(&self) -> &'static str {
        self.reporter_name
    }
}

impl ConsoleReporter {
    pub fn new(registry: Arc<StdRegistry<'static>>,
               reporter_name: &'static str)
               -> ConsoleReporter {
        ConsoleReporter {
            registry: registry,
            reporter_name: reporter_name,
        }
    }
    pub fn start(&self, delay_ms: u32) {
        use metrics::metric::MetricValue::{Counter, Gauge, Histogram, Meter};
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
                thread::sleep(Duration::from_millis(delay_ms as u64));
            }
        });
    }
}

#[cfg(test)]
mod test {

    use metrics::meter::{Meter, StdMeter};
    use metrics::counter::{Counter, StdCounter};
    use metrics::gauge::{Gauge, StdGauge};
    use registry::{Registry, StdRegistry};
    use reporter::base::ConsoleReporter;
    use std::sync::Arc;
    use std::time::Duration;
    use std::thread;
    use histogram::*;

    #[test]
    fn meter() {
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
        let reporter = ConsoleReporter::new(arc_registry.clone(), "test");
        reporter.start(1);
        g.set(1.4);
        thread::sleep(Duration::from_millis(200));
        println!("poplopit");

    }
}
