pub fn main() {
  let assert True = enabled(Ok(#(True, [])))
  let assert False = enabled(Ok(#(False, [])))
  let assert False = enabled(Ok(#(True, ["tail"])))
  let assert False = enabled(Error("invalid"))
}

pub fn enabled(input: Result(#(Bool, List(String)), String)) -> Bool {
  case input {
    Error(_) -> False
    Ok(#(_, [_, ..])) -> False
    Ok(#(value, [])) -> value
  }
}
// @geam:expect Bool(false)
