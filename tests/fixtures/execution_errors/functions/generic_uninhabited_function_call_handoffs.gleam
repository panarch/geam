pub type Boxed(value) {
  Boxed(value)
}

fn fail() -> value {
  panic as "uninhabited function argument failed"
}

fn make_tuple(_trigger: #(value)) -> fn() -> #(value) {
  fn() { panic as "tuple function must not run" }
}

fn make_custom(_trigger: Boxed(value)) -> fn() -> Boxed(value) {
  fn() { panic as "custom function must not run" }
}

fn make_generic(_trigger) -> fn(Int) -> value {
  fn(_value) { panic as "generic function must not run" }
}

fn call_tuple() {
  let make = make_tuple
  make(#(fail()))
}

fn call_custom() {
  fn(_prefix: Int, _trigger: Boxed(value)) {
    fn() { panic as "custom function must not run" }
  }(1, Boxed(fail()))
}

fn call_generic() {
  let make = make_generic
  make(fail())
}

fn custom_from_list(flag: Bool) -> fn() -> Boxed(value) {
  case flag {
    True -> {
      let assert [function] = [fn() { panic as "list function must not run" }]
      function
    }
    False -> fn() { panic as "fallback function must not run" }
  }
}

fn custom_from_function_call(flag: Bool) -> fn() -> Boxed(value) {
  let make = make_custom
  case flag {
    True -> make(Boxed(fail()))
    False -> fn() { panic as "fallback function must not run" }
  }
}

fn selected_path() {
  0
}

pub fn main() {
  case selected_path() {
    0 -> call_custom() == call_custom()
    1 -> call_tuple() == call_tuple()
    2 -> call_generic() == call_generic()
    3 -> custom_from_list(True) == custom_from_list(True)
    _ -> custom_from_function_call(True) == custom_from_function_call(True)
  }
}

// geam:expect-error
// geam::panic
//
//   x panic: uninhabited function argument failed
//    ,-[tests/fixtures/execution_errors/functions/generic_uninhabited_function_call_handoffs.gleam:6:3]
//  5 | fn fail() -> value {
//  6 |   panic as "uninhabited function argument failed"
//    :   ^^^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^^^^
//    :                          `-- panic in main.fail
//  7 | }
//    `----
