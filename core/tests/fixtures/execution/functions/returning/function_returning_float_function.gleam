fn add_half(value: Float) {
  value +. 0.5
}

fn get() {
  add_half
}

pub fn main() {
  get()(1.5)
}

// @geam:expect Float(2.0)
