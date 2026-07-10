pub fn main() {
  assert case 1.0 { 1.0 -> True _ -> False }
  assert case 0.0 { 1.0 -> False _ -> True }

  assert case 1 { 1 -> 1.0 _ -> 0.0 } == 1.0
  assert case 0 { 1 -> 0.0 _ -> 2.0 } == 2.0
  assert case "hit" { "hit" -> 1.0 _ -> 0.0 } == 1.0
  assert case "miss" { "hit" -> 0.0 _ -> 2.0 } == 2.0
  assert case 1.0 { 1.0 -> 1.0 _ -> 0.0 } == 1.0
  assert case 0.0 { 1.0 -> 0.0 _ -> 2.0 } == 2.0
  assert case False { True -> 0.0 False -> 2.0 } == 2.0

  assert case 1 { 1 -> 1 _ -> 0 } == 1
  assert case 1.0 { 1.0 -> 1 _ -> 0 } == 1
  assert case 0.0 { 1.0 -> 0 _ -> 2 } == 2

  assert case 1 { 1 -> "one" _ -> "zero" } == "one"
  assert case 0 { 1 -> "zero" _ -> "two" } == "two"
  assert case 1.0 { 1.0 -> "one" _ -> "zero" } == "one"
  assert case 0.0 { 1.0 -> "zero" _ -> "two" } == "two"
  assert case False { True -> "zero" False -> "two" } == "two"

  Nil
}

// geam:expect Nil
