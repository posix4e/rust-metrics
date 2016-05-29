use metric::Metric;
use registry::{Registry, StdRegistry};
use std::time::Duration;
use std::thread;
use std::sync::Arc;
use meter::Meter;
use reporter::base::Reporter;
use counter::StdCounter;
use gauge::StdGauge;
use meter::MeterSnapshot;
use histogram::Histogram;
use time;
use time::Timespec;

use iron::{Iron, Request, Response, IronResult, AfterMiddleware, Chain};
use iron::error::{IronError};
use iron::status;
use router::{Router, NoRoute};
use std::result;
use iron;

struct Custom404;

impl AfterMiddleware for Custom404 {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        println!("Hitting custom 404 middleware");

        if let Some(_) = err.error.downcast::<NoRoute>() {
            Ok(Response::with((status::NotFound, "Custom 404 response")))
        } else {
            Err(err)
        }
    }
}

pub struct PrometheusReporter {
    host_and_port: &'static str,
    prefix: &'static str,
    registry: Arc<StdRegistry<'static>>,
    reporter_name: &'static str
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

    pub fn start(self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let mut router = Router::new();
            router.get("/", handler);
            let mut chain = Chain::new(router);
            chain.link_after(Custom404);
            // TODO -> Result<iron::Listening, iron::error::Error>
            Iron::new(chain).http(self.host_and_port).unwrap();
        })
    }

}

fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Handling response")))
}

#[cfg(test)]
mod test {
    use meter::{Meter, StdMeter};
    use counter::{Counter, StdCounter};
    use gauge::{Gauge, StdGauge};
    use registry::{Registry, StdRegistry};
    use reporter::prometheus::PrometheusReporter;
    use std::sync::Arc;
    use histogram::*;

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

        let mut r = StdRegistry::new();
        r.insert("meter1", m);
        r.insert("counter1", c);
        r.insert("gauge1", g);
        r.insert("histogram", h);

        let arc_registry = Arc::new(r);
        let reporter = PrometheusReporter::new(arc_registry.clone(),
                            "test",
                            "0.0.0.0:8080",
                            "asd.asdf");
        reporter.start();

        let client = hyper::client::Client::new();

        let res = client.get("http://127.0.0.1:8080").send().unwrap();


    }
}
