use super::super::draft::{
    BitArrayFunctionFamily, BitArrayListFamily, BoolFunctionFamily, BoolListFamily,
    CustomFunctionFamily, CustomListFamily, DraftBitArray, DraftBool, DraftCustom, DraftFloat,
    DraftFunction, DraftGraphValue, DraftInt, DraftList, DraftNeverReturn, DraftNil,
    DraftStoredList, DraftString, DraftTuple, DraftTypedFunction, DraftTypedList,
    DraftUtfCodepoint, DraftValueKey, DraftValueRef, FloatFunctionFamily, FloatListFamily,
    FunctionFunctionFamily, FunctionListFamily, GenericFunctionFamily, IntFunctionFamily,
    IntListFamily, ListFunctionFamily, ListListFamily, NeverFunctionFamily, NilFunctionFamily,
    NilListFamily, ParameterListFamily, ParameterListListFamily, StringFunctionFamily,
    StringListFamily, TupleFunctionFamily, TupleListFamily, UtfCodepointFunctionFamily,
    UtfCodepointListFamily,
};
use crate::plan::execution;
use std::collections::HashMap;

pub(in crate::plan::execution::lowering) trait FreezeGraphValue:
    DraftGraphValue
{
    type Frozen;

    fn freeze(&self, values: &BlockValues) -> Self::Frozen;
}

pub(in crate::plan::execution::lowering) trait FreezeListFamily {
    type Frozen;

    fn freeze(values: &BlockValues, value: &DraftList) -> Self::Frozen;
}

pub(in crate::plan::execution::lowering) trait FreezeFunctionFamily {
    type Frozen;

    fn freeze(values: &BlockValues, value: &DraftFunction) -> Self::Frozen;
}

#[derive(Default)]
pub(in crate::plan::execution::lowering) struct BlockValues {
    prefix: super::super::super::local::ParameterPrefix,
    slots: HashMap<DraftValueKey, execution::graph::ParamSlot>,
    all: HashMap<DraftValueKey, execution::graph::ParamLocal>,
    ints: HashMap<DraftValueKey, execution::graph::IntLocalId>,
    floats: HashMap<DraftValueKey, execution::graph::FloatLocalId>,
    strings: HashMap<DraftValueKey, execution::graph::StringLocalId>,
    bit_arrays: HashMap<DraftValueKey, execution::graph::BitArrayLocalId>,
    utf_codepoints: HashMap<DraftValueKey, execution::graph::UtfCodepointLocalId>,
    customs: HashMap<DraftValueKey, execution::graph::CustomLocal>,
    bools: HashMap<DraftValueKey, execution::graph::BoolLocalId>,
    nils: HashMap<DraftValueKey, execution::graph::NilLocalId>,
    tuples: HashMap<DraftValueKey, execution::graph::TupleLocalId>,
    lists: HashMap<DraftValueKey, execution::graph::ListLocal>,
    parameter_lists: HashMap<DraftValueKey, execution::graph::ParameterListLocalId>,
    parameter_list_lists: HashMap<DraftValueKey, execution::graph::ParameterListListLocalId>,
    int_lists: HashMap<DraftValueKey, execution::graph::IntListLocalId>,
    string_lists: HashMap<DraftValueKey, execution::graph::StringListLocalId>,
    bit_array_lists: HashMap<DraftValueKey, execution::graph::BitArrayListLocalId>,
    utf_codepoint_lists: HashMap<DraftValueKey, execution::graph::UtfCodepointListLocalId>,
    custom_lists: HashMap<DraftValueKey, execution::graph::CustomListLocalId>,
    float_lists: HashMap<DraftValueKey, execution::graph::FloatListLocalId>,
    bool_lists: HashMap<DraftValueKey, execution::graph::BoolListLocalId>,
    nil_lists: HashMap<DraftValueKey, execution::graph::NilListLocalId>,
    tuple_lists: HashMap<DraftValueKey, execution::graph::TupleListLocalId>,
    list_lists: HashMap<DraftValueKey, execution::graph::ListListLocalId>,
    function_lists: HashMap<DraftValueKey, execution::graph::FunctionListLocalId>,
    functions: HashMap<DraftValueKey, execution::graph::FunctionLocal>,
    int_functions: HashMap<DraftValueKey, execution::graph::IntFunctionLocalId>,
    float_functions: HashMap<DraftValueKey, execution::graph::FloatFunctionLocalId>,
    string_functions: HashMap<DraftValueKey, execution::graph::StringFunctionLocalId>,
    bit_array_functions: HashMap<DraftValueKey, execution::graph::BitArrayFunctionLocalId>,
    utf_codepoint_functions: HashMap<DraftValueKey, execution::graph::UtfCodepointFunctionLocalId>,
    generic_functions: HashMap<DraftValueKey, execution::graph::GenericFunctionLocal>,
    never_functions: HashMap<DraftValueKey, execution::graph::NeverFunctionLocal>,
    custom_functions: HashMap<DraftValueKey, execution::graph::CustomFunctionLocal>,
    bool_functions: HashMap<DraftValueKey, execution::graph::BoolFunctionLocalId>,
    nil_functions: HashMap<DraftValueKey, execution::graph::NilFunctionLocalId>,
    tuple_functions: HashMap<DraftValueKey, execution::graph::TupleFunctionLocalId>,
    list_functions: HashMap<DraftValueKey, execution::graph::ListFunctionLocal>,
    function_functions: HashMap<DraftValueKey, execution::graph::FunctionFunctionLocal>,
}

