pub fn main() {
  let assert "keep" = select(Ok(#("keep", [])), "fallback")
}

pub fn select(
  input: Result(#(String, List(String)), String),
  fallback: String,
) -> String {
  case input {
    Error(_) -> fallback
    Ok(#(_, [_, ..])) -> fallback
    Ok(#(value, [])) -> value
  }
}
// @geam:expect String("keep")
