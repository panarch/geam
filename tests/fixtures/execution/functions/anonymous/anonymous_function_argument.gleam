fn apply(function: fn(Int) -> Int, value: Int) {
  function(value)
}

pub fn main() {
  apply(fn(value) { value + 1 }, 41)
}

// geam:expect Int(42)
