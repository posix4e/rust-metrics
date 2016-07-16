extern crate protobuf_build;

use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
  let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
  let source = root.join("proto");
  let out = PathBuf::from(env::var("OUT_DIR").unwrap());
  let mut compiler = protobuf_build::Compiler::new(&source, &out);
  compiler.compile("prometheus.proto").unwrap();


  let path = out.join(&("prometheus.rs"));
  let contents = {
    let mut src = File::open(path).unwrap();
    let mut contents = Vec::new();
    src.read_to_end(&mut contents).unwrap();
    contents
  };

  let mut dst = File::create(out.join("prometheus.rs")).unwrap();
  dst.write_all(format!("pub mod prometheus {{ ").as_bytes()).unwrap();
  dst.write_all(&contents).unwrap();
  dst.write_all("}".as_bytes()).unwrap();
}
