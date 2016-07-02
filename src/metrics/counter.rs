// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Naive implementation of a `Counter`.
#[derive(Debug)]
pub struct StdCounter {
    /// The counter value.
    value: AtomicUsize,
}

/// A snapshot of the current value of a `Counter`.
#[derive(Debug)]
pub struct CounterSnapshot {
    /// The snapshot of the counter value.
    pub value: usize,
}

/// `Counter` is a `Metric` that represents a single numerical value that can
/// increases over time.
pub trait Counter: Send + Sync {
    /// Clear the counter, setting the value to `0`.
    fn clear(&self);
    /// Increment the counter by 1.
    fn inc(&self);
    /// Increment the counter by the given amount. MUST check that v >= 0.
    fn add(&self, value: usize);
    /// Take a snapshot of the current value for use with a `Reporter`.
    fn snapshot(&self) -> CounterSnapshot;
}


impl Counter for StdCounter {
    fn clear(&self) {
        self.value.store(0, Ordering::Relaxed);
    }

    fn inc(&self) {
        self.value.fetch_add(1, Ordering::Relaxed);
    }

    fn add(&self, value: usize) {
        self.value.fetch_add(value, Ordering::Relaxed);
    }

    fn snapshot(&self) -> CounterSnapshot {
        CounterSnapshot { value: self.value.load(Ordering::Relaxed) }
    }
}

impl StdCounter {
    /// Create a new `StdCounter`.
    pub fn new() -> Arc<Self> {
        Arc::new(StdCounter { value: AtomicUsize::new(0) })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a_counting_counter() {
        let c = StdCounter::new();
        c.add(1);

        assert_eq!(c.snapshot().value, 1);

        let c = StdCounter::new();
        c.inc();

        assert_eq!(c.snapshot().value, 1);
    }

    #[test]
    fn validate_snapshots() {
        let c = StdCounter::new();
        let snapshot_1 = c.snapshot();
        c.add(1);
        let snapshot_2 = c.snapshot();
        assert_eq!(snapshot_1.value, 0);
        assert_eq!(snapshot_2.value, 1);
    }
}
