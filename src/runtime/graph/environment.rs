use crate::host::{
    HostBitArrayArgumentSlot, HostBoolArgumentSlot, HostCallArguments, HostFloatArgumentSlot,
    HostIntArgumentSlot, HostNilArgumentSlot, HostStringArgumentSlot, HostUtfCodepointArgumentSlot,
};
use crate::plan::execution::graph::{
    BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId, BoolFunctionLocalId,
    BoolListLocalId, BoolLocalId, CustomFunctionLocal, CustomListLocalId, CustomLocal,
    FloatFunctionLocalId, FloatListLocalId, FloatLocalId, FunctionFunctionLocal,
    FunctionListLocalId, GenericFunctionLocal, IntFunctionLocalId, IntListLocalId, IntLocalId,
    ListFunctionLocal, ListListLocalId, ListLocal, NeverFunctionLocal, NilFunctionLocalId,
    NilListLocalId, NilLocalId, ParamLocal, ParameterListListLocalId, ParameterListLocalId,
    StringFunctionLocalId, StringListLocalId, StringLocalId, TupleFunctionLocalId,
    TupleListLocalId, TupleLocalId, UtfCodepointFunctionLocalId, UtfCodepointListLocalId,
    UtfCodepointLocalId,
};
use crate::runtime::evaluated::{
    EvaluatedBitArray, EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCapture,
    EvaluatedCaptureKind, EvaluatedCustomFunction, EvaluatedCustomValue, EvaluatedFloatFunction,
    EvaluatedFunctionFunction, EvaluatedFunctionValue, EvaluatedGenericFunction,
    EvaluatedIntFunction, EvaluatedListCapture, EvaluatedListFunction, EvaluatedNeverFunction,
    EvaluatedNilFunction, EvaluatedStringFunction, EvaluatedTupleFunction,
    EvaluatedUtfCodepointFunction, EvaluatedValue,
};
use crate::runtime::state::{
    BitArrayListValueId, BoolListValueId, CustomListValueId, FloatListValueId, FunctionListValueId,
    IntListValueId, ListListValueId, ListValueId, NilListValueId, ParameterListListValueId,
    ParameterListValueId, StoredListValueId, StringListValueId, TupleListValueId,
    UtfCodepointListValueId,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Default)]
struct BlockValues {
    ints: Vec<BigInt>,
    floats: Vec<f64>,
    strings: Vec<EcoString>,
    bit_arrays: Vec<EvaluatedBitArray>,
    utf_codepoints: Vec<char>,
    customs: Vec<EvaluatedCustomValue>,
    bools: Vec<bool>,
    tuples: Vec<Vec<EvaluatedValue>>,
    parameter_lists: Vec<ParameterListValueId>,
    int_lists: Vec<IntListValueId>,
    string_lists: Vec<StringListValueId>,
    bit_array_lists: Vec<BitArrayListValueId>,
    utf_codepoint_lists: Vec<UtfCodepointListValueId>,
    custom_lists: Vec<CustomListValueId>,
    float_lists: Vec<FloatListValueId>,
    bool_lists: Vec<BoolListValueId>,
    nil_lists: Vec<NilListValueId>,
    tuple_lists: Vec<TupleListValueId>,
    parameter_list_lists: Vec<ParameterListListValueId>,
    list_lists: Vec<ListListValueId>,
    function_lists: Vec<FunctionListValueId>,
    int_functions: Vec<EvaluatedIntFunction>,
    float_functions: Vec<EvaluatedFloatFunction>,
    string_functions: Vec<EvaluatedStringFunction>,
    bit_array_functions: Vec<EvaluatedBitArrayFunction>,
    utf_codepoint_functions: Vec<EvaluatedUtfCodepointFunction>,
    custom_functions: Vec<EvaluatedCustomFunction>,
    bool_functions: Vec<EvaluatedBoolFunction>,
    nil_functions: Vec<EvaluatedNilFunction>,
    tuple_functions: Vec<EvaluatedTupleFunction>,
    parameter_list_functions: Vec<EvaluatedListFunction>,
    parameter_list_list_functions: Vec<EvaluatedListFunction>,
    int_list_functions: Vec<EvaluatedListFunction>,
    string_list_functions: Vec<EvaluatedListFunction>,
    bit_array_list_functions: Vec<EvaluatedListFunction>,
    utf_codepoint_list_functions: Vec<EvaluatedListFunction>,
    custom_list_functions: Vec<EvaluatedListFunction>,
    float_list_functions: Vec<EvaluatedListFunction>,
    bool_list_functions: Vec<EvaluatedListFunction>,
    nil_list_functions: Vec<EvaluatedListFunction>,
    tuple_list_functions: Vec<EvaluatedListFunction>,
    list_list_functions: Vec<EvaluatedListFunction>,
    function_list_functions: Vec<EvaluatedListFunction>,
    function_functions: Vec<EvaluatedFunctionFunction>,
    generic_functions: Vec<EvaluatedGenericFunction>,
    never_functions: Vec<EvaluatedNeverFunction>,
}

