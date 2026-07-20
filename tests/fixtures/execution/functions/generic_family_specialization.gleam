pub type Wrapped(value) {
  Wrapped(value: value)
}

pub type Callable(value) {
  Callable(function: fn(value) -> value)
}

pub type WrappedList(value) {
  WrappedList(values: List(value))
}

pub type Marker {
  Marker(Int)
}

fn identity(value: value) -> value {
  value
}

fn callable_identity(function: fn(value) -> value) -> fn(value) -> value {
  function
}

fn list_identity(values: List(value)) -> List(value) {
  values
}

fn generic_case(value: value, fallback: value) -> value {
  case value {
    selected as alias if True -> alias
    _ -> fallback
  }
}

fn generic_string_return(selector: String, value: value, fallback: value) -> value {
  case selector {
    "selected" -> value
    _ -> fallback
  }
}

fn generic_float_return(selector: Float, value: value, fallback: value) -> value {
  case selector {
    1.0 -> value
    _ -> fallback
  }
}

fn generic_block_return(value: value) -> value {
  {
    let _ = Nil
    value
  }
}

fn generic_string_callable(
  selector: String,
  function: fn(value) -> value,
  fallback: fn(value) -> value,
) -> fn(value) -> value {
  case selector {
    "selected" -> function
    _ -> fallback
  }
}

fn generic_float_callable(
  selector: Float,
  function: fn(value) -> value,
  fallback: fn(value) -> value,
) -> fn(value) -> value {
  case selector {
    1.0 -> function
    _ -> fallback
  }
}

fn generic_block_callable(function: fn(value) -> value) -> fn(value) -> value {
  {
    let _ = Nil
    function
  }
}

fn tail_identity(count: Int, value: value) -> value {
  case count {
    0 -> value
    _ -> tail_identity(count - 1, value)
  }
}

fn tail_list(count: Int, values: List(value)) -> List(value) {
  case count {
    0 -> values
    _ -> tail_list(count - 1, values)
  }
}

fn tail_callable(count: Int, function: fn(value) -> value) -> fn(value) -> value {
  case count {
    0 -> function
    _ -> tail_callable(count - 1, function)
  }
}

fn capture_value(value: value) -> fn() -> value {
  fn() { value }
}

fn capture_callable(function: fn(value) -> value) -> fn() -> fn(value) -> value {
  fn() { function }
}

fn capture_list(value: value) -> fn() -> List(value) {
  fn() { [value] }
}

fn call_through_anonymous(value: value, function: fn(value) -> value) -> value {
  let invoke = fn(callable: fn(value) -> value) { callable(value) }
  invoke(function)
}

fn generic_callable_subject(function: fn(value) -> value) -> fn(value) -> value {
  case function {
    selected -> selected
  }
}

fn exercise(value: value, fallback: value, mapper: fn(value) -> value) -> value {
  let local = value
  let direct = identity(local)
  let called = mapper(direct)
  let tuple = #(called)
  let from_tuple = tuple.0
  let wrapped = Wrapped(value: from_tuple)
  let from_field = wrapped.value
  let values = [from_field]
  let from_list = case values {
    [first, ..] -> first
    _ -> fallback
  }
  let from_bool = case True {
    True -> from_list
    False -> fallback
  }
  let from_int = case 1 {
    1 -> from_bool
    _ -> fallback
  }
  let from_string = case "hit" {
    "hit" -> from_int
    _ -> fallback
  }
  let from_float = case 1.0 {
    1.0 -> from_string
    _ -> fallback
  }
  let from_block = {
    let _ = Nil
    from_float
  }
  case True {
    True -> from_block
    False -> panic as "unselected generic value"
  }
}

fn exercise_callable(
  function: fn(value) -> value,
  fallback: fn(value) -> value,
  provider: fn() -> fn(value) -> value,
) -> fn(value) -> value {
  let seeded = case True {
    True -> identity
    False -> fn(item) { item }
  }
  let local = case True {
    True -> function
    False -> seeded
  }
  let direct = callable_identity(local)
  let called = provider()
  let tuple = #(direct)
  let from_tuple = tuple.0
  let wrapped = Callable(function: from_tuple)
  let from_field = wrapped.function
  let functions = [from_field]
  let from_list = case functions {
    [first, ..] -> first
    _ -> fallback
  }
  let from_bool = case True {
    True -> from_list
    False -> called
  }
  let from_int = case 1 {
    1 -> from_bool
    _ -> fallback
  }
  let from_string = case "hit" {
    "hit" -> from_int
    _ -> fallback
  }
  let from_float = case 1.0 {
    1.0 -> from_string
    _ -> fallback
  }
  let from_block = {
    let _ = Nil
    from_float
  }
  case True {
    True -> from_block
    False -> panic as "unselected generic function"
  }
}

