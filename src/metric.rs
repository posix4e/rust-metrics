use counter::StdCounter;
use gauge::{Gauge, StdGauge};
use meter::MeterSnapshot;
use num::traits::Zero;
use std::ops::{Add, Sub};
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
