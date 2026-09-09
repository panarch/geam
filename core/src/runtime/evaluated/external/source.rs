pub(in crate::runtime) fn values_equal(
    storage: &crate::runtime::RuntimeListStorage,
    left: &crate::runtime::EvaluatedExternalValue,
    right: &crate::runtime::EvaluatedExternalValue,
) -> bool {
    let equal = |left: &crate::runtime::retained::RetainedValueRef,
                 right: &crate::runtime::retained::RetainedValueRef| {
        crate::runtime::evaluated::values_equal(storage, left.value(), right.value())
    };
    left.source_equal(
        &crate::runtime::retained::RetainedValueEquality::new(&equal),
        right,
    )
}

pub(in crate::runtime) fn source_hash(
    storage: &crate::runtime::RuntimeListStorage,
    value: &crate::runtime::EvaluatedExternalValue,
) -> u64 {
    let source_hash = |value: &crate::runtime::retained::RetainedValueRef| {
        crate::runtime::evaluated::value_source_hash(storage, value.value())
    };
    value.source_hash(&crate::runtime::retained::RetainedValueHashing::new(
        &source_hash,
    ))
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::type_::ExternalTypeId;
    use crate::runtime::retained::{RetainedValueInspection, RetainedValueRef};
    use crate::runtime::{EvaluatedExternalValue, EvaluatedValue};

    fn equal(context: &crate::host::HostExternalEquality<'_>, left: &usize, right: &usize) -> bool {
        let left = EvaluatedValue::Int((*left).into());
        let right = EvaluatedValue::Int((*right).into());
        context.0.stored_values_equal(
            &RetainedValueRef::new(&left),
            &RetainedValueRef::new(&right),
        )
    }

    fn hash(context: &crate::host::HostExternalHashing<'_>, value: &usize) -> u64 {
        context
            .0
            .stored_value_hash(&RetainedValueRef::new(&EvaluatedValue::Int(
                (*value).into(),
            )))
    }

    fn inspect(
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
    fn transfer_profile_routes_external_source_semantics_through_its_owned_storage() {
        let store = crate::host::HostExternalStore::default();
        let first = EvaluatedExternalValue::new(
            ExternalTypeId::new(0),
            store.insert(7usize, equal, hash, inspect, |_| None),
        );
        let second = EvaluatedExternalValue::new(
            ExternalTypeId::new(0),
            store.insert(7usize, equal, hash, inspect, |_| None),
        );
        let different = EvaluatedExternalValue::new(
            ExternalTypeId::new(0),
            store.insert(8usize, equal, hash, inspect, |_| None),
        );
        let lists = crate::runtime::RuntimeListStorage::default();

        assert!(super::values_equal(&lists, &first, &second,));
        assert!(!super::values_equal(&lists, &first, &different,));
        let expected_hash =
            crate::runtime::evaluated::value_source_hash(&lists, &EvaluatedValue::Int(7.into()));
        assert_eq!(super::source_hash(&lists, &first), expected_hash,);
        let stored_inspect = |_: &RetainedValueRef| "7".into();
        let inspection = RetainedValueInspection::new(&stored_inspect);
        assert_eq!(first.lease().inspection(&inspection), "7");
    }
}
