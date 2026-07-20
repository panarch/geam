fn identity(value) {
  value
}

fn nested_identity(values: List(List(value))) -> List(List(value)) {
  values
}

fn nested_direct_call(values: List(List(value))) -> List(List(value)) {
  nested_identity(values)
}

fn nested_generic_call(values: List(List(value))) {
  identity(values)
}

fn identity_list(values: List(value)) -> List(value) {
  values
}

fn nested_generic_list_call(values: List(List(value))) -> List(List(value)) {
  identity_list(values)
}

fn nested_literal() -> List(List(value)) {
  [[]]
}

fn nested_literal_call() -> List(List(value)) {
  nested_literal()
}

fn int_nested_literal_call() -> List(List(Int)) {
  nested_literal_call()
}

fn first_or_empty(values: List(List(value))) -> List(value) {
  case values {
    [first, ..] -> first
    _ -> []
  }
}

fn first_nested(values: List(List(List(value)))) -> List(List(value)) {
  case values {
    [first, ..] -> first
    _ -> [[]]
  }
}

fn nested_singleton(value) -> List(List(value)) {
  [[value]]
}

fn triple_singleton(value) -> List(List(List(value))) {
  [[[value]]]
}

fn tail_wrapped(value, count: Int) -> List(value) {
  case count {
    0 -> [value]
    _ -> tail_wrapped(value, count - 1)
  }
}

fn tail_nested(values: List(List(value)), count: Int) -> List(List(value)) {
  case count {
    0 -> values
    _ -> tail_nested(values, count - 1)
  }
}

pub fn main() {
  #(
    nested_direct_call([[]]) == [[]],
    nested_direct_call([[1]]) == [[1]],
    nested_generic_call([[]]) == [[]],
    nested_generic_list_call([[]]) == [[]],
    nested_generic_list_call([[1]]) == [[1]],
    nested_literal_call() == [[]],
    int_nested_literal_call() == [[]],
    first_or_empty([[[]]]) == [[]],
    first_nested([[[]]]) == [[]],
    nested_singleton([]) == [[[]]],
    triple_singleton(1) == [[[1]]],
    tail_wrapped([], 1) == [[]],
    tail_nested([[]], 1) == [[]],
    tail_nested([[1]], 1) == [[1]],
  )
}

// geam:expect Tuple([Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true)])
