type Holder {
  Holder(value: Int)
}

const limit = 2

fn subject() {
  echo 1
  [#(Holder(3), fn(value) { value + 1 }), #(Holder(4), fn(value) { value + 2 })]
}

fn retained_tail(values: List(a), enabled: Bool) -> List(a) {
  case values {
    [head, ..tail] if enabled -> tail
    _ -> values
  }
}

pub fn main() {
  let head = 100
  let result = case subject(), True {
    [#(record, callback), ..tail], enabled if !enabled || record.value < limit ->
      panic as "false guard selected"
    [#(record, callback), ..tail], enabled
      if enabled && { record.value > limit }
    -> #(callback(record.value), tail)
    _, _ -> panic as "missing branch"
  }
  let assert #(4, [#(Holder(4), callback)]) = result
  let assert 5 = callback(3)
  let assert 100 = head
  let head = case [2, 3] {
    [head, ..tail] | [_, head, ..tail] if head == limit -> head
    _ -> 0
  }
  let assert 2 = head
  let assert [3] = retained_tail([2, 3], True)
  let assert [2, 3] = retained_tail([2, 3], False)
  let assert [] = retained_tail([], True)
  let assert [Holder(4)] = retained_tail([Holder(3), Holder(4)], True)
  let assert [last] = retained_tail([fn() { 1 }, fn() { 42 }], True)
  last()
}
// @geam:echo
// tests/fixtures/execution/control_flow/case/guard_binding_selection.gleam:8
// 1
// @geam:expect Int(42)
