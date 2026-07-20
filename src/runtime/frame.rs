use crate::plan::execution::{
    BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId, BoolFunctionLocalId,
    BoolListLocalId, BoolLocalId, CustomFunctionLocal, CustomFunctionLocalId, CustomListLocalId,
    CustomLocal, CustomLocalId, FloatFunctionLocalId, FloatListLocalId, FloatLocalId, FrameLayout,
    FunctionFunctionLocal, FunctionFunctionLocalId, FunctionListLocalId, GenericFunctionLocal,
    GenericFunctionLocalId, IntFunctionLocalId, IntListLocalId, IntLocalId, ListFunctionLocal,
    ListListLocalId, NeverFunctionLocal, NeverFunctionLocalId, NilFunctionLocalId, NilListLocalId,
    NilLocalId, ParameterListListLocalId, ParameterListLocalId, StringFunctionLocalId,
    StringListLocalId, StringLocalId, TupleFunctionLocalId, TupleListLocalId, TupleLocalId,
    UtfCodepointFunctionLocalId, UtfCodepointListLocalId, UtfCodepointLocalId,
};
use crate::runtime::evaluated::{
    EvaluatedBitArray, EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCustomFunction,
    EvaluatedCustomValue, EvaluatedFloatFunction, EvaluatedFunctionFunction,
    EvaluatedGenericFunction, EvaluatedIntFunction, EvaluatedListFunction, EvaluatedNeverFunction,
    EvaluatedNilFunction, EvaluatedStringFunction, EvaluatedTupleFunction,
    EvaluatedUtfCodepointFunction, EvaluatedValue,
};
use crate::runtime::state::{
    BitArrayListValueId, BoolListValueId, CustomListValueId, FloatListValueId, FunctionListValueId,
    IntListValueId, ListListValueId, NilListValueId, ParameterListListValueId,
    ParameterListValueId, RuntimeState, StringListValueId, TupleListValueId,
    UtfCodepointListValueId,
};
use ecow::EcoString;
use num_bigint::BigInt;
use std::collections::HashMap;

pub(super) struct Frame {
    ints: Vec<BigInt>,
    floats: Vec<f64>,
    strings: Vec<EcoString>,
    bit_arrays: Vec<EvaluatedBitArray>,
    utf_codepoints: Vec<char>,
    customs: HashMap<CustomLocalId, EvaluatedCustomValue>,
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
    int_functions: HashMap<IntFunctionLocalId, EvaluatedIntFunction>,
    float_functions: HashMap<FloatFunctionLocalId, EvaluatedFloatFunction>,
    string_functions: HashMap<StringFunctionLocalId, EvaluatedStringFunction>,
    bit_array_functions: HashMap<BitArrayFunctionLocalId, EvaluatedBitArrayFunction>,
    utf_codepoint_functions: HashMap<UtfCodepointFunctionLocalId, EvaluatedUtfCodepointFunction>,
    custom_functions: HashMap<CustomFunctionLocalId, EvaluatedCustomFunction>,
    bool_functions: HashMap<BoolFunctionLocalId, EvaluatedBoolFunction>,
    nil_functions: HashMap<NilFunctionLocalId, EvaluatedNilFunction>,
    tuple_functions: HashMap<TupleFunctionLocalId, EvaluatedTupleFunction>,
    list_functions: HashMap<ListFunctionLocal, EvaluatedListFunction>,
    function_functions: HashMap<FunctionFunctionLocalId, EvaluatedFunctionFunction>,
    generic_functions: HashMap<GenericFunctionLocalId, EvaluatedGenericFunction>,
    never_functions: HashMap<NeverFunctionLocalId, EvaluatedNeverFunction>,
}

