pub type CallableBox(input, output) {
  CallableBox(function: fn(input) -> output)
}

pub type ListBox(value) {
  ListBox(values: List(value))
}

pub type NestedListBox(value) {
  NestedListBox(values: List(List(value)))
}

pub type ValueBox(value) {
  ValueBox(value: value)
}

fn identity(value: value) {
  value
}

fn int_function(_value) { 1 }
fn float_function(_value) { 1.5 }
fn string_function(_value) { "one" }
fn bit_array_function(_value) { <<1>> }
fn utf_codepoint_function(_value) -> UtfCodepoint {
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  codepoint
}
fn custom_function(_value) { ValueBox(value: Nil) }
fn bool_function(_value) { True }
fn nil_function(_value) { Nil }
fn tuple_function(_value) { #(1, "one") }
fn list_function(_value) { [1] }
fn function_function(_value) { fn(value: Int) { value } }

const int_function_constant = int_function
const float_function_constant = float_function
const string_function_constant = string_function
const bit_array_function_constant = bit_array_function
const utf_codepoint_function_constant = utf_codepoint_function
const custom_function_constant = custom_function
const bool_function_constant = bool_function
const nil_function_constant = nil_function
const tuple_function_constant = tuple_function
const list_function_constant = list_function
const function_function_constant = function_function

fn exercise_int_function(
  function: fn(input) -> Int,
  bool_selector: Bool,
  int_selector: Int,
  string_selector: String,
  float_selector: Float,
) -> fn(input) -> Int {
  let local = function
  let from_list = case [local] {
    [first] -> first
    _ -> function
  }
  let from_block = {
    let _ = Nil
    from_list
  }
  let from_int = case int_selector {
    0 -> int_function_constant
    1 -> int_function
    2 -> fn(value) { function(value) }
    3 -> local
    4 -> exercise_int_function(function, False, 0, "", 0.0)
    5 -> {
      let provider = fn() { function }
      provider()
    }
    6 -> #(function).0
    7 -> CallableBox(function: function).function
    8 -> from_block
    _ -> panic as "unselected symbolic Int function"
  }
  let from_string = case string_selector {
    "selected" -> from_int
    _ -> function
  }
  let from_float = case float_selector {
    1.0 -> from_string
    _ -> function
  }
  case bool_selector {
    True -> from_float
    False -> function
  }
}

fn exercise_float_function(
  function: fn(input) -> Float,
  bool_selector: Bool,
  int_selector: Int,
  string_selector: String,
  float_selector: Float,
) -> fn(input) -> Float {
  let local = function
  let from_list = case [local] {
    [first] -> first
    _ -> function
  }
  let from_block = {
    let _ = Nil
    from_list
  }
  let from_int = case int_selector {
    0 -> float_function_constant
    1 -> float_function
    2 -> fn(value) { function(value) }
    3 -> local
    4 -> exercise_float_function(function, False, 0, "", 0.0)
    5 -> {
      let provider = fn() { function }
      provider()
    }
    6 -> #(function).0
    7 -> CallableBox(function: function).function
    8 -> from_block
    _ -> panic as "unselected symbolic Float function"
  }
  let from_string = case string_selector {
    "selected" -> from_int
    _ -> function
  }
  let from_float = case float_selector {
    1.0 -> from_string
    _ -> function
  }
  case bool_selector {
    True -> from_float
    False -> function
  }
}

fn exercise_string_function(
  function: fn(input) -> String,
  bool_selector: Bool,
  int_selector: Int,
  string_selector: String,
  float_selector: Float,
) -> fn(input) -> String {
  let local = function
  let from_list = case [local] {
    [first] -> first
    _ -> function
  }
  let from_block = {
    let _ = Nil
    from_list
  }
  let from_int = case int_selector {
    0 -> string_function_constant
    1 -> string_function
    2 -> fn(value) { function(value) }
    3 -> local
    4 -> exercise_string_function(function, False, 0, "", 0.0)
    5 -> {
      let provider = fn() { function }
      provider()
    }
    6 -> #(function).0
    7 -> CallableBox(function: function).function
    8 -> from_block
    _ -> panic as "unselected symbolic String function"
  }
  let from_string = case string_selector {
    "selected" -> from_int
    _ -> function
  }
  let from_float = case float_selector {
    1.0 -> from_string
    _ -> function
  }
  case bool_selector {
    True -> from_float
    False -> function
  }
}

fn exercise_bit_array_function(
  function: fn(input) -> BitArray,
  bool_selector: Bool,
  int_selector: Int,
  string_selector: String,
  float_selector: Float,
) -> fn(input) -> BitArray {
  let local = function
  let from_list = case [local] {
    [first] -> first
    _ -> function
  }
  let from_block = {
    let _ = Nil
    from_list
  }
  let from_int = case int_selector {
    0 -> bit_array_function_constant
    1 -> bit_array_function
    2 -> fn(value) { function(value) }
    3 -> local
    4 -> exercise_bit_array_function(function, False, 0, "", 0.0)
    5 -> {
      let provider = fn() { function }
      provider()
    }
    6 -> #(function).0
    7 -> CallableBox(function: function).function
    8 -> from_block
    _ -> panic as "unselected symbolic BitArray function"
  }
  let from_string = case string_selector {
    "selected" -> from_int
    _ -> function
  }
  let from_float = case float_selector {
    1.0 -> from_string
    _ -> function
  }
  case bool_selector {
    True -> from_float
    False -> function
  }
}

fn exercise_utf_codepoint_function(
  function: fn(input) -> UtfCodepoint,
  bool_selector: Bool,
  int_selector: Int,
  string_selector: String,
  float_selector: Float,
) -> fn(input) -> UtfCodepoint {
  let local = function
  let from_list = case [local] {
    [first] -> first
    _ -> function
  }
  let from_block = {
    let _ = Nil
    from_list
  }
  let from_int = case int_selector {
    0 -> utf_codepoint_function_constant
    1 -> utf_codepoint_function
    2 -> fn(value) { function(value) }
    3 -> local
    4 -> exercise_utf_codepoint_function(function, False, 0, "", 0.0)
    5 -> {
      let provider = fn() { function }
      provider()
    }
    6 -> #(function).0
    7 -> CallableBox(function: function).function
    8 -> from_block
    _ -> panic as "unselected symbolic UtfCodepoint function"
  }
  let from_string = case string_selector {
    "selected" -> from_int
    _ -> function
  }
  let from_float = case float_selector {
    1.0 -> from_string
    _ -> function
  }
  case bool_selector {
    True -> from_float
    False -> function
  }
}

fn exercise_custom_function(
  function: fn(input) -> ValueBox(Nil),
  bool_selector: Bool,
  int_selector: Int,
  string_selector: String,
  float_selector: Float,
) -> fn(input) -> ValueBox(Nil) {
  let local = function
  let from_list = case [local] {
    [first] -> first
    _ -> function
  }
  let from_block = {
    let _ = Nil
    from_list
  }
  let from_int = case int_selector {
    0 -> custom_function_constant
    1 -> custom_function
    2 -> fn(value) { function(value) }
    3 -> local
    4 -> exercise_custom_function(function, False, 0, "", 0.0)
    5 -> {
      let provider = fn() { function }
      provider()
    }
    6 -> #(function).0
    7 -> CallableBox(function: function).function
    8 -> from_block
    _ -> panic as "unselected symbolic custom function"
  }
  let from_string = case string_selector {
    "selected" -> from_int
    _ -> function
  }
  let from_float = case float_selector {
    1.0 -> from_string
    _ -> function
  }
  case bool_selector {
    True -> from_float
    False -> function
  }
}

fn exercise_bool_function(
  function: fn(input) -> Bool,
  bool_selector: Bool,
  int_selector: Int,
  string_selector: String,
  float_selector: Float,
) -> fn(input) -> Bool {
  let local = function
  let from_list = case [local] {
    [first] -> first
    _ -> function
  }
  let from_block = {
    let _ = Nil
    from_list
  }
  let from_int = case int_selector {
    0 -> bool_function_constant
    1 -> bool_function
    2 -> fn(value) { function(value) }
    3 -> local
    4 -> exercise_bool_function(function, False, 0, "", 0.0)
    5 -> {
      let provider = fn() { function }
      provider()
    }
    6 -> #(function).0
    7 -> CallableBox(function: function).function
    8 -> from_block
    _ -> panic as "unselected symbolic Bool function"
  }
  let from_string = case string_selector {
    "selected" -> from_int
    _ -> function
  }
  let from_float = case float_selector {
    1.0 -> from_string
    _ -> function
  }
  case bool_selector {
    True -> from_float
    False -> function
  }
}

fn exercise_nil_function(
  function: fn(input) -> Nil,
  bool_selector: Bool,
  int_selector: Int,
  string_selector: String,
  float_selector: Float,
) -> fn(input) -> Nil {
  let local = function
  let from_list = case [local] {
    [first] -> first
    _ -> function
  }
  let from_block = {
    let _ = Nil
    from_list
  }
  let from_int = case int_selector {
    0 -> nil_function_constant
    1 -> nil_function
    2 -> fn(value) { function(value) }
    3 -> local
    4 -> exercise_nil_function(function, False, 0, "", 0.0)
    5 -> {
      let provider = fn() { function }
      provider()
    }
    6 -> #(function).0
    7 -> CallableBox(function: function).function
    8 -> from_block
    _ -> panic as "unselected symbolic Nil function"
  }
  let from_string = case string_selector {
    "selected" -> from_int
    _ -> function
  }
  let from_float = case float_selector {
    1.0 -> from_string
    _ -> function
  }
  case bool_selector {
    True -> from_float
    False -> function
  }
}

fn exercise_tuple_function(
  function: fn(input) -> #(Int, String),
  bool_selector: Bool,
  int_selector: Int,
  string_selector: String,
  float_selector: Float,
) -> fn(input) -> #(Int, String) {
  let local = function
  let from_list = case [local] {
    [first] -> first
    _ -> function
  }
  let from_block = {
    let _ = Nil
    from_list
  }
  let from_int = case int_selector {
    0 -> tuple_function_constant
    1 -> tuple_function
    2 -> fn(value) { function(value) }
    3 -> local
    4 -> exercise_tuple_function(function, False, 0, "", 0.0)
    5 -> {
      let provider = fn() { function }
      provider()
    }
    6 -> #(function).0
    7 -> CallableBox(function: function).function
    8 -> from_block
    _ -> panic as "unselected symbolic tuple function"
  }
  let from_string = case string_selector {
    "selected" -> from_int
    _ -> function
  }
  let from_float = case float_selector {
    1.0 -> from_string
    _ -> function
  }
  case bool_selector {
    True -> from_float
    False -> function
  }
}

