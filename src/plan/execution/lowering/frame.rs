use super::specialization::{ConcreteFunctionShape, ConcreteValueShape};
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
    entries: Box<[(LocalKey, ConcreteValueShape)]>,
    allocations: HashMap<LocalKey, LocalAllocation>,
}

#[derive(Clone, Copy)]
struct LocalAllocation {
    entry: usize,
    index: usize,
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
    IntFunction,
    FloatFunction,
    StringFunction,
    BitArrayFunction,
    UtfCodepointFunction,
    CustomFunction,
    BoolFunction,
    NilFunction,
    TupleFunction,
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
        substitution: &super::specialization::ConcreteTypeSubstitution,
    ) -> LocalAllocationPlan {
        let entries = self
            .entries
            .iter()
            .map(|(key, shape)| (*key, ConcreteValueShape::instantiate(shape, substitution)))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let mut next = HashMap::<StorageFamily, usize>::new();
        let mut allocations = HashMap::with_capacity(entries.len());

        for (entry, (key, shape)) in entries.iter().enumerate() {
            let index = next.entry(StorageFamily::of(shape)).or_default();
            allocations.insert(
                *key,
                LocalAllocation {
                    entry,
                    index: *index,
                },
            );
            *index += 1;
        }

        LocalAllocationPlan {
            entries,
            allocations,
        }
    }
}

impl LocalAllocationPlan {
    pub(super) fn index(&self, key: LocalKey) -> usize {
        self.allocations[&key].index
    }

    pub(super) fn shape(&self, key: LocalKey) -> &ConcreteValueShape {
        &self.entries[self.allocations[&key].entry].1
    }

    pub(super) fn entries(&self) -> &[(LocalKey, ConcreteValueShape)] {
        &self.entries
    }
}

impl ParameterPrefix {
    pub(super) fn allocate(&mut self, shape: &ConcreteValueShape) -> usize {
        let index = self.next.entry(StorageFamily::of(shape)).or_default();
        let allocated = *index;
        *index += 1;
        allocated
    }
}

impl StorageFamily {
    fn of(shape: &ConcreteValueShape) -> Self {
        match shape {
            ConcreteValueShape::Int => Self::Int,
            ConcreteValueShape::Float => Self::Float,
            ConcreteValueShape::String => Self::String,
            ConcreteValueShape::BitArray => Self::BitArray,
            ConcreteValueShape::UtfCodepoint => Self::UtfCodepoint,
            ConcreteValueShape::Custom(_) => Self::Custom,
            ConcreteValueShape::Bool => Self::Bool,
            ConcreteValueShape::Nil => Self::Nil,
            ConcreteValueShape::Tuple(_) => Self::Tuple,
            ConcreteValueShape::List(item) => Self::list(item),
            ConcreteValueShape::Function(function) => Self::function(function.return_()),
        }
    }

    fn list(item: &ConcreteValueShape) -> Self {
        match item {
            ConcreteValueShape::Int => Self::IntList,
            ConcreteValueShape::String => Self::StringList,
            ConcreteValueShape::BitArray => Self::BitArrayList,
            ConcreteValueShape::UtfCodepoint => Self::UtfCodepointList,
            ConcreteValueShape::Custom(_) => Self::CustomList,
            ConcreteValueShape::Float => Self::FloatList,
            ConcreteValueShape::Bool => Self::BoolList,
            ConcreteValueShape::Nil => Self::NilList,
            ConcreteValueShape::Tuple(_) => Self::TupleList,
            ConcreteValueShape::List(_) => Self::ListList,
            ConcreteValueShape::Function(_) => Self::FunctionList,
        }
    }

