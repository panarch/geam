fn nested(count: Int) -> List(List(value)) {
  case count {
    0 -> [[]]
    _ -> nested(count - 1)
  }
}

pub fn main() {
  nested(1)
}

// geam:expect List(List(Parameter(0)))([List(Parameter(0))([])])
