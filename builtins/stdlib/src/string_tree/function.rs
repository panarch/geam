use super::provider::StringTreePayload;
use super::storage::StringTree as StoredStringTree;
use geam_core::{HostFailure, StringValue};
use num_bigint::BigInt;
use std::ops::Deref;
use unicode_segmentation::UnicodeSegmentation;

pub(super) fn append_tree(
    tree: impl Deref<Target = StringTreePayload>,
    suffix: impl Deref<Target = StringTreePayload>,
) -> StringTreePayload {
    StringTreePayload::from_stored(tree.stored().append(suffix.stored()))
}

pub(super) fn from_string(string: StringValue) -> StringTreePayload {
    StringTreePayload::from_stored(StoredStringTree::text(string))
}

pub(super) fn to_string(tree: impl Deref<Target = StringTreePayload>) -> StringValue {
    tree.stored().flatten()
}

pub(super) fn byte_size(tree: impl Deref<Target = StringTreePayload>) -> BigInt {
    BigInt::from(tree.stored().byte_len())
}

pub(super) fn lowercase(
    tree: impl Deref<Target = StringTreePayload>,
) -> Result<StringTreePayload, HostFailure> {
    let text = tree
        .stored()
        .flatten()
        .into_ecostring()
        .map_err(|error| HostFailure::new(error.to_string()))?;
    let value = text.to_lowercase().into();
    Ok(StringTreePayload::from_stored(StoredStringTree::text(
        value,
    )))
}

pub(super) fn uppercase(
    tree: impl Deref<Target = StringTreePayload>,
) -> Result<StringTreePayload, HostFailure> {
    let text = tree
        .stored()
        .flatten()
        .into_ecostring()
        .map_err(|error| HostFailure::new(error.to_string()))?;
    let value = text.to_uppercase().into();
    Ok(StringTreePayload::from_stored(StoredStringTree::text(
        value,
    )))
}

pub(super) fn do_to_graphemes(string: StringValue) -> Result<Vec<StringValue>, HostFailure> {
    let text = string
        .as_str()
        .map_err(|error| HostFailure::new(error.to_string()))?;
    Ok(text
        .grapheme_indices(true)
        .map(|(index, grapheme)| string.slice(index..index + grapheme.len()))
        .collect())
}

pub(super) fn erl_split(
    tree: impl Deref<Target = StringTreePayload>,
    pattern: StringValue,
) -> Vec<StringTreePayload> {
    let text = tree.stored().flatten();
    let parts = if pattern.is_empty() {
        vec![text]
    } else {
        let mut parts = Vec::new();
        let mut start = 0;
        for index in memchr::memmem::find_iter(text.as_bytes(), pattern.as_bytes()) {
            parts.push(text.slice(start..index));
            start = index + pattern.len();
        }
        parts.push(text.slice(start..text.len()));
        parts
    };
    parts
        .into_iter()
        .map(|part| StringTreePayload::from_stored(StoredStringTree::text(part)))
        .collect()
}

pub(super) fn replace(
    tree: impl Deref<Target = StringTreePayload>,
    pattern: StringValue,
    substitute: StringValue,
) -> StringTreePayload {
    let text = tree.stored().flatten();
    if pattern.is_empty() {
        return StringTreePayload::from_stored(StoredStringTree::text(text));
    }
    let replaced = match (text.as_str(), pattern.as_str(), substitute.as_str()) {
        (Ok(text), Ok(pattern), Ok(substitute)) => text.replace(pattern, substitute).into(),
        _ => {
            let mut bytes = Vec::with_capacity(text.len());
            let mut start = 0;
            for index in memchr::memmem::find_iter(text.as_bytes(), pattern.as_bytes()) {
                bytes.extend_from_slice(&text.as_bytes()[start..index]);
                bytes.extend_from_slice(substitute.as_bytes());
                start = index + pattern.len();
            }
            bytes.extend_from_slice(&text.as_bytes()[start..]);
            StringValue::from_bytes(bytes)
        }
    };
    StringTreePayload::from_stored(StoredStringTree::text(replaced))
}

pub(super) fn is_equal(
    left: impl Deref<Target = StringTreePayload>,
    right: impl Deref<Target = StringTreePayload>,
) -> bool {
    left.stored().flatten() == right.stored().flatten()
}

pub(super) fn is_empty(tree: impl Deref<Target = StringTreePayload>) -> bool {
    tree.stored().byte_len() == 0
}

#[cfg(test)]
mod tests {
    use super::super::{STRING_TREE_DECLARATIONS, host_provider};
    use crate::{GleamStdlibProfile, GleamStdlibRunState};
    use crate::{
        HostModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource,
        compile_typed_host_program, plan_host_program,
    };
    use ecow::EcoString;
    use geam_core::StringValue;

