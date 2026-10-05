fn identity(value: Bool) -> Bool {
  value
}

pub fn nonempty(values: List(Int)) -> Bool {
  let empty = case values {
    [] -> True
    _ -> False
  }
  let result = identity(empty)
  !result
}

pub fn main() {
  let _ = nonempty([])
  Nil
}
