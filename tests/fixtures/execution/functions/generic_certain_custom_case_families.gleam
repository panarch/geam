pub type Boxed(value) {
  Boxed(value)
}

pub type Filled(value) {
  Filled(value)
}

fn codepoint(value: Int) -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<value>>
  value
}

fn identity(value: value) -> value {
  value
}

fn unresolved() -> value {
  panic
}

fn unresolved_filled() -> Filled(value) {
  Filled(panic)
}

fn select(result: Result(value, error), fallback: value) -> value {
  let selected = case result {
    Ok(value) -> value
    Error(_) -> fallback
  }
  selected
}

fn select_symbolic_function(
  result: Result(fn(value) -> value, error),
) -> fn(value) -> value {
  let selected = case result {
    Ok(function) -> function
    Error(_) -> identity
  }
  selected
}

fn select_never_function(
  result: Result(fn() -> value, error),
) -> fn() -> value {
  let selected = case result {
    Ok(function) -> function
    Error(_) -> unresolved
  }
  selected
}

fn select_symbolic_constructor(
  result: Result(fn(value) -> Boxed(value), error),
) -> fn(value) -> Boxed(value) {
  let selected = case result {
    Ok(function) -> function
    Error(_) -> Boxed
  }
  selected
}

fn select_uninhabited_custom_function(
  result: Result(fn() -> Filled(value), error),
) -> fn() -> Filled(value) {
  let selected = case result {
    Ok(function) -> function
    Error(_) -> #(unresolved_filled).0
  }
  selected
}

fn select_after_false_block(first: value, second: value) -> value {
  case {
    let _ = Nil
    False
  } {
    True -> first
    False -> second
  }
}

fn select_after_dynamic_block(
  selector: Bool,
  first: value,
  second: value,
) -> value {
  case {
    let _ = Nil
    selector
  } {
    True -> first
    False -> second
  }
}

fn exact_ok(value: value) -> value {
  case Ok(value) {
    Error(_) -> value
    Ok(inner) -> inner
  }
}

pub fn main() {
  let int_function = select(Ok(fn() { 1 }), fn() { 2 })
  let float_function = select(Ok(fn() { 1.5 }), fn() { 2.5 })
  let string_function = select(Ok(fn() { "first" }), fn() { "second" })
  let bit_array_function = select(Ok(fn() { <<1>> }), fn() { <<2>> })
  let utf_codepoint_function =
    select(Ok(fn() { codepoint(65) }), fn() { codepoint(66) })
  let custom_function = select(Ok(fn() { Boxed(1) }), fn() { Boxed(2) })
  let bool_function = select(Ok(fn() { True }), fn() { False })
  let nil_function = select(Ok(fn() { Nil }), fn() { Nil })
  let tuple_function = select(Ok(fn() { #(1, "first") }), fn() {
    #(2, "second")
  })
  let list_function = select(Ok(fn() { [1] }), fn() { [2] })
  let function_function = select(Ok(fn() { fn() { 1 } }), fn() {
    fn() { 2 }
  })
  let generic_function = select(Ok(identity), identity)
  let never_function = select(Ok(unresolved), unresolved)
  let constructor_function = select(Ok(Boxed), Boxed)
  let symbolic_function = select_symbolic_function(Ok(identity))
  let selected_never_function = select_never_function(Ok(unresolved))
  let symbolic_constructor = select_symbolic_constructor(Ok(Boxed))
  let uninhabited_custom_function =
    select_uninhabited_custom_function(Ok(unresolved_filled))
  let projected_uninhabited_custom_function = #(unresolved_filled).0

  #(
    select(Ok(1), 2),
    select(Ok(1.5), 2.5),
    select(Ok("first"), "second"),
    select(Ok(<<1>>), <<2>>),
    select(Ok(codepoint(65)), codepoint(66)),
    select(Ok(Boxed(1)), Boxed(2)),
    select(Ok(True), False),
    select(Ok(Nil), Nil),
    select(Ok(#(1, "first")), #(2, "second")),
    select(Ok([1]), [2]),
    select(Ok([]), []) == [],
    select(Ok([[]]), [[]]) == [[]],
    int_function(),
    float_function(),
    string_function(),
    bit_array_function(),
    utf_codepoint_function(),
    custom_function(),
    bool_function(),
    nil_function(),
    tuple_function(),
    list_function(),
    function_function()(),
    generic_function(3),
    never_function == unresolved,
    constructor_function(4),
    select(Ok(identity), identity) == identity,
    symbolic_function == identity,
    selected_never_function == unresolved,
    symbolic_constructor == Boxed,
    projected_uninhabited_custom_function == unresolved_filled,
    uninhabited_custom_function == unresolved_filled,
    select_after_false_block(1, 2),
    select_after_dynamic_block(True, "first", "second"),
    exact_ok(5),
  )
}

// geam:expect Tuple([Int(1), Float(1.5), String("first"), BitArray(bytes=[1], bit_len=8), UtfCodepoint('A'), Custom(type=geam/main/Boxed(Int), constructor=Boxed#0, fields=[Int(1)]), Bool(true), Nil, Tuple([Int(1), String("first")]), List(Int)([Int(1)]), Bool(true), Bool(true), Int(1), Float(1.5), String("first"), BitArray(bytes=[1], bit_len=8), UtfCodepoint('A'), Custom(type=geam/main/Boxed(Int), constructor=Boxed#0, fields=[Int(1)]), Bool(true), Nil, Tuple([Int(1), String("first")]), List(Int)([Int(1)]), Int(1), Int(3), Bool(true), Custom(type=geam/main/Boxed(Int), constructor=Boxed#0, fields=[Int(4)]), Bool(true), Bool(true), Bool(true), Bool(false), Bool(true), Bool(true), Int(2), String("first"), Int(5)])
