pub fn main() {
  let base = 10
  let make = fn(value) { [base, value] }
  make(1)
}

// geam:expect List(Int)([Int(10), Int(1)])
