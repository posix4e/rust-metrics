#![crate_type = "bin"]

extern crate metrics;

use metrics::{counter,gauge};


fn main() {
    // Create a metric w val 0
    let mut g1: gauge::Gauge = gauge::Gauge{value: 0};
    println!("{}", g1.value);
    g1.update(100);

    // Get a snapshot of it to g2
    let g2: gauge::Gauge = g1.snapshot();

    // Update g1 to 200
    g1.update(200);

    println!("g1 {} g2 {}", g1.value, g2.value);

    println!("{}", g1.value);

    let mut c1 = counter::Counter{value: 0};
    c1.inc(1);
    c1.inc(5);
    println!("{}", c1.value)
}