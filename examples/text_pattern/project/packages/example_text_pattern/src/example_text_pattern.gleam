//// Regular expressions for Gleam programs running on
//// [Geam](https://github.com/panarch/geam).
////
//// The `geam-example-text-pattern` Rust provider implements these externals.
//// This package does not include Erlang or JavaScript implementations.

/// A compiled regular expression, opaque to Gleam.
///
/// Patterns compiled from identical source text compare equal. Inspection
/// displays the pattern text, for example `Pattern("[A-Za-z]+")`.
@external(erlang, "geam_example_text_pattern", "Pattern")
pub type Pattern

/// An invalid regular expression, with a message from the Rust regex parser.
pub type CompileError {
  CompileError(message: String)
}

/// Compile a regular expression, or return `Error(CompileError(message))` if
/// the pattern is invalid.
@external(erlang, "geam_example_text_pattern", "compile")
pub fn compile(source: String) -> Result(Pattern, CompileError)

/// Check whether any part of `text` matches the pattern.
/// The pattern is not automatically anchored to the start or end of the text.
@external(erlang, "geam_example_text_pattern", "is_match")
pub fn is_match(pattern: Pattern, text: String) -> Bool

/// Return every non-overlapping match in source order, or an empty list if
/// there are no matches.
@external(erlang, "geam_example_text_pattern", "find_all")
pub fn find_all(pattern: Pattern, text: String) -> List(String)

/// Replace every non-overlapping match with `replacement`.
/// `$0` in the replacement inserts the complete match.
@external(erlang, "geam_example_text_pattern", "replace_all")
pub fn replace_all(
  pattern: Pattern,
  text: String,
  replacement: String,
) -> String
