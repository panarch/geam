pub type Control {
  Ping
}

pub type InternalMessage {
  ReceiveMessage(Int)
  Closed
  Passive
  SocketError(Int)
  Ready
  Close
}

pub type Message {
  Internal(InternalMessage)
  User(Control)
}

pub fn choose(message: Message) -> Int {
  case message {
    Internal(Closed) | Internal(Close) -> 0
    Internal(Ready) -> 1
    User(_) -> 2
    Internal(ReceiveMessage(_)) -> 3
    Internal(Passive) -> 4
    Internal(SocketError(reason)) -> reason
  }
}

pub fn main() {
  let assert 0 = choose(Internal(Closed))
  let assert 0 = choose(Internal(Close))
  let assert 1 = choose(Internal(Ready))
  let assert 2 = choose(User(Ping))
  let assert 3 = choose(Internal(ReceiveMessage(13)))
  let assert 4 = choose(Internal(Passive))
  let assert 9 = choose(Internal(SocketError(9)))
  Nil
}
// @geam:expect Nil