fn exercise_list(
  value: value,
  fallback: value,
  provider: fn() -> List(value),
  mapper: fn(List(value)) -> List(value),
) -> List(value) {
  let literal = [value]
  let spread = [fallback, ..literal]
  let local = case spread {
    [_, ..tail] -> tail
    _ -> [fallback]
  }
  let direct = list_identity(local)
  let mapped = mapper(direct)
  let called = provider()
  let tuple = #(mapped)
  let from_tuple = tuple.0
  let wrapped = WrappedList(values: from_tuple)
  let WrappedList(values: rebound) = wrapped
  let from_field = wrapped.values
  let nested = [from_field, rebound]
  let from_nested = case nested {
    [first, ..] -> first
    _ -> [fallback]
  }
  let from_bool = case True {
    True -> from_nested
    False -> called
  }
  let from_int = case 1 {
    1 -> from_bool
    _ -> [fallback]
  }
  let from_string = case "hit" {
    "hit" -> from_int
    _ -> [fallback]
  }
  let from_float = case 1.0 {
    1.0 -> from_string
    _ -> [fallback]
  }
  let from_block = {
    let _ = Nil
    from_float
  }
  case True {
    True -> from_block
    False -> panic as "unselected generic list"
  }
}

fn assert_bound_callable(function: fn(value) -> value) -> fn(value) -> value {
  let assert [bound] = [function]
  bound
}

fn int_identity(value: Int) -> Int {
  value
}

fn string_identity(value: String) -> String {
  value
}

fn bool_identity(value: Bool) -> Bool {
  value
}

fn nil_identity(value: Nil) -> Nil {
  value
}

fn choose_int_function(selector: Float) -> fn(Int) -> Int {
  case selector {
    1.0 -> int_identity
    _ -> int_identity
  }
}

fn choose_string_function(selector: Float) -> fn(String) -> String {
  case selector {
    1.0 -> string_identity
    _ -> string_identity
  }
}

fn choose_bool_function(selector: Float) -> fn(Bool) -> Bool {
  case selector {
    1.0 -> bool_identity
    _ -> bool_identity
  }
}

fn choose_nil_function(selector: Float) -> fn(Nil) -> Nil {
  case selector {
    1.0 -> nil_identity
    _ -> nil_identity
  }
}

fn int_value() -> Int {
  1
}

fn float_value() -> Float {
  1.5
}

fn string_value() -> String {
  "one"
}

fn bit_array_value() -> BitArray {
  <<1>>
}

fn codepoint_value() -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
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
  [1]
}

fn function_value() -> fn(Int) -> Int {
  int_identity
}

fn custom_value() -> Marker {
  Marker(1)
}

