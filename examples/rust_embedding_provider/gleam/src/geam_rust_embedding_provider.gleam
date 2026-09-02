import example_text_pattern as pattern

pub fn matches(source: String, value: String) -> Result(Bool, String) {
  case pattern.compile(source) {
    Ok(compiled) -> Ok(pattern.is_match(compiled, value))
    Error(pattern.CompileError(message)) -> Error(message)
  }
}
