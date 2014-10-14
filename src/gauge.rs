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

#[cfg(test)]
mod test {
    use gauge::Gauge;

    #[test]
    fn create_and_shnapshot() {
        let g = Gauge{value: 0};

        let mut g_snapshot = g.snapshot();
        g_snapshot.update(10);

        assert!(g.value == 0);
        assert!(g_snapshot.value == 10);
    }
}