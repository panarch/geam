mod function;
mod schema;
mod storage;

pub(in crate::gleam_stdlib) use schema::{StringTree, StringTreeSchema};
pub(super) use storage::{Stores, StringTree as StoredStringTree, StringTreePayload};

use self::function::{
    StringTreeProvider, append_tree, byte_size, concat, do_to_graphemes, erl_split, from_string,
    from_strings, is_empty, is_equal, lowercase, replace, to_string, uppercase,
};
use self::schema::{Direction, StringList, StringTreeList};
use super::GleamStdlibHostProfile;
use crate::{HostProviderModule, HostRegistrationError};
use ecow::EcoString;
use num_bigint::BigInt;

pub(super) fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamStdlibHostProfile,
{
    HostProviderModule::new("gleam_stdlib", "gleam/string_tree")
        .and_then(HostProviderModule::with_external_type::<StringTreeSchema>)
        .and_then(|provider| {
            provider.with_scoped_function::<
                StringTreeProvider<Profile>,
                (StringTree, StringTree),
                StringTree,
                _,
            >("append_tree", append_tree::<Profile>)
        })
        .and_then(|provider| {
            provider
                .with_scoped_function::<StringTreeProvider<Profile>, (StringList,), StringTree, _>(
                    "from_strings",
                    from_strings::<Profile>,
                )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<
                StringTreeProvider<Profile>,
                (StringTreeList,),
                StringTree,
                _,
            >("concat", concat::<Profile>)
        })
        .and_then(|provider| {
            provider
                .with_scoped_function::<StringTreeProvider<Profile>, (EcoString,), StringTree, _>(
                    "from_string",
                    from_string::<Profile>,
                )
        })
        .and_then(|provider| {
            provider
                .with_scoped_function::<StringTreeProvider<Profile>, (StringTree,), EcoString, _>(
                    "to_string",
                    to_string::<Profile>,
                )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<StringTreeProvider<Profile>, (StringTree,), BigInt, _>(
                "byte_size",
                byte_size::<Profile>,
            )
        })
        .and_then(|provider| {
            provider
                .with_scoped_function::<StringTreeProvider<Profile>, (StringTree,), StringTree, _>(
                    "lowercase",
                    lowercase::<Profile>,
                )
        })
        .and_then(|provider| {
            provider
                .with_scoped_function::<StringTreeProvider<Profile>, (StringTree,), StringTree, _>(
                    "uppercase",
                    uppercase::<Profile>,
                )
        })
        .and_then(|provider| {
            provider
                .with_scoped_function::<StringTreeProvider<Profile>, (EcoString,), StringList, _>(
                    "do_to_graphemes",
                    do_to_graphemes::<Profile>,
                )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<
                StringTreeProvider<Profile>,
                (StringTree, EcoString, Direction),
                StringTreeList,
                _,
            >("erl_split", erl_split::<Profile>)
        })
        .and_then(|provider| {
            provider.with_scoped_function::<
                StringTreeProvider<Profile>,
                (StringTree, EcoString, EcoString),
                StringTree,
                _,
            >("replace", replace::<Profile>)
        })
        .and_then(|provider| {
            provider.with_scoped_function::<
                StringTreeProvider<Profile>,
                (StringTree, StringTree),
                bool,
                _,
            >("is_equal", is_equal::<Profile>)
        })
        .and_then(|provider| {
            provider.with_scoped_function::<StringTreeProvider<Profile>, (StringTree,), bool, _>(
                "is_empty",
                is_empty::<Profile>,
            )
        })
}

#[cfg(test)]
mod tests {
    use super::host_provider;
    use crate::gleam_stdlib::GleamStdlibProfile;

    #[test]
    fn registers_the_exact_official_string_tree_provider_inventory() {
        let provider = host_provider::<GleamStdlibProfile>()
            .expect("official string tree provider should register");

        assert_eq!(provider.package(), "gleam_stdlib");
        assert_eq!(provider.module(), "gleam/string_tree");
        assert_eq!(
            provider
                .external_types()
                .map(|schema| (schema.name().as_str(), schema.parameter_count()))
                .collect::<Vec<_>>(),
            [("StringTree", 0)],
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
