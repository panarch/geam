pub fn verify() -> Bool {
  case accepted() {
    True -> accepted()
    False -> False
  }
}

fn accepted() -> Bool {
  let value = "ok"
  value == "ok"
}
