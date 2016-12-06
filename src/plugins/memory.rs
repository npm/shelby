extern crate libc;
extern crate timer;
extern crate chrono;
extern crate numbat;
extern crate serde_json;

use std::sync::mpsc::channel;
use std::fs::File;
use std::io::Read;
use forza;

pub struct Memory<'a> {
  timer: timer::Timer,
  emitter: numbat::Emitter<'a>,
  started: bool
}

impl<'a> Memory<'a> {
  pub fn new(emitter: numbat::Emitter<'a>) -> Memory {
    Memory {
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
    let mut file = File::open("/proc/meminfo").expect("failed to open /proc/meminfo");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("failed to read /proc/meminfo");

    let lines = content.lines();

    let mut total : Option<u64> = None;
    let mut free : Option<u64> = None;
    let mut cached : Option<u64> = None;
    let mut buffers : Option<u64> = None;

    for line in lines {
      let mut split = line.split_whitespace();
      let name = split.next().expect("expected a metric name");
      let value = split.next().expect("expected a metric value");

      if name == "MemTotal:" {
        total = Some(value.parse::<u64>().expect("expected MemTotal to be parsable"));
      }
      else if name == "MemFree:" {
        free = Some(value.parse::<u64>().expect("expected MemFree to be parsable"));
      }
      else if name == "Cached:" {
        cached = Some(value.parse::<u64>().expect("expected Cached to be parsable"));
      }
      else if name == "Buffers:" {
        buffers = Some(value.parse::<u64>().expect("expected Buffers to be parsable"));
      }
    }

    let t = total.expect("expected MemTotal");
    let used =
      t -
      free.expect("expected MemFree") -
      cached.expect("expected Cached") -
      buffers.expect("expected Buffers");

    self.emitter.emit_f64("memory", used as f64 / t as f64);
  }
}

impl<'a> forza::ForzaPlugin for Memory<'a> {
  fn start(&mut self) {
    println!("starting memory plugin");
    self.started = true;
    self.schedule();
  }

  fn stop(&mut self) {
    println!("stopping memory plugin");
    self.started = false;
  }
}
