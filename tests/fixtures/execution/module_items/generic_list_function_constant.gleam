fn empty(_value: value) -> List(value) {
  []
}

const empty_constant = empty

pub fn main() {
  #(empty_constant, empty_constant(1))
}

// geam:expect Tuple([Function(fn(Parameter(0)) -> List(Parameter(0))), List(Int)([])])