fn exercise_list_function(
  function: fn(input) -> List(Int),
  bool_selector: Bool,
  int_selector: Int,
  string_selector: String,
  float_selector: Float,
) -> fn(input) -> List(Int) {
  let local = function
  let from_list = case [local] {
    [first] -> first
    _ -> function
  }
  let from_block = {
    let _ = Nil
    from_list
  }
  let from_int = case int_selector {
    0 -> list_function_constant
    1 -> list_function
    2 -> fn(value) { function(value) }
    3 -> local
    4 -> exercise_list_function(function, False, 0, "", 0.0)
    5 -> {
      let provider = fn() { function }
      provider()
    }
    6 -> #(function).0
    7 -> CallableBox(function: function).function
    8 -> from_block
    _ -> panic as "unselected symbolic list function"
  }
  let from_string = case string_selector {
    "selected" -> from_int
    _ -> function
  }
  let from_float = case float_selector {
    1.0 -> from_string
    _ -> function
  }
  case bool_selector {
    True -> from_float
    False -> function
  }
}

fn exercise_function_function(
  function: fn(input) -> fn(Int) -> Int,
  bool_selector: Bool,
  int_selector: Int,
  string_selector: String,
  float_selector: Float,
) -> fn(input) -> fn(Int) -> Int {
  let local = function
  let from_list = case [local] {
    [first] -> first
    _ -> function
  }
  let from_block = {
    let _ = Nil
    from_list
  }
  let from_int = case int_selector {
    0 -> function_function_constant
    1 -> function_function
    2 -> fn(value) { function(value) }
    3 -> local
    4 -> exercise_function_function(function, False, 0, "", 0.0)
    5 -> {
      let provider = fn() { function }
      provider()
    }
    6 -> #(function).0
    7 -> CallableBox(function: function).function
    8 -> from_block
    _ -> panic as "unselected symbolic function-returning function"
  }
  let from_string = case string_selector {
    "selected" -> from_int
    _ -> function
  }
  let from_float = case float_selector {
    1.0 -> from_string
    _ -> function
  }
  case bool_selector {
    True -> from_float
    False -> function
  }
}

