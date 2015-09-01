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
    metrics: HashMap<String, Box<Metric>>,
}

impl Reporter for ConsoleReporter {
    fn report(self, report_card: ReportCard ) {

    }

    fn get_unique_reporter_name(&self) -> &'static str {
        "console-reporter"
    }
}


pub struct ConsoleReporter;
/*

impl<'a> ConsoleReporter<'a> {
    pub fn new(registry_to_report_to_console:Rc<Registry<'a>>) -> ConsoleReporter<'a> {
        ConsoleReporter {registry: registry_to_report_to_console}
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
*/
