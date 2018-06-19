extern crate libc;
extern crate numbat;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use shelby;

pub struct Uptime<'a> {
  emitter: numbat::Emitter<'a>
}

impl<'a> Uptime<'a> {
  pub fn new(emitter: numbat::Emitter<'a>) -> Uptime {
    Uptime {
      emitter: emitter
    }
  }

  fn send(&mut self) {
    let mut file = File::open("/proc/uptime").expect("failed to open /proc/uptime");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("failed to read /proc/uptime");

    let mut lines = content.lines();
    let line = lines.next().expect("expected /proc/uptime to have at least one line");
    let mut split = line.split_whitespace();
    let uptime_str = split.next().expect("expected /proc/uptime to have at least one field");
    let uptime = uptime_str.parse::<f64>().expect("expected uptime to be parsable");
    self.emitter.emit("uptime", uptime);
  }
}

impl<'a> shelby::ShelbyPlugin for Uptime<'a> {
  fn start(&mut self) {
    if Path::new("/proc/uptime").exists() {
        println!("starting uptime plugin");
        shelby::schedule_repeating(move || {
          self.send();
        }, 60);
    } else {
        println!("skipping uptime plugin; no /proc on this system");
    }
  }
}
