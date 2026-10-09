use super::{
    BitArrayCallable, BoolCallable, CallBitArray, CallCapture, CallCaptureInputs, CallCaptures,
    FloatCallable, IntCallable, NilCallable, StringCallable, UtfCodepointCallable,
};
use crate::StringValue;
use crate::plan::execution::graph::{
    BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId, BoolFunctionLocalId,
    BoolListLocalId, BoolLocalId, FloatFunctionLocalId, FloatListLocalId, FloatLocalId,
    IntFunctionLocalId, IntListLocalId, IntLocalId, NilFunctionLocalId, NilListLocalId, NilLocalId,
    StringFunctionLocalId, StringListLocalId, StringLocalId, UtfCodepointFunctionLocalId,
    UtfCodepointListLocalId, UtfCodepointLocalId,
};
use crate::runtime::compiled::int_list::IntList;
use crate::runtime::compiled::primitive_list::{
    BitArrayList, BoolList, FloatList, NilList, StringList, UtfCodepointList,
};
use crate::runtime::evaluated::{EvaluatedCapture, EvaluatedCaptureKind, EvaluatedListCapture};

impl CallCaptureInputs<'_> {
    pub fn retain(&self) -> CallCaptures {
        CallCaptures(self.0.clone())
    }

    pub fn int(&self, local: IntLocalId) -> Option<i128> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::Int {
                    local: target,
                    value,
                } if *target == local => value.small().map(i128::from),
                _ => None,
            })
    }

    pub fn bool(&self, local: BoolLocalId) -> Option<bool> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::Bool {
                    local: target,
                    value,
                } if *target == local => Some(*value),
                _ => None,
            })
    }

    pub fn float(&self, local: FloatLocalId) -> Option<f64> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::Float {
                    local: target,
                    value,
                } if *target == local => Some(*value),
                _ => None,
            })
    }

    pub fn string(&self, local: StringLocalId) -> Option<StringValue> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::String {
                    local: target,
                    value,
                } if *target == local => Some(value.clone()),
                _ => None,
            })
    }

    pub fn bit_array(&self, local: BitArrayLocalId) -> Option<CallBitArray> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::BitArray {
                    local: target,
                    value,
                } if *target == local => Some(CallBitArray(value.clone())),
                _ => None,
            })
    }

    pub fn utf_codepoint(&self, local: UtfCodepointLocalId) -> Option<char> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::UtfCodepoint {
                    local: target,
                    value,
                } if *target == local => Some(*value),
                _ => None,
            })
    }

    pub fn nil(&self, local: NilLocalId) -> Option<()> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::Nil { local: target } if *target == local => Some(()),
                _ => None,
            })
    }

    pub fn int_list(&self, local: IntListLocalId) -> Option<IntList> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::List(EvaluatedListCapture::Int {
                    local: target,
                    value,
                }) if *target == local => Some(IntList(value.clone())),
                _ => None,
            })
    }

    pub fn int_function(&self, local: IntFunctionLocalId) -> Option<IntCallable> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::IntFunction {
                    local: target,
                    value,
                } if *target == local => Some(IntCallable(value.clone())),
                _ => None,
            })
    }

    pub fn bool_list(&self, local: BoolListLocalId) -> Option<BoolList> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::List(EvaluatedListCapture::Bool {
                    local: target,
                    value,
                }) if *target == local => Some(BoolList(value.clone())),
                _ => None,
            })
    }

    pub fn bool_function(&self, local: BoolFunctionLocalId) -> Option<BoolCallable> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::BoolFunction {
                    local: target,
                    value,
                } if *target == local => Some(BoolCallable(value.clone())),
                _ => None,
            })
    }

    pub fn float_list(&self, local: FloatListLocalId) -> Option<FloatList> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::List(EvaluatedListCapture::Float {
                    local: target,
                    value,
                }) if *target == local => Some(FloatList(value.clone())),
                _ => None,
            })
    }

    pub fn float_function(&self, local: FloatFunctionLocalId) -> Option<FloatCallable> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::FloatFunction {
                    local: target,
                    value,
                } if *target == local => Some(FloatCallable(value.clone())),
                _ => None,
            })
    }

    pub fn string_list(&self, local: StringListLocalId) -> Option<StringList> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::List(EvaluatedListCapture::String {
                    local: target,
                    value,
                }) if *target == local => Some(StringList(value.clone())),
                _ => None,
            })
    }

    pub fn string_function(&self, local: StringFunctionLocalId) -> Option<StringCallable> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::StringFunction {
                    local: target,
                    value,
                } if *target == local => Some(StringCallable(value.clone())),
                _ => None,
            })
    }

    pub fn bit_array_list(&self, local: BitArrayListLocalId) -> Option<BitArrayList> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::List(EvaluatedListCapture::BitArray {
                    local: target,
                    value,
                }) if *target == local => Some(BitArrayList(value.clone())),
                _ => None,
            })
    }

    pub fn bit_array_function(&self, local: BitArrayFunctionLocalId) -> Option<BitArrayCallable> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::BitArrayFunction {
                    local: target,
                    value,
                } if *target == local => Some(BitArrayCallable(value.clone())),
                _ => None,
            })
    }

    pub fn utf_codepoint_list(&self, local: UtfCodepointListLocalId) -> Option<UtfCodepointList> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::List(EvaluatedListCapture::UtfCodepoint {
                    local: target,
                    value,
                }) if *target == local => Some(UtfCodepointList(value.clone())),
                _ => None,
            })
    }

    pub fn utf_codepoint_function(
        &self,
        local: UtfCodepointFunctionLocalId,
    ) -> Option<UtfCodepointCallable> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::UtfCodepointFunction {
                    local: target,
                    value,
                } if *target == local => Some(UtfCodepointCallable(value.clone())),
                _ => None,
            })
    }

    pub fn nil_list(&self, local: NilListLocalId) -> Option<NilList> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::List(EvaluatedListCapture::Nil {
                    local: target,
                    value,
                }) if *target == local => Some(NilList(value.clone())),
                _ => None,
            })
    }

    pub fn nil_function(&self, local: NilFunctionLocalId) -> Option<NilCallable> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::NilFunction {
                    local: target,
                    value,
                } if *target == local => Some(NilCallable(value.clone())),
                _ => None,
            })
    }
}

