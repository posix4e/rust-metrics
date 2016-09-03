// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use metrics::Metric;
use reporter::Reporter;
use std::time::Duration;
use std::thread;
use std::sync::mpsc;

pub struct ConsoleReporter {
    metrics: mpsc::Sender<Result<Metric, &'static str>>,
    reporter_name: String,
}

impl Reporter for ConsoleReporter {
    fn get_unique_reporter_name(&self) -> &str {
        &(*self.reporter_name)
    }
    fn stop(&mut self) {
        match self.metrics.send(Err("stop")) {
            Ok(_) => {}
            Err(x) => println!("Unable to stop reporter: {}", x),
        }
    }
}

impl ConsoleReporter {
    pub fn new<S: Into<String>>(reporter_name: S, delay_ms: u64) -> Self {
        let (tx, rx) = mpsc::channel();
        let reporter = ConsoleReporter {
            metrics: tx,
            reporter_name: reporter_name.into(),
        };

        thread::spawn(move || {
            let mut stop = false;
            while !stop {
                for metric in &rx {
                    match metric {
                        Ok(metric) => {
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
                        // Todo log the error somehow
                        Err(e) => {
                            println!("Stopping reporter because..:{}", e);
                            stop = true;
                        }
                    }

                    thread::sleep(Duration::from_millis(delay_ms));
                }
            }
        });
        reporter
    }

    pub fn add(&mut self, metric: Metric) {
        match self.metrics.send(Ok(metric)) {
            Ok(_) => {}
            Err(x) => println!("Unable to send metric reporter{}", x),
        }
    }
}

#[cfg(test)]
mod test {

    use histogram::Histogram;
    use metrics::{Counter, Gauge, Meter, Metric, StdCounter, StdGauge, StdMeter};
    use std::thread;
    use std::time::Duration;
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
        reporter.add(Metric::Meter(m.clone()));
        reporter.add(Metric::Counter(c.clone()));
        reporter.add(Metric::Gauge(g.clone()));
        reporter.add(Metric::Histogram(h));
        g.set(4);
        thread::sleep(Duration::from_millis(200));
        reporter.stop();
    }
}
