// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use metrics::Metric;
use reporter::{Reporter, ReporterMsg};
use std::time::Duration;
use std::thread;
use std::sync::mpsc;
use std::collections::HashMap;

pub struct ConsoleReporter {
    metrics: mpsc::Sender<Result<ReporterMsg, &'static str>>,
    reporter_name: String,
    join_handle: thread::JoinHandle<Result<(), String>>,
}

impl Reporter for ConsoleReporter {
    fn get_unique_reporter_name(&self) -> &str {
        &(*self.reporter_name)
    }
    fn stop(self) -> Result<thread::JoinHandle<Result<(), String>>, String> {
        match self.metrics.send(Err("stop")) {
            Ok(_) => Ok(self.join_handle),
            Err(x) => Err(format!("Unable to stop reporter: {}", x)),
        }
    }
    fn addl<S: Into<String>>(&mut self,
                             name: S,
                             metric: Metric,
                             labels: Option<HashMap<String, String>>)
                             -> Result<(), String> {
        match self.metrics.send(Ok(ReporterMsg::AddMetric(name.into(), metric, labels))) {
            Ok(_) => Ok(()),
            Err(x) => Err(format!("Unable to send metric reporter{}", x)),
        }
    }
    fn remove<S: Into<String>>(&mut self, name: S) -> Result<(), String> {
        match self.metrics
            .send(Ok(ReporterMsg::RemoveMetric(name.into()))) {
            Ok(_) => Ok(()),
            Err(x) => Err(format!("Unable to remove metric reporter{}", x)),
        }
    }
}

impl ConsoleReporter {
    pub fn new<S: Into<String>>(reporter_name: S, delay_ms: u64) -> Self {
        let (tx, rx) = mpsc::channel();
        ConsoleReporter {
            metrics: tx,
            reporter_name: reporter_name.into(),
            join_handle: thread::spawn(move || {
                let mut stop = false;
                while !stop {
                    for metric in &rx {
                        match metric {
                            Ok(ReporterMsg::AddMetric(name, metric, labels)) => {
                                println!("name: {} labels: {:?}", name, labels);
                                match metric {
                                    Metric::Meter(ref x) => {
                                        println!("{:?}", x.snapshot());
                                    }
                                    Metric::Gauge(ref x) => {
                                        println!("{:?}", x.snapshot());
                                    }
                                    Metric::Counter(ref x) => {
                                        println!("{:?}", x.snapshot());
                                    }
                                    Metric::Histogram(ref x) => {
                                        println!("histogram{:?}", x);
                                    }
                                }
                            }
                            Ok(ReporterMsg::RemoveMetric(name)) => {

                                println!("Remove metric {}", name);
                            }
                            // Todo log the error somehow
                            Err(e) => {
                                println!("Stopping reporter because..:{}", e);
                                stop = true;
                            }
                        }
                        thread::sleep(Duration::from_millis(delay_ms));
                    }
                }
                Ok(())
            }),
        }
    }
}

#[cfg(test)]
mod test {

    use histogram::Histogram;
    use metrics::{Counter, Gauge, Meter, Metric, StdCounter, StdGauge, StdMeter};
    use super::ConsoleReporter;
    use reporter::Reporter;

    #[test]
    fn meter() {
        let m = StdMeter::new();
        m.mark(100);

        let c = StdCounter::new();
        c.inc();

        let g = StdGauge::new();
        g.set(2);

        let mut h = Histogram::configure()
            .max_value(100)
            .precision(1)
            .build()
            .unwrap();

        h.increment_by(1, 1).unwrap();

        let mut reporter = ConsoleReporter::new("test", 1);
        reporter.add("meter", Metric::Meter(m.clone())).unwrap();
        reporter.add("clone", Metric::Counter(c.clone())).unwrap();
        reporter.add("gauge", Metric::Gauge(g.clone())).unwrap();
        reporter.add("histo", Metric::Histogram(h)).unwrap();
        reporter.remove("histo").unwrap();
        g.set(4);
        reporter.stop().unwrap().join().unwrap().unwrap();
    }
}
