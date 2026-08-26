mod decode;
mod encode;

pub(super) use provider::__GeamStores as Stores;

use crate::{Component, GleamJsonHostProfile};

#[geam_macros::module(
    path = "gleam/json",
    crate_path = geam_core,
    profile = crate::GleamJsonHostProfile,
    component = crate::Component,
    stores = crate::provider_stores,
)]
pub(super) mod provider {
    use super::{decode, encode};
    use crate::BitArrayValue;
    use ecow::EcoString;
    use geam_core::provider::{ExternalPayload, HostResult};
    use geam_stdlib::provider_support::StoredStringTree;
    use num_bigint::BigInt;

    #[geam_macros::external(name = "Json", manual)]
    pub struct JsonPayload {
        tree: StoredStringTree,
    }

    impl JsonPayload {
        pub(crate) fn from_tree(tree: StoredStringTree) -> Self {
            Self { tree }
        }

        pub(crate) fn tree(&self) -> &StoredStringTree {
            &self.tree
        }
    }

    impl ExternalPayload for JsonPayload {
        fn source_equal(&self, other: &Self) -> bool {
            self.tree.structurally_equal(&other.tree)
        }

        fn source_hash(&self) -> u64 {
            self.tree.structural_hash()
        }

        fn inspect(&self) -> EcoString {
            format!("{:?}", self.tree.flatten()).into()
        }
    }

    #[geam_macros::custom]
    #[allow(dead_code)]
    pub(crate) enum DecodeError {
        UnexpectedEndOfInput,
        UnexpectedByte(EcoString),
        UnexpectedSequence(EcoString),
        UnableToDecode(Vec<geam_stdlib::provider_support::DynamicDecodeErrorValue>),
    }

    #[geam_macros::function]
    fn decode_to_dynamic(json: BitArrayValue) -> decode::DecodeOutput {
        decode::decode_to_dynamic(json)
    }

    #[geam_macros::function]
    fn do_to_string(json: &JsonPayload) -> EcoString {
        encode::do_to_string(json)
    }

    #[geam_macros::function]
    fn to_string_tree(json: &JsonPayload) -> geam_stdlib::provider_support::StringTreePayload {
        encode::to_string_tree(json)
    }

    #[geam_macros::function]
    fn do_string(value: EcoString) -> JsonPayload {
        encode::do_string(value)
    }

    #[geam_macros::function]
    fn do_bool(value: bool) -> JsonPayload {
        encode::do_bool(value)
    }

    #[geam_macros::function]
    fn do_int(value: BigInt) -> JsonPayload {
        encode::do_int(value)
    }

    #[geam_macros::function]
    fn do_float(value: f64) -> HostResult<JsonPayload> {
        encode::do_float(value)
    }

    #[geam_macros::function]
    fn do_null() -> JsonPayload {
        encode::do_null()
    }

    #[geam_macros::function]
    fn do_object(entries: geam_core::provider::List<(EcoString, JsonPayload)>) -> JsonPayload {
        let mut index = 0;
        let mut trees = vec![StoredStringTree::text("{".into())];
        while let Some((key, value)) = entries.get(index) {
            if index != 0 {
                trees.push(StoredStringTree::text(",".into()));
            }
            trees.push(StoredStringTree::text(encode::encode_string(&key)));
            trees.push(StoredStringTree::text(":".into()));
            trees.push(value.tree().clone());
            index += 1;
        }
        trees.push(StoredStringTree::text("}".into()));
        JsonPayload::from_tree(StoredStringTree::sequence(trees))
    }

    #[geam_macros::function]
    fn do_preprocessed_array(values: geam_core::provider::List<JsonPayload>) -> JsonPayload {
        let mut index = 0;
        let mut trees = vec![StoredStringTree::text("[".into())];
        while let Some(value) = values.get(index) {
            if index != 0 {
                trees.push(StoredStringTree::text(",".into()));
            }
            trees.push(value.tree().clone());
            index += 1;
        }
        trees.push(StoredStringTree::text("]".into()));
        JsonPayload::from_tree(StoredStringTree::sequence(trees))
    }
}

