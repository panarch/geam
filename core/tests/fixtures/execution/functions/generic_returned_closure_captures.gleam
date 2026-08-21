fn identity(value: value) {
  value
}

fn capture_empty() -> fn() -> List(value) {
  let values = []
  fn() { values }
}

fn capture_nested() -> fn() -> List(List(value)) {
  let values = [[]]
  fn() { values }
}

fn capture_function() -> fn(value) -> value {
  let function = identity
  fn(value) { function(value) }
}

pub fn main() {
  #(
    capture_empty(),
    capture_nested(),
    capture_function(),
  )
}

// @geam:expect Tuple([Function(fn() -> List(Parameter(0))), Function(fn() -> List(List(Parameter(1)))), Function(fn(Parameter(2)) -> Parameter(2))])
