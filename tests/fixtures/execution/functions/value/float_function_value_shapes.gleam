fn add_half(value: Float) {
  value +. 0.5
}

fn add_one(value: Float) {
  value +. 1.0
}

fn get_by_bool(flag: Bool) {
  case flag {
    True -> add_half
    False -> add_one
  }
}

fn get_by_int(value: Int) {
  case value {
    1 -> add_half
    _ -> add_one
  }
}

fn get_by_string(value: String) {
  case value {
    "hit" -> add_half
    _ -> add_one
  }
}

fn get_by_float(value: Float) {
  case value {
    1.0 -> add_half
    _ -> add_one
  }
}

fn get_by_block() {
  {
    let _ = 0.0
    add_half
  }
}

pub fn main() {
  let bool_add = get_by_bool(True)
  let bool_fallback = get_by_bool(False)
  let int_add = get_by_int(1)
  let int_fallback = get_by_int(0)
  let string_add = get_by_string("hit")
  let string_fallback = get_by_string("miss")
  let float_add = get_by_float(1.0)
  let float_fallback = get_by_float(0.0)
  let block_add = get_by_block()

  bool_add(1.0)
  +. bool_fallback(1.0)
  +. int_add(1.0)
  +. int_fallback(1.0)
  +. string_add(1.0)
  +. string_fallback(1.0)
  +. float_add(1.0)
  +. float_fallback(1.0)
  +. block_add(1.0)
}

// geam:expect Float(15.5)
