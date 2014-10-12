extern crate metrics;

use metrics::gauge::Gauge;


fn main() {
    // Create a metric w val 0
    let g1: Gauge = Gauge{value: 0};
    println!("{}", g1.value);

    let g2: Gauge = g1.snapshot();
    // Get a snapshot of it to g2
    g2.update(10i64);

    println!("{}", g1.value);
}