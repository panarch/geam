pub type Boxed(value) {
  Boxed(value: value, label: String)
}

pub fn map(boxed: Boxed(a), apply: fn(a) -> b) -> Boxed(b) {
  case boxed {
    Boxed(value, label) -> Boxed(apply(value), label)
  }
}
