pub type Optional(value) {
  Empty
  Full(value)
}

fn ignore(_value: value) {
  1
}

fn value_or_zero(value: Optional(item)) {
  case value {
    Full(inner) -> ignore(inner)
    Empty -> 0
  }
}

fn value_or_zero_local(value: Optional(item)) {
  let result = case value {
    Full(inner) -> ignore(inner)
    Empty -> 0
  }
  result
}

pub fn main() {
  #(value_or_zero(Empty), value_or_zero_local(Empty))
}

// geam:expect Tuple([Int(0), Int(0)])