pub(super) struct BlockEnvironment {
    values: Box<BlockValues>,
}

pub(in crate::runtime) struct RetainedValues {
    values: Box<BlockValues>,
}

impl BlockEnvironment {
    pub(super) fn from_retained(values: RetainedValues) -> Self {
        Self {
            values: values.values,
        }
    }

    pub(super) fn retain(&self, locals: &[ParamLocal]) -> RetainedValues {
        let mut retained = RetainedValues::empty();
        for local in locals {
            retained.push_local(self, local);
        }
        retained
    }

    pub(super) fn value(&self, local: &ParamLocal) -> EvaluatedValue {
        match local {
            ParamLocal::Int(local) => EvaluatedValue::Int(self.int(*local)),
            ParamLocal::Float(local) => EvaluatedValue::Float(self.float(*local)),
            ParamLocal::String(local) => EvaluatedValue::String(self.string(*local)),
            ParamLocal::BitArray(local) => EvaluatedValue::BitArray(self.bit_array(*local)),
            ParamLocal::UtfCodepoint(local) => {
                EvaluatedValue::UtfCodepoint(self.utf_codepoint(*local))
            }
            ParamLocal::Custom(local) => EvaluatedValue::Custom(self.custom(*local)),
            ParamLocal::Bool(local) => EvaluatedValue::Bool(self.bool(*local)),
            ParamLocal::Nil(local) => {
                self.nil(*local);
                EvaluatedValue::Nil
            }
            ParamLocal::Tuple { local, .. } => EvaluatedValue::Tuple(self.tuple(*local)),
            ParamLocal::List(local) => EvaluatedValue::List(self.list(local)),
            ParamLocal::IntFunction { local, .. } => {
                EvaluatedValue::Function(self.int_function(*local).into())
            }
            ParamLocal::FloatFunction { local, .. } => {
                EvaluatedValue::Function(self.float_function(*local).into())
            }
            ParamLocal::StringFunction { local, .. } => {
                EvaluatedValue::Function(self.string_function(*local).into())
            }
            ParamLocal::BitArrayFunction { local, .. } => {
                EvaluatedValue::Function(self.bit_array_function(*local).into())
            }
            ParamLocal::UtfCodepointFunction { local, .. } => {
                EvaluatedValue::Function(self.utf_codepoint_function(*local).into())
            }
            ParamLocal::GenericFunction(local) => {
                EvaluatedValue::Function(self.generic_function(local).into())
            }
            ParamLocal::NeverFunction(local) => {
                EvaluatedValue::Function(self.never_function(local).into())
            }
            ParamLocal::CustomFunction(local) => {
                EvaluatedValue::Function(self.custom_function(local).into())
            }
            ParamLocal::BoolFunction { local, .. } => {
                EvaluatedValue::Function(self.bool_function(*local).into())
            }
            ParamLocal::NilFunction { local, .. } => {
                EvaluatedValue::Function(self.nil_function(*local).into())
            }
            ParamLocal::TupleFunction { local, .. } => {
                EvaluatedValue::Function(self.tuple_function(*local).into())
            }
            ParamLocal::ListFunction(local) => {
                EvaluatedValue::Function(self.list_function(local).into())
            }
            ParamLocal::FunctionFunction(local) => {
                EvaluatedValue::Function(self.function_function(local).into())
            }
        }
    }

