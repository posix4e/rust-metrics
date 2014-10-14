use std::sync::{Mutex, Arc};
use std::sync::atomics::{AtomicUint, SeqCst};


pub struct EWMA {
    pub uncounted: AtomicUint, // This tracks uncounted events
    alpha: f64,
    rate: Mutex<f64>,
    init: bool,
}


pub struct EWMASnapshot {
    value: f64
}


impl EWMASnapshot {
    fn rate(&self) -> f64 {
        return self.value;
    }
}

impl EWMA {
    pub fn rate(&self) -> f64 {
        let r = self.rate.lock();
        *r * (1e9 as f64)
    }

    pub fn snapshot(&self) -> EWMASnapshot {
        return EWMASnapshot{ value: self.rate() };
    }

    pub fn tick(&mut self) {
        let counter: uint = self.uncounted.load(SeqCst);

        self.uncounted.fetch_sub(counter, SeqCst); // Broken atm

        let mut rate = self.rate.lock();
        let i_rate = (counter as f64) / (5e9);

        if self.init {
            *rate = self.alpha * (i_rate - *rate);
        } else {
            self.init = true;
            *rate = i_rate;
        }

        rate.cond.signal();
    }

    pub fn update(&self, n: uint) {
        self.uncounted.fetch_add(n, SeqCst);
    }
}

// Construct a new EWMA with a mutex etc.
pub fn new() -> EWMA {
    return EWMA{
        uncounted: AtomicUint::new(0u),
        alpha: 0f64,
        rate: Mutex::new(0f64),
        init: false,
    }
}

#[cfg(test)]
mod test {
    use ewma;

    fn elapse_minute(mut e: ewma::EWMA) {
        for i in range(0i, 12i) {
            e.tick();
        }
    }

    #[test]
    fn ewma1() {
        let mut e = ewma::new();
        e.update(3u);
        e.tick();

        let mut r: f64;

        r = e.rate();
        assert_eq!(r, 0.6f64);

        elapse_minute(e);
        r = e.rate();
        assert_eq!(r, 0.22072766470286553)
    }
}