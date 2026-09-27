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

pub fn main() {
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
