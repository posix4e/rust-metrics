// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Report metrics to a collector

#![allow(missing_docs)]

mod carbon;
mod console;

pub use self::carbon::CarbonReporter;
pub use self::console::ConsoleReporter;

#[cfg(feature = "prometheus")]
mod prometheus;

#[cfg(feature = "prometheus")]
pub use self::prometheus::PrometheusReporter;

// Todo create sync wrappers with mutexes.
pub trait Reporter: Send {
    fn get_unique_reporter_name(&self) -> &str;
    fn stop(&mut self);
}
