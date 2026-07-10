use crate::plan::execution::{
    BoolFunctionLocalId, BoolListLocalId, BoolLocalId, FloatFunctionLocalId, FloatListLocalId,
    FloatLocalId, FrameLayout, FunctionFunctionLocalId, FunctionListLocalId, IntFunctionLocalId,
    IntListLocalId, IntLocalId, ListFunctionLocal, ListListLocalId, NilFunctionLocalId,
    NilListLocalId, NilLocalId, StringFunctionLocalId, StringListLocalId, StringLocalId,
    TupleFunctionLocalId, TupleListLocalId, TupleLocalId,
};
use crate::runtime::{
    BoolFunctionValue, FloatFunctionValue, FunctionFunctionValue, FunctionValue, IntFunctionValue,
    ListFunctionValue, ListValue, NilFunctionValue, StringFunctionValue, TupleFunctionValue, Value,
};
use ecow::EcoString;
use num_bigint::BigInt;
use std::collections::HashMap;

pub(super) struct Frame {
    ints: Vec<BigInt>,
    floats: Vec<f64>,
    strings: Vec<EcoString>,
    bools: Vec<bool>,
    tuples: Vec<Vec<Value>>,
    int_lists: Vec<Vec<BigInt>>,
    string_lists: Vec<Vec<EcoString>>,
    float_lists: Vec<Vec<f64>>,
    bool_lists: Vec<Vec<bool>>,
    nil_lists: Vec<usize>,
    tuple_lists: Vec<Vec<Vec<Value>>>,
    list_lists: Vec<Vec<ListValue>>,
    function_lists: Vec<Vec<FunctionValue>>,
    int_functions: HashMap<IntFunctionLocalId, IntFunctionValue>,
    float_functions: HashMap<FloatFunctionLocalId, FloatFunctionValue>,
    string_functions: HashMap<StringFunctionLocalId, StringFunctionValue>,
    bool_functions: HashMap<BoolFunctionLocalId, BoolFunctionValue>,
    nil_functions: HashMap<NilFunctionLocalId, NilFunctionValue>,
    tuple_functions: HashMap<TupleFunctionLocalId, TupleFunctionValue>,
    list_functions: HashMap<ListFunctionLocal, ListFunctionValue>,
    function_functions: HashMap<FunctionFunctionLocalId, FunctionFunctionValue>,
}