impl Frame {
    pub(super) fn new(layout: &FrameLayout, state: &mut RuntimeState) -> Self {
        Self {
            ints: vec![BigInt::from(0); layout.ints()],
            floats: vec![0.0; layout.floats()],
            strings: vec![EcoString::default(); layout.strings()],
            bit_arrays: vec![EvaluatedBitArray::new(Default::default()); layout.bit_arrays()],
            utf_codepoints: vec!['\0'; layout.utf_codepoints()],
            customs: HashMap::with_capacity(layout.customs().len()),
            bools: vec![false; layout.bools()],
            tuples: vec![Vec::new(); layout.tuples()],
            parameter_lists: layout
                .parameter_lists()
                .iter()
                .map(|type_id| ParameterListValueId::new(*type_id))
                .collect(),
            int_lists: layout
                .int_lists()
                .iter()
                .map(|type_id| state.int(*type_id, Vec::new()))
                .collect(),
            string_lists: layout
                .string_lists()
                .iter()
                .map(|type_id| state.string(*type_id, Vec::new()))
                .collect(),
            bit_array_lists: layout
                .bit_array_lists()
                .iter()
                .map(|type_id| state.bit_array(*type_id, Vec::new()))
                .collect(),
            utf_codepoint_lists: layout
                .utf_codepoint_lists()
                .iter()
                .map(|type_id| state.utf_codepoint(*type_id, Vec::new()))
                .collect(),
            custom_lists: layout
                .custom_lists()
                .iter()
                .map(|type_id| state.empty_custom(*type_id))
                .collect(),
            float_lists: layout
                .float_lists()
                .iter()
                .map(|type_id| state.float(*type_id, Vec::new()))
                .collect(),
            bool_lists: layout
                .bool_lists()
                .iter()
                .map(|type_id| state.bool(*type_id, Vec::new()))
                .collect(),
            nil_lists: layout
                .nil_lists()
                .iter()
                .map(|type_id| state.nil(*type_id, 0))
                .collect(),
            tuple_lists: layout
                .tuple_lists()
                .iter()
                .map(|type_id| state.tuple(*type_id, Vec::new()))
                .collect(),
            parameter_list_lists: layout
                .parameter_list_lists()
                .iter()
                .map(|type_id| state.parameter_list_list(*type_id, 0))
                .collect(),
            list_lists: layout
                .list_lists()
                .iter()
                .map(|type_id| state.list(*type_id, Vec::new()))
                .collect(),
            function_lists: layout
                .function_lists()
                .iter()
                .map(|type_id| state.function(*type_id, Vec::new()))
                .collect(),
            int_functions: HashMap::with_capacity(layout.int_functions()),
            float_functions: HashMap::with_capacity(layout.float_functions()),
            string_functions: HashMap::with_capacity(layout.string_functions()),
            bit_array_functions: HashMap::with_capacity(layout.bit_array_functions()),
            utf_codepoint_functions: HashMap::with_capacity(layout.utf_codepoint_functions()),
            custom_functions: HashMap::with_capacity(layout.custom_functions().len()),
            bool_functions: HashMap::with_capacity(layout.bool_functions()),
            nil_functions: HashMap::with_capacity(layout.nil_functions()),
            tuple_functions: HashMap::with_capacity(layout.tuple_functions()),
            list_functions: HashMap::with_capacity(layout.list_functions().len()),
            function_functions: HashMap::with_capacity(layout.function_functions().len()),
            generic_functions: HashMap::with_capacity(layout.generic_functions().len()),
            never_functions: HashMap::with_capacity(layout.never_functions().len()),
        }
    }

    pub(super) fn set_int(&mut self, local: IntLocalId, value: BigInt) {
        set_slot(&mut self.ints, local.0, value);
    }

    pub(super) fn get_int(&self, local: IntLocalId) -> BigInt {
        self.ints[local.0].clone()
    }

    pub(super) fn set_float(&mut self, local: FloatLocalId, value: f64) {
        set_slot(&mut self.floats, local.0, value);
    }

    pub(super) fn get_float(&self, local: FloatLocalId) -> f64 {
        self.floats[local.0]
    }

