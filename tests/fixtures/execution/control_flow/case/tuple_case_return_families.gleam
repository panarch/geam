fn choose_from_bool(flag: Bool) {
  case flag {
    True -> #(1, "bool")
    False -> #(0, "bool")
  }
}

fn choose_from_int(value: Int) {
  case value {
    1 -> #(1, "int")
    _ -> #(0, "int")
  }
}

fn choose_from_string(value: String) {
  case value {
    "one" -> #(1, "string")
    _ -> #(0, "string")
  }
}

fn choose_from_float(value: Float) {
  case value {
    1.5 -> #(1, "float")
    _ -> #(0, "float")
  }
}

pub fn main() {
  let a = {
    let marker = 1
    marker
    choose_from_bool(True)
  }
  let b = choose_from_int(1)
  let c = choose_from_string("one")
  let d = choose_from_float(1.5)

  #(a.0 + b.0 + c.0 + d.0, d.1)
}

// @geam:expect Tuple([Int(4), String("float")])
