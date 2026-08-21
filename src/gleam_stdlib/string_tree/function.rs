use super::schema::{
    Direction, SplitConstructions, SplitStringTreeIndex, StringList, StringTree, StringTreeList,
};
use super::storage::{
    StringTree as StoredStringTree, StringTreeExternalStorage, StringTreePayload,
};
use crate::gleam_stdlib::{GleamStdlibHostProfile, GleamStdlibRunState, stdlib_state};
use crate::{
    HostCall, HostCallCompletion, HostCallError, HostConstructions, HostCustom, HostExternal,
    HostExternalBinding, HostList, HostProvider,
};
use ecow::EcoString;
use num_bigint::BigInt;
use std::marker::PhantomData;
use unicode_segmentation::UnicodeSegmentation;

pub(super) struct StringTreeProvider<Profile>(PhantomData<Profile>);

impl<Profile> HostProvider<Profile> for StringTreeProvider<Profile>
where
    Profile: GleamStdlibHostProfile,
{
    type State = GleamStdlibRunState<Profile::Io>;

    fn project(state: &mut Profile::RunState) -> &mut Self::State {
        stdlib_state::<Profile>(state)
    }
}

impl<Profile> HostExternalBinding<Profile, super::schema::StringTreeSchema>
    for StringTreeProvider<Profile>
where
    Profile: GleamStdlibHostProfile,
{
    type Storage = StringTreeExternalStorage;
}

pub(super) fn append_tree<'call, Profile>(
    mut call: HostCall<'call, Profile, StringTreeProvider<Profile>, StringTree>,
    tree: HostExternal<'call, StringTree>,
    suffix: HostExternal<'call, StringTree>,
) -> Result<HostCallCompletion<'call, StringTree>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let tree = call.external_payload(tree).tree.clone();
    let suffix = call.external_payload(suffix).tree.clone();
    let value = call.create_external(StringTreePayload {
        tree: tree.append(&suffix),
    });
    Ok(call.return_value(value))
}

pub(super) fn from_strings<'call, Profile>(
    mut call: HostCall<'call, Profile, StringTreeProvider<Profile>, StringTree>,
    strings: HostList<'call, EcoString>,
) -> Result<HostCallCompletion<'call, StringTree>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let mut index = 0;
    let mut trees = Vec::with_capacity(call.list_len(strings));
    while let Some(string) = call.list_item(strings, index) {
        trees.push(StoredStringTree::text(string));
        index += 1;
    }
    let value = call.create_external(StringTreePayload {
        tree: StoredStringTree::sequence(trees),
    });
    Ok(call.return_value(value))
}

pub(super) fn concat<'call, Profile>(
    mut call: HostCall<'call, Profile, StringTreeProvider<Profile>, StringTree>,
    values: HostList<'call, StringTree>,
) -> Result<HostCallCompletion<'call, StringTree>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let mut index = 0;
    let mut trees = Vec::with_capacity(call.list_len(values));
    while let Some(value) = call.list_item(values, index) {
        trees.push(call.external_payload(value).tree.clone());
        index += 1;
    }
    let value = call.create_external(StringTreePayload {
        tree: StoredStringTree::sequence(trees),
    });
    Ok(call.return_value(value))
}

pub(super) fn from_string<'call, Profile>(
    mut call: HostCall<'call, Profile, StringTreeProvider<Profile>, StringTree>,
    string: EcoString,
) -> Result<HostCallCompletion<'call, StringTree>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let value = call.create_external(StringTreePayload {
        tree: StoredStringTree::text(string),
    });
    Ok(call.return_value(value))
}

pub(super) fn to_string<'call, Profile>(
    call: HostCall<'call, Profile, StringTreeProvider<Profile>, EcoString>,
    tree: HostExternal<'call, StringTree>,
) -> Result<HostCallCompletion<'call, EcoString>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let string = call.external_payload(tree).tree.flatten();
    Ok(call.return_value(string))
}

pub(super) fn byte_size<'call, Profile>(
    call: HostCall<'call, Profile, StringTreeProvider<Profile>, BigInt>,
    tree: HostExternal<'call, StringTree>,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let size = BigInt::from(call.external_payload(tree).tree.byte_len());
    Ok(call.return_value(size))
}

pub(super) fn lowercase<'call, Profile>(
    mut call: HostCall<'call, Profile, StringTreeProvider<Profile>, StringTree>,
    tree: HostExternal<'call, StringTree>,
) -> Result<HostCallCompletion<'call, StringTree>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let value = call.external_payload(tree).tree.flatten().to_lowercase();
    let value = call.create_external(StringTreePayload {
        tree: StoredStringTree::text(value),
    });
    Ok(call.return_value(value))
}

pub(super) fn uppercase<'call, Profile>(
    mut call: HostCall<'call, Profile, StringTreeProvider<Profile>, StringTree>,
    tree: HostExternal<'call, StringTree>,
) -> Result<HostCallCompletion<'call, StringTree>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let value = call.external_payload(tree).tree.flatten().to_uppercase();
    let value = call.create_external(StringTreePayload {
        tree: StoredStringTree::text(value),
    });
    Ok(call.return_value(value))
}

pub(super) fn do_to_graphemes<'call, Profile>(
    call: HostCall<'call, Profile, StringTreeProvider<Profile>, StringList>,
    string: EcoString,
) -> Result<HostCallCompletion<'call, StringList>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    Ok(call.return_list(
        string
            .graphemes(true)
            .map(EcoString::from)
            .collect::<Vec<_>>(),
    ))
}

