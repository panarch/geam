fn done(count: Int, values: List(Int)) {
  case count {
    0 -> values
    _ -> done(count - 1, values)
  }
}

pub fn main() {
  done(10000, [1, 2, 3])
}

// @geam:expect List(Int)([Int(1), Int(2), Int(3)])
