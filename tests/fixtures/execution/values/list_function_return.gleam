fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  [add_one]
}

// @geam:expect List(fn(Int) -> Int)([Function(fn(Int) -> Int)])
