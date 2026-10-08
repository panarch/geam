pub fn verify(flag: Bool) -> Bool {
  let assert True = accepted(flag)
  accepted(flag)
}

fn accepted(flag: Bool) -> Bool {
  let value = "ok"
  flag && value == "ok"
}