    pub(super) fn set_string(&mut self, local: StringLocalId, value: EcoString) {
        set_slot(&mut self.strings, local.0, value);
    }

    pub(super) fn get_string(&self, local: StringLocalId) -> EcoString {
        self.strings[local.0].clone()
    }

    pub(super) fn set_bit_array(&mut self, local: BitArrayLocalId, value: EvaluatedBitArray) {
        set_slot(&mut self.bit_arrays, local.0, value);
    }

    pub(super) fn get_bit_array(&self, local: BitArrayLocalId) -> EvaluatedBitArray {
        self.bit_arrays[local.0].clone()
    }

    pub(super) fn set_utf_codepoint(&mut self, local: UtfCodepointLocalId, value: char) {
        set_slot(&mut self.utf_codepoints, local.0, value);
    }

    pub(super) fn get_utf_codepoint(&self, local: UtfCodepointLocalId) -> char {
        self.utf_codepoints[local.0]
    }

    pub(super) fn set_custom(&mut self, local: CustomLocal, value: EvaluatedCustomValue) {
        self.customs.insert(local.id(), value);
    }

    pub(super) fn get_custom(&self, local: CustomLocal) -> EvaluatedCustomValue {
        self.customs[&local.id()].clone()
    }

    pub(super) fn set_bool(&mut self, local: BoolLocalId, value: bool) {
        set_slot(&mut self.bools, local.0, value);
    }

    pub(super) fn get_bool(&self, local: BoolLocalId) -> bool {
        self.bools[local.0]
    }

    pub(super) fn set_nil(&mut self, _local: NilLocalId) {}

    pub(super) fn get_nil(&self, _local: NilLocalId) {}

    pub(super) fn set_tuple(&mut self, local: TupleLocalId, value: Vec<EvaluatedValue>) {
        set_slot(&mut self.tuples, local.0, value);
    }

    pub(super) fn get_tuple(&self, local: TupleLocalId) -> Vec<EvaluatedValue> {
        self.tuples[local.0].clone()
    }

    pub(super) fn set_parameter_list(
        &mut self,
        local: ParameterListLocalId,
        value: ParameterListValueId,
    ) {
        set_slot(&mut self.parameter_lists, local.0, value);
    }

    pub(super) fn get_parameter_list(&self, local: ParameterListLocalId) -> ParameterListValueId {
        self.parameter_lists[local.0]
    }

    pub(super) fn set_int_list(&mut self, local: IntListLocalId, value: IntListValueId) {
        set_slot(&mut self.int_lists, local.0, value);
    }

    pub(super) fn get_int_list(&self, local: IntListLocalId) -> IntListValueId {
        self.int_lists[local.0].clone()
    }

    pub(super) fn set_string_list(&mut self, local: StringListLocalId, value: StringListValueId) {
        set_slot(&mut self.string_lists, local.0, value);
    }

    pub(super) fn get_string_list(&self, local: StringListLocalId) -> StringListValueId {
        self.string_lists[local.0].clone()
    }

    pub(super) fn set_bit_array_list(
        &mut self,
        local: BitArrayListLocalId,
        value: BitArrayListValueId,
    ) {
        set_slot(&mut self.bit_array_lists, local.0, value);
    }

    pub(super) fn get_bit_array_list(&self, local: BitArrayListLocalId) -> BitArrayListValueId {
        self.bit_array_lists[local.0].clone()
    }

    pub(super) fn set_utf_codepoint_list(
        &mut self,
        local: UtfCodepointListLocalId,
        value: UtfCodepointListValueId,
    ) {
        set_slot(&mut self.utf_codepoint_lists, local.0, value);
    }

    pub(super) fn get_utf_codepoint_list(
        &self,
        local: UtfCodepointListLocalId,
    ) -> UtfCodepointListValueId {
        self.utf_codepoint_lists[local.0].clone()
    }

