fn append(value: Int) {
  [value, 2]
}

pub fn main() {
  let base = [1]
  let make = append
  let closure = fn(value) {
    case value {
      0 -> base
      _ -> make(value)
    }
  }
  closure(1)
}

// geam:expect List(Int)([Int(1), Int(2)])
