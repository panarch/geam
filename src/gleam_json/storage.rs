use super::schema::JsonSchema;
use super::{GleamJsonHostProfile, json_stores};
use crate::gleam_stdlib::StoredStringTree;
use crate::{
    HostExternalEquality, HostExternalHashing, HostExternalInspection, HostExternalStorage,
    HostExternalStore,
};
use ecow::EcoString;

#[derive(Default)]
pub(super) struct Stores {
    pub(super) values: HostExternalStore<JsonPayload>,
}

pub(super) struct JsonPayload {
    pub(super) tree: StoredStringTree,
}

pub(super) struct JsonStorage;

impl<Profile> HostExternalStorage<Profile, JsonSchema> for JsonStorage
where
    Profile: GleamJsonHostProfile,
{
    type Payload = JsonPayload;

    fn store(stores: &Profile::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &json_stores::<Profile>(stores).json.values
    }

    fn source_equal(
        _context: &HostExternalEquality<'_>,
        left: &Self::Payload,
        right: &Self::Payload,
    ) -> bool {
        left.tree.structurally_equal(&right.tree)
    }

    fn source_hash(_context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        value.tree.structural_hash()
    }

    fn inspect(_context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        format!("{:?}", value.tree.flatten()).into()
    }
}

#[cfg(test)]
mod tests {
    use super::{JsonPayload, JsonSchema, JsonStorage};
    use crate::gleam_json::test_support::{CustomProfile, CustomStores, execution, run_state};
    use crate::gleam_json::{GleamJsonProfile, GleamJsonProfileStores, json_stores};
    use crate::gleam_stdlib::StoredStringTree;
    use crate::{
        HostExternalEquality, HostExternalHashing, HostExternalInspection, HostExternalStorage,
    };

    #[test]
    fn delegates_structural_source_semantics_and_canonical_inspection() {
        let segmented = JsonPayload {
            tree: StoredStringTree::sequence([
                StoredStringTree::text("[".into()),
                StoredStringTree::text("1".into()),
                StoredStringTree::text("]".into()),
            ]),
        };
        let same = JsonPayload {
            tree: StoredStringTree::sequence([
                StoredStringTree::text("[".into()),
                StoredStringTree::text("1".into()),
                StoredStringTree::text("]".into()),
            ]),
        };
        let flat = JsonPayload {
            tree: StoredStringTree::text("[1]".into()),
        };
        let equal =
            |_: &crate::runtime::StoredRuntimeValue, _: &crate::runtime::StoredRuntimeValue| true;
        let hash = |_: &crate::runtime::StoredRuntimeValue| 0;
        let inspect = |_: &crate::runtime::StoredRuntimeValue| "unused".into();
        let stored = crate::runtime::StoredRuntimeValue::test_int(0.into());
        assert_eq!(inspect(&stored), "unused");
        let equality = HostExternalEquality::new(&equal);
        let hashing = HostExternalHashing::new(&hash);
        let inspection = HostExternalInspection::new(&inspect);

        assert!(<JsonStorage as HostExternalStorage<
            GleamJsonProfile,
            JsonSchema,
        >>::source_equal(&equality, &segmented, &same,));
        assert!(!<JsonStorage as HostExternalStorage<
            GleamJsonProfile,
            JsonSchema,
        >>::source_equal(&equality, &segmented, &flat,));
        assert_eq!(
            <JsonStorage as HostExternalStorage<GleamJsonProfile, JsonSchema>>::source_hash(
                &hashing, &segmented,
            ),
            <JsonStorage as HostExternalStorage<GleamJsonProfile, JsonSchema>>::source_hash(
                &hashing, &same,
            ),
        );
        assert_ne!(
            <JsonStorage as HostExternalStorage<GleamJsonProfile, JsonSchema>>::source_hash(
                &hashing, &segmented,
            ),
            <JsonStorage as HostExternalStorage<GleamJsonProfile, JsonSchema>>::source_hash(
                &hashing, &flat,
            ),
        );
        assert_eq!(
            <JsonStorage as HostExternalStorage<GleamJsonProfile, JsonSchema>>::inspect(
                &inspection,
                &segmented,
            ),
            r#""[1]""#,
        );

        let stores = GleamJsonProfileStores::default();
        assert!(std::ptr::eq(
            <JsonStorage as HostExternalStorage<GleamJsonProfile, JsonSchema>>::store(&stores),
            &json_stores::<GleamJsonProfile>(&stores).json.values,
        ));

        let custom = CustomStores::default();
        assert!(std::ptr::eq(
            <JsonStorage as HostExternalStorage<CustomProfile, JsonSchema>>::store(&custom),
            &custom.json.json.values,
        ));
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
