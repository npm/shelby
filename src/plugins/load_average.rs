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
        let mut point1: numbat::Point = numbat::Point::new();
        point1.insert("name", serde_json::to_value("load-average.1"));
        point1.insert("value", serde_json::to_value(loads[0]));

        let mut point5: numbat::Point = numbat::Point::new();
        point5.insert("name", serde_json::to_value("load-average.5"));
        point5.insert("value", serde_json::to_value(loads[1]));

        let mut point15: numbat::Point = numbat::Point::new();
        point15.insert("name", serde_json::to_value("load-average.15"));
        point15.insert("value", serde_json::to_value(loads[2]));

        self.emitter.emit(point1);
        self.emitter.emit(point5);
        self.emitter.emit(point15);
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
