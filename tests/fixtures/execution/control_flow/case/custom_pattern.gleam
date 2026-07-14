pub type Token {
  Empty
  Full(Int)
  Labelled(value: Int, name: String)
}

fn inspect(token: Token) {
  case token {
    Empty -> 0
    Full(value) if value > 0 -> value
    Labelled(name: name, value: value) -> value + 10
    _ -> -1
  }
}

fn exhaustive(token: Token) {
  case token {
    Empty -> 0
    Full(_) -> 1
    Labelled(value: _, name: _) -> 2
  }
}

fn bind_whole(token: Token) {
  case token {
    value -> value == token
  }
}

fn inferred_variant() {
  let token = Full(7)
  case token {
    Full(value) -> value
  }
}

fn nested_inferred_variant() {
  let token = #(Full(8))
  case token {
    #(Full(value)) -> value
  }
}

fn partial_labelled_pattern(token: Token) {
  case token {
    Labelled(value: value, ..) -> value
    _ -> 0
  }
}

fn multi_subject_inferred_variant() {
  let token = Full(9)
  case token, 0 {
    Full(value), _ -> value
  }
}

pub fn main() {
  #(
    inspect(Empty),
    inspect(Full(2)),
    inspect(Labelled(value: 3, name: "three")),
    exhaustive(Labelled(value: 3, name: "three")),
    bind_whole(Full(2)),
    inferred_variant(),
    nested_inferred_variant(),
    multi_subject_inferred_variant(),
    partial_labelled_pattern(Labelled(value: 10, name: "ten")),
  )
}

// geam:expect Tuple([Int(0), Int(2), Int(13), Int(2), Bool(true), Int(7), Int(8), Int(9), Int(10)])
