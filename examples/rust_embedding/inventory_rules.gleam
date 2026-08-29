fn decorate(prefix: String, value: String) {
  prefix <> value
}

pub fn label(prefix: String, value: String) {
  decorate(prefix, value)
}

pub fn double(value: Int) {
  value * 2
}

pub fn choose(enabled: Bool, left: Float, right: Float) {
  case enabled {
    True -> left
    False -> right
  }
}
