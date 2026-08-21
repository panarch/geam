use super::schema::JsonSchema;
use super::{GleamJsonHostProfile, json_stores};
use crate::{
    HostExternalEquality, HostExternalHashing, HostExternalInspection, HostExternalStorage,
    HostExternalStore,
};
use ecow::EcoString;
use geam_stdlib::provider_support::StoredStringTree;

#[derive(Default)]
pub(super) struct Stores {
    pub(super) values: HostExternalStore<JsonPayload>,
}

pub(super) struct JsonPayload {
    pub(super) tree: StoredStringTree,
}

pub(super) struct JsonStorage;

fn source_equal(left: &JsonPayload, right: &JsonPayload) -> bool {
    left.tree.structurally_equal(&right.tree)
}

fn source_hash(value: &JsonPayload) -> u64 {
    value.tree.structural_hash()
}

fn inspect(value: &JsonPayload) -> EcoString {
    format!("{:?}", value.tree.flatten()).into()
}

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
        source_equal(left, right)
    }

    fn source_hash(_context: &HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
        source_hash(value)
    }

    fn inspect(_context: &HostExternalInspection<'_>, value: &Self::Payload) -> EcoString {
        inspect(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{JsonPayload, JsonSchema, JsonStorage, inspect, source_equal, source_hash};
    use crate::HostExternalStorage;
    use crate::gleam_json::test_support::{CustomProfile, CustomStores, execution, run_state};
    use crate::gleam_json::{GleamJsonProfile, GleamJsonProfileStores, json_stores};
    use geam_stdlib::provider_support::StoredStringTree;

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
        assert!(source_equal(&segmented, &same));
        assert!(!source_equal(&segmented, &flat));
        assert_eq!(source_hash(&segmented), source_hash(&same));
        assert_ne!(source_hash(&segmented), source_hash(&flat));
        assert_eq!(inspect(&segmented), r#""[1]""#);

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
