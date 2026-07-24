pub fn main() {
  let from_constructor = case Nil {
    Nil -> 1
  }
  let from_variable = case Nil {
    value -> value
  }
  let from_alias = case Nil {
    _ as alias -> alias
  }
  let from_constructor_alias = case Nil {
    Nil as alias -> alias
  }

  #(from_constructor, from_variable, from_alias, from_constructor_alias)
}

// @geam:expect Tuple([Int(1), Nil, Nil, Nil])
