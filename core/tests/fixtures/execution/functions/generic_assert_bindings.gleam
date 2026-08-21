fn identity(value: value) {
  value
}

fn bind_function(function: fn(value) -> value) {
  let assert #(bound, 1) = #(function, 1)
  bound
}

fn bind_list(values: List(value)) {
  let assert #(bound, 1) = #(values, 1)
  bound
}

pub fn main() {
  #(bind_function(identity), bind_list([]))
}

// @geam:expect Tuple([Function(fn(Parameter(0)) -> Parameter(0)), List(Parameter(1))([])])
