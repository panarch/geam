pub fn main() {
  let make = fn(value) { [value, value + 1] }
  make(1)
}

// geam:expect List(Int)([Int(1), Int(2)])
