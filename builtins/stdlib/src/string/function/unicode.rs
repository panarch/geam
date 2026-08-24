use crate::HostFailure;
use ecow::EcoString;
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use unicode_segmentation::UnicodeSegmentation;

pub(in crate::string) fn pop_grapheme(string: EcoString) -> Result<(EcoString, EcoString), ()> {
    let Some(grapheme) = string.graphemes(true).next() else {
        return Err(());
    };
    let rest = EcoString::from(&string[grapheme.len()..]);
    Ok((grapheme.into(), rest))
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
    use super::{unsafe_int_to_utf_codepoint, utf_codepoint_to_int};
    use num_bigint::BigInt;

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
