pub type Inner {
  Inner(Int)
}

pub type Everything {
  Everything(
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Inner,
    Bool,
    Nil,
    #(Int),
    List(Int),
    fn(Int) -> Int,
  )
}

pub type PatternFields {
  PatternFields(String, String, BitArray, BitArray, List(Int), List(Int), Bool)
}

pub type TotalFields {
  TotalFields(Int, BitArray, BitArray, List(Int), List(Int), #(Int), Inner)
}

pub type ConstructorFamilies {
  ConstructorFamilies(
    List(Int),
    List(String),
    List(BitArray),
    List(UtfCodepoint),
    List(Inner),
    List(Float),
    List(Bool),
    List(Nil),
    List(#(Int)),
    List(List(Int)),
    List(fn(Int) -> Int),
    fn(Int) -> Int,
    fn() -> Float,
    fn() -> String,
    fn() -> BitArray,
    fn(UtfCodepoint) -> UtfCodepoint,
    fn() -> Inner,
    fn() -> Bool,
    fn() -> Nil,
    fn() -> #(Int),
    fn() -> List(Int),
    fn() -> fn(Int) -> Int,
  )
}

fn add_one(value: Int) {
  value + 1
}

fn float_value() { 1.0 }
fn string_value() { "one" }
fn bit_array_value() { <<1>> }
fn codepoint_value(value: UtfCodepoint) { value }
fn custom_value() { Inner(1) }
fn bool_value() { True }
fn nil_value() { Nil }
fn tuple_value() { #(1) }
fn list_value() { [1] }
fn function_value() { add_one }

pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  let constructor_families = ConstructorFamilies
  let ConstructorFamilies(
    int_list,
    string_list,
    bit_array_list,
    codepoint_list,
    custom_list,
    float_list,
    bool_list,
    nil_list,
    tuple_list,
    list_list,
    function_list,
    int_function,
    float_function,
    string_function,
    bit_array_function,
    codepoint_function,
    custom_function,
    bool_function,
    nil_function,
    tuple_function,
    list_function,
    function_function,
  ) = constructor_families(
    [1],
    ["one"],
    [<<1>>],
    [codepoint],
    [Inner(1)],
    [1.0],
    [True],
    [Nil],
    [#(1)],
    [[1]],
    [add_one],
    add_one,
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
  assert int_list == [1]
  assert string_list == ["one"]
  assert bit_array_list == [<<1>>]
  assert codepoint_list == [codepoint]
  assert custom_list == [Inner(1)]
  assert float_list == [1.0]
  assert bool_list == [True]
  assert nil_list == [Nil]
  assert tuple_list == [#(1)]
  assert list_list == [[1]]
  assert case function_list { [function] -> function(1) == 2 _ -> False }
  assert int_function(1) == 2
  assert float_function() == 1.0
  assert string_function() == "one"
  assert bit_array_function() == <<1>>
  assert codepoint_function(codepoint) == codepoint
  assert custom_function() == Inner(1)
  assert bool_function()
  nil_function()
  assert tuple_function() == #(1)
  assert list_function() == [1]
  assert function_function()(1) == 2
  let constructor = Everything
  let value =
    constructor(
      1,
      1.5,
      "prefix!",
      <<1>>,
      codepoint,
      Inner(2),
      True,
      Nil,
      #(3),
      [4],
      add_one,
    )

  let matched = case value {
    Everything(
      1,
      1.5,
      "prefix" <> suffix,
      <<bits:bits>>,
      scalar,
      Inner(2),
      True,
      Nil,
      #(3),
      [4],
      function,
    ) if suffix == "!" && bits == <<1>> && scalar == codepoint -> function(4)
    _ -> 0
  }

  let Everything(
    int,
    float,
    string,
    bit_array,
    scalar,
    Inner(inner),
    bool,
    nil,
    #(tuple),
    list,
    function,
  ) = value

  let exact = case PatternFields("exact", "prefix!", <<2>>, <<3>>, [6], [7], False) {
    PatternFields(
      "exact",
      "prefix" <> suffix,
      <<_:bits>>,
      <<_ as alias_bits:bits>>,
      [..items],
      [..],
      False,
    ) if suffix == "!" && alias_bits == <<3>> && items == [6] -> 1
    _ -> 0
  }

  let TotalFields(
    total_int as int_alias,
    <<all_bits:bits>>,
    <<_ as bit_alias:bits>>,
    [..all_items],
    [..],
    #(total_tuple),
    Inner(total_inner),
  ) = TotalFields(8, <<9>>, <<10>>, [11], [12], #(13), Inner(14))
  assert total_int == 8
  assert int_alias == 8
  assert all_bits == <<9>>
  assert bit_alias == <<10>>
  assert all_items == [11]
  assert total_tuple == 13
  assert total_inner == 14

  assert float == 1.5
  assert string == "prefix!"
  assert bit_array == <<1>>
  assert scalar == codepoint
  assert bool
  nil
  case list {
    [item] -> matched + int + inner + tuple + item + function(5) + exact
    _ -> 0
  }
}

// geam:expect Int(22)
