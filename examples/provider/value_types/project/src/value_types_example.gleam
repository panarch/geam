import example_value_types/customs
import example_value_types/lists
import example_value_types/results
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

  assert customs.describe(customs.pending()) == "pending"
  assert customs.describe(customs.named("compile")) == "named:compile"
  assert customs.describe(customs.scheduled("retry", 3)) == "scheduled:retry:3"
  assert customs.describe(customs.prioritized()) == "priority:high"
  assert customs.describe(customs.tagged("first", "second")) == "tags:2:first"
  assert customs.first_priority([]) == "missing"
  assert customs.first_priority([customs.normal(), customs.high()]) == "normal"

  assert results.describe(results.parse("42")) == "ok:42"
  assert results.describe(results.parse("")) == "error:empty"
  assert results.describe(results.parse("bad")) == "error:bad"
  assert results.describe_option(results.optional(7, True)) == "some:kept:7"
  assert results.describe_option(results.optional(7, False)) == "none"
  assert results.first([]) == "missing"
  assert results.first(results.samples()) == "ok:3"
}
