fn score_string(value: String) {
  case value {
    "a" -> 3
    "b" -> 5
    _ -> 0
  }
}

fn score_bool(value: Bool) {
  case value {
    True -> 7
    False -> 11
  }
}

fn score_nil(_: Nil) {
  13
}

pub fn main() {
  let int_hit = case "hit" {
    "hit" -> 1
    _ -> 2
  }
  let int_miss = case "miss" {
    "hit" -> 1
    _ -> 2
  }
  let string_hit = score_string(case "hit" {
    "hit" -> "a"
    _ -> "b"
  })
  let string_miss = score_string(case "miss" {
    "hit" -> "a"
    _ -> "b"
  })
  let bool_hit = score_bool(case "hit" {
    "hit" -> True
    _ -> False
  })
  let bool_miss = score_bool(case "miss" {
    "hit" -> True
    _ -> False
  })
  let nil_hit = score_nil(case "hit" {
    "hit" -> Nil
    _ -> Nil
  })
  let nil_miss = score_nil(case "miss" {
    "hit" -> Nil
    _ -> Nil
  })

  int_hit + int_miss + string_hit + string_miss + bool_hit + bool_miss + nil_hit + nil_miss
}

// @geam:expect Int(55)
