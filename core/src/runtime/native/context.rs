use super::{NativeValue, value_hash, values_equal};
use crate::plan::execution::runtime::RuntimeValueMetadata;
use crate::runtime::{EvaluatedValue, RuntimeListStorage, StoredRuntimeValue};
use num_bigint::BigInt;

/// Call-borrowed native value operations, without mutable host or execution access.
///
/// Constructed scalar views still pass through the registered native conversion
/// boundary before they can be used as typed Gleam values.
#[derive(Clone, Copy)]
pub struct NativeValues<'call> {
    lists: &'call RuntimeListStorage,
    metadata: RuntimeValueMetadata<'call>,
}

impl<'call> NativeValues<'call> {
    pub(crate) fn new(
        lists: &'call RuntimeListStorage,
        metadata: RuntimeValueMetadata<'call>,
    ) -> Self {
        Self { lists, metadata }
    }

    pub(crate) fn value_retention(self) -> crate::runtime::ValueRetention {
        crate::runtime::ValueRetention::new(self.metadata)
    }

    pub fn equal(self, left: &NativeValue, right: &NativeValue) -> bool {
        values_equal(self.lists, left, right)
    }

    pub fn hash(self, value: &NativeValue) -> u64 {
        value_hash(self.lists, value)
    }

    pub fn integer(self, value: BigInt) -> NativeValue {
        NativeValue::from_stored(StoredRuntimeValue::new(
            EvaluatedValue::Int(value),
            self.metadata,
        ))
    }

    pub fn string(self, value: crate::StringValue) -> NativeValue {
        NativeValue::from_stored(StoredRuntimeValue::new(
            EvaluatedValue::String(value),
            self.metadata,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::NativeValues;
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::runtime::{NativeKind, NativeValue, RuntimeListStorage};

    #[test]
    fn native_nested_string_reads_retain_visible_storage_and_binary_semantics() {
        let plan = crate::runtime::plan_src("pub fn main() { Nil }");
        let lists = RuntimeListStorage::default();
        let values = NativeValues::new(&lists, plan.value_metadata());
        let root = crate::StringValue::from("hidden:abcdefghijklmnopqrstuvwxyz:hidden");
        let slice = root.slice(7..33);
        let pointer = slice.as_ptr();
        let native = values.string(slice.clone());
        let tuple = NativeValue::tuple([NativeValue::tuple([native.clone()])]);
        let nested = tuple
            .index(0)
            .expect("outer tuple element")
            .index(0)
            .expect("retained string element");
        let independent = values.string("abcdefghijklmnopqrstuvwxyz".into());
        let binary = NativeValue::from_stored(crate::runtime::StoredRuntimeValue::new(
            crate::runtime::EvaluatedValue::BitArray(
                crate::runtime::evaluated::EvaluatedBitArray::from_value(
                    crate::BitArrayValue::from_bytes(b"abcdefghijklmnopqrstuvwxyz".to_vec()),
                ),
            ),
            plan.value_metadata(),
        ));
        assert_eq!(native.bit_len(), Some(208));
        assert!(values.equal(&native, &independent));
        assert!(values.equal(&native, &binary));
        assert_eq!(values.hash(&native), values.hash(&binary));
        assert_eq!(binary.as_string(), Some(slice));
        drop(root);
        drop(native);
        drop(tuple);
        let text = nested
            .as_string()
            .expect("native String should stay a String");
        assert_eq!(text, "abcdefghijklmnopqrstuvwxyz");
        assert_eq!(text.as_ptr(), pointer);
    }

    #[test]
    fn native_scalars_preserve_kind_contents_and_structural_comparison() {
        let plan = crate::runtime::plan_src("pub fn main() { #(42, \"native\") }");
        let lists = RuntimeListStorage::default();
        let values = NativeValues::new(&lists, plan.value_metadata());
        let integer = values.integer(42.into());
        let string = values.string("native".into());
        assert_eq!(integer.kind(), NativeKind::Int);
        assert_eq!(integer.as_int(), Some(42.into()));
        assert_eq!(string.kind(), NativeKind::Binary);
        assert_eq!(string.as_string().as_deref(), Some("native"));
        assert!(values.equal(&integer, &values.integer(42.into())));
        assert!(!values.equal(&integer, &values.integer(43.into())));
        assert!(!values.equal(&integer, &string));
        let tuple = NativeValue::tuple([integer, string]);
        let equal = NativeValue::tuple([values.integer(42.into()), values.string("native".into())]);
        assert!(values.equal(&tuple, &equal));
        assert_eq!(values.hash(&tuple), values.hash(&equal));
    }
}
