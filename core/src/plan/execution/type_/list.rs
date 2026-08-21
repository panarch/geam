use super::{
    CustomTypeId, CustomTypeTable, ExternalTypeId, ExternalTypeTable, FunctionType, ValueType,
};
use crate::plan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ListTypeId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct IntListTypeId {
    list_type: ListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct StringListTypeId {
    list_type: ListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct BitArrayListTypeId {
    list_type: ListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct UtfCodepointListTypeId {
    list_type: ListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct FloatListTypeId {
    list_type: ListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct BoolListTypeId {
    list_type: ListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct NilListTypeId {
    list_type: ListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ParameterListTypeId {
    list_type: ListTypeId,
    item: plan::TypeParameterId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TupleItemTypeId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct FunctionItemTypeId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct TupleListTypeId {
    list_type: ListTypeId,
    item_type: TupleItemTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ListListTypeId {
    list_type: ListTypeId,
    item_type: ListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ParameterListListTypeId {
    list_type: ListTypeId,
    item_type: ParameterListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct FunctionListTypeId {
    list_type: ListTypeId,
    item_type: FunctionItemTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct CustomListTypeId {
    list_type: ListTypeId,
    item_type: CustomTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ExternalListTypeId {
    list_type: ListTypeId,
    item_type: ExternalTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ListStorageTypeId {
    Parameter(ParameterListTypeId),
    Int(IntListTypeId),
    String(StringListTypeId),
    BitArray(BitArrayListTypeId),
    UtfCodepoint(UtfCodepointListTypeId),
    Float(FloatListTypeId),
    Bool(BoolListTypeId),
    Nil(NilListTypeId),
    Tuple(TupleListTypeId),
    ParameterList(ParameterListListTypeId),
    List(ListListTypeId),
    Function(FunctionListTypeId),
    Custom(CustomListTypeId),
    External(ExternalListTypeId),
}

#[derive(Default)]
pub(crate) struct ListTypeTable {
    types: Vec<ListStorageTypeId>,
    tuple_items: Vec<Vec<ValueType>>,
    function_items: Vec<FunctionType>,
}

impl ListTypeId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl IntListTypeId {
    pub(in crate::plan::execution) fn new(list_type: ListTypeId) -> Self {
        Self { list_type }
    }

    pub(crate) fn list_type(self) -> ListTypeId {
        self.list_type
    }
}

impl StringListTypeId {
    pub(in crate::plan::execution) fn new(list_type: ListTypeId) -> Self {
        Self { list_type }
    }

    pub(crate) fn list_type(self) -> ListTypeId {
        self.list_type
    }
}

impl BitArrayListTypeId {
    pub(in crate::plan::execution) fn new(list_type: ListTypeId) -> Self {
        Self { list_type }
    }

    pub(crate) fn list_type(self) -> ListTypeId {
        self.list_type
    }
}

impl UtfCodepointListTypeId {
    pub(in crate::plan::execution) fn new(list_type: ListTypeId) -> Self {
        Self { list_type }
    }

    pub(crate) fn list_type(self) -> ListTypeId {
        self.list_type
    }
}

impl FloatListTypeId {
    pub(in crate::plan::execution) fn new(list_type: ListTypeId) -> Self {
        Self { list_type }
    }

    pub(crate) fn list_type(self) -> ListTypeId {
        self.list_type
    }
}

impl BoolListTypeId {
    pub(in crate::plan::execution) fn new(list_type: ListTypeId) -> Self {
        Self { list_type }
    }

    pub(crate) fn list_type(self) -> ListTypeId {
        self.list_type
    }
}

impl NilListTypeId {
    pub(in crate::plan::execution) fn new(list_type: ListTypeId) -> Self {
        Self { list_type }
    }

    pub(crate) fn list_type(self) -> ListTypeId {
        self.list_type
    }
}

impl ParameterListTypeId {
    pub(in crate::plan::execution) fn new(
        list_type: ListTypeId,
        item: plan::TypeParameterId,
    ) -> Self {
        Self { list_type, item }
    }

    pub(crate) fn list_type(self) -> ListTypeId {
        self.list_type
    }

    pub(crate) fn item(self) -> plan::TypeParameterId {
        self.item
    }
}

impl TupleListTypeId {
    pub(in crate::plan::execution) fn new(list_type: ListTypeId, item_index: usize) -> Self {
        Self {
            list_type,
            item_type: TupleItemTypeId(item_index),
        }
    }

    pub(crate) fn list_type(self) -> ListTypeId {
        self.list_type
    }
}

impl ListListTypeId {
    pub(in crate::plan::execution) fn new(list_type: ListTypeId, item_type: ListTypeId) -> Self {
        Self {
            list_type,
            item_type,
        }
    }

    pub(crate) fn list_type(self) -> ListTypeId {
        self.list_type
    }

    #[cfg(test)]
    pub(crate) fn item_type(self) -> ListTypeId {
        self.item_type
    }
}

impl ParameterListListTypeId {
    pub(in crate::plan::execution) fn new(
        list_type: ListTypeId,
        item_type: ParameterListTypeId,
    ) -> Self {
        Self {
            list_type,
            item_type,
        }
    }

    pub(crate) fn list_type(self) -> ListTypeId {
        self.list_type
    }

    pub(crate) fn item_type(self) -> ParameterListTypeId {
        self.item_type
    }
}

impl FunctionListTypeId {
    pub(in crate::plan::execution) fn new(list_type: ListTypeId, item_index: usize) -> Self {
        Self {
            list_type,
            item_type: FunctionItemTypeId(item_index),
        }
    }

    pub(crate) fn list_type(self) -> ListTypeId {
        self.list_type
    }
}

impl CustomListTypeId {
    pub(in crate::plan::execution) fn new(list_type: ListTypeId, item_type: CustomTypeId) -> Self {
        Self {
            list_type,
            item_type,
        }
    }

    pub(crate) fn list_type(self) -> ListTypeId {
        self.list_type
    }

    pub(crate) fn item_type(self) -> CustomTypeId {
        self.item_type
    }
}

impl ExternalListTypeId {
    pub(in crate::plan::execution) fn new(
        list_type: ListTypeId,
        item_type: ExternalTypeId,
    ) -> Self {
        Self {
            list_type,
            item_type,
        }
    }

    pub(crate) fn list_type(self) -> ListTypeId {
        self.list_type
    }

    pub(crate) fn item_type(self) -> ExternalTypeId {
        self.item_type
    }
}

impl ListTypeTable {
    pub(in crate::plan::execution) fn from_parts(
        types: Vec<ListStorageTypeId>,
        tuple_items: Vec<Vec<ValueType>>,
        function_items: Vec<FunctionType>,
    ) -> Self {
        Self {
            types,
            tuple_items,
            function_items,
        }
    }

    pub(crate) fn storage_type(&self, id: ListTypeId) -> ListStorageTypeId {
        self.get(id)
    }

    fn get(&self, id: ListTypeId) -> ListStorageTypeId {
        self.types[id.index()]
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn entries(
        &self,
    ) -> impl Iterator<Item = (ListTypeId, ListStorageTypeId)> + '_ {
        self.types
            .iter()
            .copied()
            .enumerate()
            .map(|(index, type_)| (ListTypeId(index), type_))
    }

    pub(crate) fn value_type(
        &self,
        value: &ValueType,
        custom_types: &CustomTypeTable,
        external_types: &ExternalTypeTable,
    ) -> plan::ValueType {
        match value {
            ValueType::Parameter(parameter) => plan::ValueType::Parameter(*parameter),
            ValueType::Int => plan::ValueType::Int,
            ValueType::Float => plan::ValueType::Float,
            ValueType::String => plan::ValueType::String,
            ValueType::BitArray => plan::ValueType::BitArray,
            ValueType::UtfCodepoint => plan::ValueType::UtfCodepoint,
            ValueType::Bool => plan::ValueType::Bool,
            ValueType::Nil => plan::ValueType::Nil,
            ValueType::Tuple(elements) => plan::ValueType::Tuple(
                elements
                    .iter()
                    .map(|element| self.value_type(element, custom_types, external_types))
                    .collect(),
            ),
            ValueType::List(id) => self.list_value_type(*id, custom_types, external_types),
            ValueType::Function(type_) => plan::ValueType::Function(Box::new(self.function_type(
                type_,
                custom_types,
                external_types,
            ))),
            ValueType::Custom(id) => plan::ValueType::Custom(custom_types.value_type(*id)),
            ValueType::External(id) => plan::ValueType::External(external_types.value_type(*id)),
        }
    }

    pub(crate) fn function_type(
        &self,
        type_: &FunctionType,
        custom_types: &CustomTypeTable,
        external_types: &ExternalTypeTable,
    ) -> plan::FunctionType {
        plan::FunctionType::new(
            type_
                .argument_types()
                .iter()
                .map(|argument| self.value_type(argument, custom_types, external_types))
                .collect(),
            self.value_type(type_.return_(), custom_types, external_types),
        )
    }

    pub(crate) fn list_value_type(
        &self,
        id: ListTypeId,
        custom_types: &CustomTypeTable,
        external_types: &ExternalTypeTable,
    ) -> plan::ValueType {
        plan::ValueType::List(Box::new(self.item_value_type(
            id,
            custom_types,
            external_types,
        )))
    }

    pub(crate) fn item_value_type(
        &self,
        id: ListTypeId,
        custom_types: &CustomTypeTable,
        external_types: &ExternalTypeTable,
    ) -> plan::ValueType {
        match self.storage_type(id) {
            ListStorageTypeId::Parameter(id) => plan::ValueType::Parameter(id.item()),
            ListStorageTypeId::Int(_) => plan::ValueType::Int,
            ListStorageTypeId::String(_) => plan::ValueType::String,
            ListStorageTypeId::BitArray(_) => plan::ValueType::BitArray,
            ListStorageTypeId::UtfCodepoint(_) => plan::ValueType::UtfCodepoint,
            ListStorageTypeId::Float(_) => plan::ValueType::Float,
            ListStorageTypeId::Bool(_) => plan::ValueType::Bool,
            ListStorageTypeId::Nil(_) => plan::ValueType::Nil,
            ListStorageTypeId::Tuple(id) => {
                plan::ValueType::Tuple(self.tuple_item_type(id, custom_types, external_types))
            }
            ListStorageTypeId::ParameterList(id) => {
                plan::ValueType::List(Box::new(plan::ValueType::Parameter(id.item_type().item())))
            }
            ListStorageTypeId::List(id) => plan::ValueType::List(Box::new(
                self.nested_list_item_type(id, custom_types, external_types),
            )),
            ListStorageTypeId::Function(id) => plan::ValueType::Function(Box::new(
                self.function_item_type(id, custom_types, external_types),
            )),
            ListStorageTypeId::Custom(id) => {
                plan::ValueType::Custom(custom_types.value_type(id.item_type()))
            }
            ListStorageTypeId::External(id) => {
                plan::ValueType::External(external_types.value_type(id.item_type()))
            }
        }
    }

    pub(crate) fn tuple_item_type(
        &self,
        id: TupleListTypeId,
        custom_types: &CustomTypeTable,
        external_types: &ExternalTypeTable,
    ) -> Vec<plan::ValueType> {
        self.tuple_items[id.item_type.0]
            .iter()
            .map(|type_| self.value_type(type_, custom_types, external_types))
            .collect()
    }

    pub(crate) fn nested_list_item_type(
        &self,
        id: ListListTypeId,
        custom_types: &CustomTypeTable,
        external_types: &ExternalTypeTable,
    ) -> plan::ValueType {
        self.item_value_type(id.item_type, custom_types, external_types)
    }

    pub(crate) fn function_item_type(
        &self,
        id: FunctionListTypeId,
        custom_types: &CustomTypeTable,
        external_types: &ExternalTypeTable,
    ) -> plan::FunctionType {
        self.function_type(
            &self.function_items[id.item_type.0],
            custom_types,
            external_types,
        )
    }
}
