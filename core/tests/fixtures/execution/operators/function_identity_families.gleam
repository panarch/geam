pub type Boxed {
  Boxed
}

fn int_value() -> Int {
  1
}

fn int_closure() -> fn() -> Int {
  fn() { 1 }
}

fn float_value() -> Float {
  1.0
}

fn float_closure() -> fn() -> Float {
  fn() { 1.0 }
}

fn string_value() -> String {
  "one"
}

fn string_closure() -> fn() -> String {
  fn() { "one" }
}

fn bit_array_value() -> BitArray {
  <<>>
}

fn bit_array_closure() -> fn() -> BitArray {
  fn() { <<>> }
}

fn utf_codepoint_value(value: UtfCodepoint) -> UtfCodepoint {
  value
}

fn utf_codepoint_closure(value: UtfCodepoint) -> fn() -> UtfCodepoint {
  fn() { value }
}

fn custom_value() -> Boxed {
  Boxed
}

fn custom_closure() -> fn() -> Boxed {
  fn() { Boxed }
}

fn bool_value() -> Bool {
  True
}

fn bool_closure() -> fn() -> Bool {
  fn() { True }
}

fn nil_value() -> Nil {
  Nil
}

fn nil_closure() -> fn() -> Nil {
  fn() { Nil }
}

fn tuple_value() -> #(Int) {
  #(1)
}

fn tuple_closure() -> fn() -> #(Int) {
  fn() { #(1) }
}

fn list_value() -> List(Int) {
  [1]
}

fn list_closure() -> fn() -> List(Int) {
  fn() { [1] }
}

fn function_value() -> fn() -> Int {
  int_value
}

fn function_closure() -> fn() -> fn() -> Int {
  fn() { int_value }
}

pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<"a":utf8>>

  #(
    int_value == int_value,
    int_closure() == int_closure(),
    float_value == float_value,
    float_closure() == float_closure(),
    string_value == string_value,
    string_closure() == string_closure(),
    bit_array_value == bit_array_value,
    bit_array_closure() == bit_array_closure(),
    utf_codepoint_value == utf_codepoint_value,
    utf_codepoint_closure(codepoint) == utf_codepoint_closure(codepoint),
    custom_value == custom_value,
    custom_closure() == custom_closure(),
    bool_value == bool_value,
    bool_closure() == bool_closure(),
    nil_value == nil_value,
    nil_closure() == nil_closure(),
    tuple_value == tuple_value,
    tuple_closure() == tuple_closure(),
    list_value == list_value,
    list_closure() == list_closure(),
    function_value == function_value,
    function_closure() == function_closure(),
  )
}

// @geam:expect Tuple([Bool(true), Bool(false), Bool(true), Bool(false), Bool(true), Bool(false), Bool(true), Bool(false), Bool(true), Bool(false), Bool(true), Bool(false), Bool(true), Bool(false), Bool(true), Bool(false), Bool(true), Bool(false), Bool(true), Bool(false), Bool(true), Bool(false)])
