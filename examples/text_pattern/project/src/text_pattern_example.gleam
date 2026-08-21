import example_text_pattern

pub fn main() {
  let assert Ok(words) = example_text_pattern.compile("[A-Za-z]+")
  let assert Ok(same_words) = example_text_pattern.compile("[A-Za-z]+")
  let assert Ok(numbers) = example_text_pattern.compile("[0-9]+")

  echo words as "compiled pattern"
  assert words == same_words
  assert words != numbers
  assert example_text_pattern.is_match(words, "Geam + Gleam + Rust 2026")
  assert !example_text_pattern.is_match(words, "2026")
  assert example_text_pattern.find_all(words, "Geam + Gleam + Rust 2026")
    == [
      "Geam",
      "Gleam",
      "Rust",
    ]
  assert example_text_pattern.replace_all(
      words,
      "Geam + Gleam + Rust 2026",
      "<$0>",
    )
    == "<Geam> + <Gleam> + <Rust> 2026"

  let assert Error(example_text_pattern.CompileError(message)) =
    example_text_pattern.compile("(")
  assert message != ""
}
