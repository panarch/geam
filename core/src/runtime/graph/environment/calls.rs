use super::{BlockEnvironment, RetainedValues};
use crate::StringValue;
use crate::plan::execution::function::{
    BitArrayFunctionId, BoolFunctionId, FloatFunctionId, IntFunctionId, NilFunctionId,
    StringFunctionId, UtfCodepointFunctionId,
};
use crate::runtime::compiled::calls::{
    BitArrayCallable, BoolCallable, CallArguments, CallBitArray, CallInputs, CallNativeInput,
    CallNullary, CallValues, FloatCallable, IntCallable, NilCallable, StringCallable,
    UtfCodepointCallable,
};
use crate::runtime::compiled::int_list::IntList;
use crate::runtime::compiled::primitive_list::{
    BitArrayList, BoolList, FloatList, NilList, StringList, UtfCodepointList,
};
use crate::runtime::evaluated::EvaluatedFunction;

impl BlockEnvironment {
    pub(in crate::runtime::graph) fn call_inputs(&self) -> CallInputs<'_> {
        CallInputs::new(self)
    }

    pub(in crate::runtime::graph) fn clear_call_values(&mut self) {
        self.values.customs.clear();
        self.values.ints.clear();
        self.values.bools.clear();
        self.values.floats.clear();
        self.values.strings.clear();
        self.values.bit_arrays.clear();
        self.values.utf_codepoints.clear();
        self.values.int_lists.clear();
        self.values.bool_lists.clear();
        self.values.float_lists.clear();
        self.values.string_lists.clear();
        self.values.bit_array_lists.clear();
        self.values.utf_codepoint_lists.clear();
        self.values.nil_lists.clear();
        self.values.int_functions.clear();
        self.values.bool_functions.clear();
        self.values.float_functions.clear();
        self.values.string_functions.clear();
        self.values.bit_array_functions.clear();
        self.values.utf_codepoint_functions.clear();
        self.values.nil_functions.clear();
    }
}

impl CallNativeInput {
    pub(in crate::runtime) fn into_retained(self) -> RetainedValues {
        let mut inputs = RetainedValues::empty();
        match self {
            Self::Int(value) => inputs.values.ints = vec![value.0],
            Self::Bool(value) => inputs.values.bools = vec![value],
        }
        inputs
    }
}

impl CallValues {
    pub(in crate::runtime) fn into_retained(self) -> RetainedValues {
        let mut inputs = RetainedValues::empty();
        inputs.values.customs = self
            .nullaries
            .into_iter()
            .map(|value| value.into_evaluated())
            .collect();
        inputs.values.ints = self.ints.into_iter().map(|value| value.0).collect();
        inputs.values.bools = self.bools;
        inputs.values.floats = self.floats;
        inputs.values.strings = self.strings;
        inputs.values.bit_arrays = self.bit_arrays.into_iter().map(|value| value.0).collect();
        inputs.values.utf_codepoints = self.utf_codepoints;
        inputs.values.int_lists = self.int_lists.into_iter().map(|value| value.0).collect();
        inputs.values.bool_lists = self.bool_lists.into_iter().map(|value| value.0).collect();
        inputs.values.float_lists = self.float_lists.into_iter().map(|value| value.0).collect();
        inputs.values.string_lists = self.string_lists.into_iter().map(|value| value.0).collect();
        inputs.values.bit_array_lists = self
            .bit_array_lists
            .into_iter()
            .map(|value| value.0)
            .collect();
        inputs.values.utf_codepoint_lists = self
            .utf_codepoint_lists
            .into_iter()
            .map(|value| value.0)
            .collect();
        inputs.values.nil_lists = self.nil_lists.into_iter().map(|value| value.0).collect();
        inputs.values.int_functions = self
            .int_functions
            .into_iter()
            .map(|value| value.0)
            .collect();
        inputs.values.bool_functions = self
            .bool_functions
            .into_iter()
            .map(|value| value.0)
            .collect();
        inputs.values.float_functions = self
            .float_functions
            .into_iter()
            .map(|value| value.0)
            .collect();
        inputs.values.string_functions = self
            .string_functions
            .into_iter()
            .map(|value| value.0)
            .collect();
        inputs.values.bit_array_functions = self
            .bit_array_functions
            .into_iter()
            .map(|value| value.0)
            .collect();
        inputs.values.utf_codepoint_functions = self
            .utf_codepoint_functions
            .into_iter()
            .map(|value| value.0)
            .collect();
        inputs.values.nil_functions = self
            .nil_functions
            .into_iter()
            .map(|value| value.0)
            .collect();
        inputs
    }
}

