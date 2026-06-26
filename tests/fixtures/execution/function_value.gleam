fn add_one(value: Int) {
  value + 1
}

fn add_ten(value: Int) {
  value + 10
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
  let add = 1
  let add = add_one
  let string = string_identity
  let bool = bool_identity
  let nil = nil_identity

  string("geam")
  bool(True)
  nil(Nil)

  let inner = {
    let add = add_ten
    add(10)
  }

  let int_shadow = {
    let add = add_one
    let add = 5
    add + 2
  }

  inner + int_shadow + add(1)
}

// geam:expect Int(29)
