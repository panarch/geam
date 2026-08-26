import gleam/string
import gleam/string_tree

pub type Person {
  Person(name: String)
}

fn increment(value: Int) -> Int {
  value + 1
}

pub fn main() {
  let assert Ok(codepoint) = string.utf_codepoint(65)
  let assert Ok(newline) = string.utf_codepoint(10)
  let assert Ok(quote) = string.utf_codepoint(34)
  let assert Ok(backslash) = string.utf_codepoint(92)
  let newline = string.from_utf_codepoints([newline])
  let quote = string.from_utf_codepoints([quote])
  let backslash = string.from_utf_codepoints([backslash])
  assert string.inspect(1) == "1"
  assert string.inspect(1.5) == "1.5"
  assert string.inspect("hello" <> newline)
    == quote <> "hello" <> backslash <> "n" <> quote
  assert string.inspect(<<1, 2>>) == "<<1, 2>>"
  assert string.inspect(codepoint) == "'A'"
  assert string.inspect(True) == "True"
  assert string.inspect(Nil) == "Nil"
  assert string.inspect(#(1, "one"))
    == "#(1, " <> quote <> "one" <> quote <> ")"
  assert string.inspect([1, 2]) == "[1, 2]"
  assert string.inspect(Person(name: "Kim"))
    == "Person(name: " <> quote <> "Kim" <> quote <> ")"
  assert string.inspect(increment) == "//fn(a) { ... }"
  assert string.inspect(string_tree.from_string("tree"))
    == "string_tree.from_string(" <> quote <> "tree" <> quote <> ")"

  Nil
}

// @geam:expect Nil
