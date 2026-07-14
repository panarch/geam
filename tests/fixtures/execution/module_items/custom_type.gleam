pub type Boxed {
  Boxed(Int)
}

type Never

fn stop() -> Never {
  panic
}

pub fn main() {
  1
}

// geam:expect Int(1)
