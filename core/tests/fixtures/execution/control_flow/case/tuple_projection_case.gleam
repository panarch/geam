pub fn main() {
  let nested = #(#(1, "one"), #(True, 2.5), #(3, #(4, "four")), Nil)
  let int_value = nested.0.0
  let string_value = nested.0.1
  let bool_value = nested.1.0
  let float_value = nested.1.1
  let tuple_value = nested.2.1
  nested.3

  case string_value {
    "one" ->
      case bool_value {
        True -> #(int_value + tuple_value.0, float_value)
        False -> #(0, 0.0)
      }
    _ -> #(0, 0.0)
  }
}

// @geam:expect Tuple([Int(5), Float(2.5)])
