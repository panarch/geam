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
  case True {
    True -> string_identity
    False -> string_identity
  }("geam")
  case False {
    True -> string_identity
    False -> string_identity
  }("geam")
  case True {
    True -> bool_identity
    False -> bool_identity
  }(True)
  case False {
    True -> bool_identity
    False -> bool_identity
  }(True)
  case True {
    True -> nil_identity
    False -> nil_identity
  }(Nil)
  case False {
    True -> nil_identity
    False -> nil_identity
  }(Nil)

  case 0 {
    0 -> string_identity
    _ -> string_identity
  }("geam")
  case 1 {
    0 -> string_identity
    _ -> string_identity
  }("geam")
  case 0 {
    0 -> bool_identity
    _ -> bool_identity
  }(True)
  case 1 {
    0 -> bool_identity
    _ -> bool_identity
  }(True)
  case 0 {
    0 -> nil_identity
    _ -> nil_identity
  }(Nil)
  case 1 {
    0 -> nil_identity
    _ -> nil_identity
  }(Nil)

  let bool_hit = case True {
    True -> add_one
    False -> add_ten
  }(1)
  let bool_miss = case False {
    True -> add_one
    False -> add_ten
  }(1)
  let int_hit = case 0 {
    0 -> add_ten
    _ -> add_one
  }(1)
  let int_fallback = case 1 {
    0 -> add_ten
    _ -> add_one
  }(1)

  bool_hit + bool_miss + int_hit + int_fallback
}

// @geam:expect Int(26)
