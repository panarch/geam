use crate::runtime::{RetainedValueRef, StoredRuntimeValue};
use ecow::EcoString;

pub(crate) struct RetainedValueEquality<'context> {
    equal: &'context dyn Fn(&RetainedValueRef, &RetainedValueRef) -> bool,
}

pub(crate) struct RetainedValueHashing<'context> {
    source_hash: &'context dyn Fn(&RetainedValueRef) -> u64,
}

pub(crate) struct RetainedValueInspection<'context> {
    inspect: &'context dyn Fn(&RetainedValueRef) -> EcoString,
}

use crate::host::{HostExternalEquality, HostExternalHashing, HostExternalInspection};

impl<'context> RetainedValueEquality<'context> {
    pub(crate) fn new(
        equal: &'context dyn Fn(&RetainedValueRef, &RetainedValueRef) -> bool,
    ) -> Self {
        Self { equal }
    }

    pub(crate) fn stored_values_equal(
        &self,
        left: &RetainedValueRef,
        right: &RetainedValueRef,
    ) -> bool {
        (self.equal)(left, right)
    }
}

impl<'context> RetainedValueHashing<'context> {
    pub(crate) fn new(source_hash: &'context dyn Fn(&RetainedValueRef) -> u64) -> Self {
        Self { source_hash }
    }

    pub(crate) fn stored_value_hash(&self, value: &RetainedValueRef) -> u64 {
        (self.source_hash)(value)
    }
}

impl<'context> RetainedValueInspection<'context> {
    pub(crate) fn new(inspect: &'context dyn Fn(&RetainedValueRef) -> EcoString) -> Self {
        Self { inspect }
    }

    pub(crate) fn inspect_stored_value(&self, value: &RetainedValueRef) -> EcoString {
        (self.inspect)(value)
    }
}

impl HostExternalEquality<'_> {
    pub(crate) fn provider_stored_values_equal(
        &self,
        left: &StoredRuntimeValue,
        right: &StoredRuntimeValue,
    ) -> bool {
        self.0
            .stored_values_equal(&left.semantic_value(), &right.semantic_value())
    }
}

impl HostExternalHashing<'_> {
    pub(crate) fn provider_stored_value_hash(&self, value: &StoredRuntimeValue) -> u64 {
        self.0.stored_value_hash(&value.semantic_value())
    }
}

impl HostExternalInspection<'_> {
    pub(crate) fn provider_inspect_stored_value(&self, value: &StoredRuntimeValue) -> EcoString {
        self.0.inspect_stored_value(&value.semantic_value())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        HostExternalEquality, HostExternalHashing, HostExternalInspection, RetainedValueEquality,
        RetainedValueHashing, RetainedValueInspection, RetainedValueRef, StoredRuntimeValue,
    };
    use std::cell::Cell;

    #[test]
    fn contexts_forward_retained_values_through_their_source_semantic_boundary() {
        let left = StoredRuntimeValue::test_int(7.into());
        let right = StoredRuntimeValue::test_int(8.into());
        let left_semantics = left.semantic_value();
        let right_semantics = right.semantic_value();
        let equal = |a: &RetainedValueRef, b: &RetainedValueRef| {
            assert!(std::ptr::eq(a, &left_semantics));
            assert!(std::ptr::eq(b, &right_semantics));
            false
        };
        let hash = |value: &RetainedValueRef| {
            assert!(std::ptr::eq(value, &left_semantics));
            17
        };
        let inspect = |value: &RetainedValueRef| {
            assert!(std::ptr::eq(value, &right_semantics));
            "retained integer".into()
        };
        assert!(
            !RetainedValueEquality::new(&equal)
                .stored_values_equal(&left_semantics, &right_semantics)
        );
        assert_eq!(
            RetainedValueHashing::new(&hash).stored_value_hash(&left_semantics),
            17
        );
        assert_eq!(
            RetainedValueInspection::new(&inspect).inspect_stored_value(&right_semantics),
            "retained integer"
        );

        let calls = Cell::new(0);
        let equal = |_: &RetainedValueRef, _: &RetainedValueRef| {
            calls.set(calls.get() + 1);
            false
        };
        let hash = |_: &RetainedValueRef| {
            calls.set(calls.get() + 1);
            17
        };
        let inspect = |_: &RetainedValueRef| {
            calls.set(calls.get() + 1);
            "retained integer".into()
        };
        assert!(
            !HostExternalEquality(&RetainedValueEquality::new(&equal))
                .provider_stored_values_equal(&left, &right)
        );
        assert_eq!(
            HostExternalHashing(&RetainedValueHashing::new(&hash))
                .provider_stored_value_hash(&left),
            17
        );
        assert_eq!(
            HostExternalInspection(&RetainedValueInspection::new(&inspect))
                .provider_inspect_stored_value(&right),
            "retained integer"
        );
        assert_eq!(calls.get(), 3);
    }
}
