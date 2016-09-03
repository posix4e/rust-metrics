/// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// CarbonReporter sends a message to a carbon end point at a regular basis.
use std::time::Duration;
use std::thread;
use reporter::Reporter;
use metrics::{CounterSnapshot, GaugeSnapshot, MeterSnapshot, Metric};
use histogram::Histogram;
use time;
use time::Timespec;
use std::io::Write;
use std::io::Error;
use std::sync::mpsc;
use std::net::TcpStream;
use std::collections::HashMap;

struct CarbonMetricEntry {
    metric_name: String,
    metric: Metric,
}

struct CarbonStream {
    graphite_stream: Option<TcpStream>,
    host_and_port: String,
}

// TODO perhaps we autodiscover the host and port
//
pub struct CarbonReporter {
    metrics: mpsc::Sender<Result<CarbonMetricEntry, &'static str>>,
    reporter_name: String,
    join_handle: thread::JoinHandle<Result<(), String>>,
}

impl CarbonStream {
    pub fn new<S: Into<String>>(host_and_port: S) -> Self {
        CarbonStream {
            host_and_port: host_and_port.into(),
            graphite_stream: None,
        }
    }

    pub fn connect(&mut self) -> Result<(), Error> {
        let graphite_stream = try!(TcpStream::connect(&(*self.host_and_port)));
        self.graphite_stream = Some(graphite_stream);
        Ok(())
    }

