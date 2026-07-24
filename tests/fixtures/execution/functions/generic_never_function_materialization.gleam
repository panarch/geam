fn diverge(_value: Int) -> value {
  panic as "unreached diverging function"
}

fn capture() {
  let function = diverge
  fn() { function }
}

pub fn main() {
  #(diverge, capture())
}

// @geam:expect Tuple([Function(fn(Int) -> Parameter(0)), Function(fn() -> fn(Int) -> Parameter(1))])
