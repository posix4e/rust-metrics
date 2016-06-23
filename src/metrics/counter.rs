// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cell::Cell;
use std::sync::Arc;

/// Naive implementation of a `Counter`.
///
/// It might be nice to make one built on atomics. It would also be nice
/// if this weren't based on `f64`.
#[derive(Debug)]
pub struct StdCounter {
    /// The counter value.
    pub value: Cell<f64>,
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
    fn clear(&self);
    /// Increment the counter by 1.
    fn inc(&self);
    /// Increment the counter by the given amount. MUST check that v >= 0.
    fn add(&self, value: f64);
    /// Take a snapshot of the current value for use with a `Reporter`.
    fn snapshot(&self) -> CounterSnapshot;
}


impl Counter for StdCounter {
    fn clear(&self) {
        self.value.set(0.0);
    }

    fn inc(&self) {
        self.value.set(self.value.get() + 1.0);
    }

    fn add(&self, value: f64) {
        self.value.set(self.value.get() + value);
    }

    fn snapshot(&self) -> CounterSnapshot {
        CounterSnapshot { value: self.value.get() }
    }
}

impl StdCounter {
    /// Create a new `StdCounter`.
    pub fn new() -> Arc<Self> {
        Arc::new(StdCounter { value: Cell::new(0.0) })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a_counting_counter() {
        let c = StdCounter::new();
        c.add(1.0);

        assert_eq!(c.value.get(), 1.0);

        let c = StdCounter::new();
        c.inc();

        assert_eq!(c.value.get(), 1.0);
    }

    #[test]
    fn validate_snapshots() {
        let c = StdCounter::new();
        let snapshot_1 = c.snapshot();
        c.add(1.0);
        let snapshot_2 = c.snapshot();
        assert_eq!(c.value.get(), 1.0);
        assert_eq!(snapshot_1.value, 0.0);
        assert_eq!(snapshot_2.value, 1.0);
    }
}