fn transform_value(value: value, fallback: value, mapper: fn(value) -> value) {
  let local = value
  let direct = identity(local)
  let called = mapper(direct)
  let tuple = #(called)
  let from_tuple = tuple.0
  let boxed = ValueBox(value: from_tuple)
  let from_field = boxed.value
  let from_list = case [from_field] {
    [first] -> first
    _ -> fallback
  }
  let from_int = case 1 {
    1 -> from_list
    _ -> fallback
  }
  let from_string = case "selected" {
    "selected" -> from_int
    _ -> fallback
  }
  let from_float = case 1.0 {
    1.0 -> from_string
    _ -> fallback
  }
  let from_block = {
    let _ = Nil
    from_float
  }
  case True {
    True -> from_block
    False -> panic as "unselected symbolic value"
  }
}

fn forward_function(function: fn(input) -> output) {
  function
}

fn provide_function(function: fn(input) -> output) {
  fn() { function }
}

fn transform_function(function: fn(input) -> output) {
  let function = transform_value(function, function, identity)
  let closure = fn(value) { function(value) }
  let local = case True {
    True -> forward_function(closure)
    False -> function
  }
  let provider = provide_function(local)
  let called = provider()
  let tuple = #(called)
  let from_tuple = tuple.0
  let boxed = CallableBox(function: from_tuple)
  let from_field = boxed.function
  let from_list = case [from_field] {
    [first] -> first
    _ -> function
  }
  let from_int = case 1 {
    1 -> from_list
    _ -> function
  }
  let from_string = case "selected" {
    "selected" -> from_int
    _ -> function
  }
  let from_float = case 1.0 {
    1.0 -> from_string
    _ -> function
  }
  let from_block = {
    let _ = Nil
    from_float
  }
  case True {
    True -> from_block
    False -> panic as "unselected symbolic function"
  }
}

