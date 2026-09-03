import example_text_pattern as pattern

pub fn main() {
  let assert Ok(words) = pattern.compile("[A-Za-z]+")
  assert pattern.replace_all(words, "Geam + Gleam + Rust 2026", "<&>")
    == "<Geam> + <Gleam> + <Rust> 2026"

  let assert Ok(captures) = pattern.compile("([A-Za-z]+)-([0-9]+)")
  assert pattern.replace_all(captures, "Geam-12 Gleam-34", "\\2:\\1")
    == "12:Geam 34:Gleam"

  let assert Ok(lookahead) = pattern.compile("Geam(?=-)")
  assert pattern.find_all(lookahead, "Geam-12 Geam") == ["Geam"]

  // The native diagnostic at the OTP 29 verification baseline.
  assert pattern.compile("abc(")
    == Error(pattern.CompileError("missing closing parenthesis at byte 4"))
}
