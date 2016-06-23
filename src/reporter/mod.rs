// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Report metrics to a collector

#![allow(missing_docs)]

mod carbon;
mod console;
mod prometheus;

pub use self::carbon::CarbonReporter;
pub use self::console::ConsoleReporter;
pub use self::prometheus::PrometheusReporter;

pub trait Reporter: Send + Sync {
    fn get_unique_reporter_name(&self) -> &'static str;
}
