fn same(values: List(Int)) {
  values
}

pub fn main() {
  [1, 2, 3]
  |> same
}

// geam:expect List(Int)([Int(1), Int(2), Int(3)])
