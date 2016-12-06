extern crate libc;
extern crate timer;
extern crate chrono;
extern crate numbat;
extern crate serde_json;

use std::sync::mpsc::channel;
use self::libc::{c_double,c_int};

use forza;

#[cfg(unix)]
extern {
  fn getloadavg(loadavg: *mut c_double, nelem: c_int) -> c_int;
}

fn get_load_average() -> Result<[f64; 3], String> {
  let mut avg: [c_double; 3] = [0.0; 3];
  unsafe {
    match getloadavg(avg.as_mut_ptr(), 3) {
      -1 => Err(String::from("getloadavg() failed")),
      _ => Ok(avg)
    }
  }
}

pub struct LoadAverage<'a> {
  timer: timer::Timer,
  emitter: numbat::Emitter<'a>,
  started: bool
}

impl<'a> LoadAverage<'a> {
  pub fn new(emitter: numbat::Emitter<'a>) -> LoadAverage {
    LoadAverage {
      timer: timer::Timer::new(),
      emitter: emitter,
      started: false
    }
  }

  fn schedule(&mut self) {
    self.send();

    let (tx, rx) = channel();

    let _guard = self.timer.schedule_repeating(chrono::Duration::seconds(10), move || {
      let _ignored = tx.send(());
    });

    while self.started {
      rx.recv().unwrap();
      self.send();
    }
  }

  fn send(&mut self) {
    let ret = get_load_average();
    match ret {
      Ok(loads) => {
        self.emitter.emit_f64("load-average.1", loads[0]);
        self.emitter.emit_f64("load-average.5", loads[1]);
        self.emitter.emit_f64("load-average.15", loads[2]);
      },
      Err(e) => panic!("getloadavg() failed: {}", e)
    }

  }
}

impl<'a> forza::ForzaPlugin for LoadAverage<'a> {
  fn start(&mut self) {
    println!("starting load average plugin");
    self.started = true;
    self.schedule();
  }

  fn stop(&mut self) {
    println!("stopping load average plugin");
    self.started = false;
  }
}
