use crate::StringValue;
use crate::plan::execution::type_::{
    BitArrayListTypeId, BoolListTypeId, FloatListTypeId, NilListTypeId, StringListTypeId,
    UtfCodepointListTypeId,
};
use crate::runtime::compiled::calls::CallBitArray;
use crate::runtime::state::list::{
    BitArrayListValueId, BoolListValueId, FloatListValueId, NilListValueId, RuntimeListStorage,
    StringListValueId, UtfCodepointListValueId,
};

/// Opaque typed leases. Clones and suffixes retain the existing storage owner.
#[derive(Clone)]
pub struct BoolList(pub(in crate::runtime) BoolListValueId);

#[derive(Clone)]
pub struct FloatList(pub(in crate::runtime) FloatListValueId);

#[derive(Clone)]
pub struct StringList(pub(in crate::runtime) StringListValueId);

#[derive(Clone)]
pub struct BitArrayList(pub(in crate::runtime) BitArrayListValueId);

#[derive(Clone)]
pub struct UtfCodepointList(pub(in crate::runtime) UtfCodepointListValueId);

#[derive(Clone)]
pub struct NilList(pub(in crate::runtime) NilListValueId);

/// A borrowed adapter over the canonical list storage, never a second owner.
pub struct PrimitiveListOps<'runtime> {
    storage: &'runtime RuntimeListStorage,
}

