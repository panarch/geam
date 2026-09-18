use crate::HostFailure;
use geam_core::StringValue;
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use unicode_segmentation::UnicodeSegmentation;

pub(in crate::string) fn pop_grapheme(
    string: StringValue,
) -> Result<(StringValue, StringValue), ()> {
    let Some(grapheme) = string.graphemes(true).next() else {
        return Err(());
    };
    Ok((
        string.slice(0..grapheme.len()),
        string.slice(grapheme.len()..string.len()),
    ))
}

pub(in crate::string) fn unsafe_int_to_utf_codepoint(value: BigInt) -> Result<char, HostFailure> {
    value
        .to_u32()
        .and_then(char::from_u32)
        .ok_or_else(|| HostFailure::new("integer is not a valid Unicode codepoint"))
}

pub(in crate::string) fn utf_codepoint_to_int(value: char) -> BigInt {
    BigInt::from(u32::from(value))
}

#[cfg(test)]
mod tests {
    use super::{pop_grapheme, unsafe_int_to_utf_codepoint, utf_codepoint_to_int};
    use geam_core::StringValue;
    use num_bigint::BigInt;

    #[test]
    fn graphemes_share_large_ranges_and_resegment_the_visible_text() {
        let first = format!("a{}", "\u{301}".repeat(9));
        let original = StringValue::from(format!("{first}abcdefghijklmnopqrstuvwxyz"));
        let (grapheme, rest) = pop_grapheme(original.clone()).expect("nonempty string");
        assert_eq!(grapheme.as_str(), first);
        assert_eq!(grapheme.as_ptr(), original.as_ptr());
        assert_eq!(rest, "abcdefghijklmnopqrstuvwxyz");
        assert_eq!(rest.as_ptr(), original.as_ptr().wrapping_add(first.len()));
        let regional = StringValue::from("\u{1f1e6}\u{1f1e7}\u{1f1e8}abcdefghijklmnopqrstuvwxyz");
        let inside_original_grapheme = regional.slice(4..regional.len());
        let (grapheme, rest) = pop_grapheme(inside_original_grapheme).expect("valid UTF-8 view");
        assert_eq!(grapheme, "\u{1f1e7}\u{1f1e8}");
        assert_eq!(rest, "abcdefghijklmnopqrstuvwxyz");
        assert_eq!(rest.as_ptr(), regional.as_ptr().wrapping_add(12));
        assert_eq!(pop_grapheme(StringValue::new()), Err(()));
    }

    #[test]
    fn converts_exact_unicode_scalar_values() {
        assert_eq!(unsafe_int_to_utf_codepoint(65.into()), Ok('A'));
        assert_eq!(
            utf_codepoint_to_int('\u{10ffff}'),
            BigInt::from(0x10ffff_u32)
        );
        assert_eq!(
            unsafe_int_to_utf_codepoint((-1).into())
                .expect_err("negative codepoint should fail")
                .message(),
            "integer is not a valid Unicode codepoint",
        );
        assert_eq!(
            unsafe_int_to_utf_codepoint(0xd800_u32.into())
                .expect_err("surrogate should fail")
                .message(),
            "integer is not a valid Unicode codepoint",
        );
        assert_eq!(
            unsafe_int_to_utf_codepoint(BigInt::from(u64::MAX))
                .expect_err("unrepresentable codepoint should fail")
                .message(),
            "integer is not a valid Unicode codepoint",
        );
    }
}
