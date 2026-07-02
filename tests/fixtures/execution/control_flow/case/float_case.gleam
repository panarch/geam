fn score(value: Float) {
  case value {
    1.5 -> 10
    2.0 -> 20
    _ -> 30
  }
}

fn add_half(value: Float) {
  value +. 0.5
}

pub fn main() {
  let int_expr = case 1.0 {
    1.0 -> 1
    _ -> 0
  }
  let string_expr = case 1.0 {
    1.0 -> "hit"
    _ -> "miss"
  }
  let bool_expr = case 1.0 {
    1.0 -> True
    _ -> False
  }
  let nil_expr = case 1.0 {
    1.0 -> Nil
    _ -> Nil
  }
  let float_expr = case 1.0 {
    1.0 -> 1.0
    _ -> 0.0
  }
  let fn_expr = case 1.0 {
    1.0 -> add_half
    _ -> add_half
  }

  let hit = score(1.5)
  let miss = score(9.0)
  let fallback_first = case 2.0 {
    _ -> 5.5
    2.0 -> 9.0
  }
  let duplicate = case 1.0 {
    1.0 -> 1.0
    1.0 -> 2.0
    _ -> 0.0
  }
  let add = case 1.0 {
    1.0 -> add_half
    _ -> add_half
  }
  nil_expr
  let string_score = case string_expr {
    "hit" -> 1.0
    _ -> 0.0
  }
  let bool_score = case bool_expr {
    True -> 1.0
    False -> 0.0
  }

  case hit + miss + int_expr {
    41 -> add(fallback_first +. duplicate) +. string_score +. bool_score +. float_expr +. fn_expr(1.0)
    _ -> 0.0
  }
}

// geam:expect Float(11.5)
