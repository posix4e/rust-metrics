// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Simple Prometheus support. Still a work in progress.
// TODO
// We aren't collecting metrics properly we should be
// on the regular collecting metrics, and snapshotting them
// and sending them all up when Prometheus comes to scrape.
use metrics::Metric;
use registry::{Registry, StdRegistry};
use std::thread;
use std::sync::Arc;
use reporter::Reporter;
use time;
use promo_proto::MetricFamily;
use router::Router;
use iron;
use iron::typemap::Key;
use iron::prelude::*;
use iron::status;

use promo_proto;
use persistent::Read;
use protobuf::Message;
use protobuf::repeated::RepeatedField;
use promo_proto::LabelPair;
use std::collections::HashMap;

#[derive(Copy, Clone)]
struct HandlerStorage;

#[allow(dead_code)] // until `prefix is used
pub struct PrometheusReporter {
    host_and_port: &'static str,
    // TODO to use as the application name?
    prefix: &'static str,
    registry: Arc<StdRegistry<'static>>,
    reporter_name: &'static str,
}

impl Key for HandlerStorage {
    type Value = Arc<StdRegistry<'static>>;
}

impl Reporter for PrometheusReporter {
    fn get_unique_reporter_name(&self) -> &'static str {
        self.reporter_name
    }
}

#[allow(dead_code)] // until `prefix is used
fn prefix(metric_line: String, prefix_str: &'static str) -> String {
    format!("{}.{}", prefix_str, metric_line)
}

impl PrometheusReporter {
    pub fn new(registry: Arc<StdRegistry<'static>>,
               reporter_name: &'static str,
               host_and_port: &'static str,
               prefix: &'static str)
               -> Self {
        PrometheusReporter {
            host_and_port: host_and_port,
            prefix: prefix,
            registry: registry,
            reporter_name: reporter_name,
        }
    }

    pub fn start(self) -> thread::JoinHandle<iron::Listening> {
        thread::spawn(move || {
            let mut router = Router::new();
            router.get("/metrics", handler);
            let mut chain = Chain::new(router);
            // The double long ARC pointer FTW!
            chain.link_before(Read::<HandlerStorage>::one(self.registry));
            // TODO -> Result<iron::Listening, iron::error::Error>
            Iron::new(chain).http(self.host_and_port).unwrap()
        })
    }
}

// TODO get an i64 instead?
fn timestamp() -> f64 {
    let timespec = time::get_time();
    // 1459440009.113178
    timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0)
}

fn handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok,
                       families_to_u8(to_pba(req.get::<Read<HandlerStorage>>().unwrap())))))
}

fn families_to_u8(metric_families: Vec<MetricFamily>) -> Vec<u8> {
    let mut buf = Vec::new();
    for family in metric_families {
        family.write_length_delimited_to_writer(&mut buf).unwrap();
    }
    buf
}

fn to_repeated_fields_labels(labels: HashMap<String, String>) -> RepeatedField<LabelPair> {
    let mut repeated_fields = Vec::new();
    // name/value is what they call it in the protobufs *shrug*
    for (name, value) in labels {
        let mut label_pair = LabelPair::new();
        label_pair.set_name(name);
        label_pair.set_value(value);
        repeated_fields.push(label_pair);
    }
    RepeatedField::from_vec(repeated_fields)
}

// This is totally terrible, it'd be much better to use macros
// and serde once nightly is stable. I'd consider setting a feature flag but
// it still might increase complexity to deploy
// To an array of MetricFamily
fn to_pba(registry: Arc<Arc<StdRegistry<'static>>>) -> Vec<MetricFamily> {
    let mut metric_families = Vec::new();
    let metric_names = registry.get_metrics_names();
    for metric_by_name in metric_names {
        let mut metric_family = MetricFamily::new();
        let mut pb_metric = promo_proto::Metric::new();
        let metric = registry.get(metric_by_name);
        let formated_metric = format!("{}_{}_{}", "application_name", metric_by_name, "bytes");
        metric_family.set_name(String::from(formated_metric));
        let ts = timestamp() as i64;
        pb_metric.set_timestamp_ms(ts);
        pb_metric.set_label(to_repeated_fields_labels(registry.labels()));

        match *metric {
            Metric::Counter(ref x) => {
                let snapshot = x.snapshot();
                let mut counter = promo_proto::Counter::new();
                counter.set_value(snapshot.value);
                pb_metric.set_counter(counter);
                metric_family.set_field_type(promo_proto::MetricType::COUNTER);
            }
            Metric::Gauge(ref x) => {
                let snapshot = x.snapshot();
                let mut gauge = promo_proto::Gauge::new();
                gauge.set_value(snapshot.value);
                pb_metric.set_gauge(gauge);
                metric_family.set_field_type(promo_proto::MetricType::GAUGE);

            }
            Metric::Meter(_) => {
                // TODO ask the Prometheus guys what we want to do
                pb_metric.set_summary(promo_proto::Summary::new());
                metric_family.set_field_type(promo_proto::MetricType::SUMMARY);

            }
            Metric::Histogram(_) => {
                pb_metric.set_histogram(promo_proto::Histogram::new());
                metric_family.set_field_type(promo_proto::MetricType::HISTOGRAM);
            }
        }

        metric_family.set_metric(RepeatedField::from_vec(vec![pb_metric,]));
        metric_families.push(metric_family);
    }
    metric_families
}

#[cfg(test)]
mod test {
    use histogram::*;
    use metrics::{Counter, Gauge, Meter, Metric, StdCounter, StdGauge, StdMeter};
    use registry::{Registry, StdRegistry};
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;
    use super::PrometheusReporter;

    #[test]
    fn add_some_stats_and_slurp_them_with_http() {
        extern crate hyper;
        let m = StdMeter::new();
        m.mark(100);

        let c = StdCounter::new();
        c.inc();

        let g = StdGauge::new();
        g.set(1.2);

        let mut h = Histogram::configure()
            .max_value(100)
            .precision(1)
            .build()
            .unwrap();

        h.increment_by(1, 1).unwrap();

        let mut r = StdRegistry::new_with_labels(HashMap::new());
        r.insert("meter1", Metric::Meter(Box::new(m)));
        r.insert("counter1", Metric::Counter(c.clone()));
        r.insert("gauge1", Metric::Gauge(g.clone()));
        r.insert("histogram", Metric::Histogram(h));

        let arc_registry = Arc::new(r);
        let reporter =
            PrometheusReporter::new(arc_registry.clone(), "test", "0.0.0.0:8080", "asd.asdf");
        reporter.start();

        let client = hyper::client::Client::new();
        // Seems as though iron isn't running maybe
        thread::sleep(Duration::from_millis(1024));
        client.get("http://127.0.0.1:8080").send().unwrap();
        // TODO fix url and check to make sure we got a valid protobuf
    }
}
