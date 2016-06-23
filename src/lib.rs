//! Metrics
//!
//! Metrics are things you can use to safely & directly store metrics with
//! little overhead. Metrics can be attached to a registry and that registry
//! can be collected across a system. This registry also provides reporting
//! services. Current reporters include:
//!
//! - [Prometheus](https://prometheus.io/)
//! - Graphite/Carbon/Whisper
//! - Console/Syslog/Journald (via stdout)

#![warn(missing_docs)]
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
