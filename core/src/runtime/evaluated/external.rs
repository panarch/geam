pub(in crate::runtime) mod source;

use crate::plan::execution::type_::ExternalTypeId;

#[derive(Clone)]
pub(crate) struct EvaluatedExternalValue {
    type_id: ExternalTypeId,
    lease: crate::runtime::ExternalPayloadLease,
}

impl EvaluatedExternalValue {
    pub(in crate::runtime) fn new(
        type_id: ExternalTypeId,
        lease: crate::runtime::ExternalPayloadLease,
    ) -> Self {
        Self { type_id, lease }
    }

    pub(in crate::runtime) fn type_id(&self) -> ExternalTypeId {
        self.type_id
    }

    pub(crate) fn lease(&self) -> &crate::runtime::ExternalPayloadLease {
        &self.lease
    }

    pub(in crate::runtime) fn into_parts(
        self,
    ) -> (ExternalTypeId, crate::runtime::ExternalPayloadLease) {
        (self.type_id, self.lease)
    }

    pub(in crate::runtime) fn source_equal(
        &self,
        context: &crate::runtime::retained::RetainedValueEquality<'_>,
        other: &Self,
    ) -> bool {
        self.type_id == other.type_id && self.lease.source_equal(context, &other.lease)
    }

    pub(in crate::runtime) fn source_hash(
        &self,
        context: &crate::runtime::retained::RetainedValueHashing<'_>,
    ) -> u64 {
        self.lease.source_hash(context)
    }
}

impl PartialEq for EvaluatedExternalValue {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id && self.lease.identity() == other.lease.identity()
    }
}

impl std::fmt::Debug for EvaluatedExternalValue {
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
    use crate::runtime::EvaluatedValue;
    use crate::runtime::retained::{
        RetainedValueEquality, RetainedValueHashing, RetainedValueInspection, RetainedValueRef,
    };

    fn transfer_equal(
        context: &crate::host::HostExternalEquality<'_>,
        left: &usize,
        right: &usize,
    ) -> bool {
        let left = EvaluatedValue::Int((*left).into());
        let right = EvaluatedValue::Int((*right).into());
        context.0.stored_values_equal(
            &RetainedValueRef::new(&left),
            &RetainedValueRef::new(&right),
        )
    }

    fn transfer_hash(context: &crate::host::HostExternalHashing<'_>, value: &usize) -> u64 {
        context
            .0
            .stored_value_hash(&RetainedValueRef::new(&EvaluatedValue::Int(
                (*value).into(),
            )))
    }

    fn transfer_inspect(
        context: &crate::host::HostExternalInspection<'_>,
        value: &usize,
    ) -> ecow::EcoString {
        context
            .0
            .inspect_stored_value(&RetainedValueRef::new(&EvaluatedValue::Int(
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
        let first: EvaluatedExternalValue =
            EvaluatedExternalValue::new(ExternalTypeId::new(0), first);
        let second: EvaluatedExternalValue =
            EvaluatedExternalValue::new(ExternalTypeId::new(0), second);
        let other_type =
            EvaluatedExternalValue::new(ExternalTypeId::new(1), second.lease().clone());
        let stored_equal =
            |_: &crate::runtime::RetainedValueRef, _: &crate::runtime::RetainedValueRef| false;
        let equality = crate::host::RetainedValueEquality::new(&stored_equal);
        let stored_hash = |_: &crate::runtime::RetainedValueRef| 17;
        let stored_inspect = |_: &crate::runtime::RetainedValueRef| "7".into();
        let hashing = crate::host::RetainedValueHashing::new(&stored_hash);
        let inspection = crate::host::RetainedValueInspection::new(&stored_inspect);

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
        assert_eq!(lease.identity(), first.lease().identity());
        assert_eq!(first.type_id(), ExternalTypeId::new(0));
    }

    #[test]
    fn transferred_external_value_preserves_source_semantics_and_runtime_identity() {
        let store = crate::host::HostExternalStore::default();
        let first = store.insert(
            7usize,
            transfer_equal,
            transfer_hash,
            transfer_inspect,
            |_| None,
        );
        let second = store.insert(
            7usize,
            transfer_equal,
            transfer_hash,
            transfer_inspect,
            |_| None,
        );
        let first: EvaluatedExternalValue =
            EvaluatedExternalValue::new(ExternalTypeId::new(0), first);
        let second: EvaluatedExternalValue =
            EvaluatedExternalValue::new(ExternalTypeId::new(0), second);
        let other_type =
            EvaluatedExternalValue::new(ExternalTypeId::new(1), second.lease().clone());
        let stored_equal =
            |left: &RetainedValueRef, right: &RetainedValueRef| left.value() == right.value();
        let equality = RetainedValueEquality::new(&stored_equal);
        let stored_hash = |_: &RetainedValueRef| 7;
        let hashing = RetainedValueHashing::new(&stored_hash);
        let stored_inspect = |_: &RetainedValueRef| "7".into();
        let inspection = RetainedValueInspection::new(&stored_inspect);

        assert_ne!(first, second);
        assert!(first.source_equal(&equality, &second));
        assert!(second.source_equal(&equality, &first));
        assert!(!first.source_equal(&equality, &other_type));
        assert_eq!(first.source_hash(&hashing), 7);
        assert_eq!(first.lease().inspection(&inspection), "7");
    }
}
