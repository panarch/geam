pub fn main() {
  after_main(41)
}

fn after_main(value: Int) {
  value + 1
}

pub fn after_main_unused() {
  1
}

// @geam:expect Int(42)
