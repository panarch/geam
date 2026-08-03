import gleam/function

type Boxed(value) {
  Boxed(value)
}

pub fn main() {
  assert function.identity(1) == 1
  assert function.identity(#("one", True)) == #("one", True)
  assert function.identity([1, 2, 3]) == [1, 2, 3]
  assert function.identity(Boxed("value")) == Boxed("value")

  let increment = function.identity(fn(value) { value + 1 })
  assert increment(1) == 2

  Nil
}
// @geam:expect Nil
