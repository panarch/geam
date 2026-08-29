pub fn render(prefix: String, value: String) -> String {
  prefix <> value
}

pub fn choose(preferred: Bool, primary: Float, fallback: Float) -> Float {
  case preferred {
    True -> primary
    False -> fallback
  }
}
