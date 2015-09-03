use time::get_time;
use time::Timespec;
use std::sync::{Mutex, MutexGuard};
use ewma::EWMA;
use metric::{Metric, MetricValue};

const WINDOW: [f64; 3] = [1f64, 5f64, 15f64];

// A MeterSnapshot
#[derive(Debug)]
pub struct MeterSnapshot {
    pub count: i64,
    pub rates: [f64; 3],
    pub mean: f64
}

// A StdMeter struct
#[derive(Debug)]
pub struct StdMeter {
    data: Mutex<MeterSnapshot>,
    ewma: [EWMA; 3],
    start: Timespec
}

// A Meter trait
pub trait Meter : Metric {

    fn get_meter(&self) -> MeterSnapshot {
        self.snapshot()
    }

    fn export_metric(&self) -> MetricValue;

    fn snapshot(&self) -> MeterSnapshot;

    fn mark(&self, n: i64);

    fn tick(&mut self);

    fn rate(&self, rate: f64) -> f64;

    fn mean(&self) -> f64;

    fn count(&self) -> i64;
}

impl Meter for StdMeter {
    fn export_metric(&self) -> MetricValue {
        use metric::MetricValue::Meter;
        let snapshot: MeterSnapshot = self.snapshot();
        Meter(snapshot)
    }

    fn snapshot(&self) -> MeterSnapshot {
        let s = self.data.lock().unwrap();

        MeterSnapshot { count: s.count, rates: s.rates, mean: s.mean }
    }

    fn mark(&self, n: i64) {
        let mut s = self.data.lock().unwrap();

        s.count += n;

        for i in (0..WINDOW.len()) {
            self.ewma[i].update(n as usize);
        }

        self.update_snapshot(s);
    }

    fn tick(&mut self) {
        let s = self.data.lock().unwrap();

        for i in (0..WINDOW.len()) {
            self.ewma[i].tick();
        }

        self.update_snapshot(s);
    }

    /// Return the given EWMA for a rate like 1, 5, 15 minutes
    fn rate(&self, rate: f64) -> f64 {
        let s = self.data.lock().unwrap();

        if let Some(pos) = WINDOW.iter().position(|w| *w == rate) {
            let r: f64 = s.rates[pos];
            return r;
        }
        0f64
    }
    /// Return the mean rate
    fn mean(&self) -> f64 {
        let s = self.data.lock().unwrap();

        s.mean
    }

    fn count(&self) -> i64 {
        let s = self.data.lock().unwrap();

        s.count
    }
}

impl Metric for StdMeter {
    fn export_metric(&self) -> MetricValue {
        use metric::MetricValue::Meter;
        let snapshot: MeterSnapshot = self.snapshot();
        Meter(snapshot)
    }
}

impl StdMeter {
    fn update_snapshot(&self, mut s: MutexGuard<MeterSnapshot>) {
        for i in (0..WINDOW.len()) {
            s.rates[i] = self.ewma[i].rate();
        }

        let diff = get_time() - self.start;
        s.mean = s.count as f64 / diff.num_seconds() as f64;
    }

    pub fn new() -> StdMeter {
        let data: MeterSnapshot = MeterSnapshot { count: 0i64, rates: [0f64, 0f64, 0f64], mean: 0f64 };

        let ewma: [EWMA; 3] = [EWMA::new(1f64), EWMA::new(5f64), EWMA::new(15f64)];

        StdMeter { data: Mutex::new(data), ewma: ewma, start: get_time() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zero() {
        let m: StdMeter = StdMeter::new();
        let s: MeterSnapshot = m.snapshot();

        assert_eq!(s.count, 0);
    }

    #[test]
    fn non_zero() {
        let m: StdMeter = StdMeter::new();
        m.mark(3);

        let s: MeterSnapshot = m.snapshot();

        assert_eq!(s.count, 3);
    }

    #[test]
    fn snapshot() {
        let m: StdMeter = StdMeter::new();
        m.mark(1);
        m.mark(1);

        let s = m.snapshot();

        m.mark(1);

        assert_eq!(s.count, 2);
        assert_eq!(m.snapshot().count, 3);
    }

    // Test that decay works correctly
    #[test]
    fn decay() {
        let mut m: StdMeter = StdMeter::new();

        m.tick();
    }
}
