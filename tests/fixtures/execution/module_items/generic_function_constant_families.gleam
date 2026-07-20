pub type Token {
  Token(Int)
}

fn identity(value: value) {
  value
}

fn int_value() { 1 }
fn float_value() { 1.5 }
fn string_value() { "one" }
fn bit_array_value() { <<1>> }
fn codepoint_value() {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}
fn custom_value() { Token(1) }
fn bool_value() { True }
fn nil_value() { Nil }
fn tuple_value() { #(1, "one") }
fn list_value() { [1] }
fn function_value() { int_value }

const generic_function = identity
const generic_function_alias = generic_function
const generic_specialized_function = generic_function_alias
const generic_int_specialization: fn(Int) -> Int = generic_specialized_function
const generic_float_specialization: fn(Float) -> Float = generic_specialized_function
const generic_string_specialization: fn(String) -> String = generic_specialized_function
const generic_bit_array_specialization: fn(BitArray) -> BitArray = generic_specialized_function
const generic_utf_codepoint_specialization: fn(UtfCodepoint) -> UtfCodepoint = generic_specialized_function
const generic_custom_specialization: fn(Token) -> Token = generic_specialized_function
const generic_bool_specialization: fn(Bool) -> Bool = generic_specialized_function
const generic_nil_specialization: fn(Nil) -> Nil = generic_specialized_function
const generic_tuple_specialization: fn(#(Int, String)) -> #(Int, String) = generic_specialized_function
const generic_list_specialization: fn(List(Int)) -> List(Int) = generic_specialized_function
const generic_function_specialization: fn(fn() -> Int) -> fn() -> Int = generic_specialized_function
const int_function = int_value
const int_function_alias = int_function
const float_function = float_value
const float_function_alias = float_function
const string_function = string_value
const string_function_alias = string_function
const bit_array_function = bit_array_value
const bit_array_function_alias = bit_array_function
const utf_codepoint_function = codepoint_value
const utf_codepoint_function_alias = utf_codepoint_function
const custom_function = custom_value
const custom_function_alias = custom_function
const bool_function = bool_value
const bool_function_alias = bool_function
const nil_function = nil_value
const nil_function_alias = nil_function
const tuple_function = tuple_value
const tuple_function_alias = tuple_function
const list_function = list_value
const list_function_alias = list_function
const function_function = function_value
const function_function_alias = function_function
const constructor_function = Token
const constructor_function_alias = constructor_function
const direct_custom_callables = #(custom_value, Token)
const direct_function_values = #(
  identity,
  int_value,
  float_value,
  string_value,
  bit_array_value,
  codepoint_value,
  custom_value,
  bool_value,
  nil_value,
  tuple_value,
  list_value,
  function_value,
)
const all_function_values = #(
  generic_function,
  int_function,
  float_function,
  string_function,
  bit_array_function,
  utf_codepoint_function,
  custom_function,
  bool_function,
  nil_function,
  tuple_function,
  list_function,
  function_function,
  constructor_function,
)

fn forwarded_function() -> fn(value) -> value {
  generic_function_alias
}

fn specialized_identity(_sample: value) -> fn(value) -> value {
  generic_function_alias
}

fn custom_identity() -> fn(Token) -> Token {
  generic_function
}

fn list_identity() -> fn(List(Int)) -> List(Int) {
  generic_function
}

fn function_identity() -> fn(fn() -> Int) -> fn() -> Int {
  generic_function
}

pub fn main() {
  #(
    generic_function_alias(1),
    generic_function_alias("one"),
    all_function_values == all_function_values,
    forwarded_function()(2),
    forwarded_function()(2.5),
    forwarded_function()("two"),
    forwarded_function()(<<2>>),
    forwarded_function()(codepoint_value()) == codepoint_value(),
    forwarded_function()(Token(2)),
    forwarded_function()(False),
    forwarded_function()(Nil),
    forwarded_function()(#(2, "two")),
    forwarded_function()([2]),
    forwarded_function()(int_value)(),
    generic_int_specialization(3),
    generic_float_specialization(3.5),
    generic_string_specialization("three"),
    generic_bit_array_specialization(<<3>>),
    generic_utf_codepoint_specialization(codepoint_value()) == codepoint_value(),
    generic_custom_specialization(Token(3)),
    generic_bool_specialization(True),
    generic_nil_specialization(Nil),
    generic_tuple_specialization(#(3, "three")),
    generic_list_specialization([3]),
    generic_function_specialization(int_value)(),
    int_function_alias(),
    float_function_alias(),
    string_function_alias(),
    bit_array_function_alias(),
    utf_codepoint_function_alias() == codepoint_value(),
    custom_function_alias(),
    bool_function_alias(),
    nil_function_alias(),
    tuple_function_alias(),
    list_function_alias(),
    function_function_alias()(),
    constructor_function_alias(2),
    direct_custom_callables == direct_custom_callables,
    custom_identity()(Token(4)),
    list_identity()([4]),
    function_identity()(int_value)(),
    specialized_identity(7)(8),
    specialized_identity(7.5)(8.5),
    specialized_identity("seven")("eight"),
    specialized_identity(<<7>>)(<<8>>),
    specialized_identity(codepoint_value())(codepoint_value())
      == codepoint_value(),
    specialized_identity(False)(True),
    specialized_identity(Nil)(Nil),
    specialized_identity(#(7, "seven"))(#(8, "eight")),
    specialized_identity(Token(4))(Token(5)),
    specialized_identity([5])([6]),
    specialized_identity(int_value)(int_value)(),
    direct_function_values == direct_function_values,
  )
}

// geam:expect Tuple([Int(1), String("one"), Bool(false), Int(2), Float(2.5), String("two"), BitArray(bytes=[2], bit_len=8), Bool(true), Custom(type=geam/main/Token, constructor=Token#0, fields=[Int(2)]), Bool(false), Nil, Tuple([Int(2), String("two")]), List(Int)([Int(2)]), Int(1), Int(3), Float(3.5), String("three"), BitArray(bytes=[3], bit_len=8), Bool(true), Custom(type=geam/main/Token, constructor=Token#0, fields=[Int(3)]), Bool(true), Nil, Tuple([Int(3), String("three")]), List(Int)([Int(3)]), Int(1), Int(1), Float(1.5), String("one"), BitArray(bytes=[1], bit_len=8), Bool(true), Custom(type=geam/main/Token, constructor=Token#0, fields=[Int(1)]), Bool(true), Nil, Tuple([Int(1), String("one")]), List(Int)([Int(1)]), Int(1), Custom(type=geam/main/Token, constructor=Token#0, fields=[Int(2)]), Bool(false), Custom(type=geam/main/Token, constructor=Token#0, fields=[Int(4)]), List(Int)([Int(4)]), Int(1), Int(8), Float(8.5), String("eight"), BitArray(bytes=[8], bit_len=8), Bool(true), Bool(true), Nil, Tuple([Int(8), String("eight")]), Custom(type=geam/main/Token, constructor=Token#0, fields=[Int(5)]), List(Int)([Int(6)]), Int(1), Bool(true)])
