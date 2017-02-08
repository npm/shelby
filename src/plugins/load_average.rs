extern crate libc;
extern crate numbat;

use self::libc::{c_double,c_int};

use shelby;

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
  emitter: numbat::Emitter<'a>,
}

impl<'a> LoadAverage<'a> {
  pub fn new(emitter: numbat::Emitter<'a>) -> LoadAverage {
    LoadAverage {
      emitter: emitter
    }
  }

  fn send(&mut self) {
    let ret = get_load_average();
    match ret {
      Ok(loads) => {
        self.emitter.emit("load-average.1", loads[0]);
        self.emitter.emit("load-average.5", loads[1]);
        self.emitter.emit("load-average.15", loads[2]);
      },
      Err(e) => panic!("getloadavg() failed: {}", e)
    }

  }
}

impl<'a> shelby::ShelbyPlugin for LoadAverage<'a> {
  fn start(&mut self) {
    println!("starting load average plugin");
    shelby::schedule_repeating(move || {
      self.send();
    }, 10);
  }
}