pub(super) fn erl_split<'call, Profile>(
    mut call: HostCall<'call, Profile, StringTreeProvider<Profile>, StringTreeList>,
    constructions: HostConstructions<'call, SplitConstructions>,
    tree: HostExternal<'call, StringTree>,
    pattern: EcoString,
    _direction: HostCustom<'call, Direction>,
) -> Result<HostCallCompletion<'call, StringTreeList>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let text = call.external_payload(tree).tree.flatten();
    let parts = if pattern.is_empty() {
        vec![text]
    } else {
        text.split(pattern.as_str()).map(EcoString::from).collect()
    };
    let mut values = Vec::with_capacity(parts.len());
    for part in parts {
        values.push(call.construct_external(
            constructions.at::<SplitStringTreeIndex>(),
            StringTreePayload {
                tree: StoredStringTree::text(part),
            },
        ));
    }
    Ok(call.return_list(values))
}

pub(super) fn replace<'call, Profile>(
    mut call: HostCall<'call, Profile, StringTreeProvider<Profile>, StringTree>,
    tree: HostExternal<'call, StringTree>,
    pattern: EcoString,
    substitute: EcoString,
) -> Result<HostCallCompletion<'call, StringTree>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let text = call.external_payload(tree).tree.flatten();
    let replaced = if pattern.is_empty() {
        text
    } else {
        text.replace(pattern.as_str(), substitute.as_str())
    };
    let value = call.create_external(StringTreePayload {
        tree: StoredStringTree::text(replaced),
    });
    Ok(call.return_value(value))
}

pub(super) fn is_equal<'call, Profile>(
    call: HostCall<'call, Profile, StringTreeProvider<Profile>, bool>,
    left: HostExternal<'call, StringTree>,
    right: HostExternal<'call, StringTree>,
) -> Result<HostCallCompletion<'call, bool>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let left = call.external_payload(left).tree.flatten();
    let right = call.external_payload(right).tree.flatten();
    Ok(call.return_value(left == right))
}

pub(super) fn is_empty<'call, Profile>(
    call: HostCall<'call, Profile, StringTreeProvider<Profile>, bool>,
    tree: HostExternal<'call, StringTree>,
) -> Result<HostCallCompletion<'call, bool>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let empty = call.external_payload(tree).tree.byte_len() == 0;
    Ok(call.return_value(empty))
}

#[cfg(test)]
mod tests {
    use super::super::host_provider;
    use super::super::schema::StringTree;
    use super::StringTreeProvider;
    use crate::gleam_stdlib::{GleamStdlibProfile, GleamStdlibRunState};
    use crate::{
        HostCall, HostCallCompletion, HostCallError, HostExternal, HostModule, HostProvider,
        HostProviderSet, HostedExecution, ModuleSource, PackageSource, compile_typed_host_program,
        plan_host_program,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;

    const STRING_TREE_DECLARATIONS: &str = r#"
pub type StringTree

type Direction {
  All
}

@external(erlang, "host", "append_tree")
pub fn append_tree(tree: StringTree, suffix: StringTree) -> StringTree

@external(erlang, "host", "from_strings")
pub fn from_strings(strings: List(String)) -> StringTree

@external(erlang, "host", "concat")
pub fn concat(trees: List(StringTree)) -> StringTree

@external(erlang, "host", "from_string")
pub fn from_string(string: String) -> StringTree

@external(erlang, "host", "to_string")
pub fn to_string(tree: StringTree) -> String

@external(erlang, "host", "byte_size")
pub fn byte_size(tree: StringTree) -> Int

@external(erlang, "host", "lowercase")
pub fn lowercase(tree: StringTree) -> StringTree

@external(erlang, "host", "uppercase")
pub fn uppercase(tree: StringTree) -> StringTree

@external(erlang, "host", "do_to_graphemes")
@external(javascript, "host", "do_to_graphemes")
fn do_to_graphemes(string: String) -> List(String)

@external(erlang, "host", "erl_split")
fn erl_split(tree: StringTree, pattern: String, direction: Direction) -> List(StringTree)

@external(erlang, "host", "replace")
pub fn replace(tree: StringTree, pattern: String, substitute: String) -> StringTree

@external(erlang, "host", "is_equal")
pub fn is_equal(left: StringTree, right: StringTree) -> Bool

@external(erlang, "host", "is_empty")
pub fn is_empty(tree: StringTree) -> Bool

@external(erlang, "host", "test_source_hash")
fn test_source_hash(tree: StringTree) -> Int
"#;

    fn source_hash<'call>(
        call: HostCall<'call, GleamStdlibProfile, StringTreeProvider<GleamStdlibProfile>, BigInt>,
        tree: HostExternal<'call, StringTree>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let hash = BigInt::from(call.source_hash::<StringTree>(tree));
        Ok(call.return_value(hash))
    }

    fn execution(source: &str) -> HostedExecution<GleamStdlibProfile> {
        let source = format!("{STRING_TREE_DECLARATIONS}\n{source}");
        let provider = host_provider::<GleamStdlibProfile>()
            .and_then(|provider| {
                provider.with_scoped_function::<
                    StringTreeProvider<GleamStdlibProfile>,
                    (StringTree,),
                    BigInt,
                    _,
                >("test_source_hash", source_hash)
            })
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
    fn projects_the_complete_stdlib_run_state() {
        let mut state = GleamStdlibRunState::from_seed([0; 32]);
        let projected = <StringTreeProvider<GleamStdlibProfile> as HostProvider<
            GleamStdlibProfile,
        >>::project(&mut state);

        assert!(std::ptr::eq(projected, &state));
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
  assert test_source_hash(segmented) == test_source_hash(from_strings(["a", "b"]))
  assert test_source_hash(segmented) != test_source_hash(flat)
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
