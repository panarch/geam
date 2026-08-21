fn int_identity(value: Int) {
  value
}

fn int_increment(value: Int) {
  value + 1
}

fn string_identity(value: String) {
  value
}

fn string_suffix(value: String) {
  value <> "!"
}

fn bool_identity(value: Bool) {
  value
}

fn bool_not(value: Bool) {
  !value
}

fn nil_identity(value: Nil) {
  value
}

fn nil_other(_: Nil) {
  Nil
}

fn score_string(value: String) {
  case value {
    "a" -> 3
    "a!" -> 4
    _ -> 0
  }
}

fn score_bool(value: Bool) {
  case value {
    True -> 6
    False -> 7
  }
}

fn score_nil(_: Nil) {
  8
}

fn choose_int(key: String) {
  let f = case key {
    "hit" -> int_identity
    _ -> int_increment
  }
  f(10)
}

fn choose_string(key: String) {
  let f = case key {
    "hit" -> string_identity
    _ -> string_suffix
  }
  f("a")
}

fn choose_bool(key: String) {
  let f = case key {
    "hit" -> bool_identity
    _ -> bool_not
  }
  f(True)
}

fn choose_nil(key: String) {
  let f = case key {
    "hit" -> nil_identity
    _ -> nil_other
  }
  f(Nil)
}

fn get_identity(_: String) {
  int_identity
}

fn get_increment(_: String) {
  int_increment
}

fn choose_getter(key: String) {
  let getter = case key {
    "hit" -> get_identity
    _ -> get_increment
  }
  getter("ignored")(10)
}

pub fn main() {
  choose_int("hit") + choose_int("miss") + score_string(choose_string("hit"))
  + score_string(choose_string("miss")) + score_bool(choose_bool("hit"))
  + score_bool(choose_bool("miss")) + score_nil(choose_nil("hit")) + score_nil(choose_nil("miss"))
  + choose_getter("hit") + choose_getter("miss")
}

// @geam:expect Int(78)
