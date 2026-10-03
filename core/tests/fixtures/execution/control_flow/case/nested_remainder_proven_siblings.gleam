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
  Nil
}
// @geam:expect Nil
