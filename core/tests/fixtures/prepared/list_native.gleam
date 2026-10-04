@external(erlang, "list_provider", "hold")
fn hold(values: List(Int), fail: Bool) -> List(Int)

pub fn native_tail(value: Int, tail: List(Int), fail: Bool) -> List(Int) {
  let values = [value, ..tail]
  case fail {
    True -> hold(values, True)
    False -> hold(values, False)
  }
}

pub fn caller(
  value: Int,
  tail: List(Int),
  text: String,
  fail: Bool,
) -> #(String, Int, List(Int), List(Int)) {
  let result = native_tail(value, tail, fail)
  #(text, value, tail, result)
}
