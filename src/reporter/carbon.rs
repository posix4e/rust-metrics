/// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// CarbonReporter sends a message to a carbon end point at a regular basis.
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
        if self.graphite_stream.is_none() {
            try!(self.connect());
        }
        if let Some(ref mut stream) = self.graphite_stream {
            let carbon_command =
                format!("{} {} {}\n", metric_path.into(), value.into(), timespec.sec).into_bytes();
            try!(stream.write_all(&carbon_command));
        }
        Ok(())
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
                             _labels: Option<HashMap<String, String>>)
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
        let mut metrics = vec!();

        while !stop {
            while let Ok(msg) = rx.try_recv() {
                match msg {
                    Ok(metric) => metrics.push(metric),
                    Err(_) => stop = true,
                }
            }
            let ts = time::get_time();
            let delay_ms = delay_ms as i64;
            let next_tick_ms = ((ts.sec * 1000 + ts.nsec as i64 / 1_000_000)/ delay_ms + 1) * delay_ms;
            let next_tick = Timespec {
                sec: (next_tick_ms / 1000),
                nsec: ((next_tick_ms % 1000) * 1_000_000) as i32,
            };
            thread::sleep((next_tick - ts).to_std().unwrap());
            for entry in &metrics {
                let metric_name = &entry.metric_name;
                let metric = &entry.metric;
                // Maybe one day we can do more to handle this failure
                let result = match *metric {
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
                // if an error happens, just stop and wait for next loop.
                if let Err(_) = result {
                    break;
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
    use std::collections::HashSet;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::net::TcpListener;
    use std::thread;
    use std::time::Duration;
    use super::CarbonReporter;
    use reporter::Reporter;
    use time;

    #[test]
    fn reporter() {
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

        let test_host_and_port = "127.0.0.1:34254";
        let listener = TcpListener::bind(test_host_and_port).unwrap();
        let mut reporter = CarbonReporter::new("test", test_host_and_port, "asd.asdf", 1000);
        reporter.add("meter1", Metric::Meter(m.clone())).unwrap();
        reporter.add("counter1", Metric::Counter(c.clone())).unwrap();
        reporter.add("gauge1", Metric::Gauge(g.clone())).unwrap();
        reporter.add("histogram", Metric::Histogram(h)).unwrap();

        let stream = listener.incoming().next().expect("client did not show up").unwrap();
        let buffer = BufReader::new(stream);
        thread::sleep(Duration::from_secs(2));
        reporter.stop().unwrap().join().unwrap().unwrap();

        let lines:Vec<String> = buffer.lines().map(|l| l.unwrap()).collect();
        let now = time::get_time();

        // graphite protocol is: "prefix.and.value.name <value> <tm is secs>\n"
        for line in &lines {
            let line = line.to_string();
            let tokens:Vec<&str> = line.split(" ").collect();
            assert_eq!(tokens.len(), 3);

            assert!(tokens[1].parse::<f64>().is_ok());

            let ts:isize = tokens[2].parse().unwrap();
            assert!((ts-now.sec as isize).abs() < 10);
        }

        let metrics_seen:HashSet<String> = lines.iter().map(|x| x.split(" ").next().unwrap().into()).collect();
        assert!(metrics_seen.iter().all(|m| m.starts_with("asd.asdf")));
        assert!(metrics_seen.contains("asd.asdf.gauge1"));
        assert!(metrics_seen.contains("asd.asdf.counter1"));
        assert!(metrics_seen.contains("asd.asdf.meter1.count"));
        assert!(metrics_seen.contains("asd.asdf.histogram.p95"));
    }
}
