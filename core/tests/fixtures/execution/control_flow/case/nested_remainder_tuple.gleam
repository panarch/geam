pub fn main() {
  let assert #(7, True) = select(Ok(#(#(7, True), [])), #(0, False))
}

pub fn select(
  input: Result(#(#(Int, Bool), List(String)), String),
  fallback: #(Int, Bool),
) -> #(Int, Bool) {
  case input {
    Error(_) -> fallback
    Ok(#(_, [_, ..])) -> fallback
    Ok(#(value, [])) -> value
  }
}
// @geam:expect Tuple([Int(7), Bool(true)])