impl BlockValues {
    pub(super) fn allocate(
        &mut self,
        value: &DraftValueRef,
        context: &mut super::super::super::LoweringContext,
    ) -> execution::graph::ParamSlot {
        let (index, shape) = self
            .prefix
            .allocate_stored(value.shape().clone(), &context.representations);
        let local = super::super::super::local::stored_value_local_at(&shape, index, context);
        let shape_id = context.types.value_shape(&shape.to_specialized());
        let slot = execution::graph::ParamSlot::new(local.clone(), shape_id);
        self.slots.insert(value.key, slot.clone());
        self.insert(value.key, local);
        slot
    }

    fn insert(&mut self, key: DraftValueKey, local: execution::graph::ParamLocal) {
        use execution::graph::ParamLocal as L;

        self.all.insert(key, local.clone());
        match local {
            L::Int(local) => {
                self.ints.insert(key, local);
            }
            L::Float(local) => {
                self.floats.insert(key, local);
            }
            L::String(local) => {
                self.strings.insert(key, local);
            }
            L::BitArray(local) => {
                self.bit_arrays.insert(key, local);
            }
            L::UtfCodepoint(local) => {
                self.utf_codepoints.insert(key, local);
            }
            L::Custom(local) => {
                self.customs.insert(key, local);
            }
            L::Bool(local) => {
                self.bools.insert(key, local);
            }
            L::Nil(local) => {
                self.nils.insert(key, local);
            }
            L::Tuple { local, .. } => {
                self.tuples.insert(key, local);
            }
            L::List(local) => {
                self.insert_list(key, local);
            }
            L::IntFunction { local, .. } => {
                self.int_functions.insert(key, local);
                self.functions
                    .insert(key, execution::graph::FunctionLocal::Int(local));
            }
            L::FloatFunction { local, .. } => {
                self.float_functions.insert(key, local);
                self.functions
                    .insert(key, execution::graph::FunctionLocal::Float(local));
            }
            L::StringFunction { local, .. } => {
                self.string_functions.insert(key, local);
                self.functions
                    .insert(key, execution::graph::FunctionLocal::String(local));
            }
            L::BitArrayFunction { local, .. } => {
                self.bit_array_functions.insert(key, local);
                self.functions
                    .insert(key, execution::graph::FunctionLocal::BitArray(local));
            }
            L::UtfCodepointFunction { local, .. } => {
                self.utf_codepoint_functions.insert(key, local);
                self.functions
                    .insert(key, execution::graph::FunctionLocal::UtfCodepoint(local));
            }
            L::GenericFunction(local) => {
                self.generic_functions.insert(key, local.clone());
                self.functions
                    .insert(key, execution::graph::FunctionLocal::Generic(local));
            }
            L::NeverFunction(local) => {
                self.never_functions.insert(key, local.clone());
                self.functions
                    .insert(key, execution::graph::FunctionLocal::Never(local));
            }
            L::CustomFunction(local) => {
                self.custom_functions.insert(key, local.clone());
                self.functions
                    .insert(key, execution::graph::FunctionLocal::Custom(local));
            }
            L::BoolFunction { local, .. } => {
                self.bool_functions.insert(key, local);
                self.functions
                    .insert(key, execution::graph::FunctionLocal::Bool(local));
            }
            L::NilFunction { local, .. } => {
                self.nil_functions.insert(key, local);
                self.functions
                    .insert(key, execution::graph::FunctionLocal::Nil(local));
            }
            L::TupleFunction { local, .. } => {
                self.tuple_functions.insert(key, local);
                self.functions
                    .insert(key, execution::graph::FunctionLocal::Tuple(local));
            }
            L::ListFunction(local) => {
                self.list_functions.insert(key, local.clone());
                self.functions
                    .insert(key, execution::graph::FunctionLocal::List(local));
            }
            L::FunctionFunction(local) => {
                self.function_functions.insert(key, local.clone());
                self.functions
                    .insert(key, execution::graph::FunctionLocal::Function(local));
            }
        }
    }

