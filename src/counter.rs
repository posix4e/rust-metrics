pub struct Counter {
    pub value: i64
}

impl Counter {
    pub fn clear(&mut self) {
        self.value = 0;
    }

    pub fn dec(&mut self, value: i64) {
        self.value -= value;
    }

    pub fn inc(&mut self, value: i64) {
        self.value += value;
    }

    pub fn snapshot(&self) -> Counter {
        Counter { value: self.value }
    }
}


#[cfg(test)]
mod test {
    use counter::Counter;

    #[test]
    fn increment_by_1() {
        let mut c = Counter{ value: 0 };
        c.inc(1);
        assert!(c.value == 1);
    }

    #[test]
    fn snapshot() {
        let c = Counter{value: 0};
        let mut c_snapshot = c.snapshot();

        c_snapshot.inc(1);

        assert!(c.value == 0);
        assert!(c_snapshot.value == 1);
    }
}