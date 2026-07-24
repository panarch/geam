fn add_one(value: Int) {
  value + 1
}

fn get() {
  add_one
}

fn run(getter: fn() -> fn(Int) -> Int, value: Int) {
  getter()(value)
}

pub fn main() {
  let runner = run
  runner(get, 41)
}

// @geam:expect Int(42)