    fn insert_list(&mut self, key: DraftValueKey, local: execution::graph::ListLocal) {
        use execution::graph::ListLocal as L;

        self.lists.insert(key, local.clone());
        match local {
            L::Parameter { local, .. } => {
                self.parameter_lists.insert(key, local);
            }
            L::ParameterList { local, .. } => {
                self.parameter_list_lists.insert(key, local);
            }
            L::Int { local, .. } => {
                self.int_lists.insert(key, local);
            }
            L::String { local, .. } => {
                self.string_lists.insert(key, local);
            }
            L::BitArray { local, .. } => {
                self.bit_array_lists.insert(key, local);
            }
            L::UtfCodepoint { local, .. } => {
                self.utf_codepoint_lists.insert(key, local);
            }
            L::Custom { local, .. } => {
                self.custom_lists.insert(key, local);
            }
            L::Float { local, .. } => {
                self.float_lists.insert(key, local);
            }
            L::Bool { local, .. } => {
                self.bool_lists.insert(key, local);
            }
            L::Nil { local, .. } => {
                self.nil_lists.insert(key, local);
            }
            L::Tuple { local, .. } => {
                self.tuple_lists.insert(key, local);
            }
            L::List { local, .. } => {
                self.list_lists.insert(key, local);
            }
            L::Function { local, .. } => {
                self.function_lists.insert(key, local);
            }
        }
    }

    pub(super) fn any(&self, value: &DraftValueRef) -> execution::graph::ParamLocal {
        self.all[&value.key].clone()
    }

    pub(super) fn slot(&self, value: &DraftValueRef) -> execution::graph::ParamSlot {
        self.slots[&value.key].clone()
    }

