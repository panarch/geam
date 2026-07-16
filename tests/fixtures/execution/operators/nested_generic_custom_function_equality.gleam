pub type Boxed(value) {
  Boxed(value)
}

pub type Wrapper(value) {
  Wrapper(Boxed(value))
}

fn value() {
  1
}

pub fn main() {
  Wrapper(Boxed(value)) == Wrapper(Boxed(value))
}

// geam:expect Bool(true)
