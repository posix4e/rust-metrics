extern crate num;

use metric::{Metric, MetricValue};

#[derive(Copy, Clone, Debug)]
pub struct StdCounter {
    pub value: i64
}

pub trait Counter{
    fn clear(&mut self);

    fn dec(&mut self, value: i64);

    fn inc(&mut self, value: i64);

    fn snapshot(self) -> Self;
}

impl Counter for StdCounter {
    fn clear(&mut self) {
        self.value = 0;
    }

    fn dec(&mut self, value: i64) {
        self.value = self.value - value;
    }

    fn inc(&mut self, value: i64) {
        self.value = self.value + value;
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
        StdCounter { value: 0 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn increment_by_1() {
        let mut c: StdCounter = StdCounter::new();
        c.inc(1);

        assert!(c.value == 1);
    }

    #[test]
    fn snapshot() {
        let c: StdCounter = StdCounter::new();
        let mut c_snapshot = c.snapshot();

        c_snapshot.inc(1);

        assert!(c.value == 0);
        assert!(c_snapshot.value == 1);
    }
}
