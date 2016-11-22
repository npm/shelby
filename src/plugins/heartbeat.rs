use forza;

pub struct Heartbeat {
}

impl Heartbeat {
  pub fn new() -> Heartbeat {
    Heartbeat {}
  }
}

impl forza::ForzaPlugin for Heartbeat {
  fn start(&self) {
    println!("starting heartbeat");
  }

  fn stop(&self) {
    println!("stopping heartbeat");
  }
}
