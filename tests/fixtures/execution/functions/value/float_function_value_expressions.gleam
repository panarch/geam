fn add_half(value: Float) {
  value +. 0.5
}

fn add_one(value: Float) {
  value +. 1.0
}

fn get_add_half() {
  add_half
}

pub fn main() {
  let bool_add = case True {
    True -> add_half
    False -> add_one
  }
  let int_add = case 1 {
    1 -> add_half
    _ -> add_one
  }
  let string_add = case "hit" {
    "hit" -> add_half
    _ -> add_one
  }
  let float_add = case 1.0 {
    1.0 -> add_half
    _ -> add_one
  }
  let block_add = {
    let _ = 0.0
    add_half
  }
  let direct_call_add = get_add_half()
  let provider = get_add_half
  let function_call_add = provider()

  bool_add(1.0)
  +. int_add(1.0)
  +. string_add(1.0)
  +. float_add(1.0)
  +. block_add(1.0)
  +. direct_call_add(1.0)
  +. function_call_add(1.0)
}

// geam:expect Float(10.5)
