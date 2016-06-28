// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Metrics

use std::sync::Arc;

mod counter;
mod gauge;
mod meter;

pub use self::counter::{Counter, CounterSnapshot, StdCounter};
pub use self::gauge::{Gauge, GaugeSnapshot, StdGauge};
pub use self::meter::{Meter, MeterSnapshot, StdMeter};

/// a Metric
use histogram::Histogram;

#[allow(missing_docs)]
pub enum Metric {
    Counter(Arc<Counter>),
    Gauge(Arc<Gauge>),
    Meter(Arc<Meter>),
    Histogram(Histogram),
}
