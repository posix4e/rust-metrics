// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Metrics
//!
//! Metrics are things you can use to safely & directly store metrics with
//! little overhead. Metrics can be attached to a reporter.
//!
//! Current reporters include:
//!
//! - Graphite/Carbon/Whisper
//! - Console/Syslog/Journald (via stdout)

#![warn(missing_docs)]
#![deny(trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]

extern crate time;
extern crate histogram;

pub mod metrics;
pub mod reporter;
pub mod utils;
