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
use std::thread::JoinHandle;
use super::metrics::Metric;
use std::collections::HashMap;

// Todo create sync wrappers with mutexes.
// Currently our only reporter runs as a seperate thread so stop returns its handler
// In future versions we wont be so specific

enum ReporterMsg {
    AddMetric(String, Metric, Option<HashMap<String, String>>),
    RemoveMetric(String),
}

pub trait Reporter: Send {
    fn get_unique_reporter_name(&self) -> &str;
    fn stop(self) -> Result<JoinHandle<Result<(), String>>, String>;

    fn addl<S: Into<String>>(&mut self,
                             name: S,
                             metric: Metric,
                             labels: Option<HashMap<String, String>>)
                             -> Result<(), String>;
    // This will be added once it is implemented for prometheus
    // fn remove <S: Into<String>>(&mut self, name: S) -> Result<(), String>;

    fn add<S: Into<String>>(&mut self, name: S, metric: Metric) -> Result<(), String> {
        self.addl(name, metric, None)
    }
}
