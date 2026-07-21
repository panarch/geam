pub type Boxed(value) {
  Boxed(value)
}

fn identity(value) {
  value
}

fn boxed(value) {
  Boxed(value)
}

fn listed(value) {
  [value]
}

fn nested_function(value) {
  fn(_input) { value }
}

fn int_function(_input) {
  1
}

fn float_function(_input) {
  1.0
}

fn string_function(_input) {
  "one"
}

fn bit_array_function(_input) {
  <<1>>
}

fn utf_codepoint_function(_input) {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}

fn bool_function(_input) {
  True
}

fn nil_function(_input) {
  Nil
}

fn tuple_function(_input) {
  #(1)
}

fn string_list_function(_input) {
  ["one"]
}

fn bit_array_list_function(_input) {
  [<<1>>]
}

fn utf_codepoint_list_function(_input) {
  let assert <<value:utf8_codepoint>> = <<65>>
  [value]
}

fn float_list_function(_input) {
  [1.0]
}

fn bool_list_function(_input) {
  [True]
}

fn nil_list_function(_input) {
  [Nil]
}

fn retain_generic(function: fn(input) -> output, count: Int) {
  case count {
    0 -> function
    _ -> retain_generic(function, count - 1)
  }
}

fn retain_value(value, count: Int) {
  case count {
    0 -> value
    _ -> retain_value(value, count - 1)
  }
}

fn retain_custom(
  function: fn(input) -> Boxed(output),
  count: Int,
) -> fn(input) -> Boxed(output) {
  case count {
    0 -> function
    _ -> retain_custom(function, count - 1)
  }
}

fn retain_list(
  function: fn(input) -> List(output),
  count: Int,
) -> fn(input) -> List(output) {
  case count {
    0 -> function
    _ -> retain_list(function, count - 1)
  }
}

fn retain_function(
  function: fn(input) -> fn(argument) -> output,
  count: Int,
) -> fn(input) -> fn(argument) -> output {
  case count {
    0 -> function
    _ -> retain_function(function, count - 1)
  }
}

fn retain_int_function(
  function: fn(input) -> Int,
  count: Int,
) -> fn(input) -> Int {
  case count {
    0 -> function
    _ -> retain_int_function(function, count - 1)
  }
}

fn retain_float_function(
  function: fn(input) -> Float,
  count: Int,
) -> fn(input) -> Float {
  case count {
    0 -> function
    _ -> retain_float_function(function, count - 1)
  }
}

fn retain_string_function(
  function: fn(input) -> String,
  count: Int,
) -> fn(input) -> String {
  case count {
    0 -> function
    _ -> retain_string_function(function, count - 1)
  }
}

fn retain_bit_array_function(
  function: fn(input) -> BitArray,
  count: Int,
) -> fn(input) -> BitArray {
  case count {
    0 -> function
    _ -> retain_bit_array_function(function, count - 1)
  }
}

fn retain_utf_codepoint_function(
  function: fn(input) -> UtfCodepoint,
  count: Int,
) -> fn(input) -> UtfCodepoint {
  case count {
    0 -> function
    _ -> retain_utf_codepoint_function(function, count - 1)
  }
}

fn retain_bool_function(
  function: fn(input) -> Bool,
  count: Int,
) -> fn(input) -> Bool {
  case count {
    0 -> function
    _ -> retain_bool_function(function, count - 1)
  }
}

fn retain_nil_function(
  function: fn(input) -> Nil,
  count: Int,
) -> fn(input) -> Nil {
  case count {
    0 -> function
    _ -> retain_nil_function(function, count - 1)
  }
}

fn retain_tuple_function(
  function: fn(input) -> #(Int),
  count: Int,
) -> fn(input) -> #(Int) {
  case count {
    0 -> function
    _ -> retain_tuple_function(function, count - 1)
  }
}

pub fn main() {
  let generic = retain_generic(identity, 1)
  let custom = retain_custom(boxed, 1)
  let list = retain_list(listed, 1)
  let function = retain_function(nested_function, 1)
  let int = retain_int_function(int_function, 1)
  let float = retain_float_function(float_function, 1)
  let string = retain_string_function(string_function, 1)
  let bit_array = retain_bit_array_function(bit_array_function, 1)
  let utf_codepoint = retain_utf_codepoint_function(utf_codepoint_function, 1)
  let bool = retain_bool_function(bool_function, 1)
  let nil = retain_nil_function(nil_function, 1)
  let tuple = retain_tuple_function(tuple_function, 1)
  let string_list = retain_list(string_list_function, 1)
  let bit_array_list = retain_list(bit_array_list_function, 1)
  let utf_codepoint_list = retain_list(utf_codepoint_list_function, 1)
  let float_list = retain_list(float_list_function, 1)
  let bool_list = retain_list(bool_list_function, 1)
  let nil_list = retain_list(nil_list_function, 1)
  let retained_string_list = retain_value(["one"], 1)
  let retained_bit_array_list = retain_value([<<1>>], 1)
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  let retained_utf_codepoint_list = retain_value([codepoint], 1)
  let retained_float_list = retain_value([1.0], 1)
  let retained_bool_list = retain_value([True], 1)
  let retained_nil_list = retain_value([Nil], 1)
  #(
    generic == generic,
    custom == custom,
    list == list,
    function == function,
    int == int,
    float == float,
    string == string,
    bit_array == bit_array,
    utf_codepoint == utf_codepoint,
    bool == bool,
    nil == nil,
    tuple == tuple,
    string_list == string_list,
    bit_array_list == bit_array_list,
    utf_codepoint_list == utf_codepoint_list,
    float_list == float_list,
    bool_list == bool_list,
    nil_list == nil_list,
    retained_string_list == ["one"],
    retained_bit_array_list == [<<1>>],
    retained_utf_codepoint_list == [codepoint],
    retained_float_list == [1.0],
    retained_bool_list == [True],
    retained_nil_list == [Nil],
  )
}

// geam:expect Tuple([Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true)])
