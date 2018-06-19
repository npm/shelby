extern crate libc;
extern crate numbat;

mod shelby;
mod plugins;

use std::thread;
use shelby::ShelbyPlugin;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn start_plugin<T: ShelbyPlugin>(mut plugin: T) {
  plugin.start();
}

fn main() {
  println!("shelby v{}", VERSION);

  let mut emitter = numbat::Emitter::for_app("host");
  emitter.connect(&match std::env::var_os("METRICS") {
    Some(url) => url.into_string().expect("expected METRICS to be valid UTF-8"),
    None => String::from("nsq://localhost:4151/pub?topic=metrics")
  });

  emitter.emit_name("start");

  let heartbeat = plugins::heartbeat::new(emitter.clone());
  thread::spawn(|| {
    start_plugin(heartbeat);
  });

  let memory = plugins::memory::Memory::new(emitter.clone());
  thread::spawn(|| {
    start_plugin(memory);
  });

  let uptime = plugins::uptime::Uptime::new(emitter.clone());
  thread::spawn(|| {
    start_plugin(uptime);
  });

  let disk_usage = plugins::disk_usage::DiskUsage::new(emitter.clone());
  thread::spawn(|| {
    start_plugin(disk_usage);
  });

  let netstat = plugins::netstat::Netstat::new(emitter.clone());
  thread::spawn(|| {
    start_plugin(netstat);
  });

  let load_average = plugins::load_average::LoadAverage::new(emitter.clone());
  thread::spawn(|| {
    start_plugin(load_average);
  }).join().unwrap();
}
