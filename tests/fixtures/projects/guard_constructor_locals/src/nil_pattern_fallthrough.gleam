pub type Reason {
  Closed
  Timeout
}

pub type Wrap(a) {
  Wrap(a)
}

pub fn choose(value: Result(Nil, Reason)) -> Int {
  case value {
    Ok(Nil) -> 0
    Error(reason) ->
      case reason {
        Closed -> 1
        Timeout -> 2
      }
  }
}

fn control(value: Result(Nil, Reason)) -> Int {
  case value {
    Ok(_) -> 0
    Error(reason) ->
      case reason {
        Closed -> 1
        Timeout -> 2
      }
  }
}

fn tuple(value: Result(#(Nil, Nil), Reason)) -> Int {
  case value {
    Ok(#(Nil as first, Nil)) -> {
      let Nil = first
      0
    }
    Error(reason) -> choose(Error(reason))
  }
}

fn wrapped(value: Result(Wrap(Wrap(#(Nil, Nil))), Reason)) -> Int {
  case value {
    Ok(Wrap(Wrap(#(Nil, Nil)))) -> 0
    Error(reason) -> choose(Error(reason))
  }
}

fn refutable(
  value: Result(#(Nil, Bool, String, List(Int), Reason), Reason),
) -> Int {
  case value {
    Ok(#(Nil, True, "ready", [], Closed)) -> 0
    Ok(_) -> 3
    Error(reason) -> choose(Error(reason))
  }
}

fn wrapped_refutable(value: Result(Wrap(#(Nil, Bool)), Reason)) -> Int {
  case value {
    Ok(Wrap(#(Nil, True))) -> 0
    Ok(Wrap(#(_, False))) -> 3
    Error(reason) -> choose(Error(reason))
  }
}

pub fn main() {
  let assert 0 = choose(Ok(Nil))
  let assert 1 = choose(Error(Closed))
  let assert 2 = choose(Error(Timeout))
  let assert 0 = control(Ok(Nil))
  let assert 1 = control(Error(Closed))
  let assert 2 = control(Error(Timeout))
  let assert 0 = tuple(Ok(#(Nil, Nil)))
  let assert 1 = tuple(Error(Closed))
  let assert 2 = tuple(Error(Timeout))
  let assert 0 = wrapped(Ok(Wrap(Wrap(#(Nil, Nil)))))
  let assert 1 = wrapped(Error(Closed))
  let assert 2 = wrapped(Error(Timeout))
  let assert 0 = refutable(Ok(#(Nil, True, "ready", [], Closed)))
  let assert 3 = refutable(Ok(#(Nil, False, "ready", [], Closed)))
  let assert 3 = refutable(Ok(#(Nil, True, "other", [], Closed)))
  let assert 3 = refutable(Ok(#(Nil, True, "ready", [1], Closed)))
  let assert 3 = refutable(Ok(#(Nil, True, "ready", [], Timeout)))
  let assert 1 = refutable(Error(Closed))
  let assert 2 = refutable(Error(Timeout))
  let assert 0 = wrapped_refutable(Ok(Wrap(#(Nil, True))))
  let assert 3 = wrapped_refutable(Ok(Wrap(#(Nil, False))))
  let assert 1 = wrapped_refutable(Error(Closed))
  let assert 2 = wrapped_refutable(Error(Timeout))
  Nil
}
