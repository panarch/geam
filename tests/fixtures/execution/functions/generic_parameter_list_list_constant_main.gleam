const empty = []
const empty_alias = empty

fn nested() -> List(List(value)) {
  empty
}

fn generic_empty() {
  empty
}

fn specialized_nested() -> List(List(value)) {
  generic_empty()
}

fn alias_nested() -> List(List(value)) {
  empty_alias
}

fn triple_nested() -> List(List(List(value))) {
  empty_alias
}

pub fn main() {
  let result = #(nested(), specialized_nested(), alias_nested(), triple_nested())
  let _ = Nil
  result
}

// geam:expect Tuple([List(List(Parameter(0)))([]), List(List(Parameter(1)))([]), List(List(Parameter(2)))([]), List(List(List(Parameter(3))))([])])
