use super::specialization::{
    FunctionRepresentation, SpecializedFunctionShape, SpecializedValueShape, StoredValueShape,
};
use super::{LoweringContext, SpecializedFunctionLocal};
use crate::plan::{execution, module};
use std::collections::{HashMap, HashSet};

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
pub(super) struct LocalAllocationTemplate {
    entries: Box<[(LocalKey, crate::plan::ValueShape)]>,
}

#[derive(Clone)]
pub(super) struct LocalAllocationPlan {
    stored_shapes: Box<[StoredValueShape]>,
    allocations: HashMap<LocalKey, LocalAllocation>,
    uninhabited: HashMap<LocalKey, crate::plan::TypeParameterId>,
}

#[derive(Clone)]
struct LocalAllocation {
    index: usize,
    shape: StoredValueShape,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum AllocatedLocal {
    Stored {
        index: usize,
        shape: StoredValueShape,
    },
    Uninhabited(crate::plan::TypeParameterId),
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
pub(super) struct ParameterPrefix {
    next: HashMap<StorageFamily, usize>,
}

impl LocalAllocationTemplate {
    pub(super) fn new(template: &module::FunctionTemplate) -> Self {
        let mut entries = Vec::new();
        let mut included = HashSet::new();

        for param in template.params() {
            push_template_entry(
                &mut entries,
                &mut included,
                param_local_key(param.local()),
                param.shape().clone(),
            );
        }

        let parts = template.execution_frame_layout().parts();
        push_value_entries(&mut entries, &mut included, &parts);
        push_list_entries(&mut entries, &mut included, &parts);
        push_function_entries(&mut entries, &mut included, &parts);

        Self {
            entries: entries.into_boxed_slice(),
        }
    }

    pub(super) fn specialize(
        &self,
        substitution: &super::specialization::SpecializedTypeSubstitution,
        representations: &super::specialization::RepresentationContext,
    ) -> LocalAllocationPlan {
        let mut next = HashMap::<StorageFamily, usize>::new();
        let mut allocations = HashMap::with_capacity(self.entries.len());
        let mut uninhabited = HashMap::new();
        let mut stored_shapes = Vec::new();

        for (key, shape) in &self.entries {
            let shape = SpecializedValueShape::instantiate(shape, substitution);
            let stored = match representations.representation(&shape) {
                super::specialization::ValueRepresentation::Uninhabited(parameter) => {
                    uninhabited.insert(*key, parameter);
                    continue;
                }
                super::specialization::ValueRepresentation::Stored(stored) => stored,
            };
            let index = next
                .entry(StorageFamily::of(&stored, representations))
                .or_default();
            allocations.insert(
                *key,
                LocalAllocation {
                    index: *index,
                    shape: stored.clone(),
                },
            );
            stored_shapes.push(stored);
            *index += 1;
        }

        LocalAllocationPlan {
            stored_shapes: stored_shapes.into_boxed_slice(),
            allocations,
            uninhabited,
        }
    }
}

impl LocalAllocationPlan {
    pub(super) fn index(&self, key: LocalKey) -> usize {
        self.allocations[&key].index
    }

    pub(super) fn allocation(&self, key: LocalKey) -> AllocatedLocal {
        match self.allocations.get(&key) {
            Some(allocation) => AllocatedLocal::Stored {
                index: allocation.index,
                shape: allocation.shape.clone(),
            },
            None => AllocatedLocal::Uninhabited(self.uninhabited[&key]),
        }
    }

    pub(super) fn stored_allocation(&self, key: LocalKey) -> (usize, StoredValueShape) {
        let allocation = &self.allocations[&key];
        (allocation.index, allocation.shape.clone())
    }

    pub(super) fn stored_shapes(&self) -> &[StoredValueShape] {
        &self.stored_shapes
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
            SpecializedValueShape::Parameter(_) => Self::GenericFunction,
            SpecializedValueShape::Int => Self::IntListFunction,
            SpecializedValueShape::String => Self::StringListFunction,
            SpecializedValueShape::BitArray => Self::BitArrayListFunction,
            SpecializedValueShape::UtfCodepoint => Self::UtfCodepointListFunction,
            SpecializedValueShape::Custom(_) => Self::CustomListFunction,
            SpecializedValueShape::Float => Self::FloatListFunction,
            SpecializedValueShape::Bool => Self::BoolListFunction,
            SpecializedValueShape::Nil => Self::NilListFunction,
            SpecializedValueShape::Tuple(_) => Self::TupleListFunction,
            SpecializedValueShape::List(_) => Self::ListListFunction,
            SpecializedValueShape::Function(_) => Self::FunctionListFunction,
        }
    }
}

fn push_template_entry(
    entries: &mut Vec<(LocalKey, crate::plan::ValueShape)>,
    included: &mut HashSet<LocalKey>,
    key: LocalKey,
    shape: crate::plan::ValueShape,
) {
    if included.insert(key) {
        entries.push((key, shape));
    }
}

fn push_counted_entries(
    entries: &mut Vec<(LocalKey, crate::plan::ValueShape)>,
    included: &mut HashSet<LocalKey>,
    kind: LocalKind,
    count: usize,
    shape: crate::plan::ValueShape,
) {
    for index in 0..count {
        push_template_entry(entries, included, LocalKey::new(kind, index), shape.clone());
    }
}

fn push_value_entries(
    entries: &mut Vec<(LocalKey, crate::plan::ValueShape)>,
    included: &mut HashSet<LocalKey>,
    parts: &module::FrameLayoutParts<'_>,
) {
    push_counted_entries(
        entries,
        included,
        LocalKind::Int,
        parts.ints,
        crate::plan::ValueShape::Int,
    );
    push_counted_entries(
        entries,
        included,
        LocalKind::Float,
        parts.floats,
        crate::plan::ValueShape::Float,
    );
    push_counted_entries(
        entries,
        included,
        LocalKind::String,
        parts.strings,
        crate::plan::ValueShape::String,
    );
    push_counted_entries(
        entries,
        included,
        LocalKind::BitArray,
        parts.bit_arrays,
        crate::plan::ValueShape::BitArray,
    );
    push_counted_entries(
        entries,
        included,
        LocalKind::UtfCodepoint,
        parts.utf_codepoints,
        crate::plan::ValueShape::UtfCodepoint,
    );
    for local in parts.customs {
        push_template_entry(
            entries,
            included,
            LocalKey::new(LocalKind::Custom, local.id().0),
            crate::plan::ValueShape::Custom(local.shape().clone()),
        );
    }
    push_counted_entries(
        entries,
        included,
        LocalKind::Bool,
        parts.bools,
        crate::plan::ValueShape::Bool,
    );
    push_counted_entries(
        entries,
        included,
        LocalKind::Nil,
        parts.nils,
        crate::plan::ValueShape::Nil,
    );
    push_counted_entries(
        entries,
        included,
        LocalKind::Tuple,
        parts.tuples,
        crate::plan::ValueShape::Tuple(Box::new([])),
    );
    for local in parts.generics {
        push_template_entry(
            entries,
            included,
            LocalKey::new(LocalKind::Generic, local.id().0),
            crate::plan::ValueShape::Parameter(local.parameter()),
        );
    }
}

fn push_list_entries(
    entries: &mut Vec<(LocalKey, crate::plan::ValueShape)>,
    included: &mut HashSet<LocalKey>,
    parts: &module::FrameLayoutParts<'_>,
) {
    push_counted_list_entries(
        entries,
        included,
        LocalKind::IntList,
        parts.int_lists,
        crate::plan::ValueShape::Int,
    );
    push_counted_list_entries(
        entries,
        included,
        LocalKind::StringList,
        parts.string_lists,
        crate::plan::ValueShape::String,
    );
    push_counted_list_entries(
        entries,
        included,
        LocalKind::BitArrayList,
        parts.bit_array_lists,
        crate::plan::ValueShape::BitArray,
    );
    push_counted_list_entries(
        entries,
        included,
        LocalKind::UtfCodepointList,
        parts.utf_codepoint_lists,
        crate::plan::ValueShape::UtfCodepoint,
    );
    for (index, item) in parts.custom_lists.iter().enumerate() {
        push_template_entry(
            entries,
            included,
            LocalKey::new(LocalKind::CustomList, index),
            crate::plan::ValueShape::List(Box::new(crate::plan::ValueShape::Custom(
                crate::plan::CustomValueShape::any(item.clone()),
            ))),
        );
    }
    push_counted_list_entries(
        entries,
        included,
        LocalKind::FloatList,
        parts.float_lists,
        crate::plan::ValueShape::Float,
    );
    push_counted_list_entries(
        entries,
        included,
        LocalKind::BoolList,
        parts.bool_lists,
        crate::plan::ValueShape::Bool,
    );
    push_counted_list_entries(
        entries,
        included,
        LocalKind::NilList,
        parts.nil_lists,
        crate::plan::ValueShape::Nil,
    );
    for (index, item) in parts.tuple_lists.iter().enumerate() {
        push_template_entry(
            entries,
            included,
            LocalKey::new(LocalKind::TupleList, index),
            crate::plan::ValueShape::List(Box::new(crate::plan::ValueShape::Tuple(
                item.iter()
                    .cloned()
                    .map(crate::plan::ValueShape::from_value_type)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ))),
        );
    }
    for (index, item) in parts.list_lists.iter().enumerate() {
        push_template_entry(
            entries,
            included,
            LocalKey::new(LocalKind::ListList, index),
            crate::plan::ValueShape::List(Box::new(crate::plan::ValueShape::List(Box::new(
                crate::plan::ValueShape::from_value_type(item.clone()),
            )))),
        );
    }
    for (index, item) in parts.function_lists.iter().enumerate() {
        push_template_entry(
            entries,
            included,
            LocalKey::new(LocalKind::FunctionList, index),
            crate::plan::ValueShape::List(Box::new(crate::plan::ValueShape::Function(Box::new(
                crate::plan::FunctionShape::from_function_type(item.clone()),
            )))),
        );
    }
    for (local, parameter) in parts.generic_lists {
        push_template_entry(
            entries,
            included,
            LocalKey::new(LocalKind::GenericList, local.0),
            crate::plan::ValueShape::List(Box::new(crate::plan::ValueShape::Parameter(*parameter))),
        );
    }
}

fn push_counted_list_entries(
    entries: &mut Vec<(LocalKey, crate::plan::ValueShape)>,
    included: &mut HashSet<LocalKey>,
    kind: LocalKind,
    count: usize,
    item: crate::plan::ValueShape,
) {
    push_counted_entries(
        entries,
        included,
        kind,
        count,
        crate::plan::ValueShape::List(Box::new(item)),
    );
}

fn push_function_entries(
    entries: &mut Vec<(LocalKey, crate::plan::ValueShape)>,
    included: &mut HashSet<LocalKey>,
    parts: &module::FrameLayoutParts<'_>,
) {
    push_counted_function_entries(
        entries,
        included,
        LocalKind::IntFunction,
        parts.int_functions,
        crate::plan::ValueShape::Int,
    );
    push_counted_function_entries(
        entries,
        included,
        LocalKind::FloatFunction,
        parts.float_functions,
        crate::plan::ValueShape::Float,
    );
    push_counted_function_entries(
        entries,
        included,
        LocalKind::StringFunction,
        parts.string_functions,
        crate::plan::ValueShape::String,
    );
    push_counted_function_entries(
        entries,
        included,
        LocalKind::BitArrayFunction,
        parts.bit_array_functions,
        crate::plan::ValueShape::BitArray,
    );
    push_counted_function_entries(
        entries,
        included,
        LocalKind::UtfCodepointFunction,
        parts.utf_codepoint_functions,
        crate::plan::ValueShape::UtfCodepoint,
    );
    for local in parts.custom_functions {
        push_template_entry(
            entries,
            included,
            LocalKey::new(LocalKind::CustomFunction, local.id().0),
            crate::plan::ValueShape::Function(Box::new(crate::plan::FunctionShape::new(
                local.type_().argument_shapes().to_vec(),
                crate::plan::ValueShape::Custom(local.type_().return_().clone()),
            ))),
        );
    }
    push_counted_function_entries(
        entries,
        included,
        LocalKind::BoolFunction,
        parts.bool_functions,
        crate::plan::ValueShape::Bool,
    );
    push_counted_function_entries(
        entries,
        included,
        LocalKind::NilFunction,
        parts.nil_functions,
        crate::plan::ValueShape::Nil,
    );
    for (local, shape) in parts.tuple_functions {
        push_template_entry(
            entries,
            included,
            LocalKey::new(LocalKind::TupleFunction, local.0),
            crate::plan::ValueShape::Function(Box::new(shape.clone())),
        );
    }
    for local in parts.list_functions {
        let (kind, index, type_) = list_function_local_parts(local);
        push_template_entry(
            entries,
            included,
            LocalKey::new(kind, index),
            crate::plan::ValueShape::Function(Box::new(
                crate::plan::FunctionShape::from_function_type(type_),
            )),
        );
    }
    for local in parts.function_functions {
        push_template_entry(
            entries,
            included,
            LocalKey::new(LocalKind::FunctionFunction, local.id().0),
            crate::plan::ValueShape::Function(Box::new(crate::plan::FunctionShape::new(
                local.type_().argument_shapes().to_vec(),
                crate::plan::ValueShape::Function(Box::new(local.type_().return_shape().clone())),
            ))),
        );
    }
    for local in parts.generic_functions {
        push_template_entry(
            entries,
            included,
            LocalKey::new(LocalKind::GenericFunction, local.id().0),
            crate::plan::ValueShape::Function(Box::new(local.type_().shape())),
        );
    }
}

fn push_counted_function_entries(
    entries: &mut Vec<(LocalKey, crate::plan::ValueShape)>,
    included: &mut HashSet<LocalKey>,
    kind: LocalKind,
    count: usize,
    return_: crate::plan::ValueShape,
) {
    push_counted_entries(
        entries,
        included,
        kind,
        count,
        crate::plan::ValueShape::Function(Box::new(crate::plan::FunctionShape::new(
            Vec::new(),
            return_,
        ))),
    );
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

pub(super) fn frame_layout(context: &mut LoweringContext) -> execution::FrameLayout {
    let entries = context.current_local_shapes();
    let mut slots = execution::frame::FrameSlots::default();
    let mut nils = 0;

    for shape in entries {
        allocate_value_local(&shape, &mut slots, &mut nils, context);
    }

    execution::FrameLayout::from_slots(slots)
}

pub(super) fn generic_list_returning_function_local(
    local: &crate::plan::GenericFunctionLocal,
    item: &SpecializedValueShape,
    context: &mut LoweringContext,
) -> execution::ListFunctionLocal {
    let shape = context.concrete_function_shape(&local.type_().shape());
    let type_ = context.lower_concrete_function_type(&shape);
    list_function_local_at(
        item,
        type_,
        context.generic_function_local_index(local.id()),
        context,
    )
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

fn allocate_value_local(
    shape: &StoredValueShape,
    slots: &mut execution::frame::FrameSlots,
    nils: &mut usize,
    context: &mut LoweringContext,
) -> execution::ParamLocal {
    match shape {
        StoredValueShape::Int => {
            let local = execution::IntLocalId(slots.ints);
            slots.ints += 1;
            execution::ParamLocal::Int(local)
        }
        StoredValueShape::Float => {
            let local = execution::FloatLocalId(slots.floats);
            slots.floats += 1;
            execution::ParamLocal::Float(local)
        }
        StoredValueShape::String => {
            let local = execution::StringLocalId(slots.strings);
            slots.strings += 1;
            execution::ParamLocal::String(local)
        }
        StoredValueShape::BitArray => {
            let local = execution::BitArrayLocalId(slots.bit_arrays);
            slots.bit_arrays += 1;
            execution::ParamLocal::BitArray(local)
        }
        StoredValueShape::UtfCodepoint => {
            let local = execution::UtfCodepointLocalId(slots.utf_codepoints);
            slots.utf_codepoints += 1;
            execution::ParamLocal::UtfCodepoint(local)
        }
        StoredValueShape::Custom(shape) => {
            let local = execution::CustomLocal::new(
                execution::CustomLocalId(slots.customs.len()),
                context.lower_concrete_custom_shape(shape),
            );
            slots.customs.push(local);
            execution::ParamLocal::Custom(local)
        }
        StoredValueShape::Bool => {
            let local = execution::BoolLocalId(slots.bools);
            slots.bools += 1;
            execution::ParamLocal::Bool(local)
        }
        StoredValueShape::Nil => {
            let local = execution::NilLocalId(*nils);
            *nils += 1;
            execution::ParamLocal::Nil(local)
        }
        StoredValueShape::Tuple(elements) => {
            let local = execution::TupleLocalId(slots.tuples);
            slots.tuples += 1;
            execution::ParamLocal::Tuple {
                local,
                type_: elements
                    .iter()
                    .map(|element| context.lower_concrete_value_type(element))
                    .collect(),
            }
        }
        StoredValueShape::List(item) => {
            execution::ParamLocal::List(allocate_list_local(item, slots, context))
        }
        StoredValueShape::Function(function) => {
            function_local_as_param(allocate_function_local(function, slots, context))
        }
    }
}

fn allocate_list_local(
    item: &SpecializedValueShape,
    slots: &mut execution::frame::FrameSlots,
    context: &mut LoweringContext,
) -> execution::ListLocal {
    match item {
        SpecializedValueShape::Parameter(parameter) => push_list_local(
            &mut slots.parameter_lists,
            context.parameter_list_type(*parameter),
            execution::ParameterListLocalId,
            |local, type_id| execution::ListLocal::Parameter { local, type_id },
        ),
        SpecializedValueShape::Int => push_list_local(
            &mut slots.int_lists,
            context.int_list_type(),
            execution::IntListLocalId,
            |local, type_id| execution::ListLocal::Int { local, type_id },
        ),
        SpecializedValueShape::String => push_list_local(
            &mut slots.string_lists,
            context.string_list_type(),
            execution::StringListLocalId,
            |local, type_id| execution::ListLocal::String { local, type_id },
        ),
        SpecializedValueShape::BitArray => push_list_local(
            &mut slots.bit_array_lists,
            context.bit_array_list_type(),
            execution::BitArrayListLocalId,
            |local, type_id| execution::ListLocal::BitArray { local, type_id },
        ),
        SpecializedValueShape::UtfCodepoint => push_list_local(
            &mut slots.utf_codepoint_lists,
            context.utf_codepoint_list_type(),
            execution::UtfCodepointListLocalId,
            |local, type_id| execution::ListLocal::UtfCodepoint { local, type_id },
        ),
        SpecializedValueShape::Custom(custom) => push_list_local(
            &mut slots.custom_lists,
            context.specialized_custom_list_type(custom),
            execution::CustomListLocalId,
            |local, type_id| execution::ListLocal::Custom { local, type_id },
        ),
        SpecializedValueShape::Float => push_list_local(
            &mut slots.float_lists,
            context.float_list_type(),
            execution::FloatListLocalId,
            |local, type_id| execution::ListLocal::Float { local, type_id },
        ),
        SpecializedValueShape::Bool => push_list_local(
            &mut slots.bool_lists,
            context.bool_list_type(),
            execution::BoolListLocalId,
            |local, type_id| execution::ListLocal::Bool { local, type_id },
        ),
        SpecializedValueShape::Nil => push_list_local(
            &mut slots.nil_lists,
            context.nil_list_type(),
            execution::NilListLocalId,
            |local, type_id| execution::ListLocal::Nil { local, type_id },
        ),
        SpecializedValueShape::Tuple(elements) => push_list_local(
            &mut slots.tuple_lists,
            context.specialized_tuple_list_type(elements),
            execution::TupleListLocalId,
            |local, type_id| execution::ListLocal::Tuple { local, type_id },
        ),
        SpecializedValueShape::List(item) => match context.specialized_list_list_type(item) {
            super::value_type::NestedListTypeId::Parameter(type_id) => push_list_local(
                &mut slots.parameter_list_lists,
                type_id,
                execution::ParameterListListLocalId,
                |local, type_id| execution::ListLocal::ParameterList { local, type_id },
            ),
            super::value_type::NestedListTypeId::Stored(type_id) => push_list_local(
                &mut slots.list_lists,
                type_id,
                execution::ListListLocalId,
                |local, type_id| execution::ListLocal::List { local, type_id },
            ),
        },
        SpecializedValueShape::Function(function) => push_list_local(
            &mut slots.function_lists,
            context.specialized_function_list_type(function),
            execution::FunctionListLocalId,
            |local, type_id| execution::ListLocal::Function { local, type_id },
        ),
    }
}

fn push_list_local<TypeId, Local, Result>(
    slots: &mut Vec<TypeId>,
    type_id: TypeId,
    local: impl FnOnce(usize) -> Local,
    result: impl FnOnce(Local, TypeId) -> Result,
) -> Result
where
    TypeId: Copy,
{
    let local = local(slots.len());
    slots.push(type_id);
    result(local, type_id)
}

fn allocate_function_local(
    shape: &SpecializedFunctionShape,
    slots: &mut execution::frame::FrameSlots,
    context: &mut LoweringContext,
) -> SpecializedFunctionLocal {
    let type_ = context.lower_concrete_function_type(shape);
    match context.function_representation(shape) {
        FunctionRepresentation::Symbolic => {
            let local = execution::GenericFunctionLocal::new(
                execution::GenericFunctionLocalId(slots.generic_functions.len()),
                context.generic_function_type(shape),
            );
            slots.generic_functions.push(local.clone());
            SpecializedFunctionLocal::Generic(local)
        }
        FunctionRepresentation::Never(_) => {
            let local = execution::NeverFunctionLocal::new(
                execution::NeverFunctionLocalId(slots.never_functions.len()),
                context.generic_function_type(shape),
            );
            slots.never_functions.push(local.clone());
            SpecializedFunctionLocal::Never(local)
        }
        FunctionRepresentation::Executable(StoredValueShape::Int) => {
            let local = execution::IntFunctionLocalId(slots.int_functions);
            slots.int_functions += 1;
            SpecializedFunctionLocal::Int { local, type_ }
        }
        FunctionRepresentation::Executable(StoredValueShape::Float) => {
            let local = execution::FloatFunctionLocalId(slots.float_functions);
            slots.float_functions += 1;
            SpecializedFunctionLocal::Float { local, type_ }
        }
        FunctionRepresentation::Executable(StoredValueShape::String) => {
            let local = execution::StringFunctionLocalId(slots.string_functions);
            slots.string_functions += 1;
            SpecializedFunctionLocal::String { local, type_ }
        }
        FunctionRepresentation::Executable(StoredValueShape::BitArray) => {
            let local = execution::BitArrayFunctionLocalId(slots.bit_array_functions);
            slots.bit_array_functions += 1;
            SpecializedFunctionLocal::BitArray { local, type_ }
        }
        FunctionRepresentation::Executable(StoredValueShape::UtfCodepoint) => {
            let local = execution::UtfCodepointFunctionLocalId(slots.utf_codepoint_functions);
            slots.utf_codepoint_functions += 1;
            SpecializedFunctionLocal::UtfCodepoint { local, type_ }
        }
        FunctionRepresentation::Executable(StoredValueShape::Custom(custom)) => {
            let type_ = context.specialized_custom_function_type(shape.arguments(), &custom);
            let local = execution::CustomFunctionLocal::new(
                execution::CustomFunctionLocalId(next_custom_function_id(&slots.custom_functions)),
                type_,
            );
            slots.custom_functions.push(local.clone());
            SpecializedFunctionLocal::Custom(local)
        }
        FunctionRepresentation::Executable(StoredValueShape::Bool) => {
            let local = execution::BoolFunctionLocalId(slots.bool_functions);
            slots.bool_functions += 1;
            SpecializedFunctionLocal::Bool { local, type_ }
        }
        FunctionRepresentation::Executable(StoredValueShape::Nil) => {
            let local = execution::NilFunctionLocalId(slots.nil_functions);
            slots.nil_functions += 1;
            SpecializedFunctionLocal::Nil { local, type_ }
        }
        FunctionRepresentation::Executable(StoredValueShape::Tuple(_)) => {
            let local = execution::TupleFunctionLocalId(slots.tuple_functions);
            slots.tuple_functions += 1;
            SpecializedFunctionLocal::Tuple { local, type_ }
        }
        FunctionRepresentation::Executable(StoredValueShape::List(item)) => {
            let local = allocate_list_function_local(&item, type_, &slots.list_functions, context);
            slots.list_functions.push(local.clone());
            SpecializedFunctionLocal::List(local)
        }
        FunctionRepresentation::Executable(StoredValueShape::Function(returned)) => {
            let type_ = context.specialized_function_function_type(shape.arguments(), &returned);
            let local = execution::FunctionFunctionLocal::new(
                execution::FunctionFunctionLocalId(next_function_function_id(
                    &slots.function_functions,
                )),
                type_,
            );
            slots.function_functions.push(local.clone());
            SpecializedFunctionLocal::Function(local)
        }
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

fn next_custom_function_id(locals: &[execution::CustomFunctionLocal]) -> usize {
    locals
        .iter()
        .fold(0, |next, local| next.max(local.id().0 + 1))
}

fn next_function_function_id(locals: &[execution::FunctionFunctionLocal]) -> usize {
    locals
        .iter()
        .fold(0, |next, local| next.max(local.id().0 + 1))
}

fn allocate_list_function_local(
    item: &SpecializedValueShape,
    type_: execution::FunctionType,
    locals: &[execution::ListFunctionLocal],
    context: &mut LoweringContext,
) -> execution::ListFunctionLocal {
    use execution::ListFunctionLocal as L;

    match item {
        SpecializedValueShape::Parameter(parameter) => L::Parameter {
            local: execution::ParameterListFunctionLocalId(next_list_function_id(
                locals,
                |local| match local {
                    L::Parameter { local, .. } => Some(local.0),
                    _ => None,
                },
            )),
            type_,
            list_type: context.parameter_list_type(*parameter),
        },
        SpecializedValueShape::Int => L::Int {
            local: execution::IntListFunctionLocalId(next_list_function_id(locals, |local| {
                match local {
                    L::Int { local, .. } => Some(local.0),
                    _ => None,
                }
            })),
            type_,
            list_type: context.int_list_type(),
        },
        SpecializedValueShape::String => L::String {
            local: execution::StringListFunctionLocalId(next_list_function_id(locals, |local| {
                match local {
                    L::String { local, .. } => Some(local.0),
                    _ => None,
                }
            })),
            type_,
            list_type: context.string_list_type(),
        },
        SpecializedValueShape::BitArray => L::BitArray {
            local: execution::BitArrayListFunctionLocalId(next_list_function_id(locals, |local| {
                match local {
                    L::BitArray { local, .. } => Some(local.0),
                    _ => None,
                }
            })),
            type_,
            list_type: context.bit_array_list_type(),
        },
        SpecializedValueShape::UtfCodepoint => L::UtfCodepoint {
            local: execution::UtfCodepointListFunctionLocalId(next_list_function_id(
                locals,
                |local| match local {
                    L::UtfCodepoint { local, .. } => Some(local.0),
                    _ => None,
                },
            )),
            type_,
            list_type: context.utf_codepoint_list_type(),
        },
        SpecializedValueShape::Custom(custom) => L::Custom {
            local: execution::CustomListFunctionLocalId(next_list_function_id(locals, |local| {
                match local {
                    L::Custom { local, .. } => Some(local.0),
                    _ => None,
                }
            })),
            type_,
            list_type: context.specialized_custom_list_type(custom),
        },
        SpecializedValueShape::Float => L::Float {
            local: execution::FloatListFunctionLocalId(next_list_function_id(locals, |local| {
                match local {
                    L::Float { local, .. } => Some(local.0),
                    _ => None,
                }
            })),
            type_,
            list_type: context.float_list_type(),
        },
        SpecializedValueShape::Bool => L::Bool {
            local: execution::BoolListFunctionLocalId(next_list_function_id(locals, |local| {
                match local {
                    L::Bool { local, .. } => Some(local.0),
                    _ => None,
                }
            })),
            type_,
            list_type: context.bool_list_type(),
        },
        SpecializedValueShape::Nil => L::Nil {
            local: execution::NilListFunctionLocalId(next_list_function_id(locals, |local| {
                match local {
                    L::Nil { local, .. } => Some(local.0),
                    _ => None,
                }
            })),
            type_,
            list_type: context.nil_list_type(),
        },
        SpecializedValueShape::Tuple(elements) => L::Tuple {
            local: execution::TupleListFunctionLocalId(next_list_function_id(locals, |local| {
                match local {
                    L::Tuple { local, .. } => Some(local.0),
                    _ => None,
                }
            })),
            type_,
            list_type: context.specialized_tuple_list_type(elements),
        },
        SpecializedValueShape::List(item) => match context.specialized_list_list_type(item) {
            super::value_type::NestedListTypeId::Parameter(list_type) => L::ParameterList {
                local: execution::ParameterListListFunctionLocalId(next_list_function_id(
                    locals,
                    |local| match local {
                        L::ParameterList { local, .. } => Some(local.0),
                        _ => None,
                    },
                )),
                type_,
                list_type,
            },
            super::value_type::NestedListTypeId::Stored(list_type) => L::List {
                local: execution::ListListFunctionLocalId(next_list_function_id(locals, |local| {
                    match local {
                        L::List { local, .. } => Some(local.0),
                        _ => None,
                    }
                })),
                type_,
                list_type,
            },
        },
        SpecializedValueShape::Function(function) => L::Function {
            local: execution::FunctionListFunctionLocalId(next_list_function_id(locals, |local| {
                match local {
                    L::Function { local, .. } => Some(local.0),
                    _ => None,
                }
            })),
            type_,
            list_type: context.specialized_function_list_type(function),
        },
    }
}

fn next_list_function_id(
    locals: &[execution::ListFunctionLocal],
    index: impl Fn(&execution::ListFunctionLocal) -> Option<usize>,
) -> usize {
    locals
        .iter()
        .filter_map(index)
        .fold(0, |next, index| next.max(index + 1))
}

#[cfg(test)]
mod tests {
    use super::super::super::{
        ExecutionPlan, FunctionType as ExecutionFunctionType, IntFunctionId,
        IntListFunctionLocalId, IntListTypeId, ListFunctionLocal, StringListFunctionLocalId,
        StringListTypeId, ValueType as ExecutionValueType,
    };
    use crate::plan::{FunctionType, TypeParameterId, ValueType};

    #[test]
    fn lowering_preserves_every_execution_frame_slot_family() {
        let source = r#"
pub type Marker {
  Marker
}

fn all_slots(
  int: Int,
  float: Float,
  string: String,
  bit_array: BitArray,
  utf_codepoint: UtfCodepoint,
  bool: Bool,
  nil: Nil,
  tuple: #(Int),
  int_list: List(Int),
  string_list: List(String),
  bit_array_list: List(BitArray),
  utf_codepoint_list: List(UtfCodepoint),
  float_list: List(Float),
  bool_list: List(Bool),
  nil_list: List(Nil),
  tuple_list: List(#(Int)),
  list_list: List(List(Int)),
  function_list: List(fn() -> Int),
  int_function: fn() -> Int,
  float_function: fn() -> Float,
  string_function: fn() -> String,
  bit_array_function: fn() -> BitArray,
  utf_codepoint_function: fn() -> UtfCodepoint,
  bool_function: fn() -> Bool,
  nil_function: fn() -> Nil,
  tuple_function: fn() -> #(Int),
  string_list_function: fn() -> List(String),
  string_list_function_2: fn() -> List(String),
  int_list_function: fn() -> List(Int),
  int_list_function_2: fn() -> List(Int),
  bit_array_list_function: fn() -> List(BitArray),
  bit_array_list_function_2: fn() -> List(BitArray),
  utf_codepoint_list_function: fn() -> List(UtfCodepoint),
  utf_codepoint_list_function_2: fn() -> List(UtfCodepoint),
  custom_list_function: fn() -> List(Marker),
  custom_list_function_2: fn() -> List(Marker),
  float_list_function: fn() -> List(Float),
  float_list_function_2: fn() -> List(Float),
  bool_list_function: fn() -> List(Bool),
  bool_list_function_2: fn() -> List(Bool),
  nil_list_function: fn() -> List(Nil),
  nil_list_function_2: fn() -> List(Nil),
  tuple_list_function: fn() -> List(#(Int)),
  tuple_list_function_2: fn() -> List(#(Int)),
  list_list_function: fn() -> List(List(Int)),
  list_list_function_2: fn() -> List(List(Int)),
  function_list_function: fn() -> List(fn() -> Int),
  function_list_function_2: fn() -> List(fn() -> Int),
  function_function: fn() -> fn() -> Int,
) {
  int
}

fn string_list_function_after_int(
  int_list_function: fn() -> List(Int),
  string_list_function: fn() -> List(String),
) {
  0
}

fn parameter_list_function_after_int(
  int_list_function: fn() -> List(Int),
  parameter_list_function: fn() -> List(value),
) {
  0
}

fn int_values() {
  [1]
}

fn empty_values() {
  []
}

pub fn main() {
  let _ = #(all_slots, string_list_function_after_int)
  parameter_list_function_after_int(int_values, empty_values)
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);
        let layout = plan.int_function(IntFunctionId(1)).frame_layout();

        assert_eq!(layout.ints(), 1);
        assert_eq!(layout.floats(), 1);
        assert_eq!(layout.strings(), 1);
        assert_eq!(layout.bit_arrays(), 1);
        assert_eq!(layout.utf_codepoints(), 1);
        assert_eq!(layout.bools(), 1);
        assert_eq!(layout.tuples(), 1);
        assert_eq!(layout.int_lists().len(), 1);
        assert_eq!(layout.string_lists().len(), 1);
        assert_eq!(layout.bit_array_lists().len(), 1);
        assert_eq!(layout.utf_codepoint_lists().len(), 1);
        assert_eq!(layout.float_lists().len(), 1);
        assert_eq!(layout.bool_lists().len(), 1);
        assert_eq!(layout.nil_lists().len(), 1);
        assert_eq!(layout.tuple_lists().len(), 1);
        assert_eq!(layout.list_lists().len(), 1);
        assert_eq!(layout.function_lists().len(), 1);
        assert_eq!(
            plan.tuple_list_item_type(layout.tuple_lists()[0]),
            vec![ValueType::Int]
        );
        assert_eq!(
            plan.nested_list_item_type(layout.list_lists()[0]),
            ValueType::Int
        );
        assert_eq!(
            plan.function_list_item_type(layout.function_lists()[0]),
            FunctionType::new(Vec::new(), ValueType::Int)
        );
        assert_eq!(layout.int_functions(), 1);
        assert_eq!(layout.float_functions(), 1);
        assert_eq!(layout.string_functions(), 1);
        assert_eq!(layout.bit_array_functions(), 1);
        assert_eq!(layout.utf_codepoint_functions(), 1);
        assert_eq!(layout.bool_functions(), 1);
        assert_eq!(layout.nil_functions(), 1);
        assert_eq!(layout.tuple_functions(), 1);
        assert_eq!(layout.function_functions().len(), 1);

        let item_function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let marker_type = crate::plan::CustomType::new(
            crate::plan::CustomTypeName::new("geam".into(), "main".into(), "Marker".into()),
            Vec::new(),
        );
        let expected_returns = [
            ValueType::List(Box::new(ValueType::String)),
            ValueType::List(Box::new(ValueType::String)),
            ValueType::List(Box::new(ValueType::Int)),
            ValueType::List(Box::new(ValueType::Int)),
            ValueType::List(Box::new(ValueType::BitArray)),
            ValueType::List(Box::new(ValueType::BitArray)),
            ValueType::List(Box::new(ValueType::UtfCodepoint)),
            ValueType::List(Box::new(ValueType::UtfCodepoint)),
            ValueType::List(Box::new(ValueType::Custom(marker_type.clone()))),
            ValueType::List(Box::new(ValueType::Custom(marker_type))),
            ValueType::List(Box::new(ValueType::Float)),
            ValueType::List(Box::new(ValueType::Float)),
            ValueType::List(Box::new(ValueType::Bool)),
            ValueType::List(Box::new(ValueType::Bool)),
            ValueType::List(Box::new(ValueType::Nil)),
            ValueType::List(Box::new(ValueType::Nil)),
            ValueType::List(Box::new(ValueType::Tuple(vec![ValueType::Int]))),
            ValueType::List(Box::new(ValueType::Tuple(vec![ValueType::Int]))),
            ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Int)))),
            ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Int)))),
            ValueType::List(Box::new(ValueType::Function(Box::new(item_function_type)))),
            ValueType::List(Box::new(ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Int,
            ))))),
        ];
        let list_functions = layout.list_functions();
        assert_eq!(list_functions.len(), expected_returns.len());
        for (local, expected_return) in list_functions.iter().zip(expected_returns) {
            assert_eq!(
                plan.function_type(local.type_()),
                FunctionType::new(Vec::new(), expected_return.clone())
            );
            assert_eq!(plan.list_value_type(local.list_type()), expected_return);
        }

        let mixed_layout = plan.int_function(IntFunctionId(2)).frame_layout();
        let mixed_locals = mixed_layout.list_functions();
        assert_eq!(mixed_locals.len(), 2);
        assert_eq!(
            mixed_locals,
            &[
                ListFunctionLocal::Int {
                    local: IntListFunctionLocalId(0),
                    type_: ExecutionFunctionType::new(
                        Vec::new(),
                        ExecutionValueType::List(mixed_locals[0].list_type()),
                    ),
                    list_type: IntListTypeId::new(mixed_locals[0].list_type()),
                },
                ListFunctionLocal::String {
                    local: StringListFunctionLocalId(0),
                    type_: ExecutionFunctionType::new(
                        Vec::new(),
                        ExecutionValueType::List(mixed_locals[1].list_type()),
                    ),
                    list_type: StringListTypeId::new(mixed_locals[1].list_type()),
                },
            ],
        );

        let parameter_layout = plan.int_function(IntFunctionId(3)).frame_layout();
        let parameter_locals = parameter_layout.list_functions();
        let parameter_list_type = parameter_locals[1].list_type();
        assert_eq!(
            parameter_locals,
            &[
                ListFunctionLocal::Int {
                    local: IntListFunctionLocalId(0),
                    type_: ExecutionFunctionType::new(
                        Vec::new(),
                        ExecutionValueType::List(parameter_locals[0].list_type()),
                    ),
                    list_type: IntListTypeId::new(parameter_locals[0].list_type()),
                },
                ListFunctionLocal::Parameter {
                    local: super::super::super::ParameterListFunctionLocalId(0),
                    type_: ExecutionFunctionType::new(
                        Vec::new(),
                        ExecutionValueType::List(parameter_list_type),
                    ),
                    list_type: super::super::super::ParameterListTypeId::new(
                        parameter_list_type,
                        TypeParameterId(0),
                    ),
                },
            ],
        );
    }
}
