pub fn main() {
  let assert 7 = select(Ok(#(7, [])), 0)
}

pub fn select(
  input: Result(#(Int, List(String)), String),
  fallback: Int,
) -> Int {
  case input {
    Error(_) -> fallback
    Ok(#(_, [_, ..])) -> fallback
    Ok(#(value, [])) -> value
  }
}
// @geam:expect Int(7)
