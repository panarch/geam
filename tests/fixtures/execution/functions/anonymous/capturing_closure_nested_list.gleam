fn retain(values: List(List(Int))) {
  fn() { values }
}

pub fn main() {
  let closure = retain([[1, 2]])
  closure()
}

// geam:expect List(List(Int))([List(Int)([Int(1), Int(2)])])
