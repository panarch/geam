pub type Payload {
  Payload(Int, BitArray, String, fn(Int) -> Int)
  Empty
}

fn add(captured: Int) {
  fn(value) { captured + value }
}

fn final_literal(value: Int) {
  let assert 42 = value
}

pub fn main() {
  let assert 1 as one = 1
  let assert 1.5 = 1.5
  let assert "ready" = "ready"
  let assert Nil = Nil

  let function = add(10)
  let subject = #(
    [1],
    <<2>>,
    Payload(3, <<4>>, "prefix", function),
  )
  let assert #(
    [first],
    <<second>>,
    Payload(third, <<fourth>>, "pre" <> suffix, nested_function) as payload,
  ) as whole = subject
  let assert #(
    [whole_first],
    <<whole_second>>,
    Payload(whole_third, _, _, whole_function),
  ) = whole
  let assert Payload(payload_number, _, _, payload_function) = payload

  let captured = 5
  let message = "unused"
  let closure = fn(value) {
    let assert #(captured_value, [item]) = #(captured, [value]) as message
    captured_value + item
  }

  #(
    first,
    one,
    second,
    third,
    fourth,
    suffix,
    nested_function(1),
    whole_first + whole_second + whole_third + whole_function(1),
    payload_number + payload_function(1),
    closure(6),
    final_literal(42),
  )
}

// @geam:expect Tuple([Int(1), Int(1), Int(2), Int(3), Int(4), String("fix"), Int(11), Int(17), Int(14), Int(11), Int(42)])
