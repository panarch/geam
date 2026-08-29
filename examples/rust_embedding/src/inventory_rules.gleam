import inventory_format

pub fn label(prefix: String, value: String) {
  inventory_format.decorate(prefix, value)
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
