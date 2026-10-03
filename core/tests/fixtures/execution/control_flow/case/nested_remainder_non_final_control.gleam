pub fn main() {
  let assert True = enabled(Ok(#(True, [])))
  let assert False = enabled(Ok(#(False, [])))
  let assert False = enabled(Ok(#(True, ["tail"])))
  let assert False = enabled(Error("invalid"))
}

pub fn enabled(input: Result(#(Bool, List(String)), String)) -> Bool {
  case input {
    Ok(#(_, [_, ..])) -> False
    Ok(#(value, [])) -> value
    Error(_) -> False
  }
}
// @geam:expect Bool(false)
