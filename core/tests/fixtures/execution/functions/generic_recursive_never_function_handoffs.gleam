pub type FunctionBox(output) {
  FunctionBox(function: fn(Int) -> output)
}

pub type ValueBox(value) {
  ValueBox(value)
}

pub type TupleFunctionBox(value) {
  TupleFunctionBox(function: fn(Int) -> #(value))
}

pub type CustomFunctionBox(value) {
  CustomFunctionBox(function: fn(Int) -> ValueBox(value))
}

fn tuple_diverge(_value: Int) -> #(value) {
  #(panic as "unreached tuple function")
}

fn custom_diverge(_value: Int) -> ValueBox(value) {
  ValueBox(panic as "unreached custom function")
}

const tuple_constant = tuple_diverge
const custom_constant = custom_diverge

fn tuple_from_reference() -> fn(Int) -> #(value) {
  tuple_diverge
}

fn tuple_from_closure() -> fn(Int) -> #(value) {
  fn(_value: Int) { #(panic as "unreached tuple closure") }
}

fn tuple_from_argument(function: fn(Int) -> #(value)) {
  let local = function
  local
}

fn tuple_from_call() -> fn(Int) -> #(value) {
  tuple_from_reference()
}

fn tuple_from_function_call(
  provider: fn() -> fn(Int) -> #(value),
) -> fn(Int) -> #(value) {
  let selected = provider()
  selected
}

fn tuple_from_tuple() -> fn(Int) -> #(value) {
  #(tuple_diverge).0
}

fn tuple_from_custom() -> fn(Int) -> #(value) {
  TupleFunctionBox(function: tuple_diverge).function
}

fn tuple_from_list() -> fn(Int) -> #(value) {
  let assert [function] = [tuple_diverge]
  function
}

fn tuple_from_list_case() -> fn(Int) -> #(value) {
  case [tuple_diverge] {
    [function] -> function
    _ -> tuple_constant
  }
}

fn tuple_provider(_value: input) -> fn(Int) -> #(output) {
  tuple_diverge
}

fn tuple_from_diverging_function_argument(
  provider: fn(input) -> fn(Int) -> #(output),
) -> fn(Int) -> #(output) {
  provider(panic as "unselected tuple provider argument")
}

fn tuple_from_bool(selector: Bool) -> fn(Int) -> #(value) {
  let selected = case selector {
    True -> tuple_diverge
    False -> tuple_constant
  }
  selected
}

fn tuple_from_int(selector: Int) -> fn(Int) -> #(value) {
  let selected = case selector {
    0 -> tuple_diverge
    _ -> tuple_constant
  }
  selected
}

fn tuple_from_string(selector: String) -> fn(Int) -> #(value) {
  let selected = case selector {
    "reference" -> tuple_diverge
    _ -> tuple_constant
  }
  selected
}

fn tuple_from_float(selector: Float) -> fn(Int) -> #(value) {
  let selected = case selector {
    1.0 -> tuple_diverge
    _ -> tuple_constant
  }
  selected
}

fn tuple_from_block() -> fn(Int) -> #(value) {
  let selected = {
    let _ = Nil
    tuple_diverge
  }
  selected
}

fn tuple_from_panic() -> fn(Int) -> #(value) {
  panic as "unselected tuple function panic"
}

fn tuple_from_nested_expression(
  provider: fn() -> fn(Int) -> #(value),
  selector: Int,
  bool_selector: Bool,
  string_selector: String,
  float_selector: Float,
) -> fn(Int) -> #(value) {
  let selected = case selector {
    0 -> tuple_from_reference()
    1 -> provider()
    2 -> #(tuple_diverge).0
    3 -> TupleFunctionBox(function: tuple_diverge).function
    4 -> {
      let assert [function] = [tuple_diverge]
      function
    }
    5 -> case bool_selector {
      True -> tuple_diverge
      False -> tuple_constant
    }
    6 -> case string_selector {
      "reference" -> tuple_diverge
      _ -> tuple_constant
    }
    7 -> case float_selector {
      1.0 -> tuple_diverge
      _ -> tuple_constant
    }
    8 -> {
      let _ = Nil
      tuple_diverge
    }
    _ -> panic as "unselected nested tuple function panic"
  }
  selected
}

fn custom_from_reference() -> fn(Int) -> ValueBox(value) {
  custom_diverge
}

fn custom_from_closure() -> fn(Int) -> ValueBox(value) {
  fn(_value: Int) { ValueBox(panic as "unreached custom closure") }
}