impl Frame {
    pub(super) fn new(layout: &FrameLayout) -> Self {
        Self {
            ints: vec![BigInt::from(0); layout.ints()],
            floats: vec![0.0; layout.floats()],
            strings: vec![EcoString::default(); layout.strings()],
            bools: vec![false; layout.bools()],
            tuples: vec![Vec::new(); layout.tuples()],
            int_lists: vec![Vec::new(); layout.int_lists()],
            string_lists: vec![Vec::new(); layout.string_lists()],
            float_lists: vec![Vec::new(); layout.float_lists()],
            bool_lists: vec![Vec::new(); layout.bool_lists()],
            nil_lists: vec![0; layout.nil_lists()],
            tuple_lists: vec![Vec::new(); layout.tuple_lists().len()],
            list_lists: vec![Vec::new(); layout.list_lists().len()],
            function_lists: vec![Vec::new(); layout.function_lists().len()],
            int_functions: HashMap::with_capacity(layout.int_functions()),
            float_functions: HashMap::with_capacity(layout.float_functions()),
            string_functions: HashMap::with_capacity(layout.string_functions()),
            bool_functions: HashMap::with_capacity(layout.bool_functions()),
            nil_functions: HashMap::with_capacity(layout.nil_functions()),
            tuple_functions: HashMap::with_capacity(layout.tuple_functions()),
            list_functions: HashMap::with_capacity(layout.list_functions().len()),
            function_functions: HashMap::with_capacity(layout.function_functions()),
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

    pub(super) fn set_bool(&mut self, local: BoolLocalId, value: bool) {
        set_slot(&mut self.bools, local.0, value);
    }

    pub(super) fn get_bool(&self, local: BoolLocalId) -> bool {
        self.bools[local.0]
    }

    pub(super) fn set_nil(&mut self, _local: NilLocalId) {}

    pub(super) fn get_nil(&self, _local: NilLocalId) {}

    pub(super) fn set_tuple(&mut self, local: TupleLocalId, value: Vec<Value>) {
        set_slot(&mut self.tuples, local.0, value);
    }

    pub(super) fn get_tuple(&self, local: TupleLocalId) -> Vec<Value> {
        self.tuples[local.0].clone()
    }

    pub(super) fn set_int_list(&mut self, local: IntListLocalId, value: Vec<BigInt>) {
        set_slot(&mut self.int_lists, local.0, value);
    }

    pub(super) fn get_int_list(&self, local: IntListLocalId) -> Vec<BigInt> {
        self.int_lists[local.0].clone()
    }

    pub(super) fn set_string_list(&mut self, local: StringListLocalId, value: Vec<EcoString>) {
        set_slot(&mut self.string_lists, local.0, value);
    }

    pub(super) fn get_string_list(&self, local: StringListLocalId) -> Vec<EcoString> {
        self.string_lists[local.0].clone()
    }

    pub(super) fn set_float_list(&mut self, local: FloatListLocalId, value: Vec<f64>) {
        set_slot(&mut self.float_lists, local.0, value);
    }

    pub(super) fn get_float_list(&self, local: FloatListLocalId) -> Vec<f64> {
        self.float_lists[local.0].clone()
    }

    pub(super) fn set_bool_list(&mut self, local: BoolListLocalId, value: Vec<bool>) {
        set_slot(&mut self.bool_lists, local.0, value);
    }

    pub(super) fn get_bool_list(&self, local: BoolListLocalId) -> Vec<bool> {
        self.bool_lists[local.0].clone()
    }

    pub(super) fn set_nil_list(&mut self, local: NilListLocalId, len: usize) {
        set_slot(&mut self.nil_lists, local.0, len);
    }

    pub(super) fn get_nil_list(&self, local: NilListLocalId) -> usize {
        self.nil_lists[local.0]
    }

    pub(super) fn set_tuple_list(&mut self, local: TupleListLocalId, value: Vec<Vec<Value>>) {
        set_slot(&mut self.tuple_lists, local.0, value);
    }

    pub(super) fn get_tuple_list(&self, local: TupleListLocalId) -> Vec<Vec<Value>> {
        self.tuple_lists[local.0].clone()
    }

    pub(super) fn set_list_list(&mut self, local: ListListLocalId, value: Vec<ListValue>) {
        set_slot(&mut self.list_lists, local.0, value);
    }

    pub(super) fn get_list_list(&self, local: ListListLocalId) -> Vec<ListValue> {
        self.list_lists[local.0].clone()
    }

    pub(super) fn set_function_list(
        &mut self,
        local: FunctionListLocalId,
        value: Vec<FunctionValue>,
    ) {
        set_slot(&mut self.function_lists, local.0, value);
    }

    pub(super) fn get_function_list(&self, local: FunctionListLocalId) -> Vec<FunctionValue> {
        self.function_lists[local.0].clone()
    }

    pub(super) fn set_int_function(&mut self, local: IntFunctionLocalId, value: IntFunctionValue) {
        self.int_functions.insert(local, value);
    }

    pub(super) fn get_int_function(&self, local: IntFunctionLocalId) -> IntFunctionValue {
        self.int_functions[&local].clone()
    }

    pub(super) fn set_float_function(
        &mut self,
        local: FloatFunctionLocalId,
        value: FloatFunctionValue,
    ) {
        self.float_functions.insert(local, value);
    }

    pub(super) fn get_float_function(&self, local: FloatFunctionLocalId) -> FloatFunctionValue {
        self.float_functions[&local].clone()
    }

    pub(super) fn set_string_function(
        &mut self,
        local: StringFunctionLocalId,
        value: StringFunctionValue,
    ) {
        self.string_functions.insert(local, value);
    }

    pub(super) fn get_string_function(&self, local: StringFunctionLocalId) -> StringFunctionValue {
        self.string_functions[&local].clone()
    }

    pub(super) fn set_bool_function(
        &mut self,
        local: BoolFunctionLocalId,
        value: BoolFunctionValue,
    ) {
        self.bool_functions.insert(local, value);
    }

    pub(super) fn get_bool_function(&self, local: BoolFunctionLocalId) -> BoolFunctionValue {
        self.bool_functions[&local].clone()
    }

    pub(super) fn set_nil_function(&mut self, local: NilFunctionLocalId, value: NilFunctionValue) {
        self.nil_functions.insert(local, value);
    }

    pub(super) fn get_nil_function(&self, local: NilFunctionLocalId) -> NilFunctionValue {
        self.nil_functions[&local].clone()
    }

    pub(super) fn set_tuple_function(
        &mut self,
        local: TupleFunctionLocalId,
        value: TupleFunctionValue,
    ) {
        self.tuple_functions.insert(local, value);
    }

    pub(super) fn get_tuple_function(&self, local: TupleFunctionLocalId) -> TupleFunctionValue {
        self.tuple_functions[&local].clone()
    }

    pub(super) fn set_list_function(&mut self, local: ListFunctionLocal, value: ListFunctionValue) {
        self.list_functions.insert(local, value);
    }

    pub(super) fn get_list_function(&self, local: &ListFunctionLocal) -> ListFunctionValue {
        self.list_functions[local].clone()
    }

    pub(super) fn set_function_function(
        &mut self,
        local: FunctionFunctionLocalId,
        value: FunctionFunctionValue,
    ) {
        self.function_functions.insert(local, value);
    }

    pub(super) fn get_function_function(
        &self,
        local: FunctionFunctionLocalId,
    ) -> FunctionFunctionValue {
        self.function_functions[&local].clone()
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self::new(&FrameLayout::default())
    }
}

fn set_slot<T>(slots: &mut [T], index: usize, value: T) {
    slots[index] = value;
}
