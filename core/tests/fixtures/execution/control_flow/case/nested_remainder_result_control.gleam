pub type Request {
  Request(enabled: Bool)
}

pub fn main() {
  let assert True = enabled(Ok(Request(True)))
  let assert False = enabled(Ok(Request(False)))
  let assert False = enabled(Error("invalid"))
}

pub fn enabled(input: Result(Request, String)) -> Bool {
  case input {
    Error(_) -> False
    Ok(request) -> request.enabled
  }
}
// @geam:expect Bool(false)
