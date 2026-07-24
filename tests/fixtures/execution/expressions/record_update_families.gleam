pub type Inner {
  Inner(Int)
}

pub type Everything {
  Everything(
    int: Int,
    float: Float,
    string: String,
    bits: BitArray,
    codepoint: UtfCodepoint,
    custom: Inner,
    bool: Bool,
    nil: Nil,
    tuple: #(Int),
    list: List(Int),
    int_function: fn(Int) -> Int,
    float_function: fn() -> Float,
    string_function: fn() -> String,
    bit_array_function: fn() -> BitArray,
    utf_codepoint_function: fn() -> UtfCodepoint,
    custom_function: fn() -> Inner,
    bool_function: fn() -> Bool,
    nil_function: fn() -> Nil,
    tuple_function: fn() -> #(Int),
    list_function: fn() -> List(Int),
    function_function: fn() -> fn(Int) -> Int,
  )
}

fn add_one(value: Int) { value + 1 }
fn add_two(value: Int) { value + 2 }
fn float_value() { 1.5 }
fn string_value() { "one" }
fn bit_array_value() { <<1>> }
fn custom_value() { Inner(2) }
fn bool_value() { True }
fn nil_value() { Nil }
fn tuple_value() { #(3) }
fn list_value() { [4] }
fn function_value() { add_one }

fn update_everything(codepoint: UtfCodepoint) {
  let value = Everything(
    int: 1,
    float: 1.5,
    string: "one",
    bits: <<1>>,
    codepoint: codepoint,
    custom: Inner(2),
    bool: True,
    nil: Nil,
    tuple: #(3),
    list: [4],
    int_function: add_one,
    float_function: float_value,
    string_function: string_value,
    bit_array_function: bit_array_value,
    utf_codepoint_function: fn() { codepoint },
    custom_function: custom_value,
    bool_function: bool_value,
    nil_function: nil_value,
    tuple_function: tuple_value,
    list_function: list_value,
    function_function: function_value,
  )
  let value = Everything(..value, int: value.int)
  Everything(
    ..value,
    int: 2,
    float: 2.5,
    string: "two",
    bits: <<2>>,
    codepoint: codepoint,
    custom: Inner(5),
    bool: False,
    nil: Nil,
    tuple: #(6),
    list: [7],
    int_function: add_two,
    float_function: fn() { 2.5 },
    string_function: fn() { "two" },
    bit_array_function: fn() { <<2>> },
    utf_codepoint_function: fn() { codepoint },
    custom_function: fn() { Inner(5) },
    bool_function: fn() { False },
    nil_function: fn() { Nil },
    tuple_function: fn() { #(6) },
    list_function: fn() { [7] },
    function_function: fn() { add_two },
  )
}

pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<"A":utf8>>
  let value = update_everything(codepoint)
  #(
    value.int,
    value.float,
    value.string,
    value.bits,
    value.codepoint,
    value.custom,
    value.bool,
    value.nil,
    value.tuple,
    value.list,
    value.int_function(5),
    value.float_function(),
    value.string_function(),
    value.bit_array_function(),
    value.utf_codepoint_function(),
    value.custom_function(),
    value.bool_function(),
    value.nil_function(),
    value.tuple_function(),
    value.list_function(),
    value.function_function()(6),
  )
}

// @geam:expect Tuple([Int(2), Float(2.5), String("two"), BitArray(bytes=[2], bit_len=8), UtfCodepoint('A'), Custom(type=geam/main/Inner, constructor=Inner#0, fields=[Int(5)]), Bool(false), Nil, Tuple([Int(6)]), List(Int)([Int(7)]), Int(7), Float(2.5), String("two"), BitArray(bytes=[2], bit_len=8), UtfCodepoint('A'), Custom(type=geam/main/Inner, constructor=Inner#0, fields=[Int(5)]), Bool(false), Nil, Tuple([Int(6)]), List(Int)([Int(7)]), Int(8)])
