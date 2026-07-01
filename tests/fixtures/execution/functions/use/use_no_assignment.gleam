fn run(continue: fn() -> Int) {
  continue()
}

pub fn main() {
  use <- run
  42
}

// geam:expect Int(42)
