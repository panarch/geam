pub type Boxed {
  Boxed
}

fn generic_value(value) {
  echo value
}

fn int_value() -> Int {
  1
}

fn float_value() -> Float {
  1.0
}

fn string_value() -> String {
  "one"
}

fn bit_array_value() -> BitArray {
  <<1, 2>>
}

fn utf_codepoint_value(value: UtfCodepoint) -> UtfCodepoint {
  value
}

fn custom_value() -> Boxed {
  Boxed
}

fn bool_value() -> Bool {
  True
}

fn nil_value() -> Nil {
  Nil
}

fn tuple_value() -> #(Int, String) {
  #(1, "one")
}

fn list_value() -> List(Int) {
  [72, 105]
}

fn function_value() -> fn() -> Int {
  int_value
}

pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<"a":utf8>>

  echo generic_value(1)
  echo 1
  echo 1.0
  echo "one"
  echo <<1, 2>>
  echo codepoint
  echo Boxed
  echo True
  echo Nil
  echo #(1, "one")
  echo [72, 105]

  echo generic_value
  echo int_value
  echo float_value
  echo string_value
  echo bit_array_value
  echo utf_codepoint_value
  echo custom_value
  echo bool_value
  echo nil_value
  echo tuple_value
  echo list_value
  echo function_value

  1
}

// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:6
// 1
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:56
// 1
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:57
// 1
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:58
// 1.0
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:59
// "one"
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:60
// <<1, 2>>
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:61
// 'a'
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:62
// Boxed
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:63
// True
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:64
// Nil
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:65
// #(1, "one")
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:66
// charlist.from_string("Hi")
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:68
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:69
// //fn() { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:70
// //fn() { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:71
// //fn() { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:72
// //fn() { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:73
// //fn(a) { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:74
// //fn() { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:75
// //fn() { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:76
// //fn() { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:77
// //fn() { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:78
// //fn() { ... }
// @geam:echo
// tests/fixtures/execution/expressions/echo_families.gleam:79
// //fn() { ... }
// @geam:expect Int(1)
