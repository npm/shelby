extern crate timer;
extern crate chrono;
extern crate numbat;

use std::sync::mpsc::channel;

pub trait ShelbyPlugin {
  fn start(&mut self);
}

pub fn schedule_repeating<F: FnMut()>(mut send: F, interval: i64) {
  let timer = timer::Timer::new();

  send();

  let (tx, rx) = channel();

  let _guard = timer.schedule_repeating(chrono::Duration::seconds(interval), move || {
    let _ignored = tx.send(());
  });

  while rx.recv().is_ok() {
    send();
  }
}
