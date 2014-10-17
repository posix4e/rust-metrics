use time::get_time;
use time::Timespec;

use std::sync::Mutex;

use ewma::EWMA;
use metric::Metric;


const WINDOW: [f64, ..3] = [1f64, 5f64, 15f64];


// A MeterSnapshot
pub struct MeterSnapshot {
    count: i64,
    rates: [f64, ..3],
    rate_mean: f64
}


// A StdMeter struct
pub struct StdMeter {
    data: Mutex<MeterSnapshot>,
    ewma: [EWMA, ..3],
    start: Timespec
}


// A Meter trait
pub trait Meter : Metric {
    fn snapshot(&self) -> MeterSnapshot;

    fn mark(&self, n: i64);

    fn tick(&mut self);

    fn rate(&self, rate: uint) -> f64;

    fn count(&self) -> i64;
}


impl Meter for StdMeter {
    fn snapshot(&self) -> MeterSnapshot {
        let s = self.data.lock();

        MeterSnapshot {
            count: s.count,
            rates: s.rates,
            rate_mean: s.rate_mean
        }
    }

    fn mark(&self, n: i64) {
        let mut s = self.data.lock();

        s.count += n;

        for i in range(0, WINDOW.len()) {
            self.ewma[i].update(n as uint);
        }

        self.update_snapshot(*s);
    }

    fn tick(&mut self) {
        let mut s = self.data.lock();

        for i in range(0, WINDOW.len()) {
            self.ewma[i].tick();
        }

        self.update_snapshot(*s);
    }

    fn rate(&self, rate: uint) -> f64 {
        let mut s = self.data.lock();
        s.rates[rate]
    }

    fn count(&self) -> i64 {
        let s = self.data.lock();
        s.count
    }
}


impl Metric for StdMeter {
}


impl StdMeter {
    fn update_snapshot(&self, mut s: MeterSnapshot) {
        for i in range(0, WINDOW.len()) {
            s.rates[i] = self.ewma[i].rate();
        }

        let diff = get_time() - self.start;
        s.rate_mean = s.count as f64 / diff.num_seconds() as f64;
    }

    pub fn new() -> StdMeter {
        let data = MeterSnapshot{
            count: 0i64,
            rates: [0f64, 0f64, 0f64],
            rate_mean: 0f64
        };

        let ewma: [EWMA, ..3] = [EWMA::new(1f64), EWMA::new(5f64), EWMA::new(15f64)];

        StdMeter {
            data: Mutex::new(data),
            ewma: ewma,
            start: get_time()
        }
    }

}


#[cfg(test)]
mod test {
    use meter::{Meter,MeterSnapshot,StdMeter};

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
}