impl CallInputs<'_> {
    pub fn nullary(
        &self,
        index: usize,
        constructors: &[crate::plan::execution::type_::CustomConstructorId],
    ) -> Option<CallNullary> {
        let value = self.0.values.customs.get(index)?;
        (value.fields().is_empty() && constructors.contains(&value.constructor()))
            .then(|| CallNullary::new(value.constructor()))
    }

    pub fn int(&self, index: usize) -> Option<i128> {
        self.0
            .values
            .ints
            .get(index)
            .and_then(|value| value.small())
            .map(i128::from)
    }
    pub fn bool(&self, index: usize) -> Option<bool> {
        self.0.values.bools.get(index).copied()
    }
    pub fn float(&self, index: usize) -> Option<f64> {
        self.0.values.floats.get(index).copied()
    }
    pub fn string(&self, index: usize) -> Option<StringValue> {
        self.0.values.strings.get(index).cloned()
    }
    pub fn bit_array(&self, index: usize) -> Option<CallBitArray> {
        self.0
            .values
            .bit_arrays
            .get(index)
            .cloned()
            .map(CallBitArray)
    }
    pub fn utf_codepoint(&self, index: usize) -> Option<char> {
        self.0.values.utf_codepoints.get(index).copied()
    }
    pub fn int_list(&self, index: usize) -> Option<IntList> {
        self.0.values.int_lists.get(index).cloned().map(IntList)
    }
    pub fn bool_list(&self, index: usize) -> Option<BoolList> {
        self.0.values.bool_lists.get(index).cloned().map(BoolList)
    }
    pub fn float_list(&self, index: usize) -> Option<FloatList> {
        self.0.values.float_lists.get(index).cloned().map(FloatList)
    }
    pub fn string_list(&self, index: usize) -> Option<StringList> {
        self.0
            .values
            .string_lists
            .get(index)
            .cloned()
            .map(StringList)
    }
    pub fn bit_array_list(&self, index: usize) -> Option<BitArrayList> {
        self.0
            .values
            .bit_array_lists
            .get(index)
            .cloned()
            .map(BitArrayList)
    }
    pub fn utf_codepoint_list(&self, index: usize) -> Option<UtfCodepointList> {
        self.0
            .values
            .utf_codepoint_lists
            .get(index)
            .cloned()
            .map(UtfCodepointList)
    }
    pub fn nil_list(&self, index: usize) -> Option<NilList> {
        self.0.values.nil_lists.get(index).cloned().map(NilList)
    }
    pub fn int_function(&self, index: usize) -> Option<IntCallable> {
        self.0
            .values
            .int_functions
            .get(index)
            .cloned()
            .map(IntCallable)
    }
    pub fn bool_function(&self, index: usize) -> Option<BoolCallable> {
        self.0
            .values
            .bool_functions
            .get(index)
            .cloned()
            .map(BoolCallable)
    }
    pub fn float_function(&self, index: usize) -> Option<FloatCallable> {
        self.0
            .values
            .float_functions
            .get(index)
            .cloned()
            .map(FloatCallable)
    }
    pub fn string_function(&self, index: usize) -> Option<StringCallable> {
        self.0
            .values
            .string_functions
            .get(index)
            .cloned()
            .map(StringCallable)
    }
    pub fn bit_array_function(&self, index: usize) -> Option<BitArrayCallable> {
        self.0
            .values
            .bit_array_functions
            .get(index)
            .cloned()
            .map(BitArrayCallable)
    }
    pub fn utf_codepoint_function(&self, index: usize) -> Option<UtfCodepointCallable> {
        self.0
            .values
            .utf_codepoint_functions
            .get(index)
            .cloned()
            .map(UtfCodepointCallable)
    }
    pub fn nil_function(&self, index: usize) -> Option<NilCallable> {
        self.0
            .values
            .nil_functions
            .get(index)
            .cloned()
            .map(NilCallable)
    }
    pub fn int_function_target(&self, index: usize) -> Option<IntFunctionId> {
        self.0
            .values
            .int_functions
            .get(index)
            .map(EvaluatedFunction::runtime_id)
    }
    pub fn bool_function_target(&self, index: usize) -> Option<BoolFunctionId> {
        self.0
            .values
            .bool_functions
            .get(index)
            .map(EvaluatedFunction::runtime_id)
    }
    pub fn float_function_target(&self, index: usize) -> Option<FloatFunctionId> {
        self.0
            .values
            .float_functions
            .get(index)
            .map(EvaluatedFunction::runtime_id)
    }
    pub fn string_function_target(&self, index: usize) -> Option<StringFunctionId> {
        self.0
            .values
            .string_functions
            .get(index)
            .map(EvaluatedFunction::runtime_id)
    }
    pub fn bit_array_function_target(&self, index: usize) -> Option<BitArrayFunctionId> {
        self.0
            .values
            .bit_array_functions
            .get(index)
            .map(EvaluatedFunction::runtime_id)
    }
    pub fn utf_codepoint_function_target(&self, index: usize) -> Option<UtfCodepointFunctionId> {
        self.0
            .values
            .utf_codepoint_functions
            .get(index)
            .map(EvaluatedFunction::runtime_id)
    }
    pub fn nil_function_target(&self, index: usize) -> Option<NilFunctionId> {
        self.0
            .values
            .nil_functions
            .get(index)
            .map(EvaluatedFunction::runtime_id)
    }
}