impl CallCapture {
    pub fn int(local: IntLocalId, value: i128) -> Self {
        Self(EvaluatedCapture::from_kind(EvaluatedCaptureKind::Int {
            local,
            value: value.into(),
        }))
    }

    pub fn bool(local: BoolLocalId, value: bool) -> Self {
        Self(EvaluatedCapture::from_kind(EvaluatedCaptureKind::Bool {
            local,
            value,
        }))
    }

    pub fn float(local: FloatLocalId, value: f64) -> Self {
        Self(EvaluatedCapture::from_kind(EvaluatedCaptureKind::Float {
            local,
            value,
        }))
    }

    pub fn string(local: StringLocalId, value: StringValue) -> Self {
        Self(EvaluatedCapture::from_kind(EvaluatedCaptureKind::String {
            local,
            value,
        }))
    }

    pub fn bit_array(local: BitArrayLocalId, value: CallBitArray) -> Self {
        Self(EvaluatedCapture::from_kind(
            EvaluatedCaptureKind::BitArray {
                local,
                value: value.0,
            },
        ))
    }

    pub fn utf_codepoint(local: UtfCodepointLocalId, value: char) -> Self {
        Self(EvaluatedCapture::from_kind(
            EvaluatedCaptureKind::UtfCodepoint { local, value },
        ))
    }

    pub fn nil(local: NilLocalId, value: ()) -> Self {
        let () = value;
        Self(EvaluatedCapture::from_kind(EvaluatedCaptureKind::Nil {
            local,
        }))
    }

    pub fn int_list(local: IntListLocalId, value: IntList) -> Self {
        Self(EvaluatedCapture::list(EvaluatedListCapture::Int {
            local,
            value: value.0,
        }))
    }

    pub fn int_function(local: IntFunctionLocalId, value: IntCallable) -> Self {
        Self(EvaluatedCapture::from_kind(
            EvaluatedCaptureKind::IntFunction {
                local,
                value: value.0,
            },
        ))
    }

    pub fn bool_list(local: BoolListLocalId, value: BoolList) -> Self {
        Self(EvaluatedCapture::list(EvaluatedListCapture::Bool {
            local,
            value: value.0,
        }))
    }

    pub fn bool_function(local: BoolFunctionLocalId, value: BoolCallable) -> Self {
        Self(EvaluatedCapture::from_kind(
            EvaluatedCaptureKind::BoolFunction {
                local,
                value: value.0,
            },
        ))
    }