fn select_function(
  function: fn(input) -> output,
  bool_selector: Bool,
  int_selector: Int,
  string_selector: String,
  float_selector: Float,
) {
  let from_bool = case bool_selector {
    True -> function
    False -> function
  }
  let from_int = case int_selector {
    1 -> from_bool
    _ -> function
  }
  let from_string = case string_selector {
    "selected" -> from_int
    _ -> function
  }
  case float_selector {
    1.0 -> from_string
    _ -> function
  }
}

fn forward_list(values: List(value)) {
  values
}

fn provide_list(values: List(value)) {
  fn() { values }
}

fn forward_nested_list(values: List(List(value))) {
  values
}

fn provide_nested_list(values: List(List(value))) {
  fn() { values }
}

fn transform_list(values: List(value)) {
  let values = transform_value(values, values, identity)
  let direct = forward_list(values)
  let provider = provide_list(direct)
  let called = provider()
  let tuple = #(called)
  let from_tuple = tuple.0
  let boxed = ListBox(values: from_tuple)
  let from_field = boxed.values
  let from_nested = case [from_field] {
    [first] -> first
    _ -> values
  }
  let from_int = case 1 {
    1 -> from_nested
    _ -> values
  }
  let from_string = case "selected" {
    "selected" -> from_int
    _ -> values
  }
  let from_float = case 1.0 {
    1.0 -> from_string
    _ -> values
  }
  let from_block = {
    let _ = Nil
    from_float
  }
  case True {
    True -> from_block
    False -> panic as "unselected symbolic list"
  }
}

