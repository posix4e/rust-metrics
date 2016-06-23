use metrics::{Metric, MetricValue};

/// Naive implementation of a `Counter`.
///
/// It might be nice to make one built on atomics. It would also be nice
/// if this weren't based on `f64`.
#[derive(Copy, Clone, Debug, Default)]
pub struct StdCounter {
    /// The counter value.
    pub value: f64,
}

/// A snapshot of the current value of a `Counter`.
#[derive(Debug)]
pub struct CounterSnapshot {
    /// The snapshot of the counter value.
    pub value: f64,
}

/// `Counter` is a `Metric` that represents a single numerical value that can
/// increases over time.
pub trait Counter {
    /// Clear the counter, setting the value to `0`.
    fn clear(&mut self);
    /// Increment the counter by 1.
    fn inc(&mut self);
    /// Increment the counter by the given amount. MUST check that v >= 0.
    fn add(&mut self, value: f64);
    /// Take a snapshot of the current value for use with a `Reporter`.
    fn snapshot(&self) -> CounterSnapshot;
}


impl Counter for StdCounter {
    fn clear(&mut self) {
        self.value = 0.0;
    }

    fn inc(&mut self) {
        self.value += 1.0;
    }

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
    /// Create a new `StdCounter`.
    pub fn new() -> Self {
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
