// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::Arc;

/// Naive implementation of a `Gauge`.
#[derive(Debug)]
pub struct StdGauge {
    /// The gauge value.
    pub value: AtomicIsize,
}

/// A snapshot of the value of a `Gauge`.
#[derive(Debug)]
pub struct GaugeSnapshot {
    /// The snapshot of the gauge value.
    pub value: isize,
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
    fn add(&self, value: isize);
    /// Decrement the gauge by the given amount.
    fn sub(&self, value: isize);
    /// Set the current value of the gauge.
    fn set(&self, value: isize);
    /// Take a snapshot of the current value for use with a `Reporter`.
    fn snapshot(&self) -> GaugeSnapshot;
}

impl Gauge for StdGauge {
    fn inc(&self) {
        self.value.fetch_add(1, Ordering::Relaxed);
    }

    fn dec(&self) {
        self.value.fetch_sub(1, Ordering::Relaxed);
    }

    fn add(&self, value: isize) {
        self.value.fetch_add(value, Ordering::Relaxed);
        // TODO check for negative
    }

    fn sub(&self, value: isize) {
        self.value.fetch_sub(value, Ordering::Relaxed);
    }

    fn set(&self, value: isize) {
        self.value.store(value, Ordering::Relaxed);
    }

    fn snapshot(&self) -> GaugeSnapshot {
        GaugeSnapshot { value: self.value.load(Ordering::Relaxed) }
    }
}

impl StdGauge {
    /// Create a new `StdGauge`.
    pub fn new() -> Arc<Self> {
        Arc::new(StdGauge { value: AtomicIsize::new(0) })
    }
}

#[cfg(test)]
mod test {
    use std::sync::atomic::Ordering;
    use super::*;

    #[test]
    fn create_and_snapshot() {
        let g = StdGauge::new();
        let snapshot_1 = g.snapshot();
        g.set(10);
        let snapshot_2 = g.snapshot();

        assert_eq!(g.value.load(Ordering::Relaxed), 10);
        assert_eq!(snapshot_1.value, 0);
        assert_eq!(snapshot_2.value, 10);
    }
}
