fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let variable = case add_one {
    f -> f(41)
  }

  let alias = case add_one {
    f as alias -> alias(41)
  }

  let discard = case add_one {
    _ -> 42
  }

  #(variable, alias, discard)
}

// geam:expect Tuple([Int(42), Int(42), Int(42)])
