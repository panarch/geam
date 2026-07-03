fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  {
    let rest = [2]
    [1, ..rest]
    add_one
  }
}

// geam:expect Function(fn(Int) -> Int)
