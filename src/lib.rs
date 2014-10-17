#![crate_name = "metrics"]

#![feature(if_let)]

extern crate time;

#[cfg(test)] extern crate test;
pub mod counter;
pub mod gauge;
pub mod ewma;
pub mod meter;
pub mod metric;
