fn fail_condition() -> Bool {
  panic as "condition"
}

pub fn main() {
  assert fail_condition() as "checked"
  1
}

// geam:expect-error Panic(panic, "condition")