pub(super) fn host_provider<Profile>()
-> Result<crate::HostProviderModule<Profile>, crate::HostRegistrationError>
where
    Profile: GleamJsonHostProfile,
{
    provider::__geam_module::<Profile>()
}

#[cfg(test)]
type Json = crate::HostExternalType<provider::__GeamExternalSchema0>;

#[cfg(test)]
impl
    crate::HostExternalBinding<
        crate::GleamJsonProfile,
        geam_stdlib::provider_support::DynamicSchema,
    > for provider::__GeamProvider
{
    type Storage = geam_stdlib::provider_support::DynamicExternalStorage;
}

#[cfg(test)]
impl crate::HostExternalBinding<crate::GleamJsonProfile, geam_stdlib::provider_support::DictSchema>
    for provider::__GeamProvider
{
    type Storage = geam_stdlib::provider_support::DictExternalStorage;
}

#[cfg(test)]
impl
    crate::HostExternalBinding<
        crate::GleamJsonProfile,
        geam_stdlib::provider_support::StringTreeSchema,
    > for provider::__GeamProvider
{
    type Storage = geam_stdlib::provider_support::StringTreeExternalStorage;
}

#[cfg(test)]
fn json_source_hash<'call>(
    call: crate::HostCall<
        'call,
        crate::GleamJsonProfile,
        provider::__GeamProvider,
        num_bigint::BigInt,
    >,
    json: crate::HostExternal<'call, Json>,
) -> Result<crate::HostCallCompletion<'call, num_bigint::BigInt>, crate::HostCallError> {
    let hash = num_bigint::BigInt::from(call.source_hash::<Json>(json));
    Ok(call.return_value(hash))
}

#[cfg(test)]
pub(super) fn test_host_providers() -> [crate::HostProviderModule<crate::GleamJsonProfile>; 4] {
    use geam_stdlib::provider_support::{DictSchema, DynamicSchema, StringTreeSchema};

    let json = host_provider::<crate::GleamJsonProfile>()
        .expect("official JSON provider should register")
        .with_scoped_function::<provider::__GeamProvider, (Json,), num_bigint::BigInt, _>(
            "test_source_hash",
            json_source_hash,
        )
        .expect("synthetic JSON hash function should register");

    [
        crate::HostProviderModule::new("gleam_stdlib", "gleam/dynamic")
            .and_then(crate::HostProviderModule::with_external_type::<
                provider::__GeamProvider,
                DynamicSchema,
            >)
            .expect("synthetic Dynamic declaration should register"),
        crate::HostProviderModule::new("gleam_stdlib", "gleam/dict")
            .and_then(crate::HostProviderModule::with_external_type::<
                provider::__GeamProvider,
                DictSchema,
            >)
            .expect("synthetic Dict declaration should register"),
        crate::HostProviderModule::new("gleam_stdlib", "gleam/string_tree")
            .and_then(crate::HostProviderModule::with_external_type::<
                provider::__GeamProvider,
                StringTreeSchema,
            >)
            .expect("synthetic StringTree declaration should register"),
        json,
    ]
}

#[cfg(test)]
mod tests {
    use super::provider::{
        __GeamExternalSchema0 as JsonSchema, __GeamExternalStorage0 as JsonStorage,
        __GeamProvider as JsonProvider, JsonPayload,
    };
    use crate::test_support::{CustomProfile, CustomStores, execution, run_state};
    use crate::{GleamJsonProfile, GleamJsonProfileStores, HostExternalStorage, HostProvider};
    use geam_core::provider::ExternalPayload;
    use geam_stdlib::provider_support::StoredStringTree;

    #[test]
    fn projects_only_the_json_component_state() {
        let mut state = run_state([0; 32]);
        let json = &mut state.json as *mut ();
        let projected = <JsonProvider as HostProvider<GleamJsonProfile>>::project(&mut state);

        assert!(std::ptr::eq(projected, json));
    }

