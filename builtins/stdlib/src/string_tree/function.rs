use super::provider::StringTreePayload;
use super::storage::StringTree as StoredStringTree;
use ecow::EcoString;
use num_bigint::BigInt;
use unicode_segmentation::UnicodeSegmentation;

pub(super) fn append_tree(
    tree: &StringTreePayload,
    suffix: &StringTreePayload,
) -> StringTreePayload {
    StringTreePayload::from_stored(tree.stored().append(suffix.stored()))
}

pub(super) fn from_string(string: EcoString) -> StringTreePayload {
    StringTreePayload::from_stored(StoredStringTree::text(string))
}

pub(super) fn to_string(tree: &StringTreePayload) -> EcoString {
    tree.stored().flatten()
}

pub(super) fn byte_size(tree: &StringTreePayload) -> BigInt {
    BigInt::from(tree.stored().byte_len())
}

pub(super) fn lowercase(tree: &StringTreePayload) -> StringTreePayload {
    let value = tree.stored().flatten().to_lowercase();
    StringTreePayload::from_stored(StoredStringTree::text(value))
}

pub(super) fn uppercase(tree: &StringTreePayload) -> StringTreePayload {
    let value = tree.stored().flatten().to_uppercase();
    StringTreePayload::from_stored(StoredStringTree::text(value))
}

pub(super) fn do_to_graphemes(string: EcoString) -> Vec<EcoString> {
    string.graphemes(true).map(EcoString::from).collect()
}

pub(super) fn erl_split(tree: &StringTreePayload, pattern: EcoString) -> Vec<StringTreePayload> {
    let text = tree.stored().flatten();
    let parts = if pattern.is_empty() {
        vec![text]
    } else {
        text.split(pattern.as_str()).map(EcoString::from).collect()
    };
    parts
        .into_iter()
        .map(|part| StringTreePayload::from_stored(StoredStringTree::text(part)))
        .collect()
}

pub(super) fn replace(
    tree: &StringTreePayload,
    pattern: EcoString,
    substitute: EcoString,
) -> StringTreePayload {
    let text = tree.stored().flatten();
    let replaced = if pattern.is_empty() {
        text
    } else {
        text.replace(pattern.as_str(), substitute.as_str())
    };
    StringTreePayload::from_stored(StoredStringTree::text(replaced))
}

pub(super) fn is_equal(left: &StringTreePayload, right: &StringTreePayload) -> bool {
    left.stored().flatten() == right.stored().flatten()
}

pub(super) fn is_empty(tree: &StringTreePayload) -> bool {
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
        let execution = execution(
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
        let value = execution
            .run_main(
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
