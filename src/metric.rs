use counter::StdCounter;
use gauge::StdGauge;
use meter::MeterSnapshot;
/// a Metric
use histogram::Histogram;

pub trait Metric: Send + Sync {
    fn export_metric(&self) -> MetricValue;
}

impl Metric for Histogram {
    fn export_metric(&self) -> MetricValue {
        MetricValue::Histogram(self.clone())
    }
}
pub enum MetricValue {
    Counter(StdCounter),
    Gauge(StdGauge),
    Meter(MeterSnapshot),
    Histogram(Histogram)
}
