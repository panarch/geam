pub fn main() {
  let rest = [2, 3]
  [1, ..rest]
}

// @geam:expect List(Int)([Int(1), Int(2), Int(3)])
