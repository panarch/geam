fn values(value: Int) {
  [value]
}

fn apply(callback: fn(Int) -> List(Int), value: Int) {
  callback(value)
}

pub fn main() {
  apply(values, 41)
}

// @geam:expect List(Int)([Int(41)])
