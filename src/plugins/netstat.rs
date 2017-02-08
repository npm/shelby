extern crate linux_stats;
extern crate numbat;

use shelby;
use plugins::netstat::linux_stats::SocketState;

pub struct Netstat<'a> {
  emitter: numbat::Emitter<'a>
}

impl<'a> Netstat<'a> {
  pub fn new(emitter: numbat::Emitter<'a>) -> Netstat {
    Netstat {
      emitter: emitter
    }
  }

  fn send(&mut self) {
    let check = linux_stats::tcp();
    if !check.is_ok() {
      return;
    }

    let tcp = check.unwrap();
    let (mut established, mut syn_sent, mut syn_recv, mut fin_wait1,
         mut fin_wait2, mut time_wait, mut close, mut close_wait, mut last_ack,
         mut listen, mut closing) =
      (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);

    self.emitter.emit("sockets", tcp.len());

    for socket in tcp.iter() {
      match socket.state {
        SocketState::Established => established += 1,
        SocketState::SynSent => syn_sent += 1,
        SocketState::SynRecv => syn_recv += 1,
        SocketState::FinWait1 => fin_wait1 += 1,
        SocketState::FinWait2 => fin_wait2 += 1,
        SocketState::TimeWait => time_wait += 1,
        SocketState::Close => close += 1,
        SocketState::CloseWait => close_wait += 1,
        SocketState::LastAck => last_ack += 1,
        SocketState::Listen => listen += 1,
        SocketState::Closing => closing += 1
      }
    }

    self.emitter.emit("sockets.ESTABLISHED", established);
    self.emitter.emit("sockets.SYN_SENT", syn_sent);
    self.emitter.emit("sockets.SYN_RECV", syn_recv);
    self.emitter.emit("sockets.FIN_WAIT1", fin_wait1);
    self.emitter.emit("sockets.FIN_WAIT2", fin_wait2);
    self.emitter.emit("sockets.TIME_WAIT", time_wait);
    self.emitter.emit("sockets.CLOSE", close);
    self.emitter.emit("sockets.CLOSE_WAIT", close_wait);
    self.emitter.emit("sockets.LAST_ACK", last_ack);
    self.emitter.emit("sockets.LISTEN", listen);
    self.emitter.emit("sockets.CLOSING", closing);
  }
}

impl<'a> shelby::ShelbyPlugin for Netstat<'a> {
  fn start(&mut self) {
    println!("starting netstat plugin");
    shelby::schedule_repeating(move || {
      self.send();
    }, 10);
  }
}
