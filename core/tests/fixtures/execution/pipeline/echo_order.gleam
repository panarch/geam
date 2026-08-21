pub fn main() {
  {
    echo 1 as "value"
  }
  |> echo as {
    echo "message" as "message"
  }
}

// @geam:echo
// tests/fixtures/execution/pipeline/echo_order.gleam:3 value
// 1
// @geam:echo
// tests/fixtures/execution/pipeline/echo_order.gleam:6 message
// "message"
// @geam:echo
// tests/fixtures/execution/pipeline/echo_order.gleam:5 message
// 1
// @geam:expect Int(1)
