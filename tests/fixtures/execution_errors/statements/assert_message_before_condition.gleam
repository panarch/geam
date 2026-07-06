fn fail_message() -> String {
  panic as "message"
}

pub fn main() {
  assert True as fail_message()
  1
}

// geam:expect-error Panic(panic, "message")