    pub(super) fn values(&self, locals: &[ParamLocal]) -> Box<[EvaluatedValue]> {
        locals
            .iter()
            .map(|local| self.value(local))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub(super) fn push_int(&mut self, value: BigInt) {
        self.values.ints.push(value);
    }

    pub(super) fn int(&self, local: IntLocalId) -> BigInt {
        self.values.ints[local.0].clone()
    }

    pub(super) fn push_float(&mut self, value: f64) {
        self.values.floats.push(value);
    }

    pub(super) fn float(&self, local: FloatLocalId) -> f64 {
        self.values.floats[local.0]
    }

    pub(super) fn push_string(&mut self, value: EcoString) {
        self.values.strings.push(value);
    }

    pub(super) fn string(&self, local: StringLocalId) -> EcoString {
        self.values.strings[local.0].clone()
    }

    pub(super) fn push_bit_array(&mut self, value: EvaluatedBitArray) {
        self.values.bit_arrays.push(value);
    }

    pub(super) fn bit_array(&self, local: BitArrayLocalId) -> EvaluatedBitArray {
        self.values.bit_arrays[local.0].clone()
    }

    pub(super) fn push_utf_codepoint(&mut self, value: char) {
        self.values.utf_codepoints.push(value);
    }

    pub(super) fn utf_codepoint(&self, local: UtfCodepointLocalId) -> char {
        self.values.utf_codepoints[local.0]
    }

    pub(super) fn push_custom(&mut self, value: EvaluatedCustomValue) {
        self.values.customs.push(value);
    }

    pub(super) fn custom(&self, local: CustomLocal) -> EvaluatedCustomValue {
        self.values.customs[local.id().0].clone()
    }

    pub(super) fn push_bool(&mut self, value: bool) {
        self.values.bools.push(value);
    }

    pub(super) fn bool(&self, local: BoolLocalId) -> bool {
        self.values.bools[local.0]
    }

    pub(super) fn push_nil(&mut self) {}

    pub(super) fn nil(&self, _local: NilLocalId) {}

    pub(super) fn push_tuple(&mut self, value: Vec<EvaluatedValue>) {
        self.values.tuples.push(value);
    }

    pub(super) fn tuple(&self, local: TupleLocalId) -> Vec<EvaluatedValue> {
        self.values.tuples[local.0].clone()
    }

    pub(super) fn push_parameter_list(&mut self, value: ParameterListValueId) {
        self.values.parameter_lists.push(value);
    }

    pub(super) fn parameter_list(&self, local: ParameterListLocalId) -> ParameterListValueId {
        self.values.parameter_lists[local.0]
    }

    pub(super) fn push_int_list(&mut self, value: IntListValueId) {
        self.values.int_lists.push(value);
    }

    pub(super) fn int_list(&self, local: IntListLocalId) -> IntListValueId {
        self.values.int_lists[local.0].clone()
    }

    pub(super) fn push_string_list(&mut self, value: StringListValueId) {
        self.values.string_lists.push(value);
    }

    pub(super) fn string_list(&self, local: StringListLocalId) -> StringListValueId {
        self.values.string_lists[local.0].clone()
    }

    pub(super) fn push_bit_array_list(&mut self, value: BitArrayListValueId) {
        self.values.bit_array_lists.push(value);
    }

    pub(super) fn bit_array_list(&self, local: BitArrayListLocalId) -> BitArrayListValueId {
        self.values.bit_array_lists[local.0].clone()
    }

    pub(super) fn push_utf_codepoint_list(&mut self, value: UtfCodepointListValueId) {
        self.values.utf_codepoint_lists.push(value);
    }

    pub(super) fn utf_codepoint_list(
        &self,
        local: UtfCodepointListLocalId,
    ) -> UtfCodepointListValueId {
        self.values.utf_codepoint_lists[local.0].clone()
    }

    pub(super) fn push_custom_list(&mut self, value: CustomListValueId) {
        self.values.custom_lists.push(value);
    }

    pub(super) fn custom_list(&self, local: CustomListLocalId) -> CustomListValueId {
        self.values.custom_lists[local.0].clone()
    }

    pub(super) fn push_float_list(&mut self, value: FloatListValueId) {
        self.values.float_lists.push(value);
    }

    pub(super) fn float_list(&self, local: FloatListLocalId) -> FloatListValueId {
        self.values.float_lists[local.0].clone()
    }

    pub(super) fn push_bool_list(&mut self, value: BoolListValueId) {
        self.values.bool_lists.push(value);
    }

    pub(super) fn bool_list(&self, local: BoolListLocalId) -> BoolListValueId {
        self.values.bool_lists[local.0].clone()
    }

    pub(super) fn push_nil_list(&mut self, value: NilListValueId) {
        self.values.nil_lists.push(value);
    }

    pub(super) fn nil_list(&self, local: NilListLocalId) -> NilListValueId {
        self.values.nil_lists[local.0].clone()
    }

    pub(super) fn push_tuple_list(&mut self, value: TupleListValueId) {
        self.values.tuple_lists.push(value);
    }

    pub(super) fn tuple_list(&self, local: TupleListLocalId) -> TupleListValueId {
        self.values.tuple_lists[local.0].clone()
    }

    pub(super) fn push_parameter_list_list(&mut self, value: ParameterListListValueId) {
        self.values.parameter_list_lists.push(value);
    }

    pub(super) fn parameter_list_list(
        &self,
        local: ParameterListListLocalId,
    ) -> ParameterListListValueId {
        self.values.parameter_list_lists[local.0].clone()
    }

    pub(super) fn push_list_list(&mut self, value: ListListValueId) {
        self.values.list_lists.push(value);
    }

    pub(super) fn list_list(&self, local: ListListLocalId) -> ListListValueId {
        self.values.list_lists[local.0].clone()
    }

    pub(super) fn push_function_list(&mut self, value: FunctionListValueId) {
        self.values.function_lists.push(value);
    }

    pub(super) fn function_list(&self, local: FunctionListLocalId) -> FunctionListValueId {
        self.values.function_lists[local.0].clone()
    }

    pub(super) fn list(&self, local: &ListLocal) -> ListValueId {
        match local {
            ListLocal::Parameter { local, .. } => self.parameter_list(*local).into(),
            ListLocal::ParameterList { local, .. } => self.parameter_list_list(*local).into(),
            ListLocal::Int { local, .. } => self.int_list(*local).into(),
            ListLocal::String { local, .. } => self.string_list(*local).into(),
            ListLocal::BitArray { local, .. } => self.bit_array_list(*local).into(),
            ListLocal::UtfCodepoint { local, .. } => self.utf_codepoint_list(*local).into(),
            ListLocal::Custom { local, .. } => self.custom_list(*local).into(),
            ListLocal::Float { local, .. } => self.float_list(*local).into(),
            ListLocal::Bool { local, .. } => self.bool_list(*local).into(),
            ListLocal::Nil { local, .. } => self.nil_list(*local).into(),
            ListLocal::Tuple { local, .. } => self.tuple_list(*local).into(),
            ListLocal::List { local, .. } => self.list_list(*local).into(),
            ListLocal::Function { local, .. } => self.function_list(*local).into(),
        }
    }

    pub(super) fn stored_list(
        &self,
        local: &crate::plan::execution::graph::StoredListLocal,
    ) -> StoredListValueId {
        use crate::plan::execution::graph::StoredListLocal as L;

        match local {
            L::ParameterList(local) => self.parameter_list_list(*local).into(),
            L::Int(local) => self.int_list(*local).into(),
            L::String(local) => self.string_list(*local).into(),
            L::BitArray(local) => self.bit_array_list(*local).into(),
            L::UtfCodepoint(local) => self.utf_codepoint_list(*local).into(),
            L::Custom(local) => self.custom_list(*local).into(),
            L::Float(local) => self.float_list(*local).into(),
            L::Bool(local) => self.bool_list(*local).into(),
            L::Nil(local) => self.nil_list(*local).into(),
            L::Tuple(local) => self.tuple_list(*local).into(),
            L::List(local) => self.list_list(*local).into(),
            L::Function(local) => self.function_list(*local).into(),
        }
    }

    pub(super) fn push_int_function(&mut self, value: EvaluatedIntFunction) {
        self.values.int_functions.push(value);
    }

    pub(super) fn int_function(&self, local: IntFunctionLocalId) -> EvaluatedIntFunction {
        self.values.int_functions[local.0].clone()
    }

    pub(super) fn push_float_function(&mut self, value: EvaluatedFloatFunction) {
        self.values.float_functions.push(value);
    }

    pub(super) fn float_function(&self, local: FloatFunctionLocalId) -> EvaluatedFloatFunction {
        self.values.float_functions[local.0].clone()
    }

    pub(super) fn push_string_function(&mut self, value: EvaluatedStringFunction) {
        self.values.string_functions.push(value);
    }

    pub(super) fn string_function(&self, local: StringFunctionLocalId) -> EvaluatedStringFunction {
        self.values.string_functions[local.0].clone()
    }

    pub(super) fn push_bit_array_function(&mut self, value: EvaluatedBitArrayFunction) {
        self.values.bit_array_functions.push(value);
    }

    pub(super) fn bit_array_function(
        &self,
        local: BitArrayFunctionLocalId,
    ) -> EvaluatedBitArrayFunction {
        self.values.bit_array_functions[local.0].clone()
    }

    pub(super) fn push_utf_codepoint_function(&mut self, value: EvaluatedUtfCodepointFunction) {
        self.values.utf_codepoint_functions.push(value);
    }

    pub(super) fn utf_codepoint_function(
        &self,
        local: UtfCodepointFunctionLocalId,
    ) -> EvaluatedUtfCodepointFunction {
        self.values.utf_codepoint_functions[local.0].clone()
    }

    pub(super) fn push_custom_function(&mut self, value: EvaluatedCustomFunction) {
        self.values.custom_functions.push(value);
    }

    pub(super) fn custom_function(&self, local: &CustomFunctionLocal) -> EvaluatedCustomFunction {
        self.values.custom_functions[local.id().0].clone()
    }

    pub(super) fn push_bool_function(&mut self, value: EvaluatedBoolFunction) {
        self.values.bool_functions.push(value);
    }

    pub(super) fn bool_function(&self, local: BoolFunctionLocalId) -> EvaluatedBoolFunction {
        self.values.bool_functions[local.0].clone()
    }

    pub(super) fn push_nil_function(&mut self, value: EvaluatedNilFunction) {
        self.values.nil_functions.push(value);
    }

    pub(super) fn nil_function(&self, local: NilFunctionLocalId) -> EvaluatedNilFunction {
        self.values.nil_functions[local.0].clone()
    }

    pub(super) fn push_tuple_function(&mut self, value: EvaluatedTupleFunction) {
        self.values.tuple_functions.push(value);
    }

    pub(super) fn tuple_function(&self, local: TupleFunctionLocalId) -> EvaluatedTupleFunction {
        self.values.tuple_functions[local.0].clone()
    }

    pub(super) fn list_function(&self, local: &ListFunctionLocal) -> EvaluatedListFunction {
        match local {
            ListFunctionLocal::Parameter { local, .. } => {
                self.values.parameter_list_functions[local.0].clone()
            }
            ListFunctionLocal::ParameterList { local, .. } => {
                self.values.parameter_list_list_functions[local.0].clone()
            }
            ListFunctionLocal::Int { local, .. } => self.values.int_list_functions[local.0].clone(),
            ListFunctionLocal::String { local, .. } => {
                self.values.string_list_functions[local.0].clone()
            }
            ListFunctionLocal::BitArray { local, .. } => {
                self.values.bit_array_list_functions[local.0].clone()
            }
            ListFunctionLocal::UtfCodepoint { local, .. } => {
                self.values.utf_codepoint_list_functions[local.0].clone()
            }
            ListFunctionLocal::Custom { local, .. } => {
                self.values.custom_list_functions[local.0].clone()
            }
            ListFunctionLocal::Float { local, .. } => {
                self.values.float_list_functions[local.0].clone()
            }
            ListFunctionLocal::Bool { local, .. } => {
                self.values.bool_list_functions[local.0].clone()
            }
            ListFunctionLocal::Nil { local, .. } => self.values.nil_list_functions[local.0].clone(),
            ListFunctionLocal::Tuple { local, .. } => {
                self.values.tuple_list_functions[local.0].clone()
            }
            ListFunctionLocal::List { local, .. } => {
                self.values.list_list_functions[local.0].clone()
            }
            ListFunctionLocal::Function { local, .. } => {
                self.values.function_list_functions[local.0].clone()
            }
        }
    }

    pub(super) fn push_function_function(&mut self, value: EvaluatedFunctionFunction) {
        self.values.function_functions.push(value);
    }

    pub(super) fn function_function(
        &self,
        local: &FunctionFunctionLocal,
    ) -> EvaluatedFunctionFunction {
        self.values.function_functions[local.id().0].clone()
    }

    pub(super) fn push_generic_function(&mut self, value: EvaluatedGenericFunction) {
        self.values.generic_functions.push(value);
    }

    pub(super) fn generic_function(
        &self,
        local: &GenericFunctionLocal,
    ) -> EvaluatedGenericFunction {
        self.values.generic_functions[local.id().0].clone()
    }

    pub(super) fn push_never_function(&mut self, value: EvaluatedNeverFunction) {
        self.values.never_functions.push(value);
    }

    pub(super) fn never_function(&self, local: &NeverFunctionLocal) -> EvaluatedNeverFunction {
        self.values.never_functions[local.id().0].clone()
    }

    pub(super) fn push_function_value(&mut self, value: EvaluatedFunctionValue) {
        use crate::runtime::EvaluatedFunctionValueKind as F;

        match value.kind() {
            F::Generic(value) => self.push_generic_function(value.clone()),
            F::Never(value) => self.push_never_function(value.clone()),
            F::Int(value) => self.push_int_function(value.clone()),
            F::Float(value) => self.push_float_function(value.clone()),
            F::String(value) => self.push_string_function(value.clone()),
            F::BitArray(value) => self.push_bit_array_function(value.clone()),
            F::UtfCodepoint(value) => self.push_utf_codepoint_function(value.clone()),
            F::Custom(value) => self.push_custom_function(value.clone()),
            F::Bool(value) => self.push_bool_function(value.clone()),
            F::Nil(value) => self.push_nil_function(value.clone()),
            F::Tuple(value) => self.push_tuple_function(value.clone()),
            F::List(value) => self.values.push_list_function(value.clone()),
            F::Function(value) => self.push_function_function(value.clone()),
        }
    }

    pub(super) fn function_value(
        &self,
        local: &crate::plan::execution::graph::FunctionLocal,
    ) -> EvaluatedFunctionValue {
        use crate::plan::execution::graph::FunctionLocal as L;

        match local {
            L::Generic(local) => self.generic_function(local).into(),
            L::Never(local) => self.never_function(local).into(),
            L::Int(local) => self.int_function(*local).into(),
            L::Float(local) => self.float_function(*local).into(),
            L::String(local) => self.string_function(*local).into(),
            L::BitArray(local) => self.bit_array_function(*local).into(),
            L::UtfCodepoint(local) => self.utf_codepoint_function(*local).into(),
            L::Custom(local) => self.custom_function(local).into(),
            L::Bool(local) => self.bool_function(*local).into(),
            L::Nil(local) => self.nil_function(*local).into(),
            L::Tuple(local) => self.tuple_function(*local).into(),
            L::List(local) => self.list_function(local).into(),
            L::Function(local) => self.function_function(local).into(),
        }
    }
}

impl RetainedValues {
    pub(in crate::runtime) fn empty() -> Self {
        Self {
            values: Box::default(),
        }
    }

