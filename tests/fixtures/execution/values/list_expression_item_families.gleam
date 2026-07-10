fn int_values() { [1] }
fn string_values() { ["one"] }
fn float_values() { [1.0] }
fn bool_values() { [True] }
fn nil_values() { [Nil] }
fn tuple_values() { [#(1)] }
fn list_values() { [[1]] }

fn one(value: Int) { value + 1 }
fn two(value: Int) { value + 2 }
fn function_values() { [one] }

fn is_one_list(values: List(fn(Int) -> Int)) {
  case values {
    [function] -> function(0) == 1
    _ -> False
  }
}

fn is_two_list(values: List(fn(Int) -> Int)) {
  case values {
    [function] -> function(0) == 2
    _ -> False
  }
}

fn is_two_one_list(values: List(fn(Int) -> Int)) {
  case values {
    [first, second] -> first(0) == 2 && second(0) == 1
    _ -> False
  }
}

fn int_lists() {
  let local = [1]
  let provider = int_values
  let nested = [[1]]
  let projected = case nested {
    [value] -> value
    _ -> []
  }
  let dropped = case [0, 1] {
    [_, ..rest] -> rest
    _ -> []
  }

  assert local == [1]
  assert [0, ..local] == [0, 1]
  assert int_values() == [1]
  assert provider() == [1]
  assert #(local).0 == [1]
  assert projected == [1]
  assert dropped == [1]
  assert case True { True -> local False -> [2] } == [1]
  assert case False { True -> local False -> [2] } == [2]
  assert case 1 { 1 -> local _ -> [2] } == [1]
  assert case 0 { 1 -> local _ -> [2] } == [2]
  assert case "hit" { "hit" -> local _ -> [2] } == [1]
  assert case "miss" { "hit" -> local _ -> [2] } == [2]
  assert case 1.0 { 1.0 -> local _ -> [2] } == [1]
  assert case 0.0 { 1.0 -> local _ -> [2] } == [2]
  assert { let _ = 0 local } == [1]
}

fn string_lists() {
  let local = ["one"]
  let provider = string_values
  let nested = [["one"]]
  let projected = case nested {
    [value] -> value
    _ -> []
  }
  let dropped = case ["zero", "one"] {
    [_, ..rest] -> rest
    _ -> []
  }

  assert local == ["one"]
  assert ["zero", ..local] == ["zero", "one"]
  assert string_values() == ["one"]
  assert provider() == ["one"]
  assert #(local).0 == ["one"]
  assert projected == ["one"]
  assert dropped == ["one"]
  assert case True { True -> local False -> ["two"] } == ["one"]
  assert case False { True -> local False -> ["two"] } == ["two"]
  assert case 1 { 1 -> local _ -> ["two"] } == ["one"]
  assert case 0 { 1 -> local _ -> ["two"] } == ["two"]
  assert case "hit" { "hit" -> local _ -> ["two"] } == ["one"]
  assert case "miss" { "hit" -> local _ -> ["two"] } == ["two"]
  assert case 1.0 { 1.0 -> local _ -> ["two"] } == ["one"]
  assert case 0.0 { 1.0 -> local _ -> ["two"] } == ["two"]
  assert { let _ = 0 local } == ["one"]
}

fn float_lists() {
  let local = [1.0]
  let provider = float_values
  let nested = [[1.0]]
  let projected = case nested {
    [value] -> value
    _ -> []
  }
  let dropped = case [0.0, 1.0] {
    [_, ..rest] -> rest
    _ -> []
  }

  assert local == [1.0]
  assert [0.0, ..local] == [0.0, 1.0]
  assert float_values() == [1.0]
  assert provider() == [1.0]
  assert #(local).0 == [1.0]
  assert projected == [1.0]
  assert dropped == [1.0]
  assert case True { True -> local False -> [2.0] } == [1.0]
  assert case False { True -> local False -> [2.0] } == [2.0]
  assert case 1 { 1 -> local _ -> [2.0] } == [1.0]
  assert case 0 { 1 -> local _ -> [2.0] } == [2.0]
  assert case "hit" { "hit" -> local _ -> [2.0] } == [1.0]
  assert case "miss" { "hit" -> local _ -> [2.0] } == [2.0]
  assert case 1.0 { 1.0 -> local _ -> [2.0] } == [1.0]
  assert case 0.0 { 1.0 -> local _ -> [2.0] } == [2.0]
  assert { let _ = 0 local } == [1.0]
}

fn bool_lists() {
  let local = [True]
  let provider = bool_values
  let nested = [[True]]
  let projected = case nested {
    [value] -> value
    _ -> []
  }
  let dropped = case [False, True] {
    [_, ..rest] -> rest
    _ -> []
  }

  assert local == [True]
  assert [False, ..local] == [False, True]
  assert bool_values() == [True]
  assert provider() == [True]
  assert #(local).0 == [True]
  assert projected == [True]
  assert dropped == [True]
  assert case True { True -> local False -> [False] } == [True]
  assert case False { True -> local False -> [False] } == [False]
  assert case 1 { 1 -> local _ -> [False] } == [True]
  assert case 0 { 1 -> local _ -> [False] } == [False]
  assert case "hit" { "hit" -> local _ -> [False] } == [True]
  assert case "miss" { "hit" -> local _ -> [False] } == [False]
  assert case 1.0 { 1.0 -> local _ -> [False] } == [True]
  assert case 0.0 { 1.0 -> local _ -> [False] } == [False]
  assert { let _ = 0 local } == [True]
}

fn nil_lists() {
  let local = [Nil]
  let provider = nil_values
  let nested = [[Nil]]
  let projected = case nested {
    [value] -> value
    _ -> []
  }
  let dropped = case [Nil, Nil] {
    [_, ..rest] -> rest
    _ -> []
  }

  assert local == [Nil]
  assert [Nil, ..local] == [Nil, Nil]
  assert nil_values() == [Nil]
  assert provider() == [Nil]
  assert #(local).0 == [Nil]
  assert projected == [Nil]
  assert dropped == [Nil]
  assert case True { True -> local False -> [Nil] } == [Nil]
  assert case False { True -> local False -> [Nil] } == [Nil]
  assert case 1 { 1 -> local _ -> [Nil] } == [Nil]
  assert case 0 { 1 -> local _ -> [Nil] } == [Nil]
  assert case "hit" { "hit" -> local _ -> [Nil] } == [Nil]
  assert case "miss" { "hit" -> local _ -> [Nil] } == [Nil]
  assert case 1.0 { 1.0 -> local _ -> [Nil] } == [Nil]
  assert case 0.0 { 1.0 -> local _ -> [Nil] } == [Nil]
  assert { let _ = 0 local } == [Nil]
}

fn tuple_lists() {
  let local = [#(1)]
  let provider = tuple_values
  let nested = [[#(1)]]
  let projected = case nested {
    [value] -> value
    _ -> []
  }
  let dropped = case [#(0), #(1)] {
    [_, ..rest] -> rest
    _ -> []
  }

  assert local == [#(1)]
  assert [#(0), ..local] == [#(0), #(1)]
  assert tuple_values() == [#(1)]
  assert provider() == [#(1)]
  assert #(local).0 == [#(1)]
  assert projected == [#(1)]
  assert dropped == [#(1)]
  assert case True { True -> local False -> [#(2)] } == [#(1)]
  assert case False { True -> local False -> [#(2)] } == [#(2)]
  assert case 1 { 1 -> local _ -> [#(2)] } == [#(1)]
  assert case 0 { 1 -> local _ -> [#(2)] } == [#(2)]
  assert case "hit" { "hit" -> local _ -> [#(2)] } == [#(1)]
  assert case "miss" { "hit" -> local _ -> [#(2)] } == [#(2)]
  assert case 1.0 { 1.0 -> local _ -> [#(2)] } == [#(1)]
  assert case 0.0 { 1.0 -> local _ -> [#(2)] } == [#(2)]
  assert { let _ = 0 local } == [#(1)]
}

fn list_lists() {
  let local = [[1]]
  let provider = list_values
  let nested = [[[1]]]
  let projected = case nested {
    [value] -> value
    _ -> []
  }
  let dropped = case [[0], [1]] {
    [_, ..rest] -> rest
    _ -> []
  }

  assert local == [[1]]
  assert [[0], ..local] == [[0], [1]]
  assert list_values() == [[1]]
  assert provider() == [[1]]
  assert #(local).0 == [[1]]
  assert projected == [[1]]
  assert dropped == [[1]]
  assert case True { True -> local False -> [[2]] } == [[1]]
  assert case False { True -> local False -> [[2]] } == [[2]]
  assert case 1 { 1 -> local _ -> [[2]] } == [[1]]
  assert case 0 { 1 -> local _ -> [[2]] } == [[2]]
  assert case "hit" { "hit" -> local _ -> [[2]] } == [[1]]
  assert case "miss" { "hit" -> local _ -> [[2]] } == [[2]]
  assert case 1.0 { 1.0 -> local _ -> [[2]] } == [[1]]
  assert case 0.0 { 1.0 -> local _ -> [[2]] } == [[2]]
  assert { let _ = 0 local } == [[1]]
}

fn function_lists() {
  let local = [one]
  let provider = function_values
  let nested = [[one]]
  let projected = case nested {
    [value] -> value
    _ -> []
  }
  let dropped = case [two, one] {
    [_, ..rest] -> rest
    _ -> []
  }

  assert is_one_list(local)
  assert is_two_one_list([two, ..local])
  assert is_one_list(function_values())
  assert is_one_list(provider())
  assert is_one_list(#(local).0)
  assert is_one_list(projected)
  assert is_one_list(dropped)
  assert is_one_list(case True { True -> local False -> [two] })
  assert is_two_list(case False { True -> local False -> [two] })
  assert is_one_list(case 1 { 1 -> local _ -> [two] })
  assert is_two_list(case 0 { 1 -> local _ -> [two] })
  assert is_one_list(case "hit" { "hit" -> local _ -> [two] })
  assert is_two_list(case "miss" { "hit" -> local _ -> [two] })
  assert is_one_list(case 1.0 { 1.0 -> local _ -> [two] })
  assert is_two_list(case 0.0 { 1.0 -> local _ -> [two] })
  assert is_one_list({ let _ = 0 local })
}

pub fn main() {
  int_lists()
  string_lists()
  float_lists()
  bool_lists()
  nil_lists()
  tuple_lists()
  list_lists()
  function_lists()
  42
}

// geam:expect Int(42)
