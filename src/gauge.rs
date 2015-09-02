use metric::{Metric, MetricValue};

#[derive(Copy, Clone, Debug)]
pub struct StdGauge {
    pub value: f64
}

pub trait Gauge {
    fn update(&mut self, value: f64);

    fn snapshot(self) -> Self;
}

impl Gauge for StdGauge {
    fn update(&mut self, value: f64) {
        self.value = value
    }

    fn snapshot(self) -> StdGauge {
        StdGauge { value: self.value }
    }
}

impl Metric for StdGauge {
    fn export_metric(&self) -> MetricValue {
        MetricValue::Gauge(self.snapshot())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_and_snapshot() {
        let g: StdGauge = StdGauge { value: 0f64 };
        let mut g_snapshot = g.snapshot();

        g_snapshot.update(10f64);

        assert_eq!(g.value, 0f64);
        assert_eq!(g_snapshot.value, 10f64);
    }
}
