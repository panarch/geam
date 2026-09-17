use crate::HostFailure;
use geam_core::StringValue;
use num_bigint::{BigInt, Sign};
use num_traits::ToPrimitive;
use unicode_segmentation::UnicodeSegmentation;

pub(in crate::string) fn grapheme_slice(
    string: StringValue,
    index: BigInt,
    length: BigInt,
) -> Result<StringValue, HostFailure> {
    if index.sign() == Sign::Minus || length.sign() == Sign::Minus {
        return Err(HostFailure::new(
            "string grapheme slice requires non-negative bounds",
        ));
    }
    let Some(index) = index.to_usize() else {
        return Ok(StringValue::new());
    };
    let length = length.to_usize().unwrap_or(usize::MAX);
    if length == 0 {
        return Ok(StringValue::new());
    }
    let mut graphemes = string.grapheme_indices(true).skip(index);
    let Some((start, first)) = graphemes.next() else {
        return Ok(StringValue::new());
    };
    let end = graphemes
        .take(length - 1)
        .last()
        .map_or(start + first.len(), |(offset, text)| offset + text.len());
    Ok(string.slice(start..end))
}

pub(in crate::string) fn unsafe_byte_slice(
    string: StringValue,
    index: BigInt,
    length: BigInt,
) -> Result<StringValue, HostFailure> {
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
        .ok_or_else(|| HostFailure::new("string byte slice is outside UTF-8 boundaries"))
}

pub(in crate::string) fn erl_split(string: StringValue, pattern: StringValue) -> Vec<StringValue> {
    match string.split_once(pattern.as_str()) {
        Some((first, rest)) if !pattern.is_empty() => {
            vec![
                string.slice(0..first.len()),
                string.slice(string.len() - rest.len()..string.len()),
            ]
        }
        _ => vec![string],
    }
}

pub(in crate::string) fn erl_trim(string: StringValue, leading: bool) -> StringValue {
    if leading {
        let rest = string.trim_start_matches(is_pattern_whitespace);
        string.slice(string.len() - rest.len()..string.len())
    } else {
        let rest = string.trim_end_matches(is_pattern_whitespace);
        string.slice(0..rest.len())
    }
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
    use super::{erl_split, erl_trim, grapheme_slice, is_pattern_whitespace, unsafe_byte_slice};
    use geam_core::StringValue;
    use num_bigint::BigInt;

    #[test]
    fn byte_grapheme_split_and_trim_views_share_selected_ranges() {
        let text = StringValue::from("  abcdefghijklmnopqrstuvwxyz,ABCDEFGHIJKLMNOPQRSTUVWXYZ  ");
        let bytes = unsafe_byte_slice(text.clone(), 2.into(), 26.into()).expect("valid bytes");
        let graphemes = grapheme_slice(text.clone(), 2.into(), 26.into()).expect("valid graphemes");
        for selected in [bytes, graphemes] {
            assert_eq!(selected, "abcdefghijklmnopqrstuvwxyz");
            assert_eq!(selected.as_ptr(), text.as_ptr().wrapping_add(2));
        }
        let parts = erl_split(text.clone(), ",".into());
        assert_eq!(
            parts,
            [
                "  abcdefghijklmnopqrstuvwxyz",
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ  "
            ]
        );
        assert_eq!(parts[0].as_ptr(), text.as_ptr());
        assert_eq!(parts[1].as_ptr(), text.as_ptr().wrapping_add(29));
        let trimmed = erl_trim(erl_trim(text.clone(), true), false);
        assert_eq!(
            trimmed,
            "abcdefghijklmnopqrstuvwxyz,ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        );
        assert_eq!(trimmed.as_ptr(), text.as_ptr().wrapping_add(2));
        for (index, length) in [(0, 0), (text.len(), 1), (text.len() + 1, 3)] {
            assert_eq!(
                grapheme_slice(text.clone(), index.into(), length.into()),
                Ok(StringValue::new())
            );
        }
        let unicode = StringValue::from("\u{1f1e6}\u{1f1e7}\u{1f1e8}abcdefghijklmnopqrstuvwxyz");
        assert_eq!(
            grapheme_slice(unicode.slice(4..unicode.len()), 0.into(), 1.into()),
            Ok("\u{1f1e7}\u{1f1e8}".into())
        );
    }

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
