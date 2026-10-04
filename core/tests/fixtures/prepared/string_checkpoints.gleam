pub fn after_step(text: String, total: Int) -> Int {
  let incremented = total + 1
  let assert "λ" <> _ = text as "lambda required"
  incremented
}

pub fn stop(message: String) -> Int {
  panic as message
}

pub fn list_stop(message: String) -> List(Int) {
  panic as message
}
