pub type Option(a) {
  Some(a)
  None
}

pub type Pair {
  Pair(left: Int, right: Int)
}

const base = Pair(1, 2)

fn identity(value: a) -> a {
  value
}

fn function_value(expected: Option(fn(Int) -> Int)) -> Bool {
  case expected {
    candidate if Some(identity) == candidate -> True
    _ -> False
  }
}

fn matches(value: a, expected: Option(a)) -> Bool {
  case expected {
    candidate if Some(value) == candidate -> True
    _ -> False
  }
}

fn nested(value: a, tail: List(a), expected: Option(#(List(a), a))) -> Bool {
  case expected {
    candidate if Some(#([value, ..tail], value)) == candidate -> True
    _ -> False
  }
}

fn captured(value: a) {
  fn(expected: Option(a)) {
    case expected {
      candidate if Some(value) == candidate -> True
      _ -> False
    }
  }
}

fn pattern(values: List(Int), expected: Option(Int)) -> Int {
  case values {
    [head, ..] if expected == Some(head) -> 1
    [head, ..] if Some(head) != expected -> 2
    _ -> 3
  }
}

fn alternatives(values: #(Int, Int), expected: Option(Int)) -> Int {
  case values {
    #(head, 0) | #(0, head) if Some(head) == expected -> head
    _ -> -1
  }
}

fn composite(prefix: String, suffix: String, value: Int, expected) -> Bool {
  case expected {
    candidate if Some(#(prefix <> suffix, <<value:size(8)>>)) == candidate ->
      True
    _ -> False
  }
}

fn updated(value: Int, expected: Pair) -> Bool {
  case expected {
    candidate if Pair(..base, left: value) == candidate -> True
    _ -> False
  }
}

fn short_circuit(value: Int, expected: Option(BitArray)) -> Int {
  case expected {
    candidate if False && Some(<<value:size(8)>>) == candidate -> 1
    candidate if True || Some(<<value:size(8)>>) != candidate -> 2
    _ -> 3
  }
}

fn arithmetic_captured_match(value: Int, offset: Int, expected: Int) -> Bool {
  let raised = value + offset
  let restored = raised - offset
  let check = captured(restored)
  let alias = check
  alias(Some(expected))
}

pub fn main() {
  let assert True = function_value(Some(identity))
  let assert False = function_value(Some(fn(value) { value }))
  let assert True = matches(7, Some(7))
  let assert False = matches(7, Some(8))
  let assert True = matches("a", Some("a"))
  let assert True = matches([7, 8], Some([7, 8]))
  let assert True = matches(#(7, "a"), Some(#(7, "a")))
  let assert True = matches(Some(7), Some(Some(7)))
  let assert False = matches(Some(7), Some(Some(8)))
  let assert True = nested(7, [8], Some(#([7, 8], 7)))
  let assert False = nested(7, [8], Some(#([7, 9], 7)))
  let assert True = nested("a", ["b"], Some(#(["a", "b"], "a")))
  let check = captured(7)
  let alias = check
  let assert True = check(Some(7))
  let assert False = check(Some(8))
  let assert True = alias(Some(7))
  let assert True = captured("a")(Some("a"))
  let assert 1 = pattern([7, 8], Some(7))
  let assert 2 = pattern([7, 8], Some(8))
  let assert 3 = pattern([], Some(7))
  let assert 7 = alternatives(#(7, 0), Some(7))
  let assert 7 = alternatives(#(0, 7), Some(7))
  let assert -1 = alternatives(#(0, 8), Some(7))
  let assert True = composite("a", "b", 7, Some(#("ab", <<7>>)))
  let assert False = composite("a", "c", 7, Some(#("ab", <<7>>)))
  let assert True = updated(7, Pair(7, 2))
  let assert False = updated(7, Pair(8, 2))
  let assert 2 = short_circuit(7, Some(<<7>>))
  let assert True = arithmetic_captured_match(7, 1, 7)
  let assert False = arithmetic_captured_match(7, 1, 8)
  let assert True =
    arithmetic_captured_match(
      9_223_372_036_854_775_807,
      1,
      9_223_372_036_854_775_807,
    )
  let assert True =
    arithmetic_captured_match(
      9_223_372_036_854_775_808,
      1,
      9_223_372_036_854_775_808,
    )
  let assert True =
    arithmetic_captured_match(
      7,
      170_141_183_460_469_231_731_687_303_715_884_105_727,
      7,
    )
  let assert True =
    arithmetic_captured_match(
      170_141_183_460_469_231_731_687_303_715_884_105_727,
      1,
      170_141_183_460_469_231_731_687_303_715_884_105_727,
    )
  let assert False =
    arithmetic_captured_match(
      170_141_183_460_469_231_731_687_303_715_884_105_727,
      1,
      170_141_183_460_469_231_731_687_303_715_884_105_726,
    )
  let assert True = arithmetic_captured_match(7, -9_223_372_036_854_775_816, 7)
  Nil
}
// @geam:expect Nil
