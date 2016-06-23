// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
