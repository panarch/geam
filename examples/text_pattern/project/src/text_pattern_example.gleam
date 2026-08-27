import example_text_pattern as pattern

pub fn main() {
  let assert Ok(words) = pattern.compile("[A-Za-z]+")
  let assert Ok(numbers) = pattern.compile("[0-9]+")

  assert pattern.is_match(words, "Geam + Gleam + Rust 2026")
  assert !pattern.is_match(words, "2026")
  assert !pattern.is_match(words, "")
  assert pattern.find_all(words, "Geam + Gleam + Rust 2026")
    == [
      "Geam",
      "Gleam",
      "Rust",
    ]
  assert pattern.find_all(words, "2026") == []
  assert pattern.find_all(words, "") == []
  assert pattern.find_all(numbers, "Geam + Gleam + Rust 2026") == ["2026"]
  assert pattern.replace_all(words, "Geam + Gleam + Rust 2026", "<word>")
    == "<word> + <word> + <word> 2026"
  assert pattern.replace_all(words, "2026", "<word>") == "2026"
  assert pattern.replace_all(words, "", "<word>") == ""
  assert pattern.replace_all(words, "Geam + Gleam", "") == " + "

  let assert Ok(captures) = pattern.compile("([A-Za-z]+)-([0-9]+)")
  assert pattern.find_all(captures, "Geam-12 Gleam-34")
    == ["Geam-12", "Gleam-34"]

  let assert Ok(unicode_words) = pattern.compile("\\w+")
  assert pattern.find_all(unicode_words, "caf\u{e9} \u{d55c}\u{ae00} Rust")
    == ["caf\u{e9}", "\u{d55c}\u{ae00}", "Rust"]
  assert pattern.replace_all(unicode_words, "caf\u{e9}", "th\u{e9}")
    == "th\u{e9}"

  let assert Ok(empty) = pattern.compile("")
  assert pattern.is_match(empty, "")
  assert pattern.find_all(empty, "ab") == ["", "", ""]
  assert pattern.replace_all(empty, "ab", "-") == "-a-b-"

  let assert Error(pattern.CompileError(message)) = pattern.compile("(")
  assert message != ""
}
