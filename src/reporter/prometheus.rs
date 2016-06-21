// Simple prometheus support. Still a work in progress.
// TODO
// We aren't collecting metrics properly we should be
// on the regular collecting metrics, and snapshoting them
// and sending them all up when prometheus comes to scrape.
use metrics::metric::Metric;
use registry::{Registry, StdRegistry};
use std::thread;
use std::sync::Arc;
use reporter::base::Reporter;
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
use metrics::metric::MetricValue::{Counter, Gauge, Histogram, Meter};

#[derive(Copy, Clone)]
struct HandlerStorage;

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

fn prefix(metric_line: String, prefix_str: &'static str) -> String {
    format!("{}.{}", prefix_str, metric_line)
}

impl PrometheusReporter {
    pub fn new(registry: Arc<StdRegistry<'static>>,
               reporter_name: &'static str,
               host_and_port: &'static str,
               prefix: &'static str)
               -> PrometheusReporter {
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
    let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
    mills
}

fn handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok,
                       families_to_u8(to_pba(req.get::<Read<HandlerStorage>>().unwrap())))))
}

fn families_to_u8(metric_families: Vec<promo_proto::MetricFamily>) -> Vec<u8> {
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
fn to_pba(registry: Arc<Arc<StdRegistry<'static>>>) -> Vec<promo_proto::MetricFamily> {
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

        match metric.export_metric() {
            Counter(x) => {
                let mut counter = promo_proto::Counter::new();
                counter.set_value(x.value);
                pb_metric.set_counter(counter);
                metric_family.set_field_type(promo_proto::MetricType::COUNTER);
            }
            Gauge(x) => {
                let mut gauge = promo_proto::Gauge::new();
                gauge.set_value(x.value);
                pb_metric.set_gauge(gauge);
                metric_family.set_field_type(promo_proto::MetricType::GAUGE);

            }
            Meter(x) => {
                // TODO ask the prometheus guys what we want to do
                pb_metric.set_summary(promo_proto::Summary::new());
                metric_family.set_field_type(promo_proto::MetricType::SUMMARY);

            }
            Histogram(x) => {
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
    use metrics::meter::{Meter, StdMeter};
    use metrics::counter::{Counter, StdCounter};
    use metrics::gauge::{Gauge, StdGauge};
    use registry::{Registry, StdRegistry};
    use reporter::prometheus::PrometheusReporter;
    use std::sync::Arc;
    use std::time::Duration;
    use std::thread;
    use histogram::*;
    use std::collections::HashMap;

    #[test]
    fn add_some_stats_and_slurp_them_with_http() {
        extern crate hyper;
        let m = StdMeter::new();
        m.mark(100);

        let mut c: StdCounter = StdCounter::new();
        c.inc();

        let mut g: StdGauge = StdGauge { value: 0f64 };
        g.set(1.2);

        let mut hc = HistogramConfig::new();
        hc.max_value(100).precision(1);
        let mut h = Histogram::configured(hc).unwrap();

        h.record(1, 1);

        let mut r = StdRegistry::new_with_labels(HashMap::new());
        r.insert("meter1", m);
        r.insert("counter1", c);
        r.insert("gauge1", g);
        r.insert("histogram", h);

        let arc_registry = Arc::new(r);
        let reporter =
            PrometheusReporter::new(arc_registry.clone(), "test", "0.0.0.0:8080", "asd.asdf");
        reporter.start();

        let client = hyper::client::Client::new();
        // Seems as though iron isn't running maybe
        thread::sleep(Duration::from_millis(1024 as u64));
        let res = client.get("http://127.0.0.1:8080").send().unwrap();
        // TODO fix url and check to make sure we got a valid protobuf
    }
}