    pub(super) fn any_slice(
        &self,
        values: &[DraftValueRef],
    ) -> Box<[execution::graph::ParamLocal]> {
        values
            .iter()
            .map(|value| self.any(value))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub(super) fn capture(
        &self,
        target: &execution::graph::ParamLocal,
        source: &DraftValueRef,
    ) -> execution::graph::FunctionCapture {
        use execution::graph::FunctionCapture as C;
        use execution::graph::ListLocal as L;
        use execution::graph::ParamLocal as P;

        match target {
            P::Int(target) => C::Int {
                target: *target,
                source: self.ints[&source.key],
            },
            P::Float(target) => C::Float {
                target: *target,
                source: self.floats[&source.key],
            },
            P::String(target) => C::String {
                target: *target,
                source: self.strings[&source.key],
            },
            P::BitArray(target) => C::BitArray {
                target: *target,
                source: self.bit_arrays[&source.key],
            },
            P::UtfCodepoint(target) => C::UtfCodepoint {
                target: *target,
                source: self.utf_codepoints[&source.key],
            },
            P::Custom(target) => C::Custom {
                target: *target,
                source: self.customs[&source.key],
            },
            P::Bool(target) => C::Bool {
                target: *target,
                source: self.bools[&source.key],
            },
            P::Nil(target) => C::Nil {
                target: *target,
                source: self.nils[&source.key],
            },
            P::Tuple { local: target, .. } => C::Tuple {
                target: *target,
                source: self.tuples[&source.key],
            },
            P::List(list) => match list {
                L::Parameter { local: target, .. } => C::ParameterList {
                    target: *target,
                    source: self.parameter_lists[&source.key],
                },
                L::ParameterList { local: target, .. } => C::ParameterListList {
                    target: *target,
                    source: self.parameter_list_lists[&source.key],
                },
                L::Int { local: target, .. } => C::IntList {
                    target: *target,
                    source: self.int_lists[&source.key],
                },
                L::String { local: target, .. } => C::StringList {
                    target: *target,
                    source: self.string_lists[&source.key],
                },
                L::BitArray { local: target, .. } => C::BitArrayList {
                    target: *target,
                    source: self.bit_array_lists[&source.key],
                },
                L::UtfCodepoint { local: target, .. } => C::UtfCodepointList {
                    target: *target,
                    source: self.utf_codepoint_lists[&source.key],
                },
                L::Custom { local: target, .. } => C::CustomList {
                    target: *target,
                    source: self.custom_lists[&source.key],
                },
                L::Float { local: target, .. } => C::FloatList {
                    target: *target,
                    source: self.float_lists[&source.key],
                },
                L::Bool { local: target, .. } => C::BoolList {
                    target: *target,
                    source: self.bool_lists[&source.key],
                },
                L::Nil { local: target, .. } => C::NilList {
                    target: *target,
                    source: self.nil_lists[&source.key],
                },
                L::Tuple { local: target, .. } => C::TupleList {
                    target: *target,
                    source: self.tuple_lists[&source.key],
                },
                L::List { local: target, .. } => C::ListList {
                    target: *target,
                    source: self.list_lists[&source.key],
                },
                L::Function { local: target, .. } => C::FunctionList {
                    target: *target,
                    source: self.function_lists[&source.key],
                },
            },
            P::IntFunction { local: target, .. } => C::IntFunction {
                target: *target,
                source: self.int_functions[&source.key],
            },
            P::FloatFunction { local: target, .. } => C::FloatFunction {
                target: *target,
                source: self.float_functions[&source.key],
            },
            P::StringFunction { local: target, .. } => C::StringFunction {
                target: *target,
                source: self.string_functions[&source.key],
            },
            P::BitArrayFunction { local: target, .. } => C::BitArrayFunction {
                target: *target,
                source: self.bit_array_functions[&source.key],
            },
            P::UtfCodepointFunction { local: target, .. } => C::UtfCodepointFunction {
                target: *target,
                source: self.utf_codepoint_functions[&source.key],
            },
            P::GenericFunction(target) => C::GenericFunction {
                target: target.clone(),
                source: self.generic_functions[&source.key].clone(),
            },
            P::NeverFunction(target) => C::NeverFunction {
                target: target.clone(),
                source: self.never_functions[&source.key].clone(),
            },
            P::CustomFunction(target) => C::CustomFunction {
                target: target.clone(),
                source: self.custom_functions[&source.key].clone(),
            },
            P::BoolFunction { local: target, .. } => C::BoolFunction {
                target: *target,
                source: self.bool_functions[&source.key],
            },
            P::NilFunction { local: target, .. } => C::NilFunction {
                target: *target,
                source: self.nil_functions[&source.key],
            },
            P::TupleFunction { local: target, .. } => C::TupleFunction {
                target: *target,
                source: self.tuple_functions[&source.key],
            },
            P::ListFunction(target) => C::ListFunction {
                target: target.clone(),
                source: self.list_functions[&source.key].clone(),
            },
            P::FunctionFunction(target) => C::FunctionFunction {
                target: target.clone(),
                source: self.function_functions[&source.key].clone(),
            },
        }
    }

    pub(super) fn int(&self, value: &DraftInt) -> execution::graph::IntLocalId {
        self.ints[&value.key]
    }

    pub(super) fn float(&self, value: &DraftFloat) -> execution::graph::FloatLocalId {
        self.floats[&value.key]
    }

    pub(super) fn string(&self, value: &DraftString) -> execution::graph::StringLocalId {
        self.strings[&value.key]
    }

    pub(super) fn bit_array(&self, value: &DraftBitArray) -> execution::graph::BitArrayLocalId {
        self.bit_arrays[&value.key]
    }

    pub(super) fn utf_codepoint(
        &self,
        value: &DraftUtfCodepoint,
    ) -> execution::graph::UtfCodepointLocalId {
        self.utf_codepoints[&value.key]
    }

    pub(super) fn custom(&self, value: &DraftCustom) -> execution::graph::CustomLocal {
        self.customs[&value.key]
    }

    pub(super) fn bool(&self, value: &DraftBool) -> execution::graph::BoolLocalId {
        self.bools[&value.key]
    }

    pub(super) fn nil(&self, value: &DraftNil) -> execution::graph::NilLocalId {
        self.nils[&value.key]
    }

    pub(super) fn tuple(&self, value: &DraftTuple) -> execution::graph::TupleLocalId {
        self.tuples[&value.key]
    }

    pub(super) fn list(&self, value: &DraftList) -> execution::graph::ListLocal {
        self.lists[&value.key].clone()
    }

    pub(super) fn function(&self, value: &DraftFunction) -> execution::graph::FunctionLocal {
        self.functions[&value.key].clone()
    }

    pub(super) fn parameter_list(
        &self,
        value: &DraftList,
    ) -> execution::graph::ParameterListLocalId {
        self.parameter_lists[&value.key]
    }

    pub(super) fn parameter_list_list(
        &self,
        value: &DraftList,
    ) -> execution::graph::ParameterListListLocalId {
        self.parameter_list_lists[&value.key]
    }

    pub(super) fn int_list(&self, value: &DraftList) -> execution::graph::IntListLocalId {
        self.int_lists[&value.key]
    }

    pub(super) fn string_list(&self, value: &DraftList) -> execution::graph::StringListLocalId {
        self.string_lists[&value.key]
    }

    pub(super) fn bit_array_list(
        &self,
        value: &DraftList,
    ) -> execution::graph::BitArrayListLocalId {
        self.bit_array_lists[&value.key]
    }

    pub(super) fn utf_codepoint_list(
        &self,
        value: &DraftList,
    ) -> execution::graph::UtfCodepointListLocalId {
        self.utf_codepoint_lists[&value.key]
    }

    pub(super) fn custom_list(&self, value: &DraftList) -> execution::graph::CustomListLocalId {
        self.custom_lists[&value.key]
    }

    pub(super) fn float_list(&self, value: &DraftList) -> execution::graph::FloatListLocalId {
        self.float_lists[&value.key]
    }

    pub(super) fn bool_list(&self, value: &DraftList) -> execution::graph::BoolListLocalId {
        self.bool_lists[&value.key]
    }

    pub(super) fn nil_list(&self, value: &DraftList) -> execution::graph::NilListLocalId {
        self.nil_lists[&value.key]
    }

    pub(super) fn tuple_list(&self, value: &DraftList) -> execution::graph::TupleListLocalId {
        self.tuple_lists[&value.key]
    }

    pub(super) fn list_list(&self, value: &DraftList) -> execution::graph::ListListLocalId {
        self.list_lists[&value.key]
    }

    pub(super) fn function_list(&self, value: &DraftList) -> execution::graph::FunctionListLocalId {
        self.function_lists[&value.key]
    }

    pub(super) fn stored_list(&self, value: &DraftStoredList) -> execution::graph::StoredListLocal {
        use execution::graph::StoredListLocal as S;

        match value {
            DraftStoredList::ParameterList(value) => {
                S::ParameterList(self.parameter_list_lists[&value.key])
            }
            DraftStoredList::Int(value) => S::Int(self.int_lists[&value.key]),
            DraftStoredList::String(value) => S::String(self.string_lists[&value.key]),
            DraftStoredList::BitArray(value) => S::BitArray(self.bit_array_lists[&value.key]),
            DraftStoredList::UtfCodepoint(value) => {
                S::UtfCodepoint(self.utf_codepoint_lists[&value.key])
            }
            DraftStoredList::Custom(value) => S::Custom(self.custom_lists[&value.key]),
            DraftStoredList::Float(value) => S::Float(self.float_lists[&value.key]),
            DraftStoredList::Bool(value) => S::Bool(self.bool_lists[&value.key]),
            DraftStoredList::Nil(value) => S::Nil(self.nil_lists[&value.key]),
            DraftStoredList::Tuple(value) => S::Tuple(self.tuple_lists[&value.key]),
            DraftStoredList::List(value) => S::List(self.list_lists[&value.key]),
            DraftStoredList::Function(value) => S::Function(self.function_lists[&value.key]),
        }
    }

    pub(super) fn int_function(
        &self,
        value: &DraftFunction,
    ) -> execution::graph::IntFunctionLocalId {
        self.int_functions[&value.key]
    }

    pub(super) fn float_function(
        &self,
        value: &DraftFunction,
    ) -> execution::graph::FloatFunctionLocalId {
        self.float_functions[&value.key]
    }

    pub(super) fn string_function(
        &self,
        value: &DraftFunction,
    ) -> execution::graph::StringFunctionLocalId {
        self.string_functions[&value.key]
    }

    pub(super) fn bit_array_function(
        &self,
        value: &DraftFunction,
    ) -> execution::graph::BitArrayFunctionLocalId {
        self.bit_array_functions[&value.key]
    }

    pub(super) fn utf_codepoint_function(
        &self,
        value: &DraftFunction,
    ) -> execution::graph::UtfCodepointFunctionLocalId {
        self.utf_codepoint_functions[&value.key]
    }

    pub(super) fn generic_function(
        &self,
        value: &DraftFunction,
    ) -> execution::graph::GenericFunctionLocal {
        self.generic_functions[&value.key].clone()
    }

    pub(super) fn never_function(
        &self,
        value: &DraftFunction,
    ) -> execution::graph::NeverFunctionLocal {
        self.never_functions[&value.key].clone()
    }

    pub(super) fn custom_function(
        &self,
        value: &DraftFunction,
    ) -> execution::graph::CustomFunctionLocal {
        self.custom_functions[&value.key].clone()
    }

    pub(super) fn bool_function(
        &self,
        value: &DraftFunction,
    ) -> execution::graph::BoolFunctionLocalId {
        self.bool_functions[&value.key]
    }

    pub(super) fn nil_function(
        &self,
        value: &DraftFunction,
    ) -> execution::graph::NilFunctionLocalId {
        self.nil_functions[&value.key]
    }

    pub(super) fn tuple_function(
        &self,
        value: &DraftFunction,
    ) -> execution::graph::TupleFunctionLocalId {
        self.tuple_functions[&value.key]
    }

    pub(super) fn list_function(
        &self,
        value: &DraftFunction,
    ) -> execution::graph::ListFunctionLocal {
        self.list_functions[&value.key].clone()
    }

    pub(super) fn function_function(
        &self,
        value: &DraftFunction,
    ) -> execution::graph::FunctionFunctionLocal {
        self.function_functions[&value.key].clone()
    }
}

impl FreezeGraphValue for DraftInt {
    type Frozen = execution::graph::IntLocalId;