    fn function(return_: &ConcreteValueShape) -> Self {
        match return_ {
            ConcreteValueShape::Int => Self::IntFunction,
            ConcreteValueShape::Float => Self::FloatFunction,
            ConcreteValueShape::String => Self::StringFunction,
            ConcreteValueShape::BitArray => Self::BitArrayFunction,
            ConcreteValueShape::UtfCodepoint => Self::UtfCodepointFunction,
            ConcreteValueShape::Custom(_) => Self::CustomFunction,
            ConcreteValueShape::Bool => Self::BoolFunction,
            ConcreteValueShape::Nil => Self::NilFunction,
            ConcreteValueShape::Tuple(_) => Self::TupleFunction,
            ConcreteValueShape::List(item) => Self::list_function(item),
            ConcreteValueShape::Function(_) => Self::FunctionFunction,
        }
    }

    fn list_function(item: &ConcreteValueShape) -> Self {
        match item {
            ConcreteValueShape::Int => Self::IntListFunction,
            ConcreteValueShape::String => Self::StringListFunction,
            ConcreteValueShape::BitArray => Self::BitArrayListFunction,
            ConcreteValueShape::UtfCodepoint => Self::UtfCodepointListFunction,
            ConcreteValueShape::Custom(_) => Self::CustomListFunction,
            ConcreteValueShape::Float => Self::FloatListFunction,
            ConcreteValueShape::Bool => Self::BoolListFunction,
            ConcreteValueShape::Nil => Self::NilListFunction,
            ConcreteValueShape::Tuple(_) => Self::TupleListFunction,
            ConcreteValueShape::List(_) => Self::ListListFunction,
            ConcreteValueShape::Function(_) => Self::FunctionListFunction,
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
    push_counted_function_entries(
        entries,
        included,
        LocalKind::TupleFunction,
        parts.tuple_functions,
        crate::plan::ValueShape::Tuple(Box::new([])),
    );
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
    let entries = context.current_local_entries().to_vec();
    let mut slots = execution::frame::FrameSlots::default();
    let mut nils = 0;

    for (_, shape) in entries {
        allocate_value_local(&shape, &mut slots, &mut nils, context);
    }

    execution::FrameLayout::from_slots(slots)
}

pub(super) fn generic_list_returning_function_local(
    local: &crate::plan::GenericFunctionLocal,
    item: &ConcreteValueShape,
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

pub(super) fn value_local_at(
    shape: &ConcreteValueShape,
    index: usize,
    context: &mut LoweringContext,
) -> execution::ParamLocal {
    match shape {
        ConcreteValueShape::Int => execution::ParamLocal::Int(execution::IntLocalId(index)),
        ConcreteValueShape::Float => execution::ParamLocal::Float(execution::FloatLocalId(index)),
        ConcreteValueShape::String => {
            execution::ParamLocal::String(execution::StringLocalId(index))
        }
        ConcreteValueShape::BitArray => {
            execution::ParamLocal::BitArray(execution::BitArrayLocalId(index))
        }
        ConcreteValueShape::UtfCodepoint => {
            execution::ParamLocal::UtfCodepoint(execution::UtfCodepointLocalId(index))
        }
        ConcreteValueShape::Custom(shape) => {
            execution::ParamLocal::Custom(execution::CustomLocal::new(
                execution::CustomLocalId(index),
                context.lower_concrete_custom_shape(shape),
            ))
        }
        ConcreteValueShape::Bool => execution::ParamLocal::Bool(execution::BoolLocalId(index)),
        ConcreteValueShape::Nil => execution::ParamLocal::Nil(execution::NilLocalId(index)),
        ConcreteValueShape::Tuple(elements) => execution::ParamLocal::Tuple {
            local: execution::TupleLocalId(index),
            type_: elements
                .iter()
                .map(|element| context.lower_concrete_value_type(element))
                .collect(),
        },
        ConcreteValueShape::List(item) => {
            execution::ParamLocal::List(list_local_at(item, index, context))
        }
        ConcreteValueShape::Function(function) => {
            function_local_as_param(function_local_at(function, index, context))
        }
    }
}

pub(super) fn list_local_at(
    item: &ConcreteValueShape,
    index: usize,
    context: &mut LoweringContext,
) -> execution::ListLocal {
    match item {
        ConcreteValueShape::Int => execution::ListLocal::Int {
            local: execution::IntListLocalId(index),
            type_id: context.int_list_type(),
        },
        ConcreteValueShape::String => execution::ListLocal::String {
            local: execution::StringListLocalId(index),
            type_id: context.string_list_type(),
        },
        ConcreteValueShape::BitArray => execution::ListLocal::BitArray {
            local: execution::BitArrayListLocalId(index),
            type_id: context.bit_array_list_type(),
        },
        ConcreteValueShape::UtfCodepoint => execution::ListLocal::UtfCodepoint {
            local: execution::UtfCodepointListLocalId(index),
            type_id: context.utf_codepoint_list_type(),
        },
        ConcreteValueShape::Custom(custom) => execution::ListLocal::Custom {
            local: execution::CustomListLocalId(index),
            type_id: context.custom_list_type(custom.to_module_shape().type_().clone()),
        },
        ConcreteValueShape::Float => execution::ListLocal::Float {
            local: execution::FloatListLocalId(index),
            type_id: context.float_list_type(),
        },
        ConcreteValueShape::Bool => execution::ListLocal::Bool {
            local: execution::BoolListLocalId(index),
            type_id: context.bool_list_type(),
        },
        ConcreteValueShape::Nil => execution::ListLocal::Nil {
            local: execution::NilListLocalId(index),
            type_id: context.nil_list_type(),
        },
        ConcreteValueShape::Tuple(elements) => execution::ListLocal::Tuple {
            local: execution::TupleListLocalId(index),
            type_id: context.tuple_list_type(
                elements
                    .iter()
                    .map(ConcreteValueShape::value_type)
                    .collect(),
            ),
        },
        ConcreteValueShape::List(item) => execution::ListLocal::List {
            local: execution::ListListLocalId(index),
            type_id: context.list_list_type(item.value_type()),
        },
        ConcreteValueShape::Function(function) => execution::ListLocal::Function {
            local: execution::FunctionListLocalId(index),
            type_id: context.function_list_type(function.to_module_shape().type_()),
        },
    }
}

pub(super) fn function_local_at(
    shape: &ConcreteFunctionShape,
    index: usize,
    context: &mut LoweringContext,
) -> SpecializedFunctionLocal {
    let type_ = context.lower_concrete_function_type(shape);
    match shape.return_() {
        ConcreteValueShape::Int => SpecializedFunctionLocal::Int {
            local: execution::IntFunctionLocalId(index),
            type_,
        },
        ConcreteValueShape::Float => SpecializedFunctionLocal::Float {
            local: execution::FloatFunctionLocalId(index),
            type_,
        },
        ConcreteValueShape::String => SpecializedFunctionLocal::String {
            local: execution::StringFunctionLocalId(index),
            type_,
        },
        ConcreteValueShape::BitArray => SpecializedFunctionLocal::BitArray {
            local: execution::BitArrayFunctionLocalId(index),
            type_,
        },
        ConcreteValueShape::UtfCodepoint => SpecializedFunctionLocal::UtfCodepoint {
            local: execution::UtfCodepointFunctionLocalId(index),
            type_,
        },
        ConcreteValueShape::Custom(custom) => {
            let type_ = context.custom_function_type(crate::plan::CustomFunctionType::from_shapes(
                shape
                    .arguments()
                    .iter()
                    .map(ConcreteValueShape::to_module_shape)
                    .collect(),
                custom.to_module_shape(),
            ));
            SpecializedFunctionLocal::Custom(execution::CustomFunctionLocal::new(
                execution::CustomFunctionLocalId(index),
                type_,
            ))
        }
        ConcreteValueShape::Bool => SpecializedFunctionLocal::Bool {
            local: execution::BoolFunctionLocalId(index),
            type_,
        },
        ConcreteValueShape::Nil => SpecializedFunctionLocal::Nil {
            local: execution::NilFunctionLocalId(index),
            type_,
        },
        ConcreteValueShape::Tuple(_) => SpecializedFunctionLocal::Tuple {
            local: execution::TupleFunctionLocalId(index),
            type_,
        },
        ConcreteValueShape::List(item) => {
            SpecializedFunctionLocal::List(list_function_local_at(item, type_, index, context))
        }
        ConcreteValueShape::Function(returned) => {
            let type_ =
                context.function_function_type(crate::plan::FunctionFunctionType::from_shapes(
                    shape
                        .arguments()
                        .iter()
                        .map(ConcreteValueShape::to_module_shape)
                        .collect(),
                    returned.to_module_shape(),
                ));
            SpecializedFunctionLocal::Function(execution::FunctionFunctionLocal::new(
                execution::FunctionFunctionLocalId(index),
                type_,
            ))
        }
    }
}

pub(super) fn list_function_local_at(
    item: &ConcreteValueShape,
    type_: execution::FunctionType,
    index: usize,
    context: &mut LoweringContext,
) -> execution::ListFunctionLocal {
    use execution::ListFunctionLocal as L;

    match item {
        ConcreteValueShape::Int => L::Int {
            local: execution::IntListFunctionLocalId(index),
            type_,
            list_type: context.int_list_type(),
        },
        ConcreteValueShape::String => L::String {
            local: execution::StringListFunctionLocalId(index),
            type_,
            list_type: context.string_list_type(),
        },
        ConcreteValueShape::BitArray => L::BitArray {
            local: execution::BitArrayListFunctionLocalId(index),
            type_,
            list_type: context.bit_array_list_type(),
        },
        ConcreteValueShape::UtfCodepoint => L::UtfCodepoint {
            local: execution::UtfCodepointListFunctionLocalId(index),
            type_,
            list_type: context.utf_codepoint_list_type(),
        },
        ConcreteValueShape::Custom(custom) => L::Custom {
            local: execution::CustomListFunctionLocalId(index),
            type_,
            list_type: context.custom_list_type(custom.to_module_shape().type_().clone()),
        },
        ConcreteValueShape::Float => L::Float {
            local: execution::FloatListFunctionLocalId(index),
            type_,
            list_type: context.float_list_type(),
        },
        ConcreteValueShape::Bool => L::Bool {
            local: execution::BoolListFunctionLocalId(index),
            type_,
            list_type: context.bool_list_type(),
        },
        ConcreteValueShape::Nil => L::Nil {
            local: execution::NilListFunctionLocalId(index),
            type_,
            list_type: context.nil_list_type(),
        },
        ConcreteValueShape::Tuple(elements) => L::Tuple {
            local: execution::TupleListFunctionLocalId(index),
            type_,
            list_type: context.tuple_list_type(
                elements
                    .iter()
                    .map(ConcreteValueShape::value_type)
                    .collect(),
            ),
        },
        ConcreteValueShape::List(item) => L::List {
            local: execution::ListListFunctionLocalId(index),
            type_,
            list_type: context.list_list_type(item.value_type()),
        },
        ConcreteValueShape::Function(function) => L::Function {
            local: execution::FunctionListFunctionLocalId(index),
            type_,
            list_type: context.function_list_type(function.to_module_shape().type_()),
        },
    }
}

fn allocate_value_local(
    shape: &ConcreteValueShape,
    slots: &mut execution::frame::FrameSlots,
    nils: &mut usize,
    context: &mut LoweringContext,
) -> execution::ParamLocal {
    match shape {
        ConcreteValueShape::Int => {
            let local = execution::IntLocalId(slots.ints);
            slots.ints += 1;
            execution::ParamLocal::Int(local)
        }
        ConcreteValueShape::Float => {
            let local = execution::FloatLocalId(slots.floats);
            slots.floats += 1;
            execution::ParamLocal::Float(local)
        }
        ConcreteValueShape::String => {
            let local = execution::StringLocalId(slots.strings);
            slots.strings += 1;
            execution::ParamLocal::String(local)
        }
        ConcreteValueShape::BitArray => {
            let local = execution::BitArrayLocalId(slots.bit_arrays);
            slots.bit_arrays += 1;
            execution::ParamLocal::BitArray(local)
        }
        ConcreteValueShape::UtfCodepoint => {
            let local = execution::UtfCodepointLocalId(slots.utf_codepoints);
            slots.utf_codepoints += 1;
            execution::ParamLocal::UtfCodepoint(local)
        }
        ConcreteValueShape::Custom(shape) => {
            let local = execution::CustomLocal::new(
                execution::CustomLocalId(slots.customs.len()),
                context.lower_concrete_custom_shape(shape),
            );
            slots.customs.push(local);
            execution::ParamLocal::Custom(local)
        }
        ConcreteValueShape::Bool => {
            let local = execution::BoolLocalId(slots.bools);
            slots.bools += 1;
            execution::ParamLocal::Bool(local)
        }
        ConcreteValueShape::Nil => {
            let local = execution::NilLocalId(*nils);
            *nils += 1;
            execution::ParamLocal::Nil(local)
        }
        ConcreteValueShape::Tuple(elements) => {
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
        ConcreteValueShape::List(item) => {
            execution::ParamLocal::List(allocate_list_local(item, slots, context))
        }
        ConcreteValueShape::Function(function) => {
            function_local_as_param(allocate_function_local(function, slots, context))
        }
    }
}

fn allocate_list_local(
    item: &ConcreteValueShape,
    slots: &mut execution::frame::FrameSlots,
    context: &mut LoweringContext,
) -> execution::ListLocal {
    match item {
        ConcreteValueShape::Int => push_list_local(
            &mut slots.int_lists,
            context.int_list_type(),
            execution::IntListLocalId,
            |local, type_id| execution::ListLocal::Int { local, type_id },
        ),
        ConcreteValueShape::String => push_list_local(
            &mut slots.string_lists,
            context.string_list_type(),
            execution::StringListLocalId,
            |local, type_id| execution::ListLocal::String { local, type_id },
        ),
        ConcreteValueShape::BitArray => push_list_local(
            &mut slots.bit_array_lists,
            context.bit_array_list_type(),
            execution::BitArrayListLocalId,
            |local, type_id| execution::ListLocal::BitArray { local, type_id },
        ),
        ConcreteValueShape::UtfCodepoint => push_list_local(
            &mut slots.utf_codepoint_lists,
            context.utf_codepoint_list_type(),
            execution::UtfCodepointListLocalId,
            |local, type_id| execution::ListLocal::UtfCodepoint { local, type_id },
        ),
        ConcreteValueShape::Custom(custom) => push_list_local(
            &mut slots.custom_lists,
            context.custom_list_type(custom.to_module_shape().type_().clone()),
            execution::CustomListLocalId,
            |local, type_id| execution::ListLocal::Custom { local, type_id },
        ),
        ConcreteValueShape::Float => push_list_local(
            &mut slots.float_lists,
            context.float_list_type(),
            execution::FloatListLocalId,
            |local, type_id| execution::ListLocal::Float { local, type_id },
        ),
        ConcreteValueShape::Bool => push_list_local(
            &mut slots.bool_lists,
            context.bool_list_type(),
            execution::BoolListLocalId,
            |local, type_id| execution::ListLocal::Bool { local, type_id },
        ),
        ConcreteValueShape::Nil => push_list_local(
            &mut slots.nil_lists,
            context.nil_list_type(),
            execution::NilListLocalId,
            |local, type_id| execution::ListLocal::Nil { local, type_id },
        ),
        ConcreteValueShape::Tuple(elements) => push_list_local(
            &mut slots.tuple_lists,
            context.tuple_list_type(
                elements
                    .iter()
                    .map(ConcreteValueShape::value_type)
                    .collect(),
            ),
            execution::TupleListLocalId,
            |local, type_id| execution::ListLocal::Tuple { local, type_id },
        ),
        ConcreteValueShape::List(item) => push_list_local(
            &mut slots.list_lists,
            context.list_list_type(item.value_type()),
            execution::ListListLocalId,
            |local, type_id| execution::ListLocal::List { local, type_id },
        ),
        ConcreteValueShape::Function(function) => push_list_local(
            &mut slots.function_lists,
            context.function_list_type(function.to_module_shape().type_()),
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
    shape: &ConcreteFunctionShape,
    slots: &mut execution::frame::FrameSlots,
    context: &mut LoweringContext,
) -> SpecializedFunctionLocal {
    let type_ = context.lower_concrete_function_type(shape);
    match shape.return_() {
        ConcreteValueShape::Int => {
            let local = execution::IntFunctionLocalId(slots.int_functions);
            slots.int_functions += 1;
            SpecializedFunctionLocal::Int { local, type_ }
        }
        ConcreteValueShape::Float => {
            let local = execution::FloatFunctionLocalId(slots.float_functions);
            slots.float_functions += 1;
            SpecializedFunctionLocal::Float { local, type_ }
        }
        ConcreteValueShape::String => {
            let local = execution::StringFunctionLocalId(slots.string_functions);
            slots.string_functions += 1;
            SpecializedFunctionLocal::String { local, type_ }
        }
        ConcreteValueShape::BitArray => {
            let local = execution::BitArrayFunctionLocalId(slots.bit_array_functions);
            slots.bit_array_functions += 1;
            SpecializedFunctionLocal::BitArray { local, type_ }
        }
        ConcreteValueShape::UtfCodepoint => {
            let local = execution::UtfCodepointFunctionLocalId(slots.utf_codepoint_functions);
            slots.utf_codepoint_functions += 1;
            SpecializedFunctionLocal::UtfCodepoint { local, type_ }
        }
        ConcreteValueShape::Custom(custom) => {
            let type_ = context.custom_function_type(crate::plan::CustomFunctionType::from_shapes(
                shape
                    .arguments()
                    .iter()
                    .map(ConcreteValueShape::to_module_shape)
                    .collect(),
                custom.to_module_shape(),
            ));
            let local = execution::CustomFunctionLocal::new(
                execution::CustomFunctionLocalId(next_custom_function_id(&slots.custom_functions)),
                type_,
            );
            slots.custom_functions.push(local.clone());
            SpecializedFunctionLocal::Custom(local)
        }
        ConcreteValueShape::Bool => {
            let local = execution::BoolFunctionLocalId(slots.bool_functions);
            slots.bool_functions += 1;
            SpecializedFunctionLocal::Bool { local, type_ }
        }
        ConcreteValueShape::Nil => {
            let local = execution::NilFunctionLocalId(slots.nil_functions);
            slots.nil_functions += 1;
            SpecializedFunctionLocal::Nil { local, type_ }
        }
        ConcreteValueShape::Tuple(_) => {
            let local = execution::TupleFunctionLocalId(slots.tuple_functions);
            slots.tuple_functions += 1;
            SpecializedFunctionLocal::Tuple { local, type_ }
        }
        ConcreteValueShape::List(item) => {
            let local = allocate_list_function_local(item, type_, &slots.list_functions, context);
            slots.list_functions.push(local.clone());
            SpecializedFunctionLocal::List(local)
        }
        ConcreteValueShape::Function(returned) => {
            let type_ =
                context.function_function_type(crate::plan::FunctionFunctionType::from_shapes(
                    shape
                        .arguments()
                        .iter()
                        .map(ConcreteValueShape::to_module_shape)
                        .collect(),
                    returned.to_module_shape(),
                ));
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
    item: &ConcreteValueShape,
    type_: execution::FunctionType,
    locals: &[execution::ListFunctionLocal],
    context: &mut LoweringContext,
) -> execution::ListFunctionLocal {
    use execution::ListFunctionLocal as L;

    match item {
        ConcreteValueShape::Int => L::Int {
            local: execution::IntListFunctionLocalId(next_list_function_id(locals, |local| {
                match local {
                    L::Int { local, .. } => Some(local.0),
                    _ => None,
                }
            })),
            type_,
            list_type: context.int_list_type(),
        },
        ConcreteValueShape::String => L::String {
            local: execution::StringListFunctionLocalId(next_list_function_id(locals, |local| {
                match local {
                    L::String { local, .. } => Some(local.0),
                    _ => None,
                }
            })),
            type_,
            list_type: context.string_list_type(),
        },
        ConcreteValueShape::BitArray => L::BitArray {
            local: execution::BitArrayListFunctionLocalId(next_list_function_id(locals, |local| {
                match local {
                    L::BitArray { local, .. } => Some(local.0),
                    _ => None,
                }
            })),
            type_,
            list_type: context.bit_array_list_type(),
        },
        ConcreteValueShape::UtfCodepoint => L::UtfCodepoint {
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
        ConcreteValueShape::Custom(custom) => L::Custom {
            local: execution::CustomListFunctionLocalId(next_list_function_id(locals, |local| {
                match local {
                    L::Custom { local, .. } => Some(local.0),
                    _ => None,
                }
            })),
            type_,
            list_type: context.custom_list_type(custom.to_module_shape().type_().clone()),
        },
        ConcreteValueShape::Float => L::Float {
            local: execution::FloatListFunctionLocalId(next_list_function_id(locals, |local| {
                match local {
                    L::Float { local, .. } => Some(local.0),
                    _ => None,
                }
            })),
            type_,
            list_type: context.float_list_type(),
        },
        ConcreteValueShape::Bool => L::Bool {
            local: execution::BoolListFunctionLocalId(next_list_function_id(locals, |local| {
                match local {
                    L::Bool { local, .. } => Some(local.0),
                    _ => None,
                }
            })),
            type_,
            list_type: context.bool_list_type(),
        },
        ConcreteValueShape::Nil => L::Nil {
            local: execution::NilListFunctionLocalId(next_list_function_id(locals, |local| {
                match local {
                    L::Nil { local, .. } => Some(local.0),
                    _ => None,
                }
            })),
            type_,
            list_type: context.nil_list_type(),
        },
        ConcreteValueShape::Tuple(elements) => L::Tuple {
            local: execution::TupleListFunctionLocalId(next_list_function_id(locals, |local| {
                match local {
                    L::Tuple { local, .. } => Some(local.0),
                    _ => None,
                }
            })),
            type_,
            list_type: context.tuple_list_type(
                elements
                    .iter()
                    .map(ConcreteValueShape::value_type)
                    .collect(),
            ),
        },
        ConcreteValueShape::List(item) => L::List {
            local: execution::ListListFunctionLocalId(next_list_function_id(locals, |local| {
                match local {
                    L::List { local, .. } => Some(local.0),
                    _ => None,
                }
            })),
            type_,
            list_type: context.list_list_type(item.value_type()),
        },
        ConcreteValueShape::Function(function) => L::Function {
            local: execution::FunctionListFunctionLocalId(next_list_function_id(locals, |local| {
                match local {
                    L::Function { local, .. } => Some(local.0),
                    _ => None,
                }
            })),
            type_,
            list_type: context.function_list_type(function.to_module_shape().type_()),
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
    use crate::plan::{FunctionType, ValueType};

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

pub fn main() { 0 }
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
    }
}
