fn identity(value: value) {
  value
}

fn diverge(_value: Int) -> value {
  panic
}

fn capture_all() {
  let empty = []
  let nested = [[]]
  let concrete = [[1]]
  let generic = identity
  let never = diverge
  fn() { #(empty, nested, concrete, generic, never) }
}

pub fn main() {
  capture_all()
}

// geam:expect Function(fn() -> #(List(Parameter(0)), List(List(Parameter(1))), List(List(Int)), fn(Parameter(2)) -> Parameter(2), fn(Int) -> Parameter(3)))
