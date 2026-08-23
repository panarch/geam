import example_value_types/lists
import example_value_types/scalars
import example_value_types/tuples

pub fn main() {
  assert scalars.join("geam", "types") == "geam:types"
  assert scalars.add(20, 22) == 42
  assert scalars.multiply(1.5, 2.0) == 3.0
  assert scalars.keep_bits(<<1, 2, 3>>) == <<1, 2, 3>>
  let assert <<codepoint:utf8_codepoint>> = <<"G":utf8>>
  assert scalars.keep_codepoint(codepoint) == codepoint
  assert scalars.invert(False)
  assert scalars.keep_nil(Nil) == Nil

  let wrapped = tuples.wrap("geam")
  assert wrapped == #("geam")
  assert tuples.unwrap(wrapped) == "geam"

  assert tuples.swap(#("items", 3)) == #(3, "items")

  assert tuples.rotate(#("ready", 1.5, True)) == #(True, "ready", 1.5)

  assert tuples.reassociate(#("jobs", #(4, False))) == #(#("jobs", 4), False)

  assert lists.length([]) == 0
  assert lists.length([1, 2, 3]) == 3
  assert lists.first_or([], "fallback") == "fallback"
  assert lists.first_or(["first", "second"], "fallback") == "first"

  let numbers = [1, 2, 3]
  assert lists.identity(numbers) == numbers
  assert lists.reverse(["first", "second", "third"]) == [
    "third",
    "second",
    "first",
  ]
  assert lists.labels([#("alpha", 1), #("beta", 2)]) == ["alpha", "beta"]
}
