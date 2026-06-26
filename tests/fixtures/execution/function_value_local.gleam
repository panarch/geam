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
  let add = add_one
  let string = "geam"
  let string = string_identity
  let bool = False
  let bool = bool_identity
  let nil = Nil
  let nil = nil_identity

  string("geam")
  bool(True)
  nil(Nil)

  add(41)
}

// geam:expect Int(42)
