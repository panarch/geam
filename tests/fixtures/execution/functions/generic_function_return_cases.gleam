fn identity(value: value) -> value {
  value
}

fn choose_bool(selector: Bool) -> fn(value) -> value {
  case selector {
    True -> identity
    False -> identity
  }
}

fn choose_int(selector: Int) -> fn(value) -> value {
  case selector {
    0 -> identity
    _ -> identity
  }
}

fn choose_string(selector: String) -> fn(value) -> value {
  case selector {
    "left" -> identity
    _ -> identity
  }
}

fn choose_float(selector: Float) -> fn(value) -> value {
  case selector {
    1.0 -> identity
    _ -> identity
  }
}

fn forward_bool(selector: Bool) -> fn(value) -> value {
  choose_bool(selector)
}

fn choose_block(selector: Int) -> fn(value) -> value {
  let _ = selector
  identity
}

fn choose_function(function: fn(value) -> value) -> fn(value) -> value {
  case function {
    candidate -> candidate
  }
}

pub fn main() {
  let from_bool = choose_bool(True)
  let from_int = choose_int(0)
  let from_string = choose_string("left")
  let from_float = choose_float(1.0)
  let forwarded = forward_bool(False)
  let from_block = choose_block(1)
  let from_function = choose_function(identity)

  #(
    from_bool(1),
    from_int("two"),
    from_string(3),
    from_float("four"),
    forwarded(5),
    from_block("six"),
    from_function(True),
  )
}

// @geam:expect Tuple([Int(1), String("two"), Int(3), String("four"), Int(5), String("six"), Bool(true)])
