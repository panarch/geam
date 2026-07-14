fn identity(value: Int) {
  value
}

pub fn main() {
  Ok(identity) == Ok(identity)
}

// geam:reject unsupported binary operator: equality on function values
