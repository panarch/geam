pub type Choice {
  Value(Int)
  Empty
}

pub type Text {
  Text(String)
}

pub fn main() {
  let assert Value(value) = Value(3)
  let assert [Value(first), ..rest] = [Value(1), Value(2)]
  let assert Text("pre" as prefix <> suffix) = Text("prefix") as "expected prefix"
  assert prefix == "pre"
  assert suffix == "fix"

  case rest {
    [Value(second)] -> value + first + second
    _ -> 0
  }
}

// @geam:expect Int(6)