    pub fn float_list(local: FloatListLocalId, value: FloatList) -> Self {
        Self(EvaluatedCapture::list(EvaluatedListCapture::Float {
            local,
            value: value.0,
        }))
    }

    pub fn float_function(local: FloatFunctionLocalId, value: FloatCallable) -> Self {
        Self(EvaluatedCapture::from_kind(
            EvaluatedCaptureKind::FloatFunction {
                local,
                value: value.0,
            },
        ))
    }

    pub fn string_list(local: StringListLocalId, value: StringList) -> Self {
        Self(EvaluatedCapture::list(EvaluatedListCapture::String {
            local,
            value: value.0,
        }))
    }

    pub fn string_function(local: StringFunctionLocalId, value: StringCallable) -> Self {
        Self(EvaluatedCapture::from_kind(
            EvaluatedCaptureKind::StringFunction {
                local,
                value: value.0,
            },
        ))
    }

    pub fn bit_array_list(local: BitArrayListLocalId, value: BitArrayList) -> Self {
        Self(EvaluatedCapture::list(EvaluatedListCapture::BitArray {
            local,
            value: value.0,
        }))
    }

    pub fn bit_array_function(local: BitArrayFunctionLocalId, value: BitArrayCallable) -> Self {
        Self(EvaluatedCapture::from_kind(
            EvaluatedCaptureKind::BitArrayFunction {
                local,
                value: value.0,
            },
        ))
    }

    pub fn utf_codepoint_list(local: UtfCodepointListLocalId, value: UtfCodepointList) -> Self {
        Self(EvaluatedCapture::list(EvaluatedListCapture::UtfCodepoint {
            local,
            value: value.0,
        }))
    }

    pub fn utf_codepoint_function(
        local: UtfCodepointFunctionLocalId,
        value: UtfCodepointCallable,
    ) -> Self {
        Self(EvaluatedCapture::from_kind(
            EvaluatedCaptureKind::UtfCodepointFunction {
                local,
                value: value.0,
            },
        ))
    }

    pub fn nil_list(local: NilListLocalId, value: NilList) -> Self {
        Self(EvaluatedCapture::list(EvaluatedListCapture::Nil {
            local,
            value: value.0,
        }))
    }

