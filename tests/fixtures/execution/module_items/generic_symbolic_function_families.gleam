pub type Phantom(value) {
  Phantom
}

pub type Boxed(value) {
  Boxed(value)
}

fn identity(value) { value }
fn int_function(_value) { 1 }
fn float_function(_value) { 1.5 }
fn string_function(_value) { "one" }
fn bit_array_function(_value) { <<1>> }
fn utf_codepoint_function(_value) -> UtfCodepoint {
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  codepoint
}
fn custom_function(_value) { Phantom }
fn bool_function(_value) { True }
fn nil_function(_value) { Nil }
fn tuple_function(_value) { #(1, "one") }
fn list_function(_value) { [] }
fn function_function(_value) { identity }

const int_constant = int_function
const float_constant = float_function
const string_constant = string_function
const bit_array_constant = bit_array_function
const utf_codepoint_constant = utf_codepoint_function
const custom_constant = custom_function
const bool_constant = bool_function
const nil_constant = nil_function
const tuple_constant = tuple_function
const list_constant = list_function
const function_constant = function_function
const constructor_constant = Boxed

pub fn main() {
  #(
    int_constant,
    float_constant,
    string_constant,
    bit_array_constant,
    utf_codepoint_constant,
    custom_constant,
    bool_constant,
    nil_constant,
    tuple_constant,
    list_constant,
    function_constant,
    constructor_constant,
  )
}

// geam:expect Tuple([Function(fn(Parameter(0)) -> Int), Function(fn(Parameter(1)) -> Float), Function(fn(Parameter(2)) -> String), Function(fn(Parameter(3)) -> BitArray), Function(fn(Parameter(4)) -> UtfCodepoint), Function(fn(Parameter(5)) -> geam/main/Phantom(Parameter(6))), Function(fn(Parameter(7)) -> Bool), Function(fn(Parameter(8)) -> Nil), Function(fn(Parameter(9)) -> #(Int, String)), Function(fn(Parameter(10)) -> List(Parameter(11))), Function(fn(Parameter(12)) -> fn(Parameter(13)) -> Parameter(13)), Function(fn(Parameter(14)) -> geam/main/Boxed(Parameter(14)))])
