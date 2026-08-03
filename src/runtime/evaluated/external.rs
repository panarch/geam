use crate::host::ExternalPayloadLease;
use crate::plan::execution::type_::ExternalTypeId;

#[derive(Clone)]
pub(in crate::runtime) struct EvaluatedExternalValue {
    type_id: ExternalTypeId,
    lease: ExternalPayloadLease,
}

impl EvaluatedExternalValue {
    pub(in crate::runtime) fn new(type_id: ExternalTypeId, lease: ExternalPayloadLease) -> Self {
        Self { type_id, lease }
    }

    pub(in crate::runtime) fn type_id(&self) -> ExternalTypeId {
        self.type_id
    }

    pub(in crate::runtime) fn lease(&self) -> &ExternalPayloadLease {
        &self.lease
    }

    pub(in crate::runtime) fn source_equal(
        &self,
        context: &crate::host::HostExternalEquality<'_>,
        other: &Self,
    ) -> bool {
        self.type_id == other.type_id && self.lease.source_equal(context, &other.lease)
    }

    pub(in crate::runtime) fn source_hash(
        &self,
        context: &crate::host::HostExternalHashing<'_>,
    ) -> u64 {
        self.lease.source_hash(context)
    }

    pub(in crate::runtime) fn into_parts(self) -> (ExternalTypeId, ExternalPayloadLease) {
        (self.type_id, self.lease)
    }
}

impl PartialEq for EvaluatedExternalValue {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id && self.lease.id() == other.lease.id()
    }
}

impl std::fmt::Debug for EvaluatedExternalValue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("EvaluatedExternalValue")
            .field("type_id", &self.type_id)
            .field("identity", &self.lease.id())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::EvaluatedExternalValue;
    use crate::host::HostExternalStore;
    use crate::plan::execution::type_::ExternalTypeId;

    #[test]
    fn evaluated_external_value_separates_source_equality_from_runtime_identity() {
        let store = HostExternalStore::default();
        let source_hash = |_: &crate::host::HostExternalHashing<'_>, value: &usize| *value as u64;
        let inspect = |context: &crate::host::HostExternalInspection<'_>, value: &usize| {
            let stored = crate::host::HostStoredValue::<num_bigint::BigInt>::new(
                crate::runtime::StoredRuntimeValue::test_int((*value).into()),
            );
            format!("Resource({})", context.inspect_stored_value(&stored)).into()
        };
        let first = store.insert(7usize, |_, left, right| left == right, source_hash, inspect);
        let second = store.insert(7usize, |_, left, right| left == right, source_hash, inspect);
        let first = EvaluatedExternalValue::new(ExternalTypeId::new(0), first);
        let second = EvaluatedExternalValue::new(ExternalTypeId::new(0), second);
        let other_type =
            EvaluatedExternalValue::new(ExternalTypeId::new(1), second.lease().clone());
        let stored_equal =
            |_: &crate::runtime::StoredRuntimeValue, _: &crate::runtime::StoredRuntimeValue| false;
        let equality = crate::host::HostExternalEquality::new(&stored_equal);
        let stored_hash = |_: &crate::runtime::StoredRuntimeValue| 17;
        let stored_inspect = |_: &crate::runtime::StoredRuntimeValue| "7".into();
        let hashing = crate::host::HostExternalHashing::new(&stored_hash);
        let inspection = crate::host::HostExternalInspection::new(&stored_inspect);

        assert_ne!(first, second);
        assert!(first.source_equal(&equality, &second));
        assert!(second.source_equal(&equality, &first));
        assert!(!first.source_equal(&equality, &other_type));
        assert_eq!(first.source_hash(&hashing), 7);
        assert_eq!(first.lease().inspection(&inspection), "Resource(7)");
        assert_eq!(second.lease().inspection(&inspection), "Resource(7)");
        assert!(format!("{first:?}").contains("EvaluatedExternalValue"));

        let (type_id, lease) = first.clone().into_parts();
        assert_eq!(type_id, ExternalTypeId::new(0));
        assert_eq!(lease.id(), first.lease().id());
        assert_eq!(first.type_id(), ExternalTypeId::new(0));
    }
}
