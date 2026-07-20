pub type Boxed(value) {
  Boxed(value)
}

fn codepoint(value: Int) -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<value>>
  value
}

fn choose(selector: Bool, first: value, second: value) {
  case selector {
    True -> first
    False -> second
  }
}

fn choose_or_panic(selector: Bool, value: value) {
  case selector {
    True -> value
    False -> panic as "unselected generic value"
  }
}

fn identity(value) {
  value
}

pub fn main() {
  let int_function =
    choose(False, fn(_value: Int) { 1 }, fn(_value: Int) { 2 })
  let float_function =
    choose(False, fn(_value: Int) { 1.0 }, fn(_value: Int) { 2.0 })
  let string_function =
    choose(False, fn(_value: Int) { "first" }, fn(_value: Int) { "second" })
  let bit_array_function =
    choose(False, fn(_value: Int) { <<1>> }, fn(_value: Int) { <<2>> })
  let utf_codepoint_function = choose(
    False,
    fn(_value: Int) { codepoint(65) },
    fn(_value: Int) { codepoint(66) },
  )
  let custom_function = choose(
    False,
    fn(_value: Int) { Boxed(1) },
    fn(_value: Int) { Boxed(2) },
  )
  let bool_function =
    choose(False, fn(_value: Int) { True }, fn(_value: Int) { False })
  let nil_function =
    choose(False, fn(_value: Int) { Nil }, fn(_value: Int) { Nil })
  let tuple_function = choose(
    False,
    fn(_value: Int) { #(1, "first") },
    fn(_value: Int) { #(2, "second") },
  )
  let list_function =
    choose(False, fn(_value: Int) { [1] }, fn(_value: Int) { [2] })
  let function_function = choose(
    False,
    fn(_value: Int) { fn(value: Int) { value + 1 } },
    fn(_value: Int) { fn(value: Int) { value + 2 } },
  )
  let symbolic_function = choose(False, identity, identity)

  let panic_int_function = choose_or_panic(True, fn(_value: Int) { 1 })
  let panic_tuple_function =
    choose_or_panic(True, fn(_value: Int) { #(1, "one") })
  let panic_custom_function =
    choose_or_panic(True, fn(_value: Int) { Boxed(1) })
  let panic_list_function = choose_or_panic(True, fn(_value: Int) { [1] })
  let panic_function_function =
    choose_or_panic(True, fn(_value: Int) { fn(value: Int) { value } })
  let panic_symbolic_function = choose_or_panic(True, identity)

  #(
    choose(False, 1, 2),
    choose(False, 1.0, 2.0),
    choose(False, "first", "second"),
    choose(False, <<1>>, <<2>>),
    choose(False, codepoint(65), codepoint(66)),
    choose(False, Boxed(1), Boxed(2)),
    choose(False, True, False),
    choose(False, Nil, Nil),
    choose(False, #(1, "first"), #(2, "second")),
    choose(False, [1], [2]),
    choose(False, [], []) == [],
    choose(False, [[]], [[]]) == [[]],
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
    symbolic_function(3),
    choose_or_panic(True, 1),
    choose_or_panic(True, #(1, "one")),
    choose_or_panic(True, Boxed(1)),
    choose_or_panic(True, [1]),
    choose_or_panic(True, []) == [],
    choose_or_panic(True, [[]]) == [[]],
    panic_int_function(0),
    panic_tuple_function(0),
    panic_custom_function(0),
    panic_list_function(0),
    panic_function_function(0)(1),
    panic_symbolic_function(4),
  )
}

// geam:expect Tuple([Int(2), Float(2.0), String("second"), BitArray(bytes=[2], bit_len=8), UtfCodepoint('B'), Custom(type=geam/main/Boxed(Int), constructor=Boxed#0, fields=[Int(2)]), Bool(false), Nil, Tuple([Int(2), String("second")]), List(Int)([Int(2)]), Bool(true), Bool(true), Int(2), Float(2.0), String("second"), BitArray(bytes=[2], bit_len=8), UtfCodepoint('B'), Custom(type=geam/main/Boxed(Int), constructor=Boxed#0, fields=[Int(2)]), Bool(false), Nil, Tuple([Int(2), String("second")]), List(Int)([Int(2)]), Int(3), Int(3), Int(1), Tuple([Int(1), String("one")]), Custom(type=geam/main/Boxed(Int), constructor=Boxed#0, fields=[Int(1)]), List(Int)([Int(1)]), Bool(true), Bool(true), Int(1), Tuple([Int(1), String("one")]), Custom(type=geam/main/Boxed(Int), constructor=Boxed#0, fields=[Int(1)]), List(Int)([Int(1)]), Int(1), Int(4)])
