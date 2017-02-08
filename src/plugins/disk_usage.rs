extern crate libc;
extern crate numbat;

use std::mem::zeroed;
use std::ffi::CString;
use std::path::Path;

use shelby;

struct Usage {
  blocks: f64, // from 0 (nothing used) to 1.0 (all bytes used); mult by 100 to get percent duh
  inodes: f64, // from 0 (nothing used) to 1.0 (all used)
}

fn get_disk_usage(mountpoint: &str) -> Result<Usage, String> {
  unsafe {
    let mut stats: libc::statvfs = zeroed();
    match libc::statvfs(CString::new(mountpoint).unwrap().as_ptr(), &mut stats) {
      -1 => Err(String::from("statvfs() failed")),
      _ => {

        let free_bytes: f64 = stats.f_bavail as f64 * stats.f_frsize as f64;
        let total_bytes: f64 = stats.f_blocks as f64 * stats.f_frsize as f64;

        Ok(Usage {
          blocks: (total_bytes - free_bytes)/total_bytes,
          inodes: (stats.f_files - stats.f_favail) as f64/stats.f_files as f64
        })
      }
    }
  }
}

pub struct DiskUsage<'a> {
  emitter: numbat::Emitter<'a>,
  mounts: Vec<String>
}

impl<'a> DiskUsage<'a> {
  pub fn new(emitter: numbat::Emitter<'a>) -> DiskUsage {
    let mut mounts = Vec::new();
    if Path::new("/").exists() {
      mounts.push(String::from("/"))
    }
    if Path::new("/mnt").exists() {
      mounts.push(String::from("/mnt"))
    }
    DiskUsage {
      emitter: emitter,
      mounts: mounts
    }
  }

  fn send(&mut self) {
    for p in self.mounts.clone() {
      self.send_for_mountpoint(&*p);
    }
  }

  fn send_for_mountpoint(&mut self, mountpoint: &str) {
    let usage = get_disk_usage(mountpoint);
    match usage {
      Ok(u) => {
        self.emitter.emit(&(String::from("disk-usage.") + mountpoint), u.blocks);
        self.emitter.emit(&(String::from("inode-usage.") + mountpoint), u.inodes);
      },
      Err(_) => {}
    }
  }
}

impl<'a> shelby::ShelbyPlugin for DiskUsage<'a> {
  fn start(&mut self) {
    println!("starting disk usage plugin");
    shelby::schedule_repeating(move || {
      self.send();
    }, 120);
  }
}
