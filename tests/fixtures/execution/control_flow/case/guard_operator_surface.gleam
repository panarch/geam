fn int_ordering(value: Int) {
  case value {
    other if other >= 5 && other <= 5 && other < 6 -> True
    _ -> False
  }
}

fn int_arithmetic(value: Int) {
  case value {
    other if other + 1 == 6 && other - 1 == 4 && other * 2 == 10 && other / 2 == 2 && other % 2 == 1 -> True
    _ -> False
  }
}

fn float_operators(value: Float) {
  case value {
    other if other >=. 1.5 && other <=. 1.5 && other <. 2.0 && other +. 0.5 == 2.0 && other -. 0.5 == 1.0 && other *. 2.0 == 3.0 && other /. 1.5 == 1.0 -> True
    _ -> False
  }
}

fn string_operators(value: String) {
  case value {
    other if other <> "am" == "geam" && other != "no" -> True
    _ -> False
  }
}

fn bool_operators(value: Bool) {
  case value {
    other if !False && other -> True
    _ -> False
  }
}

fn bool_or_operator(value: Bool) {
  case value {
    other if other || False -> True
    _ -> False
  }
}

fn tuple_index_guard(pair: #(Int, String)) {
  case pair.0 {
    value if pair.1 == "ok" && value == 1 -> True
    _ -> False
  }
}

pub fn main() {
  let int_ok = int_ordering(5) && int_arithmetic(5)
  let float_ok = float_operators(1.5)
  let string_ok = string_operators("ge")
  let bool_ok = bool_operators(True) && bool_or_operator(True)
  let tuple_ok = tuple_index_guard(#(1, "ok"))

  int_ok && float_ok && string_ok && bool_ok && tuple_ok
}

// geam:expect Bool(true)
