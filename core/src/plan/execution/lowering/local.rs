use super::LoweringContext;
use super::specialization::{
    FunctionRepresentation, SpecializedFunctionShape, SpecializedTypeSubstitution,
    SpecializedValueShape, StoredValueShape,
};
use crate::plan::{execution, module};
use std::collections::HashMap;

#[derive(Clone)]
pub(super) enum SpecializedFunctionLocal {
    Generic(execution::graph::GenericFunctionLocal),
    Never(execution::graph::NeverFunctionLocal),
    Int {
        local: execution::graph::IntFunctionLocalId,
        type_: execution::type_::FunctionType,
    },
    Float {
        local: execution::graph::FloatFunctionLocalId,
        type_: execution::type_::FunctionType,
    },
    String {
        local: execution::graph::StringFunctionLocalId,
        type_: execution::type_::FunctionType,
    },
    BitArray {
        local: execution::graph::BitArrayFunctionLocalId,
        type_: execution::type_::FunctionType,
    },
    UtfCodepoint {
        local: execution::graph::UtfCodepointFunctionLocalId,
        type_: execution::type_::FunctionType,
    },
    Custom(execution::graph::CustomFunctionLocal),
    External(execution::graph::ExternalFunctionLocal),
    Bool {
        local: execution::graph::BoolFunctionLocalId,
        type_: execution::type_::FunctionType,
    },
    Nil {
        local: execution::graph::NilFunctionLocalId,
        type_: execution::type_::FunctionType,
    },
    Tuple {
        local: execution::graph::TupleFunctionLocalId,
        type_: execution::type_::FunctionType,
    },
    List(execution::graph::ListFunctionLocal),
    Function(execution::graph::FunctionFunctionLocal),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum LocalKind {
    Generic,
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Custom,
    External,
    Bool,
    Nil,
    Tuple,
    GenericList,
    IntList,
    StringList,
    BitArrayList,
    UtfCodepointList,
    CustomList,
    ExternalList,
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
    ExternalFunction,
    BoolFunction,
    NilFunction,
    TupleFunction,
    GenericListFunction,
    IntListFunction,
    StringListFunction,
    BitArrayListFunction,
    UtfCodepointListFunction,
    CustomListFunction,
    ExternalListFunction,
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
    External,
    Bool,
    Nil,
    Tuple,
    ParameterList,
    IntList,
    StringList,
    BitArrayList,
    UtfCodepointList,
    CustomList,
    ExternalList,
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
    ExternalFunction,
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
    ExternalListFunction,
    FloatListFunction,
    BoolListFunction,
    NilListFunction,
    TupleListFunction,
    ParameterListListFunction,
    ListListFunction,
    FunctionListFunction,
    FunctionFunction,
    ExternalFunctionFunction,
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

    pub(super) fn from_shapes(params: Vec<crate::plan::ValueShape>) -> Self {
        Self {
            params: params.into_boxed_slice(),
            captures: Vec::new().into_boxed_slice(),
        }
    }

    pub(super) fn parameter_shapes(&self) -> &[crate::plan::ValueShape] {
        &self.params
    }

    pub(super) fn stored_parameters(
        &self,
        substitution: &SpecializedTypeSubstitution,
        representations: &super::specialization::RepresentationContext,
    ) -> super::specialization::Representability<Box<[StoredValueShape]>> {
        super::specialization::Representability::collect(self.params.iter().map(|shape| {
            let shape = SpecializedValueShape::instantiate(shape, substitution);
            representations.inhabitation(&shape).into_representability()
        }))
        .map(Vec::into_boxed_slice)
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
            StoredValueShape::External(_) => Self::External,
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
            SpecializedValueShape::External(_) => Self::ExternalList,
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
                StoredValueShape::External(_) => Self::ExternalFunction,
                StoredValueShape::Bool => Self::BoolFunction,
                StoredValueShape::Nil => Self::NilFunction,
                StoredValueShape::Tuple(_) => Self::TupleFunction,
                StoredValueShape::List(item) => Self::list_function(&item),
                StoredValueShape::Function(returned) => {
                    if is_external_function_function(&returned, representations) {
                        Self::ExternalFunctionFunction
                    } else {
                        Self::FunctionFunction
                    }
                }
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
            SpecializedValueShape::External(_) => Self::ExternalListFunction,
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
        module::ParamLocal::External(local) => LocalKey::new(LocalKind::External, local.id().0),
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
        module::ParamLocal::ExternalFunction(local) => {
            LocalKey::new(LocalKind::ExternalFunction, local.id().0)
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
        module::ListLocal::External { local, .. } => {
            LocalKey::new(LocalKind::ExternalList, local.0)
        }
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
        module::ListFunctionLocal::External { local, type_, .. } => {
            (LocalKind::ExternalListFunction, local.0, type_.clone())
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
) -> execution::graph::ParamLocal {
    match shape {
        StoredValueShape::Int => {
            execution::graph::ParamLocal::Int(execution::graph::IntLocalId(index))
        }
        StoredValueShape::Float => {
            execution::graph::ParamLocal::Float(execution::graph::FloatLocalId(index))
        }
        StoredValueShape::String => {
            execution::graph::ParamLocal::String(execution::graph::StringLocalId(index))
        }
        StoredValueShape::BitArray => {
            execution::graph::ParamLocal::BitArray(execution::graph::BitArrayLocalId(index))
        }
        StoredValueShape::UtfCodepoint => {
            execution::graph::ParamLocal::UtfCodepoint(execution::graph::UtfCodepointLocalId(index))
        }
        StoredValueShape::Custom(shape) => {
            execution::graph::ParamLocal::Custom(execution::graph::CustomLocal::new(
                execution::graph::CustomLocalId(index),
                context.lower_concrete_custom_shape(shape),
            ))
        }
        StoredValueShape::External(shape) => {
            execution::graph::ParamLocal::External(execution::graph::ExternalLocal::new(
                execution::graph::ExternalLocalId(index),
                context.lower_concrete_external_type(shape),
            ))
        }
        StoredValueShape::Bool => {
            execution::graph::ParamLocal::Bool(execution::graph::BoolLocalId(index))
        }
        StoredValueShape::Nil => {
            execution::graph::ParamLocal::Nil(execution::graph::NilLocalId(index))
        }
        StoredValueShape::Tuple(elements) => execution::graph::ParamLocal::Tuple {
            local: execution::graph::TupleLocalId(index),
            type_: elements
                .iter()
                .map(|element| context.lower_concrete_value_type(element))
                .collect(),
        },
        StoredValueShape::List(item) => {
            execution::graph::ParamLocal::List(list_local_at(item, index, context))
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
) -> execution::graph::ListLocal {
    match item {
        SpecializedValueShape::Parameter(parameter) => execution::graph::ListLocal::Parameter {
            local: execution::graph::ParameterListLocalId(index),
            type_id: context.parameter_list_type(*parameter),
        },
        SpecializedValueShape::Int => execution::graph::ListLocal::Int {
            local: execution::graph::IntListLocalId(index),
            type_id: context.int_list_type(),
        },
        SpecializedValueShape::String => execution::graph::ListLocal::String {
            local: execution::graph::StringListLocalId(index),
            type_id: context.string_list_type(),
        },
        SpecializedValueShape::BitArray => execution::graph::ListLocal::BitArray {
            local: execution::graph::BitArrayListLocalId(index),
            type_id: context.bit_array_list_type(),
        },
        SpecializedValueShape::UtfCodepoint => execution::graph::ListLocal::UtfCodepoint {
            local: execution::graph::UtfCodepointListLocalId(index),
            type_id: context.utf_codepoint_list_type(),
        },
        SpecializedValueShape::Custom(custom) => execution::graph::ListLocal::Custom {
            local: execution::graph::CustomListLocalId(index),
            type_id: context.specialized_custom_list_type(custom),
        },
        SpecializedValueShape::External(external) => execution::graph::ListLocal::External {
            local: execution::graph::ExternalListLocalId(index),
            type_id: context.specialized_external_list_type(external),
        },
        SpecializedValueShape::Float => execution::graph::ListLocal::Float {
            local: execution::graph::FloatListLocalId(index),
            type_id: context.float_list_type(),
        },
        SpecializedValueShape::Bool => execution::graph::ListLocal::Bool {
            local: execution::graph::BoolListLocalId(index),
            type_id: context.bool_list_type(),
        },
        SpecializedValueShape::Nil => execution::graph::ListLocal::Nil {
            local: execution::graph::NilListLocalId(index),
            type_id: context.nil_list_type(),
        },
        SpecializedValueShape::Tuple(elements) => execution::graph::ListLocal::Tuple {
            local: execution::graph::TupleListLocalId(index),
            type_id: context.specialized_tuple_list_type(elements),
        },
        SpecializedValueShape::List(item) => match context.specialized_list_list_type(item) {
            super::value_type::NestedListTypeId::Parameter(type_id) => {
                execution::graph::ListLocal::ParameterList {
                    local: execution::graph::ParameterListListLocalId(index),
                    type_id,
                }
            }
            super::value_type::NestedListTypeId::Stored(type_id) => {
                execution::graph::ListLocal::List {
                    local: execution::graph::ListListLocalId(index),
                    type_id,
                }
            }
        },
        SpecializedValueShape::Function(function) => execution::graph::ListLocal::Function {
            local: execution::graph::FunctionListLocalId(index),
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
            SpecializedFunctionLocal::Generic(execution::graph::GenericFunctionLocal::new(
                execution::graph::GenericFunctionLocalId(index),
                context.generic_function_type(shape),
            ))
        }
        FunctionRepresentation::Never(_) => {
            SpecializedFunctionLocal::Never(execution::graph::NeverFunctionLocal::new(
                execution::graph::NeverFunctionLocalId(index),
                context.generic_function_type(shape),
            ))
        }
        FunctionRepresentation::Executable(StoredValueShape::Int) => {
            SpecializedFunctionLocal::Int {
                local: execution::graph::IntFunctionLocalId(index),
                type_,
            }
        }
        FunctionRepresentation::Executable(StoredValueShape::Float) => {
            SpecializedFunctionLocal::Float {
                local: execution::graph::FloatFunctionLocalId(index),
                type_,
            }
        }
        FunctionRepresentation::Executable(StoredValueShape::String) => {
            SpecializedFunctionLocal::String {
                local: execution::graph::StringFunctionLocalId(index),
                type_,
            }
        }
        FunctionRepresentation::Executable(StoredValueShape::BitArray) => {
            SpecializedFunctionLocal::BitArray {
                local: execution::graph::BitArrayFunctionLocalId(index),
                type_,
            }
        }
        FunctionRepresentation::Executable(StoredValueShape::UtfCodepoint) => {
            SpecializedFunctionLocal::UtfCodepoint {
                local: execution::graph::UtfCodepointFunctionLocalId(index),
                type_,
            }
        }
        FunctionRepresentation::Executable(StoredValueShape::Custom(custom)) => {
            let type_ = context.specialized_custom_function_type(shape.arguments(), &custom);
            SpecializedFunctionLocal::Custom(execution::graph::CustomFunctionLocal::new(
                execution::graph::CustomFunctionLocalId(index),
                type_,
            ))
        }
        FunctionRepresentation::Executable(StoredValueShape::External(external)) => {
            let type_ = context.specialized_external_function_type(shape.arguments(), &external);
            SpecializedFunctionLocal::External(execution::graph::ExternalFunctionLocal::new(
                execution::graph::ExternalFunctionLocalId(index),
                type_,
            ))
        }
        FunctionRepresentation::Executable(StoredValueShape::Bool) => {
            SpecializedFunctionLocal::Bool {
                local: execution::graph::BoolFunctionLocalId(index),
                type_,
            }
        }
        FunctionRepresentation::Executable(StoredValueShape::Nil) => {
            SpecializedFunctionLocal::Nil {
                local: execution::graph::NilFunctionLocalId(index),
                type_,
            }
        }
        FunctionRepresentation::Executable(StoredValueShape::Tuple(_)) => {
            SpecializedFunctionLocal::Tuple {
                local: execution::graph::TupleFunctionLocalId(index),
                type_,
            }
        }
        FunctionRepresentation::Executable(StoredValueShape::List(item)) => {
            SpecializedFunctionLocal::List(list_function_local_at(&item, type_, index, context))
        }
        FunctionRepresentation::Executable(StoredValueShape::Function(returned)) => {
            let type_ = context.specialized_function_function_type(shape.arguments(), &returned);
            let local = if is_external_function_function(&returned, &context.representations) {
                execution::graph::FunctionFunctionLocal::External(
                    execution::graph::ExternalFunctionFunctionLocal::new(
                        execution::graph::ExternalFunctionFunctionLocalId(index),
                        type_,
                    ),
                )
            } else {
                execution::graph::FunctionFunctionLocal::Core(
                    execution::graph::CoreFunctionFunctionLocal::new(
                        execution::graph::CoreFunctionFunctionLocalId(index),
                        type_,
                    ),
                )
            };
            SpecializedFunctionLocal::Function(local)
        }
    }
}

fn is_external_function_function(
    returned: &SpecializedFunctionShape,
    representations: &super::specialization::RepresentationContext,
) -> bool {
    match returned.representation(representations) {
        FunctionRepresentation::Executable(StoredValueShape::External(_)) => true,
        FunctionRepresentation::Executable(StoredValueShape::List(item)) => {
            matches!(item.as_ref(), SpecializedValueShape::External(_))
        }
        FunctionRepresentation::Symbolic
        | FunctionRepresentation::Never(_)
        | FunctionRepresentation::Executable(_) => false,
    }
}

pub(super) fn list_function_local_at(
    item: &SpecializedValueShape,
    type_: execution::type_::FunctionType,
    index: usize,
    context: &mut LoweringContext,
) -> execution::graph::ListFunctionLocal {
    use execution::graph::ListFunctionLocal as L;

    match item {
        SpecializedValueShape::Parameter(parameter) => L::Parameter {
            local: execution::graph::ParameterListFunctionLocalId(index),
            type_,
            list_type: context.parameter_list_type(*parameter),
        },
        SpecializedValueShape::Int => L::Int {
            local: execution::graph::IntListFunctionLocalId(index),
            type_,
            list_type: context.int_list_type(),
        },
        SpecializedValueShape::String => L::String {
            local: execution::graph::StringListFunctionLocalId(index),
            type_,
            list_type: context.string_list_type(),
        },
        SpecializedValueShape::BitArray => L::BitArray {
            local: execution::graph::BitArrayListFunctionLocalId(index),
            type_,
            list_type: context.bit_array_list_type(),
        },
        SpecializedValueShape::UtfCodepoint => L::UtfCodepoint {
            local: execution::graph::UtfCodepointListFunctionLocalId(index),
            type_,
            list_type: context.utf_codepoint_list_type(),
        },
        SpecializedValueShape::Custom(custom) => L::Custom {
            local: execution::graph::CustomListFunctionLocalId(index),
            type_,
            list_type: context.specialized_custom_list_type(custom),
        },
        SpecializedValueShape::External(external) => L::External {
            local: execution::graph::ExternalListFunctionLocalId(index),
            type_,
            list_type: context.specialized_external_list_type(external),
        },
        SpecializedValueShape::Float => L::Float {
            local: execution::graph::FloatListFunctionLocalId(index),
            type_,
            list_type: context.float_list_type(),
        },
        SpecializedValueShape::Bool => L::Bool {
            local: execution::graph::BoolListFunctionLocalId(index),
            type_,
            list_type: context.bool_list_type(),
        },
        SpecializedValueShape::Nil => L::Nil {
            local: execution::graph::NilListFunctionLocalId(index),
            type_,
            list_type: context.nil_list_type(),
        },
        SpecializedValueShape::Tuple(elements) => L::Tuple {
            local: execution::graph::TupleListFunctionLocalId(index),
            type_,
            list_type: context.specialized_tuple_list_type(elements),
        },
        SpecializedValueShape::List(item) => match context.specialized_list_list_type(item) {
            super::value_type::NestedListTypeId::Parameter(list_type) => L::ParameterList {
                local: execution::graph::ParameterListListFunctionLocalId(index),
                type_,
                list_type,
            },
            super::value_type::NestedListTypeId::Stored(list_type) => L::List {
                local: execution::graph::ListListFunctionLocalId(index),
                type_,
                list_type,
            },
        },
        SpecializedValueShape::Function(function) => L::Function {
            local: execution::graph::FunctionListFunctionLocalId(index),
            type_,
            list_type: context.specialized_function_list_type(function),
        },
    }
}

pub(super) fn function_local_as_param(
    local: SpecializedFunctionLocal,
) -> execution::graph::ParamLocal {
    match local {
        SpecializedFunctionLocal::Generic(local) => {
            execution::graph::ParamLocal::GenericFunction(local)
        }
        SpecializedFunctionLocal::Never(local) => {
            execution::graph::ParamLocal::NeverFunction(local)
        }
        SpecializedFunctionLocal::Int { local, type_ } => {
            execution::graph::ParamLocal::IntFunction { local, type_ }
        }
        SpecializedFunctionLocal::Float { local, type_ } => {
            execution::graph::ParamLocal::FloatFunction { local, type_ }
        }
        SpecializedFunctionLocal::String { local, type_ } => {
            execution::graph::ParamLocal::StringFunction { local, type_ }
        }
        SpecializedFunctionLocal::BitArray { local, type_ } => {
            execution::graph::ParamLocal::BitArrayFunction { local, type_ }
        }
        SpecializedFunctionLocal::UtfCodepoint { local, type_ } => {
            execution::graph::ParamLocal::UtfCodepointFunction { local, type_ }
        }
        SpecializedFunctionLocal::Custom(local) => {
            execution::graph::ParamLocal::CustomFunction(local)
        }
        SpecializedFunctionLocal::External(local) => {
            execution::graph::ParamLocal::ExternalFunction(local)
        }
        SpecializedFunctionLocal::Bool { local, type_ } => {
            execution::graph::ParamLocal::BoolFunction { local, type_ }
        }
        SpecializedFunctionLocal::Nil { local, type_ } => {
            execution::graph::ParamLocal::NilFunction { local, type_ }
        }
        SpecializedFunctionLocal::Tuple { local, type_ } => {
            execution::graph::ParamLocal::TupleFunction { local, type_ }
        }
        SpecializedFunctionLocal::List(local) => execution::graph::ParamLocal::ListFunction(local),
        SpecializedFunctionLocal::Function(local) => {
            execution::graph::ParamLocal::FunctionFunction(local)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::specialization::{
        SpecializedFunctionShape, SpecializedTypeSubstitution, SpecializedValueShape,
        StoredValueShape,
    };
    use super::{
        FunctionEntryTemplate, LocalKey, LocalKind, ParameterPrefix, function_local_as_param,
        function_local_at, param_local_key, stored_value_local_at,
    };
    use crate::plan::execution::{graph, type_ as execution_type};
    use crate::plan::{
        BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId, BoolFunctionLocalId,
        BoolListLocalId, BoolLocalId, CustomConstructorDefinition, CustomFunctionLocal,
        CustomFunctionLocalId, CustomFunctionType, CustomListLocalId, CustomLocalId, CustomType,
        CustomTypeDefinition, CustomTypeName, CustomTypePublicity, CustomValueShape,
        ExternalFunctionLocal, ExternalFunctionLocalId, ExternalFunctionType, ExternalListLocalId,
        ExternalLocalId, ExternalType, ExternalTypeName, ExternalValueShape, FloatFunctionLocalId,
        FloatListLocalId, FloatLocalId, FunctionFunctionLocal, FunctionFunctionLocalId,
        FunctionFunctionType, FunctionListLocalId, FunctionType, GenericFunctionLocal,
        GenericFunctionLocalId, GenericFunctionType, GenericListLocalId, GenericLocal,
        GenericLocalId, IntFunctionLocalId, IntListLocalId, IntLocalId, ListFunctionLocal,
        ListListLocalId, ListLocal, NilFunctionLocalId, NilListLocalId, NilLocalId, ParamLocal,
        StringFunctionLocalId, StringListLocalId, StringLocalId, TupleFunctionLocalId,
        TupleListLocalId, TupleLocalId, TypeParameterId, TypeScheme, UtfCodepointFunctionLocalId,
        UtfCodepointListLocalId, UtfCodepointLocalId, ValueShape, ValueType,
    };

    #[test]
    fn source_parameter_locals_map_to_distinct_lowering_keys() {
        let index = 7;
        let parameter = TypeParameterId(0);
        let custom = custom_type();
        let external = external_type();
        let function = FunctionType::new(vec![ValueType::Int], ValueType::String);
        let custom_function = CustomFunctionType::new(vec![ValueType::Int], custom.clone());
        let external_shape = ExternalValueShape::new(
            external.type_name().clone(),
            external
                .arguments()
                .iter()
                .cloned()
                .map(ValueShape::from_value_type)
                .collect(),
        );
        let external_function =
            ExternalFunctionType::from_shapes(vec![ValueShape::Int], external_shape.clone());
        let function_function = FunctionFunctionType::new(vec![ValueType::Int], function.clone());
        let generic_function = GenericFunctionType::new(vec![ValueShape::Int], parameter);

        let list_locals = vec![
            ListLocal::generic(GenericListLocalId(index), parameter),
            ListLocal::int(IntListLocalId(index)),
            ListLocal::string(StringListLocalId(index)),
            ListLocal::bit_array(BitArrayListLocalId(index)),
            ListLocal::utf_codepoint(UtfCodepointListLocalId(index)),
            ListLocal::custom(CustomListLocalId(index), custom.clone()),
            ListLocal::external(ExternalListLocalId(index), external.clone()),
            ListLocal::float(FloatListLocalId(index)),
            ListLocal::bool(BoolListLocalId(index)),
            ListLocal::nil(NilListLocalId(index)),
            ListLocal::tuple(TupleListLocalId(index), vec![ValueType::Int]),
            ListLocal::list(ListListLocalId(index), ValueType::Int),
            ListLocal::function(FunctionListLocalId(index), function.clone()),
        ];
        let list_function_locals = [
            ValueType::Parameter(parameter),
            ValueType::Int,
            ValueType::String,
            ValueType::BitArray,
            ValueType::UtfCodepoint,
            ValueType::Custom(custom.clone()),
            ValueType::External(external.clone()),
            ValueType::Float,
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int]),
            ValueType::List(Box::new(ValueType::Int)),
            ValueType::Function(Box::new(function.clone())),
        ]
        .into_iter()
        .map(|item| ListFunctionLocal::from_item_type(index, function.clone(), item));

        let mut locals = vec![
            ParamLocal::generic(GenericLocal::new(GenericLocalId(index), parameter)),
            ParamLocal::int(IntLocalId(index)),
            ParamLocal::float(FloatLocalId(index)),
            ParamLocal::string(StringLocalId(index)),
            ParamLocal::bit_array(BitArrayLocalId(index)),
            ParamLocal::utf_codepoint(UtfCodepointLocalId(index)),
            ParamLocal::custom(CustomLocalId(index), custom),
            ParamLocal::external_shape(ExternalLocalId(index), external_shape),
            ParamLocal::bool(BoolLocalId(index)),
            ParamLocal::nil(NilLocalId(index)),
            ParamLocal::tuple(TupleLocalId(index), vec![ValueType::Int]),
        ];
        locals.extend(list_locals.into_iter().map(ParamLocal::list));
        locals.extend([
            ParamLocal::int_function(IntFunctionLocalId(index), function.clone()),
            ParamLocal::float_function(FloatFunctionLocalId(index), function.clone()),
            ParamLocal::string_function(StringFunctionLocalId(index), function.clone()),
            ParamLocal::bit_array_function(BitArrayFunctionLocalId(index), function.clone()),
            ParamLocal::utf_codepoint_function(
                UtfCodepointFunctionLocalId(index),
                function.clone(),
            ),
            ParamLocal::custom_function(CustomFunctionLocal::new(
                CustomFunctionLocalId(index),
                custom_function,
            )),
            ParamLocal::external_function(ExternalFunctionLocal::new(
                ExternalFunctionLocalId(index),
                external_function,
            )),
            ParamLocal::bool_function(BoolFunctionLocalId(index), function.clone()),
            ParamLocal::nil_function(NilFunctionLocalId(index), function.clone()),
            ParamLocal::tuple_function(TupleFunctionLocalId(index), function.clone()),
        ]);
        locals.extend(list_function_locals.map(ParamLocal::list_function));
        locals.extend([
            ParamLocal::function_function(FunctionFunctionLocal::new(
                FunctionFunctionLocalId(index),
                function_function,
            )),
            ParamLocal::generic_function(GenericFunctionLocal::new(
                GenericFunctionLocalId(index),
                generic_function,
            )),
        ]);

        let expected_kinds = [
            LocalKind::Generic,
            LocalKind::Int,
            LocalKind::Float,
            LocalKind::String,
            LocalKind::BitArray,
            LocalKind::UtfCodepoint,
            LocalKind::Custom,
            LocalKind::External,
            LocalKind::Bool,
            LocalKind::Nil,
            LocalKind::Tuple,
            LocalKind::GenericList,
            LocalKind::IntList,
            LocalKind::StringList,
            LocalKind::BitArrayList,
            LocalKind::UtfCodepointList,
            LocalKind::CustomList,
            LocalKind::ExternalList,
            LocalKind::FloatList,
            LocalKind::BoolList,
            LocalKind::NilList,
            LocalKind::TupleList,
            LocalKind::ListList,
            LocalKind::FunctionList,
            LocalKind::IntFunction,
            LocalKind::FloatFunction,
            LocalKind::StringFunction,
            LocalKind::BitArrayFunction,
            LocalKind::UtfCodepointFunction,
            LocalKind::CustomFunction,
            LocalKind::ExternalFunction,
            LocalKind::BoolFunction,
            LocalKind::NilFunction,
            LocalKind::TupleFunction,
            LocalKind::GenericListFunction,
            LocalKind::IntListFunction,
            LocalKind::StringListFunction,
            LocalKind::BitArrayListFunction,
            LocalKind::UtfCodepointListFunction,
            LocalKind::CustomListFunction,
            LocalKind::ExternalListFunction,
            LocalKind::FloatListFunction,
            LocalKind::BoolListFunction,
            LocalKind::NilListFunction,
            LocalKind::TupleListFunction,
            LocalKind::ListListFunction,
            LocalKind::FunctionListFunction,
            LocalKind::FunctionFunction,
            LocalKind::GenericFunction,
        ];

        assert_eq!(
            locals.iter().map(param_local_key).collect::<Vec<_>>(),
            expected_kinds
                .into_iter()
                .map(|kind| LocalKey::new(kind, index))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn parameter_prefix_counts_each_storage_family_and_capture_in_entry_order() {
        let context = super::super::test_support::lowering_context(Vec::new());
        let mut prefix = ParameterPrefix::default();

        assert_eq!(
            prefix.allocate_stored(StoredValueShape::Int, &context.representations),
            (0, StoredValueShape::Int)
        );
        assert_eq!(
            prefix.allocate_stored(StoredValueShape::String, &context.representations),
            (0, StoredValueShape::String)
        );
        assert_eq!(
            prefix.allocate_stored(StoredValueShape::Int, &context.representations),
            (1, StoredValueShape::Int)
        );
        assert_eq!(
            prefix.allocate_stored(
                StoredValueShape::List(Box::new(SpecializedValueShape::Int)),
                &context.representations,
            ),
            (
                0,
                StoredValueShape::List(Box::new(SpecializedValueShape::Int))
            )
        );

        let substitution = TypeScheme::new(1)
            .try_substitution(vec![ValueShape::Int])
            .expect("one type argument should match the capture template");
        let substitution = SpecializedTypeSubstitution::instantiate(
            &substitution,
            &SpecializedTypeSubstitution::empty(),
        );
        let entry = context
            .entry_templates
            .get(&crate::plan::FunctionTemplateId::new(1))
            .cloned()
            .expect("the lowering fixture should contain the capture target");

        assert_eq!(
            entry.stored_parameters(&substitution, &context.representations),
            super::super::specialization::Representability::Inhabited(
                vec![StoredValueShape::Int].into_boxed_slice()
            )
        );
        assert_eq!(
            entry.capture_target(
                crate::plan::CapturePosition::new(1),
                StoredValueShape::Int,
                &substitution,
                &context.representations,
            ),
            (2, StoredValueShape::Int)
        );

        let direct = FunctionEntryTemplate::from_shapes(vec![
            ValueShape::Int,
            ValueShape::String,
            ValueShape::Int,
        ]);
        assert_eq!(
            direct.stored_parameters(
                &SpecializedTypeSubstitution::empty(),
                &context.representations,
            ),
            super::super::specialization::Representability::Inhabited(
                vec![
                    StoredValueShape::Int,
                    StoredValueShape::String,
                    StoredValueShape::Int,
                ]
                .into_boxed_slice()
            )
        );
    }

    #[test]
    fn specialized_shapes_lower_to_exact_execution_local_families() {
        let custom_name = CustomTypeName::new("geam".into(), "main".into(), "Boxed".into());
        let custom = CustomType::new(custom_name.clone(), Vec::new());
        let custom_definition = CustomTypeDefinition::new(
            custom_name,
            CustomTypePublicity::Private,
            false,
            Vec::new(),
            vec![CustomConstructorDefinition::new(
                "Boxed".into(),
                0,
                Vec::new(),
            )],
        );
        let external = external_type();
        let mut context = super::super::test_support::lowering_context(vec![custom_definition]);

        assert_eq!(
            stored_value_local_at(&StoredValueShape::Int, 3, &mut context),
            graph::ParamLocal::Int(graph::IntLocalId(3))
        );
        assert_eq!(
            stored_value_local_at(
                &StoredValueShape::Tuple(
                    vec![SpecializedValueShape::Int, SpecializedValueShape::String]
                        .into_boxed_slice(),
                ),
                4,
                &mut context,
            ),
            graph::ParamLocal::Tuple {
                local: graph::TupleLocalId(4),
                type_: vec![
                    execution_type::ValueType::Int,
                    execution_type::ValueType::String
                ],
            }
        );

        for (index, item) in [
            SpecializedValueShape::Int,
            SpecializedValueShape::Parameter(TypeParameterId(0)),
        ]
        .into_iter()
        .enumerate()
        {
            let index = index + 5;
            let expected = match context.specialized_list_list_type(&item) {
                super::super::value_type::NestedListTypeId::Parameter(type_id) => {
                    graph::ListLocal::ParameterList {
                        local: graph::ParameterListListLocalId(index),
                        type_id,
                    }
                }
                super::super::value_type::NestedListTypeId::Stored(type_id) => {
                    graph::ListLocal::List {
                        local: graph::ListListLocalId(index),
                        type_id,
                    }
                }
            };
            assert_eq!(
                stored_value_local_at(
                    &StoredValueShape::List(Box::new(SpecializedValueShape::List(Box::new(item,)))),
                    index,
                    &mut context,
                ),
                graph::ParamLocal::List(expected)
            );
        }

        let custom_shape = context.concrete_custom_value_shape(&CustomValueShape::any(custom));
        let expected_custom = context.lower_concrete_custom_shape(&custom_shape);
        assert_eq!(
            stored_value_local_at(&StoredValueShape::Custom(custom_shape), 6, &mut context,),
            graph::ParamLocal::Custom(graph::CustomLocal::new(
                graph::CustomLocalId(6),
                expected_custom,
            ))
        );

        let external_shape = context.concrete_external_value_shape(&ExternalValueShape::new(
            external.type_name().clone(),
            Vec::new(),
        ));
        let expected_external = context.lower_concrete_external_type(&external_shape);
        assert_eq!(
            stored_value_local_at(
                &StoredValueShape::External(external_shape.clone()),
                7,
                &mut context,
            ),
            graph::ParamLocal::External(graph::ExternalLocal::new(
                graph::ExternalLocalId(7),
                expected_external,
            ))
        );

        let symbolic = SpecializedFunctionShape::new(
            vec![SpecializedValueShape::Parameter(TypeParameterId(0))],
            SpecializedValueShape::Int,
        );
        let symbolic_type = context.generic_function_type(&symbolic);
        assert_eq!(
            function_local_as_param(function_local_at(&symbolic, 8, &mut context)),
            graph::ParamLocal::GenericFunction(graph::GenericFunctionLocal::new(
                graph::GenericFunctionLocalId(8),
                symbolic_type,
            ))
        );

        let never = SpecializedFunctionShape::new(
            vec![SpecializedValueShape::Int],
            SpecializedValueShape::Parameter(TypeParameterId(0)),
        );
        let never_type = context.generic_function_type(&never);
        assert_eq!(
            function_local_as_param(function_local_at(&never, 9, &mut context)),
            graph::ParamLocal::NeverFunction(graph::NeverFunctionLocal::new(
                graph::NeverFunctionLocalId(9),
                never_type,
            ))
        );

        let scalar = SpecializedFunctionShape::new(
            vec![SpecializedValueShape::Int],
            SpecializedValueShape::String,
        );
        let scalar_type = context.lower_concrete_function_type(&scalar);
        assert_eq!(
            function_local_as_param(function_local_at(&scalar, 10, &mut context)),
            graph::ParamLocal::StringFunction {
                local: graph::StringFunctionLocalId(10),
                type_: scalar_type,
            }
        );

        let returning_external = SpecializedFunctionShape::new(
            Vec::new(),
            SpecializedValueShape::External(external_shape),
        );
        let callable = SpecializedFunctionShape::new(
            vec![SpecializedValueShape::Int],
            SpecializedValueShape::Function(Box::new(returning_external.clone())),
        );
        let callable_type =
            context.specialized_function_function_type(callable.arguments(), &returning_external);
        assert_eq!(
            function_local_as_param(function_local_at(&callable, 11, &mut context)),
            graph::ParamLocal::FunctionFunction(graph::FunctionFunctionLocal::External(
                graph::ExternalFunctionFunctionLocal::new(
                    graph::ExternalFunctionFunctionLocalId(11),
                    callable_type,
                ),
            ))
        );
    }

    fn custom_type() -> CustomType {
        CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Custom".into()),
            Vec::new(),
        )
    }

    fn external_type() -> ExternalType {
        ExternalType::new(
            ExternalTypeName::new("geam".into(), "main".into(), "External".into()),
            Vec::new(),
        )
    }
}
