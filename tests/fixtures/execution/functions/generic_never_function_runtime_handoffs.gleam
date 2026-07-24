fn fail(_value: Int) -> value {
  panic as "diverging function must not run"
}

fn provide() -> fn(Int) -> value {
  fail
}

fn identity(function: fn(Int) -> value) {
  function
}

pub fn main() {
  let local = provide()
  let via_argument = identity(fail)
  let provider = provide
  let via_function_value = provider()

  #(
    local == fail,
    via_argument == fail,
    via_function_value == fail,
  )
}

// @geam:expect Tuple([Bool(true), Bool(true), Bool(true)])