    pub(in crate::runtime) fn push_evaluated(&mut self, value: EvaluatedValue) {
        match value {
            EvaluatedValue::Int(value) => self.values.ints.push(value),
            EvaluatedValue::Float(value) => self.values.floats.push(value),
            EvaluatedValue::String(value) => self.values.strings.push(value),
            EvaluatedValue::BitArray(value) => self.values.bit_arrays.push(value),
            EvaluatedValue::UtfCodepoint(value) => self.values.utf_codepoints.push(value),
            EvaluatedValue::Custom(value) => self.values.customs.push(value),
            EvaluatedValue::Bool(value) => self.values.bools.push(value),
            EvaluatedValue::Nil => {}
            EvaluatedValue::Tuple(value) => self.values.tuples.push(value),
            EvaluatedValue::List(value) => self.push_list(value),
            EvaluatedValue::Function(value) => self.push_function(value),
        }
    }

    pub(in crate::runtime) fn append_captures(&mut self, captures: &[EvaluatedCapture]) {
        for capture in captures {
            match capture.kind() {
                EvaluatedCaptureKind::Int { value, .. } => self.values.ints.push(value.clone()),
                EvaluatedCaptureKind::Float { value, .. } => self.values.floats.push(*value),
                EvaluatedCaptureKind::String { value, .. } => {
                    self.values.strings.push(value.clone())
                }
                EvaluatedCaptureKind::BitArray { value, .. } => {
                    self.values.bit_arrays.push(value.clone())
                }
                EvaluatedCaptureKind::UtfCodepoint { value, .. } => {
                    self.values.utf_codepoints.push(*value)
                }
                EvaluatedCaptureKind::Custom { value, .. } => {
                    self.values.customs.push(value.clone())
                }
                EvaluatedCaptureKind::Bool { value, .. } => self.values.bools.push(*value),
                EvaluatedCaptureKind::Nil { .. } => {}
                EvaluatedCaptureKind::Tuple { value, .. } => self.values.tuples.push(value.clone()),
                EvaluatedCaptureKind::List(value) => self.push_list_capture(value),
                EvaluatedCaptureKind::IntFunction { value, .. } => {
                    self.values.int_functions.push(value.clone())
                }
                EvaluatedCaptureKind::FloatFunction { value, .. } => {
                    self.values.float_functions.push(value.clone())
                }
                EvaluatedCaptureKind::StringFunction { value, .. } => {
                    self.values.string_functions.push(value.clone())
                }
                EvaluatedCaptureKind::BitArrayFunction { value, .. } => {
                    self.values.bit_array_functions.push(value.clone())
                }
                EvaluatedCaptureKind::UtfCodepointFunction { value, .. } => {
                    self.values.utf_codepoint_functions.push(value.clone())
                }
                EvaluatedCaptureKind::CustomFunction { value, .. } => {
                    self.values.custom_functions.push(value.clone())
                }
                EvaluatedCaptureKind::BoolFunction { value, .. } => {
                    self.values.bool_functions.push(value.clone())
                }
                EvaluatedCaptureKind::NilFunction { value, .. } => {
                    self.values.nil_functions.push(value.clone())
                }
                EvaluatedCaptureKind::TupleFunction { value, .. } => {
                    self.values.tuple_functions.push(value.clone())
                }
                EvaluatedCaptureKind::ListFunction { value, .. } => {
                    self.values.push_list_function(value.clone())
                }
                EvaluatedCaptureKind::FunctionFunction { value, .. } => {
                    self.values.function_functions.push(value.clone())
                }
                EvaluatedCaptureKind::GenericFunction { value, .. } => {
                    self.values.generic_functions.push(value.clone())
                }
                EvaluatedCaptureKind::NeverFunction { value, .. } => {
                    self.values.never_functions.push(value.clone())
                }
            }
        }
    }

