fn identity(value: Bool) -> Bool {
  value
}

pub fn flip(value: Bool) -> Bool {
  let result = identity(value)
  !result
}

pub fn main() {
  let _ = flip(False)
  Nil
}
