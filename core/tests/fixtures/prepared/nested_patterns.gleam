pub type Request {
  Request(enabled: Bool)
}

fn enabled(input: Result(#(Bool, List(String)), String)) -> Bool {
  case input {
    Error(_) -> False
    Ok(#(_, [_, ..])) -> False
    Ok(#(value, [])) -> value
  }
}

fn enabled_request(input: Result(#(Request, List(String)), String)) -> Bool {
  case input {
    Error(_) -> False
    Ok(#(_, [_, ..])) -> False
    Ok(#(request, [])) -> request.enabled
  }
}

fn nonempty(input: Result(List(Int), String)) -> #(Int, List(Int)) {
  case input {
    Error(_) -> #(0, [])
    Ok([]) -> #(0, [])
    Ok([head, ..tail]) -> #(head, tail)
  }
}

type Option(a) {
  Some(a)
  None
}

fn inspect(value: Result(Option(Int), String)) -> String {
  case value {
    Ok(Some(_)) -> "present:"
    Ok(None) -> "missing:"
    Error(reason) -> reason
  }
}

fn nested(value: Result(#(Option(Option(Int)), Int), String)) -> String {
  case value {
    Ok(#(Some(Some(_)), _)) -> "nested:"
    Ok(#(Some(None), _)) -> "empty:"
    Ok(#(None, _)) -> "none:"
    Error(reason) -> reason
  }
}

fn guarded_tail(values: List(a), take_tail: Bool) -> List(a) {
  case values {
    [head, ..tail] if take_tail -> tail
    _ -> values
  }
}

fn two_heads(input: Result(List(Int), String)) -> #(Int, Int, List(Int)) {
  case input {
    Error(_) -> #(0, 0, [])
    Ok([]) -> #(0, 0, [])
    Ok([_]) -> #(0, 0, [])
    Ok([first, second, ..rest]) -> #(first, second, rest)
  }
}

pub type Payload {
  Missing
  Present(Bool, List(Int))
}

fn second(input: Result(#(Bool, List(Int)), String)) -> Int {
  case input {
    Error(_) -> 0
    Ok(#(False, _)) -> 0
    Ok(#(True, [])) -> 0
    Ok(#(True, [_])) -> 0
    Ok(#(True, [_, value, ..])) -> value
  }
}

fn second_custom(input: Result(Payload, String)) -> Int {
  case input {
    Error(_) -> 0
    Ok(Missing) -> 0
    Ok(Present(False, _)) -> 0
    Ok(Present(True, [])) -> 0
    Ok(Present(True, [_])) -> 0
    Ok(Present(True, [_, value, ..])) -> value
  }
}

pub fn main() {
  let assert 0 = second(Error("invalid"))
  let assert 0 = second(Ok(#(False, [1, 7])))
  let assert 0 = second(Ok(#(True, [])))
  let assert 0 = second(Ok(#(True, [1])))
  let assert 7 = second(Ok(#(True, [1, 7])))
  let assert 7 = second(Ok(#(True, [1, 7, 9])))
  let assert 0 = second_custom(Error("invalid"))
  let assert 0 = second_custom(Ok(Missing))
  let assert 0 = second_custom(Ok(Present(False, [1, 7])))
  let assert 0 = second_custom(Ok(Present(True, [])))
  let assert 0 = second_custom(Ok(Present(True, [1])))
  let assert 7 = second_custom(Ok(Present(True, [1, 7])))
  let assert 7 = second_custom(Ok(Present(True, [1, 7, 9])))
  let assert #(3, 5, [7]) = two_heads(Ok([3, 5, 7]))
  let assert #(0, 0, []) = two_heads(Ok([]))
  let assert #(0, 0, []) = two_heads(Ok([3]))
  let assert #(0, 0, []) = two_heads(Error("invalid"))
  let assert True = enabled(Ok(#(True, [])))
  let assert False = enabled(Ok(#(False, [])))
  let assert False = enabled(Ok(#(True, ["tail"])))
  let assert False = enabled(Error("invalid"))
  let assert True = enabled_request(Ok(#(Request(True), [])))
  let assert False = enabled_request(Ok(#(Request(False), [])))
  let assert False = enabled_request(Ok(#(Request(True), ["tail"])))
  let assert False = enabled_request(Error("invalid"))
  let assert #(7, [9]) = nonempty(Ok([7, 9]))
  let assert #(0, []) = nonempty(Ok([]))
  let assert #(0, []) = nonempty(Error("invalid"))
  let assert [42] = guarded_tail([1, 42], True)
  let assert [1, 42] = guarded_tail([1, 42], False)
  let assert [] = guarded_tail([], True)
  inspect(Ok(Some(42)))
  <> inspect(Ok(None))
  <> inspect(Error("failed:"))
  <> nested(Ok(#(Some(Some(42)), 1)))
  <> nested(Ok(#(Some(None), 2)))
  <> nested(Ok(#(None, 3)))
  <> nested(Error("done"))
}
