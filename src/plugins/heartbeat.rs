extern crate numbat;

use shelby;

pub struct Heartbeat<'a> {
  emitter: numbat::Emitter<'a>
}

pub fn new(emitter: numbat::Emitter) -> Heartbeat {
  Heartbeat {
    emitter: emitter
  }
}

impl<'a> shelby::ShelbyPlugin for Heartbeat<'a> {
  fn start(&mut self) {
    println!("starting heartbeat plugin");
    shelby::schedule_repeating(move || {
      self.emitter.emit_name("heartbeat");
    }, 10);
  }
}
