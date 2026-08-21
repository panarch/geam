mod inspect;
mod slice;
mod unicode;

pub(super) use self::inspect::do_inspect;
pub(super) use self::slice::{erl_split, erl_trim, grapheme_slice, unsafe_byte_slice};
pub(super) use self::unicode::{
    from_utf_codepoints, pop_grapheme, unsafe_int_to_utf_codepoint, utf_codepoint_to_int,
};

use crate::{GleamStdlibRunState, StringTreeExternalStorage, StringTreeSchema, stdlib_state};
use crate::{HostExternalBinding, HostProvider};
use ecow::EcoString;
use num_bigint::BigInt;
use std::marker::PhantomData;

pub(super) struct StringProvider<Profile>(PhantomData<Profile>);

impl<Profile> HostProvider<Profile> for StringProvider<Profile>
where
    Profile: super::super::GleamStdlibHostProfile,
{
    type State = GleamStdlibRunState<Profile::Io>;

    fn project(state: &mut Profile::RunState) -> &mut Self::State {
        stdlib_state::<Profile>(state)
    }
}

impl<Profile> HostExternalBinding<Profile, StringTreeSchema> for StringProvider<Profile>
where
    Profile: super::super::GleamStdlibHostProfile,
{
    type Storage = StringTreeExternalStorage;
}

pub(super) fn length(string: EcoString) -> BigInt {
    BigInt::from(
        unicode_segmentation::UnicodeSegmentation::graphemes(string.as_str(), true).count(),
    )
}

pub(super) fn lowercase(string: EcoString) -> EcoString {
    string.to_lowercase()
}

pub(super) fn uppercase(string: EcoString) -> EcoString {
    string.to_uppercase()
}

pub(super) fn less_than(left: EcoString, right: EcoString) -> bool {
    left < right
}

pub(super) fn crop(string: EcoString, substring: EcoString) -> EcoString {
    match string.find(substring.as_str()) {
        Some(index) => string[index..].into(),
        None => string,
    }
}

pub(super) fn contains(haystack: EcoString, needle: EcoString) -> bool {
    haystack.contains(needle.as_str())
}

pub(super) fn starts_with(string: EcoString, prefix: EcoString) -> bool {
    string.starts_with(prefix.as_str())
}

pub(super) fn ends_with(string: EcoString, suffix: EcoString) -> bool {
    string.ends_with(suffix.as_str())
}

pub(super) fn byte_size(string: EcoString) -> BigInt {
    BigInt::from(string.len())
}

pub(super) fn remove_prefix(string: EcoString, prefix: EcoString) -> EcoString {
    string
        .strip_prefix(prefix.as_str())
        .map(EcoString::from)
        .unwrap_or(string)
}

pub(super) fn remove_suffix(string: EcoString, suffix: EcoString) -> EcoString {
    string
        .strip_suffix(suffix.as_str())
        .map(EcoString::from)
        .unwrap_or(string)
}

#[cfg(test)]
mod tests {
    use super::{
        StringProvider, byte_size, contains, crop, ends_with, length, less_than, lowercase,
        remove_prefix, remove_suffix, starts_with, uppercase,
    };
    use crate::HostProvider;
    use crate::{GleamStdlibProfile, GleamStdlibRunState};
    use num_bigint::BigInt;

    #[test]
    fn projects_the_stdlib_run_state() {
        let mut state = GleamStdlibRunState::from_seed([0; 32]);
        let original = std::ptr::from_ref(&state);
        let projected =
            <StringProvider<GleamStdlibProfile> as HostProvider<GleamStdlibProfile>>::project(
                &mut state,
            );

        assert_eq!(std::ptr::from_mut(projected).cast_const(), original);
    }

    #[test]
    fn applies_scalar_string_semantics() {
        assert_eq!(length("A👍🏽e\u{301}".into()), BigInt::from(3));
        assert_eq!(lowercase("Gleam İ".into()), "gleam i\u{307}");
        assert_eq!(uppercase("Gleam ß".into()), "GLEAM SS");
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
}
