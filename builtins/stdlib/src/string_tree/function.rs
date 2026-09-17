use super::provider::StringTreePayload;
use super::storage::StringTree as StoredStringTree;
use geam_core::StringValue;
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

pub(super) fn lowercase(tree: impl Deref<Target = StringTreePayload>) -> StringTreePayload {
    let value = tree.stored().flatten().into_ecostring().to_lowercase();
    StringTreePayload::from_stored(StoredStringTree::text(value.into()))
}

pub(super) fn uppercase(tree: impl Deref<Target = StringTreePayload>) -> StringTreePayload {
    let value = tree.stored().flatten().into_ecostring().to_uppercase();
    StringTreePayload::from_stored(StoredStringTree::text(value.into()))
}

pub(super) fn do_to_graphemes(string: StringValue) -> Vec<StringValue> {
    string
        .grapheme_indices(true)
        .map(|(index, grapheme)| string.slice(index..index + grapheme.len()))
        .collect()
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
        for (index, matched) in text.match_indices(pattern.as_str()) {
            parts.push(text.slice(start..index));
            start = index + matched.len();
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
    let replaced = if pattern.is_empty() {
        text
    } else {
        text.into_ecostring()
            .replace(pattern.as_str(), substitute.as_str())
            .into()
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

    #[test]
    fn grapheme_lists_retain_large_selected_ranges() {
        let grapheme = format!("a{}", "\u{301}".repeat(9));
        let text = geam_core::StringValue::from(format!("{grapheme}x{grapheme}"));
        let parts = super::do_to_graphemes(text.clone());
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0].as_str(), grapheme);
        assert_eq!(parts[1], "x");
        assert_eq!(parts[2].as_str(), grapheme);
        assert_eq!(parts[0].as_ptr(), text.as_ptr());
        assert_eq!(
            parts[2].as_ptr(),
            text.as_ptr().wrapping_add(grapheme.len() + 1)
        );
        drop(text);
        assert_eq!(parts[0], parts[2]);
        assert!(super::do_to_graphemes("".into()).is_empty());
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
                Vec::<HostModule<GleamStdlibProfile>>::new(),
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
}