    fn freeze(&self, values: &BlockValues) -> Self::Frozen {
        values.int(self)
    }
}

impl FreezeGraphValue for DraftFloat {
    type Frozen = execution::graph::FloatLocalId;

    fn freeze(&self, values: &BlockValues) -> Self::Frozen {
        values.float(self)
    }
}

impl FreezeGraphValue for DraftString {
    type Frozen = execution::graph::StringLocalId;

    fn freeze(&self, values: &BlockValues) -> Self::Frozen {
        values.string(self)
    }
}

impl FreezeGraphValue for DraftBitArray {
    type Frozen = execution::graph::BitArrayLocalId;

    fn freeze(&self, values: &BlockValues) -> Self::Frozen {
        values.bit_array(self)
    }
}

impl FreezeGraphValue for DraftUtfCodepoint {
    type Frozen = execution::graph::UtfCodepointLocalId;

    fn freeze(&self, values: &BlockValues) -> Self::Frozen {
        values.utf_codepoint(self)
    }
}

impl FreezeGraphValue for DraftCustom {
    type Frozen = execution::graph::CustomLocal;

    fn freeze(&self, values: &BlockValues) -> Self::Frozen {
        values.custom(self)
    }
}

impl FreezeGraphValue for DraftBool {
    type Frozen = execution::graph::BoolLocalId;

