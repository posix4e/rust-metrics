use metric::Metric;
use registry::{Registry, StdRegistry};
use std::thread;
use std::sync::Arc;
use meter::Meter;
use reporter::Reporter;
use counter::StdCounter;
use gauge::StdGauge;
use meter::MeterSnapshot;
use histogram::Histogram;
use carbon_sender::Carbon;
pub struct CarbonReporter {
    hostname: &'static str,
    port: u16,
    prefix: &'static str,
    registry: Arc<StdRegistry<'static>>,
    reporter_name: &'static str
}

impl Reporter for CarbonReporter {
    fn report<'report>(&self, delay_ms: u32) {
        use metric::MetricValue::{Counter, Gauge, Histogram, Meter};

        let prefix = self.prefix;
        let mut carbon = Carbon::new(self.hostname, self.port);
        let registry = self.registry.clone();
        thread::spawn(move || {
                               loop {
                                   let ts = 0;
                                   for metric_name in &registry.get_metrics_names() {
                                       let metric = registry.get(metric_name);
                                       let cloned_metric_name = metric_name.clone();
                                       let mnas = metric_name.to_string(); // Metric name as string
                                       match metric.export_metric() {
                                           Meter(x) => send_meter_metric(mnas, x, & mut carbon,  prefix, ts),
                                           Gauge(x) => send_gauge_metric(mnas, x, & mut carbon,  prefix, ts),
                                           Counter(x) => send_counter_metric(mnas, x, & mut carbon, prefix, ts),
                                           Histogram(mut x) => send_histogram_metric(mnas, & mut x, & mut carbon,  prefix, ts),
                                       }
                                   }
                                   thread::sleep_ms(delay_ms);
                               }
                           });
    }

    fn get_unique_reporter_name(&self) -> &'static str {
        self.reporter_name
    }
}

fn prefix(metric_line: String, prefix_str: & 'static str) -> String {
        format!("{}.{}", prefix_str, metric_line)
}

fn send_meter_metric( metric_name: String,
    meter: MeterSnapshot,
     carbon:&mut Carbon,
     prefix_str: & 'static str,
     ts: u32) {

    let count = meter.count.to_string();
    let m1_rate = meter.rates[0].to_string();
    let m5_rate = meter.rates[1].to_string();
    let m15_rate = meter.rates[2].to_string();
    let mean_rate = meter.mean.to_string();
    carbon.write(prefix(format!("{}.count", metric_name), prefix_str), count, ts);
    carbon.write(prefix(format!("{}.m1", metric_name), prefix_str), m1_rate, ts);
    carbon.write(prefix(format!("{}.m5", metric_name), prefix_str), m5_rate, ts);
    carbon.write(prefix(format!("{}.m15", metric_name), prefix_str), m15_rate, ts);
    carbon.write(prefix(format!("{}.mean", metric_name), prefix_str), mean_rate, ts);

}

fn send_gauge_metric(metric_name: String,
     gauge: StdGauge,
     carbon:&mut Carbon,
     prefix_str: & 'static str,
     ts: u32) {
         carbon
         .write(prefix(format!("{}", metric_name), prefix_str),
         gauge.value.to_string(),
          ts);
}

fn send_counter_metric(metric_name: String,
    counter: StdCounter,
    carbon:& mut Carbon,
    prefix_str: & 'static str,
    ts: u32){
        carbon
        .write(prefix(format!("{}", metric_name), prefix_str),
        counter.value.to_string(),
         ts);
}
fn send_histogram_metric(metric_name: String,
    histogram:& mut Histogram,
    carbon:& mut Carbon,
    prefix_str: & 'static str,
    ts: u32) {
        let count = histogram.count();
        //let max = histogram.max().unwrap();
        //let sum = histogram.sum();
        //let mean = sum / count;
//        let min = histogram.min();

        let p50 = histogram.percentile(0.5).unwrap();
        let p75 = histogram.percentile(0.75).unwrap();
        let p95 = histogram.percentile(0.95).unwrap();
        let p98 = histogram.percentile(0.98).unwrap();
        let p99 = histogram.percentile(0.99).unwrap();
        let p999 = histogram.percentile(0.999).unwrap();
        let p9999 = histogram.percentile(0.9999).unwrap();
        let p99999 = histogram.percentile(0.99999).unwrap();

        carbon
        .write(prefix(format!("{}.count", metric_name), prefix_str),
        count.to_string(),
         ts);

        // carbon
         //.write(prefix(format!("{}.max", metric_name), prefix_str),
         //max.to_string(),
         // ts);

          //carbon
          //.write(prefix(format!("{}.mean", metric_name), prefix_str),
          //mean.into_string(),
          // ts);

           //carbon
           //.write(prefix(format!("{}.min", metric_name), prefix_str),
           //min.to_string(),
//            ts);

            carbon
            .write(prefix(format!("{}.p50", metric_name), prefix_str),
            p50.to_string(),
             ts);

             carbon
             .write(prefix(format!("{}.p75", metric_name), prefix_str),
             p75.to_string(),
              ts);

              carbon
              .write(prefix(format!("{}.p98", metric_name), prefix_str),
              p98.to_string(),
               ts);

               carbon
               .write(prefix(format!("{}.p99", metric_name), prefix_str),
               p99.to_string(),
                ts);

                carbon
                .write(prefix(format!("{}.p999", metric_name), prefix_str),
                p999.to_string(),
                 ts);

                 carbon
                 .write(prefix(format!("{}.p9999", metric_name), prefix_str),
                 p9999.to_string(),
                  ts);

                  carbon
                  .write(prefix(format!("{}.p99999", metric_name), prefix_str),
                  p99999.to_string(),
                   ts);
}

impl CarbonReporter {
    pub fn new(registry: Arc<StdRegistry<'static>>,
     reporter_name: &'static str,
     hostname: &'static str,
     port: u16,
     prefix: &'static str) -> CarbonReporter {
        CarbonReporter {
            hostname: hostname,
            prefix: prefix,
            port: port,
             registry: registry,
              reporter_name: reporter_name
              }
    }

    pub fn start(self, delay_ms: u32) {
        self.report(delay_ms);
    }
}

#[cfg(test)]
mod test {

    use meter::{Meter, StdMeter};
    use counter::{Counter, StdCounter};
    use gauge::{Gauge, StdGauge};
    use registry::{Registry, StdRegistry};
    use reporter::Reporter;
    use carbon_reporter::CarbonReporter;
    use std::sync::Arc;
    use std::thread;
    use histogram::*;

    #[test]
    fn meter() {
        let m = StdMeter::new();
        m.mark(100);

        let mut c: StdCounter = StdCounter::new();
        c.inc(1);

        let mut g: StdGauge = StdGauge { value: 0f64 };
        g.update(1.2);

        let mut h = Histogram::new(
    HistogramConfig{
        max_memory: 0,
        max_value: 1000000,
        precision: 3,
}).unwrap();
        h.record(1, 1);


        let mut r = StdRegistry::new();
        r.insert("meter1", m);
        r.insert("counter1", c);
        r.insert("gauge1", g);
        r.insert("histogram", h);

        let arc_registry = Arc::new(r);
        let reporter = CarbonReporter::new(arc_registry.clone(), "test", "localhost", 2003, "asd.asdf");
        reporter.start(1);

        g.update(1.4);
        thread::sleep_ms(200);
        println!("poplopit");

    }
}
