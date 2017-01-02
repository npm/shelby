extern crate libc;
extern crate numbat;

use std::mem::zeroed;
use std::ffi::CString;
use forza;

fn get_disk_usage(mountpoint: &str) -> Result<f64, String> {
  unsafe {
    let mut stats: libc::statvfs = zeroed();
    match libc::statvfs(CString::new(mountpoint).unwrap().as_ptr(), &mut stats) {
      -1 => Err(String::from("statvfs() failed")),
      _ => {
        let total = stats.f_blocks * stats.f_frsize;
        let used = total - (stats.f_bavail * stats.f_bsize);
        Ok(used as f64 / total as f64)
      }
    }
  }
}

pub struct DiskUsage<'a> {
  emitter: numbat::Emitter<'a>,
}

impl<'a> DiskUsage<'a> {
  pub fn new(emitter: numbat::Emitter<'a>) -> DiskUsage {
    DiskUsage {
      emitter: emitter
    }
  }

  fn send(&mut self) {
    self.send_for_mountpoint("/");
    self.send_for_mountpoint("/mnt");
  }

  fn send_for_mountpoint(&mut self, mountpoint: &str) {
    let usage = get_disk_usage(mountpoint);
    if usage.is_ok() {
      self.emitter.emit(&(String::from("disk-usage.") + mountpoint), usage.unwrap());
    }
  }
}

impl<'a> forza::ForzaPlugin for DiskUsage<'a> {
  fn start(&mut self) {
    println!("starting disk usage plugin");
    forza::schedule_repeating(move || {
      self.send();
    }, 120);
  }
}