fn select_list(
  values: List(value),
  bool_selector: Bool,
  int_selector: Int,
  string_selector: String,
  float_selector: Float,
) {
  let from_bool = case bool_selector {
    True -> values
    False -> values
  }
  let from_int = case int_selector {
    1 -> from_bool
    _ -> values
  }
  let from_string = case string_selector {
    "selected" -> from_int
    _ -> values
  }
  case float_selector {
    1.0 -> from_string
    _ -> values
  }
}

fn select_list_expression(values: List(value), selector: Float) {
  #(case selector {
    1.0 -> values
    _ -> values
  }).0
}

fn bind_empty_tail(values: List(value)) {
  let [..tail] = values
  tail
}

fn transform_nested_list(values: List(List(value))) {
  let local = values
  let direct = forward_nested_list(local)
  let provider = provide_nested_list(direct)
  let called = provider()
  let tuple = #(called)
  let from_tuple = tuple.0
  let boxed = NestedListBox(values: from_tuple)
  let from_field = boxed.values
  let from_outer = case [from_field] {
    [first] -> first
    _ -> values
  }
  let prefixed = [[], ..from_outer]
  let from_tail = case prefixed {
    [_, ..tail] -> tail
    _ -> values
  }
  let from_int = case 1 {
    1 -> from_tail
    _ -> values
  }
  let from_string = case "selected" {
    "selected" -> from_int
    _ -> values
  }
  let from_float = case 1.0 {
    1.0 -> from_string
    _ -> values
  }
  let from_block = {
    let _ = Nil
    from_float
  }
  case True {
    True -> from_block
    False -> panic as "unselected symbolic nested list"
  }
}

const function_constant = identity
const list_constant = []

