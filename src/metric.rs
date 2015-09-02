use counter::StdCounter;
use gauge::StdGauge;
use meter::MeterSnapshot;
/// a Metric
pub trait Metric: Send + Sync {
    fn get_type(&self) -> MetricType;
}
pub enum MetricType {
    Counter(StdCounter),
    Gauge(StdGauge),
    Meter(MeterSnapshot),
    Histogram
}
