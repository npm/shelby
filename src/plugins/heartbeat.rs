extern crate timer;
extern crate chrono;
extern crate numbat;
extern crate serde_json;

use std::sync::mpsc::channel;

use forza;

pub struct Heartbeat<'a> {
  timer: timer::Timer,
  emitter: numbat::Emitter<'a>,
  started: bool
}

impl<'a> Heartbeat<'a> {
  pub fn new(emitter: numbat::Emitter<'a>) -> Heartbeat {
    Heartbeat {
      timer: timer::Timer::new(),
      emitter: emitter,
      started: false
    }
  }

  fn schedule(&mut self) {
    self.send();

    let (tx, rx) = channel();

    let _guard = self.timer.schedule_with_delay(chrono::Duration::seconds(10), move || {
      let _ignored = tx.send(());
    });

    rx.recv().unwrap();

    if self.started {
      self.schedule();
    }
  }

  fn send(&mut self) {
    let mut point: numbat::Point = numbat::Point::new();
    point.insert("name", serde_json::to_value("heartbeat"));
    point.insert("value", serde_json::to_value(1));
    self.emitter.emit(point);
  }
}

impl<'a> forza::ForzaPlugin for Heartbeat<'a> {
  fn start(&mut self) {
    println!("starting heartbeat plugin");
    self.started = true;
    self.schedule();
  }

  fn stop(&mut self) {
    println!("stopping heartbeat plugin");
    self.started = false;
  }
}
