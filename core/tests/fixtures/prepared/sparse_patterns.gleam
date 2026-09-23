pub type Option(a) {
  Some(a)
  None
}

pub type Builder {
  Builder(name: Option(String))
}

fn name(builder: Builder) -> String {
  case builder.name {
    None -> "unnamed"
    Some(value) -> value
  }
}

pub type Wrapped(a) {
  Wrapped(a)
}

pub type Choice {
  Empty
  Full(Wrapped(String))
}

fn nested(value: Choice) -> String {
  case value {
    Empty -> "empty"
    Full(Wrapped(text)) -> text
  }
}

fn number(value: Option(Int)) -> String {
  case value {
    None -> "missing"
    Some(_) -> "number"
  }
}

pub type Grow(a) {
  Stop
  Grow(value: a, tail: Grow(List(a)))
}

fn first(value: Grow(a), fallback: a) -> a {
  case value {
    Stop -> fallback
    Grow(head, _) -> head
  }
}

pub fn main() {
  name(Builder(None))
  <> ":"
  <> nested(Empty)
  <> ":"
  <> number(Some(42))
  <> ":"
  <> first(Stop, "fallback")
}
