fn tail(values: List(value)) {
  let [..tail] = values
  tail
}

pub fn main() {
  tail([])
}

// geam:expect List(Parameter(0))([])
