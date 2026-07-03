pub type Values(value) =
  List(value)

fn identity(values: Values(Int)) -> Values(Int) {
  values
}

pub fn main() {
  identity([1])
}

// geam:expect List(Int)([Int(1)])
