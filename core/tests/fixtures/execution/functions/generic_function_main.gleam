fn identity(value: value) {
  value
}

const generic_function = identity
const generic_function_alias = generic_function

pub fn main() {
  #(
    generic_function_alias,
    #(case 1.0 {
      1.0 -> generic_function_alias
      _ -> generic_function_alias
    }).0,
    #(case 0.0 {
      1.0 -> generic_function_alias
      _ -> generic_function_alias
    }).0,
  )
}

// @geam:expect Tuple([Function(fn(Parameter(0)) -> Parameter(0)), Function(fn(Parameter(1)) -> Parameter(1)), Function(fn(Parameter(2)) -> Parameter(2))])
