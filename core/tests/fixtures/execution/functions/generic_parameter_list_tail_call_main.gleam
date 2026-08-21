fn empty(count: Int) -> List(value) {
  case count {
    0 -> []
    _ -> empty(count - 1)
  }
}

pub fn main() {
  empty(1)
}

// @geam:expect List(Parameter(0))([])
