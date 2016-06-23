use metrics::counter::CounterSnapshot;
use metrics::gauge::GaugeSnapshot;
use metrics::meter::MeterSnapshot;
/// a Metric
use histogram::Histogram;

//  TODO rename to MetricSnapshot
pub trait Metric: Send + Sync {
    fn export_metric(&self) -> MetricValue;
}

impl Metric for Histogram {
    fn export_metric(&self) -> MetricValue {
        MetricValue::Histogram(self.clone())
    }
}
pub enum MetricValue {
    Counter(CounterSnapshot),
    Gauge(GaugeSnapshot),
    Meter(MeterSnapshot),
    Histogram(Histogram),
}
