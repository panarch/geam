fn apply(value: Int) {
  value + 1
}

pub fn main() {
  let bool_value = case True {
    False | True -> 1
  }
  let string_value = case "b" {
    "a" | "b" -> 2
    _ -> 0
  }
  let float_value = case 2.5 {
    1.5 | 2.5 -> 3
    _ -> 0
  }
  let nil_value = case Nil {
    Nil | _ -> 4
  }
  let list_value = case [1, 2] {
    values | _ as values -> values == [1, 2]
  }
  let function_value = case apply {
    f | _ as f -> f(5)
  }

  bool_value + string_value + float_value + nil_value + function_value
    + case list_value {
      True -> 6
      False -> 0
    }
}

// geam:expect Int(22)