fn custom_from_argument(function: fn(Int) -> ValueBox(value)) {
  let local = function
  local
}

fn custom_from_call() -> fn(Int) -> ValueBox(value) {
  custom_from_reference()
}

fn custom_from_function_call(
  provider: fn() -> fn(Int) -> ValueBox(value),
) -> fn(Int) -> ValueBox(value) {
  let selected = provider()
  selected
}

fn custom_from_tuple() -> fn(Int) -> ValueBox(value) {
  #(custom_diverge).0
}

fn custom_from_custom() -> fn(Int) -> ValueBox(value) {
  let selected = CustomFunctionBox(function: custom_diverge).function
  selected
}

fn custom_from_list() -> fn(Int) -> ValueBox(value) {
  let assert [function] = [custom_diverge]
  function
}

fn custom_from_list_case() -> fn(Int) -> ValueBox(value) {
  case [custom_diverge] {
    [function] -> function
    _ -> custom_constant
  }
}

fn custom_provider(_value: input) -> fn(Int) -> ValueBox(output) {
  custom_diverge
}

fn custom_from_diverging_function_argument(
  provider: fn(input) -> fn(Int) -> ValueBox(output),
) -> fn(Int) -> ValueBox(output) {
  provider(panic as "unselected custom provider argument")
}

fn generic_diverge(_value: Int) -> output {
  panic as "unreached generic function"
}

fn generic_provider(_value: input) -> fn(Int) -> output {
  generic_diverge
}

fn generic_apply(provider: fn(input) -> output) -> output {
  provider(panic as "unselected generic provider argument")
}

fn generic_first(values: List(value), fallback: value) -> value {
  case values {
    [first] -> first
    _ -> fallback
  }
}

fn custom_from_bool(selector: Bool) -> fn(Int) -> ValueBox(value) {
  let selected = case selector {
    True -> custom_diverge
    False -> custom_constant
  }
  selected
}

fn custom_from_int(selector: Int) -> fn(Int) -> ValueBox(value) {
  let selected = case selector {
    0 -> custom_diverge
    _ -> custom_constant
  }
  selected
}

fn custom_from_string(selector: String) -> fn(Int) -> ValueBox(value) {
  let selected = case selector {
    "reference" -> custom_diverge
    _ -> custom_constant
  }
  selected
}

fn custom_from_float(selector: Float) -> fn(Int) -> ValueBox(value) {
  let selected = case selector {
    1.0 -> custom_diverge
    _ -> custom_constant
  }
  selected
}

fn custom_from_block() -> fn(Int) -> ValueBox(value) {
  let selected = {
    let _ = Nil
    custom_diverge
  }
  selected
}

fn custom_from_nested_int_case(
  outer: Bool,
  inner: Int,
) -> fn(Int) -> ValueBox(value) {
  case outer {
    True -> case inner {
      0 -> custom_diverge
      _ -> custom_constant
    }
    False -> custom_diverge
  }
}

fn custom_from_panic() -> fn(Int) -> ValueBox(value) {
  let selected = panic as "unselected custom function panic"
  selected
}

fn custom_from_nested_expression(
  provider: fn() -> fn(Int) -> ValueBox(value),
  selector: Int,
  bool_selector: Bool,
  string_selector: String,
  float_selector: Float,
) -> fn(Int) -> ValueBox(value) {
  let selected = case selector {
    0 -> custom_from_reference()
    1 -> provider()
    2 -> #(custom_diverge).0
    3 -> CustomFunctionBox(function: custom_diverge).function
    4 -> {
      let assert [function] = [custom_diverge]
      function
    }
    5 -> case bool_selector {
      True -> custom_diverge
      False -> custom_constant
    }
    6 -> case string_selector {
      "reference" -> custom_diverge
      _ -> custom_constant
    }
    7 -> case float_selector {
      1.0 -> custom_diverge
      _ -> custom_constant
    }
    8 -> {
      let _ = Nil
      custom_diverge
    }
    _ -> panic as "unselected nested custom function panic"
  }
  selected
}

fn same(function: fn(Int) -> value) {
  function == function
}