    fn push_local(&mut self, environment: &BlockEnvironment, local: &ParamLocal) {
        self.push_evaluated(environment.value(local));
    }

    fn push_list(&mut self, value: ListValueId) {
        match value {
            ListValueId::Parameter(value) => self.values.parameter_lists.push(value),
            ListValueId::Int(value) => self.values.int_lists.push(value),
            ListValueId::String(value) => self.values.string_lists.push(value),
            ListValueId::BitArray(value) => self.values.bit_array_lists.push(value),
            ListValueId::UtfCodepoint(value) => self.values.utf_codepoint_lists.push(value),
            ListValueId::Custom(value) => self.values.custom_lists.push(value),
            ListValueId::Float(value) => self.values.float_lists.push(value),
            ListValueId::Bool(value) => self.values.bool_lists.push(value),
            ListValueId::Nil(value) => self.values.nil_lists.push(value),
            ListValueId::Tuple(value) => self.values.tuple_lists.push(value),
            ListValueId::ParameterList(value) => self.values.parameter_list_lists.push(value),
            ListValueId::List(value) => self.values.list_lists.push(value),
            ListValueId::Function(value) => self.values.function_lists.push(value),
        }
    }

    fn push_function(&mut self, value: EvaluatedFunctionValue) {
        use crate::runtime::EvaluatedFunctionValueKind as F;

        match value.kind() {
            F::Generic(value) => self.values.generic_functions.push(value.clone()),
            F::Never(value) => self.values.never_functions.push(value.clone()),
            F::Int(value) => self.values.int_functions.push(value.clone()),
            F::Float(value) => self.values.float_functions.push(value.clone()),
            F::String(value) => self.values.string_functions.push(value.clone()),
            F::BitArray(value) => self.values.bit_array_functions.push(value.clone()),
            F::UtfCodepoint(value) => self.values.utf_codepoint_functions.push(value.clone()),
            F::Custom(value) => self.values.custom_functions.push(value.clone()),
            F::Bool(value) => self.values.bool_functions.push(value.clone()),
            F::Nil(value) => self.values.nil_functions.push(value.clone()),
            F::Tuple(value) => self.values.tuple_functions.push(value.clone()),
            F::List(value) => self.values.push_list_function(value.clone()),
            F::Function(value) => self.values.function_functions.push(value.clone()),
        }
    }

