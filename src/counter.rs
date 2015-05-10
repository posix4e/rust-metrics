extern crate num;

use self::num::traits::Zero;
use std::ops::{Add,Sub};
use metric::Metric;

pub struct StdCounter<T: Zero + Add<T, Output = T> + Sub<T, Output = T>> {
    pub value: T,
}

pub trait Counter<T>: Metric {
    fn clear(&mut self);

    fn dec(&mut self, value: T);

    fn inc(&mut self, value: T);

    fn snapshot(self) -> Self;
}

impl<T: Zero + Add<T, Output = T> + Sub<T, Output = T> + Copy> Counter<T> for StdCounter<T> {
    fn clear(&mut self) {
        self.value = T::zero();
    }

    fn dec(&mut self, value: T) {
        self.value = self.value - value;
    }

    fn inc(&mut self, value: T) {
        self.value = self.value + value;
    }

    fn snapshot(self) -> StdCounter<T> {
        StdCounter { value: self.value }
    }
}

impl<T> Metric for StdCounter<T> { }

impl<T: Zero + Add<T, Output = T> + Sub<T, Output = T>> StdCounter<T> {
    pub fn new() -> StdCounter<T> {
        StdCounter{ value: T::zero() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn increment_by_1() {
        let mut c: StdCounter<i32> = StdCounter::new();
        c.inc(1);

        assert!(c.value == 1);
    }

    #[test]
    fn snapshot() {
        let c: StdCounter<i32> = StdCounter::new();
        let mut c_snapshot = c.snapshot();

        c_snapshot.inc(1);

        assert!(c.value == 0);
        assert!(c_snapshot.value == 1);
    }
}
