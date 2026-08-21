fn increment(value: Int) {
  value + 1
}

fn emitter(message: String) {
  fn(value) {
    value |> echo as message
  }
}

pub fn main() {
  let captured = emitter("captured")

  {
    1
    |> echo as "first"
    |> increment
    |> captured
    |> echo
  }
}

// @geam:echo
// tests/fixtures/execution/pipeline/echo.gleam:16 first
// 1
// @geam:echo
// tests/fixtures/execution/pipeline/echo.gleam:7 captured
// 2
// @geam:echo
// tests/fixtures/execution/pipeline/echo.gleam:19
// 2
// @geam:expect Int(2)
