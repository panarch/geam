fn add_one(value: Int) {
  value + 1
}

fn string_identity(value: String) {
  value
}

fn bool_identity(value: Bool) {
  value
}

fn nil_identity(value: Nil) {
  value
}

pub fn main() {
  { string_identity }("geam")
  { bool_identity }(True)
  { nil_identity }(Nil)

  { add_one }(41)
}

// @geam:expect Int(42)
