pub type Boxed(value) {
  Boxed(value)
}

fn identity(value) {
  value
}

fn boxed(value) {
  Boxed(value)
}

fn listed(value) {
  [value]
}

fn nested_function(value) {
  fn(_input) { value }
}

fn int_function(_input) {
  1
}

fn retain_generic(function: fn(input) -> output, count: Int) {
  case count {
    0 -> function
    _ -> retain_generic(function, count - 1)
  }
}

fn retain_custom(
  function: fn(input) -> Boxed(output),
  count: Int,
) -> fn(input) -> Boxed(output) {
  case count {
    0 -> function
    _ -> retain_custom(function, count - 1)
  }
}

fn retain_list(
  function: fn(input) -> List(output),
  count: Int,
) -> fn(input) -> List(output) {
  case count {
    0 -> function
    _ -> retain_list(function, count - 1)
  }
}

fn retain_function(
  function: fn(input) -> fn(argument) -> output,
  count: Int,
) -> fn(input) -> fn(argument) -> output {
  case count {
    0 -> function
    _ -> retain_function(function, count - 1)
  }
}

fn retain_int_function(
  function: fn(input) -> Int,
  count: Int,
) -> fn(input) -> Int {
  case count {
    0 -> function
    _ -> retain_int_function(function, count - 1)
  }
}

pub fn main() {
  let generic = retain_generic(identity, 1)
  let custom = retain_custom(boxed, 1)
  let list = retain_list(listed, 1)
  let function = retain_function(nested_function, 1)
  let int = retain_int_function(int_function, 1)
  #(
    generic == generic,
    custom == custom,
    list == list,
    function == function,
    int == int,
  )
}

// geam:expect Tuple([Bool(true), Bool(true), Bool(true), Bool(true), Bool(true)])
