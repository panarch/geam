pub type Boxed(value) {
  Boxed(value)
}

fn wrap(value: value) -> Boxed(value) {
  Boxed(value)
}

fn unwrap(value: Boxed(value)) -> value {
  case value {
    Boxed(value) -> value
  }
}

pub fn main() {
  #(unwrap(wrap(1)), unwrap(wrap("two")))
}

// geam:expect Tuple([Int(1), String("two")])
