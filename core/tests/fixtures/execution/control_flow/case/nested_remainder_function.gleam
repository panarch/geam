pub fn main() {
  let selected = select(Ok(#(fn() { 7 }, [])), fn() { 0 })
  let assert 7 = selected()
}

pub fn select(
  input: Result(#(fn() -> Int, List(String)), String),
  fallback: fn() -> Int,
) -> fn() -> Int {
  case input {
    Error(_) -> fallback
    Ok(#(_, [_, ..])) -> fallback
    Ok(#(value, [])) -> value
  }
}
// @geam:expect Int(7)
