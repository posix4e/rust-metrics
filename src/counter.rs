use std::num::Int;

use metric::Metric;

pub struct StdCounter<T: Int> {
    pub value: T,
}

pub trait Counter<T: Int>: Metric {
    fn clear(&mut self);

    fn dec(&mut self, value: T);

    fn inc(&mut self, value: T);

    fn snapshot(self) -> Self;
}

impl<T: Int> Counter<T> for StdCounter<T> {
    fn clear(&mut self) {
        self.value = Int::zero();
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

impl<T: Int> Metric for StdCounter<T> { }

impl<T: Int> StdCounter<T> {
    pub fn new() -> StdCounter<T> {
        StdCounter{ value: Int::zero() }
    }
}

#[cfg(test)]
mod test {
    use counter::StdCounter;
    use counter::Counter;

    #[test]
    fn increment_by_1() {
        let mut c: StdCounter<int> = StdCounter{ value: 0i };
        c.inc(1);

        assert!(c.value == 1);
    }

    #[test]
    fn snapshot() {
        let c: StdCounter<int> = StdCounter{value: 0i };
        let mut c_snapshot = c.snapshot();

        c_snapshot.inc(1);

        assert!(c.value == 0);
        assert!(c_snapshot.value == 1);
    }
}
