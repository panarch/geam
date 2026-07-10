fn ones() {
  #(1)
}

fn twos() {
  #(2)
}

fn get_by_bool(flag: Bool) {
  case flag {
    True -> ones
    False -> twos
  }
}

fn get_by_int(value: Int) {
  case value {
    1 -> ones
    _ -> twos
  }
}

fn get_by_string(value: String) {
  case value {
    "hit" -> ones
    _ -> twos
  }
}

fn get_by_float(value: Float) {
  case value {
    1.0 -> ones
    _ -> twos
  }
}

fn get_by_block() {
  {
    let _ = 0
    ones
  }
}

fn first(function: fn() -> #(Int)) {
  function().0
}

pub fn main() {
  first(get_by_bool(True))
  + first(get_by_bool(False))
  + first(get_by_int(1))
  + first(get_by_int(0))
  + first(get_by_string("hit"))
  + first(get_by_string("miss"))
  + first(get_by_float(1.0))
  + first(get_by_float(0.0))
  + first(get_by_block())
}

// geam:expect Int(13)