    fn freeze(&self, values: &BlockValues) -> Self::Frozen {
        values.bool(self)
    }
}

impl FreezeGraphValue for DraftNil {
    type Frozen = execution::graph::NilLocalId;

    fn freeze(&self, values: &BlockValues) -> Self::Frozen {
        values.nil(self)
    }
}

impl FreezeGraphValue for DraftTuple {
    type Frozen = execution::graph::TupleLocalId;

    fn freeze(&self, values: &BlockValues) -> Self::Frozen {
        values.tuple(self)
    }
}

impl FreezeGraphValue for DraftFunction {
    type Frozen = execution::graph::FunctionLocal;

    fn freeze(&self, values: &BlockValues) -> Self::Frozen {
        values.function(self)
    }
}

impl<Family: FreezeListFamily> FreezeGraphValue for DraftTypedList<Family> {
    type Frozen = Family::Frozen;

    fn freeze(&self, values: &BlockValues) -> Self::Frozen {
        Family::freeze(values, self.value())
    }
}

impl<Family: FreezeFunctionFamily> FreezeGraphValue for DraftTypedFunction<Family> {
    type Frozen = Family::Frozen;

    fn freeze(&self, values: &BlockValues) -> Self::Frozen {
        Family::freeze(values, self.value())
    }
}

impl FreezeListFamily for ParameterListFamily {
    type Frozen = execution::graph::ParameterListLocalId;

