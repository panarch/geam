pub type ValueBox(value) {
  ValueBox(value: value)
}

fn fail() -> value {
  panic as "unreached generic argument"
}

fn tuple_argument(value: value) -> #(value) {
  #(value)
}

fn custom_argument(value: value) -> ValueBox(value) {
  ValueBox(value: value)
}

fn tuple_leaf(_value: Int) -> #(value) {
  #(panic as "unreached tuple leaf")
}

fn custom_leaf(_value: Int) -> ValueBox(value) {
  ValueBox(value: panic as "unreached custom leaf")
}

fn tuple_value(_selector: Int) -> #(#(value)) {
  #(#(panic as "unreached tuple value"))
}

fn tuple_call(_selector: Int) -> #(#(value)) {
  #(tuple_leaf(0))
}

fn tuple_function_call(_selector: Int) -> #(#(value)) {
  let function = tuple_leaf
  #(function(0))
}

fn tuple_diverging_call(_selector: Int) -> #(#(value)) {
  #(tuple_argument(fail()))
}

fn tuple_diverging_function_call(_selector: Int) -> #(#(value)) {
  let function = tuple_argument
  #(function(fail()))
}

fn tuple_tuple_index(_selector: Int) -> #(#(value)) {
  #(#(#(panic as "unreached nested tuple projection")).0)
}

fn tuple_custom_field(_selector: Int) -> #(#(value)) {
  #(ValueBox(value: #(panic as "unreached tuple custom field")).value)
}

fn tuple_panic(_selector: Int) -> #(#(value)) {
  #(panic as "unreached tuple panic")
}

fn tuple_bool_case(selector: Int) -> #(#(value)) {
  #(case selector == 0 {
    True -> tuple_leaf(0)
    False -> tuple_leaf(1)
  })
}

fn tuple_int_case(selector: Int) -> #(#(value)) {
  #(case selector {
    0 -> tuple_leaf(0)
    _ -> tuple_leaf(1)
  })
}

fn tuple_string_case(selector: Int) -> #(#(value)) {
  #(case case selector { 0 -> "selected" _ -> "fallback" } {
    "selected" -> tuple_leaf(0)
    _ -> tuple_leaf(1)
  })
}

fn tuple_float_case(selector: Int) -> #(#(value)) {
  #(case case selector { 0 -> 1.0 _ -> 2.0 } {
    1.0 -> tuple_leaf(0)
    _ -> tuple_leaf(1)
  })
}

fn tuple_block(_selector: Int) -> #(#(value)) {
  #({
    let _ = Nil
    tuple_leaf(0)
  })
}

fn tuple_diverging_block(_selector: Int) -> #(#(value)) {
  #({
    let _ = panic as "unreached tuple block step"
    tuple_leaf(0)
  })
}

fn custom_value(_selector: Int) -> #(ValueBox(value)) {
  #(ValueBox(value: panic as "unreached custom value"))
}

fn custom_call(_selector: Int) -> #(ValueBox(value)) {
  #(custom_leaf(0))
}

fn custom_function_call(_selector: Int) -> #(ValueBox(value)) {
  let function = custom_leaf
  #(function(0))
}

fn custom_diverging_call(_selector: Int) -> #(ValueBox(value)) {
  #(custom_argument(fail()))
}

fn custom_diverging_function_call(_selector: Int) -> #(ValueBox(value)) {
  let function = custom_argument
  #(function(fail()))
}

fn custom_tuple_index(_selector: Int) -> #(ValueBox(value)) {
  #(#(ValueBox(value: panic as "unreached custom tuple projection")).0)
}

fn custom_custom_field(_selector: Int) -> #(ValueBox(value)) {
  #(ValueBox(value: ValueBox(value: panic as "unreached nested custom field")).value)
}

fn custom_panic(_selector: Int) -> #(ValueBox(value)) {
  #(panic as "unreached custom panic")
}

fn custom_bool_case(selector: Int) -> #(ValueBox(value)) {
  #(case selector == 0 {
    True -> custom_leaf(0)
    False -> custom_leaf(1)
  })
}

fn custom_int_case(selector: Int) -> #(ValueBox(value)) {
  #(case selector {
    0 -> custom_leaf(0)
    _ -> custom_leaf(1)
  })
}

fn custom_string_case(selector: Int) -> #(ValueBox(value)) {
  #(case case selector { 0 -> "selected" _ -> "fallback" } {
    "selected" -> custom_leaf(0)
    _ -> custom_leaf(1)
  })
}

fn custom_float_case(selector: Int) -> #(ValueBox(value)) {
  #(case case selector { 0 -> 1.0 _ -> 2.0 } {
    1.0 -> custom_leaf(0)
    _ -> custom_leaf(1)
  })
}

fn custom_block(_selector: Int) -> #(ValueBox(value)) {
  #({
    let _ = Nil
    custom_leaf(0)
  })
}

fn custom_diverging_block(_selector: Int) -> #(ValueBox(value)) {
  #({
    let _ = panic as "unreached custom block step"
    custom_leaf(0)
  })
}

fn same(function: fn(Int) -> value) {
  function == function
}

pub fn main() {
  #(
    same(tuple_value),
    same(tuple_call),
    same(tuple_function_call),
    same(tuple_diverging_call),
    same(tuple_diverging_function_call),
    same(tuple_tuple_index),
    same(tuple_custom_field),
    same(tuple_panic),
    same(tuple_bool_case),
    same(tuple_int_case),
    same(tuple_string_case),
    same(tuple_float_case),
    same(tuple_block),
    same(tuple_diverging_block),
    same(custom_value),
    same(custom_call),
    same(custom_function_call),
    same(custom_diverging_call),
    same(custom_diverging_function_call),
    same(custom_tuple_index),
    same(custom_custom_field),
    same(custom_panic),
    same(custom_bool_case),
    same(custom_int_case),
    same(custom_string_case),
    same(custom_float_case),
    same(custom_block),
    same(custom_diverging_block),
  )
}

// geam:expect Tuple([Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true)])
