pub type Request {
  Request(enabled: Bool)
}

pub fn enabled(input: Result(#(Bool, List(String)), String)) -> Bool {
  case input {
    Error(_) -> False
    Ok(#(_, [_, ..])) -> False
    Ok(#(value, [])) -> value
  }
}

pub fn enabled_request(
  input: Result(#(Request, List(String)), String),
) -> Bool {
  case input {
    Error(_) -> False
    Ok(#(_, [_, ..])) -> False
    Ok(#(request, [])) -> request.enabled
  }
}

pub type Slot {
  Vacant
  Present(#(Bool, List(Int)))
}

pub type Choice {
  First(#(Bool, List(Int)))
  Second(#(Bool, List(Int)))
}

fn selected(input: Result(#(a, List(String)), String), otherwise: a) -> a {
  case input {
    Error(_) -> otherwise
    Ok(#(_, [_, ..])) -> otherwise
    Ok(#(value, [])) -> value
  }
}

fn nonempty(input: Result(List(Int), String)) -> #(Int, List(Int)) {
  case input {
    Error(_) -> #(0, [])
    Ok([]) -> #(0, [])
    Ok([head, ..tail]) -> #(head, tail)
  }
}

fn nested(input: Result(Slot, String)) -> Bool {
  case input {
    Error(_) -> False
    Ok(Vacant) -> False
    Ok(Present(#(_, [_, ..]))) -> False
    Ok(Present(#(value, []))) -> value
  }
}

fn aliases(input: Result(#(Bool, List(Int)), String)) -> Bool {
  case input {
    Error(_) -> False
    Ok(#(_, [_, ..])) -> False
    Ok(#(value, [] as rest) as pair) as whole ->
      value && pair.0 && rest == [] && whole == Ok(#(value, rest))
  }
}

fn guarded(input: Result(#(Bool, List(Int)), String), negate: Bool) -> Bool {
  case input {
    Ok(#(value, [])) if negate -> !value
    Error(_) -> False
    Ok(#(_, [_, ..])) -> False
    Ok(#(value, [])) -> value
  }
}

fn alternatives(input: Choice) -> Bool {
  case input {
    First(#(_, [_, ..])) | Second(#(_, [_, ..])) -> False
    First(#(value, [])) | Second(#(value, [])) -> value
  }
}

fn captured(input: Result(#(Bool, List(Int)), String), fallback: Bool) {
  let value = fallback
  case input {
    Error(_) -> fn() { value }
    Ok(#(_, [_, ..])) -> fn() { value }
    Ok(#(value, [])) -> fn() { value }
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
  Available(Bool, List(Int))
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
    Ok(Available(False, _)) -> 0
    Ok(Available(True, [])) -> 0
    Ok(Available(True, [_])) -> 0
    Ok(Available(True, [_, value, ..])) -> value
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
  let assert 0 = second_custom(Ok(Available(False, [1, 7])))
  let assert 0 = second_custom(Ok(Available(True, [])))
  let assert 0 = second_custom(Ok(Available(True, [1])))
  let assert 7 = second_custom(Ok(Available(True, [1, 7])))
  let assert 7 = second_custom(Ok(Available(True, [1, 7, 9])))
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
  let assert 1.5 = selected(Ok(#(1.5, [])), 0.0)
  let assert <<1, 2>> = selected(Ok(#(<<1, 2>>, [])), <<>>)
  let assert [1, 2] = selected(Ok(#([1, 2], [])), [])
  let Nil = selected(Ok(#(Nil, [])), Nil)
  let assert #(7, [9, 11]) = nonempty(Ok([7, 9, 11]))
  let assert #(0, []) = nonempty(Ok([]))
  let assert #(0, []) = nonempty(Error("invalid"))
  let assert True = nested(Ok(Present(#(True, []))))
  let assert False = nested(Ok(Present(#(False, []))))
  let assert False = nested(Ok(Present(#(True, [1]))))
  let assert False = nested(Ok(Vacant))
  let assert False = nested(Error("invalid"))
  let assert True = aliases(Ok(#(True, [])))
  let assert False = aliases(Ok(#(False, [])))
  let assert False = aliases(Ok(#(True, [1])))
  let assert True = guarded(Ok(#(True, [])), False)
  let assert False = guarded(Ok(#(True, [])), True)
  let assert True = guarded(Ok(#(False, [])), True)
  let assert False = guarded(Ok(#(False, [])), False)
  let assert False = guarded(Ok(#(True, [1])), True)
  let assert True = alternatives(First(#(True, [])))
  let assert True = alternatives(Second(#(True, [])))
  let assert False = alternatives(Second(#(False, [])))
  let assert False = alternatives(First(#(True, [1])))
  let yes = captured(Ok(#(True, [])), False)
  let no = captured(Ok(#(False, [])), True)
  let other = captured(Ok(#(False, [1])), True)
  let assert True = yes()
  let assert False = no()
  let assert True = other()
  let assert True = yes()
  Nil
}
