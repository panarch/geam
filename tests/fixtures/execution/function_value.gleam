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

  { string_identity }("geam")
  { bool_identity }(True)
  { nil_identity }(Nil)

  case True {
    True -> string_identity
    False -> string_identity
  }("geam")
  case True {
    True -> bool_identity
    False -> bool_identity
  }(True)
  case True {
    True -> nil_identity
    False -> nil_identity
  }(Nil)

  case 0 {
    0 -> string_identity
    _ -> string_identity
  }("geam")
  case 0 {
    0 -> bool_identity
    _ -> bool_identity
  }(True)
  case 0 {
    0 -> nil_identity
    _ -> nil_identity
  }(Nil)

  let inner = {
    let add = add_ten
    add(10)
  }

  let int_shadow = {
    let add = add_one
    let add = 5
    add + 2
  }

  let block_call = { add_one }(1)
  let bool_case_call = case True {
    True -> add_one
    False -> add_ten
  }(1)
  let int_case_call = case 0 {
    0 -> add_ten
    _ -> add_one
  }(1)

  inner + int_shadow + add(1) + block_call + bool_case_call + int_case_call
}

// geam:expect Int(44)