impl CallArguments {
    pub(in crate::runtime::graph) fn into_retained(self) -> RetainedValues {
        let mut inputs = self.values.into_retained();
        if let Some(captures) = self.captures {
            inputs.append_captures(&captures.0);
        }
        inputs
    }
}

#[cfg(test)]
mod tests {
    use super::{BlockEnvironment, RetainedValues};
    use crate::plan::execution::function::{BoolFunctionId, IntFunctionId};
    use crate::plan::execution::graph::IntListLocalId;
    use crate::plan::execution::type_::{FunctionType, ValueType};
    use crate::runtime::CaptureStorage;
    use crate::runtime::compiled::calls::{CallArguments, CallCapture, CallOps, CallValues};
    use crate::runtime::compiled::numeric::NumericValues;
    use crate::runtime::integer::IntegerValue;
    use crate::runtime::plan_src;
    use crate::runtime::state::list::RuntimeListStorage;

    #[test]
    fn nullary_entry_projection_is_read_only_and_rejects_foreign_constructors_and_fields() {
        use crate::plan::execution::type_::{CustomConstructorId, CustomTypeId};
        use crate::runtime::compiled::calls::CallNullary;
        use crate::runtime::evaluated::{EvaluatedCustomValue, EvaluatedValue};

        let before = CustomConstructorId {
            type_id: CustomTypeId(3),
            index: 7,
        };
        let after = CustomConstructorId {
            type_id: CustomTypeId(3),
            index: 8,
        };
        let foreign = CustomConstructorId {
            type_id: CustomTypeId(4),
            index: 7,
        };
        let mut values = RetainedValues::empty();
        values.push_custom(EvaluatedCustomValue::from_fields(before, Box::default()));
        values.push_custom(EvaluatedCustomValue::from_fields(
            before,
            vec![EvaluatedValue::Int(42.into())].into_boxed_slice(),
        ));
        let mut environment = BlockEnvironment::from_retained(values);
        let inputs = environment.call_inputs();
        assert!(inputs.nullary(0, &[before, after]).is_some());
        assert!(inputs.nullary(0, &[foreign]).is_none());
        assert!(inputs.nullary(0, &[after]).is_none());
        assert!(inputs.nullary(1, &[before]).is_none());
        assert!(inputs.nullary(2, &[before]).is_none());
        assert_eq!(environment.values.customs.len(), 2);
        assert_eq!(environment.values.customs[1].fields().len(), 1);
        let restored = CallValues {
            nullaries: vec![CallNullary::new(before)],
            ..Default::default()
        }
        .into_retained();
        let restored = BlockEnvironment::from_retained(restored);
        assert_eq!(restored.values.customs[0].constructor(), before);
        assert!(restored.values.customs[0].fields().is_empty());
        environment.clear_call_values();
        assert!(environment.values.customs.is_empty());
    }

