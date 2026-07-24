fn int_function() { 1 }
fn string_function() { "one" }
fn float_function() { 1.0 }
fn bool_function() { True }
fn nil_function() { Nil }
fn tuple_function() { #(1) }
fn list_function() { [1] }
fn function_function() { int_function }

fn other_function() { 2 }

fn accept_int(values: List(Int)) { assert values == [1] }
fn accept_string(values: List(String)) { assert values == ["one"] }
fn accept_float(values: List(Float)) { assert values == [1.0] }
fn accept_bool(values: List(Bool)) { assert values == [True] }
fn accept_nil(values: List(Nil)) { assert values == [Nil] }
fn accept_tuple(values: List(#(Int))) { assert values == [#(1)] }
fn accept_list(values: List(List(Int))) { assert values == [[1]] }
fn accept_function(values: List(fn() -> Int)) {
  let assert [function] = values
  assert function() == 1
}

pub fn main() {
  let ints = [1]
  let strings = ["one"]
  let floats = [1.0]
  let bools = [True]
  let nils = [Nil]
  let tuples = [#(1)]
  let lists = [[1]]
  let functions = [int_function]

  accept_int(ints)
  accept_string(strings)
  accept_float(floats)
  accept_bool(bools)
  accept_nil(nils)
  accept_tuple(tuples)
  accept_list(lists)
  accept_function(functions)

  let capture_int = fn() { accept_int(ints) }
  let capture_string = fn() { accept_string(strings) }
  let capture_float = fn() { accept_float(floats) }
  let capture_bool = fn() { accept_bool(bools) }
  let capture_nil = fn() { accept_nil(nils) }
  let capture_tuple = fn() { accept_tuple(tuples) }
  let capture_list = fn() { accept_list(lists) }
  let capture_function = fn() { accept_function(functions) }

  capture_int()
  capture_string()
  capture_float()
  capture_bool()
  capture_nil()
  capture_tuple()
  capture_list()
  capture_function()

  let assert [_, ..int_tail] = [0, 1]
  let assert [_, ..string_tail] = ["zero", "one"]
  let assert [_, ..float_tail] = [0.0, 1.0]
  let assert [_, ..bool_tail] = [False, True]
  let assert [_, ..nil_tail] = [Nil, Nil]
  let assert [_, ..tuple_tail] = [#(0), #(1)]
  let assert [_, ..list_tail] = [[0], [1]]
  let assert [_, ..function_tail] = [other_function, int_function]

  accept_int(int_tail)
  accept_string(string_tail)
  accept_float(float_tail)
  accept_bool(bool_tail)
  accept_nil(nil_tail)
  accept_tuple(tuple_tail)
  accept_list(list_tail)
  accept_function(function_tail)

  let assert [#(
    int_value,
    string_value,
    float_value,
    bool_value,
    nil_value,
    tuple_value,
    list_value,
    int_callback,
    string_callback,
    float_callback,
    bool_callback,
    nil_callback,
    tuple_callback,
    list_callback,
    function_callback,
  )] = [#(
    1,
    "one",
    1.0,
    True,
    Nil,
    #(1),
    [1],
    int_function,
    string_function,
    float_function,
    bool_function,
    nil_function,
    tuple_function,
    list_function,
    function_function,
  )]

  assert int_value == 1
  assert string_value == "one"
  assert float_value == 1.0
  assert bool_value
  nil_value
  assert tuple_value == #(1)
  assert list_value == [1]
  assert int_callback() == 1
  assert string_callback() == "one"
  assert float_callback() == 1.0
  assert bool_callback()
  nil_callback()
  assert tuple_callback() == #(1)
  assert list_callback() == [1]
  function_callback()()
}

// @geam:expect Int(1)
