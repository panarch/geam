pub fn main() {
  let literal_before_variable = case 2 {
    1 -> 10
    other -> other
  }

  let variable_before_literal = case 1 {
    other -> other
    1 -> 999
  }

  literal_before_variable + variable_before_literal
}

// geam:expect Int(3)
