extern crate num;

use metric::{Metric, MetricValue};

// This can be much better with a different datatype
#[derive(Copy, Clone, Debug)]
pub struct StdCounter {
    pub value: f64,
}

pub trait Counter {
    fn clear(&mut self);
    fn inc(&mut self);
    fn add(&mut self, value: f64);
    fn snapshot(self) -> Self;
}


impl Counter for StdCounter {
    fn clear(&mut self) {
        self.value = 0 as f64;
    }

    // inc(): Increment the counter by 1
    fn inc(&mut self) {
        self.value = self.value + 1 as f64;
    }

    // inc(double v): Increment the counter by the given amount. MUST check that v >= 0.
    // We crash with interger overflow
    fn add(&mut self, value: f64) {
        self.value = self.value + value as f64;
    }

    fn snapshot(self) -> StdCounter {
        StdCounter { value: self.value }
    }
}

impl Metric for StdCounter {
    fn export_metric(&self) -> MetricValue {
        MetricValue::Counter(self.snapshot())
    }
}

impl StdCounter {
    pub fn new() -> StdCounter {
        StdCounter { value: 0 as f64 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a_counting_counter() {
        let mut c: StdCounter = StdCounter::new();
        c.add(1 as f64);

        assert!(c.value == 1 as f64);

        let mut c: StdCounter = StdCounter::new();
        c.inc();

        assert!(c.value == 1 as f64);
    }

    #[test]
    fn make_sure_we_can_actually_export_metrics() {
        let c: StdCounter = StdCounter::new();
        let mut c_snapshot = c.snapshot();

        c_snapshot.add(1 as f64);

        assert!(c.value == 0 as f64);
        assert!(c_snapshot.value == 1 as f64);
    }
}
