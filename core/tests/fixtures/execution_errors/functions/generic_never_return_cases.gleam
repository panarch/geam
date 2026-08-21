fn first(value: Int, _other: other) {
  value
}

fn static_never_expression() {
  first(
    1,
    case True {
      True -> panic as "static generic bool case failed"
      False -> panic as "unselected static generic bool case"
    },
  )
}

fn nested_fail(selector: Bool, float_selector: Float, string_selector: String) -> value {
  case selector {
    True -> {
      let _ = Nil
      case float_selector {
        1.0 -> case string_selector {
          "selected" -> panic as "generic return case failed"
          _ -> panic as "unselected generic string return case"
        }
        _ -> panic as "unselected generic float return case"
      }
    }
    False -> {
      let _ = argument_fail()
      panic as "unselected generic bool return case"
    }
  }
}

fn argument_fail() -> value {
  panic as "generic tail argument failed"
}

fn tail_target(_value: Int) -> value {
  panic as "generic tail target should not run"
}

fn tail_argument_fail() -> value {
  tail_target(argument_fail())
}

fn choose(selector: Int) -> value {
  case selector {
    0 -> nested_fail(True, 1.0, "selected")
    _ -> tail_argument_fail()
  }
}

fn concrete_return_with_diverging_block(selector: Bool) -> Int {
  case selector {
    True -> {
      argument_fail()
      1
    }
    False -> 0
  }
}

pub fn main() {
  let _ = concrete_return_with_diverging_block(False)
  choose(0)
}

// @geam:expect-error
// geam::panic
//
//   x panic: generic return case failed
//     ,-[tests/fixtures/execution_errors/functions/generic_never_return_cases.gleam:21:25]
//  20 |         1.0 -> case string_selector {
//  21 |           "selected" -> panic as "generic return case failed"
//     :                         ^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^
//     :                                           `-- panic in main.nested_fail
//  22 |           _ -> panic as "unselected generic string return case"
//     `----
