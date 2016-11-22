use std::collections::BTreeMap;

extern crate numbat;

mod forza;
mod plugins;

use forza::ForzaPlugin;

fn start_plugin<T: ForzaPlugin>(mut plugin: T) {
  plugin.start();
}

fn main() {
  let mut emitter = numbat::Emitter::new(BTreeMap::new(), "forza");
  emitter.connect("tcp://127.0.0.1:1337");

  let heartbeat = plugins::heartbeat::Heartbeat::new(emitter);
  start_plugin(heartbeat);
}