    #[test]
    fn json_payload_owns_structural_source_semantics_and_canonical_inspection() {
        let segmented = JsonPayload::from_tree(StoredStringTree::sequence([
            StoredStringTree::text("[".into()),
            StoredStringTree::text("1".into()),
            StoredStringTree::text("]".into()),
        ]));
        let same = JsonPayload::from_tree(StoredStringTree::sequence([
            StoredStringTree::text("[".into()),
            StoredStringTree::text("1".into()),
            StoredStringTree::text("]".into()),
        ]));
        let flat = JsonPayload::from_tree(StoredStringTree::text("[1]".into()));

        assert!(segmented.source_equal(&same));
        assert!(!segmented.source_equal(&flat));
        assert_eq!(segmented.source_hash(), same.source_hash());
        assert_ne!(segmented.source_hash(), flat.source_hash());
        assert_eq!(segmented.inspect(), r#""[1]""#);

        let stores = GleamJsonProfileStores::default();
        let first =
            <JsonStorage as HostExternalStorage<GleamJsonProfile, JsonSchema>>::store(&stores);
        let repeated =
            <JsonStorage as HostExternalStorage<GleamJsonProfile, JsonSchema>>::store(&stores);
        assert!(std::ptr::eq(first, repeated));

        let custom = CustomStores::default();
        let custom =
            <JsonStorage as HostExternalStorage<CustomProfile, JsonSchema>>::store(&custom);
        assert!(!std::ptr::eq(first, custom));
    }

    #[test]
    fn escaped_json_remains_self_contained_after_execution_and_state_drop() {
        let value = {
            let execution = execution(
                r#"
pub fn main() {
  do_object([#("items", do_preprocessed_array([do_int(1), do_int(2)]))])
}
"#,
            );
            let mut state = run_state([0; 32]);
            let value = execution
                .run_main(&mut state, &mut Vec::new())
                .expect("JSON value should escape the run");
            drop(state);
            drop(execution);
            value
        };

        assert_eq!(value.inspect().to_string(), r#""{\"items\":[1,2]}""#);
        assert_eq!(value.clone(), value);
    }

    #[test]
    fn json_values_use_structural_hashing_as_dict_keys() {
        let execution = execution(
            r#"
pub fn main() {
  let segmented = do_preprocessed_array([do_int(1)])
  let same = do_preprocessed_array([do_int(1)])
  assert segmented == same
  assert segmented != do_int(1)
  assert test_source_hash(segmented) == test_source_hash(same)
  segmented
}
"#,
        );
        let value = execution
            .run_main(&mut run_state([0; 32]), &mut Vec::new())
            .expect("JSON values should expose their structural source hash");

        assert_eq!(value.inspect().to_string(), r#""[1]""#);
    }

    #[test]
    fn repeated_execution_is_independent_of_the_caller_owned_run_state() {
        let execution = execution(
            r#"
pub fn main() {
  do_object([#("items", do_preprocessed_array([do_int(1), do_int(2)]))])
}
"#,
        );
        let mut first_state = run_state([1; 32]);
        let mut second_state = run_state([2; 32]);

        let first = execution
            .run_main(&mut first_state, &mut Vec::new())
            .expect("first JSON execution should run");
        let repeated = execution
            .run_main(&mut first_state, &mut Vec::new())
            .expect("repeated JSON execution should run");
        let independent = execution
            .run_main(&mut second_state, &mut Vec::new())
            .expect("independent JSON execution should run");

        let first_inspection = first.inspect().to_string();
        assert_ne!(first, repeated);
        assert_ne!(first, independent);
        assert_eq!(first_inspection, repeated.inspect().to_string());
        assert_eq!(first_inspection, independent.inspect().to_string());
        assert_eq!(first_inspection, r#""{\"items\":[1,2]}""#);
    }
}
