import example_text_pattern as pattern

pub fn main() {
  let assert Ok(words) = pattern.compile("[A-Za-z]+")
  let assert Ok(same_words) = pattern.compile("[A-Za-z]+")
  let assert Ok(numbers) = pattern.compile("[0-9]+")

  echo words as "compiled pattern"
  assert words == same_words
  assert words != numbers
  assert pattern.replace_all(words, "Geam + Gleam + Rust 2026", "<$0>")
    == "<Geam> + <Gleam> + <Rust> 2026"

  let assert Ok(captures) = pattern.compile("([A-Za-z]+)-([0-9]+)")
  assert pattern.replace_all(captures, "Geam-12 Gleam-34", "$2:$1")
    == "12:Geam 34:Gleam"

  let assert Error(pattern.CompileError(message)) = pattern.compile("Geam(?=-)")
  assert message != ""
}
