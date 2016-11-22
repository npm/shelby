use std;
use std::collections::BTreeMap;
use std::collections::LinkedList;

extern crate numbat;

pub trait ForzaPlugin {
  fn start(&self);
  fn stop(&self);
}

pub struct Forza<'a> {
  started: bool,
  emitter: numbat::Emitter<'a>,
  plugins: LinkedList<Box<ForzaPlugin>>
}

impl<'a> Forza<'a> {
  pub fn start(&mut self) {
    self.started = true;
    for plugin in self.plugins.iter() {
      plugin.start();
    }
  }

  pub fn stop(&mut self) {
    self.started = false;
  }

  pub fn new(dest: &str, plugins: LinkedList<Box<ForzaPlugin>>) -> Forza<'a> {
    let mut emitter = numbat::Emitter::new(BTreeMap::new(), "forza");
    emitter.connect(dest);
    return Forza {
      started: false,
      emitter: emitter,
      plugins: plugins
    }
  }
}