pub fn main() {
  let tuple_function_call = case True {
    True -> tuple_diverge
    False -> {
      let provider = tuple_provider
      provider(panic as "unselected tuple function argument")
    }
  }
  let custom_function_call = case True {
    True -> custom_diverge
    False -> {
      let provider = custom_provider
      provider(panic as "unselected custom function argument")
    }
  }
  let custom_nested_kind = case True {
    True -> case 0 {
      0 -> custom_diverge
      _ -> custom_constant
    }
    False -> custom_diverge
  }
  let custom_list_kind = case True {
    True -> case [custom_diverge] {
      [function] -> function
      _ -> custom_constant
    }
    False -> custom_diverge
  }
  let generic_function_call = case True {
    True -> generic_diverge
    False -> {
      let provider = generic_provider
      provider(panic as "unselected generic function argument")
    }
  }
  let generic_value_function_call = case True {
    True -> generic_diverge
    False -> generic_apply(generic_provider)
  }
  let generic_value_list = generic_first([generic_diverge], generic_diverge)
  let _ = #(
    tuple_function_call,
    custom_function_call,
    custom_nested_kind,
    custom_list_kind,
    generic_function_call,
    generic_value_function_call,
    generic_value_list,
  )

  #(
    same(tuple_constant),
    same(tuple_from_reference()),
    same(tuple_from_closure()),
    same(tuple_from_argument(tuple_diverge)),
    same(tuple_from_call()),
    same(tuple_from_function_call(tuple_from_reference)),
    same(tuple_from_tuple()),
    same(tuple_from_custom()),
    same(tuple_from_list()),
    same(tuple_from_list_case()),
    same(tuple_from_bool(True)),
    same(tuple_from_bool(False)),
    same(tuple_from_int(0)),
    same(tuple_from_int(1)),
    same(tuple_from_string("reference")),
    same(tuple_from_string("fallback")),
    same(tuple_from_float(1.0)),
    same(tuple_from_float(2.0)),
    same(tuple_from_block()),
    same(case True {
      True -> tuple_diverge
      False -> tuple_from_diverging_function_argument(tuple_provider)
    }),
    tuple_from_panic == tuple_from_panic,
    same(tuple_from_nested_expression(tuple_from_reference, 0, True, "reference", 1.0)),
    same(tuple_from_nested_expression(tuple_from_reference, 1, True, "reference", 1.0)),
    same(tuple_from_nested_expression(tuple_from_reference, 2, True, "reference", 1.0)),
    same(tuple_from_nested_expression(tuple_from_reference, 3, True, "reference", 1.0)),
    same(tuple_from_nested_expression(tuple_from_reference, 4, True, "reference", 1.0)),
    same(tuple_from_nested_expression(tuple_from_reference, 5, True, "reference", 1.0)),
    same(tuple_from_nested_expression(tuple_from_reference, 6, True, "reference", 1.0)),
    same(tuple_from_nested_expression(tuple_from_reference, 7, True, "reference", 1.0)),
    same(tuple_from_nested_expression(tuple_from_reference, 8, True, "reference", 1.0)),
    same(custom_constant),
    same(custom_from_reference()),
    same(custom_from_closure()),
    same(custom_from_argument(custom_diverge)),
    same(custom_from_call()),
    same(custom_from_function_call(custom_from_reference)),
    same(custom_from_tuple()),
    same(custom_from_custom()),
    same(custom_from_list()),
    same(custom_from_list_case()),
    same(custom_from_bool(True)),
    same(custom_from_bool(False)),
    same(custom_from_int(0)),
    same(custom_from_int(1)),
    same(custom_from_string("reference")),
    same(custom_from_string("fallback")),
    same(custom_from_float(1.0)),
    same(custom_from_float(2.0)),
    same(custom_from_block()),
    same(custom_from_nested_int_case(True, 0)),
    same(custom_from_nested_int_case(True, 1)),
    same(case True {
      True -> custom_diverge
      False -> custom_from_diverging_function_argument(custom_provider)
    }),
    custom_from_panic == custom_from_panic,
    same(custom_from_nested_expression(custom_from_reference, 0, True, "reference", 1.0)),
    same(custom_from_nested_expression(custom_from_reference, 1, True, "reference", 1.0)),
    same(custom_from_nested_expression(custom_from_reference, 2, True, "reference", 1.0)),
    same(custom_from_nested_expression(custom_from_reference, 3, True, "reference", 1.0)),
    same(custom_from_nested_expression(custom_from_reference, 4, True, "reference", 1.0)),
    same(custom_from_nested_expression(custom_from_reference, 5, True, "reference", 1.0)),
    same(custom_from_nested_expression(custom_from_reference, 6, True, "reference", 1.0)),
    same(custom_from_nested_expression(custom_from_reference, 7, True, "reference", 1.0)),
    same(custom_from_nested_expression(custom_from_reference, 8, True, "reference", 1.0)),
  )
}

// @geam:expect Tuple([Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true)])
