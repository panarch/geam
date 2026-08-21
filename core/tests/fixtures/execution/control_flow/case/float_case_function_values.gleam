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

fn choose_int(key: Float) {
  let f = case key {
    1.0 -> int_identity
    _ -> int_increment
  }
  f(10)
}

fn choose_string(key: Float) {
  let f = case key {
    1.0 -> string_identity
    _ -> string_suffix
  }
  f("a")
}

fn choose_bool(key: Float) {
  let f = case key {
    1.0 -> bool_identity
    _ -> bool_not
  }
  f(True)
}

fn choose_nil(key: Float) {
  let f = case key {
    1.0 -> nil_identity
    _ -> nil_other
  }
  f(Nil)
}

fn get_identity(_: Float) {
  int_identity
}

fn get_increment(_: Float) {
  int_increment
}

fn choose_getter(key: Float) {
  let getter = case key {
    1.0 -> get_identity
    _ -> get_increment
  }
  getter(0.0)(10)
}

pub fn main() {
  choose_int(1.0) + choose_int(2.0) + score_string(choose_string(1.0))
  + score_string(choose_string(2.0)) + score_bool(choose_bool(1.0))
  + score_bool(choose_bool(2.0)) + score_nil(choose_nil(1.0))
  + score_nil(choose_nil(2.0)) + choose_getter(1.0) + choose_getter(2.0)
}

// @geam:expect Int(78)