    #[test]
    fn grapheme_lists_retain_large_selected_ranges() {
        let grapheme = format!("a{}", "\u{301}".repeat(9));
        let text = StringValue::from(format!("{grapheme}x{grapheme}"));
        let parts = super::do_to_graphemes(text.clone()).unwrap();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0].as_str().unwrap(), grapheme);
        assert_eq!(parts[1], "x");
        assert_eq!(parts[2].as_str().unwrap(), grapheme);
        assert_eq!(parts[0].as_ptr(), text.as_ptr());
        assert_eq!(
            parts[2].as_ptr(),
            text.as_ptr().wrapping_add(grapheme.len() + 1)
        );
        drop(text);
        assert_eq!(parts[0], parts[2]);
        assert!(super::do_to_graphemes("".into()).unwrap().is_empty());
    }

    fn execution(source: &str) -> HostedExecution<GleamStdlibProfile> {
        let source = format!("{STRING_TREE_DECLARATIONS}\n{source}");
        let provider = host_provider::<GleamStdlibProfile>()
            .expect("official string tree provider should register");
        let typed = compile_typed_host_program(
            "gleam_stdlib",
            "gleam/string_tree",
            [PackageSource::new(
                "gleam_stdlib",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "gleam/string_tree",
                    "src/gleam/string_tree.gleam",
                    source,
                )],
            )],
            HostProviderSet::with_providers(
                [
                    HostModule::<GleamStdlibProfile>::new_for_profile("gleam_stdlib", "fixture")
                        .unwrap()
                        .with_function("raw", || StringValue::from_bytes(vec![0xff]))
                        .unwrap(),
                ],
                [provider],
            )
            .expect("string tree provider module should be unique"),
        )
        .expect("synthetic string tree source should compile");
        let plan = plan_host_program(typed).expect("synthetic string tree source should plan");
        HostedExecution::try_from_module_plan(plan)
            .expect("synthetic string tree execution should seal")
    }

    #[test]
    fn raw_tree_unicode_failures_keep_the_registered_native_origin() {
        for (body, function) in [
            ("lowercase(from_string(fixture.raw()))", "lowercase"),
            ("uppercase(from_string(fixture.raw()))", "uppercase"),
            ("do_to_graphemes(fixture.raw())", "do_to_graphemes"),
        ] {
            let source = format!(
                r#"
import fixture
pub fn main() {{ {body} }}
"#
            );
            let error = crate::execution_fixture::run(
                &mut execution(&source),
                &mut GleamStdlibRunState::from_seed([0; 32]),
                &mut Vec::new(),
            )
            .unwrap_err();
            assert_eq!(
                error.to_string(),
                format!(
                    "host function gleam_stdlib::gleam/string_tree.{function} failed: invalid utf-8 sequence of 1 bytes from index 0"
                )
            );
        }
    }

    #[test]
    fn executes_every_string_tree_provider_through_the_hosted_pipeline() {
        let mut execution = execution(
            r#"
pub fn main() {
  let segmented = from_strings(["a", "b"])
  let flat = from_string("ab")
  let joined = concat([segmented, from_string("c")])
  let appended = append_tree(joined, from_string("d"))
  assert segmented != flat
  assert is_equal(segmented, flat)
  assert to_string(appended) == "abcd"
  assert byte_size(appended) == 4
  assert to_string(lowercase(from_string("Gleam"))) == "gleam"
  assert to_string(uppercase(from_string("Gleam"))) == "GLEAM"
  assert do_to_graphemes("A👍🏽é") == ["A", "👍🏽", "é"]
  assert erl_split(from_string("a,b,c"), ",", All)
    == [from_string("a"), from_string("b"), from_string("c")]
  assert erl_split(from_string("abc"), "", All) == [from_string("abc")]
  assert to_string(replace(from_string("a-b-a"), "a", "x")) == "x-b-x"
  assert to_string(replace(from_string("abc"), "", "x")) == "abc"
  assert is_empty(from_strings([]))
  appended
}
"#,
        );
        let value = crate::execution_fixture::run(
            &mut execution,
            &mut GleamStdlibRunState::from_seed([0; 32]),
            &mut Vec::new(),
        )
        .expect("string tree providers should run");

        assert_eq!(
            value.inspect().to_string(),
            r#"string_tree.from_string("abcd")"#,
        );
    }
    #[test]
    fn tree_byte_operations_preserve_raw_leaves_and_case_conversion_requires_text() {
        use super::{
            do_to_graphemes, erl_split, from_string, is_equal, lowercase, replace, to_string,
            uppercase,
        };
        let raw = StringValue::from_bytes(vec![b'a', 0xff, b'b', 0xff]);
        let tree = from_string(raw.clone());
        let pattern = StringValue::from_bytes(vec![0xff]);
        let parts = erl_split(&tree, pattern.clone());
        assert_eq!(
            parts
                .iter()
                .map(|part| to_string(part).as_bytes().to_vec())
                .collect::<Vec<_>>(),
            [b"a".to_vec(), b"b".to_vec(), vec![]]
        );
        let substituted = replace(&tree, pattern, StringValue::from_bytes(vec![0x80, 0]));
        assert_eq!(
            to_string(&substituted).as_bytes(),
            &[b'a', 0x80, 0, b'b', 0x80, 0]
        );
        let unchanged = replace(&tree, "".into(), "ignored".into());
        assert!(is_equal(&tree, &unchanged));
        assert_eq!(to_string(&tree), raw);
        let text = from_string("é-é".into());
        assert_eq!(to_string(&replace(&text, "é".into(), "🙂".into())), "🙂-🙂");
        assert_eq!(
            lowercase(&tree).err().unwrap().message(),
            "invalid utf-8 sequence of 1 bytes from index 1"
        );
        assert_eq!(
            uppercase(&tree).err().unwrap().message(),
            "invalid utf-8 sequence of 1 bytes from index 1"
        );
        assert_eq!(
            do_to_graphemes(raw).unwrap_err().message(),
            "invalid utf-8 sequence of 1 bytes from index 1"
        );
    }
}
