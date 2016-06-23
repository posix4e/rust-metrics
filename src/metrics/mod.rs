mod meter;
mod counter;
mod gauge;

pub use self::counter::{Counter, CounterSnapshot, StdCounter};
pub use self::gauge::{Gauge, GaugeSnapshot, StdGauge};
pub use self::meter::{Meter, MeterSnapshot, StdMeter};

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
