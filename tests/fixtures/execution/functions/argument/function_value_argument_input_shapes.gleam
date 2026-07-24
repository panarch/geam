fn append_suffix(value: String) {
  value <> "-arg"
}

fn invert(value: Bool) {
  !value
}

fn accept_nil(value: Nil) {
  value
}

fn apply_string(function: fn(String) -> String, value: String) {
  function(value)
}

fn apply_bool(function: fn(Bool) -> Bool, value: Bool) {
  function(value)
}

fn apply_nil(function: fn(Nil) -> Nil, value: Nil) {
  function(value)
}

pub fn main() {
  let string_result = apply_string(append_suffix, "geam")
  let bool_result = apply_bool(invert, False)

  apply_nil(accept_nil, Nil)

  case string_result == "geam-arg" && bool_result {
    True -> 42
    False -> 0
  }
}

// @geam:expect Int(42)
