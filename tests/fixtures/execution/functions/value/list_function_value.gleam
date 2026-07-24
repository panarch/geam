fn values() {
  [1, 2, 3]
}

fn run(function: fn() -> List(Int)) {
  function()
}

pub fn main() {
  let f = values
  run(f)
}

// @geam:expect List(Int)([Int(1), Int(2), Int(3)])
