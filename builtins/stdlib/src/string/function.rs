mod inspect;
mod slice;
mod unicode;

pub(super) use self::inspect::do_inspect;
pub(super) use self::slice::{erl_split, erl_trim, grapheme_slice, unsafe_byte_slice};
pub(super) use self::unicode::{pop_grapheme, unsafe_int_to_utf_codepoint, utf_codepoint_to_int};

use geam_core::{HostFailure, StringValue};
use num_bigint::BigInt;

pub(super) fn length(string: StringValue) -> Result<BigInt, HostFailure> {
    let text = string
        .as_str()
        .map_err(|error| HostFailure::new(error.to_string()))?;
    Ok(BigInt::from(
        unicode_segmentation::UnicodeSegmentation::graphemes(text, true).count(),
    ))
}

pub(super) fn lowercase(string: StringValue) -> Result<StringValue, HostFailure> {
    let text = string
        .into_ecostring()
        .map_err(|error| HostFailure::new(error.to_string()))?;
    Ok(text.to_lowercase().into())
}

pub(super) fn uppercase(string: StringValue) -> Result<StringValue, HostFailure> {
    let text = string
        .into_ecostring()
        .map_err(|error| HostFailure::new(error.to_string()))?;
    Ok(text.to_uppercase().into())
}

pub(super) fn less_than(left: StringValue, right: StringValue) -> bool {
    left < right
}

pub(super) fn crop(string: StringValue, substring: StringValue) -> StringValue {
    match memchr::memmem::find(string.as_bytes(), substring.as_bytes()) {
        Some(index) => string.slice(index..string.len()),
        None => string,
    }
}

pub(super) fn contains(haystack: StringValue, needle: StringValue) -> bool {
    memchr::memmem::find(haystack.as_bytes(), needle.as_bytes()).is_some()
}

pub(super) fn starts_with(string: StringValue, prefix: StringValue) -> bool {
    string.starts_with(prefix.as_bytes())
}

pub(super) fn ends_with(string: StringValue, suffix: StringValue) -> bool {
    string.ends_with(suffix.as_bytes())
}

pub(super) fn byte_size(string: StringValue) -> BigInt {
    BigInt::from(string.len())
}

pub(super) fn remove_prefix(string: StringValue, prefix: StringValue) -> StringValue {
    if string.starts_with(prefix.as_bytes()) {
        string.slice(prefix.len()..string.len())
    } else {
        string
    }
}

pub(super) fn remove_suffix(string: StringValue, suffix: StringValue) -> StringValue {
    if string.ends_with(suffix.as_bytes()) {
        string.slice(0..string.len() - suffix.len())
    } else {
        string
    }
}

#[cfg(test)]
mod tests {
    use super::{
        byte_size, contains, crop, ends_with, length, less_than, lowercase, remove_prefix,
        remove_suffix, starts_with, uppercase,
    };
    use geam_core::StringValue;
    use num_bigint::BigInt;

    #[test]
    fn selections_keep_visible_ranges_of_the_original_string() {
        let original = StringValue::from("prefix:abcdefghijklmnopqrstuvwxyz:suffix");
        let cropped = crop(original.clone(), "abcdefghijklmnopqrstuvwxyz".into());
        let unprefixed = remove_prefix(original.clone(), "prefix:".into());
        let middle = remove_suffix(unprefixed.clone(), ":suffix".into());
        assert_eq!(cropped, "abcdefghijklmnopqrstuvwxyz:suffix");
        assert_eq!(unprefixed, cropped);
        assert_eq!(cropped.as_ptr(), original.as_ptr().wrapping_add(7));
        assert_eq!(unprefixed.as_ptr(), cropped.as_ptr());
        assert_eq!(middle, "abcdefghijklmnopqrstuvwxyz");
        assert_eq!(middle.as_ptr(), cropped.as_ptr());
        for unchanged in [
            crop(original.clone(), "absent".into()),
            remove_prefix(original.clone(), "absent".into()),
            remove_suffix(original.clone(), "absent".into()),
        ] {
            assert_eq!(unchanged, original);
            assert_eq!(unchanged.as_ptr(), original.as_ptr());
        }
    }

    #[test]
    fn case_conversion_uses_only_the_visible_range() {
        let text = StringValue::from("hidden:ABCdefghijklmnopqr:end");
        let visible = text.slice(7..25);
        assert_eq!(lowercase(visible.clone()).unwrap(), "abcdefghijklmnopqr");
        assert_eq!(uppercase(visible.clone()).unwrap(), "ABCDEFGHIJKLMNOPQR");
        assert_eq!(visible, "ABCdefghijklmnopqr");
    }

    #[test]
    fn applies_scalar_string_semantics() {
        assert_eq!(length("A👍🏽e\u{301}".into()).unwrap(), BigInt::from(3));
        assert_eq!(lowercase("Gleam İ".into()).unwrap(), "gleam i\u{307}");
        assert_eq!(uppercase("Gleam ß".into()).unwrap(), "GLEAM SS");
        assert!(less_than("A".into(), "B".into()));
        assert!(!less_than("B".into(), "A".into()));
        assert_eq!(crop("The Lone Gunmen".into(), "Lone".into()), "Lone Gunmen");
        assert_eq!(
            crop("The Lone Gunmen".into(), "Fox".into()),
            "The Lone Gunmen"
        );
        assert!(contains("theory".into(), "ory".into()));
        assert!(!contains("theory".into(), "THE".into()));
        assert!(starts_with("theory".into(), "the".into()));
        assert!(ends_with("theory".into(), "ory".into()));
        assert_eq!(byte_size("👍".into()), BigInt::from(4));
        assert_eq!(remove_prefix("@lpil".into(), "@".into()), "lpil");
        assert_eq!(remove_prefix("hello!".into(), "@".into()), "hello!");
        assert_eq!(remove_suffix("Hello!".into(), "!".into()), "Hello");
        assert_eq!(remove_suffix("Hello!?".into(), "!".into()), "Hello!?");
    }
    #[test]
    fn byte_operations_preserve_invalid_ranges_and_unicode_operations_reject_them() {
        let raw = StringValue::from_bytes(vec![0xc3, 0xa9, 0, 0xff, 0x80]);
        let pattern = StringValue::from_bytes(vec![0xff]);
        assert_eq!(byte_size(raw.clone()), BigInt::from(5));
        assert!(contains(raw.clone(), pattern.clone()));
        assert!(starts_with(raw.clone(), "é".into()));
        assert!(ends_with(raw.clone(), StringValue::from_bytes(vec![0x80])));
        assert_eq!(crop(raw.clone(), pattern.clone()).as_bytes(), &[0xff, 0x80]);
        assert_eq!(
            remove_prefix(raw.clone(), "é".into()).as_bytes(),
            &[0, 0xff, 0x80]
        );
        assert_eq!(
            remove_suffix(
                raw.clone(),
                pattern.concat(&StringValue::from_bytes(vec![0x80]))
            )
            .as_bytes(),
            &[0xc3, 0xa9, 0]
        );
        assert!(less_than("é".into(), pattern));
        for result in [lowercase(raw.clone()), uppercase(raw.clone())] {
            assert_eq!(
                result.unwrap_err().message(),
                "invalid utf-8 sequence of 1 bytes from index 3"
            );
        }
        assert_eq!(
            length(raw.clone()).unwrap_err().message(),
            "invalid utf-8 sequence of 1 bytes from index 3"
        );
        let valid = raw.slice(0..2);
        assert_eq!(length(valid.clone()), Ok(1.into()));
        assert_eq!(uppercase(valid).unwrap(), "É");
    }
}
