// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]

use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use utils::{EWMA, TICK_RATE_SECS};

const NANOS_PER_SEC: u64 = 1_000_000_000;
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
    ewma: [EWMA; 3],
    next_tick: Instant,
}

// A StdMeter struct
#[derive(Debug)]
pub struct StdMeter {
    data: Mutex<StdMeterData>,
    start: Instant,
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
        let mut s = self.data.lock().unwrap();
        self.tick_inner(&mut s);

        MeterSnapshot {
            count: s.count,
            rates: [s.ewma[0].rate(), s.ewma[1].rate(), s.ewma[2].rate()],
            mean: self.mean_inner(&s),
        }
    }

    fn mark(&self, n: i64) {
        let mut s = self.data.lock().unwrap();
        self.tick_inner(&mut s);

        s.count += n;

        for i in 0..WINDOW.len() {
            s.ewma[i].update(n as usize);
        }
    }

    fn tick(&self) {
        let mut s = self.data.lock().unwrap();
        self.tick_inner(&mut s);
    }

    /// Return the given EWMA for a rate like 1, 5, 15 minutes
    fn rate(&self, rate: f64) -> f64 {
        let mut s = self.data.lock().unwrap();
        self.tick_inner(&mut s);

        if let Some(pos) = WINDOW.iter().position(|w| *w == rate) {
            return s.ewma[pos].rate();
        }
        0.0
    }

    /// Return the mean rate
    fn mean(&self) -> f64 {
        let s = self.data.lock().unwrap();
        self.mean_inner(&s)
    }

    fn count(&self) -> i64 {
        let s = self.data.lock().unwrap();

        s.count
    }
}

impl StdMeter {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    fn mean_inner(&self, s: &StdMeterData) -> f64 {
        if s.count == 0 {
            0.
        } else {
            let dur = self.start.elapsed();
            let nanos = dur.as_secs() * NANOS_PER_SEC + dur.subsec_nanos() as u64;
            s.count as f64 / nanos as f64 * NANOS_PER_SEC as f64
        }
    }

    fn tick_inner(&self, s: &mut StdMeterData) {
        let now = Instant::now();

        while s.next_tick <= now {
            for ewma in &mut s.ewma {
                ewma.tick();
            }
            s.next_tick += Duration::from_secs(TICK_RATE_SECS);
        }
    }
}

impl Default for StdMeter {
    fn default() -> Self {
        let now = Instant::now();
        StdMeter {
            data: Mutex::new(StdMeterData {
                count: 0,
                ewma: [EWMA::new(1.0), EWMA::new(5.0), EWMA::new(15.0)],
                next_tick: now + Duration::from_secs(TICK_RATE_SECS),
            }),
            start: now,
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
