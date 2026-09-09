use crate::plan::ExternalType;
use ecow::EcoString;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_EXTERNAL_VALUE_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Clone)]
pub struct ExternalValue {
    type_: ExternalType,
    identity: ExternalValueIdentity,
    inspection: EcoString,
    _lease: crate::host::ExternalPayloadLease,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExternalValueIdentity(u64);

impl ExternalValueIdentity {
    pub(crate) fn allocate_id() -> u64 {
        NEXT_EXTERNAL_VALUE_ID.fetch_add(1, Ordering::Relaxed)
    }
}

impl ExternalValue {
    pub(crate) fn from_evaluated(
        type_: ExternalType,
        lease: crate::host::ExternalPayloadLease,
        inspection: EcoString,
    ) -> Self {
        Self {
            type_,
            identity: ExternalValueIdentity(lease.identity()),
            inspection,
            _lease: lease,
        }
    }

    pub fn type_(&self) -> &ExternalType {
        &self.type_
    }

    pub fn identity(&self) -> ExternalValueIdentity {
        self.identity
    }

    pub fn inspection(&self) -> &EcoString {
        &self.inspection
    }
}

impl PartialEq for ExternalValue {
    fn eq(&self, other: &Self) -> bool {
        self.identity == other.identity
    }
}

impl std::fmt::Debug for ExternalValue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ExternalValue")
            .field("type_", &self.type_)
            .field("identity", &self.identity)
            .field("inspection", &self.inspection)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::{ExternalValue, ExternalValueIdentity};
    use crate::host::HostExternalStore;
    use crate::plan::{ExternalType, ExternalTypeName};
    use crate::runtime::EvaluatedValue;
    use crate::runtime::retained::{
        RetainedValueEquality, RetainedValueHashing, RetainedValueInspection, RetainedValueRef,
    };

    #[test]
    fn opaque_external_value_exposes_identity_and_inspection_without_payload_access() {
        let store = HostExternalStore::default();
        let source_hash = |_: &crate::host::HostExternalHashing<'_>, value: &usize| *value as u64;
        let inspect = |context: &crate::host::HostExternalInspection<'_>, value: &usize| {
            let stored = crate::host::HostStoredValue::<num_bigint::BigInt>::new(
                crate::runtime::StoredRuntimeValue::test_int((*value).into()),
            );
            format!("Resource({})", context.inspect_stored_value(&stored)).into()
        };
        let before = ExternalValueIdentity::allocate_id();
        let first = store.insert(
            7usize,
            |_, left, right| left == right,
            source_hash,
            inspect,
            |_| None,
        );
        let second = store.insert(
            7usize,
            |_, left, right| left == right,
            source_hash,
            inspect,
            |_| None,
        );
        let after = ExternalValueIdentity::allocate_id();
        assert!(before < first.identity());
        assert!(first.identity() < second.identity());
        assert!(second.identity() < after);
        let type_ = ExternalType::new(
            ExternalTypeName::new("domain".into(), "domain/resource".into(), "Resource".into()),
            Vec::new(),
        );
        let stored_equal =
            |_: &crate::runtime::RetainedValueRef, _: &crate::runtime::RetainedValueRef| false;
        let equality = crate::host::RetainedValueEquality::new(&stored_equal);
        let stored_hash = |_: &crate::runtime::RetainedValueRef| 17;
        let stored_inspect = |_: &crate::runtime::RetainedValueRef| "7".into();
        let hashing = crate::host::RetainedValueHashing::new(&stored_hash);
        let inspection = crate::host::RetainedValueInspection::new(&stored_inspect);
        assert!(first.source_equal(&equality, &second));
        assert!(second.source_equal(&equality, &first));
        assert_eq!(first.source_hash(&hashing), 7);
        assert_eq!(first.inspection(&inspection), "Resource(7)");
        assert_eq!(second.inspection(&inspection), "Resource(7)");
        let first = ExternalValue::from_evaluated(type_.clone(), first, "Resource(7)".into());
        let second = ExternalValue::from_evaluated(type_.clone(), second, "Resource(7)".into());
        let cloned = first.clone();
        let debug = format!("{first:?}");

        assert_eq!(first.type_(), &type_);
        assert_eq!(first.inspection(), "Resource(7)");
        assert_eq!(first, cloned);
        assert_ne!(first, second);
        assert_eq!(first.identity(), cloned.identity());
        assert!(debug.contains("ExternalValue"));
        assert!(debug.contains("Resource(7)"));
        assert!(!debug.contains("7usize"));
    }

    #[test]
    fn transferred_external_value_keeps_its_payload_lease_opaque_and_alive() {
        fn equal(_: &crate::host::HostExternalEquality<'_>, left: &usize, right: &usize) -> bool {
            left == right
        }

        fn hash(_: &crate::host::HostExternalHashing<'_>, value: &usize) -> u64 {
            *value as u64
        }

        fn inspect(
            context: &crate::host::HostExternalInspection<'_>,
            value: &usize,
        ) -> ecow::EcoString {
            format!(
                "Resource({})",
                context
                    .0
                    .inspect_stored_value(&RetainedValueRef::new(&EvaluatedValue::Int(
                        (*value).into()
                    )))
            )
            .into()
        }

        let store = crate::host::HostExternalStore::default();
        let before = ExternalValueIdentity::allocate_id();
        let lease = store.insert(7usize, equal, hash, inspect, |_| None);
        let identity = lease.identity();
        let after = ExternalValueIdentity::allocate_id();
        assert!(before < identity);
        assert!(identity < after);
        let stored_equal = |_: &RetainedValueRef, _: &RetainedValueRef| true;
        let equality = RetainedValueEquality::new(&stored_equal);
        let stored_hash = |_: &RetainedValueRef| 0;
        let hashing = RetainedValueHashing::new(&stored_hash);
        let stored_inspect = |_: &RetainedValueRef| "7".into();
        let inspection = RetainedValueInspection::new(&stored_inspect);
        assert!(lease.source_equal(&equality, &lease));
        assert_eq!(lease.source_hash(&hashing), 7);
        assert_eq!(lease.inspection(&inspection), "Resource(7)");
        let type_ = ExternalType::new(
            ExternalTypeName::new("domain".into(), "domain/resource".into(), "Resource".into()),
            Vec::new(),
        );
        let value = ExternalValue::from_evaluated(type_.clone(), lease, "Resource(7)".into());
        let clone = value.clone();
        drop(store);

        assert_eq!(value.type_(), &type_);
        assert_eq!(value.identity().0, identity);
        assert_eq!(value.inspection(), "Resource(7)");
        assert_eq!(value, clone);
    }
}
