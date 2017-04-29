// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]

use std::sync::{Arc, Mutex, MutexGuard};
use time::{get_time, Timespec};
use utils::EWMA;

const WINDOW: [f64; 3] = [1.0, 5.0, 15.0];

// A MeterSnapshot
#[derive(Debug)]
pub struct MeterSnapshot {
    pub count: i64,
    pub rates: [f64; 3],
    pub mean: f64,
}

#[derive(Debug)]
struct StdMeterData {
    count: i64,
    rates: [f64; 3],
    mean: f64,
    ewma: [EWMA; 3],
}

// A StdMeter struct
#[derive(Debug)]
pub struct StdMeter {
    data: Mutex<StdMeterData>,
    start: Timespec,
}

// A Meter trait
pub trait Meter: Send + Sync {
    fn snapshot(&self) -> MeterSnapshot;

    fn mark(&self, n: i64);

    fn tick(&self);

    fn rate(&self, rate: f64) -> f64;

    fn mean(&self) -> f64;

    fn count(&self) -> i64;
}

impl Meter for StdMeter {
    fn snapshot(&self) -> MeterSnapshot {
        let s = self.data.lock().unwrap();

        MeterSnapshot {
            count: s.count,
            rates: s.rates,
            mean: s.mean,
        }
    }

    fn mark(&self, n: i64) {
        let mut s = self.data.lock().unwrap();

        s.count += n;

        for i in 0..WINDOW.len() {
            s.ewma[i].update(n as usize);
        }

        self.update_data(s);
    }

    fn tick(&self) {
        let mut s = self.data.lock().unwrap();

        for i in 0..WINDOW.len() {
            s.ewma[i].tick();
        }

        self.update_data(s);
    }

    /// Return the given EWMA for a rate like 1, 5, 15 minutes
    fn rate(&self, rate: f64) -> f64 {
        let s = self.data.lock().unwrap();

        if let Some(pos) = WINDOW.iter().position(|w| *w == rate) {
            return s.rates[pos];
        }
        0.0
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

impl StdMeter {
    fn update_data(&self, mut s: MutexGuard<StdMeterData>) {
        for i in 0..WINDOW.len() {
            s.rates[i] = s.ewma[i].rate();
        }

        let diff = get_time() - self.start;
        let num_secs = diff.num_seconds();
        if num_secs == 0 {
            s.mean = 0.
        } else {
            s.mean = s.count as f64 / num_secs as f64;
        }

    }

    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }
}

impl Default for StdMeter {
    fn default() -> Self {
        StdMeter {
            data: Mutex::new(StdMeterData {
                count: 0,
                rates: [0.0, 0.0, 0.0],
                mean: 0.0,
                ewma: [EWMA::new(1.0), EWMA::new(5.0), EWMA::new(15.0)],
            }),
            start: get_time(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zero() {
        let m = StdMeter::new();
        let s = m.snapshot();

        assert_eq!(s.count, 0);
    }

    #[test]
    fn non_zero() {
        let m = StdMeter::new();
        m.mark(3);

        let s = m.snapshot();

        assert_eq!(s.count, 3);
    }

    #[test]
    fn snapshot() {
        let m = StdMeter::new();
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
        let m = StdMeter::new();

        m.tick();
    }
}
