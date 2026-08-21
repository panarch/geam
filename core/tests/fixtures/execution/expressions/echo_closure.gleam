fn captured(value: Int, message: String) {
  fn() {
    echo value as message
  }
}

pub fn main() {
  captured(1, "captured")()
}

// @geam:echo
// tests/fixtures/execution/expressions/echo_closure.gleam:3 captured
// 1
// @geam:expect Int(1)
