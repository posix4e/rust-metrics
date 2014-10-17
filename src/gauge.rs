pub struct Gauge<T> {
    pub value: T
}


impl<T> Gauge<T> {
    pub fn update(&mut self, value: T) {
        self.value = value
    }

    pub fn snapshot(self) -> Gauge<T> {
        Gauge { value: self.value }
    }
}


#[cfg(test)]
mod test {
    use gauge::Gauge;

    #[test]
    fn create_and_snapshot() {
        let g: Gauge<f64> = Gauge {value: 0f64 };
        let mut g_snapshot = g.snapshot();

        g_snapshot.update(10f64);

        assert!(g.value == 0f64);
        assert!(g_snapshot.value == 10f64);
    }
}