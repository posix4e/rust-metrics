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
use lru_cache::LruCache;
use protobuf::Message;
use std::sync::{Arc, RwLock};
use std::thread;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// http handler storage
#[derive(Copy, Clone)]
struct HandlerStorage;

// refer to https://prometheus.io/docs/instrumenting/exposition_formats/
const CONTENT_TYPE: &'static str = "application/vnd.google.protobuf; \
                                    proto=io.prometheus.client.MetricFamily;
                                    encoding=delimited";

impl Key for HandlerStorage {
    type Value = Arc<RwLock<LruCache<u64, promo_proto::MetricFamily>>>;
}


fn families_to_u8(metric_families: Vec<(&u64, &promo_proto::MetricFamily)>) -> Vec<u8> {
    let mut buf = Vec::new();
    for (_, family) in metric_families {
        family.write_length_delimited_to_writer(&mut buf).unwrap();
    }
    buf
}

fn handler(req: &mut Request) -> IronResult<Response> {
    match req.get::<persistent::Read<HandlerStorage>>() {
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
    cache: Arc<RwLock<LruCache<u64, promo_proto::MetricFamily>>>
}

impl PrometheusReporter {
    pub fn new(host_and_port: &'static str) -> Self {
        let reporter = PrometheusReporter {
            // TODO make it configurable
            cache: Arc::new(RwLock::new(LruCache::new(86400)))
        };
        let mut router = Router::new();
        router.get("/metrics", handler);
        let mut chain = Chain::new(router);
        chain.link_before(persistent::Read::<HandlerStorage>::one(reporter.cache.clone()));
        // TODO get rid of the unwrap

        thread::spawn(move ||{
            match Iron::new(chain).http(host_and_port) {
                Ok(_) => {}
                Err(x) => panic!("Unable to start prometheus reporter {}",x),
            }
        });
        reporter
    }
    // TODO require start before add
    pub fn add(&mut self, metric_families: Vec<promo_proto::MetricFamily>) -> Result<i64, String> {
        
        match self.cache.write() {
            Ok(mut cache) => {
                let mut counter = 0;
                for metric_family in metric_families {

                    let mut hasher = DefaultHasher::new();
                    metric_family.get_name().hash(&mut hasher);
                    cache.insert(hasher.finish(), metric_family);
                    counter = counter + 1;
                }
                Ok(counter)
            }
            Err(y) => Err(format!("Unable to add {}", y)),
        }
    }

    pub fn remove(&mut self, metrics_to_remove: Vec<String>) -> Result<i64, String> {
        match self.cache.write() {
            Ok(mut cache) => {
                let mut counter = 0;
                for metric_name_to_remove in metrics_to_remove {

                    let mut hasher = DefaultHasher::new();
                    metric_name_to_remove.hash(&mut hasher);
                    if let Some(_) = cache.remove(&hasher.finish()) {
                        counter = counter + 1;
                    }
                }
                Ok(counter)
            }
            Err(y) => Err(format!("Unable to remove {}", y)),
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
    use std::io::Read;
    use super::*;

    fn a_metric_family_name() -> String {
       "MetricFamily".to_string()
    }
    
 
    fn a_metric_family() -> promo_proto::MetricFamily {
        let mut family = promo_proto::MetricFamily::new();
        family.set_name(a_metric_family_name());
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
        // protobuf fields in the pair in prometheus protobuf spec
        // Thes undescriptive names are part of the protocol alas.
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
        let mut reporter = PrometheusReporter::new("0.0.0.0:8080");
        thread::sleep(Duration::from_millis(1024));
        reporter.add(vec![a_metric_family()]);
        let client = hyper::client::Client::new();
        let mut res = client.get("http://127.0.0.1:8080/metrics").send().unwrap();
        assert_eq!(res.status, hyper::Ok);
        let mut buffer = Vec::new();
        let size_of_buffer = res.read_to_end(&mut buffer).unwrap();
        println!("{:?} size:{} ", buffer, size_of_buffer);
        assert_eq!(size_of_buffer, 53);
    }

    #[test]
    fn add_and_remove_metric() {
        let mut reporter = PrometheusReporter::new("0.0.0.0:8081");
        thread::sleep(Duration::from_millis(1024));
        reporter.add(vec![a_metric_family()]);
        if let Ok(number_removed) = reporter.remove(vec![a_metric_family_name()]) {
           assert_eq!(number_removed, 1);
        }
        if let Ok(number_removed) = reporter.remove(vec![a_metric_family_name()]) {
           assert_eq!(number_removed, 0);
        }
    }

}
