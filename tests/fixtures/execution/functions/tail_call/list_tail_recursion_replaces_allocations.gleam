fn done(count: Int, values: List(Int)) {
  case count {
    0 -> values
    _ -> done(count - 1, [count])
  }
}

pub fn main() {
  done(10000, [])
}

// geam:expect List(Int)([Int(1)])