    pub(super) fn set_custom_list(&mut self, local: CustomListLocalId, value: CustomListValueId) {
        set_slot(&mut self.custom_lists, local.0, value);
    }

    pub(super) fn get_custom_list(&self, local: CustomListLocalId) -> CustomListValueId {
        self.custom_lists[local.0].clone()
    }

    pub(super) fn set_float_list(&mut self, local: FloatListLocalId, value: FloatListValueId) {
        set_slot(&mut self.float_lists, local.0, value);
    }

    pub(super) fn get_float_list(&self, local: FloatListLocalId) -> FloatListValueId {
        self.float_lists[local.0].clone()
    }

    pub(super) fn set_bool_list(&mut self, local: BoolListLocalId, value: BoolListValueId) {
        set_slot(&mut self.bool_lists, local.0, value);
    }

    pub(super) fn get_bool_list(&self, local: BoolListLocalId) -> BoolListValueId {
        self.bool_lists[local.0].clone()
    }

    pub(super) fn set_nil_list(&mut self, local: NilListLocalId, value: NilListValueId) {
        set_slot(&mut self.nil_lists, local.0, value);
    }

    pub(super) fn get_nil_list(&self, local: NilListLocalId) -> NilListValueId {
        self.nil_lists[local.0].clone()
    }

    pub(super) fn set_tuple_list(&mut self, local: TupleListLocalId, value: TupleListValueId) {
        set_slot(&mut self.tuple_lists, local.0, value);
    }

    pub(super) fn get_tuple_list(&self, local: TupleListLocalId) -> TupleListValueId {
        self.tuple_lists[local.0].clone()
    }

    pub(super) fn set_parameter_list_list(
        &mut self,
        local: ParameterListListLocalId,
        value: ParameterListListValueId,
    ) {
        set_slot(&mut self.parameter_list_lists, local.0, value);
    }

    pub(super) fn get_parameter_list_list(
        &self,
        local: ParameterListListLocalId,
    ) -> ParameterListListValueId {
        self.parameter_list_lists[local.0].clone()
    }

    pub(super) fn set_list_list(&mut self, local: ListListLocalId, value: ListListValueId) {
        set_slot(&mut self.list_lists, local.0, value);
    }

    pub(super) fn get_list_list(&self, local: ListListLocalId) -> ListListValueId {
        self.list_lists[local.0].clone()
    }

    pub(super) fn set_function_list(
        &mut self,
        local: FunctionListLocalId,
        value: FunctionListValueId,
    ) {
        set_slot(&mut self.function_lists, local.0, value);
    }

    pub(super) fn get_function_list(&self, local: FunctionListLocalId) -> FunctionListValueId {
        self.function_lists[local.0].clone()
    }

    pub(super) fn set_int_function(
        &mut self,
        local: IntFunctionLocalId,
        value: EvaluatedIntFunction,
    ) {
        self.int_functions.insert(local, value);
    }

    pub(super) fn get_int_function(&self, local: IntFunctionLocalId) -> EvaluatedIntFunction {
        self.int_functions[&local].clone()
    }

    pub(super) fn set_float_function(
        &mut self,
        local: FloatFunctionLocalId,
        value: EvaluatedFloatFunction,
    ) {
        self.float_functions.insert(local, value);
    }

    pub(super) fn get_float_function(&self, local: FloatFunctionLocalId) -> EvaluatedFloatFunction {
        self.float_functions[&local].clone()
    }

    pub(super) fn set_string_function(
        &mut self,
        local: StringFunctionLocalId,
        value: EvaluatedStringFunction,
    ) {
        self.string_functions.insert(local, value);
    }

    pub(super) fn get_string_function(
        &self,
        local: StringFunctionLocalId,
    ) -> EvaluatedStringFunction {
        self.string_functions[&local].clone()
    }

    pub(super) fn set_bit_array_function(
        &mut self,
        local: BitArrayFunctionLocalId,
        value: EvaluatedBitArrayFunction,
    ) {
        self.bit_array_functions.insert(local, value);
    }

