const empty_constant = []

fn empty() -> List(value) {
  []
}

fn nested() -> List(List(value)) {
  [[]]
}

fn identity(value) {
  value
}

fn list_identity(values: List(value)) {
  values
}

fn nested_identity(values: List(List(value))) {
  values
}

fn tail_value(value, count: Int) {
  case count {
    0 -> value
    _ -> tail_value(value, count - 1)
  }
}

fn hold_function(function) {
  let local = function
  local
}

fn constant_empty() -> List(value) {
  empty_constant
}

fn constant_nested() -> List(List(value)) {
  empty_constant
}

fn call_empty(function: fn() -> List(value)) -> List(value) {
  function()
}

fn call_nested(function: fn() -> List(List(value))) -> List(List(value)) {
  function()
}

fn provide_empty() -> fn() -> List(value) {
  empty
}

fn provide_nested() -> fn() -> List(List(value)) {
  nested
}

fn tail_empty(count: Int) -> List(value) {
  case count {
    0 -> []
    _ -> tail_empty(count - 1)
  }
}

fn tail_nested(count: Int) -> List(List(value)) {
  case count {
    0 -> [[]]
    _ -> tail_nested(count - 1)
  }
}

pub fn main() {
  let empty_local = empty
  let nested_local = nested

  #(
    empty() == [],
    nested() == [[]],
    empty_local() == [],
    nested_local() == [[]],
    call_empty(empty) == [],
    call_nested(nested) == [[]],
    provide_empty()() == [],
    provide_nested()() == [[]],
    tail_empty(1) == [],
    tail_nested(1) == [[]],
    constant_empty() == [],
    constant_nested() == [],
    identity([]) == [],
    identity([[]]) == [[]],
    list_identity([[]]) == [[]],
    nested_identity([[1]]) == [[1]],
    tail_value([], 1) == [],
    tail_value([[]], 1) == [[]],
    hold_function(empty)() == [],
    hold_function(nested)() == [[]],
  )
}

// geam:expect Tuple([Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true)])