    pub fn write<S: Into<String>>(&mut self,
                                  metric_path: S,
                                  value: S,
                                  timespec: Timespec)
                                  -> Result<(), Error> {
        let seconds_in_ms = (timespec.sec * 1000) as u32;
        let nseconds_in_ms = (timespec.nsec / 1000) as u32;
        let timestamp = seconds_in_ms + nseconds_in_ms;
        match self.graphite_stream {
            Some(ref mut stream) => {
                let carbon_command =
                    format!("{} {} {}\n", metric_path.into(), value.into(), timestamp).into_bytes();
                try!(stream.write_all(&carbon_command));
            }
            None => {
                try!(self.reconnect_stream());
                try!(self.write(metric_path.into(), value.into(), timespec));
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
    fn get_unique_reporter_name(&self) -> &str {
        &self.reporter_name
    }
    fn stop(self) -> Result<thread::JoinHandle<Result<(), String>>, String> {
        match self.metrics.send(Err("stop")) {
            Ok(_) => Ok(self.join_handle),
            Err(x) => Err(format!("Unable to stop reporter {}", x)),
        }
    }
    fn addl<S: Into<String>>(&mut self,
                             name: S,
                             metric: Metric,
                             labels: Option<HashMap<String, String>>)
                             -> Result<(), String> {
        // Todo maybe do something about the labels
        match self.metrics
            .send(Ok(CarbonMetricEntry {
                metric_name: name.into(),
                metric: metric,
            })) {
            Ok(_) => Ok(()),
            Err(x) => Err(format!("Unable to send metric reporter{}", x)),
        }
    }
}

fn prefix(metric_line: String, prefix_str: &str) -> String {
    format!("{}.{}", prefix_str, metric_line)
}

fn send_meter_metric(metric_name: &str,
                     meter: MeterSnapshot,
                     carbon: &mut CarbonStream,
                     prefix_string: String,
                     ts: Timespec)
                     -> Result<(), Error> {
    let prefix_str = &(*prefix_string);

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
                     prefix_string: String,
                     ts: Timespec)
                     -> Result<(), Error> {
    let prefix_str = &(*prefix_string);
    try!(carbon.write(prefix(format!("{}", metric_name), prefix_str),
                      gauge.value.to_string(),
                      ts));
    Ok(())
}

fn send_counter_metric(metric_name: &str,
                       counter: CounterSnapshot,
                       carbon: &mut CarbonStream,
                       prefix_string: String,
                       ts: Timespec)
                       -> Result<(), Error> {
    let prefix_str = &(*prefix_string);
    try!(carbon.write(prefix(format!("{}", metric_name), prefix_str),
                      counter.value.to_string(),
                      ts));
    Ok(())
}
fn send_histogram_metric(metric_name: &str,
                         histogram: &Histogram,
                         carbon: &mut CarbonStream,
                         prefix_string: String,
                         ts: Timespec)
                         -> Result<(), Error> {
    let prefix_str = &(*prefix_string);
    let count = histogram.into_iter().count();
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
    pub fn new<S1: Into<String>, S2: Into<String>, S3: Into<String>>(reporter_name: S1,
                                                                     host_and_port: S2,
                                                                     prefix: S3,
                                                                     aggregation_timer: u64)
                                                                     -> Self {
        let (tx, rx) = mpsc::channel();
        let hp = host_and_port.into();
        let pr = prefix.into();
        let rn = reporter_name.into();

        CarbonReporter {
            metrics: tx,
            reporter_name: rn.clone(),
            join_handle: report_to_carbon_continuously(pr, hp, aggregation_timer, rx),
        }
    }
}
fn report_to_carbon_continuously(prefix: String,
                                 host_and_port: String,
                                 delay_ms: u64,
                                 rx: mpsc::Receiver<Result<CarbonMetricEntry, &'static str>>)
                                 -> thread::JoinHandle<Result<(), String>> {
    thread::spawn(move || {
        let mut carbon = CarbonStream::new(host_and_port);
        let mut stop = false;

        while !stop {
            let ts = time::now().to_timespec();

            for entry in &rx {
                match entry {
                    Ok(entry) => {
                        let metric_name = &entry.metric_name;
                        let metric = &entry.metric;
                        // Maybe one day we can do more to handle this failure
                        match *metric {
                            Metric::Meter(ref x) => {
                                send_meter_metric(metric_name,
                                                  x.snapshot(),
                                                  &mut carbon,
                                                  prefix.clone(),
                                                  ts)
                            }
                            Metric::Gauge(ref x) => {
                                send_gauge_metric(metric_name,
                                                  x.snapshot(),
                                                  &mut carbon,
                                                  prefix.clone(),
                                                  ts)
                            }
                            Metric::Counter(ref x) => {
                                send_counter_metric(metric_name,
                                                    x.snapshot(),
                                                    &mut carbon,
                                                    prefix.clone(),
                                                    ts)
                            }
                            Metric::Histogram(ref x) => {
                                send_histogram_metric(metric_name,
                                                      &x,
                                                      &mut carbon,
                                                      prefix.clone(),
                                                      ts)
                            }
                        };
                        // TODO handle errors somehow
                        thread::sleep(Duration::from_millis(delay_ms));
                    }
                    Err(_) => stop = true,
                }
            }
        }
        Ok(())
    })
}



#[cfg(test)]
mod test {
    use histogram::Histogram;
    use metrics::{Counter, Gauge, Meter, Metric, StdCounter, StdGauge, StdMeter};
    use std::net::TcpListener;
    use super::CarbonReporter;
    use reporter::Reporter;

    #[test]
    fn meter() {
        let m = StdMeter::new();
        m.mark(100);

        let c = StdCounter::new();
        c.inc();

        let g = StdGauge::new();
        g.set(2);

        let mut h = Histogram::configure()
            .max_value(100)
            .precision(1)
            .build()
            .unwrap();

        h.increment_by(1, 1).unwrap();

        let test_port = "127.0.0.1:34254";
        let listener = TcpListener::bind(test_port).unwrap();
        let mut reporter = CarbonReporter::new("test", test_port, "asd.asdf", 1024);
        reporter.add("meter1", Metric::Meter(m.clone()));
        reporter.add("counter1", Metric::Counter(c.clone()));
        reporter.add("gauge1", Metric::Gauge(g.clone()));
        reporter.add("histogram", Metric::Histogram(h));
        reporter.stop().unwrap().join().unwrap().unwrap();
    }
}