    pub(super) fn get_bit_array_function(
        &self,
        local: BitArrayFunctionLocalId,
    ) -> EvaluatedBitArrayFunction {
        self.bit_array_functions[&local].clone()
    }

    pub(super) fn set_utf_codepoint_function(
        &mut self,
        local: UtfCodepointFunctionLocalId,
        value: EvaluatedUtfCodepointFunction,
    ) {
        self.utf_codepoint_functions.insert(local, value);
    }

    pub(super) fn get_utf_codepoint_function(
        &self,
        local: UtfCodepointFunctionLocalId,
    ) -> EvaluatedUtfCodepointFunction {
        self.utf_codepoint_functions[&local].clone()
    }

    pub(super) fn set_custom_function(
        &mut self,
        local: &CustomFunctionLocal,
        value: EvaluatedCustomFunction,
    ) {
        self.custom_functions.insert(local.id(), value);
    }

    pub(super) fn get_custom_function(
        &self,
        local: &CustomFunctionLocal,
    ) -> EvaluatedCustomFunction {
        self.custom_functions[&local.id()].clone()
    }

    pub(super) fn set_bool_function(
        &mut self,
        local: BoolFunctionLocalId,
        value: EvaluatedBoolFunction,
    ) {
        self.bool_functions.insert(local, value);
    }

    pub(super) fn get_bool_function(&self, local: BoolFunctionLocalId) -> EvaluatedBoolFunction {
        self.bool_functions[&local].clone()
    }

    pub(super) fn set_nil_function(
        &mut self,
        local: NilFunctionLocalId,
        value: EvaluatedNilFunction,
    ) {
        self.nil_functions.insert(local, value);
    }

    pub(super) fn get_nil_function(&self, local: NilFunctionLocalId) -> EvaluatedNilFunction {
        self.nil_functions[&local].clone()
    }

    pub(super) fn set_tuple_function(
        &mut self,
        local: TupleFunctionLocalId,
        value: EvaluatedTupleFunction,
    ) {
        self.tuple_functions.insert(local, value);
    }

    pub(super) fn get_tuple_function(&self, local: TupleFunctionLocalId) -> EvaluatedTupleFunction {
        self.tuple_functions[&local].clone()
    }

    pub(super) fn set_list_function(
        &mut self,
        local: ListFunctionLocal,
        value: EvaluatedListFunction,
    ) {
        self.list_functions.insert(local, value);
    }

    pub(super) fn get_list_function(&self, local: &ListFunctionLocal) -> EvaluatedListFunction {
        self.list_functions[local].clone()
    }

    pub(super) fn set_function_function(
        &mut self,
        local: &FunctionFunctionLocal,
        value: EvaluatedFunctionFunction,
    ) {
        self.function_functions.insert(local.id(), value);
    }

    pub(super) fn get_function_function(
        &self,
        local: &FunctionFunctionLocal,
    ) -> EvaluatedFunctionFunction {
        self.function_functions[&local.id()].clone()
    }

    pub(super) fn set_generic_function(
        &mut self,
        local: &GenericFunctionLocal,
        value: EvaluatedGenericFunction,
    ) {
        self.generic_functions.insert(local.id(), value);
    }

    pub(super) fn get_generic_function(
        &self,
        local: &GenericFunctionLocal,
    ) -> EvaluatedGenericFunction {
        self.generic_functions[&local.id()].clone()
    }

    pub(super) fn set_never_function(
        &mut self,
        local: &NeverFunctionLocal,
        value: EvaluatedNeverFunction,
    ) {
        self.never_functions.insert(local.id(), value);
    }

    pub(super) fn get_never_function(&self, local: &NeverFunctionLocal) -> EvaluatedNeverFunction {
        self.never_functions[&local.id()].clone()
    }
}

fn set_slot<T>(slots: &mut [T], index: usize, value: T) {
    slots[index] = value;
}
