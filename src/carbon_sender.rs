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
use std::net::TcpStream;
use std::io::Write;
pub struct Carbon {
    graphite_stream: Option<TcpStream> ,
    hostname: &'static str,
    port: u16
}


impl Carbon {
    pub fn new(hostname:&'static str, port: u16 ) -> Carbon {
        Carbon {hostname: hostname, port: port, graphite_stream: None}
    }

    pub fn connect(&mut self) {
        match TcpStream::connect((self.hostname, self.port)) {
            Ok(x) => self.graphite_stream = Some(x),
            Err(e) => panic!("Unable to connect to {} {}", self.hostname, self.port)
        }

    }

    pub fn write(& mut self, metric_path: String, value: String, timestamp: u32) {
        match self.graphite_stream {
            Some(ref mut stream) => {
                let carbon_command = format!("{} {} {}\n", metric_path, value, timestamp).into_bytes();
                match stream.write_all(&carbon_command) {
                    Ok(x) => println!("foo {:?}" , x),
                    Err(x) => println!("bar {:?}", x),
                }
            }
            None => {
                self.reconnect_stream();
                self.write(metric_path, value, timestamp);
            }
        }
    }
    fn reconnect_stream(& mut self) {
        println!("Waiting 10ms and then reconnecting");
        self.connect();
    }

}
