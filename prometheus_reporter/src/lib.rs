// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// Simple Prometheus support. Still a work in progress.
// The goal here is to develop a prometheus client that
// is fully designed for prometheus with the minimal
// dependencies and overhead.
//
// We currently provide no abstraction around collecting metrics.
// This Reporter eventually needs to handle the multiplexing and
// organization of metrics.

extern crate iron;
extern crate router;
extern crate persistent;
extern crate protobuf; // depend on rust-protobuf runtime
extern crate lru_cache;
extern crate time;

pub mod promo_proto;
use router::Router;
use iron::typemap::Key;
use iron::prelude::*;
use iron::status;
use persistent::Read;
use lru_cache::LruCache;
use protobuf::Message;
use std::sync::{Arc, RwLock};
use std::thread;

// http handler storage
#[derive(Copy, Clone)]
struct HandlerStorage;

// refer to https://prometheus.io/docs/instrumenting/exposition_formats/
const CONTENT_TYPE: &'static str = "application/vnd.google.protobuf; \
                                    proto=io.prometheus.client.MetricFamily;
                                    encoding=delimited";

impl Key for HandlerStorage {
    type Value = Arc<RwLock<LruCache<i64, promo_proto::MetricFamily>>>;
}

fn get_seconds() -> i64 {
    time::now().to_timespec().sec
}

fn families_to_u8(metric_families: Vec<(&i64, &promo_proto::MetricFamily)>) -> Vec<u8> {
    let mut buf = Vec::new();
    for (_, family) in metric_families {
        family.write_length_delimited_to_writer(&mut buf).unwrap();
    }
    buf
}

fn handler(req: &mut Request) -> IronResult<Response> {
    match req.get::<Read<HandlerStorage>>() {
        Ok(ts_and_metrics) => {
            // TODO catch unwrap
            let serialized: Vec<u8> =
                families_to_u8((*ts_and_metrics).read().unwrap().iter().collect());
            // TODO lifecycle out the metrics we sent up
            Ok(Response::with((CONTENT_TYPE, status::Ok, serialized)))
        }
        Err(_) => Ok(Response::with((status::InternalServerError, "ERROR"))),
    }
}

// TODO perhaps we autodiscover the host and port
pub struct PrometheusReporter {
    cache: Arc<RwLock<LruCache<i64, promo_proto::MetricFamily>>>,
    host_and_port: &'static str,
}

impl PrometheusReporter {
    pub fn new(host_and_port: &'static str) -> Self {
        PrometheusReporter {
            // TODO make it configurable
            cache: Arc::new(RwLock::new(LruCache::new(86400))),
            host_and_port: host_and_port,
        }
    }
    // TODO require start before add
    pub fn add(&mut self, metric_families: Vec<promo_proto::MetricFamily>) -> Result<i64, String> {
        let ts = get_seconds();
        match self.cache.write() {
            Ok(mut cache) => {
                let mut counter = 0;
                for metric_family in metric_families {
                    cache.insert(ts, metric_family);
                    counter = counter + 1;
                }
                Ok(counter)
            }
            Err(y) => Err(format!("Unable to add {}", y)),
        }
    }

    pub fn start(&self) -> Result<&str, String> {
        let mut router = Router::new();
        router.get("/metrics", handler);
        let mut chain = Chain::new(router);
        chain.link_before(Read::<HandlerStorage>::one(self.cache.clone()));
        // TODO get rid of the unwrap

        match Iron::new(chain).http(self.host_and_port) {
            Ok(iron) => {
                thread::spawn(move || iron);
                Ok("go")
            }
            Err(y) => Err(format!("Unable to start iron: {}", y)),
        }
    }
}

#[cfg(test)]
mod test {

    extern crate hyper;
    extern crate lru_cache;
    use std::thread;
    use std::time::Duration;
    use protobuf::repeated::RepeatedField;
    use super::*;

    fn a_metric_family() -> promo_proto::MetricFamily {
        let mut family = promo_proto::MetricFamily::new();
        family.set_name("MetricFamily".to_string());
        family.set_help("Help".to_string());
        family.set_field_type(promo_proto::MetricType::GAUGE);

        let mut metric = promo_proto::Metric::new();
        metric.set_timestamp_ms(a_ts());
        metric.set_gauge(a_gauge());
        metric.set_label(a_label_pair());
        family.set_metric(RepeatedField::from_vec(vec![metric]));

        family
    }

    fn a_label_pair() -> RepeatedField<promo_proto::LabelPair> {
        let mut label_pair = promo_proto::LabelPair::new();
        // The name and value alas are the names of the
        // protobuf fields in the pair in prometheus.
        label_pair.set_name("name".to_string());
        label_pair.set_value("value".to_string());
        RepeatedField::from_vec(vec![label_pair])
    }

    fn a_gauge() -> promo_proto::Gauge {
        let mut gauge = promo_proto::Gauge::new();
        gauge.set_value(0.1);
        gauge
    }

    fn a_ts() -> i64 {
        0
    }

    #[test]
    fn add_some_stats_and_slurp_them_with_http() {
        // Shouldn't need a mutex but oh well
        let mut reporter = PrometheusReporter::new("0.0.0.0:8080");
        reporter.start().unwrap();
        thread::sleep(Duration::from_millis(1024));
        reporter.add(vec![]);
        let client = hyper::client::Client::new();
        client.get("http://127.0.0.1:8080").send().unwrap();
    }

}