fn assert_value_families(codepoint: UtfCodepoint) {
  assert exercise(1, 2, identity) == 1
  assert exercise(1.5, 2.5, identity) == 1.5
  assert exercise("one", "two", identity) == "one"
  assert exercise(<<1>>, <<2>>, identity) == <<1>>
  assert exercise(codepoint, codepoint, identity) == codepoint
  assert exercise(True, False, identity) == True
  assert exercise(Nil, Nil, identity) == Nil
  assert exercise(#(1, "one"), #(2, "two"), identity) == #(1, "one")
  assert exercise([1], [2], identity) == [1]
  assert exercise(Marker(1), Marker(2), identity) == Marker(1)
  assert exercise(int_identity, int_identity, identity) == int_identity

  assert generic_case(1, 2) == 1
  assert generic_case("one", "two") == "one"
}

fn assert_list_item_families(codepoint: UtfCodepoint) {
  assert exercise([1], [2], identity) == [1]
  assert exercise([1.5], [2.5], identity) == [1.5]
  assert exercise(["one"], ["two"], identity) == ["one"]
  assert exercise([<<1>>], [<<2>>], identity) == [<<1>>]
  assert exercise([codepoint], [codepoint], identity) == [codepoint]
  assert exercise([True], [False], identity) == [True]
  assert exercise([Nil], [Nil], identity) == [Nil]
  assert exercise([#(1, "one")], [#(2, "two")], identity) == [#(1, "one")]
  assert exercise([[1]], [[2]], identity) == [[1]]
  assert exercise([Marker(1)], [Marker(2)], identity) == [Marker(1)]
  assert exercise([int_identity], [int_identity], identity) == [int_identity]
}

fn assert_function_return_families() {
  assert exercise(int_value, int_value, identity) == int_value
  assert exercise(float_value, float_value, identity) == float_value
  assert exercise(string_value, string_value, identity) == string_value
  assert exercise(bit_array_value, bit_array_value, identity) == bit_array_value
  assert exercise(codepoint_value, codepoint_value, identity) == codepoint_value
  assert exercise(bool_value, bool_value, identity) == bool_value
  assert exercise(nil_value, nil_value, identity) == nil_value
  assert exercise(tuple_value, tuple_value, identity) == tuple_value
  assert exercise(list_value, list_value, identity) == list_value
  assert exercise(custom_value, custom_value, identity) == custom_value
  assert exercise(function_value, function_value, identity) == function_value
}

fn assert_callable_specializations(codepoint: UtfCodepoint) {
  assert exercise_callable(identity, identity, fn() { identity })(1) == 1
  assert exercise_callable(identity, identity, fn() { identity })(1.5) == 1.5
  assert exercise_callable(identity, identity, fn() { identity })("one") == "one"
  assert exercise_callable(identity, identity, fn() { identity })(<<1>>) == <<1>>
  assert exercise_callable(identity, identity, fn() { identity })(codepoint) == codepoint
  assert exercise_callable(identity, identity, fn() { identity })(True) == True
  assert exercise_callable(identity, identity, fn() { identity })(Nil) == Nil
  assert exercise_callable(identity, identity, fn() { identity })(#(1, "one")) == #(1, "one")
  assert exercise_callable(identity, identity, fn() { identity })([1]) == [1]
  assert exercise_callable(identity, identity, fn() { identity })(Marker(1)) == Marker(1)
  assert exercise_callable(identity, identity, fn() { identity })(int_identity) == int_identity
}

fn assert_generic_list_expressions(codepoint: UtfCodepoint) {
  assert exercise_list(1, 2, fn() { [1] }, list_identity) == [1]
  assert exercise_list(1.5, 2.5, fn() { [1.5] }, list_identity) == [1.5]
  assert exercise_list("one", "two", fn() { ["one"] }, list_identity) == ["one"]
  assert exercise_list(<<1>>, <<2>>, fn() { [<<1>>] }, list_identity) == [<<1>>]
  assert exercise_list(codepoint, codepoint, fn() { [codepoint] }, list_identity) == [codepoint]
  assert exercise_list(True, False, fn() { [True] }, list_identity) == [True]
  assert exercise_list(Nil, Nil, fn() { [Nil] }, list_identity) == [Nil]
  assert exercise_list(#(1, "one"), #(2, "two"), fn() { [#(1, "one")] }, list_identity) == [#(1, "one")]
  assert exercise_list([1], [2], fn() { [[1]] }, list_identity) == [[1]]
  assert exercise_list(Marker(1), Marker(2), fn() { [Marker(1)] }, list_identity) == [Marker(1)]
  assert exercise_list(int_identity, int_identity, fn() { [int_identity] }, list_identity) == [int_identity]
}

fn assert_generic_tail_calls(codepoint: UtfCodepoint) {
  assert tail_identity(1, 1) == 1
  assert tail_identity(1, 1.5) == 1.5
  assert tail_identity(1, "one") == "one"
  assert tail_identity(1, <<1>>) == <<1>>
  assert tail_identity(1, codepoint) == codepoint
  assert tail_identity(1, True) == True
  assert tail_identity(1, Nil) == Nil
  assert tail_identity(1, #(1, "one")) == #(1, "one")
  assert tail_identity(1, [1]) == [1]
  assert tail_identity(1, [Marker(1)]) == [Marker(1)]
  assert tail_identity(1, [#(1, "one")]) == [#(1, "one")]
  assert tail_identity(1, [[1]]) == [[1]]
  assert tail_identity(1, [int_identity]) == [int_identity]
  assert tail_identity(1, Marker(1)) == Marker(1)
  assert tail_identity(1, int_identity) == int_identity
  assert tail_identity(1, tuple_value) == tuple_value
  assert tail_identity(1, custom_value) == custom_value
  assert tail_identity(1, list_value) == list_value
  assert tail_identity(1, function_value) == function_value

  assert tail_list(1, [1]) == [1]
  assert tail_list(1, [1.5]) == [1.5]
  assert tail_list(1, ["one"]) == ["one"]
  assert tail_list(1, [<<1>>]) == [<<1>>]
  assert tail_list(1, [codepoint]) == [codepoint]
  assert tail_list(1, [True]) == [True]
  assert tail_list(1, [Nil]) == [Nil]
  assert tail_list(1, [#(1, "one")]) == [#(1, "one")]
  assert tail_list(1, [[1]]) == [[1]]
  assert tail_list(1, [Marker(1)]) == [Marker(1)]
  assert tail_list(1, [int_identity]) == [int_identity]

  assert tail_callable(1, identity)(1) == 1
  assert tail_callable(1, identity)(1.5) == 1.5
  assert tail_callable(1, identity)("one") == "one"
  assert tail_callable(1, identity)(<<1>>) == <<1>>
  assert tail_callable(1, identity)(codepoint) == codepoint
  assert tail_callable(1, identity)(True) == True
  assert tail_callable(1, identity)(Nil) == Nil
  assert tail_callable(1, identity)(#(1, "one")) == #(1, "one")
  assert tail_callable(1, identity)([1]) == [1]
  assert tail_callable(1, identity)(Marker(1)) == Marker(1)
  assert tail_callable(1, identity)(int_identity) == int_identity

  assert assert_bound_callable(identity)(1) == 1
  assert assert_bound_callable(identity)("one") == "one"
}

fn assert_generic_return_owners() {
  assert generic_string_return("selected", 1, 2) == 1
  assert generic_float_return(1.0, 1, 2) == 1
  assert generic_block_return(1) == 1
  assert generic_string_callable("selected", identity, identity)(1) == 1
  assert generic_float_callable(1.0, identity, identity)(1) == 1
  assert generic_block_callable(identity)(1) == 1
  assert capture_list("one")() == ["one"]

  assert choose_int_function(1.0)(1) == 1
  assert choose_string_function(1.0)("one") == "one"
  assert choose_bool_function(1.0)(True) == True
  assert choose_nil_function(1.0)(Nil) == Nil

  assert identity([Marker(1)]) == [Marker(1)]
  assert identity([#(1, "one")]) == [#(1, "one")]
  assert identity([[1]]) == [[1]]
  assert identity([int_identity]) == [int_identity]

  assert identity(tuple_value) == tuple_value
  assert identity(custom_value) == custom_value
  assert identity(list_value) == list_value
  assert identity(function_value) == function_value
}

fn assert_generic_value_captures(codepoint: UtfCodepoint) {
  assert capture_value(1)() == 1
  assert capture_value(1.5)() == 1.5
  assert capture_value("one")() == "one"
  assert capture_value(<<1>>)() == <<1>>
  assert capture_value(codepoint)() == codepoint
  assert capture_value(Marker(1))() == Marker(1)
  assert capture_value(True)() == True
  assert capture_value(Nil)() == Nil
  assert capture_value(#(1, "one"))() == #(1, "one")
  assert capture_value([1])() == [1]

  assert capture_value(int_value)() == int_value
  assert capture_value(float_value)() == float_value
  assert capture_value(string_value)() == string_value
  assert capture_value(bit_array_value)() == bit_array_value
  assert capture_value(codepoint_value)() == codepoint_value
  assert capture_value(custom_value)() == custom_value
  assert capture_value(bool_value)() == bool_value
  assert capture_value(nil_value)() == nil_value
  assert capture_value(tuple_value)() == tuple_value
  assert capture_value(list_value)() == list_value
  assert capture_value(function_value)() == function_value
}

fn assert_generic_callable_captures(codepoint: UtfCodepoint) {
  assert capture_callable(identity)()(1) == 1
  assert capture_callable(identity)()(1.5) == 1.5
  assert capture_callable(identity)()("one") == "one"
  assert capture_callable(identity)()(<<1>>) == <<1>>
  assert capture_callable(identity)()(codepoint) == codepoint
  assert capture_callable(identity)()(Marker(1)) == Marker(1)
  assert capture_callable(identity)()(True) == True
  assert capture_callable(identity)()(Nil) == Nil
  assert capture_callable(identity)()(#(1, "one")) == #(1, "one")
  assert capture_callable(identity)()([1]) == [1]
  assert capture_callable(identity)()(int_identity) == int_identity

  assert call_through_anonymous(1, identity) == 1
  assert call_through_anonymous("one", identity) == "one"
  assert generic_callable_subject(identity)(1) == 1
  assert generic_callable_subject(identity)("one") == "one"
}

pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  assert_value_families(codepoint)
  assert_list_item_families(codepoint)
  assert_function_return_families()
  assert_callable_specializations(codepoint)
  assert_generic_list_expressions(codepoint)
  assert_generic_tail_calls(codepoint)
  assert_generic_return_owners()
  assert_generic_value_captures(codepoint)
  assert_generic_callable_captures(codepoint)
  0
}

// geam:expect Int(0)
