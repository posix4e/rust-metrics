use time::get_time;
use time::Timespec;

use std::sync::Mutex;

use ewma::EWMA;

pub struct MeterSnapshot{
    count: i64,
    rate1: f64,
    rate5: f64,
    rate15: f64,
    rate_mean: f64
}

pub struct Meter {
    data: Mutex<MeterSnapshot>,
    a1: EWMA,
    a5: EWMA,
    a15: EWMA,
    start: Timespec
}

impl Meter {
    pub fn mark(&self, n: i64) {
        let mut s = self.data.lock();
        s.count += n;
        self.a1.update(n as uint);
        self.a5.update(n as uint);
        self.a15.update(n as uint);
    }

    pub fn snapshot(&self) -> MeterSnapshot{
        let s = self.data.lock();
        MeterSnapshot {
            count: s.count,
            rate1: s.rate1,
            rate5: s.rate5,
            rate15: s.rate15,
            rate_mean: s.rate_mean
        }
    }

    fn update_snapshot(&self) {
        let mut s = self.data.lock();
        s.rate1 = self.a1.rate();
        s.rate5 = self.a5.rate();
        s.rate15 = self.a15.rate();

        let diff = get_time() - self.start;
        s.rate_mean = s.count as f64 / diff.num_seconds() as f64;
    }

    pub fn tick(&mut self) {
        self.data.lock();
        self.a1.tick();
        self.a5.tick();
        self.a15.tick();
        self.update_snapshot()
    }

    pub fn new() -> Meter {
        let i = -5.0f64/60.0f64/1f64;

        let data = MeterSnapshot{
            count: 0i64,
            rate1: 0f64,
            rate5: 0f64,
            rate15: 0f64,
            rate_mean: 0f64
        };

        Meter {
            data: Mutex::new(data),
            a1: EWMA::new(1f64 - i),
            a5: EWMA::new(5f64 - i),
            a15: EWMA::new(15f64 - i),
            start: get_time()
        }
    }
}

#[cfg(test)]
mod test {
    use meter::{Meter,MeterSnapshot};

    #[test]
    fn zero() {
        let m: Meter = Meter::new();
        let s: MeterSnapshot = m.snapshot();
        assert_eq!(s.count, 0);
    }

    #[test]
    fn non_zero() {
        let m: Meter = Meter::new();
        m.mark(3);
        let s: MeterSnapshot = m.snapshot();
        assert_eq!(s.count, 3);
    }

    #[test]
    fn snapshot() {
        let m: Meter = Meter::new();
        m.mark(1);
        m.mark(1);

        let s = m.snapshot();

        m.mark(1);

        assert_eq!(s.count, 2);
        assert_eq!(m.snapshot().count, 3);
    }
}