    fn push_list_capture(&mut self, value: &EvaluatedListCapture) {
        match value {
            EvaluatedListCapture::Parameter { value, .. } => {
                self.values.parameter_lists.push(*value)
            }
            EvaluatedListCapture::ParameterList { value, .. } => {
                self.values.parameter_list_lists.push(value.clone())
            }
            EvaluatedListCapture::Int { value, .. } => self.values.int_lists.push(value.clone()),
            EvaluatedListCapture::String { value, .. } => {
                self.values.string_lists.push(value.clone())
            }
            EvaluatedListCapture::BitArray { value, .. } => {
                self.values.bit_array_lists.push(value.clone())
            }
            EvaluatedListCapture::UtfCodepoint { value, .. } => {
                self.values.utf_codepoint_lists.push(value.clone())
            }
            EvaluatedListCapture::Custom { value, .. } => {
                self.values.custom_lists.push(value.clone())
            }
            EvaluatedListCapture::Float { value, .. } => {
                self.values.float_lists.push(value.clone())
            }
            EvaluatedListCapture::Bool { value, .. } => self.values.bool_lists.push(value.clone()),
            EvaluatedListCapture::Nil { value, .. } => self.values.nil_lists.push(value.clone()),
            EvaluatedListCapture::Tuple { value, .. } => {
                self.values.tuple_lists.push(value.clone())
            }
            EvaluatedListCapture::List { value, .. } => self.values.list_lists.push(value.clone()),
            EvaluatedListCapture::Function { value, .. } => {
                self.values.function_lists.push(value.clone())
            }
        }
    }
}

impl HostCallArguments for RetainedValues {
    fn int(&self, slot: HostIntArgumentSlot) -> BigInt {
        self.values.ints[slot.index()].clone()
    }

