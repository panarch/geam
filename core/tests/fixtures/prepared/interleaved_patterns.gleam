type DataFrame {
  Text(Int)
  Binary(Int)
}

type ControlFrame {
  Close(Int)
  Ping(Int)
  Pong(Int)
}

type Frame {
  Data(DataFrame)
  Control(ControlFrame)
  Continuation(Int)
}

fn encode(frame: Frame) -> Int {
  case frame {
    Data(Text(value)) -> value
    Control(Close(value)) -> value
    Data(Binary(value)) -> value
    Control(Pong(value)) -> value
    Control(Ping(value)) -> value
    Continuation(value) -> value
  }
}

fn encode_grouped(frame: Frame) -> Int {
  case frame {
    Data(data) ->
      case data {
        Text(value) -> value
        Binary(value) -> value
      }
    Control(control) ->
      case control {
        Close(value) -> value
        Pong(value) -> value
        Ping(value) -> value
      }
    Continuation(value) -> value
  }
}

fn frame(tag: Int) -> Frame {
  case tag {
    0 -> Data(Text(100))
    1 -> Data(Binary(101))
    2 -> Control(Close(102))
    3 -> Control(Ping(103))
    4 -> Control(Pong(104))
    _ -> Continuation(105)
  }
}

pub fn selected(tag: Int) -> Int {
  encode(frame(tag))
}

pub fn selected_grouped(tag: Int) -> Int {
  encode_grouped(frame(tag))
}
