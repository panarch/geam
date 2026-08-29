import example_text_pattern as pattern
import gleam/io
import gleam/string
import inventory_rules

pub fn format_words(prefix: String) -> String {
  io.println("formatting words")
  let assert Ok(words) = pattern.compile("[A-Za-z]+")
  let matches =
    pattern.find_all(words, "Geam + Gleam 2026")
    |> string.join(", ")
    |> string.uppercase
  inventory_rules.render(prefix, matches)
}

pub fn contains_only_words(text: String) -> Bool {
  let assert Ok(words) = pattern.compile("^[A-Za-z ]+$")
  pattern.is_match(words, text)
}

pub fn choose_price(preferred: Bool, primary: Float, fallback: Float) -> Float {
  inventory_rules.choose(preferred, primary, fallback)
}
