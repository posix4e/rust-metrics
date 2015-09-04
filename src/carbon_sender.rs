use std::net::TcpStream;
use std::io::Write;
use std::thread;
use time::Timespec;

pub struct Carbon {
    graphite_stream: Option<TcpStream> ,
    host_and_port: String
}

impl Carbon {
    pub fn new(host_and_port: String) -> Carbon {
        Carbon {host_and_port: host_and_port, graphite_stream: None}
    }

    pub fn connect(&mut self) {
        let  host_and_port  = &* self.host_and_port;
        match TcpStream::connect(host_and_port) {
            Ok(x) => self.graphite_stream = Some(x),
            Err(e) => panic!("Unable to connect to {} because {}", host_and_port, e)
        }

    }

    pub fn write(& mut self, metric_path: String, value: String, timespec: Timespec) {
        let seconds_in_ms = (timespec.sec * 1000) as u32;
        let nseconds_in_ms  = (timespec.nsec / 1000) as u32;
        let timestamp = seconds_in_ms + nseconds_in_ms;
        match self.graphite_stream {
            Some(ref mut stream) => {
                let carbon_command = format!("{} {} {}\n", metric_path, value, timestamp).into_bytes();
                match stream.write_all(&carbon_command) {
                    Ok(x) => {}
                    Err(x) => println!("Failed to Send {:?}", x),
                }
            }
            None => {
                self.reconnect_stream();
                self.write(metric_path, value, timespec);
            }
        }
    }
    fn reconnect_stream(& mut self) {
        println!("Waiting 10ms and then reconnecting");
        thread::sleep_ms(10);
        self.connect();
    }

}
