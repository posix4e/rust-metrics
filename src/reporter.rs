use std::collections::HashMap;

use metric::Metric;
use std::rc::Rc;
use registry::Registry;
use counter::Counter;
use ewma::EWMA;
use gauge::Gauge;
use meter::Meter;

pub trait Reporter {
    fn report(self, report_card: ReportCard);

    fn get_unique_reporter_name(&self) -> &'static str;
}

pub struct ReportCard {
    pub metrics: HashMap<String, Box<Metric>>
}

impl Reporter for ConsoleReporter {
    fn report(self, report_card: ReportCard) {
        let mut it = report_card.metrics.iter();
        loop {
            match it.next() {
                Some(x) => {
                    match x {
                        (k, v) => {
                            println!("k: {}", k);
                        }
                    }
                }
                None => break,
            }
        }
    }

    fn get_unique_reporter_name(&self) -> &'static str {
        self.reporter_name
    }
}


pub struct ConsoleReporter {
    reporter_name: &'static str
}


impl ConsoleReporter {
    pub fn new(reporter_name: &'static str) -> ConsoleReporter {
        ConsoleReporter { reporter_name: reporter_name }
    }
}

/*
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
*/
