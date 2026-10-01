import boxed as wrapped

fn matches(value: a, expected: wrapped.Boxed(a)) -> Bool {
  case expected {
    candidate if wrapped.Boxed(value) == candidate -> True
    _ -> False
  }
}

fn function_matches(expected: wrapped.Boxed(fn(Int) -> Int)) -> Bool {
  case expected {
    candidate if wrapped.Boxed(wrapped.identity) == candidate -> True
    _ -> False
  }
}

pub fn main() {
  #(
    matches(7, wrapped.Boxed(7)),
    matches(7, wrapped.Boxed(8)),
    matches("a", wrapped.Boxed("a")),
    matches([7], wrapped.Boxed([7])),
    function_matches(wrapped.Boxed(wrapped.identity)),
    function_matches(wrapped.Boxed(fn(value) { value })),
  )
}
// @geam:expect Tuple([Bool(true), Bool(false), Bool(true), Bool(true), Bool(true), Bool(false)])
