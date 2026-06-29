fn add_one(value: Int) {
  value + 1
}

fn get() {
  add_one
}

pub fn main() {
  get
}

// geam:expect Function(fn() -> fn(Int) -> Int)
