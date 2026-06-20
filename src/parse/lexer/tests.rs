use crate::parse::test_helpers::{lex, lex_reject};

#[test]
fn tokenizes_newlines_and_preserves_comment_boundaries() {
    insta::assert_snapshot!(
        "tokenizes_newlines_and_preserves_comment_boundaries",
        lex("one\n// comment\r\ntwo")
    );
}

#[test]
fn reject_bad_string_escape() {
    insta::assert_snapshot!("reject_bad_string_escape", lex_reject(r#""\q""#));
}

#[test]
fn reject_unterminated_string() {
    insta::assert_snapshot!("reject_unterminated_string", lex_reject(r#""unterminated"#));
}

#[test]
fn reject_radix_integer_without_value() {
    insta::assert_snapshot!("reject_radix_integer_without_value", lex_reject("0x"));
}

#[test]
fn reject_radix_integer_digit_out_of_range() {
    insta::assert_snapshot!(
        "reject_radix_integer_digit_out_of_range",
        lex_reject("0b102")
    );
}

#[test]
fn reject_missing_float_exponent() {
    insta::assert_snapshot!("reject_missing_float_exponent", lex_reject("1e"));
}

#[test]
fn reject_trailing_numeric_underscore() {
    insta::assert_snapshot!("reject_trailing_numeric_underscore", lex_reject("1_"));
}

#[test]
fn reject_invalid_unicode_escape() {
    insta::assert_snapshot!("reject_invalid_unicode_escape", lex_reject(r#""\u{}""#));
}

#[test]
fn reject_unrecognized_token() {
    insta::assert_snapshot!("reject_unrecognized_token", lex_reject("~"));
}