    #[test]
    fn list_columns_move_through_entry_restoration_and_callee_capture_order() {
        let plan = plan_src("pub fn main() { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let lists = RuntimeListStorage::default();
        let captures = CaptureStorage::default();
        let mut numeric = NumericValues::default();
        let mut string_scratch = None;
        let mut bit_scratch = None;
        let ops = CallOps::new(
            &captures,
            &mut numeric,
            &lists,
            &mut string_scratch,
            &mut bit_scratch,
        );
        let argument = ops.lists().value(type_id, &[3, 4]);
        let captured = ops.lists().value(type_id, &[7, 8]);
        let values = CallValues {
            ints: vec![11_i128.into()],
            bools: vec![true],
            int_lists: vec![argument.clone()],
            ..CallValues::default()
        };
        let mut environment = BlockEnvironment::from_retained(values.into_retained());
        assert_eq!(
            environment.values.int_lists.as_slice(),
            std::slice::from_ref(&argument.0)
        );
        assert_eq!(environment.call_inputs().int_list(0).unwrap().0, argument.0);
        assert!(environment.call_inputs().int_list(1).is_none());
        let capacity = environment.values.int_lists.capacity();
        environment.clear_call_values();
        assert!(environment.values.int_lists.is_empty());
        assert_eq!(environment.values.int_lists.capacity(), capacity);
        environment = BlockEnvironment::from_retained(
            CallValues {
                int_lists: vec![captured.clone(), argument.clone()],
                ..CallValues::default()
            }
            .into_retained(),
        );
        assert_eq!(
            environment.values.int_lists,
            [captured.0.clone(), argument.0.clone()]
        );
        assert!(environment.values.ints.is_empty());
        assert!(environment.values.bools.is_empty());
        let function = ops.int_closure(
            IntFunctionId(2),
            FunctionType::new(Vec::new(), ValueType::Int),
            vec![CallCapture::int_list(IntListLocalId(1), captured.clone())],
        );
        let callee = CallArguments {
            values: Box::new(CallValues {
                int_lists: vec![argument.clone()],
                ..CallValues::default()
            }),
            captures: Some(function.captures().retain()),
        }
        .into_retained();
        assert_eq!(callee.values.int_lists, [argument.0, captured.0]);
    }
    #[test]
    fn entry_inputs_borrow_original_columns_and_clone_only_selected_handles() {
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
        let integer = ops.int_reference(
            IntFunctionId(2),
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
        );
        let boolean = ops.bool_reference(
            BoolFunctionId(3),
            FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
        );
        let ints = [IntegerValue::from(7), IntegerValue::from(1_i128 << 100)];
        let bools = [false, true];
        let int_functions = [integer.0];
        let bool_functions = [boolean.0];
        let plan = plan_src("pub fn main() { [1] }");
        let int_lists = [ops
            .lists()
            .value(plan.int_list_function_id(0).type_id(), &[3, 4])
            .0];
        let mut retained = RetainedValues::empty();
        retained.values.ints = ints.to_vec();
        retained.values.bools = bools.to_vec();
        retained.values.int_lists = int_lists.to_vec();
        retained.values.int_functions = int_functions.to_vec();
        retained.values.bool_functions = bool_functions.to_vec();
        let environment = BlockEnvironment::from_retained(retained);
        let inputs = environment.call_inputs();
        assert!(std::ptr::eq(inputs.0, &environment));
        assert!(std::ptr::eq(
            inputs.0.values.ints.as_ptr(),
            environment.values.ints.as_ptr()
        ));
        assert!(std::ptr::eq(
            inputs.0.values.bools.as_ptr(),
            environment.values.bools.as_ptr()
        ));
        assert!(std::ptr::eq(
            inputs.0.values.int_lists.as_ptr(),
            environment.values.int_lists.as_ptr()
        ));
        let selected_list = inputs.int_list(0).unwrap();
        assert_eq!(selected_list.0, int_lists[0]);
        assert!(std::ptr::eq(
            selected_list.0.values(),
            int_lists[0].values()
        ));
        assert_eq!(ops.lists().index(&selected_list, 1), Some(4));
        assert!(inputs.int_list(1).is_none());
        assert!(std::ptr::eq(
            inputs.0.values.int_functions.as_ptr(),
            environment.values.int_functions.as_ptr()
        ));
        assert!(std::ptr::eq(
            inputs.0.values.bool_functions.as_ptr(),
            environment.values.bool_functions.as_ptr()
        ));
        assert_eq!(inputs.int(0), Some(7));
        assert_eq!(inputs.int(1), None);
        assert_eq!(inputs.int(2), None);
        assert_eq!(inputs.bool(0), Some(false));
        assert_eq!(inputs.bool(1), Some(true));
        assert_eq!(inputs.bool(2), None);
        assert_eq!(inputs.int_function_target(0), Some(IntFunctionId(2)));
        assert_eq!(inputs.bool_function_target(0), Some(BoolFunctionId(3)));
        assert_eq!(inputs.int_function_target(1), None);
        assert_eq!(inputs.bool_function_target(1), None);
        assert_eq!(inputs.int_function(0).unwrap().0, int_functions[0]);
        assert_eq!(inputs.bool_function(0).unwrap().0, bool_functions[0]);
        assert!(inputs.int_function(1).is_none());
        assert!(inputs.bool_function(1).is_none());
        assert_eq!(
            ints,
            [IntegerValue::from(7), IntegerValue::from(1_i128 << 100)]
        );
        assert_eq!(bools, [false, true]);
    }
    #[test]
    fn every_primitive_column_borrows_projects_restores_and_releases_its_typed_owner() {
        use crate::plan::execution::function::{
            BitArrayFunctionId, BoolFunctionId, FloatFunctionId, IntFunctionId, NilFunctionId,
            StringFunctionId, UtfCodepointFunctionId,
        };
        use crate::plan::execution::type_::{
            BitArrayListTypeId, BoolListTypeId, FloatListTypeId, IntListTypeId, ListTypeId,
            NilListTypeId, StringListTypeId, UtfCodepointListTypeId,
        };
        use crate::runtime::compiled::calls::CallBitArray;
        use crate::runtime::evaluated::EvaluatedBitArray;
        use crate::{BitArrayValue, StringValue};
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
        let string = StringValue::from("shared input beyond the call");
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
        let values = CallValues {
            ints: vec![int.into()],
            int_lists: vec![int_list.clone()],
            int_functions: vec![int_function.clone()],
            bools: vec![bool],
            bool_lists: vec![bool_list.clone()],
            bool_functions: vec![bool_function.clone()],
            floats: vec![float],
            float_lists: vec![float_list.clone()],
            float_functions: vec![float_function.clone()],
            strings: vec![string.clone()],
            string_lists: vec![string_list.clone()],
            string_functions: vec![string_function.clone()],
            bit_arrays: vec![bit_array.clone()],
            bit_array_lists: vec![bit_array_list.clone()],
            bit_array_functions: vec![bit_array_function.clone()],
            utf_codepoints: vec![utf_codepoint],
            utf_codepoint_lists: vec![utf_codepoint_list.clone()],
            utf_codepoint_functions: vec![utf_codepoint_function.clone()],
            nil_lists: vec![nil_list.clone()],
            nil_functions: vec![nil_function.clone()],
            nullaries: Vec::new(),
        };
        let mut environment = BlockEnvironment::from_retained(values.into_retained());
        let inputs = environment.call_inputs();
        assert!(std::ptr::eq(inputs.0, &environment));
        assert_eq!(inputs.int(0), Some(int));
        assert!(inputs.int(1).is_none());
        assert_eq!(inputs.int_list(0).unwrap().0, int_list.0);
        assert!(inputs.int_list(1).is_none());
        assert_eq!(inputs.int_function(0).unwrap().0, int_function.0);
        assert!(inputs.int_function(1).is_none());
        assert_eq!(inputs.int_function_target(0), Some(IntFunctionId(3)));
        assert!(inputs.int_function_target(1).is_none());
        assert_eq!(inputs.bool(0), Some(bool));
        assert!(inputs.bool(1).is_none());
        assert_eq!(inputs.bool_list(0).unwrap().0, bool_list.0);
        assert!(inputs.bool_list(1).is_none());
        assert_eq!(inputs.bool_function(0).unwrap().0, bool_function.0);
        assert!(inputs.bool_function(1).is_none());
        assert_eq!(inputs.bool_function_target(0), Some(BoolFunctionId(3)));
        assert!(inputs.bool_function_target(1).is_none());
        assert_eq!(inputs.float(0), Some(float));
        assert!(inputs.float(1).is_none());
        assert_eq!(inputs.float_list(0).unwrap().0, float_list.0);
        assert!(inputs.float_list(1).is_none());
        assert_eq!(inputs.float_function(0).unwrap().0, float_function.0);
        assert!(inputs.float_function(1).is_none());
        assert_eq!(inputs.float_function_target(0), Some(FloatFunctionId(3)));
        assert!(inputs.float_function_target(1).is_none());
        assert_eq!(inputs.string(0), Some(string.clone()));
        assert!(inputs.string(1).is_none());
        assert_eq!(inputs.string_list(0).unwrap().0, string_list.0);
        assert!(inputs.string_list(1).is_none());
        assert_eq!(inputs.string_function(0).unwrap().0, string_function.0);
        assert!(inputs.string_function(1).is_none());
        assert_eq!(inputs.string_function_target(0), Some(StringFunctionId(3)));
        assert!(inputs.string_function_target(1).is_none());
        assert!(
            inputs
                .bit_array(0)
                .is_some_and(|actual| actual == bit_array)
        );
        assert!(inputs.bit_array(1).is_none());
        assert_eq!(inputs.bit_array_list(0).unwrap().0, bit_array_list.0);
        assert!(inputs.bit_array_list(1).is_none());
        assert_eq!(
            inputs.bit_array_function(0).unwrap().0,
            bit_array_function.0
        );
        assert!(inputs.bit_array_function(1).is_none());
        assert_eq!(
            inputs.bit_array_function_target(0),
            Some(BitArrayFunctionId(3))
        );
        assert!(inputs.bit_array_function_target(1).is_none());
        assert_eq!(inputs.utf_codepoint(0), Some(utf_codepoint));
        assert!(inputs.utf_codepoint(1).is_none());
        assert_eq!(
            inputs.utf_codepoint_list(0).unwrap().0,
            utf_codepoint_list.0
        );
        assert!(inputs.utf_codepoint_list(1).is_none());
        assert_eq!(
            inputs.utf_codepoint_function(0).unwrap().0,
            utf_codepoint_function.0
        );
        assert!(inputs.utf_codepoint_function(1).is_none());
        assert_eq!(
            inputs.utf_codepoint_function_target(0),
            Some(UtfCodepointFunctionId(3))
        );
        assert!(inputs.utf_codepoint_function_target(1).is_none());
        assert_eq!(inputs.nil_list(0).unwrap().0, nil_list.0);
        assert!(inputs.nil_list(1).is_none());
        assert_eq!(inputs.nil_function(0).unwrap().0, nil_function.0);
        assert!(inputs.nil_function(1).is_none());
        assert_eq!(inputs.nil_function_target(0), Some(NilFunctionId(3)));
        assert!(inputs.nil_function_target(1).is_none());
        environment.clear_call_values();
        assert!(environment.values.ints.is_empty());
        assert!(environment.values.int_lists.is_empty());
        assert!(environment.values.int_functions.is_empty());
        assert!(environment.values.bools.is_empty());
        assert!(environment.values.bool_lists.is_empty());
        assert!(environment.values.bool_functions.is_empty());
        assert!(environment.values.floats.is_empty());
        assert!(environment.values.float_lists.is_empty());
        assert!(environment.values.float_functions.is_empty());
        assert!(environment.values.strings.is_empty());
        assert!(environment.values.string_lists.is_empty());
        assert!(environment.values.string_functions.is_empty());
        assert!(environment.values.bit_arrays.is_empty());
        assert!(environment.values.bit_array_lists.is_empty());
        assert!(environment.values.bit_array_functions.is_empty());
        assert!(environment.values.utf_codepoints.is_empty());
        assert!(environment.values.utf_codepoint_lists.is_empty());
        assert!(environment.values.utf_codepoint_functions.is_empty());
        assert!(environment.values.nil_lists.is_empty());
        assert!(environment.values.nil_functions.is_empty());
        environment = BlockEnvironment::from_retained(
            CallValues {
                ints: vec![int.into()],
                int_lists: vec![int_list.clone()],
                int_functions: vec![int_function.clone()],
                bools: vec![bool],
                bool_lists: vec![bool_list.clone()],
                bool_functions: vec![bool_function.clone()],
                floats: vec![float],
                float_lists: vec![float_list.clone()],
                float_functions: vec![float_function.clone()],
                strings: vec![string.clone()],
                string_lists: vec![string_list.clone()],
                string_functions: vec![string_function.clone()],
                bit_arrays: vec![bit_array.clone()],
                bit_array_lists: vec![bit_array_list.clone()],
                bit_array_functions: vec![bit_array_function.clone()],
                utf_codepoints: vec![utf_codepoint],
                utf_codepoint_lists: vec![utf_codepoint_list.clone()],
                utf_codepoint_functions: vec![utf_codepoint_function.clone()],
                nil_lists: vec![nil_list.clone()],
                nil_functions: vec![nil_function.clone()],
                nullaries: Vec::new(),
            }
            .into_retained(),
        );
        assert_eq!(environment.call_inputs().int(0), Some(int));
        assert_eq!(environment.call_inputs().int_list(0).unwrap().0, int_list.0);
        assert_eq!(
            environment.call_inputs().int_function(0).unwrap().0,
            int_function.0
        );
        assert_eq!(environment.call_inputs().bool(0), Some(bool));
        assert_eq!(
            environment.call_inputs().bool_list(0).unwrap().0,
            bool_list.0
        );
        assert_eq!(
            environment.call_inputs().bool_function(0).unwrap().0,
            bool_function.0
        );
        assert_eq!(environment.call_inputs().float(0), Some(float));
        assert_eq!(
            environment.call_inputs().float_list(0).unwrap().0,
            float_list.0
        );
        assert_eq!(
            environment.call_inputs().float_function(0).unwrap().0,
            float_function.0
        );
        assert_eq!(environment.call_inputs().string(0), Some(string));
        assert_eq!(
            environment.call_inputs().string_list(0).unwrap().0,
            string_list.0
        );
        assert_eq!(
            environment.call_inputs().string_function(0).unwrap().0,
            string_function.0
        );
        assert!(
            environment
                .call_inputs()
                .bit_array(0)
                .is_some_and(|actual| actual == bit_array)
        );
        assert_eq!(
            environment.call_inputs().bit_array_list(0).unwrap().0,
            bit_array_list.0
        );
        assert_eq!(
            environment.call_inputs().bit_array_function(0).unwrap().0,
            bit_array_function.0
        );
        assert_eq!(
            environment.call_inputs().utf_codepoint(0),
            Some(utf_codepoint)
        );
        assert_eq!(
            environment.call_inputs().utf_codepoint_list(0).unwrap().0,
            utf_codepoint_list.0
        );
        assert_eq!(
            environment
                .call_inputs()
                .utf_codepoint_function(0)
                .unwrap()
                .0,
            utf_codepoint_function.0
        );
        assert_eq!(environment.call_inputs().nil_list(0).unwrap().0, nil_list.0);
        assert_eq!(
            environment.call_inputs().nil_function(0).unwrap().0,
            nil_function.0
        );
    }
}
