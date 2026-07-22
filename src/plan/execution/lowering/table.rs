use super::super::FunctionTables;
use super::super::function::ExecutableFunction;
use super::super::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayFunctionReturn, BitArrayListFunctionId,
    BitArrayListReturn, BitArrayReturn, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionReturn,
    BoolListFunctionId, BoolListReturn, BoolReturn, CustomFunctionReturn, CustomListFunctionId,
    CustomListReturn, CustomReturn, FloatFunctionFunctionId, FloatFunctionId, FloatFunctionReturn,
    FloatListFunctionId, FloatListReturn, FloatReturn, FunctionFunctionId, FunctionFunctionReturn,
    FunctionListFunctionId, FunctionListReturn, GenericFunctionReturn, IntFunctionFunctionId,
    IntFunctionId, IntFunctionReturn, IntListFunctionId, IntListReturn, IntReturn,
    ListFunctionFunctionId, ListFunctionId, ListFunctionReturn, ListListFunctionId, ListListReturn,
    NeverFunctionReturn, NeverReturn, NilFunctionFunctionId, NilFunctionId, NilFunctionReturn,
    NilListFunctionId, NilListReturn, NilReturn, ParameterListFunctionId,
    ParameterListListFunctionId, ParameterListListReturn, ParameterListReturn, RuntimeFunctionId,
    StringFunctionFunctionId, StringFunctionId, StringFunctionReturn, StringListFunctionId,
    StringListReturn, StringReturn, TupleFunctionFunctionId, TupleFunctionId, TupleFunctionReturn,
    TupleListFunctionId, TupleListReturn, TupleReturn, UtfCodepointFunctionFunctionId,
    UtfCodepointFunctionId, UtfCodepointFunctionReturn, UtfCodepointListFunctionId,
    UtfCodepointListReturn, UtfCodepointReturn,
};
use super::SpecializationOutcome;
use super::specialization::{
    FunctionRepresentation, Representability, SpecializationKey, SpecializedFunctionShape,
    SpecializedValueShape, StoredValueShape,
};
use std::collections::HashSet;

mod graph;

pub(super) use graph::lower_specialized;

struct LoweredSpecialization<Value> {
    specialization: SpecializationKey,
    value: Representability<Value>,
}

type LoweredFunction<Return> = LoweredSpecialization<ExecutableFunction<Return>>;

