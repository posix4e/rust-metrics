// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cell::Cell;
use std::sync::Arc;

/// Naive implementation of a `Gauge`.
///
/// It might be nice to make one built on atomics.
#[derive(Debug)]
pub struct StdGauge {
    /// The gauge value.
    pub value: Cell<f64>,
}

/// A snapshot of the value of a `Gauge`.
#[derive(Debug)]
pub struct GaugeSnapshot {
    /// The snapshot of the gauge value.
    pub value: f64,
}

/// `Gauge` is a `Metric` that represents a single numerical value that can
/// arbitrarily go up and down.
///
/// A `Gauge` is typically used for measured values like temperatures or current
/// memory usage, but also "counts" that can go up and down.
pub trait Gauge {
    /// Increment the gauge by 1.
    fn inc(&self);
    /// Decrement the gauge by 1.
    fn dec(&self);
    /// Increment the gauge by the given amount.
    fn add(&self, value: f64);
    /// Decrement the gauge by the given amount.
    fn sub(&self, value: f64);
    /// Set the current value of the gauge.
    fn set(&self, value: f64);
    /// Take a snapshot of the current value for use with a `Reporter`.
    fn snapshot(&self) -> GaugeSnapshot;
}

impl Gauge for StdGauge {
    fn inc(&self) {
        self.value.set(self.value.get() + 1.0);
    }

    fn dec(&self) {
        self.value.set(self.value.get() - 1.0);
    }

    fn add(&self, value: f64) {
        self.value.set(self.value.get() + value);
        // TODO check for negative
    }

    fn sub(&self, value: f64) {
        self.value.set(self.value.get() - value);
    }

    fn set(&self, value: f64) {
        self.value.set(value);
    }

    fn snapshot(&self) -> GaugeSnapshot {
        GaugeSnapshot { value: self.value.get() }
    }
}

impl StdGauge {
    /// Create a new `StdGauge`.
    pub fn new() -> Arc<Self> {
        Arc::new(StdGauge { value: Cell::new(0.0) })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_and_snapshot() {
        let g = StdGauge::new();
        let snapshot_1 = g.snapshot();
        g.set(10.0);
        let snapshot_2 = g.snapshot();

        assert_eq!(g.value.get(), 10.0);
        assert_eq!(snapshot_1.value, 0.0);
        assert_eq!(snapshot_2.value, 10.0);
    }
}
