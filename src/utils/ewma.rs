use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

/// An exponentially weighted moving average.
#[allow(missing_docs)]
#[derive(Debug)]
pub struct EWMA {
    uncounted: AtomicUsize, // This tracks uncounted events
    alpha: f64,
    rate: Mutex<f64>,
    init: bool,
}

#[allow(missing_docs)]
pub struct EWMASnapshot {
    value: f64,
}

#[allow(missing_docs)]
impl EWMASnapshot {
    pub fn rate(&self) -> f64 {
        self.value
    }
}

#[allow(missing_docs)]
impl EWMA {
    pub fn rate(&self) -> f64 {
        let r = self.rate.lock().unwrap();

        *r * (1.0e9)
    }

    pub fn snapshot(&self) -> EWMASnapshot {
        EWMASnapshot { value: self.rate() }
    }

    pub fn tick(&mut self) {
        let counter: usize = self.uncounted.load(Ordering::SeqCst);

        self.uncounted.fetch_sub(counter, Ordering::SeqCst); // Broken atm

        let mut rate = self.rate.lock().unwrap();
        let i_rate = (counter as f64) / (5e9);

        if self.init {
            *rate += self.alpha * (i_rate - *rate);
        } else {
            self.init = true;
            *rate = i_rate;
        }
    }

    pub fn update(&self, n: usize) {
        self.uncounted.fetch_add(n, Ordering::SeqCst);
    }

    /// construct new by alpha
    pub fn new_by_alpha(alpha: f64) -> Self {
        EWMA {
            uncounted: AtomicUsize::new(0),
            alpha: alpha,
            rate: Mutex::new(0.0),
            init: false,
        }
    }

    /// constructs a new EWMA for a n-minute moving average.
    pub fn new(rate: f64) -> Self {
        let i = -5.0 / 60.0 / rate;
        EWMA::new_by_alpha(1.0 - i.exp())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Tick a minute
    fn elapse_minute(e: &mut EWMA) {
        for _ in 0..12 {
            e.tick();
        }
    }

    // Returns whether the rate() is within 0.0001 of expected after ticking a minute
    fn within(e: &mut EWMA, expected: f64) -> bool {
        elapse_minute(e);
        let r = e.rate();
        (r - expected).abs() < 0.0001
    }

    #[test]
    fn ewma1() {
        let mut e = EWMA::new(1.0);
        e.update(3);
        e.tick();

        // initial
        let r = e.rate();
        assert_eq!(r, 0.6);

        // 1 minute
        assert_eq!(within(&mut e, 0.22072766470286553), true);

        // 2 minute
        assert_eq!(within(&mut e, 0.08120116994196772), true);

        // 3 minute
        assert_eq!(within(&mut e, 0.029872241020718428), true);

        // 4 minute
        assert_eq!(within(&mut e, 0.01098938333324054), true);

        // 5 minute
        assert_eq!(within(&mut e, 0.004042768199451294), true);

        // 6 minute
        assert_eq!(within(&mut e, 0.0014872513059998212), true);

        // 7 minute
        assert_eq!(within(&mut e, 0.0005471291793327122), true);

        // 8 minute
        assert_eq!(within(&mut e, 0.00020127757674150815), true);

        // 9 minute
        assert_eq!(within(&mut e, 7.404588245200814e-05), true);

        // 10 minute
        assert_eq!(within(&mut e, 2.7239957857491083e-05), true);

        // 11 minute
        assert_eq!(within(&mut e, 1.0021020474147462e-05), true);

        // 12 minute
        assert_eq!(within(&mut e, 3.6865274119969525e-06), true);

        // 13 minute
        assert_eq!(within(&mut e, 1.3561976441886433e-06), true);

        // 14 minute
        assert_eq!(within(&mut e, 4.989172314621449e-07), true);

        // 15 minute
        assert_eq!(within(&mut e, 1.8354139230109722e-07), true);
    }

    #[test]
    fn ewma5() {
        let mut e = EWMA::new(5.0);
        e.update(3);
        e.tick();

        let r = e.rate();
        assert_eq!(r, 0.6);

        // 1 minute
        assert_eq!(within(&mut e, 0.49123845184678905), true);

        // 2 minute
        assert_eq!(within(&mut e, 0.4021920276213837), true);

        // 3 minute
        assert_eq!(within(&mut e, 0.32928698165641596), true);

        // 4 minute
        assert_eq!(within(&mut e, 0.269597378470333), true);

        // 5 minute
        assert_eq!(within(&mut e, 0.2207276647028654), true);

        // 6 minute
        assert_eq!(within(&mut e, 0.18071652714732128), true);

        // 7 minute
        assert_eq!(within(&mut e, 0.14795817836496392), true);

        // 8 minute
        assert_eq!(within(&mut e, 0.12113791079679326), true);

        // 9 minute
        assert_eq!(within(&mut e, 0.09917933293295193), true);

        // 10 minute
        assert_eq!(within(&mut e, 0.08120116994196763), true);

        // 11 minute
        assert_eq!(within(&mut e, 0.06648189501740036), true);

        // 12 minute
        assert_eq!(within(&mut e, 0.05443077197364752), true);

        // 13 minute
        assert_eq!(within(&mut e, 0.04456414692860035), true);

        // 14 minute
        assert_eq!(within(&mut e, 0.03648603757513079), true);

        // 15 minute
        assert_eq!(within(&mut e, 0.0298722410207183831020718428), true);
    }

    #[test]
    fn ewma15() {
        let mut e = EWMA::new(15.0);
        e.update(3);
        e.tick();

        let r = e.rate();
        assert_eq!(r, 0.6);

        // 1 minute
        assert_eq!(within(&mut e, 0.5613041910189706), true);

        // 2 minute
        assert_eq!(within(&mut e, 0.5251039914257684), true);

        // 3 minute
        assert_eq!(within(&mut e, 0.4912384518467888184678905), true);

        // 4 minute
        assert_eq!(within(&mut e, 0.459557003018789), true);

        // 5 minute
        assert_eq!(within(&mut e, 0.4299187863442732), true);

        // 6 minute
        assert_eq!(within(&mut e, 0.4021920276213831), true);

        // 7 minute
        assert_eq!(within(&mut e, 0.37625345116383313), true);

        // 8 minute
        assert_eq!(within(&mut e, 0.3519877317060185), true);

        // 9 minute
        assert_eq!(within(&mut e, 0.3292869816564153165641596), true);

        // 10 minute
        assert_eq!(within(&mut e, 0.3080502714195546), true);

        // 11 minute
        assert_eq!(within(&mut e, 0.2881831806538789), true);

        // 12 minute
        assert_eq!(within(&mut e, 0.26959737847033216), true);

        // 13 minute
        assert_eq!(within(&mut e, 0.2522102307052083), true);

        // 14 minute
        assert_eq!(within(&mut e, 0.23594443252115815), true);

        // 15 minute
        assert_eq!(within(&mut e, 0.2207276647028646247028654470286553), true);
    }
}
