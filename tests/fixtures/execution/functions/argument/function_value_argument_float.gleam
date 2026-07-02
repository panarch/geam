fn add_half(value: Float) {
  value +. 0.5
}

fn apply(function: fn(Float) -> Float, value: Float) {
  function(value)
}

pub fn main() {
  apply(add_half, 1.5)
}

// geam:expect Float(2.0)
