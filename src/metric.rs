use counter::StdCounter;
use gauge::StdGauge;
use meter::MeterSnapshot;
/// a Metric
pub trait Metric: Send + Sync {
    fn get_type(&self) -> MetricValue;
}
pub enum MetricValue {
    Counter(StdCounter),
    Gauge(StdGauge),
    Meter(MeterSnapshot),
    Histogram
}
