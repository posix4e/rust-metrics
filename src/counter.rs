pub struct Counter {
    pub count: i64
}

impl Counter {
    pub fn clear(&mut self) {
        self.count = 0;
    }

    pub fn dec(&mut self, value: i64) {
        self.count -= value;
    }

    pub fn inc(&mut self, value: i64) {
        self.count += value;
    }

    pub fn snapshot(&self) -> Counter {
        Counter { count: self.count }
    }
}


#[cfg(test)]
mod test {
    use counter::Counter;

    #[test]
    fn create() {
        let mut c = Counter{count: 0};
    }
}