    fn freeze(values: &BlockValues, value: &DraftList) -> Self::Frozen {
        values.parameter_list(value)
    }
}

impl FreezeListFamily for ParameterListListFamily {
    type Frozen = execution::graph::ParameterListListLocalId;

    fn freeze(values: &BlockValues, value: &DraftList) -> Self::Frozen {
        values.parameter_list_list(value)
    }
}

impl FreezeListFamily for IntListFamily {
    type Frozen = execution::graph::IntListLocalId;

    fn freeze(values: &BlockValues, value: &DraftList) -> Self::Frozen {
        values.int_list(value)
    }
}

impl FreezeListFamily for StringListFamily {
    type Frozen = execution::graph::StringListLocalId;

    fn freeze(values: &BlockValues, value: &DraftList) -> Self::Frozen {
        values.string_list(value)
    }
}

impl FreezeListFamily for BitArrayListFamily {
    type Frozen = execution::graph::BitArrayListLocalId;

    fn freeze(values: &BlockValues, value: &DraftList) -> Self::Frozen {
        values.bit_array_list(value)
    }
}

impl FreezeListFamily for UtfCodepointListFamily {
    type Frozen = execution::graph::UtfCodepointListLocalId;

    fn freeze(values: &BlockValues, value: &DraftList) -> Self::Frozen {
        values.utf_codepoint_list(value)
    }
}

impl FreezeListFamily for CustomListFamily {
    type Frozen = execution::graph::CustomListLocalId;

    fn freeze(values: &BlockValues, value: &DraftList) -> Self::Frozen {
        values.custom_list(value)
    }
}

impl FreezeListFamily for FloatListFamily {
    type Frozen = execution::graph::FloatListLocalId;

    fn freeze(values: &BlockValues, value: &DraftList) -> Self::Frozen {
        values.float_list(value)
    }
}

impl FreezeListFamily for BoolListFamily {
    type Frozen = execution::graph::BoolListLocalId;

    fn freeze(values: &BlockValues, value: &DraftList) -> Self::Frozen {
        values.bool_list(value)
    }
}

impl FreezeListFamily for NilListFamily {
    type Frozen = execution::graph::NilListLocalId;

    fn freeze(values: &BlockValues, value: &DraftList) -> Self::Frozen {
        values.nil_list(value)
    }
}

impl FreezeListFamily for TupleListFamily {
    type Frozen = execution::graph::TupleListLocalId;

