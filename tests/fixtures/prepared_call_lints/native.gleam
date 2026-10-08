@external(erlang, "native", "accepted")
fn accepted(flag: Bool) -> Bool

pub fn verify(flag: Bool) -> Bool {
  let assert True = accepted(flag)
  accepted(flag)
}
