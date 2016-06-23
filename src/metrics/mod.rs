//! Metrics

mod counter;
mod gauge;
mod meter;

pub use self::counter::{Counter, CounterSnapshot, StdCounter};
pub use self::gauge::{Gauge, GaugeSnapshot, StdGauge};
pub use self::meter::{Meter, MeterSnapshot, StdMeter};

/// a Metric
use histogram::Histogram;

//  TODO rename to MetricSnapshot
#[allow(missing_docs)]
pub trait Metric: Send + Sync {
    fn export_metric(&self) -> MetricValue;
}

#[allow(missing_docs)]
impl Metric for Histogram {
    fn export_metric(&self) -> MetricValue {
        MetricValue::Histogram(self.clone())
    }
}

#[allow(missing_docs)]
pub enum MetricValue {
    Counter(CounterSnapshot),
    Gauge(GaugeSnapshot),
    Meter(MeterSnapshot),
    Histogram(Histogram),
}
