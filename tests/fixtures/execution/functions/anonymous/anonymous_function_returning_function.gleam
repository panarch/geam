fn apply(function: fn(Int) -> Int, value: Int) {
  function(value)
}

pub fn main() {
  let getter = fn() { fn(value) { value + 1 } }
  let add_one = getter()

  apply(add_one, 41)
}

// @geam:expect Int(42)
