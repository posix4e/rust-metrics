pub struct GaugeF64 {
    pub value: f64
}

impl GaugeF64 {
    pub fn update(&mut self, value: f64) {
        self.value = value;
    }

    pub fn snapshot(self) -> GaugeF64 {
        GaugeF64{ value: self.value }
    }
}


#[cfg(test)]
mod test {
    use gauge_f64::GaugeF64;

    #[test]
    fn create_and_snapshot() {
        let g = GaugeF64{value: 0f64};
        let mut g_snapshot = g.snapshot();

        g_snapshot.update(10f64);

        assert!(g.value == 0f64);
        assert!(g_snapshot.value == 10f64);

    }
}