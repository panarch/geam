use std::fmt;

#[allow(private_bounds)]
pub(crate) trait RuntimeValueProfile:
    Copy + Clone + Default + fmt::Debug + PartialEq + Eq + 'static
{
    type ListHandle: Clone + fmt::Debug + PartialEq + 'static;
    type ListStorage: crate::runtime::RuntimeListStorage<Self>;
    type ExternalLease: RuntimeExternalLease;
    type PanicSubject: fmt::Debug;
    type Callable: crate::runtime::function::StoredCallable<Self>;

    fn external_values_equal(
        storage: &Self::ListStorage,
        left: &crate::runtime::EvaluatedExternalValue<Self>,
        right: &crate::runtime::EvaluatedExternalValue<Self>,
    ) -> bool;

    fn external_value_source_hash(
        storage: &Self::ListStorage,
        value: &crate::runtime::EvaluatedExternalValue<Self>,
    ) -> u64;
}

pub(crate) trait RuntimeExternalLease: Clone + 'static {
    fn identity(&self) -> u64;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct LocalValues;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct TransferValues;

impl RuntimeValueProfile for LocalValues {
    type ListHandle = crate::runtime::state::list::ListHandleCore;
    type ListStorage = crate::runtime::state::list::RuntimeListStorage;
    type ExternalLease = crate::host::ExternalPayloadLease;
    type PanicSubject = crate::Value;
    type Callable = crate::runtime::function::LocalCallable;

    fn external_values_equal(
        storage: &Self::ListStorage,
        left: &crate::runtime::EvaluatedExternalValue<Self>,
        right: &crate::runtime::EvaluatedExternalValue<Self>,
    ) -> bool {
        let equal = |left: &crate::runtime::StoredRuntimeValue,
                     right: &crate::runtime::StoredRuntimeValue| {
            crate::runtime::evaluated::values_equal(storage, left.value(), right.value())
        };
        left.source_equal(&crate::host::HostExternalEquality::new(&equal), right)
    }

    fn external_value_source_hash(
        storage: &Self::ListStorage,
        value: &crate::runtime::EvaluatedExternalValue<Self>,
    ) -> u64 {
        let source_hash = |value: &crate::runtime::StoredRuntimeValue| {
            crate::runtime::evaluated::value_source_hash(storage, value.value())
        };
        value.source_hash(&crate::host::HostExternalHashing::new(&source_hash))
    }
}

impl RuntimeValueProfile for TransferValues {
    type ListHandle = crate::runtime::transfer::TransferListHandleCore;
    type ListStorage = crate::runtime::transfer::TransferListStorage;
    type ExternalLease = crate::runtime::transfer::TransferExternalPayloadLease;
    type PanicSubject = crate::AsyncPanicValue;
    type Callable = crate::runtime::TransferCallable;

    fn external_values_equal(
        storage: &Self::ListStorage,
        left: &crate::runtime::EvaluatedExternalValue<Self>,
        right: &crate::runtime::EvaluatedExternalValue<Self>,
    ) -> bool {
        let equal =
            |left: &crate::runtime::transfer::TransferStoredRuntimeValue,
             right: &crate::runtime::transfer::TransferStoredRuntimeValue| {
                crate::runtime::evaluated::values_equal(storage, left.value(), right.value())
            };
        left.source_equal(
            &crate::runtime::transfer::TransferExternalEquality::new(&equal),
            right,
        )
    }

    fn external_value_source_hash(
        storage: &Self::ListStorage,
        value: &crate::runtime::EvaluatedExternalValue<Self>,
    ) -> u64 {
        let source_hash = |value: &crate::runtime::transfer::TransferStoredRuntimeValue| {
            crate::runtime::evaluated::value_source_hash(storage, value.value())
        };
        value.source_hash(&crate::runtime::transfer::TransferExternalHashing::new(
            &source_hash,
        ))
    }
}

impl RuntimeExternalLease for crate::host::ExternalPayloadLease {
    fn identity(&self) -> u64 {
        self.id()
    }
}

#[cfg(test)]
mod tests {
    use super::{RuntimeValueProfile, TransferValues};
    use crate::plan::execution::type_::ExternalTypeId;
    use crate::runtime::transfer::{
        TransferExternalEquality, TransferExternalHashing, TransferExternalInspection,
        TransferExternalStore, TransferStoredRuntimeValue,
    };
    use crate::runtime::{EvaluatedExternalValue, EvaluatedValue};

    fn equal(context: &TransferExternalEquality<'_>, left: &usize, right: &usize) -> bool {
        let left = TransferStoredRuntimeValue::new(EvaluatedValue::Int((*left).into()));
        let right = TransferStoredRuntimeValue::new(EvaluatedValue::Int((*right).into()));
        context.stored_values_equal(&left, &right)
    }

    fn hash(context: &TransferExternalHashing<'_>, value: &usize) -> u64 {
        context.stored_value_hash(&TransferStoredRuntimeValue::new(EvaluatedValue::Int(
            (*value).into(),
        )))
    }

    fn inspect(context: &TransferExternalInspection<'_>, value: &usize) -> ecow::EcoString {
        context.inspect_stored_value(&TransferStoredRuntimeValue::new(EvaluatedValue::Int(
            (*value).into(),
        )))
    }

    #[test]
    fn transfer_profile_routes_external_source_semantics_through_its_owned_storage() {
        let store = TransferExternalStore::default();
        let first = EvaluatedExternalValue::<TransferValues>::new(
            ExternalTypeId::new(0),
            store.insert(7usize, equal, hash, inspect),
        );
        let second = EvaluatedExternalValue::<TransferValues>::new(
            ExternalTypeId::new(0),
            store.insert(7usize, equal, hash, inspect),
        );
        let different = EvaluatedExternalValue::<TransferValues>::new(
            ExternalTypeId::new(0),
            store.insert(8usize, equal, hash, inspect),
        );
        let mut lists = <TransferValues as RuntimeValueProfile>::ListStorage::default();

        assert!(TransferValues::external_values_equal(
            &lists, &first, &second,
        ));
        assert!(!TransferValues::external_values_equal(
            &lists, &first, &different,
        ));
        let expected_hash = crate::runtime::evaluated::value_source_hash(
            &lists,
            &EvaluatedValue::<TransferValues>::Int(7.into()),
        );
        assert_eq!(
            TransferValues::external_value_source_hash(&lists, &first),
            expected_hash,
        );
        let stored_inspect = |_: &TransferStoredRuntimeValue| "7".into();
        let inspection = TransferExternalInspection::new(&stored_inspect);
        assert_eq!(first.lease().inspection(&inspection), "7");
        crate::runtime::RuntimeListStorage::drain_releases(&mut lists);
    }
}
