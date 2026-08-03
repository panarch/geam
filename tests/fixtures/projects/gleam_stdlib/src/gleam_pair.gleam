import gleam/pair

pub fn main() {
  let value = pair.new(1, "one")

  assert pair.first(value) == 1
  assert pair.second(value) == "one"
  assert pair.swap(value) == #("one", 1)
  assert pair.map_first(of: #(1, False), with: int_name) == #("one", False)
  assert pair.map_second(of: #(1, "one"), with: fn(value) { value == "one" })
    == #(1, True)

  Nil
}

fn int_name(value: Int) -> String {
  case value {
    1 -> "one"
    _ -> "other"
  }
}
// @geam:expect Nil
