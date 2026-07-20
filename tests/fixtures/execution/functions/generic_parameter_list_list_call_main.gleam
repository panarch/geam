fn nested() -> List(List(value)) {
  [[]]
}

pub fn main() {
  let result = nested()
  let _ = Nil
  result
}

// geam:expect List(List(Parameter(0)))([List(Parameter(0))([])])
