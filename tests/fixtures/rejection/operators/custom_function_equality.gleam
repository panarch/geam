pub type Holder {
  Holder(fn(Int) -> Int)
}

fn identity(value: Int) {
  value
}

pub fn main() {
  Holder(identity) == Holder(identity)
}

// geam:reject unsupported binary operator: equality on function values
