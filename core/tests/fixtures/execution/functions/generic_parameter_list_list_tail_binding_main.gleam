fn tail(values: List(List(value))) -> List(List(value)) {
  let assert [_, ..tail] = values
  tail
}

pub fn main() {
  tail([[]])
}

// @geam:expect List(List(Parameter(0)))([])
