@external(erlang, "native", "coerce")
fn coerce(value: b) -> a

pub type Box(a) {
  Box(value: a)
}

pub type Lists(a) {
  Lists(values: List(a))
}

fn round_trip(values: List(a)) {
  case values {
    [head, ..tail] -> [head, ..tail]
    [] -> []
  }
}

fn factory() {
  round_trip
}

fn capture(value: a) {
  fn() { value }
}

pub fn main() -> Int {
  let original = coerce(42)
  let tuple = #(original)
  let boxed = Box(tuple.0)
  let values = [boxed.value]
  let returned = round_trip(values)
  let call = factory()
  let transferred = call(returned)
  let list_tuple = #(transferred)
  let list_box = Lists(list_tuple.0)
  let nested = [list_box.values]
  let assert [restored] = nested
  let assert [first, ..tail] = restored
  let repeated = [first, ..tail]
  let callback = capture(first)
  let indirect = fn() { callback() }
  let assert True = indirect() == original
  let assert True = repeated == values
  42
}
// @geam:expect Int(42)
