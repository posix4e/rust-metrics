// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// PrometheusReporter aggregates metrics into metricsfamilies and passes them every second to the
// attached prometheus reporter at a regular basis.

extern crate prometheus_reporter;
extern crate protobuf;
use self::prometheus_reporter::PrometheusReporter as Pr;
use self::prometheus_reporter::promo_proto;

use std::time::Duration;
use std::thread;
use metrics::Metric;
use time;
use std::collections::HashMap;
use std::sync::mpsc;
use reporter::Reporter;
use self::protobuf::repeated::RepeatedField;

struct PrometheusMetricEntry {
    name: String,
    metric: Metric,
    labels: HashMap<String, String>,
}

// TODO perhaps we autodiscover the host and port
//
pub struct PrometheusReporter {
    reporter_name: &'static str,
    tx: mpsc::Sender<Result<PrometheusMetricEntry, &'static str>>,
    join_handle: thread::JoinHandle<Result<(), String>>,
}
impl Reporter for PrometheusReporter {
    fn get_unique_reporter_name(&self) -> &str {
        &(*self.reporter_name)
    }
    fn stop(self) -> Result<thread::JoinHandle<Result<(), String>>, String> {
        match self.tx.send(Err("stop")) {
            Ok(_) => Ok(self.join_handle),
            Err(x) => Err(format!("Unable to stop reporter:{}", x)),
        }
    }
    fn addl<S: Into<String>>(&mut self,
                             name: S,
                             metric: Metric,
                             labels: Option<HashMap<String, String>>)
                             -> Result<(), String> {
        // TODO return error
        let ref mut tx = &self.tx;
        match tx.send(Ok(PrometheusMetricEntry {
            name: name.into(),
            metric: metric,
            labels: labels.unwrap_or(HashMap::new()),
        })) {
            Ok(_) => Ok(()),
            Err(x) => Err(format!("Unable to stop reporter: {}", x)),
        }
    }
}

impl PrometheusReporter {
    pub fn new(reporter_name: &'static str, host_and_port: &'static str, delay_ms: u64) -> Self {
        let (tx, rx) = mpsc::channel();
        PrometheusReporter {
            reporter_name: reporter_name,
            tx: tx,
            join_handle: thread::spawn(move || {
                let mut stop = false;
                let mut prometheus_reporter = Pr::new(host_and_port);
                while !stop {
                    match collect_to_send(&rx) {
                        // Unwraping is always dangerouns. In this case our prometheus reporter is
                        // overwhelmed by metrics
                        Ok(metrics) => {
                            try!(prometheus_reporter.add(metrics));
                        }
                        Err(_) => stop = true,
                    }
                    thread::sleep(Duration::from_millis(delay_ms));
                }
                Ok(())
            }),
        }
    }
}

fn to_repeated_fields_labels(labels: HashMap<String, String>)
                             -> RepeatedField<promo_proto::LabelPair> {
    let mut repeated_fields = Vec::new();
    // name/value is what they call it in the protobufs *shrug*
    for (name, value) in labels {
        let mut label_pair = promo_proto::LabelPair::new();
        label_pair.set_name(name);
        label_pair.set_value(value);
        repeated_fields.push(label_pair);
    }
    RepeatedField::from_vec(repeated_fields)
}

fn make_metric(metric: &Metric,
               labels: &HashMap<String, String>)
               -> (promo_proto::Metric, promo_proto::MetricType) {

    let mut pb_metric = promo_proto::Metric::new();
    let ts = time::now().to_timespec().sec;

    pb_metric.set_timestamp_ms(ts);
    pb_metric.set_label(to_repeated_fields_labels(labels.clone()));
    match *metric {
        Metric::Counter(ref x) => {
            let snapshot = x.snapshot();
            let mut counter = promo_proto::Counter::new();
            counter.set_value(snapshot.value as f64);
            pb_metric.set_counter(counter);
            (pb_metric, promo_proto::MetricType::COUNTER)
        }
        Metric::Gauge(ref x) => {
            let snapshot = x.snapshot();
            let mut gauge = promo_proto::Gauge::new();
            gauge.set_value(snapshot.value as f64);
            pb_metric.set_gauge(gauge);
            (pb_metric, promo_proto::MetricType::GAUGE)
        }
        Metric::Meter(_) => {
            pb_metric.set_summary(promo_proto::Summary::new());
            (pb_metric, promo_proto::MetricType::SUMMARY)

        }
        Metric::Histogram(_) => {
            pb_metric.set_histogram(promo_proto::Histogram::new());
            (pb_metric, promo_proto::MetricType::HISTOGRAM)
        }
    }
}

fn collect_to_send(metric_entries: &mpsc::Receiver<Result<PrometheusMetricEntry, &'static str>>)
                   -> Result<Vec<promo_proto::MetricFamily>, &'static str> {
    let mut entries_group = HashMap::<String, Vec<PrometheusMetricEntry>>::new();

    // Group them by name TODO we should include tags and types in the grouping
    for _entry in metric_entries {
        let entry = try!(_entry);
        let name = entry.name.clone();
        let mut entries = entries_group.remove(&name).unwrap_or(vec![]);
        entries.push(entry);
        entries_group.insert(name, entries);
    }
    Ok(metric_entries_to_family(entries_group))

}

fn metric_entries_to_family(entries_group: HashMap<String, Vec<PrometheusMetricEntry>>)
                            -> Vec<promo_proto::MetricFamily> {
    let mut families = Vec::new();
    for (name, metric_entries) in &entries_group {
        let formatted_metric = format!("{}_{}_{}", "application_name", name, "bytes");
        // TODO check for 0 length

        let ref e1: PrometheusMetricEntry = metric_entries[0];
        let (_, pb_metric_type) = make_metric(&e1.metric, &e1.labels);

        let mut family = promo_proto::MetricFamily::new();
        let mut pb_metrics = Vec::new();

        for metric_entry in metric_entries {
            // TODO maybe don't assume they have the same type
            let (pb_metric, _) = make_metric(&metric_entry.metric, &metric_entry.labels);
            pb_metrics.push(pb_metric);
        }

        family.set_name(String::from(formatted_metric));
        family.set_field_type(pb_metric_type);
        family.set_metric(RepeatedField::from_vec(pb_metrics));
        families.push(family);
    }
    families
}



#[cfg(test)]
mod test {
    use histogram::Histogram;
    use std::collections::HashMap;
    use metrics::{Counter, Gauge, Meter, Metric, StdCounter, StdGauge, StdMeter};
    use super::PrometheusReporter;
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

        let mut reporter = PrometheusReporter::new("test", "0.0.0.0:80", 1024);
        let labels = Some(HashMap::new());
        reporter.addl("meter1", Metric::Meter(m.clone()), labels.clone());
        reporter.addl("counter1", Metric::Counter(c.clone()), labels.clone());
        reporter.addl("gauge1", Metric::Gauge(g.clone()), labels.clone());
        reporter.addl("histogram", Metric::Histogram(h), labels);
        reporter.stop().unwrap().join().unwrap().unwrap();
    }
}
