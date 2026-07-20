const empty = []
const nested = [empty]
const deeply_nested = [nested]
const deeply_nested_tail = [nested]
const deeply_nested_spread = [nested, ..deeply_nested_tail]
const inline_deeply_nested = [[[]]]
const boxed_empty = #(empty)

fn int_nested() -> List(List(Int)) {
  nested
}

fn int_deeply_nested() -> List(List(List(Int))) {
  deeply_nested
}

fn int_inline_deeply_nested() -> List(List(List(Int))) {
  inline_deeply_nested
}

fn parameter_boxed_empty() -> #(List(List(value))) {
  boxed_empty
}

fn parameter_nested() -> List(List(List(value))) {
  nested
}

pub fn main() {
  #(
    nested,
    int_nested(),
    int_deeply_nested(),
    int_inline_deeply_nested(),
    deeply_nested,
    deeply_nested_spread,
    parameter_boxed_empty(),
    parameter_nested(),
  )
}

// geam:expect Tuple([List(List(Parameter(0)))([List(Parameter(0))([])]), List(List(Int))([List(Int)([])]), List(List(List(Int)))([List(List(Int))([List(Int)([])])]), List(List(List(Int)))([List(List(Int))([List(Int)([])])]), List(List(List(Parameter(1))))([List(List(Parameter(1)))([List(Parameter(1))([])])]), List(List(List(Parameter(2))))([List(List(Parameter(2)))([List(Parameter(2))([])]), List(List(Parameter(2)))([List(Parameter(2))([])])]), Tuple([List(List(Parameter(3)))([])]), List(List(List(Parameter(4))))([List(List(Parameter(4)))([])])])