    pub fn nil_function(local: NilFunctionLocalId, value: NilCallable) -> Self {
        Self(EvaluatedCapture::from_kind(
            EvaluatedCaptureKind::NilFunction {
                local,
                value: value.0,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::CallCaptureInputs;
    use crate::plan::execution::function::{
        BitArrayFunctionId, BoolFunctionId, FloatFunctionId, IntFunctionId, NilFunctionId,
        StringFunctionId, UtfCodepointFunctionId,
    };
    use crate::plan::execution::graph::{
        BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId, BoolFunctionLocalId,
        BoolListLocalId, BoolLocalId, FloatFunctionLocalId, FloatListLocalId, FloatLocalId,
        IntFunctionLocalId, IntListLocalId, IntLocalId, NilFunctionLocalId, NilListLocalId,
        NilLocalId, StringFunctionLocalId, StringListLocalId, StringLocalId,
        UtfCodepointFunctionLocalId, UtfCodepointListLocalId, UtfCodepointLocalId,
    };
    use crate::plan::execution::type_::{
        BitArrayListTypeId, BoolListTypeId, FloatListTypeId, FunctionType, IntListTypeId,
        ListTypeId, NilListTypeId, StringListTypeId, UtfCodepointListTypeId, ValueType,
    };
    use crate::runtime::CaptureStorage;
    use crate::runtime::compiled::calls::{CallBitArray, CallCapture, CallOps};
    use crate::runtime::compiled::numeric::NumericValues;
    use crate::runtime::evaluated::EvaluatedBitArray;
    use crate::runtime::state::list::RuntimeListStorage;
    use crate::{BitArrayValue, StringValue};

    #[test]
    fn capture_projection_keeps_exact_local_family_value_and_owner() {
        let storage = CaptureStorage::default();
        let mut numeric = NumericValues::default();
        let mut string_scratch = None;
        let mut bit_scratch = None;
        let lists = RuntimeListStorage::default();
        let ops = CallOps::new(
            &storage,
            &mut numeric,
            &lists,
            &mut string_scratch,
            &mut bit_scratch,
        );
        let int = 7_i128;
        let int_list = ops.lists().value(
            IntListTypeId {
                list_type: ListTypeId(0),
            },
            &[int as i64],
        );
        let int_function = ops.int_reference(
            IntFunctionId(3),
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
        );
        let bool = true;
        let bool_list = ops.primitive_lists().bool_value(
            BoolListTypeId {
                list_type: ListTypeId(0),
            },
            &[bool],
        );
        let bool_function = ops.bool_reference(
            BoolFunctionId(3),
            FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
        );
        let float = 3.5;
        let float_list = ops.primitive_lists().float_value(
            FloatListTypeId {
                list_type: ListTypeId(0),
            },
            &[float],
        );
        let float_function = ops.float_reference(
            FloatFunctionId(3),
            FunctionType::new(vec![ValueType::Float], ValueType::Float),
        );
        let string = StringValue::from("shared text beyond the call");
        let string_list = ops.primitive_lists().string_value(
            StringListTypeId {
                list_type: ListTypeId(0),
            },
            std::slice::from_ref(&string),
        );
        let string_function = ops.string_reference(
            StringFunctionId(3),
            FunctionType::new(vec![ValueType::String], ValueType::String),
        );
        let bit_array = CallBitArray(EvaluatedBitArray::from_value(
            BitArrayValue::try_from_parts(vec![0xA0], 3).unwrap(),
        ));
        let bit_array_list = ops.primitive_lists().bit_array_value(
            BitArrayListTypeId {
                list_type: ListTypeId(0),
            },
            std::slice::from_ref(&bit_array),
        );
        let bit_array_function = ops.bit_array_reference(
            BitArrayFunctionId(3),
            FunctionType::new(vec![ValueType::BitArray], ValueType::BitArray),
        );
        let utf_codepoint = 'λ';
        let utf_codepoint_list = ops.primitive_lists().utf_codepoint_value(
            UtfCodepointListTypeId {
                list_type: ListTypeId(0),
            },
            &[utf_codepoint],
        );
        let utf_codepoint_function = ops.utf_codepoint_reference(
            UtfCodepointFunctionId(3),
            FunctionType::new(vec![ValueType::UtfCodepoint], ValueType::UtfCodepoint),
        );
        let nil = ();
        let nil_list = ops.primitive_lists().nil_value(
            NilListTypeId {
                list_type: ListTypeId(0),
            },
            1,
        );
        let nil_function = ops.nil_reference(
            NilFunctionId(3),
            FunctionType::new(vec![ValueType::Nil], ValueType::Nil),
        );
        let captures = storage.capture(vec![
            CallCapture::int(IntLocalId(2), int).0,
            CallCapture::int_list(IntListLocalId(4), int_list.clone()).0,
            CallCapture::int_function(IntFunctionLocalId(6), int_function.clone()).0,
            CallCapture::bool(BoolLocalId(2), bool).0,
            CallCapture::bool_list(BoolListLocalId(4), bool_list.clone()).0,
            CallCapture::bool_function(BoolFunctionLocalId(6), bool_function.clone()).0,
            CallCapture::float(FloatLocalId(2), float).0,
            CallCapture::float_list(FloatListLocalId(4), float_list.clone()).0,
            CallCapture::float_function(FloatFunctionLocalId(6), float_function.clone()).0,
            CallCapture::string(StringLocalId(2), string.clone()).0,
            CallCapture::string_list(StringListLocalId(4), string_list.clone()).0,
            CallCapture::string_function(StringFunctionLocalId(6), string_function.clone()).0,
            CallCapture::bit_array(BitArrayLocalId(2), bit_array.clone()).0,
            CallCapture::bit_array_list(BitArrayListLocalId(4), bit_array_list.clone()).0,
            CallCapture::bit_array_function(BitArrayFunctionLocalId(6), bit_array_function.clone())
                .0,
            CallCapture::utf_codepoint(UtfCodepointLocalId(2), utf_codepoint).0,
            CallCapture::utf_codepoint_list(UtfCodepointListLocalId(4), utf_codepoint_list.clone())
                .0,
            CallCapture::utf_codepoint_function(
                UtfCodepointFunctionLocalId(6),
                utf_codepoint_function.clone(),
            )
            .0,
            CallCapture::nil(NilLocalId(2), nil).0,
            CallCapture::nil_list(NilListLocalId(4), nil_list.clone()).0,
            CallCapture::nil_function(NilFunctionLocalId(6), nil_function.clone()).0,
        ]);
        let inputs = CallCaptureInputs(&captures);
        assert_eq!(inputs.int(IntLocalId(2)), Some(int));
        assert!(inputs.int(IntLocalId(1)).is_none());
        assert_eq!(inputs.int_list(IntListLocalId(4)).unwrap().0, int_list.0);
        assert!(inputs.int_list(IntListLocalId(1)).is_none());
        assert_eq!(
            inputs.int_function(IntFunctionLocalId(6)).unwrap().0,
            int_function.0
        );
        assert!(inputs.int_function(IntFunctionLocalId(1)).is_none());
        assert_eq!(inputs.bool(BoolLocalId(2)), Some(bool));
        assert!(inputs.bool(BoolLocalId(1)).is_none());
        assert_eq!(inputs.bool_list(BoolListLocalId(4)).unwrap().0, bool_list.0);
        assert!(inputs.bool_list(BoolListLocalId(1)).is_none());
        assert_eq!(
            inputs.bool_function(BoolFunctionLocalId(6)).unwrap().0,
            bool_function.0
        );
        assert!(inputs.bool_function(BoolFunctionLocalId(1)).is_none());
        assert_eq!(inputs.float(FloatLocalId(2)), Some(float));
        assert!(inputs.float(FloatLocalId(1)).is_none());
        assert_eq!(
            inputs.float_list(FloatListLocalId(4)).unwrap().0,
            float_list.0
        );
        assert!(inputs.float_list(FloatListLocalId(1)).is_none());
        assert_eq!(
            inputs.float_function(FloatFunctionLocalId(6)).unwrap().0,
            float_function.0
        );
        assert!(inputs.float_function(FloatFunctionLocalId(1)).is_none());
        assert_eq!(inputs.string(StringLocalId(2)), Some(string));
        assert!(inputs.string(StringLocalId(1)).is_none());
        assert_eq!(
            inputs.string_list(StringListLocalId(4)).unwrap().0,
            string_list.0
        );
        assert!(inputs.string_list(StringListLocalId(1)).is_none());
        assert_eq!(
            inputs.string_function(StringFunctionLocalId(6)).unwrap().0,
            string_function.0
        );
        assert!(inputs.string_function(StringFunctionLocalId(1)).is_none());
        assert!(
            inputs
                .bit_array(BitArrayLocalId(2))
                .is_some_and(|actual| actual == bit_array)
        );
        assert!(inputs.bit_array(BitArrayLocalId(1)).is_none());
        assert_eq!(
            inputs.bit_array_list(BitArrayListLocalId(4)).unwrap().0,
            bit_array_list.0
        );
        assert!(inputs.bit_array_list(BitArrayListLocalId(1)).is_none());
        assert_eq!(
            inputs
                .bit_array_function(BitArrayFunctionLocalId(6))
                .unwrap()
                .0,
            bit_array_function.0
        );
        assert!(
            inputs
                .bit_array_function(BitArrayFunctionLocalId(1))
                .is_none()
        );
        assert_eq!(
            inputs.utf_codepoint(UtfCodepointLocalId(2)),
            Some(utf_codepoint)
        );
        assert!(inputs.utf_codepoint(UtfCodepointLocalId(1)).is_none());
        assert_eq!(
            inputs
                .utf_codepoint_list(UtfCodepointListLocalId(4))
                .unwrap()
                .0,
            utf_codepoint_list.0
        );
        assert!(
            inputs
                .utf_codepoint_list(UtfCodepointListLocalId(1))
                .is_none()
        );
        assert_eq!(
            inputs
                .utf_codepoint_function(UtfCodepointFunctionLocalId(6))
                .unwrap()
                .0,
            utf_codepoint_function.0
        );
        assert!(
            inputs
                .utf_codepoint_function(UtfCodepointFunctionLocalId(1))
                .is_none()
        );
        assert_eq!(inputs.nil(NilLocalId(2)), Some(nil));
        assert!(inputs.nil(NilLocalId(1)).is_none());
        assert_eq!(inputs.nil_list(NilListLocalId(4)).unwrap().0, nil_list.0);
        assert!(inputs.nil_list(NilListLocalId(1)).is_none());
        assert_eq!(
            inputs.nil_function(NilFunctionLocalId(6)).unwrap().0,
            nil_function.0
        );
        assert!(inputs.nil_function(NilFunctionLocalId(1)).is_none());
        let retained = inputs.retain();
        drop(captures);
        assert_eq!(CallCaptureInputs(&retained.0).nil(NilLocalId(2)), Some(()));
    }
}
