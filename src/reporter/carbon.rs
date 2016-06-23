// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// CarbonReporter sends a message to a carbon end point at a regular basis.
use registry::{Registry, StdRegistry};
use std::time::Duration;
use std::thread;
use std::sync::Arc;
use reporter::Reporter;
use metrics::{CounterSnapshot, GaugeSnapshot, MeterSnapshot};
use histogram::Histogram;
use time;
use time::Timespec;
use std::net::TcpStream;
use std::io::Write;
use std::io::Error;

struct CarbonStream {
    graphite_stream: Option<TcpStream>,
    host_and_port: String,
}

// TODO perhaps we autodiscover the host and port
//
pub struct CarbonReporter {
    host_and_port: String,
    prefix: &'static str,
    registry: Arc<StdRegistry<'static>>,
    reporter_name: &'static str,
}

impl CarbonStream {
    pub fn new(host_and_port: String) -> Self {
        CarbonStream {
            host_and_port: host_and_port,
            graphite_stream: None,
        }
    }

    pub fn connect(&mut self) -> Result<(), Error> {
        let graphite_stream = try!(TcpStream::connect(&*self.host_and_port));
        self.graphite_stream = Some(graphite_stream);
        Ok(())
    }

    pub fn write(&mut self,
                 metric_path: String,
                 value: String,
                 timespec: Timespec)
                 -> Result<(), Error> {
        let seconds_in_ms = (timespec.sec * 1000) as u32;
        let nseconds_in_ms = (timespec.nsec / 1000) as u32;
        let timestamp = seconds_in_ms + nseconds_in_ms;
        match self.graphite_stream {
            Some(ref mut stream) => {
                let carbon_command = format!("{} {} {}\n", metric_path, value, timestamp)
                    .into_bytes();
                try!(stream.write_all(&carbon_command));
            }
            None => {
                try!(self.reconnect_stream());
                try!(self.write(metric_path, value, timespec));
            }
        }
        Ok(())
    }
    fn reconnect_stream(&mut self) -> Result<(), Error> {
        // TODO 123 is made up
        println!("Waiting 123ms and then reconnecting");
        thread::sleep(Duration::from_millis(123));
        self.connect()
    }
}

impl Reporter for CarbonReporter {
    fn get_unique_reporter_name(&self) -> &'static str {
        self.reporter_name
    }
}

fn prefix(metric_line: String, prefix_str: &'static str) -> String {
    format!("{}.{}", prefix_str, metric_line)
}

fn send_meter_metric(metric_name: &str,
                     meter: MeterSnapshot,
                     carbon: &mut CarbonStream,
                     prefix_str: &'static str,
                     ts: Timespec)
                     -> Result<(), Error> {

    let count = meter.count.to_string();
    let m1_rate = meter.rates[0].to_string();
    let m5_rate = meter.rates[1].to_string();
    let m15_rate = meter.rates[2].to_string();
    let mean_rate = meter.mean.to_string();
    try!(carbon.write(prefix(format!("{}.count", metric_name), prefix_str),
                      count,
                      ts));
    try!(carbon.write(prefix(format!("{}.m1", metric_name), prefix_str),
                      m1_rate,
                      ts));
    try!(carbon.write(prefix(format!("{}.m5", metric_name), prefix_str),
                      m5_rate,
                      ts));
    try!(carbon.write(prefix(format!("{}.m15", metric_name), prefix_str),
                      m15_rate,
                      ts));
    try!(carbon.write(prefix(format!("{}.mean", metric_name), prefix_str),
                      mean_rate,
                      ts));
    Ok(())
}

fn send_gauge_metric(metric_name: &str,
                     gauge: GaugeSnapshot,
                     carbon: &mut CarbonStream,
                     prefix_str: &'static str,
                     ts: Timespec)
                     -> Result<(), Error> {
    try!(carbon.write(prefix(format!("{}", metric_name), prefix_str),
                      gauge.value.to_string(),
                      ts));
    Ok(())
}

