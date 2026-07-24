pub type FunctionBox(value) {
  FunctionBox(function: fn(Int) -> value)
}

pub type ValueBox(value) {
  ValueBox(value: value)
}

fn diverge(_value: Int) -> value {
  panic as "diverging function"
}

const diverging_constant = diverge

fn provide_reference() {
  diverge
}

fn provide_closure() {
  fn(_value: Int) { panic as "diverging closure" }
}

fn provide_from_argument(function: fn(Int) -> value) {
  function
}

fn provide_from_function_call(provider: fn() -> fn(Int) -> value) {
  provider()
}

fn provide_from_tuple() {
  #(diverge).0
}

fn provide_from_custom() {
  FunctionBox(function: diverge).function
}

fn provide_from_list() {
  let assert [function] = [diverge]
  function
}

fn provide_from_bool_case(selector: Bool) {
  case selector {
    True -> diverge
    False -> diverging_constant
  }
}

fn provide_from_int_case(selector: Int) {
  case selector {
    0 -> diverge
    _ -> diverging_constant
  }
}

fn provide_from_string_case(selector: String) {
  case selector {
    "reference" -> diverge
    _ -> diverging_constant
  }
}

fn provide_from_float_case(selector: Float) {
  case selector {
    1.0 -> diverge
    _ -> diverging_constant
  }
}

fn provide_from_block() {
  let _ = Nil
  diverge
}

fn transform_function(
  function: fn(Int) -> value,
  selector: Int,
  bool_selector: Bool,
  string_selector: String,
  float_selector: Float,
) {
  let local = function
  let provider = fn() { function }
  let from_bool = case bool_selector {
    True -> function
    False -> panic as "unselected function branch"
  }
  let from_string = case string_selector {
    "selected" -> from_bool
    _ -> function
  }
  let from_float = case float_selector {
    1.0 -> from_string
    _ -> function
  }
  let selected = case selector {
    0 -> diverging_constant
    1 -> diverge
    2 -> fn(_value: Int) { panic as "transformed closure" }
    3 -> local
    4 -> provide_reference()
    5 -> provider()
    6 -> #(function).0
    7 -> FunctionBox(function: function).function
    8 -> {
      let assert [first] = [function]
      first
    }
    9 -> from_float
    _ -> {
      let _ = Nil
      function
    }
  }
  selected
}

fn identity(value: value) {
  value
}

fn retain_value(value, count: Int) {
  case count {
    0 -> value
    _ -> retain_value(value, count - 1)
  }
}

fn generic_identity(value) {
  value
}

fn transform_value(
  value: value,
  selector: Int,
  bool_selector: Bool,
  string_selector: String,
  float_selector: Float,
) {
  let local = value
  let provider = fn() { value }
  let from_bool = case bool_selector {
    True -> value
    False -> panic as "unselected value branch"
  }
  let from_string = case string_selector {
    "selected" -> from_bool
    _ -> value
  }
  let from_float = case float_selector {
    1.0 -> from_string
    _ -> value
  }
  let selected = case selector {
    0 -> local
    1 -> identity(value)
    2 -> provider()
    3 -> #(value).0
    4 -> ValueBox(value: value).value
    5 -> {
      let assert [first] = [value]
      first
    }
    6 -> from_float
    _ -> {
      let _ = Nil
      value
    }
  }
  selected
}

fn same(function: fn(Int) -> value) {
  function == function
}

pub fn main() {
  let local = diverge
  let provider = provide_reference
  let symbolic = retain_value(generic_identity, 1)

  #(
    same(diverging_constant),
    same(diverge),
    same(provide_closure()),
    same(local),
    same(provide_reference()),
    same(provide_from_argument(diverge)),
    same(provide_from_function_call(provider)),
    same(provide_from_tuple()),
    same(provide_from_custom()),
    same(provide_from_list()),
    same(provide_from_bool_case(True)),
    same(provide_from_int_case(0)),
    same(provide_from_string_case("reference")),
    same(provide_from_string_case("fallback")),
    same(provide_from_float_case(1.0)),
    same(provide_from_float_case(2.0)),
    same(provide_from_block()),
    same(transform_function(diverge, 0, True, "selected", 1.0)),
    same(transform_function(diverge, 1, True, "selected", 1.0)),
    same(transform_function(diverge, 2, True, "selected", 1.0)),
    same(transform_function(diverge, 3, True, "selected", 1.0)),
    same(transform_function(diverge, 4, True, "selected", 1.0)),
    same(transform_function(diverge, 5, True, "selected", 1.0)),
    same(transform_function(diverge, 6, True, "selected", 1.0)),
    same(transform_function(diverge, 7, True, "selected", 1.0)),
    same(transform_function(diverge, 8, True, "selected", 1.0)),
    same(transform_function(diverge, 9, True, "selected", 1.0)),
    same(transform_function(diverge, 10, True, "selected", 1.0)),
    same(transform_value(diverge, 0, True, "selected", 1.0)),
    same(transform_value(diverge, 1, True, "selected", 1.0)),
    same(transform_value(diverge, 2, True, "selected", 1.0)),
    same(transform_value(diverge, 3, True, "selected", 1.0)),
    same(transform_value(diverge, 4, True, "selected", 1.0)),
    same(transform_value(diverge, 5, True, "selected", 1.0)),
    same(transform_value(diverge, 6, True, "selected", 1.0)),
    same(transform_value(diverge, 7, True, "selected", 1.0)),
    same(retain_value(diverge, 1)),
    symbolic == symbolic,
  )
}

// @geam:expect Tuple([Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true)])
