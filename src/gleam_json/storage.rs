use super::GleamJsonHostProfile;
use super::schema::JsonSchema;
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

impl<Profile> HostExternalStorage<JsonSchema> for Profile
where
    Profile: GleamJsonHostProfile,
{
    type Payload = JsonPayload;

    fn store(stores: &Self::ExternalStores) -> &HostExternalStore<Self::Payload> {
        &Profile::gleam_json_stores(stores).json.values
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
    use super::{JsonPayload, JsonSchema};
    use crate::gleam_json::{GleamJsonHostProfile, GleamJsonProfile, GleamJsonProfileStores};
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

        assert!(
            <GleamJsonProfile as HostExternalStorage<JsonSchema>>::source_equal(
                &equality, &segmented, &same,
            )
        );
        assert!(
            !<GleamJsonProfile as HostExternalStorage<JsonSchema>>::source_equal(
                &equality, &segmented, &flat,
            )
        );
        assert_eq!(
            <GleamJsonProfile as HostExternalStorage<JsonSchema>>::source_hash(
                &hashing, &segmented,
            ),
            <GleamJsonProfile as HostExternalStorage<JsonSchema>>::source_hash(&hashing, &same),
        );
        assert_ne!(
            <GleamJsonProfile as HostExternalStorage<JsonSchema>>::source_hash(
                &hashing, &segmented,
            ),
            <GleamJsonProfile as HostExternalStorage<JsonSchema>>::source_hash(&hashing, &flat),
        );
        assert_eq!(
            <GleamJsonProfile as HostExternalStorage<JsonSchema>>::inspect(&inspection, &segmented,),
            r#""[1]""#,
        );

        let stores = GleamJsonProfileStores::default();
        assert!(std::ptr::eq(
            <GleamJsonProfile as HostExternalStorage<JsonSchema>>::store(&stores),
            &GleamJsonProfile::gleam_json_stores(&stores).json.values,
        ));
    }
}
