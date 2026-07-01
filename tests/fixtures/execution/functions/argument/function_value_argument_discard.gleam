fn add_one(value: Int) {
  value + 1
}

fn ignore(_: fn(Int) -> Int) {
  42
}

pub fn main() {
  ignore(add_one)
}

// geam:expect Int(42)
