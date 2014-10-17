use metric::Metric;

pub struct StdCounter {
    pub value: i64
}


pub trait Counter : Metric {
    fn clear(&mut self);

    fn dec(&mut self, value: i64);

    fn inc(&mut self, value: i64);

    fn snapshot(&self) -> Self;

    fn new() -> Self;
}


impl Counter for StdCounter {
    fn clear(&mut self) {
        self.value = 0;
    }

    fn dec(&mut self, value: i64) {
        self.value -= value;
    }

    fn inc(&mut self, value: i64) {
        self.value += value;
    }

    fn snapshot(&self) ->  StdCounter {
        StdCounter { value: self.value }
    }

    fn new() -> StdCounter {
        StdCounter{ value: 0i64 }
    }
}


impl Metric for StdCounter {
}


#[cfg(test)]
mod test {
    use counter::StdCounter;
    use counter::Counter;

    #[test]
    fn increment_by_1() {
        let mut c = StdCounter{ value: 0 };
        c.inc(1);
        assert!(c.value == 1);
    }

    #[test]
    fn snapshot() {
        let c = StdCounter{value: 0};
        let mut c_snapshot = c.snapshot();

        c_snapshot.inc(1);

        assert!(c.value == 0);
        assert!(c_snapshot.value == 1);
    }
}