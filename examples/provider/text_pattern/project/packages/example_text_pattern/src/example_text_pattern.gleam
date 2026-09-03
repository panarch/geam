//// Regular expressions for Gleam on Erlang and
//// [Geam](https://github.com/panarch/geam).
////
//// Erlang uses the included OTP `re` implementation with `unicode` and `ucp`.
//// Geam uses the `geam-example-text-pattern` Rust provider and its `regex`
//// engine. Pattern syntax, replacement syntax, zero-length matches, resource
//// limits, and error messages follow each engine. There is no JavaScript
//// implementation.

/// A compiled regular expression, opaque to Gleam.
///
/// Equality and inspection are runtime-specific, not portable guarantees.
/// On Geam, identical source text compares equal and inspection displays the
/// pattern text, for example `Pattern("[A-Za-z]+")`. On Erlang, this is OTP's
/// opaque compiled pattern.
@external(erlang, "example_text_pattern_ffi", "Pattern")
pub type Pattern

/// An invalid regular expression, with a message from the active regex engine.
pub type CompileError {
  CompileError(message: String)
}

/// Compile a regular expression, or return `Error(CompileError(message))` if
/// the pattern is invalid. Syntax is interpreted by the active engine without
/// translation; a pattern accepted by one engine may be rejected by the other.
@external(erlang, "example_text_pattern_ffi", "compile")
pub fn compile(source: String) -> Result(Pattern, CompileError)

/// Check whether any part of `text` matches the pattern.
/// The pattern is not automatically anchored to the start or end of the text.
@external(erlang, "example_text_pattern_ffi", "is_match")
pub fn is_match(pattern: Pattern, text: String) -> Bool

/// Return whole, non-overlapping matches from left to right, or an empty list
/// if there are no matches. Capturing groups are not returned separately.
/// Zero-length matches follow the active engine's iteration rules.
@external(erlang, "example_text_pattern_ffi", "find_all")
pub fn find_all(pattern: Pattern, text: String) -> List(String)

/// Replace every non-overlapping match with `replacement`.
/// Replacement syntax follows the active engine. Geam uses `$0` for the whole
/// match and `$1` for the first capture. Erlang uses `&` and `\\1` respectively
/// (the latter written with an escaped backslash in a Gleam string).
@external(erlang, "example_text_pattern_ffi", "replace_all")
pub fn replace_all(
  pattern: Pattern,
  text: String,
  replacement: String,
) -> String
