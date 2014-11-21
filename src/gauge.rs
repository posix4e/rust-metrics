use std::num::Num;

pub struct StdGauge<T: Num> {
    pub value: T
}


pub trait Gauge<T: Num> {
    fn update(&mut self, value: T);
    fn snapshot(self) -> Self;
}


impl<T: Num> Gauge<T> for StdGauge<T> {
    fn update(&mut self, value: T) {
        self.value = value
    }

    fn snapshot(self) -> StdGauge<T> {
        StdGauge { value: self.value }
    }
}


#[cfg(test)]
mod test {
    use gauge::{Gauge,StdGauge};

    #[test]
    fn create_and_snapshot() {
        let g: StdGauge<f64> = StdGauge {value: 0f64 };
        let mut g_snapshot = g.snapshot();

        g_snapshot.update(10f64);

        assert!(g.value == 0f64);
        assert!(g_snapshot.value == 10f64);
    }
}