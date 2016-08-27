// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
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

extern crate time;
extern crate iron;
extern crate router;
extern crate persistent;
extern crate protobuf; // depend on rust-protobuf runtime

pub mod promo_proto;
use router::Router;
use iron::typemap::Key;
use iron::prelude::*;
use iron::status;
use persistent::Read;
use std::sync::{Arc, Mutex};

use protobuf::Message;

// http handler storage
#[derive(Copy, Clone)]
struct HandlerStorage;

// refer to https://prometheus.io/docs/instrumenting/exposition_formats/
const CONTENT_TYPE: &'static str = "application/vnd.google.protobuf; \
                                    proto=io.prometheus.client.MetricFamily;
                                    encoding=delimited";

impl Key for HandlerStorage {
    type Value = Arc<Mutex<Vec<promo_proto::MetricFamily>>>;
}

// TODO perhaps we autodiscover the host and port
pub struct PromoReporter {
    host_and_port: &'static str,
    metrics: Arc<Mutex<Vec<promo_proto::MetricFamily>>>,
}

fn families_to_u8(metric_families: Vec<promo_proto::MetricFamily>) -> Vec<u8> {
    let mut buf = Vec::new();
    for family in metric_families {
        family.write_length_delimited_to_writer(&mut buf).unwrap();
    }
    buf
}

fn handler(req: &mut Request) -> IronResult<Response> {
    match req.get::<Read<HandlerStorage>>() {
        Ok(arc_metric_families) => {
            let metric_families = arc_metric_families.lock().unwrap().clone();
            let serialized: Vec<u8> = families_to_u8(metric_families);
            // TODO lifecycle out the metrics we sent up
            Ok(Response::with((CONTENT_TYPE, status::Ok, serialized)))
        }
        Err(_) => Ok(Response::with((status::InternalServerError, "ERROR"))),
    }
}

impl PromoReporter {
    pub fn new(host_and_port: &'static str, 
               metrics: Arc<Mutex<Vec<promo_proto::MetricFamily>>>) -> Self {
        PromoReporter {
            host_and_port: host_and_port,
            metrics: metrics,
        }
    }

    pub fn start(self) -> iron::Listening {
      let mut router = Router::new();
      router.get("/metrics", handler);
      let mut chain = Chain::new(router);
      chain.link_before(Read::<HandlerStorage>::one(self.metrics));
      // TODO get rid of the unwrap
      Iron::new(chain).http(self.host_and_port).unwrap()
    }

}

#[cfg(test)]
mod test {

    extern crate hyper;
    use std::thread;
    use std::time::Duration;
    use protobuf::repeated::RepeatedField;
    use super::*;
    use std::sync::{Arc, Mutex};

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
        let metrics = Arc::new(Mutex::new((vec![])));
        let reporter = PromoReporter::new("0.0.0.0:8080", metrics.clone());
        thread::spawn(move || {
            reporter.start();
        });
        thread::sleep(Duration::from_millis(1024));
        metrics.lock().unwrap().push(a_metric_family());

        let client = hyper::client::Client::new();
        client.get("http://127.0.0.1:8080").send().unwrap();
    }
}