pub fn main() {
  let transformed_function = transform_function(identity)
  let transformed_list = transform_list(list_constant)
  let increment = transform_function(fn(value) { value + 1 })
  let transformed_int_function = transform_function(int_function)
  let transformed_float_function = transform_function(float_function)
  let transformed_string_function = transform_function(string_function)
  let transformed_bit_array_function = transform_function(bit_array_function)
  let transformed_utf_codepoint_function = transform_function(utf_codepoint_function)
  let transformed_custom_function = transform_function(custom_function)
  let transformed_bool_function = transform_function(bool_function)
  let transformed_nil_function = transform_function(nil_function)
  let transformed_tuple_function = transform_function(tuple_function)
  let transformed_list_function = transform_function(list_function)
  let transformed_function_function = transform_function(function_function)
  let int_constant_path =
    exercise_int_function(int_function, True, 0, "selected", 1.0)
  let int_reference_path =
    exercise_int_function(int_function, True, 1, "selected", 1.0)
  let int_closure_path =
    exercise_int_function(int_function, True, 2, "selected", 1.0)
  let int_local_path =
    exercise_int_function(int_function, True, 3, "selected", 1.0)
  let int_call_path =
    exercise_int_function(int_function, True, 4, "selected", 1.0)
  let int_function_call_path =
    exercise_int_function(int_function, True, 5, "selected", 1.0)
  let int_tuple_path =
    exercise_int_function(int_function, True, 6, "selected", 1.0)
  let int_custom_path =
    exercise_int_function(int_function, True, 7, "selected", 1.0)
  let int_list_path =
    exercise_int_function(int_function, True, 8, "selected", 1.0)
  let int_string_fallback =
    exercise_int_function(int_function, True, 0, "fallback", 1.0)
  let int_float_fallback =
    exercise_int_function(int_function, True, 0, "selected", 0.0)
  let int_bool_fallback =
    exercise_int_function(int_function, False, 0, "selected", 1.0)
  let float_symbolic =
    exercise_float_function(float_function, True, 0, "selected", 1.0)
  let string_symbolic =
    exercise_string_function(string_function, True, 0, "selected", 1.0)
  let bit_array_symbolic =
    exercise_bit_array_function(bit_array_function, True, 0, "selected", 1.0)
  let utf_codepoint_symbolic =
    exercise_utf_codepoint_function(utf_codepoint_function, True, 0, "selected", 1.0)
  let custom_symbolic =
    exercise_custom_function(custom_function, True, 0, "selected", 1.0)
  let bool_symbolic =
    exercise_bool_function(bool_function, True, 0, "selected", 1.0)
  let nil_symbolic =
    exercise_nil_function(nil_function, True, 0, "selected", 1.0)
  let tuple_symbolic =
    exercise_tuple_function(tuple_function, True, 0, "selected", 1.0)
  let list_symbolic =
    exercise_list_function(list_function, True, 0, "selected", 1.0)
  let function_symbolic =
    exercise_function_function(function_function, True, 0, "selected", 1.0)
  #(
    function_constant == identity,
    transformed_function == transformed_function,
    transformed_list == list_constant,
    increment(1) == 2,
    transform_list([1]) == [1],
    transform_value([[1]], [[1]], identity) == [[1]],
    transform_value([[]], [[]], identity) == [[]],
    transform_nested_list([[]]) == [[]],
    select_function(identity, True, 1, "selected", 1.0) == identity,
    select_function(identity, False, 0, "fallback", 0.0) == identity,
    select_list(list_constant, True, 1, "selected", 1.0) == [],
    select_list(list_constant, False, 0, "fallback", 0.0) == [],
    select_list_expression(list_constant, 1.0) == [],
    select_list_expression(list_constant, 0.0) == [],
    bind_empty_tail(list_constant) == [],
    transformed_int_function == transformed_int_function,
    transformed_float_function == transformed_float_function,
    transformed_string_function == transformed_string_function,
    transformed_bit_array_function == transformed_bit_array_function,
    transformed_utf_codepoint_function == transformed_utf_codepoint_function,
    transformed_custom_function == transformed_custom_function,
    transformed_bool_function == transformed_bool_function,
    transformed_nil_function == transformed_nil_function,
    transformed_tuple_function == transformed_tuple_function,
    transformed_list_function == transformed_list_function,
    transformed_function_function == transformed_function_function,
    int_constant_path == int_constant_path,
    int_reference_path == int_reference_path,
    int_closure_path == int_closure_path,
    int_local_path == int_local_path,
    int_call_path == int_call_path,
    int_function_call_path == int_function_call_path,
    int_tuple_path == int_tuple_path,
    int_custom_path == int_custom_path,
    int_list_path == int_list_path,
    int_string_fallback == int_string_fallback,
    int_float_fallback == int_float_fallback,
    int_bool_fallback == int_bool_fallback,
    float_symbolic == float_symbolic,
    string_symbolic == string_symbolic,
    bit_array_symbolic == bit_array_symbolic,
    utf_codepoint_symbolic == utf_codepoint_symbolic,
    custom_symbolic == custom_symbolic,
    bool_symbolic == bool_symbolic,
    nil_symbolic == nil_symbolic,
    tuple_symbolic == tuple_symbolic,
    list_symbolic == list_symbolic,
    function_symbolic == function_symbolic,
  )
}

// geam:expect Tuple([Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true)])
