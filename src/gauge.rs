pub struct Gauge {
    pub value: i64
}

impl Gauge {
    pub fn update(&mut self, value: i64) {
        self.value = value;
    }

    pub fn snapshot(&self) -> Gauge {
        Gauge { value: self.value }
    }
}