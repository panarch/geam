use crate::plan::execution::type_::ExternalTypeId;
use crate::runtime::{LocalValues, RuntimeExternalLease, RuntimeValueProfile};

#[derive(Clone)]
pub(crate) struct EvaluatedExternalValue<Profile: RuntimeValueProfile = LocalValues> {
    type_id: ExternalTypeId,
    lease: Profile::ExternalLease,
}

impl<Profile: RuntimeValueProfile> EvaluatedExternalValue<Profile> {
    pub(in crate::runtime) fn new(type_id: ExternalTypeId, lease: Profile::ExternalLease) -> Self {
        Self { type_id, lease }
    }

    pub(in crate::runtime) fn type_id(&self) -> ExternalTypeId {
        self.type_id
    }

    pub(crate) fn lease(&self) -> &Profile::ExternalLease {
        &self.lease
    }

    pub(in crate::runtime) fn into_parts(self) -> (ExternalTypeId, Profile::ExternalLease) {
        (self.type_id, self.lease)
    }
}

impl EvaluatedExternalValue<LocalValues> {
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
}

impl EvaluatedExternalValue<crate::runtime::TransferValues> {
    pub(in crate::runtime) fn source_equal(
        &self,
        context: &crate::runtime::transfer::TransferExternalEquality<'_>,
        other: &Self,
    ) -> bool {
        self.type_id == other.type_id && self.lease.source_equal(context, &other.lease)
    }

    pub(in crate::runtime) fn source_hash(
        &self,
        context: &crate::runtime::transfer::TransferExternalHashing<'_>,
    ) -> u64 {
        self.lease.source_hash(context)
    }
}

impl<Profile: RuntimeValueProfile> PartialEq for EvaluatedExternalValue<Profile> {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id && self.lease.identity() == other.lease.identity()
    }
}

impl<Profile: RuntimeValueProfile> std::fmt::Debug for EvaluatedExternalValue<Profile> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("EvaluatedExternalValue")
            .field("type_id", &self.type_id)
            .field("identity", &self.lease.identity())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::EvaluatedExternalValue;
    use crate::host::HostExternalStore;
    use crate::plan::execution::type_::ExternalTypeId;
    use crate::runtime::transfer::{
        TransferExternalEquality, TransferExternalHashing, TransferExternalInspection,
        TransferExternalStore, TransferStoredRuntimeValue,
    };
    use crate::runtime::{EvaluatedValue, TransferValues};

    fn transfer_equal(context: &TransferExternalEquality<'_>, left: &usize, right: &usize) -> bool {
        let left = TransferStoredRuntimeValue::new(EvaluatedValue::Int((*left).into()));
        let right = TransferStoredRuntimeValue::new(EvaluatedValue::Int((*right).into()));
        context.stored_values_equal(&left, &right)
    }

    fn transfer_hash(context: &TransferExternalHashing<'_>, value: &usize) -> u64 {
        context.stored_value_hash(&TransferStoredRuntimeValue::new(EvaluatedValue::Int(
            (*value).into(),
        )))
    }

    fn transfer_inspect(
        context: &TransferExternalInspection<'_>,
        value: &usize,
    ) -> ecow::EcoString {
        context.inspect_stored_value(&TransferStoredRuntimeValue::new(EvaluatedValue::Int(
            (*value).into(),
        )))
    }

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
        let first: EvaluatedExternalValue =
            EvaluatedExternalValue::new(ExternalTypeId::new(0), first);
        let second: EvaluatedExternalValue =
            EvaluatedExternalValue::new(ExternalTypeId::new(0), second);
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

    #[test]
    fn transferred_external_value_preserves_source_semantics_and_runtime_identity() {
        let store = TransferExternalStore::default();
        let first = store.insert(7usize, transfer_equal, transfer_hash, transfer_inspect);
        let second = store.insert(7usize, transfer_equal, transfer_hash, transfer_inspect);
        let first: EvaluatedExternalValue<TransferValues> =
            EvaluatedExternalValue::new(ExternalTypeId::new(0), first);
        let second: EvaluatedExternalValue<TransferValues> =
            EvaluatedExternalValue::new(ExternalTypeId::new(0), second);
        let other_type =
            EvaluatedExternalValue::new(ExternalTypeId::new(1), second.lease().clone());
        let stored_equal = |left: &TransferStoredRuntimeValue,
                            right: &TransferStoredRuntimeValue| {
            left.value() == right.value()
        };
        let equality = TransferExternalEquality::new(&stored_equal);
        let stored_hash = |_: &TransferStoredRuntimeValue| 7;
        let hashing = TransferExternalHashing::new(&stored_hash);
        let stored_inspect = |_: &TransferStoredRuntimeValue| "7".into();
        let inspection = TransferExternalInspection::new(&stored_inspect);

        assert_ne!(first, second);
        assert!(first.source_equal(&equality, &second));
        assert!(second.source_equal(&equality, &first));
        assert!(!first.source_equal(&equality, &other_type));
        assert_eq!(first.source_hash(&hashing), 7);
        assert_eq!(first.lease().inspection(&inspection), "7");
    }
}
