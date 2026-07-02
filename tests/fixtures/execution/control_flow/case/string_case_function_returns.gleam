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
  case key {
    "hit" -> int_identity
    _ -> int_increment
  }
}

fn choose_string(key: String) {
  case key {
    "hit" -> string_identity
    _ -> string_suffix
  }
}

fn choose_bool(key: String) {
  case key {
    "hit" -> bool_identity
    _ -> bool_not
  }
}

fn choose_nil(key: String) {
  case key {
    "hit" -> nil_identity
    _ -> nil_other
  }
}

fn get_identity(_: String) {
  int_identity
}

fn get_increment(_: String) {
  int_increment
}

fn choose_getter(key: String) {
  case key {
    "hit" -> get_identity
    _ -> get_increment
  }
}

pub fn main() {
  choose_int("hit")(10) + choose_int("miss")(10) + score_string(choose_string("hit")("a"))
  + score_string(choose_string("miss")("a")) + score_bool(choose_bool("hit")(True))
  + score_bool(choose_bool("miss")(True)) + score_nil(choose_nil("hit")(Nil))
  + score_nil(choose_nil("miss")(Nil)) + choose_getter("hit")("ignored")(10)
  + choose_getter("miss")("ignored")(10)
}

// geam:expect Int(78)