    fn freeze(values: &BlockValues, value: &DraftList) -> Self::Frozen {
        values.tuple_list(value)
    }
}

impl FreezeListFamily for ListListFamily {
    type Frozen = execution::graph::ListListLocalId;

    fn freeze(values: &BlockValues, value: &DraftList) -> Self::Frozen {
        values.list_list(value)
    }
}

impl FreezeListFamily for FunctionListFamily {
    type Frozen = execution::graph::FunctionListLocalId;

    fn freeze(values: &BlockValues, value: &DraftList) -> Self::Frozen {
        values.function_list(value)
    }
}

impl FreezeFunctionFamily for GenericFunctionFamily {
    type Frozen = execution::graph::GenericFunctionLocal;

    fn freeze(values: &BlockValues, value: &DraftFunction) -> Self::Frozen {
        values.generic_function(value)
    }
}

impl FreezeFunctionFamily for NeverFunctionFamily {
    type Frozen = execution::graph::NeverFunctionLocal;

    fn freeze(values: &BlockValues, value: &DraftFunction) -> Self::Frozen {
        values.never_function(value)
    }
}

impl FreezeFunctionFamily for IntFunctionFamily {
    type Frozen = execution::graph::IntFunctionLocalId;

    fn freeze(values: &BlockValues, value: &DraftFunction) -> Self::Frozen {
        values.int_function(value)
    }
}

impl FreezeFunctionFamily for FloatFunctionFamily {
    type Frozen = execution::graph::FloatFunctionLocalId;

    fn freeze(values: &BlockValues, value: &DraftFunction) -> Self::Frozen {
        values.float_function(value)
    }
}

impl FreezeFunctionFamily for StringFunctionFamily {
    type Frozen = execution::graph::StringFunctionLocalId;

    fn freeze(values: &BlockValues, value: &DraftFunction) -> Self::Frozen {
        values.string_function(value)
    }
}

impl FreezeFunctionFamily for BitArrayFunctionFamily {
    type Frozen = execution::graph::BitArrayFunctionLocalId;

    fn freeze(values: &BlockValues, value: &DraftFunction) -> Self::Frozen {
        values.bit_array_function(value)
    }
}

impl FreezeFunctionFamily for UtfCodepointFunctionFamily {
    type Frozen = execution::graph::UtfCodepointFunctionLocalId;

    fn freeze(values: &BlockValues, value: &DraftFunction) -> Self::Frozen {
        values.utf_codepoint_function(value)
    }
}

impl FreezeFunctionFamily for CustomFunctionFamily {
    type Frozen = execution::graph::CustomFunctionLocal;

    fn freeze(values: &BlockValues, value: &DraftFunction) -> Self::Frozen {
        values.custom_function(value)
    }
}

impl FreezeFunctionFamily for BoolFunctionFamily {
    type Frozen = execution::graph::BoolFunctionLocalId;

    fn freeze(values: &BlockValues, value: &DraftFunction) -> Self::Frozen {
        values.bool_function(value)
    }
}

impl FreezeFunctionFamily for NilFunctionFamily {
    type Frozen = execution::graph::NilFunctionLocalId;

    fn freeze(values: &BlockValues, value: &DraftFunction) -> Self::Frozen {
        values.nil_function(value)
    }
}

impl FreezeFunctionFamily for TupleFunctionFamily {
    type Frozen = execution::graph::TupleFunctionLocalId;

    fn freeze(values: &BlockValues, value: &DraftFunction) -> Self::Frozen {
        values.tuple_function(value)
    }
}

impl FreezeFunctionFamily for ListFunctionFamily {
    type Frozen = execution::graph::ListFunctionLocal;

    fn freeze(values: &BlockValues, value: &DraftFunction) -> Self::Frozen {
        values.list_function(value)
    }
}

impl FreezeFunctionFamily for FunctionFunctionFamily {
    type Frozen = execution::graph::FunctionFunctionLocal;

    fn freeze(values: &BlockValues, value: &DraftFunction) -> Self::Frozen {
        values.function_function(value)
    }
}

impl FreezeGraphValue for DraftNeverReturn {
    type Frozen = std::convert::Infallible;

    fn freeze(&self, _values: &BlockValues) -> Self::Frozen {
        match *self {}
    }
}
