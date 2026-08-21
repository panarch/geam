use super::super::schema::{Direction, StringList};
use super::StringProvider;
use crate::GleamStdlibHostProfile;
use crate::{HostCall, HostCallCompletion, HostCallError, HostCustom, HostFailure};
use ecow::EcoString;
use num_bigint::{BigInt, Sign};
use num_traits::ToPrimitive;
use unicode_segmentation::UnicodeSegmentation;

pub(in crate::string) fn grapheme_slice(
    string: EcoString,
    index: BigInt,
    length: BigInt,
) -> Result<EcoString, HostFailure> {
    if index.sign() == Sign::Minus || length.sign() == Sign::Minus {
        return Err(HostFailure::new(
            "string grapheme slice requires non-negative bounds",
        ));
    }
    let Some(index) = index.to_usize() else {
        return Ok(EcoString::new());
    };
    let length = length.to_usize().unwrap_or(usize::MAX);
    Ok(string
        .graphemes(true)
        .skip(index)
        .take(length)
        .collect::<String>()
        .into())
}

pub(in crate::string) fn unsafe_byte_slice(
    string: EcoString,
    index: BigInt,
    length: BigInt,
) -> Result<EcoString, HostFailure> {
    let index = index
        .to_usize()
        .ok_or_else(|| HostFailure::new("string byte slice index is not representable"))?;
    let length = length
        .to_usize()
        .ok_or_else(|| HostFailure::new("string byte slice length is not representable"))?;
    let end = index
        .checked_add(length)
        .ok_or_else(|| HostFailure::new("string byte slice range is not representable"))?;
    string
        .get(index..end)
        .map(EcoString::from)
        .ok_or_else(|| HostFailure::new("string byte slice is outside UTF-8 boundaries"))
}

pub(in crate::string) fn erl_split<'call, Profile>(
    call: HostCall<'call, Profile, StringProvider<Profile>, StringList>,
    string: EcoString,
    pattern: EcoString,
) -> Result<HostCallCompletion<'call, StringList>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let parts = match string.split_once(pattern.as_str()) {
        Some((first, rest)) if !pattern.is_empty() => {
            vec![EcoString::from(first), EcoString::from(rest)]
        }
        _ => vec![string],
    };
    Ok(call.return_list(parts))
}

pub(in crate::string) fn erl_trim<'call, Profile>(
    call: HostCall<'call, Profile, StringProvider<Profile>, EcoString>,
    string: EcoString,
    direction: HostCustom<'call, Direction>,
) -> Result<HostCallCompletion<'call, EcoString>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let value = if call.custom_constructor(direction) == 0 {
        string.trim_start_matches(is_pattern_whitespace)
    } else {
        string.trim_end_matches(is_pattern_whitespace)
    };
    Ok(call.return_value(value.into()))
}

fn is_pattern_whitespace(codepoint: char) -> bool {
    matches!(
        codepoint,
        '\u{0009}'
            ..='\u{000d}'
                | '\u{0020}'
                | '\u{0085}'
                | '\u{200e}'
                | '\u{200f}'
                | '\u{2028}'
                | '\u{2029}'
    )
}

#[cfg(test)]
mod tests {
    use super::{grapheme_slice, is_pattern_whitespace, unsafe_byte_slice};
    use num_bigint::BigInt;

    #[test]
    fn slices_graphemes_with_checked_unbounded_lengths() {
        assert_eq!(
            grapheme_slice("A👍🏽e\u{301}".into(), 1.into(), 1.into()),
            Ok("👍🏽".into()),
        );
        assert_eq!(
            grapheme_slice("abc".into(), BigInt::from(usize::MAX) + 1, 1.into()),
            Ok("".into()),
        );
        assert_eq!(
            grapheme_slice("abc".into(), 1.into(), BigInt::from(usize::MAX) + 1),
            Ok("bc".into()),
        );
        assert_eq!(
            grapheme_slice("abc".into(), (-1).into(), 1.into())
                .expect_err("negative index should violate the private source boundary")
                .message(),
            "string grapheme slice requires non-negative bounds",
        );
        assert_eq!(
            grapheme_slice("abc".into(), 1.into(), (-1).into())
                .expect_err("negative length should violate the private source boundary")
                .message(),
            "string grapheme slice requires non-negative bounds",
        );
    }

    #[test]
    fn checks_byte_ranges_and_utf8_boundaries() {
        assert_eq!(
            unsafe_byte_slice("a👍b".into(), 1.into(), 4.into()),
            Ok("👍".into()),
        );
        assert_eq!(
            unsafe_byte_slice("abc".into(), (-1).into(), 1.into())
                .expect_err("negative index should not be representable")
                .message(),
            "string byte slice index is not representable",
        );
        assert_eq!(
            unsafe_byte_slice("abc".into(), 0.into(), (-1).into())
                .expect_err("negative length should not be representable")
                .message(),
            "string byte slice length is not representable",
        );
        assert_eq!(
            unsafe_byte_slice("abc".into(), BigInt::from(usize::MAX), 1.into())
                .expect_err("overflowing range should not be representable")
                .message(),
            "string byte slice range is not representable",
        );
        assert_eq!(
            unsafe_byte_slice("👍".into(), 1.into(), 1.into())
                .expect_err("partial UTF-8 range should be rejected")
                .message(),
            "string byte slice is outside UTF-8 boundaries",
        );
    }

    #[test]
    fn uses_exact_pattern_whitespace() {
        for codepoint in [
            '\u{0009}', '\u{000a}', '\u{000b}', '\u{000c}', '\u{000d}', '\u{0020}', '\u{0085}',
            '\u{200e}', '\u{200f}', '\u{2028}', '\u{2029}',
        ] {
            assert!(is_pattern_whitespace(codepoint));
        }
        for codepoint in ['\u{00a0}', '\u{1680}', '\u{2000}', '\u{3000}', 'A'] {
            assert!(!is_pattern_whitespace(codepoint));
        }
    }
}
