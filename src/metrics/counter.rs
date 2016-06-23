use metrics::metric::{Metric, MetricValue};

// This can be much better with a different datatype
#[derive(Copy, Clone, Debug, Default)]
pub struct StdCounter {
    pub value: f64,
}

#[derive(Debug)]
pub struct CounterSnapshot {
    pub value: f64,
}

pub trait Counter {
    fn clear(&mut self);
    fn inc(&mut self);
    fn add(&mut self, value: f64);
    fn snapshot(&self) -> CounterSnapshot;
}


impl Counter for StdCounter {
    fn clear(&mut self) {
        self.value = 0.0;
    }

    // inc(): Increment the counter by 1
    fn inc(&mut self) {
        self.value += 1.0;
    }

    // inc(double v): Increment the counter by the given amount. MUST check that v >= 0.
    // We crash with integer overflow
    fn add(&mut self, value: f64) {
        self.value += value;
    }

    fn snapshot(&self) -> CounterSnapshot {
        CounterSnapshot { value: self.value }
    }
}

impl Metric for StdCounter {
    fn export_metric(&self) -> MetricValue {
        MetricValue::Counter(self.snapshot())
    }
}

impl StdCounter {
    pub fn new() -> StdCounter {
        StdCounter { value: 0.0 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a_counting_counter() {
        let mut c = StdCounter::new();
        c.add(1.0);

        assert_eq!(c.value, 1.0);

        let mut c = StdCounter::new();
        c.inc();

        assert_eq!(c.value, 1.0);
    }

    #[test]
    fn validate_snapshots() {
        let mut c = StdCounter::new();
        let snapshot_1 = c.snapshot();
        c.add(1.0);
        let snapshot_2 = c.snapshot();
        assert_eq!(c.value, 1.0);
        assert_eq!(snapshot_1.value, 0.0);
        assert_eq!(snapshot_2.value, 1.0);
    }
}