fn send_counter_metric(metric_name: &str,
                       counter: CounterSnapshot,
                       carbon: &mut CarbonStream,
                       prefix_str: &'static str,
                       ts: Timespec)
                       -> Result<(), Error> {
    try!(carbon.write(prefix(format!("{}", metric_name), prefix_str),
                      counter.value.to_string(),
                      ts));
    Ok(())
}
fn send_histogram_metric(metric_name: &str,
                         histogram: &mut Histogram,
                         carbon: &mut CarbonStream,
                         prefix_str: &'static str,
                         ts: Timespec)
                         -> Result<(), Error> {
    let count = histogram.into_iter().count();
    // let sum = histogram.sum();
    // let mean = sum / count;
    let max = histogram.percentile(100.0).unwrap();
    let min = histogram.percentile(0.0).unwrap();

    let p50 = histogram.percentile(50.0).unwrap();
    let p75 = histogram.percentile(75.0).unwrap();
    let p95 = histogram.percentile(95.0).unwrap();
    let p98 = histogram.percentile(98.0).unwrap();
    let p99 = histogram.percentile(99.0).unwrap();
    let p999 = histogram.percentile(99.9).unwrap();
    let p9999 = histogram.percentile(99.99).unwrap();
    let p99999 = histogram.percentile(99.999).unwrap();

    try!(carbon.write(prefix(format!("{}.count", metric_name), prefix_str),
                      count.to_string(),
                      ts));

    try!(carbon.write(prefix(format!("{}.max", metric_name), prefix_str),
                      max.to_string(),
                      ts));

    // carbon
    // .write(prefix(format!("{}.mean", metric_name), prefix_str),
    // mean.into_string(),
    // ts);

    try!(carbon.write(prefix(format!("{}.min", metric_name), prefix_str),
                      min.to_string(),
                      ts));

    try!(carbon.write(prefix(format!("{}.p50", metric_name), prefix_str),
                      p50.to_string(),
                      ts));

    try!(carbon.write(prefix(format!("{}.p75", metric_name), prefix_str),
                      p75.to_string(),
                      ts));

    try!(carbon.write(prefix(format!("{}.p95", metric_name), prefix_str),
                      p95.to_string(),
                      ts));

    try!(carbon.write(prefix(format!("{}.p98", metric_name), prefix_str),
                      p98.to_string(),
                      ts));

    try!(carbon.write(prefix(format!("{}.p99", metric_name), prefix_str),
                      p99.to_string(),
                      ts));

    try!(carbon.write(prefix(format!("{}.p999", metric_name), prefix_str),
                      p999.to_string(),
                      ts));

    try!(carbon.write(prefix(format!("{}.p9999", metric_name), prefix_str),
                      p9999.to_string(),
                      ts));

    try!(carbon.write(prefix(format!("{}.p99999", metric_name), prefix_str),
                      p99999.to_string(),
                      ts));
    Ok(())
}

impl CarbonReporter {
    pub fn new(registry: Arc<StdRegistry<'static>>,
               reporter_name: &'static str,
               host_and_port: String,
               prefix: &'static str)
               -> Self {
        CarbonReporter {
            host_and_port: host_and_port,
            prefix: prefix,
            registry: registry,
            reporter_name: reporter_name,
        }
    }

    fn report_to_carbon_continuously(self, delay_ms: u64) -> thread::JoinHandle<Result<(), Error>> {
        use metrics::MetricValue::{Counter, Gauge, Histogram, Meter};

        let prefix = self.prefix;
        let host_and_port = self.host_and_port.clone();
        let mut carbon = CarbonStream::new(host_and_port);
        let registry = self.registry.clone();
        thread::spawn(move || {
            loop {
                let ts = time::now().to_timespec();
                for metric_name in &registry.get_metrics_names() {
                    let metric = registry.get(metric_name);
                    try!(match metric.export_metric() {
                        Meter(x) => send_meter_metric(metric_name, x, &mut carbon, prefix, ts),
                        Gauge(x) => send_gauge_metric(metric_name, x, &mut carbon, prefix, ts),
                        Counter(x) => send_counter_metric(metric_name, x, &mut carbon, prefix, ts),
                        Histogram(mut x) => {
                            send_histogram_metric(metric_name, &mut x, &mut carbon, prefix, ts)
                        }
                    });
                }
                thread::sleep(Duration::from_millis(delay_ms));
            }
        })
    }

    pub fn start(self, delay_ms: u64) {
        self.report_to_carbon_continuously(delay_ms);
    }
}

#[cfg(test)]
mod test {
    use histogram::*;
    use metrics::{Counter, Gauge, Meter, StdCounter, StdGauge, StdMeter};
    use registry::{Registry, StdRegistry};
    use std::sync::Arc;
    use super::CarbonReporter;

    #[test]
    fn meter() {
        let m = StdMeter::new();
        m.mark(100);

        let mut c = StdCounter::new();
        c.inc();

        let mut g = StdGauge::default();
        g.set(1.2);

        let mut h = Histogram::configure()
            .max_value(100)
            .precision(1)
            .build()
            .unwrap();

        h.increment_by(1, 1).unwrap();

        let mut r = StdRegistry::new();
        r.insert("meter1", m);
        r.insert("counter1", c);
        r.insert("gauge1", g);
        r.insert("histogram", h);

        let arc_registry = Arc::new(r);
        CarbonReporter::new(arc_registry.clone(),
                            "test",
                            "localhost:0".to_string(),
                            "asd.asdf");
    }
}
