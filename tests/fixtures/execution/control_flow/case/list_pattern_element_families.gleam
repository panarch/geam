fn add_one(value: Int) {
  value + 1
}

fn string_identity(value: String) {
  value
}

fn float_identity(value: Float) {
  value
}

fn bool_identity(value: Bool) {
  value
}

fn nil_identity(_: Nil) {
  Nil
}

fn tuple_identity(value: #(Int, String)) {
  value
}

fn list_identity(value: List(Int)) {
  value
}

fn function_identity(value: fn(Int) -> Int) {
  value
}

fn first_ok(values: List(Result(Int, Nil))) {
  case values {
    [Ok(value)] -> value
    _ -> 0
  }
}

pub fn main() {
  let string_ok = case ["one"] {
    ["one"] -> True
    _ -> False
  }
  let float_ok = case [1.5] {
    [1.5] -> True
    _ -> False
  }
  let bool_ok = case [False] {
    [False] -> True
    _ -> False
  }
  let bool_true_ok = case [True] {
    [True] -> True
    _ -> False
  }
  let nil_ok = case [Nil] {
    [Nil] -> True
    _ -> False
  }
  let nested_list_ok = case [[1]] {
    [[value]] -> value == 1
    _ -> False
  }
  let tuple_ok = case [#(1, "one")] {
    [#(left, right)] -> left == 1 && right == "one"
    _ -> False
  }
  let int_function_ok = case [add_one] {
    [f] -> f(41) == 42
    _ -> False
  }
  let string_function_ok = case [string_identity] {
    [f] -> f("one") == "one"
    _ -> False
  }
  let float_function_ok = case [float_identity] {
    [f] -> f(1.5) == 1.5
    _ -> False
  }
  let bool_function_ok = case [bool_identity] {
    [f] -> f(True)
    _ -> False
  }
  let nil_function_ok = case [nil_identity] {
    [f] -> f(Nil) == Nil
    _ -> False
  }
  let tuple_function_ok = case [tuple_identity] {
    [f] -> f(#(1, "one")) == #(1, "one")
    _ -> False
  }
  let list_function_ok = case [list_identity] {
    [f] -> f([1]) == [1]
    _ -> False
  }
  let function_function_ok = case [function_identity] {
    [f] -> f(add_one)(41) == 42
    _ -> False
  }
  let custom_ok = first_ok([Ok(1)]) == 1

  string_ok && float_ok && bool_ok && bool_true_ok && nil_ok && nested_list_ok
  && tuple_ok && int_function_ok && string_function_ok && float_function_ok
  && bool_function_ok && nil_function_ok && tuple_function_ok && list_function_ok
  && function_function_ok && custom_ok
}

// @geam:expect Bool(true)
