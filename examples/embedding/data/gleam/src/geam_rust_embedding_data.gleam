pub fn review(
  rows: List(#(String, Int)),
) -> List(Result(#(String, Int), String)) {
  case rows {
    [] -> []
    [#(code, quantity), ..rest] -> [
      case quantity < 0 {
        True -> Error("quantity must not be negative")
        False -> Ok(#(code, quantity))
      },
      ..review(rest)
    ]
  }
}

pub fn total(rows: List(Result(#(String, Int), String))) -> Int {
  case rows {
    [] -> 0
    [Ok(#(_, quantity)), ..rest] -> quantity + total(rest)
    [Error(_), ..rest] -> total(rest)
  }
}
