pub type Boxed(value) {
  Boxed(value)
}

fn boxed(value: value) -> List(Boxed(value)) {
  [Boxed(value)]
}

pub fn main() {
  boxed(1)
}

// geam:expect List(geam/main/Boxed(Int))([Custom(type=geam/main/Boxed(Int), constructor=Boxed#0, fields=[Int(1)])])
