extern crate numbat;

use forza;

pub struct Heartbeat<'a> {
  emitter: numbat::Emitter<'a>
}

pub fn new(emitter: numbat::Emitter) -> Heartbeat {
  Heartbeat {
    emitter: emitter
  }
}

impl<'a> forza::ForzaPlugin for Heartbeat<'a> {
  fn start(&mut self) {
    println!("starting heartbeat plugin");
    forza::schedule_repeating(move || {
      self.emitter.emit_name("heartbeat");
    }, 10);
  }
}
