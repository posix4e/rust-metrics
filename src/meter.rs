use ewma::EWMA;

struct Meter {
    value: i64,
    ewma1: EWMA
}


impl Meter {
}

fn new() -> Meter {
    let i = -5.0f64/60.0f64/1f64;

    return Meter {
        value: 0,
        ewma1: EWMA::new(1f64 - i)
    }
}