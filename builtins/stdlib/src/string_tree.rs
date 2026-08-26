mod function;
mod storage;

pub(super) use provider::__GeamStores as Stores;
pub use provider::{
    __GeamExternalSchema0 as StringTreeSchema, __GeamExternalStorage0 as StringTreeExternalStorage,
    StringTreePayload,
};
pub use storage::StringTree as StoredStringTree;

use super::{Component, GleamStdlibHostProfile};
use crate::{HostExternalType, HostProviderModule, HostRegistrationError, stdlib_stores};
use ecow::EcoString;
use geam_core::provider::ExternalPayload;
use num_bigint::BigInt;

pub type StringTree = HostExternalType<StringTreeSchema>;

fn stores<Profile>(stores: &Profile::ExternalStores) -> &Stores
where
    Profile: GleamStdlibHostProfile,
{
    &stdlib_stores::<Profile>(stores).string_tree
}

#[geam_macros::module(
    path = "gleam/string_tree",
    crate_path = geam_core,
    profile = crate::GleamStdlibHostProfile,
    component = crate::Component<Profile::Io>,
    stores = super::stores,
)]
mod provider {
    use super::{BigInt, EcoString, ExternalPayload, StoredStringTree, function};

    #[geam_macros::external(name = "StringTree", manual)]
    pub struct StringTreePayload {
        tree: StoredStringTree,
    }

    impl StringTreePayload {
        pub fn from_stored(tree: StoredStringTree) -> Self {
            Self { tree }
        }

        pub(super) fn stored(&self) -> &StoredStringTree {
            &self.tree
        }
    }

    impl ExternalPayload for StringTreePayload {
        fn source_equal(&self, other: &Self) -> bool {
            self.tree.structurally_equal(&other.tree)
        }

        fn source_hash(&self) -> u64 {
            self.tree.structural_hash()
        }

        fn inspect(&self) -> EcoString {
            self.tree.inspect()
        }
    }

    #[geam_macros::custom(input = DirectionInput)]
    #[allow(dead_code)]
    pub(super) enum Direction {
        All,
    }

    #[geam_macros::function]
    fn append_tree(tree: &StringTreePayload, suffix: &StringTreePayload) -> StringTreePayload {
        function::append_tree(tree, suffix)
    }

    #[geam_macros::function]
    fn from_strings(strings: geam_core::provider::List<EcoString>) -> StringTreePayload {
        let mut trees = Vec::with_capacity(strings.len());
        let mut index = 0;
        while let Some(string) = strings.get(index) {
            trees.push(StoredStringTree::text(string));
            index += 1;
        }
        StringTreePayload::from_stored(StoredStringTree::sequence(trees))
    }

    #[geam_macros::function]
    fn concat(trees: geam_core::provider::List<StringTreePayload>) -> StringTreePayload {
        let mut stored = Vec::with_capacity(trees.len());
        let mut index = 0;
        while let Some(tree) = trees.get(index) {
            stored.push(tree.stored().clone());
            index += 1;
        }
        StringTreePayload::from_stored(StoredStringTree::sequence(stored))
    }

    #[geam_macros::function]
    fn from_string(string: EcoString) -> StringTreePayload {
        function::from_string(string)
    }

    #[geam_macros::function]
    fn to_string(tree: &StringTreePayload) -> EcoString {
        function::to_string(tree)
    }

    #[geam_macros::function]
    fn byte_size(tree: &StringTreePayload) -> BigInt {
        function::byte_size(tree)
    }

    #[geam_macros::function]
    fn lowercase(tree: &StringTreePayload) -> StringTreePayload {
        function::lowercase(tree)
    }

    #[geam_macros::function]
    fn uppercase(tree: &StringTreePayload) -> StringTreePayload {
        function::uppercase(tree)
    }

    #[geam_macros::function]
    fn do_to_graphemes(string: EcoString) -> Vec<EcoString> {
        function::do_to_graphemes(string)
    }

    #[geam_macros::function]
    fn erl_split(
        tree: &StringTreePayload,
        pattern: EcoString,
        _direction: DirectionInput,
    ) -> Vec<StringTreePayload> {
        function::erl_split(tree, pattern)
    }

    #[geam_macros::function]
    fn replace(
        tree: &StringTreePayload,
        pattern: EcoString,
        substitute: EcoString,
    ) -> StringTreePayload {
        function::replace(tree, pattern, substitute)
    }

    #[geam_macros::function]
    fn is_equal(left: &StringTreePayload, right: &StringTreePayload) -> bool {
        function::is_equal(left, right)
    }

    #[geam_macros::function]
    fn is_empty(tree: &StringTreePayload) -> bool {
        function::is_empty(tree)
    }
}

pub(super) fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamStdlibHostProfile,
{
    provider::__geam_module::<Profile>()
}

#[cfg(test)]
pub(super) const STRING_TREE_DECLARATIONS: &str = r#"
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
"#;

#[cfg(test)]
mod tests {
    use super::{StoredStringTree, StringTreePayload, StringTreeSchema, host_provider};
    use crate::{GleamStdlibProfile, HostExternalTypeSchema};
    use geam_core::provider::ExternalPayload;

    #[test]
    fn delegates_payload_semantics_to_the_persistent_string_tree() {
        let segmented = StringTreePayload::from_stored(StoredStringTree::sequence([
            StoredStringTree::text("a".into()),
            StoredStringTree::text("b".into()),
        ]));
        let same = StringTreePayload::from_stored(StoredStringTree::sequence([
            StoredStringTree::text("a".into()),
            StoredStringTree::text("b".into()),
        ]));
        let flat = StringTreePayload::from_stored(StoredStringTree::text("ab".into()));

        assert!(segmented.source_equal(&same));
        assert!(!segmented.source_equal(&flat));
        assert_eq!(segmented.source_hash(), same.source_hash());
        assert_ne!(segmented.source_hash(), flat.source_hash());
        assert_eq!(segmented.inspect(), r#"string_tree.from_string("ab")"#);
    }

    #[test]
    fn registers_the_exact_official_string_tree_provider_inventory() {
        let provider = host_provider::<GleamStdlibProfile>()
            .expect("official string tree provider should register");

        assert_eq!(provider.package(), "gleam_stdlib");
        assert_eq!(provider.module(), "gleam/string_tree");
        assert_eq!(
            provider.external_types().cloned().collect::<Vec<_>>(),
            [HostExternalTypeSchema::of::<StringTreeSchema>()],
        );
        assert_eq!(
            provider
                .functions()
                .map(|function| function.name().as_str())
                .collect::<Vec<_>>(),
            [
                "append_tree",
                "from_strings",
                "concat",
                "from_string",
                "to_string",
                "byte_size",
                "lowercase",
                "uppercase",
                "do_to_graphemes",
                "erl_split",
                "replace",
                "is_equal",
                "is_empty",
            ],
        );
    }
}
