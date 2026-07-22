use super::specialization::{
    FunctionRepresentation, SpecializedFunctionShape, SpecializedTypeSubstitution,
    SpecializedValueShape, StoredValueShape,
};
use super::{LoweringContext, SpecializedFunctionLocal};
use crate::plan::{execution, module};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum LocalKind {
    Generic,
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Custom,
    Bool,
    Nil,
    Tuple,
    GenericList,
    IntList,
    StringList,
    BitArrayList,
    UtfCodepointList,
    CustomList,
    FloatList,
    BoolList,
    NilList,
    TupleList,
    ListList,
    FunctionList,
    GenericFunction,
    IntFunction,
    FloatFunction,
    StringFunction,
    BitArrayFunction,
    UtfCodepointFunction,
    CustomFunction,
    BoolFunction,
    NilFunction,
    TupleFunction,
    GenericListFunction,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct LocalKey {
    kind: LocalKind,
    index: usize,
}

impl LocalKey {
    pub(super) fn new(kind: LocalKind, index: usize) -> Self {
        Self { kind, index }
    }
}

#[derive(Clone)]
pub(super) struct FunctionEntryTemplate {
    params: Box<[crate::plan::ValueShape]>,
    captures: Box<[crate::plan::ValueShape]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum StorageFamily {
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
    ListList,
    ParameterListList,
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
    IntListFunction,
    StringListFunction,
    BitArrayListFunction,
    UtfCodepointListFunction,
    CustomListFunction,
    FloatListFunction,
    BoolListFunction,
    NilListFunction,
    TupleListFunction,
    ParameterListListFunction,
    ListListFunction,
    FunctionListFunction,
    FunctionFunction,
}

#[derive(Default)]
pub(super) struct ParameterPrefix {
    next: HashMap<StorageFamily, usize>,
}

impl FunctionEntryTemplate {
    pub(super) fn new(template: &module::FunctionTemplate) -> Self {
        let params = template
            .entry()
            .params()
            .iter()
            .map(|param| param.shape().clone())
            .collect::<Vec<_>>();
        let captures = template
            .entry()
            .captures()
            .iter()
            .map(|capture| capture.shape().clone())
            .collect::<Vec<_>>();
        Self {
            params: params.into_boxed_slice(),
            captures: captures.into_boxed_slice(),
        }
    }

    pub(super) fn capture_target(
        &self,
        position: module::CapturePosition,
        source_shape: StoredValueShape,
        substitution: &SpecializedTypeSubstitution,
        representations: &super::specialization::RepresentationContext,
    ) -> (usize, StoredValueShape) {
        let mut prefix = ParameterPrefix::default();
        for shape in self.params.iter().chain(&self.captures[..position.index()]) {
            let shape = SpecializedValueShape::instantiate(shape, substitution);
            if let Some(stored) = representations.stored_shape(&shape) {
                prefix.allocate_stored(stored, representations);
            }
        }
        prefix.allocate_stored(source_shape, representations)
    }
}

impl ParameterPrefix {
    pub(super) fn allocate_stored(
        &mut self,
        stored: StoredValueShape,
        representations: &super::specialization::RepresentationContext,
    ) -> (usize, StoredValueShape) {
        let index = self
            .next
            .entry(StorageFamily::of(&stored, representations))
            .or_default();
        let allocated = *index;
        *index += 1;
        (allocated, stored)
    }
}

impl StorageFamily {
    fn of(
        shape: &StoredValueShape,
        representations: &super::specialization::RepresentationContext,
    ) -> Self {
        match shape {
            StoredValueShape::Int => Self::Int,
            StoredValueShape::Float => Self::Float,
            StoredValueShape::String => Self::String,
            StoredValueShape::BitArray => Self::BitArray,
            StoredValueShape::UtfCodepoint => Self::UtfCodepoint,
            StoredValueShape::Custom(_) => Self::Custom,
            StoredValueShape::Bool => Self::Bool,
            StoredValueShape::Nil => Self::Nil,
            StoredValueShape::Tuple(_) => Self::Tuple,
            StoredValueShape::List(item) => Self::list(item),
            StoredValueShape::Function(function) => Self::function(function, representations),
        }
    }

    fn list(item: &SpecializedValueShape) -> Self {
        match item {
            SpecializedValueShape::Parameter(_) => Self::ParameterList,
            SpecializedValueShape::Int => Self::IntList,
            SpecializedValueShape::String => Self::StringList,
            SpecializedValueShape::BitArray => Self::BitArrayList,
            SpecializedValueShape::UtfCodepoint => Self::UtfCodepointList,
            SpecializedValueShape::Custom(_) => Self::CustomList,
            SpecializedValueShape::Float => Self::FloatList,
            SpecializedValueShape::Bool => Self::BoolList,
            SpecializedValueShape::Nil => Self::NilList,
            SpecializedValueShape::Tuple(_) => Self::TupleList,
            SpecializedValueShape::List(item) => match item.as_ref() {
                SpecializedValueShape::Parameter(_) => Self::ParameterListList,
                _ => Self::ListList,
            },
            SpecializedValueShape::Function(_) => Self::FunctionList,
        }
    }

    fn function(
        function: &SpecializedFunctionShape,
        representations: &super::specialization::RepresentationContext,
    ) -> Self {
        match function.representation(representations) {
            FunctionRepresentation::Symbolic => Self::GenericFunction,
            FunctionRepresentation::Never(_) => Self::NeverFunction,
            FunctionRepresentation::Executable(return_) => match return_ {
                StoredValueShape::Int => Self::IntFunction,
                StoredValueShape::Float => Self::FloatFunction,
                StoredValueShape::String => Self::StringFunction,
                StoredValueShape::BitArray => Self::BitArrayFunction,
                StoredValueShape::UtfCodepoint => Self::UtfCodepointFunction,
                StoredValueShape::Custom(_) => Self::CustomFunction,
                StoredValueShape::Bool => Self::BoolFunction,
                StoredValueShape::Nil => Self::NilFunction,
                StoredValueShape::Tuple(_) => Self::TupleFunction,
                StoredValueShape::List(item) => Self::list_function(&item),
                StoredValueShape::Function(_) => Self::FunctionFunction,
            },
        }
    }

    fn list_function(item: &SpecializedValueShape) -> Self {
        match item {
            SpecializedValueShape::Parameter(_) => Self::ParameterListFunction,
            SpecializedValueShape::Int => Self::IntListFunction,
            SpecializedValueShape::String => Self::StringListFunction,
            SpecializedValueShape::BitArray => Self::BitArrayListFunction,
            SpecializedValueShape::UtfCodepoint => Self::UtfCodepointListFunction,
            SpecializedValueShape::Custom(_) => Self::CustomListFunction,
            SpecializedValueShape::Float => Self::FloatListFunction,
            SpecializedValueShape::Bool => Self::BoolListFunction,
            SpecializedValueShape::Nil => Self::NilListFunction,
            SpecializedValueShape::Tuple(_) => Self::TupleListFunction,
            SpecializedValueShape::List(item) => match item.as_ref() {
                SpecializedValueShape::Parameter(_) => Self::ParameterListListFunction,
                _ => Self::ListListFunction,
            },
            SpecializedValueShape::Function(_) => Self::FunctionListFunction,
        }
    }
}

pub(super) fn param_local_key(local: &module::ParamLocal) -> LocalKey {
    match local {
        module::ParamLocal::Generic(local) => LocalKey::new(LocalKind::Generic, local.id().0),
        module::ParamLocal::Int(local) => LocalKey::new(LocalKind::Int, local.0),
        module::ParamLocal::Float(local) => LocalKey::new(LocalKind::Float, local.0),
        module::ParamLocal::String(local) => LocalKey::new(LocalKind::String, local.0),
        module::ParamLocal::BitArray(local) => LocalKey::new(LocalKind::BitArray, local.0),
        module::ParamLocal::UtfCodepoint(local) => LocalKey::new(LocalKind::UtfCodepoint, local.0),
        module::ParamLocal::Custom(local) => LocalKey::new(LocalKind::Custom, local.id().0),
        module::ParamLocal::Bool(local) => LocalKey::new(LocalKind::Bool, local.0),
        module::ParamLocal::Nil(local) => LocalKey::new(LocalKind::Nil, local.0),
        module::ParamLocal::Tuple { local, .. } => LocalKey::new(LocalKind::Tuple, local.0),
        module::ParamLocal::List(local) => list_local_key(local),
        module::ParamLocal::IntFunction { local, .. } => {
            LocalKey::new(LocalKind::IntFunction, local.0)
        }
        module::ParamLocal::FloatFunction { local, .. } => {
            LocalKey::new(LocalKind::FloatFunction, local.0)
        }
        module::ParamLocal::StringFunction { local, .. } => {
            LocalKey::new(LocalKind::StringFunction, local.0)
        }
        module::ParamLocal::BitArrayFunction { local, .. } => {
            LocalKey::new(LocalKind::BitArrayFunction, local.0)
        }
        module::ParamLocal::UtfCodepointFunction { local, .. } => {
            LocalKey::new(LocalKind::UtfCodepointFunction, local.0)
        }
        module::ParamLocal::CustomFunction(local) => {
            LocalKey::new(LocalKind::CustomFunction, local.id().0)
        }
        module::ParamLocal::BoolFunction { local, .. } => {
            LocalKey::new(LocalKind::BoolFunction, local.0)
        }
        module::ParamLocal::NilFunction { local, .. } => {
            LocalKey::new(LocalKind::NilFunction, local.0)
        }
        module::ParamLocal::TupleFunction { local, .. } => {
            LocalKey::new(LocalKind::TupleFunction, local.0)
        }
        module::ParamLocal::ListFunction(local) => list_function_local_key(local),
        module::ParamLocal::FunctionFunction(local) => {
            LocalKey::new(LocalKind::FunctionFunction, local.id().0)
        }
        module::ParamLocal::GenericFunction(local) => {
            LocalKey::new(LocalKind::GenericFunction, local.id().0)
        }
    }
}

pub(super) fn list_local_key(local: &module::ListLocal) -> LocalKey {
    match local {
        module::ListLocal::Generic { local, .. } => LocalKey::new(LocalKind::GenericList, local.0),
        module::ListLocal::Int(local) => LocalKey::new(LocalKind::IntList, local.0),
        module::ListLocal::String(local) => LocalKey::new(LocalKind::StringList, local.0),
        module::ListLocal::BitArray(local) => LocalKey::new(LocalKind::BitArrayList, local.0),
        module::ListLocal::UtfCodepoint(local) => {
            LocalKey::new(LocalKind::UtfCodepointList, local.0)
        }
        module::ListLocal::Custom { local, .. } => LocalKey::new(LocalKind::CustomList, local.0),
        module::ListLocal::Float(local) => LocalKey::new(LocalKind::FloatList, local.0),
        module::ListLocal::Bool(local) => LocalKey::new(LocalKind::BoolList, local.0),
        module::ListLocal::Nil(local) => LocalKey::new(LocalKind::NilList, local.0),
        module::ListLocal::Tuple { local, .. } => LocalKey::new(LocalKind::TupleList, local.0),
        module::ListLocal::List { local, .. } => LocalKey::new(LocalKind::ListList, local.0),
        module::ListLocal::Function { local, .. } => {
            LocalKey::new(LocalKind::FunctionList, local.0)
        }
    }
}

pub(super) fn list_function_local_key(local: &module::ListFunctionLocal) -> LocalKey {
    let (kind, index, _) = list_function_local_parts(local);
    LocalKey::new(kind, index)
}

fn list_function_local_parts(
    local: &module::ListFunctionLocal,
) -> (LocalKind, usize, crate::plan::FunctionType) {
    match local {
        module::ListFunctionLocal::Generic { local, type_, .. } => {
            (LocalKind::GenericListFunction, local.0, type_.clone())
        }
        module::ListFunctionLocal::Int { local, type_ } => {
            (LocalKind::IntListFunction, local.0, type_.clone())
        }
        module::ListFunctionLocal::String { local, type_ } => {
            (LocalKind::StringListFunction, local.0, type_.clone())
        }
        module::ListFunctionLocal::BitArray { local, type_ } => {
            (LocalKind::BitArrayListFunction, local.0, type_.clone())
        }
        module::ListFunctionLocal::UtfCodepoint { local, type_ } => {
            (LocalKind::UtfCodepointListFunction, local.0, type_.clone())
        }
        module::ListFunctionLocal::Custom { local, type_, .. } => {
            (LocalKind::CustomListFunction, local.0, type_.clone())
        }
        module::ListFunctionLocal::Float { local, type_ } => {
            (LocalKind::FloatListFunction, local.0, type_.clone())
        }
        module::ListFunctionLocal::Bool { local, type_ } => {
            (LocalKind::BoolListFunction, local.0, type_.clone())
        }
        module::ListFunctionLocal::Nil { local, type_ } => {
            (LocalKind::NilListFunction, local.0, type_.clone())
        }
        module::ListFunctionLocal::Tuple { local, type_, .. } => {
            (LocalKind::TupleListFunction, local.0, type_.clone())
        }
        module::ListFunctionLocal::List { local, type_, .. } => {
            (LocalKind::ListListFunction, local.0, type_.clone())
        }
        module::ListFunctionLocal::Function { local, type_, .. } => {
            (LocalKind::FunctionListFunction, local.0, type_.clone())
        }
    }
}

pub(super) fn stored_value_local_at(
    shape: &StoredValueShape,
    index: usize,
    context: &mut LoweringContext,
) -> execution::ParamLocal {
    match shape {
        StoredValueShape::Int => execution::ParamLocal::Int(execution::IntLocalId(index)),
        StoredValueShape::Float => execution::ParamLocal::Float(execution::FloatLocalId(index)),
        StoredValueShape::String => execution::ParamLocal::String(execution::StringLocalId(index)),
        StoredValueShape::BitArray => {
            execution::ParamLocal::BitArray(execution::BitArrayLocalId(index))
        }
        StoredValueShape::UtfCodepoint => {
            execution::ParamLocal::UtfCodepoint(execution::UtfCodepointLocalId(index))
        }
        StoredValueShape::Custom(shape) => {
            execution::ParamLocal::Custom(execution::CustomLocal::new(
                execution::CustomLocalId(index),
                context.lower_concrete_custom_shape(shape),
            ))
        }
        StoredValueShape::Bool => execution::ParamLocal::Bool(execution::BoolLocalId(index)),
        StoredValueShape::Nil => execution::ParamLocal::Nil(execution::NilLocalId(index)),
        StoredValueShape::Tuple(elements) => execution::ParamLocal::Tuple {
            local: execution::TupleLocalId(index),
            type_: elements
                .iter()
                .map(|element| context.lower_concrete_value_type(element))
                .collect(),
        },
        StoredValueShape::List(item) => {
            execution::ParamLocal::List(list_local_at(item, index, context))
        }
        StoredValueShape::Function(function) => {
            function_local_as_param(function_local_at(function, index, context))
        }
    }
}

pub(super) fn list_local_at(
    item: &SpecializedValueShape,
    index: usize,
    context: &mut LoweringContext,
) -> execution::ListLocal {
    match item {
        SpecializedValueShape::Parameter(parameter) => execution::ListLocal::Parameter {
            local: execution::ParameterListLocalId(index),
            type_id: context.parameter_list_type(*parameter),
        },
        SpecializedValueShape::Int => execution::ListLocal::Int {
            local: execution::IntListLocalId(index),
            type_id: context.int_list_type(),
        },
        SpecializedValueShape::String => execution::ListLocal::String {
            local: execution::StringListLocalId(index),
            type_id: context.string_list_type(),
        },
        SpecializedValueShape::BitArray => execution::ListLocal::BitArray {
            local: execution::BitArrayListLocalId(index),
            type_id: context.bit_array_list_type(),
        },
        SpecializedValueShape::UtfCodepoint => execution::ListLocal::UtfCodepoint {
            local: execution::UtfCodepointListLocalId(index),
            type_id: context.utf_codepoint_list_type(),
        },
        SpecializedValueShape::Custom(custom) => execution::ListLocal::Custom {
            local: execution::CustomListLocalId(index),
            type_id: context.specialized_custom_list_type(custom),
        },
        SpecializedValueShape::Float => execution::ListLocal::Float {
            local: execution::FloatListLocalId(index),
            type_id: context.float_list_type(),
        },
        SpecializedValueShape::Bool => execution::ListLocal::Bool {
            local: execution::BoolListLocalId(index),
            type_id: context.bool_list_type(),
        },
        SpecializedValueShape::Nil => execution::ListLocal::Nil {
            local: execution::NilListLocalId(index),
            type_id: context.nil_list_type(),
        },
        SpecializedValueShape::Tuple(elements) => execution::ListLocal::Tuple {
            local: execution::TupleListLocalId(index),
            type_id: context.specialized_tuple_list_type(elements),
        },
        SpecializedValueShape::List(item) => match context.specialized_list_list_type(item) {
            super::value_type::NestedListTypeId::Parameter(type_id) => {
                execution::ListLocal::ParameterList {
                    local: execution::ParameterListListLocalId(index),
                    type_id,
                }
            }
            super::value_type::NestedListTypeId::Stored(type_id) => execution::ListLocal::List {
                local: execution::ListListLocalId(index),
                type_id,
            },
        },
        SpecializedValueShape::Function(function) => execution::ListLocal::Function {
            local: execution::FunctionListLocalId(index),
            type_id: context.specialized_function_list_type(function),
        },
    }
}

pub(super) fn function_local_at(
    shape: &SpecializedFunctionShape,
    index: usize,
    context: &mut LoweringContext,
) -> SpecializedFunctionLocal {
    let type_ = context.lower_concrete_function_type(shape);
    match context.function_representation(shape) {
        FunctionRepresentation::Symbolic => {
            SpecializedFunctionLocal::Generic(execution::GenericFunctionLocal::new(
                execution::GenericFunctionLocalId(index),
                context.generic_function_type(shape),
            ))
        }
        FunctionRepresentation::Never(_) => {
            SpecializedFunctionLocal::Never(execution::NeverFunctionLocal::new(
                execution::NeverFunctionLocalId(index),
                context.generic_function_type(shape),
            ))
        }
        FunctionRepresentation::Executable(StoredValueShape::Int) => {
            SpecializedFunctionLocal::Int {
                local: execution::IntFunctionLocalId(index),
                type_,
            }
        }
        FunctionRepresentation::Executable(StoredValueShape::Float) => {
            SpecializedFunctionLocal::Float {
                local: execution::FloatFunctionLocalId(index),
                type_,
            }
        }
        FunctionRepresentation::Executable(StoredValueShape::String) => {
            SpecializedFunctionLocal::String {
                local: execution::StringFunctionLocalId(index),
                type_,
            }
        }
        FunctionRepresentation::Executable(StoredValueShape::BitArray) => {
            SpecializedFunctionLocal::BitArray {
                local: execution::BitArrayFunctionLocalId(index),
                type_,
            }
        }
        FunctionRepresentation::Executable(StoredValueShape::UtfCodepoint) => {
            SpecializedFunctionLocal::UtfCodepoint {
                local: execution::UtfCodepointFunctionLocalId(index),
                type_,
            }
        }
        FunctionRepresentation::Executable(StoredValueShape::Custom(custom)) => {
            let type_ = context.specialized_custom_function_type(shape.arguments(), &custom);
            SpecializedFunctionLocal::Custom(execution::CustomFunctionLocal::new(
                execution::CustomFunctionLocalId(index),
                type_,
            ))
        }
        FunctionRepresentation::Executable(StoredValueShape::Bool) => {
            SpecializedFunctionLocal::Bool {
                local: execution::BoolFunctionLocalId(index),
                type_,
            }
        }
        FunctionRepresentation::Executable(StoredValueShape::Nil) => {
            SpecializedFunctionLocal::Nil {
                local: execution::NilFunctionLocalId(index),
                type_,
            }
        }
        FunctionRepresentation::Executable(StoredValueShape::Tuple(_)) => {
            SpecializedFunctionLocal::Tuple {
                local: execution::TupleFunctionLocalId(index),
                type_,
            }
        }
        FunctionRepresentation::Executable(StoredValueShape::List(item)) => {
            SpecializedFunctionLocal::List(list_function_local_at(&item, type_, index, context))
        }
        FunctionRepresentation::Executable(StoredValueShape::Function(returned)) => {
            let type_ = context.specialized_function_function_type(shape.arguments(), &returned);
            SpecializedFunctionLocal::Function(execution::FunctionFunctionLocal::new(
                execution::FunctionFunctionLocalId(index),
                type_,
            ))
        }
    }
}

pub(super) fn list_function_local_at(
    item: &SpecializedValueShape,
    type_: execution::FunctionType,
    index: usize,
    context: &mut LoweringContext,
) -> execution::ListFunctionLocal {
    use execution::ListFunctionLocal as L;

    match item {
        SpecializedValueShape::Parameter(parameter) => L::Parameter {
            local: execution::ParameterListFunctionLocalId(index),
            type_,
            list_type: context.parameter_list_type(*parameter),
        },
        SpecializedValueShape::Int => L::Int {
            local: execution::IntListFunctionLocalId(index),
            type_,
            list_type: context.int_list_type(),
        },
        SpecializedValueShape::String => L::String {
            local: execution::StringListFunctionLocalId(index),
            type_,
            list_type: context.string_list_type(),
        },
        SpecializedValueShape::BitArray => L::BitArray {
            local: execution::BitArrayListFunctionLocalId(index),
            type_,
            list_type: context.bit_array_list_type(),
        },
        SpecializedValueShape::UtfCodepoint => L::UtfCodepoint {
            local: execution::UtfCodepointListFunctionLocalId(index),
            type_,
            list_type: context.utf_codepoint_list_type(),
        },
        SpecializedValueShape::Custom(custom) => L::Custom {
            local: execution::CustomListFunctionLocalId(index),
            type_,
            list_type: context.specialized_custom_list_type(custom),
        },
        SpecializedValueShape::Float => L::Float {
            local: execution::FloatListFunctionLocalId(index),
            type_,
            list_type: context.float_list_type(),
        },
        SpecializedValueShape::Bool => L::Bool {
            local: execution::BoolListFunctionLocalId(index),
            type_,
            list_type: context.bool_list_type(),
        },
        SpecializedValueShape::Nil => L::Nil {
            local: execution::NilListFunctionLocalId(index),
            type_,
            list_type: context.nil_list_type(),
        },
        SpecializedValueShape::Tuple(elements) => L::Tuple {
            local: execution::TupleListFunctionLocalId(index),
            type_,
            list_type: context.specialized_tuple_list_type(elements),
        },
        SpecializedValueShape::List(item) => match context.specialized_list_list_type(item) {
            super::value_type::NestedListTypeId::Parameter(list_type) => L::ParameterList {
                local: execution::ParameterListListFunctionLocalId(index),
                type_,
                list_type,
            },
            super::value_type::NestedListTypeId::Stored(list_type) => L::List {
                local: execution::ListListFunctionLocalId(index),
                type_,
                list_type,
            },
        },
        SpecializedValueShape::Function(function) => L::Function {
            local: execution::FunctionListFunctionLocalId(index),
            type_,
            list_type: context.specialized_function_list_type(function),
        },
    }
}

pub(super) fn function_local_as_param(local: SpecializedFunctionLocal) -> execution::ParamLocal {
    match local {
        SpecializedFunctionLocal::Generic(local) => execution::ParamLocal::GenericFunction(local),
        SpecializedFunctionLocal::Never(local) => execution::ParamLocal::NeverFunction(local),
        SpecializedFunctionLocal::Int { local, type_ } => {
            execution::ParamLocal::IntFunction { local, type_ }
        }
        SpecializedFunctionLocal::Float { local, type_ } => {
            execution::ParamLocal::FloatFunction { local, type_ }
        }
        SpecializedFunctionLocal::String { local, type_ } => {
            execution::ParamLocal::StringFunction { local, type_ }
        }
        SpecializedFunctionLocal::BitArray { local, type_ } => {
            execution::ParamLocal::BitArrayFunction { local, type_ }
        }
        SpecializedFunctionLocal::UtfCodepoint { local, type_ } => {
            execution::ParamLocal::UtfCodepointFunction { local, type_ }
        }
        SpecializedFunctionLocal::Custom(local) => execution::ParamLocal::CustomFunction(local),
        SpecializedFunctionLocal::Bool { local, type_ } => {
            execution::ParamLocal::BoolFunction { local, type_ }
        }
        SpecializedFunctionLocal::Nil { local, type_ } => {
            execution::ParamLocal::NilFunction { local, type_ }
        }
        SpecializedFunctionLocal::Tuple { local, type_ } => {
            execution::ParamLocal::TupleFunction { local, type_ }
        }
        SpecializedFunctionLocal::List(local) => execution::ParamLocal::ListFunction(local),
        SpecializedFunctionLocal::Function(local) => execution::ParamLocal::FunctionFunction(local),
    }
}
