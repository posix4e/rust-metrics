mod carbon;
mod console;
mod prometheus;

pub use self::carbon::CarbonReporter;
pub use self::console::ConsoleReporter;
pub use self::prometheus::PrometheusReporter;

pub trait Reporter: Send + Sync {
    fn get_unique_reporter_name(&self) -> &'static str;
}
