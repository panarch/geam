pub fn main() {
  case Ok(1) {
    Ok(value) -> value
    Error(Nil) -> 0
  }
}

// @geam:expect Int(1)
