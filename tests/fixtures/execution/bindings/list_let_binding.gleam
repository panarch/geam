fn id(value: Int) {
  value
}

pub fn main() {
  let values = [1, id(2), 3]
  let _ = values
  values
}

// @geam:expect List(Int)([Int(1), Int(2), Int(3)])
