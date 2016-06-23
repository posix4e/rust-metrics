use metrics::metric::{Metric, MetricValue};
use time::get_time;

/// Naive implementation of a `Gauge`.
///
/// It might be nice to make one built on atomics.
#[derive(Copy, Clone, Debug, Default)]
pub struct StdGauge {
    pub value: f64,
}

/// A snapshot of the value of a `Gauge`.
#[derive(Debug)]
pub struct GaugeSnapshot {
    pub value: f64,
}

/// `Gauge` is a `Metric` that represents a single numerical value that can
/// arbitrarily go up and down.
///
/// A `Gauge` is typically used for measured values like temperatures or current
/// memory usage, but also "counts" that can go up and down.
pub trait Gauge {
    /// Increment the gauge by 1.
    fn inc(&mut self);
    /// Decrement the gauge by 1.
    fn dec(&mut self);
    /// Increment the gauge by the given amount.
    fn add(&mut self, value: f64);
    /// Decrement the gauge by the given amount.
    fn sub(&mut self, value: f64);
    /// Set the current value of the gauge.
    fn set(&mut self, value: f64);
    ///  Set the current value to the current timestamp.
    fn set_to_current_time(&mut self);
    /// Take a snapshot of the current value for use with a `Reporter`.
    fn snapshot(&self) -> GaugeSnapshot;
}

impl Gauge for StdGauge {
    fn inc(&mut self) {
        self.value += 1.0;
    }

    fn dec(&mut self) {
        self.value -= 1.0;
    }

    fn add(&mut self, value: f64) {
        self.value += value;
        // TODO check for negative
    }

    fn sub(&mut self, value: f64) {
        self.value -= value;
    }

    fn set(&mut self, value: f64) {
        self.value = value
    }

    fn set_to_current_time(&mut self) {
        self.value = timestamp();
    }

    fn snapshot(&self) -> GaugeSnapshot {
        GaugeSnapshot { value: self.value }
    }
}

impl Metric for StdGauge {
    fn export_metric(&self) -> MetricValue {
        MetricValue::Gauge(self.snapshot())
    }
}

fn timestamp() -> f64 {
    let timespec = get_time();
    // 1459440009.113178
    timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_and_snapshot() {
        let mut g = StdGauge::default();
        let snapshot_1 = g.snapshot();
        g.set(10.0);
        let snapshot_2 = g.snapshot();

        assert_eq!(g.value, 10.0);
        assert_eq!(snapshot_1.value, 0.0);
        assert_eq!(snapshot_2.value, 10.0);
    }
}
