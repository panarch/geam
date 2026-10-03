pub type Request {
  Request(enabled: Bool)
}

pub fn main() {
  let assert True = enabled(#(Request(True), []))
  let assert False = enabled(#(Request(False), []))
  let assert False = enabled(#(Request(True), ["tail"]))
}

pub fn enabled(input: #(Request, List(String))) -> Bool {
  case input {
    #(_, [_, ..]) -> False
    #(request, []) -> request.enabled
  }
}
// @geam:expect Bool(false)
