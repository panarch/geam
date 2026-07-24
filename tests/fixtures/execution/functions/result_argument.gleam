fn inspect(value: Result(Int, Nil)) {
  case value {
    Ok(value) -> value
    Error(Nil) -> 0
  }
}

pub fn main() {
  inspect(Ok(1))
}

// @geam:expect Int(1)
