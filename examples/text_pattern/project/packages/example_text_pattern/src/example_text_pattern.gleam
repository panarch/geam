@external(erlang, "geam_example_text_pattern", "Pattern")
pub type Pattern

pub type CompileError {
  CompileError(message: String)
}

@external(erlang, "geam_example_text_pattern", "compile")
pub fn compile(source: String) -> Result(Pattern, CompileError)

@external(erlang, "geam_example_text_pattern", "is_match")
pub fn is_match(pattern: Pattern, text: String) -> Bool

@external(erlang, "geam_example_text_pattern", "find_all")
pub fn find_all(pattern: Pattern, text: String) -> List(String)

@external(erlang, "geam_example_text_pattern", "replace_all")
pub fn replace_all(
  pattern: Pattern,
  text: String,
  replacement: String,
) -> String