fn lowered_function<Return>(
    specialization: &SpecializationKey,
    graph: Representability<super::graph::LoweredFunctionGraph<Return>>,
) -> LoweredFunction<Return> {
    LoweredSpecialization {
        specialization: specialization.clone(),
        value: graph.map(|graph| ExecutableFunction::new(graph.parameter_count, graph.body)),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum FunctionTableFamily {
    Never,
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Custom,
    Bool,
    Nil,
    Tuple,
    ParameterList,
    IntList,
    StringList,
    BitArrayList,
    UtfCodepointList,
    CustomList,
    FloatList,
    BoolList,
    NilList,
    TupleList,
    ParameterListList,
    ListList,
    FunctionList,
    IntFunction,
    FloatFunction,
    StringFunction,
    BitArrayFunction,
    UtfCodepointFunction,
    CustomFunction,
    BoolFunction,
    NilFunction,
    TupleFunction,
    GenericFunction,
    NeverFunction,
    ParameterListFunction,
    ParameterListListFunction,
    IntListFunction,
    StringListFunction,
    BitArrayListFunction,
    UtfCodepointListFunction,
    CustomListFunction,
    FloatListFunction,
    BoolListFunction,
    NilListFunction,
    TupleListFunction,
    ListListFunction,
    FunctionListFunction,
    FunctionFunction,
}

#[derive(Default)]
pub(super) struct FunctionTableBuilder {
    never_functions: Vec<(usize, LoweredFunction<NeverReturn>)>,
    int_functions: Vec<(usize, LoweredFunction<IntReturn>)>,
    float_functions: Vec<(usize, LoweredFunction<FloatReturn>)>,
    string_functions: Vec<(usize, LoweredFunction<StringReturn>)>,
    bit_array_functions: Vec<(usize, LoweredFunction<BitArrayReturn>)>,
    utf_codepoint_functions: Vec<(usize, LoweredFunction<UtfCodepointReturn>)>,
    custom_functions: Vec<(usize, LoweredFunction<CustomReturn>)>,
    bool_functions: Vec<(usize, LoweredFunction<BoolReturn>)>,
    nil_functions: Vec<(usize, LoweredFunction<NilReturn>)>,
    tuple_functions: Vec<(usize, LoweredFunction<TupleReturn>)>,
    parameter_list_functions: Vec<(
        ParameterListFunctionId,
        LoweredFunction<ParameterListReturn>,
    )>,
    int_list_functions: Vec<(IntListFunctionId, LoweredFunction<IntListReturn>)>,
    string_list_functions: Vec<(StringListFunctionId, LoweredFunction<StringListReturn>)>,
    bit_array_list_functions: Vec<(BitArrayListFunctionId, LoweredFunction<BitArrayListReturn>)>,
    utf_codepoint_list_functions: Vec<(
        UtfCodepointListFunctionId,
        LoweredFunction<UtfCodepointListReturn>,
    )>,
    custom_list_functions: Vec<(CustomListFunctionId, LoweredFunction<CustomListReturn>)>,
    float_list_functions: Vec<(FloatListFunctionId, LoweredFunction<FloatListReturn>)>,
    bool_list_functions: Vec<(BoolListFunctionId, LoweredFunction<BoolListReturn>)>,
    nil_list_functions: Vec<(NilListFunctionId, LoweredFunction<NilListReturn>)>,
    tuple_list_functions: Vec<(TupleListFunctionId, LoweredFunction<TupleListReturn>)>,
    parameter_list_list_functions: Vec<(
        ParameterListListFunctionId,
        LoweredFunction<ParameterListListReturn>,
    )>,
    list_list_functions: Vec<(ListListFunctionId, LoweredFunction<ListListReturn>)>,
    function_list_functions: Vec<(FunctionListFunctionId, LoweredFunction<FunctionListReturn>)>,
    int_function_functions: Vec<(usize, LoweredFunction<IntFunctionReturn>)>,
    float_function_functions: Vec<(usize, LoweredFunction<FloatFunctionReturn>)>,
    string_function_functions: Vec<(usize, LoweredFunction<StringFunctionReturn>)>,
    bit_array_function_functions: Vec<(usize, LoweredFunction<BitArrayFunctionReturn>)>,
    utf_codepoint_function_functions: Vec<(usize, LoweredFunction<UtfCodepointFunctionReturn>)>,
    custom_function_functions: Vec<(usize, LoweredFunction<CustomFunctionReturn>)>,
    bool_function_functions: Vec<(usize, LoweredFunction<BoolFunctionReturn>)>,
    nil_function_functions: Vec<(usize, LoweredFunction<NilFunctionReturn>)>,
    tuple_function_functions: Vec<(usize, LoweredFunction<TupleFunctionReturn>)>,
    generic_function_functions: Vec<(usize, LoweredFunction<GenericFunctionReturn>)>,
    never_function_functions: Vec<(usize, LoweredFunction<NeverFunctionReturn>)>,
    parameter_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    parameter_list_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    int_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    string_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    bit_array_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    utf_codepoint_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    custom_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    float_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    bool_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    nil_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    tuple_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    list_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    function_list_function_functions: Vec<(usize, LoweredFunction<ListFunctionReturn>)>,
    function_function_functions: Vec<(usize, LoweredFunction<FunctionFunctionReturn>)>,
}

impl FunctionTableBuilder {
    pub(super) fn finish(self) -> SpecializationOutcome<Box<FunctionTables>> {
        let mut erased = HashSet::new();
        let tables = FunctionTables {
            never_functions: sort_functions(self.never_functions, &mut erased),
            int_functions: sort_functions(self.int_functions, &mut erased),
            float_functions: sort_functions(self.float_functions, &mut erased),
            string_functions: sort_functions(self.string_functions, &mut erased),
            bit_array_functions: sort_functions(self.bit_array_functions, &mut erased),
            utf_codepoint_functions: sort_functions(self.utf_codepoint_functions, &mut erased),
            custom_functions: sort_functions(self.custom_functions, &mut erased),
            bool_functions: sort_functions(self.bool_functions, &mut erased),
            nil_functions: sort_functions(self.nil_functions, &mut erased),
            tuple_functions: sort_functions(self.tuple_functions, &mut erased),
            parameter_list_functions: sort_list_functions(
                self.parameter_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            int_list_functions: sort_list_functions(
                self.int_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            string_list_functions: sort_list_functions(
                self.string_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            bit_array_list_functions: sort_list_functions(
                self.bit_array_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            utf_codepoint_list_functions: sort_list_functions(
                self.utf_codepoint_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            custom_list_functions: sort_list_functions(
                self.custom_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            float_list_functions: sort_list_functions(
                self.float_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            bool_list_functions: sort_list_functions(
                self.bool_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            nil_list_functions: sort_list_functions(
                self.nil_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            tuple_list_functions: sort_list_functions(
                self.tuple_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            parameter_list_list_functions: sort_list_functions(
                self.parameter_list_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            list_list_functions: sort_list_functions(
                self.list_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            function_list_functions: sort_list_functions(
                self.function_list_functions,
                |id| id.index(),
                &mut erased,
            ),
            int_function_functions: sort_functions(self.int_function_functions, &mut erased),
            float_function_functions: sort_functions(self.float_function_functions, &mut erased),
            string_function_functions: sort_functions(self.string_function_functions, &mut erased),
            bit_array_function_functions: sort_functions(
                self.bit_array_function_functions,
                &mut erased,
            ),
            utf_codepoint_function_functions: sort_functions(
                self.utf_codepoint_function_functions,
                &mut erased,
            ),
            custom_function_functions: sort_functions(self.custom_function_functions, &mut erased),
            bool_function_functions: sort_functions(self.bool_function_functions, &mut erased),
            nil_function_functions: sort_functions(self.nil_function_functions, &mut erased),
            tuple_function_functions: sort_functions(self.tuple_function_functions, &mut erased),
            generic_function_functions: sort_functions(
                self.generic_function_functions,
                &mut erased,
            ),
            never_function_functions: sort_functions(self.never_function_functions, &mut erased),
            parameter_list_function_functions: sort_functions(
                self.parameter_list_function_functions,
                &mut erased,
            ),
            parameter_list_list_function_functions: sort_functions(
                self.parameter_list_list_function_functions,
                &mut erased,
            ),
            int_list_function_functions: sort_functions(
                self.int_list_function_functions,
                &mut erased,
            ),
            string_list_function_functions: sort_functions(
                self.string_list_function_functions,
                &mut erased,
            ),
            bit_array_list_function_functions: sort_functions(
                self.bit_array_list_function_functions,
                &mut erased,
            ),
            utf_codepoint_list_function_functions: sort_functions(
                self.utf_codepoint_list_function_functions,
                &mut erased,
            ),
            custom_list_function_functions: sort_functions(
                self.custom_list_function_functions,
                &mut erased,
            ),
            float_list_function_functions: sort_functions(
                self.float_list_function_functions,
                &mut erased,
            ),
            bool_list_function_functions: sort_functions(
                self.bool_list_function_functions,
                &mut erased,
            ),
            nil_list_function_functions: sort_functions(
                self.nil_list_function_functions,
                &mut erased,
            ),
            tuple_list_function_functions: sort_functions(
                self.tuple_list_function_functions,
                &mut erased,
            ),
            list_list_function_functions: sort_functions(
                self.list_list_function_functions,
                &mut erased,
            ),
            function_list_function_functions: sort_functions(
                self.function_list_function_functions,
                &mut erased,
            ),
            function_function_functions: sort_functions(
                self.function_function_functions,
                &mut erased,
            ),
        };
        SpecializationOutcome::complete_unless_erased(Box::new(tables), erased)
    }
}

fn push_list_function_function(
    functions: &mut FunctionTableBuilder,
    index: usize,
    item: &SpecializedValueShape,
    function: LoweredFunction<ListFunctionReturn>,
) {
    match item {
        SpecializedValueShape::Parameter(_) => functions
            .parameter_list_function_functions
            .push((index, function)),
        SpecializedValueShape::Int => functions
            .int_list_function_functions
            .push((index, function)),
        SpecializedValueShape::String => {
            functions
                .string_list_function_functions
                .push((index, function));
        }
        SpecializedValueShape::BitArray => {
            functions
                .bit_array_list_function_functions
                .push((index, function));
        }
        SpecializedValueShape::UtfCodepoint => {
            functions
                .utf_codepoint_list_function_functions
                .push((index, function));
        }
        SpecializedValueShape::Custom(_) => {
            functions
                .custom_list_function_functions
                .push((index, function));
        }
        SpecializedValueShape::Float => {
            functions
                .float_list_function_functions
                .push((index, function));
        }
        SpecializedValueShape::Bool => {
            functions
                .bool_list_function_functions
                .push((index, function));
        }
        SpecializedValueShape::Nil => {
            functions
                .nil_list_function_functions
                .push((index, function));
        }
        SpecializedValueShape::Tuple(_) => {
            functions
                .tuple_list_function_functions
                .push((index, function));
        }
        SpecializedValueShape::List(item) => match item.as_ref() {
            SpecializedValueShape::Parameter(_) => functions
                .parameter_list_list_function_functions
                .push((index, function)),
            _ => functions
                .list_list_function_functions
                .push((index, function)),
        },
        SpecializedValueShape::Function(_) => {
            functions
                .function_list_function_functions
                .push((index, function));
        }
    }
}

pub(super) fn function_id(
    shape: &StoredValueShape,
    index: usize,
    types: &mut super::value_type::TypeInterner,
    representations: &super::specialization::RepresentationContext,
) -> RuntimeFunctionId {
    match shape {
        StoredValueShape::Int => RuntimeFunctionId::Int(IntFunctionId(index)),
        StoredValueShape::Float => RuntimeFunctionId::Float(FloatFunctionId(index)),
        StoredValueShape::String => RuntimeFunctionId::String(StringFunctionId(index)),
        StoredValueShape::BitArray => RuntimeFunctionId::BitArray(BitArrayFunctionId(index)),
        StoredValueShape::UtfCodepoint => {
            RuntimeFunctionId::UtfCodepoint(UtfCodepointFunctionId(index))
        }
        StoredValueShape::Custom(shape) => RuntimeFunctionId::Custom(
            super::super::CustomFunctionId::new(index, types.custom_value_shape(shape)),
        ),
        StoredValueShape::Bool => RuntimeFunctionId::Bool(BoolFunctionId(index)),
        StoredValueShape::Nil => RuntimeFunctionId::Nil(NilFunctionId(index)),
        StoredValueShape::Tuple(elements) => RuntimeFunctionId::Tuple {
            id: TupleFunctionId(index),
            return_type: elements
                .iter()
                .map(|shape| types.value_type(shape))
                .collect(),
        },
        StoredValueShape::List(item) => {
            RuntimeFunctionId::List(list_function_id(item, index, types))
        }
        StoredValueShape::Function(function) => RuntimeFunctionId::Function {
            id: function_function_id(function, index, types, representations),
            return_type: types.function_type(function),
        },
    }
}

pub(super) fn stored_function_table_family(
    shape: &StoredValueShape,
    representations: &super::specialization::RepresentationContext,
) -> FunctionTableFamily {
    match shape {
        StoredValueShape::Int => FunctionTableFamily::Int,
        StoredValueShape::Float => FunctionTableFamily::Float,
        StoredValueShape::String => FunctionTableFamily::String,
        StoredValueShape::BitArray => FunctionTableFamily::BitArray,
        StoredValueShape::UtfCodepoint => FunctionTableFamily::UtfCodepoint,
        StoredValueShape::Custom(_) => FunctionTableFamily::Custom,
        StoredValueShape::Bool => FunctionTableFamily::Bool,
        StoredValueShape::Nil => FunctionTableFamily::Nil,
        StoredValueShape::Tuple(_) => FunctionTableFamily::Tuple,
        StoredValueShape::List(item) => list_function_table_family(item),
        StoredValueShape::Function(function) => {
            function_function_table_family(function, representations)
        }
    }
}

pub(super) fn list_function_table_family(item: &SpecializedValueShape) -> FunctionTableFamily {
    match item {
        SpecializedValueShape::Parameter(_) => FunctionTableFamily::ParameterList,
        SpecializedValueShape::Int => FunctionTableFamily::IntList,
        SpecializedValueShape::String => FunctionTableFamily::StringList,
        SpecializedValueShape::BitArray => FunctionTableFamily::BitArrayList,
        SpecializedValueShape::UtfCodepoint => FunctionTableFamily::UtfCodepointList,
        SpecializedValueShape::Custom(_) => FunctionTableFamily::CustomList,
        SpecializedValueShape::Float => FunctionTableFamily::FloatList,
        SpecializedValueShape::Bool => FunctionTableFamily::BoolList,
        SpecializedValueShape::Nil => FunctionTableFamily::NilList,
        SpecializedValueShape::Tuple(_) => FunctionTableFamily::TupleList,
        SpecializedValueShape::List(item) => match item.as_ref() {
            SpecializedValueShape::Parameter(_) => FunctionTableFamily::ParameterListList,
            _ => FunctionTableFamily::ListList,
        },
        SpecializedValueShape::Function(_) => FunctionTableFamily::FunctionList,
    }
}

pub(super) fn function_function_table_family(
    function: &SpecializedFunctionShape,
    representations: &super::specialization::RepresentationContext,
) -> FunctionTableFamily {
    match function.representation(representations) {
        FunctionRepresentation::Symbolic => FunctionTableFamily::GenericFunction,
        FunctionRepresentation::Never(_) => FunctionTableFamily::NeverFunction,
        FunctionRepresentation::Executable(return_) => {
            executable_function_function_table_family(&return_)
        }
    }
}

fn executable_function_function_table_family(return_: &StoredValueShape) -> FunctionTableFamily {
    match return_ {
        StoredValueShape::Int => FunctionTableFamily::IntFunction,
        StoredValueShape::Float => FunctionTableFamily::FloatFunction,
        StoredValueShape::String => FunctionTableFamily::StringFunction,
        StoredValueShape::BitArray => FunctionTableFamily::BitArrayFunction,
        StoredValueShape::UtfCodepoint => FunctionTableFamily::UtfCodepointFunction,
        StoredValueShape::Custom(_) => FunctionTableFamily::CustomFunction,
        StoredValueShape::Bool => FunctionTableFamily::BoolFunction,
        StoredValueShape::Nil => FunctionTableFamily::NilFunction,
        StoredValueShape::Tuple(_) => FunctionTableFamily::TupleFunction,
        StoredValueShape::List(item) => match item.as_ref() {
            SpecializedValueShape::Parameter(_) => FunctionTableFamily::ParameterListFunction,
            SpecializedValueShape::Int => FunctionTableFamily::IntListFunction,
            SpecializedValueShape::String => FunctionTableFamily::StringListFunction,
            SpecializedValueShape::BitArray => FunctionTableFamily::BitArrayListFunction,
            SpecializedValueShape::UtfCodepoint => FunctionTableFamily::UtfCodepointListFunction,
            SpecializedValueShape::Custom(_) => FunctionTableFamily::CustomListFunction,
            SpecializedValueShape::Float => FunctionTableFamily::FloatListFunction,
            SpecializedValueShape::Bool => FunctionTableFamily::BoolListFunction,
            SpecializedValueShape::Nil => FunctionTableFamily::NilListFunction,
            SpecializedValueShape::Tuple(_) => FunctionTableFamily::TupleListFunction,
            SpecializedValueShape::List(item) => match item.as_ref() {
                SpecializedValueShape::Parameter(_) => {
                    FunctionTableFamily::ParameterListListFunction
                }
                _ => FunctionTableFamily::ListListFunction,
            },
            SpecializedValueShape::Function(_) => FunctionTableFamily::FunctionListFunction,
        },
        StoredValueShape::Function(_) => FunctionTableFamily::FunctionFunction,
    }
}

pub(super) fn list_function_function_table_family(
    item: &SpecializedValueShape,
) -> FunctionTableFamily {
    executable_function_function_table_family(&StoredValueShape::List(Box::new(item.clone())))
}

pub(super) fn list_function_id(
    item: &SpecializedValueShape,
    index: usize,
    types: &mut super::value_type::TypeInterner,
) -> ListFunctionId {
    match item {
        SpecializedValueShape::Parameter(parameter) => ListFunctionId::Parameter(
            ParameterListFunctionId::new(index, types.parameter_list_type(*parameter)),
        ),
        SpecializedValueShape::Int => {
            ListFunctionId::Int(IntListFunctionId::new(index, types.int_list_type()))
        }
        SpecializedValueShape::String => {
            ListFunctionId::String(StringListFunctionId::new(index, types.string_list_type()))
        }
        SpecializedValueShape::BitArray => ListFunctionId::BitArray(BitArrayListFunctionId::new(
            index,
            types.bit_array_list_type(),
        )),
        SpecializedValueShape::UtfCodepoint => ListFunctionId::UtfCodepoint(
            UtfCodepointListFunctionId::new(index, types.utf_codepoint_list_type()),
        ),
        SpecializedValueShape::Custom(item) => ListFunctionId::Custom(CustomListFunctionId::new(
            index,
            types.custom_list_type(item),
        )),
        SpecializedValueShape::Float => {
            ListFunctionId::Float(FloatListFunctionId::new(index, types.float_list_type()))
        }
        SpecializedValueShape::Bool => {
            ListFunctionId::Bool(BoolListFunctionId::new(index, types.bool_list_type()))
        }
        SpecializedValueShape::Nil => {
            ListFunctionId::Nil(NilListFunctionId::new(index, types.nil_list_type()))
        }
        SpecializedValueShape::Tuple(item) => {
            ListFunctionId::Tuple(TupleListFunctionId::new(index, types.tuple_list_type(item)))
        }
        SpecializedValueShape::List(item) => match types.list_list_type(item) {
            super::value_type::NestedListTypeId::Parameter(type_id) => {
                ListFunctionId::ParameterList(ParameterListListFunctionId::new(index, type_id))
            }
            super::value_type::NestedListTypeId::Stored(type_id) => {
                ListFunctionId::List(ListListFunctionId::new(index, type_id))
            }
        },
        SpecializedValueShape::Function(item) => ListFunctionId::Function(
            FunctionListFunctionId::new(index, types.function_list_type(item)),
        ),
    }
}

pub(super) fn function_function_id(
    function: &SpecializedFunctionShape,
    index: usize,
    types: &mut super::value_type::TypeInterner,
    representations: &super::specialization::RepresentationContext,
) -> FunctionFunctionId {
    use super::super as execution;

    let return_ = match function.representation(representations) {
        FunctionRepresentation::Symbolic => {
            return FunctionFunctionId::Generic(execution::GenericFunctionFunctionId::new(
                index,
                types.generic_function_type(function),
            ));
        }
        FunctionRepresentation::Never(_) => {
            return FunctionFunctionId::Never(execution::NeverFunctionFunctionId::new(
                index,
                types.generic_function_type(function),
            ));
        }
        FunctionRepresentation::Executable(return_) => return_,
    };

    match return_ {
        StoredValueShape::Int => FunctionFunctionId::Int(IntFunctionFunctionId(index)),
        StoredValueShape::Float => FunctionFunctionId::Float(FloatFunctionFunctionId(index)),
        StoredValueShape::String => FunctionFunctionId::String(StringFunctionFunctionId(index)),
        StoredValueShape::BitArray => {
            FunctionFunctionId::BitArray(BitArrayFunctionFunctionId(index))
        }
        StoredValueShape::UtfCodepoint => {
            FunctionFunctionId::UtfCodepoint(UtfCodepointFunctionFunctionId(index))
        }
        StoredValueShape::Custom(return_) => {
            FunctionFunctionId::Custom(execution::CustomFunctionFunctionId::new(
                index,
                types.custom_function_type(function.arguments(), &return_),
            ))
        }
        StoredValueShape::Bool => FunctionFunctionId::Bool(BoolFunctionFunctionId(index)),
        StoredValueShape::Nil => FunctionFunctionId::Nil(NilFunctionFunctionId(index)),
        StoredValueShape::Tuple(_) => FunctionFunctionId::Tuple(TupleFunctionFunctionId(index)),
        StoredValueShape::List(item) => {
            FunctionFunctionId::List(list_function_function_id(function, &item, index, types))
        }
        StoredValueShape::Function(return_) => {
            FunctionFunctionId::Function(execution::FunctionFunctionFunctionId::new(
                index,
                types.function_function_type(function.arguments(), &return_),
            ))
        }
    }
}

pub(super) fn list_function_function_id(
    function: &SpecializedFunctionShape,
    item: &SpecializedValueShape,
    index: usize,
    types: &mut super::value_type::TypeInterner,
) -> ListFunctionFunctionId {
    let type_ = types.function_type(function);

    match item {
        SpecializedValueShape::Parameter(parameter) => ListFunctionFunctionId::Parameter {
            id: super::super::ParameterListFunctionFunctionId(index),
            type_,
            list_type: types.parameter_list_type(*parameter),
        },
        SpecializedValueShape::Int => ListFunctionFunctionId::Int {
            id: super::super::IntListFunctionFunctionId(index),
            type_,
            list_type: types.int_list_type(),
        },
        SpecializedValueShape::String => ListFunctionFunctionId::String {
            id: super::super::StringListFunctionFunctionId(index),
            type_,
            list_type: types.string_list_type(),
        },
        SpecializedValueShape::BitArray => ListFunctionFunctionId::BitArray {
            id: super::super::BitArrayListFunctionFunctionId(index),
            type_,
            list_type: types.bit_array_list_type(),
        },
        SpecializedValueShape::UtfCodepoint => ListFunctionFunctionId::UtfCodepoint {
            id: super::super::UtfCodepointListFunctionFunctionId(index),
            type_,
            list_type: types.utf_codepoint_list_type(),
        },
        SpecializedValueShape::Custom(item) => ListFunctionFunctionId::Custom {
            id: super::super::CustomListFunctionFunctionId(index),
            type_,
            list_type: types.custom_list_type(item),
        },
        SpecializedValueShape::Float => ListFunctionFunctionId::Float {
            id: super::super::FloatListFunctionFunctionId(index),
            type_,
            list_type: types.float_list_type(),
        },
        SpecializedValueShape::Bool => ListFunctionFunctionId::Bool {
            id: super::super::BoolListFunctionFunctionId(index),
            type_,
            list_type: types.bool_list_type(),
        },
        SpecializedValueShape::Nil => ListFunctionFunctionId::Nil {
            id: super::super::NilListFunctionFunctionId(index),
            type_,
            list_type: types.nil_list_type(),
        },
        SpecializedValueShape::Tuple(item) => ListFunctionFunctionId::Tuple {
            id: super::super::TupleListFunctionFunctionId(index),
            type_,
            list_type: types.tuple_list_type(item),
        },
        SpecializedValueShape::List(item) => match types.list_list_type(item) {
            super::value_type::NestedListTypeId::Parameter(list_type) => {
                ListFunctionFunctionId::ParameterList {
                    id: super::super::ParameterListListFunctionFunctionId(index),
                    type_,
                    list_type,
                }
            }
            super::value_type::NestedListTypeId::Stored(list_type) => {
                ListFunctionFunctionId::List {
                    id: super::super::ListListFunctionFunctionId(index),
                    type_,
                    list_type,
                }
            }
        },
        SpecializedValueShape::Function(item) => ListFunctionFunctionId::Function {
            id: super::super::FunctionListFunctionFunctionId(index),
            type_,
            list_type: types.function_list_type(item),
        },
    }
}

fn sort_functions<Return>(
    functions: Vec<(usize, LoweredFunction<Return>)>,
    erased: &mut HashSet<SpecializationKey>,
) -> Vec<ExecutableFunction<Return>> {
    sort_inhabited(functions, |index| *index, erased)
        .into_iter()
        .map(|(_, function)| function)
        .collect()
}

fn sort_list_functions<Id, Return>(
    functions: Vec<(Id, LoweredFunction<Return>)>,
    index: fn(&Id) -> usize,
    erased: &mut HashSet<SpecializationKey>,
) -> Vec<(Id, ExecutableFunction<Return>)> {
    sort_inhabited(functions, index, erased)
}

fn sort_inhabited<Id, Value>(
    mut values: Vec<(Id, LoweredSpecialization<Value>)>,
    index: fn(&Id) -> usize,
    erased: &mut HashSet<SpecializationKey>,
) -> Vec<(Id, Value)> {
    values.sort_by_key(|(id, _)| index(id));
    let mut lowered = Vec::new();
    for (id, specialization) in values {
        match specialization.value {
            Representability::Inhabited(value) => lowered.push((id, value)),
            Representability::Uninhabited => {
                erased.insert(specialization.specialization);
            }
        }
    }
    lowered
}

#[cfg(test)]
mod tests {
    use super::{LoweredSpecialization, sort_inhabited};
    use crate::plan::FunctionTemplateId;
    use crate::plan::execution::lowering::specialization::{Representability, SpecializationKey};
    use std::collections::HashSet;

    #[test]
    fn lowered_specializations_sort_by_id_and_record_erased_keys() {
        let erased_key = key(2);
        let mut erased = HashSet::new();

        let values = sort_inhabited(
            vec![
                inhabited(3, key(3), "value#3"),
                (
                    2,
                    LoweredSpecialization {
                        specialization: erased_key.clone(),
                        value: Representability::Uninhabited,
                    },
                ),
                inhabited(1, key(1), "value#1"),
            ],
            |index| *index,
            &mut erased,
        );

        assert_eq!(values, vec![(1, "value#1"), (3, "value#3")]);
        assert_eq!(erased, HashSet::from([erased_key]));
    }

    fn inhabited(
        index: usize,
        specialization: SpecializationKey,
        value: &'static str,
    ) -> (usize, LoweredSpecialization<&'static str>) {
        (
            index,
            LoweredSpecialization {
                specialization,
                value: Representability::Inhabited(value),
            },
        )
    }

    fn key(index: usize) -> SpecializationKey {
        SpecializationKey::monomorphic(FunctionTemplateId::new(index))
    }
}
