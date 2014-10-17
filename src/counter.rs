use std::num::Zero;

use metric::Metric;


pub struct StdCounter<T: Num> {
    pub value: T
}


pub trait Counter<T: Num> : Metric {
    fn clear(&mut self);

    fn dec(&mut self, value: T);

    fn inc(&mut self, value: T);

    fn snapshot(self) -> Self;
}


impl<T: Num> Counter<T> for StdCounter<T> {
    fn clear(&mut self) {
        self.value = Zero::zero();
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


impl<T: Num> Metric for StdCounter<T>    {
}


impl<T: Num> StdCounter<T> {
    pub fn new() -> StdCounter<T> {
        StdCounter{ value: Zero::zero() }
    }
}


#[cfg(test)]
mod test {
    use counter::StdCounter;
    use counter::Counter;

    #[test]
    fn increment_by_1() {
        let mut c: StdCounter<int> = StdCounter{ value: 0 };
        c.inc(1);
        assert!(c.value == 1);
    }

    #[test]
    fn snapshot() {
        let c: StdCounter<int> = StdCounter{value: 0};
        let mut c_snapshot = c.snapshot();

        c_snapshot.inc(1);

        assert!(c.value == 0);
        assert!(c_snapshot.value == 1);
    }
}