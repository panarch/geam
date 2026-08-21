fn int_to_string(value: Int) {
  case value == 1 {
    True -> "one"
    False -> "other"
  }
}

fn int_to_bool(value: Int) {
  value == 1
}

fn int_to_nil(value: Int) {
  case value == 0 {
    True -> Nil
    False -> Nil
  }
}

fn int_to_tuple(value: Int) {
  #(value, "tuple")
}

fn apply_int_to_string(function: fn(Int) -> String, value: Int) {
  function(value)
}

fn apply_int_to_bool(function: fn(Int) -> Bool, value: Int) {
  function(value)
}

fn apply_int_to_nil(function: fn(Int) -> Nil, value: Int) {
  function(value)
}

fn apply_int_to_tuple(function: fn(Int) -> #(Int, String), value: Int) {
  function(value)
}

pub fn main() {
  let string_result = apply_int_to_string(int_to_string, 1)
  let bool_result = apply_int_to_bool(int_to_bool, 1)
  let tuple_result = apply_int_to_tuple(int_to_tuple, 42)

  apply_int_to_nil(int_to_nil, 0)

  case string_result == "one" && bool_result {
    True -> tuple_result.0
    False -> 0
  }
}

// @geam:expect Int(42)
