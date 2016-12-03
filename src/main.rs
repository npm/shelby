extern crate libc;
extern crate numbat;

mod forza;
mod plugins;

use std::thread;
use std::collections::BTreeMap;
use forza::ForzaPlugin;

fn start_plugin<T: ForzaPlugin>(mut plugin: T) {
  plugin.start();
}

fn main() {
  let mut emitter = numbat::Emitter::new(BTreeMap::new(), "forza");
  emitter.connect("tcp://127.0.0.1:1337");

  let heartbeat = plugins::heartbeat::Heartbeat::new(emitter.clone());
  thread::spawn(|| {
    start_plugin(heartbeat);
  });

  let memory = plugins::memory::Memory::new(emitter.clone());
  thread::spawn(|| {
    start_plugin(memory);
  });

  let load_average = plugins::load_average::LoadAverage::new(emitter.clone());
  thread::spawn(|| {
    start_plugin(load_average);
  }).join();
}
