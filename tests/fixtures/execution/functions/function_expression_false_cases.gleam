pub type Token {
  Token(Int)
}

fn identity(value) {
  value
}

fn diverge(_value: Int) -> value {
  panic as "unreached diverging function"
}

fn codepoint(value: Int) -> UtfCodepoint {
  let assert <<codepoint:utf8_codepoint>> = <<value>>
  codepoint
}

pub fn main() {
  let selector = False
  let int_function = case selector {
    True -> fn(_value: Int) { 1 }
    False -> fn(_value: Int) { 2 }
  }
  let float_function = case selector {
    True -> fn(_value: Int) { 1.0 }
    False -> fn(_value: Int) { 2.0 }
  }
  let string_function = case selector {
    True -> fn(_value: Int) { "one" }
    False -> fn(_value: Int) { "two" }
  }
  let bit_array_function = case selector {
    True -> fn(_value: Int) { <<1>> }
    False -> fn(_value: Int) { <<2>> }
  }
  let utf_codepoint_function = case selector {
    True -> fn(_value: Int) { codepoint(65) }
    False -> fn(_value: Int) { codepoint(66) }
  }
  let custom_function = case selector {
    True -> fn(_value: Int) { Token(1) }
    False -> fn(_value: Int) { Token(2) }
  }
  let bool_function = case selector {
    True -> fn(_value: Int) { True }
    False -> fn(_value: Int) { False }
  }
  let nil_function = case selector {
    True -> fn(_value: Int) { Nil }
    False -> fn(_value: Int) { Nil }
  }
  let tuple_function = case selector {
    True -> fn(_value: Int) { #(1, "one") }
    False -> fn(_value: Int) { #(2, "two") }
  }
  let list_function = case selector {
    True -> fn(_value: Int) { [1] }
    False -> fn(_value: Int) { [2] }
  }
  let function_function = case selector {
    True -> fn(_value: Int) { fn(value: Int) { value + 1 } }
    False -> fn(_value: Int) { fn(value: Int) { value + 2 } }
  }
  let generic_function = case selector {
    True -> identity
    False -> identity
  }
  let never_function = case selector {
    True -> diverge
    False -> diverge
  }

  #(
    int_function(0),
    float_function(0),
    string_function(0),
    bit_array_function(0),
    utf_codepoint_function(0),
    custom_function(0),
    bool_function(0),
    nil_function(0),
    tuple_function(0),
    list_function(0),
    function_function(0)(1),
    generic_function(3),
    never_function == never_function,
  )
}

// geam:expect Tuple([Int(2), Float(2.0), String("two"), BitArray(bytes=[2], bit_len=8), UtfCodepoint('B'), Custom(type=geam/main/Token, constructor=Token#0, fields=[Int(2)]), Bool(false), Nil, Tuple([Int(2), String("two")]), List(Int)([Int(2)]), Int(3), Int(3), Bool(true)])
