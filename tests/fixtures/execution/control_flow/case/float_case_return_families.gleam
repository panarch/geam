fn add_half(value: Float) {
  value +. 0.5
}

fn add_one(value: Float) {
  value +. 1.0
}

fn float_case_int(value: Float) {
  case value {
    1.0 -> 1
    _ -> 0
  }
}

fn float_case_string(value: Float) {
  case value {
    1.0 -> "hit"
    _ -> "miss"
  }
}

fn float_case_bool(value: Float) {
  case value {
    1.0 -> True
    _ -> False
  }
}

fn float_case_nil(value: Float) {
  case value {
    1.0 -> Nil
    _ -> Nil
  }
}

fn float_case_float(value: Float) {
  case value {
    1.0 -> 1.5
    _ -> 0.0
  }
}

fn float_case_function(value: Float) {
  case value {
    1.0 -> add_half
    _ -> add_one
  }
}

pub fn main() {
  float_case_nil(1.0)
  let add = float_case_function(1.0)
  let string_score = case float_case_string(1.0) {
    "hit" -> 1.0
    _ -> 0.0
  }
  let bool_score = case float_case_bool(1.0) {
    True -> 1.0
    False -> 0.0
  }
  let int_score = case float_case_int(1.0) {
    1 -> 1.0
    _ -> 0.0
  }

  float_case_float(1.0)
  +. add(1.0)
  +. string_score
  +. bool_score
  +. int_score
}

// geam:expect Float(6.0)
