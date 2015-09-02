use std::collections::HashMap;

use metric::Metric;
use std::rc::Rc;
use registry::{Registry, StdRegistry};
use std::thread;
use std::sync::Arc;
use meter::Meter;

pub trait Reporter: Send + Sync {
    fn report(&self);

    fn get_unique_reporter_name(&self) -> &'static str;
}


pub struct ConsoleReporter {
    registry: Arc<StdRegistry<'static>>,
    reporter_name: &'static str
}

impl Reporter for ConsoleReporter {
    fn report(&self) {
        use metric::MetricType::{Counter, Gauge, Histogram, Meter};
        let registry = self.registry.clone();
        thread::spawn(move || {
                               loop {
                                   for metric_name in &registry.get_metrics_names() {
                                       let metric = registry.get(metric_name);
                                       match metric.get_type() {
                                           Meter(x) => {
                                               println!("{:?}", x);
                                           }
                                           Gauge(x) => {
                                               println!("{:?}", x);
                                           }
                                           Counter(x) => {
                                               println!("{:?}", x);
                                           }
                                           Histogram => {

                                           }
                                       }
                                   }

                                   thread::sleep_ms(1);
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
}


#[cfg(test)]
mod test {
    use meter::{Meter, StdMeter};
    use counter::{Counter, StdCounter};
    use gauge::{Gauge, StdGauge};
    use registry::{Registry, StdRegistry};
    use reporter::{ConsoleReporter, Reporter};
    use std::sync::Arc;

    #[test]
    fn meter() {
        let m = StdMeter::new();
        m.mark(100);

        let mut c: StdCounter = StdCounter::new();
        c.inc(1);

        let mut g: StdGauge = StdGauge { value: 0f64 };
        g.update(1.2);

        let mut r = StdRegistry::new();
        r.insert("meter1", m);
        r.insert("counter1", c);
        r.insert("gauge1", g);

        let mut arc_registry = Arc::new(r);
        let mut reporter = ConsoleReporter::new(arc_registry.clone(), "test");
        reporter.report();
        println!("poplopit");

    }
}
