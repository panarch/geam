pub type Token {
  Token
}

fn identity(value: value) {
  value
}

fn apply(function: fn(value) -> value, value: value) {
  function(value)
}

fn int_function(value: Int) -> Int {
  value
}

fn float_function(_value: Int) -> Float {
  1.5
}

fn string_function(_value: Int) -> String {
  "value"
}

fn bit_array_function(_value: Int) -> BitArray {
  <<1>>
}

fn utf_codepoint_function(_value: Int) -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}

fn custom_function(_value: Int) -> Token {
  Token
}

fn bool_function(_value: Int) -> Bool {
  True
}

fn nil_function(_value: Int) -> Nil {
  Nil
}

fn tuple_function(_value: Int) -> #(Int) {
  #(1)
}

fn list_function(_value: Int) -> List(Int) {
  [1]
}

fn function_function(_value: Int) -> fn(Int) -> Int {
  int_function
}

fn never_function(_value: Int) -> value {
  panic as "never function must not run"
}

pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<65>>

  #(
    apply(identity, 1) == 1,
    apply(identity, 1.5) == 1.5,
    apply(identity, "value") == "value",
    apply(identity, <<1>>) == <<1>>,
    apply(identity, codepoint) == codepoint,
    apply(identity, Token) == Token,
    apply(identity, True) == True,
    apply(identity, Nil) == Nil,
    apply(identity, #(1)) == #(1),
    apply(identity, [1]) == [1],
    apply(identity, ["value"]) == ["value"],
    apply(identity, [<<1>>]) == [<<1>>],
    apply(identity, [codepoint]) == [codepoint],
    apply(identity, [Token]) == [Token],
    apply(identity, [1.5]) == [1.5],
    apply(identity, [True]) == [True],
    apply(identity, [Nil]) == [Nil],
    apply(identity, [#(1)]) == [#(1)],
    apply(identity, [[1]]) == [[1]],
    apply(identity, [int_function]) == [int_function],
    apply(identity, int_function) == int_function,
    apply(identity, float_function) == float_function,
    apply(identity, string_function) == string_function,
    apply(identity, bit_array_function) == bit_array_function,
    apply(identity, utf_codepoint_function) == utf_codepoint_function,
    apply(identity, custom_function) == custom_function,
    apply(identity, bool_function) == bool_function,
    apply(identity, nil_function) == nil_function,
    apply(identity, tuple_function) == tuple_function,
    apply(identity, list_function) == list_function,
    apply(identity, function_function) == function_function,
    apply(identity, identity) == identity,
    apply(identity, never_function) == never_function,
  )
}

// @geam:expect Tuple([Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true)])