impl BoolList {
    pub fn len(&self) -> usize {
        self.0.values().len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl FloatList {
    pub fn len(&self) -> usize {
        self.0.values().len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl StringList {
    pub fn len(&self) -> usize {
        self.0.values().len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl BitArrayList {
    pub fn len(&self) -> usize {
        self.0.values().len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl UtfCodepointList {
    pub fn len(&self) -> usize {
        self.0.values().len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl NilList {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'runtime> PrimitiveListOps<'runtime> {
    pub(in crate::runtime) fn new(storage: &'runtime RuntimeListStorage) -> Self {
        Self { storage }
    }

    pub fn bool_value(&self, type_id: BoolListTypeId, elements: &[bool]) -> BoolList {
        BoolList(self.storage.bool(type_id, elements.to_vec()))
    }
    pub fn bool_prepend(
        &self,
        type_id: BoolListTypeId,
        elements: &[bool],
        tail: &BoolList,
    ) -> BoolList {
        BoolList(
            self.storage
                .prepend_bool(type_id, elements.to_vec(), &tail.0),
        )
    }
    pub fn bool_index(&self, value: &BoolList, index: usize) -> Option<bool> {
        self.storage.bool_values(&value.0).get(index).copied()
    }
    pub fn bool_tail(&self, value: &BoolList, type_id: BoolListTypeId, count: usize) -> BoolList {
        BoolList(self.storage.tail_bool(type_id, &value.0, count))
    }
    pub fn bool_equal(&self, left: &BoolList, right: &BoolList) -> bool {
        left.0.type_id() == right.0.type_id()
            && left.len() == right.len()
            && self
                .storage
                .bool_values(&left.0)
                .iter()
                .eq(self.storage.bool_values(&right.0).iter())
    }

    pub fn float_value(&self, type_id: FloatListTypeId, elements: &[f64]) -> FloatList {
        FloatList(self.storage.float(type_id, elements.to_vec()))
    }
    pub fn float_prepend(
        &self,
        type_id: FloatListTypeId,
        elements: &[f64],
        tail: &FloatList,
    ) -> FloatList {
        FloatList(
            self.storage
                .prepend_float(type_id, elements.to_vec(), &tail.0),
        )
    }
    pub fn float_index(&self, value: &FloatList, index: usize) -> Option<f64> {
        self.storage.float_values(&value.0).get(index).copied()
    }
    pub fn float_tail(
        &self,
        value: &FloatList,
        type_id: FloatListTypeId,
        count: usize,
    ) -> FloatList {
        FloatList(self.storage.tail_float(type_id, &value.0, count))
    }
    pub fn float_equal(&self, left: &FloatList, right: &FloatList) -> bool {
        left.0.type_id() == right.0.type_id()
            && left.len() == right.len()
            && self
                .storage
                .float_values(&left.0)
                .iter()
                .eq(self.storage.float_values(&right.0).iter())
    }

    pub fn string_value(&self, type_id: StringListTypeId, elements: &[StringValue]) -> StringList {
        StringList(self.storage.string(type_id, elements.to_vec()))
    }
    pub fn string_prepend(
        &self,
        type_id: StringListTypeId,
        elements: &[StringValue],
        tail: &StringList,
    ) -> StringList {
        StringList(
            self.storage
                .prepend_string(type_id, elements.to_vec(), &tail.0),
        )
    }
    pub fn string_index(&self, value: &StringList, index: usize) -> Option<StringValue> {
        self.storage.string_values(&value.0).get(index).cloned()
    }
    pub fn string_tail(
        &self,
        value: &StringList,
        type_id: StringListTypeId,
        count: usize,
    ) -> StringList {
        StringList(self.storage.tail_string(type_id, &value.0, count))
    }
    pub fn string_equal(&self, left: &StringList, right: &StringList) -> bool {
        left.0.type_id() == right.0.type_id()
            && left.len() == right.len()
            && self
                .storage
                .string_values(&left.0)
                .iter()
                .eq(self.storage.string_values(&right.0).iter())
    }

    pub fn bit_array_value(
        &self,
        type_id: BitArrayListTypeId,
        elements: &[CallBitArray],
    ) -> BitArrayList {
        BitArrayList(self.storage.bit_array(
            type_id,
            elements.iter().map(|value| value.0.clone()).collect(),
        ))
    }
    pub fn bit_array_prepend(
        &self,
        type_id: BitArrayListTypeId,
        elements: &[CallBitArray],
        tail: &BitArrayList,
    ) -> BitArrayList {
        BitArrayList(self.storage.prepend_bit_array(
            type_id,
            elements.iter().map(|value| value.0.clone()).collect(),
            &tail.0,
        ))
    }
    pub fn bit_array_index(&self, value: &BitArrayList, index: usize) -> Option<CallBitArray> {
        self.storage
            .bit_array_values(&value.0)
            .get(index)
            .cloned()
            .map(CallBitArray)
    }
    pub fn bit_array_tail(
        &self,
        value: &BitArrayList,
        type_id: BitArrayListTypeId,
        count: usize,
    ) -> BitArrayList {
        BitArrayList(self.storage.tail_bit_array(type_id, &value.0, count))
    }
    pub fn bit_array_equal(&self, left: &BitArrayList, right: &BitArrayList) -> bool {
        left.0.type_id() == right.0.type_id()
            && left.len() == right.len()
            && self
                .storage
                .bit_array_values(&left.0)
                .iter()
                .eq(self.storage.bit_array_values(&right.0).iter())
    }

    pub fn utf_codepoint_value(
        &self,
        type_id: UtfCodepointListTypeId,
        elements: &[char],
    ) -> UtfCodepointList {
        UtfCodepointList(self.storage.utf_codepoint(type_id, elements.to_vec()))
    }
    pub fn utf_codepoint_prepend(
        &self,
        type_id: UtfCodepointListTypeId,
        elements: &[char],
        tail: &UtfCodepointList,
    ) -> UtfCodepointList {
        UtfCodepointList(
            self.storage
                .prepend_utf_codepoint(type_id, elements.to_vec(), &tail.0),
        )
    }
    pub fn utf_codepoint_index(&self, value: &UtfCodepointList, index: usize) -> Option<char> {
        self.storage
            .utf_codepoint_values(&value.0)
            .get(index)
            .cloned()
    }
    pub fn utf_codepoint_tail(
        &self,
        value: &UtfCodepointList,
        type_id: UtfCodepointListTypeId,
        count: usize,
    ) -> UtfCodepointList {
        UtfCodepointList(self.storage.tail_utf_codepoint(type_id, &value.0, count))
    }
    pub fn utf_codepoint_equal(&self, left: &UtfCodepointList, right: &UtfCodepointList) -> bool {
        left.0.type_id() == right.0.type_id()
            && left.len() == right.len()
            && self
                .storage
                .utf_codepoint_values(&left.0)
                .iter()
                .eq(self.storage.utf_codepoint_values(&right.0).iter())
    }

    pub fn nil_value(&self, type_id: NilListTypeId, count: usize) -> NilList {
        NilList(self.storage.nil(type_id, count))
    }
    pub fn nil_prepend(&self, type_id: NilListTypeId, count: usize, tail: &NilList) -> NilList {
        NilList(self.storage.prepend_nil(type_id, count, &tail.0))
    }
    pub fn nil_index(&self, value: &NilList, index: usize) -> Option<()> {
        (index < self.storage.nil_len(&value.0)).then_some(())
    }
    pub fn nil_tail(&self, value: &NilList, type_id: NilListTypeId, count: usize) -> NilList {
        NilList(self.storage.tail_nil(type_id, &value.0, count))
    }
    pub fn nil_equal(&self, left: &NilList, right: &NilList) -> bool {
        left.0.type_id() == right.0.type_id() && left.len() == right.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitArrayListTypeId, BoolListTypeId, CallBitArray, FloatListTypeId, NilListTypeId,
        PrimitiveListOps, RuntimeListStorage, StringListTypeId, StringValue,
        UtfCodepointListTypeId,
    };
    use crate::BitArrayValue;
    use crate::plan::execution::type_::ListTypeId;
    use crate::runtime::evaluated::EvaluatedBitArray;

    #[test]
    fn bool_lease_reads_prepend_and_suffix_preserve_order_and_owner() {
        let storage = RuntimeListStorage::default();
        let ops = PrimitiveListOps::new(&storage);
        let type_id = BoolListTypeId {
            list_type: ListTypeId(0),
        };
        let values = [false, true, false];
        let prefix = [true];
        let empty = ops.bool_value(type_id, &[]);
        assert!(empty.is_empty());
        let list = ops.bool_value(type_id, &values);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 3);
        assert_eq!(ops.bool_index(&list, 0), Some(false));
        assert!(ops.bool_index(&list, 3).is_none());
        let combined = ops.bool_prepend(type_id, &prefix, &list);
        assert_eq!(combined.len(), 4);
        assert_eq!(ops.bool_index(&combined, 0), Some(true));
        let original = list.0.values().get(1).unwrap();
        assert!(std::ptr::eq(original, combined.0.values().get(2).unwrap()));
        let no_prefix = ops.bool_prepend(type_id, &[], &list);
        assert!(std::ptr::eq(original, no_prefix.0.values().get(1).unwrap()));
        let tail = ops.bool_tail(&combined, type_id, 2);
        assert!(std::ptr::eq(original, tail.0.values().get(0).unwrap()));
        assert!(ops.bool_equal(&tail, &ops.bool_value(type_id, &values[1..])));
        assert!(!ops.bool_equal(&tail, &list));
        assert!(!ops.bool_equal(&list, &ops.bool_value(type_id, &prefix)));
        let other_type = BoolListTypeId {
            list_type: ListTypeId(1),
        };
        assert!(!ops.bool_equal(&empty, &ops.bool_value(other_type, &[])));
        assert!(ops.bool_tail(&list, type_id, usize::MAX).is_empty());
        drop(list);
        assert!(ops.bool_equal(
            &combined,
            &ops.bool_value(type_id, &[prefix[0], values[0], values[1], values[2]])
        ));
    }

    #[test]
    fn float_lease_reads_prepend_and_suffix_preserve_order_and_owner() {
        let storage = RuntimeListStorage::default();
        let ops = PrimitiveListOps::new(&storage);
        let type_id = FloatListTypeId {
            list_type: ListTypeId(0),
        };
        let values = [-0.0, 3.5, 4.5];
        let prefix = [7.0];
        let empty = ops.float_value(type_id, &[]);
        assert!(empty.is_empty());
        let list = ops.float_value(type_id, &values);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 3);
        assert!(ops.float_index(&list, 0).is_some_and(|value| value == -0.0));
        assert!(ops.float_index(&list, 3).is_none());
        let combined = ops.float_prepend(type_id, &prefix, &list);
        assert_eq!(combined.len(), 4);
        assert!(
            ops.float_index(&combined, 0)
                .is_some_and(|value| value == 7.0)
        );
        let original = list.0.values().get(1).unwrap();
        assert!(std::ptr::eq(original, combined.0.values().get(2).unwrap()));
        let no_prefix = ops.float_prepend(type_id, &[], &list);
        assert!(std::ptr::eq(original, no_prefix.0.values().get(1).unwrap()));
        let tail = ops.float_tail(&combined, type_id, 2);
        assert!(std::ptr::eq(original, tail.0.values().get(0).unwrap()));
        assert!(ops.float_equal(&tail, &ops.float_value(type_id, &values[1..])));
        assert!(!ops.float_equal(&tail, &list));
        assert!(!ops.float_equal(&list, &ops.float_value(type_id, &prefix)));
        let other_type = FloatListTypeId {
            list_type: ListTypeId(1),
        };
        assert!(!ops.float_equal(&empty, &ops.float_value(other_type, &[])));
        assert!(ops.float_tail(&list, type_id, usize::MAX).is_empty());
        drop(list);
        assert!(ops.float_equal(
            &combined,
            &ops.float_value(type_id, &[prefix[0], values[0], values[1], values[2]])
        ));
    }

    #[test]
    fn string_lease_reads_prepend_and_suffix_preserve_order_and_owner() {
        let storage = RuntimeListStorage::default();
        let ops = PrimitiveListOps::new(&storage);
        let type_id = StringListTypeId {
            list_type: ListTypeId(0),
        };
        let values = [
            StringValue::from("first string with a shared owner"),
            StringValue::from("λsecond string with a shared owner"),
            StringValue::from("third string with a shared owner"),
        ];
        let prefix = [StringValue::from("prefix string with a shared owner")];
        let empty = ops.string_value(type_id, &[]);
        assert!(empty.is_empty());
        let list = ops.string_value(type_id, &values);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 3);
        assert!(
            ops.string_index(&list, 0)
                .is_some_and(|value| value == values[0].clone())
        );
        assert!(ops.string_index(&list, 3).is_none());
        let combined = ops.string_prepend(type_id, &prefix, &list);
        assert_eq!(combined.len(), 4);
        assert!(
            ops.string_index(&combined, 0)
                .is_some_and(|value| value == prefix[0].clone())
        );
        let original = list.0.values().get(1).unwrap();
        assert!(std::ptr::eq(original, combined.0.values().get(2).unwrap()));
        let no_prefix = ops.string_prepend(type_id, &[], &list);
        assert!(std::ptr::eq(original, no_prefix.0.values().get(1).unwrap()));
        let tail = ops.string_tail(&combined, type_id, 2);
        assert!(std::ptr::eq(original, tail.0.values().get(0).unwrap()));
        assert!(ops.string_equal(&tail, &ops.string_value(type_id, &values[1..])));
        assert!(!ops.string_equal(&tail, &list));
        assert!(!ops.string_equal(&list, &ops.string_value(type_id, &prefix)));
        let other_type = StringListTypeId {
            list_type: ListTypeId(1),
        };
        assert!(!ops.string_equal(&empty, &ops.string_value(other_type, &[])));
        assert!(ops.string_tail(&list, type_id, usize::MAX).is_empty());
        drop(list);
        assert!(ops.string_equal(
            &combined,
            &ops.string_value(
                type_id,
                &[
                    prefix[0].clone(),
                    values[0].clone(),
                    values[1].clone(),
                    values[2].clone()
                ]
            )
        ));
    }

    #[test]
    fn bit_array_lease_reads_prepend_and_suffix_preserve_order_and_owner() {
        let storage = RuntimeListStorage::default();
        let ops = PrimitiveListOps::new(&storage);
        let type_id = BitArrayListTypeId {
            list_type: ListTypeId(0),
        };
        let bits = CallBitArray(EvaluatedBitArray::from_value(
            BitArrayValue::try_from_parts(vec![0xAC, 0x50], 13).unwrap(),
        ));
        let other_bits = CallBitArray(EvaluatedBitArray::from_value(
            BitArrayValue::try_from_parts(vec![0xE0], 3).unwrap(),
        ));
        let values = [bits.clone(), bits.clone(), bits.clone()];
        let prefix = [other_bits.clone()];
        let empty = ops.bit_array_value(type_id, &[]);
        assert!(empty.is_empty());
        let list = ops.bit_array_value(type_id, &values);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 3);
        assert!(
            ops.bit_array_index(&list, 0)
                .is_some_and(|value| value == bits.clone())
        );
        assert!(ops.bit_array_index(&list, 3).is_none());
        let combined = ops.bit_array_prepend(type_id, &prefix, &list);
        assert_eq!(combined.len(), 4);
        assert!(
            ops.bit_array_index(&combined, 0)
                .is_some_and(|value| value == other_bits.clone())
        );
        let original = list.0.values().get(1).unwrap();
        assert!(std::ptr::eq(original, combined.0.values().get(2).unwrap()));
        let no_prefix = ops.bit_array_prepend(type_id, &[], &list);
        assert!(std::ptr::eq(original, no_prefix.0.values().get(1).unwrap()));
        let tail = ops.bit_array_tail(&combined, type_id, 2);
        assert!(std::ptr::eq(original, tail.0.values().get(0).unwrap()));
        assert!(ops.bit_array_equal(&tail, &ops.bit_array_value(type_id, &values[1..])));
        assert!(!ops.bit_array_equal(&tail, &list));
        assert!(!ops.bit_array_equal(&list, &ops.bit_array_value(type_id, &prefix)));
        let other_type = BitArrayListTypeId {
            list_type: ListTypeId(1),
        };
        assert!(!ops.bit_array_equal(&empty, &ops.bit_array_value(other_type, &[])));
        assert!(ops.bit_array_tail(&list, type_id, usize::MAX).is_empty());
        drop(list);
        assert!(ops.bit_array_equal(
            &combined,
            &ops.bit_array_value(
                type_id,
                &[
                    prefix[0].clone(),
                    values[0].clone(),
                    values[1].clone(),
                    values[2].clone()
                ]
            )
        ));
    }

    #[test]
    fn utf_codepoint_lease_reads_prepend_and_suffix_preserve_order_and_owner() {
        let storage = RuntimeListStorage::default();
        let ops = PrimitiveListOps::new(&storage);
        let type_id = UtfCodepointListTypeId {
            list_type: ListTypeId(0),
        };
        let values = ['λ', '😀', 'x'];
        let prefix = ['β'];
        let empty = ops.utf_codepoint_value(type_id, &[]);
        assert!(empty.is_empty());
        let list = ops.utf_codepoint_value(type_id, &values);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 3);
        assert!(
            ops.utf_codepoint_index(&list, 0)
                .is_some_and(|value| value == 'λ')
        );
        assert!(ops.utf_codepoint_index(&list, 3).is_none());
        let combined = ops.utf_codepoint_prepend(type_id, &prefix, &list);
        assert_eq!(combined.len(), 4);
        assert!(
            ops.utf_codepoint_index(&combined, 0)
                .is_some_and(|value| value == 'β')
        );
        let original = list.0.values().get(1).unwrap();
        assert!(std::ptr::eq(original, combined.0.values().get(2).unwrap()));
        let no_prefix = ops.utf_codepoint_prepend(type_id, &[], &list);
        assert!(std::ptr::eq(original, no_prefix.0.values().get(1).unwrap()));
        let tail = ops.utf_codepoint_tail(&combined, type_id, 2);
        assert!(std::ptr::eq(original, tail.0.values().get(0).unwrap()));
        assert!(ops.utf_codepoint_equal(&tail, &ops.utf_codepoint_value(type_id, &values[1..])));
        assert!(!ops.utf_codepoint_equal(&tail, &list));
        assert!(!ops.utf_codepoint_equal(&list, &ops.utf_codepoint_value(type_id, &prefix)));
        let other_type = UtfCodepointListTypeId {
            list_type: ListTypeId(1),
        };
        assert!(!ops.utf_codepoint_equal(&empty, &ops.utf_codepoint_value(other_type, &[])));
        assert!(
            ops.utf_codepoint_tail(&list, type_id, usize::MAX)
                .is_empty()
        );
        drop(list);
        assert!(ops.utf_codepoint_equal(
            &combined,
            &ops.utf_codepoint_value(type_id, &[prefix[0], values[0], values[1], values[2]])
        ));
    }

    #[test]
    fn float_equality_keeps_ieee_nan_and_signed_zero_semantics() {
        let storage = RuntimeListStorage::default();
        let ops = PrimitiveListOps::new(&storage);
        let type_id = FloatListTypeId {
            list_type: ListTypeId(0),
        };
        let nan = ops.float_value(type_id, &[f64::NAN]);
        assert!(!ops.float_equal(&nan, &nan.clone()));
        assert!(ops.float_index(&nan, 0).unwrap().is_nan());
        let negative_zero = ops.float_value(type_id, &[-0.0]);
        assert_eq!(
            ops.float_index(&negative_zero, 0).unwrap().to_bits(),
            (-0.0_f64).to_bits()
        );
        assert!(ops.float_equal(&negative_zero, &ops.float_value(type_id, &[0.0])));
        assert!(!ops.float_equal(&negative_zero, &ops.float_value(type_id, &[1.0])));
    }

    #[test]
    fn nil_lists_retain_length_without_materializing_unit_elements() {
        let storage = RuntimeListStorage::default();
        let ops = PrimitiveListOps::new(&storage);
        let type_id = NilListTypeId {
            list_type: ListTypeId(0),
        };
        let empty = ops.nil_value(type_id, 0);
        assert!(empty.is_empty());
        let list = ops.nil_value(type_id, 10_000);
        assert_eq!(list.len(), 10_000);
        assert!(!list.is_empty());
        assert_eq!(ops.nil_index(&list, 9_999), Some(()));
        assert_eq!(ops.nil_index(&list, 10_000), None);
        let combined = ops.nil_prepend(type_id, 3, &list);
        assert_eq!(combined.len(), 10_003);
        let tail = ops.nil_tail(&combined, type_id, 3);
        assert!(ops.nil_equal(&list, &tail));
        assert!(!ops.nil_equal(&list, &empty));
        assert!(!ops.nil_equal(
            &list,
            &ops.nil_value(
                NilListTypeId {
                    list_type: ListTypeId(1)
                },
                10_000
            )
        ));
        assert!(ops.nil_tail(&list, type_id, usize::MAX).is_empty());
        drop(list);
        assert_eq!(ops.nil_index(&tail, 9_999), Some(()));
    }
}
