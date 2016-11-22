use std::collections::LinkedList;

extern crate rustc_serialize;

mod forza;
mod plugins;

fn main() {
  // let hostname = std::env::var("HOSTNAME").expect("HOSTNAME is expected");
  // let port = std::env::var("PORT").expect("PORT is expected");
  //
  let mut plugins: LinkedList<Box<forza::ForzaPlugin>> = LinkedList::new();
  plugins.push_back(Box::new(plugins::heartbeat::Heartbeat::new()));
  let mut forza = forza::Forza::new("tcp://127.0.0.1:1337", plugins);
  forza.start();
}