    fn float(&self, slot: HostFloatArgumentSlot) -> f64 {
        self.values.floats[slot.index()]
    }

    fn string(&self, slot: HostStringArgumentSlot) -> EcoString {
        self.values.strings[slot.index()].clone()
    }

    fn bit_array(&self, slot: HostBitArrayArgumentSlot) -> crate::BitArrayValue {
        self.values.bit_arrays[slot.index()].value()
    }

    fn utf_codepoint(&self, slot: HostUtfCodepointArgumentSlot) -> char {
        self.values.utf_codepoints[slot.index()]
    }

    fn bool(&self, slot: HostBoolArgumentSlot) -> bool {
        self.values.bools[slot.index()]
    }

    fn nil(&self, _slot: HostNilArgumentSlot) {}
}

impl BlockValues {
    fn push_list_function(&mut self, value: EvaluatedListFunction) {
        use crate::plan::execution::function::ListFunctionId as F;

        match value.runtime_id() {
            F::Parameter(_) => self.parameter_list_functions.push(value),
            F::ParameterList(_) => self.parameter_list_list_functions.push(value),
            F::Int(_) => self.int_list_functions.push(value),
            F::String(_) => self.string_list_functions.push(value),
            F::BitArray(_) => self.bit_array_list_functions.push(value),
            F::UtfCodepoint(_) => self.utf_codepoint_list_functions.push(value),
            F::Custom(_) => self.custom_list_functions.push(value),
            F::Float(_) => self.float_list_functions.push(value),
            F::Bool(_) => self.bool_list_functions.push(value),
            F::Nil(_) => self.nil_list_functions.push(value),
            F::Tuple(_) => self.tuple_list_functions.push(value),
            F::List(_) => self.list_list_functions.push(value),
            F::Function(_) => self.function_list_functions.push(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BlockEnvironment, RetainedValues};
    use crate::host::{HostFunctionDefinition, HostFunctionImplementation, HostIntFunction};
    use crate::plan::execution::graph::{IntLocalId, ParamLocal};
    use crate::runtime::{EvaluatedValue, ListValue, Value};
    use num_bigint::BigInt;

    #[test]
    fn edge_retention_preserves_argument_order_and_omits_unselected_values() {
        let mut inputs = RetainedValues::empty();
        inputs.push_evaluated(EvaluatedValue::Int(10.into()));
        inputs.push_evaluated(EvaluatedValue::String("discarded".into()));
        inputs.push_evaluated(EvaluatedValue::Int(20.into()));
        let environment = BlockEnvironment::from_retained(inputs);

        let retained = environment.retain(&[
            ParamLocal::Int(IntLocalId(1)),
            ParamLocal::Int(IntLocalId(0)),
        ]);
        assert_eq!(retained.values.ints, vec![20.into(), 10.into()]);
        assert!(retained.values.strings.is_empty());

        let environment = BlockEnvironment::from_retained(retained);
        assert_eq!(environment.int(IntLocalId(0)), 20.into());
        assert_eq!(environment.int(IntLocalId(1)), 10.into());
    }

    #[test]
    fn closure_environment_preserves_utf_codepoint_list_captures() {
        let source = r#"
fn codepoint() -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}

pub fn main() {
  let values = [codepoint()]
  let captured = fn() { values }
  captured()
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module = crate::plan_module(typed).expect("source should plan");

        assert_eq!(
            crate::run_main(
                &crate::ExecutionPlan::from_module_plan(module),
                &mut Vec::new(),
            ),
            Ok(Value::List(ListValue::utf_codepoint(vec!['A']))),
        );
    }

    #[test]
    fn closure_environment_preserves_remaining_list_and_never_capture_families() {
        let source = r#"
pub type Marker {
  Marker(Int)
}

fn identity(value: Int) -> Int {
  value
}

fn diverge(_value: Int) -> value {
  panic
}

pub fn main() {
  let parameter = []
  let parameter_lists = [[]]
  let ints = [1]
  let strings = ["one"]
  let bit_arrays = [<<1>>]
  let customs = [Marker(1)]
  let floats = [1.0]
  let bools = [True]
  let nils = [Nil]
  let tuples = [#(1)]
  let lists = [[1]]
  let functions = [identity]
  let never = diverge
  let captured = fn() {
    #(
      parameter == [],
      parameter_lists == [[]],
      ints == [1],
      strings == ["one"],
      bit_arrays == [<<1>>],
      customs == [Marker(1)],
      floats == [1.0],
      bools == [True],
      nils == [Nil],
      tuples == [#(1)],
      lists == [[1]],
      functions == [identity],
      never == diverge,
    )
  }
  captured()
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            Value::Tuple(vec![Value::Bool(true); 13]),
        );
    }

    #[test]
    fn retained_values_supply_family_local_host_arguments() {
        let definition = HostFunctionDefinition::new(
            "choose".into(),
            |condition: bool, left: BigInt, right: BigInt| {
                if condition { left } else { right }
            },
        );
        let (_, implementation) = definition.into_parts();
        let mut arguments = RetainedValues::empty();
        arguments.push_evaluated(EvaluatedValue::Int(10.into()));
        arguments.push_evaluated(EvaluatedValue::Bool(false));
        arguments.push_evaluated(EvaluatedValue::Int(20.into()));

        let implementation = int_implementation(implementation);
        assert_eq!(
            implementation.call(&mut (), &arguments),
            Ok(BigInt::from(20))
        );

        let mut arguments = RetainedValues::empty();
        arguments.push_evaluated(EvaluatedValue::Int(10.into()));
        arguments.push_evaluated(EvaluatedValue::Bool(true));
        arguments.push_evaluated(EvaluatedValue::Int(20.into()));

        assert_eq!(
            implementation.call(&mut (), &arguments),
            Ok(BigInt::from(10))
        );
    }

    #[test]
    #[should_panic(expected = "choose should retain an Int implementation")]
    fn retained_host_argument_shape_guard_is_visible() {
        let definition = HostFunctionDefinition::new("choose".into(), || true);
        let (_, implementation) = definition.into_parts();
        int_implementation(implementation);
    }

    fn int_implementation(
        implementation: HostFunctionImplementation<crate::host::StatelessHostProfile>,
    ) -> HostIntFunction<crate::host::StatelessHostProfile> {
        let HostFunctionImplementation::Int(implementation) = implementation else {
            panic!("choose should retain an Int implementation");
        };
        implementation
    }
}
