#![deny(trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]

extern crate time;
extern crate histogram;
extern crate iron;
extern crate router;
extern crate persistent;

pub mod metrics;
pub mod registry;
pub mod reporter;
pub mod utils;

extern crate protobuf; // depend on rust-protobuf runtime
#[allow(unsafe_code)]
mod promo_proto; // add generated crate
