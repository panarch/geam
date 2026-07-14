use crate::plan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ListTypeId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct CustomTypeId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct CustomConstructorId {
    type_id: CustomTypeId,
    index: usize,
}

macro_rules! primitive_list_type_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub(crate) struct $name {
            list_type: ListTypeId,
        }

        impl $name {
            pub(super) fn new(list_type: ListTypeId) -> Self {
                Self { list_type }
            }
        }
    };
}

primitive_list_type_id!(IntListTypeId);
primitive_list_type_id!(StringListTypeId);
primitive_list_type_id!(BitArrayListTypeId);
primitive_list_type_id!(UtfCodepointListTypeId);
primitive_list_type_id!(FloatListTypeId);
primitive_list_type_id!(BoolListTypeId);
primitive_list_type_id!(NilListTypeId);

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
pub(crate) enum ListStorageTypeId {
    Int(IntListTypeId),
    String(StringListTypeId),
    BitArray(BitArrayListTypeId),
    UtfCodepoint(UtfCodepointListTypeId),
    Float(FloatListTypeId),
    Bool(BoolListTypeId),
    Nil(NilListTypeId),
    Tuple(TupleListTypeId),
    List(ListListTypeId),
    Function(FunctionListTypeId),
    Custom(CustomListTypeId),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ValueType {
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    Tuple(Vec<ValueType>),
    List(ListTypeId),
    Function(Box<FunctionType>),
    Custom(CustomTypeId),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct FunctionType {
    arguments: Vec<ValueType>,
    return_: Box<ValueType>,
}

#[derive(Default)]
pub(super) struct ListTypeTable {
    types: Vec<ListStorageTypeId>,
    tuple_items: Vec<Vec<ValueType>>,
    function_items: Vec<FunctionType>,
}

impl ListTypeId {
    pub(super) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

macro_rules! list_type_id {
    ($name:ident) => {
        impl $name {
            pub(crate) fn list_type(self) -> ListTypeId {
                self.list_type
            }
        }
    };
}

list_type_id!(IntListTypeId);
list_type_id!(StringListTypeId);
list_type_id!(BitArrayListTypeId);
list_type_id!(UtfCodepointListTypeId);
list_type_id!(FloatListTypeId);
list_type_id!(BoolListTypeId);
list_type_id!(NilListTypeId);
list_type_id!(TupleListTypeId);
list_type_id!(ListListTypeId);
list_type_id!(FunctionListTypeId);
list_type_id!(CustomListTypeId);

impl CustomTypeId {
    pub(super) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl CustomConstructorId {
    pub(super) fn new(type_id: CustomTypeId, index: usize) -> Self {
        Self { type_id, index }
    }

    pub(crate) fn type_id(self) -> CustomTypeId {
        self.type_id
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }
}

impl FunctionType {
    pub(crate) fn new(arguments: Vec<ValueType>, return_: ValueType) -> Self {
        Self {
            arguments,
            return_: Box::new(return_),
        }
    }

    pub(crate) fn return_(&self) -> &ValueType {
        &self.return_
    }

    pub(crate) fn argument_types(&self) -> &[ValueType] {
        &self.arguments
    }
}

impl ListListTypeId {
    pub(super) fn new(list_type: ListTypeId, item_type: ListTypeId) -> Self {
        Self {
            list_type,
            item_type,
        }
    }

    pub(crate) fn item_type(self) -> ListTypeId {
        self.item_type
    }
}

impl TupleListTypeId {
    pub(super) fn new(list_type: ListTypeId, item_index: usize) -> Self {
        Self {
            list_type,
            item_type: TupleItemTypeId(item_index),
        }
    }
}

impl FunctionListTypeId {
    pub(super) fn new(list_type: ListTypeId, item_index: usize) -> Self {
        Self {
            list_type,
            item_type: FunctionItemTypeId(item_index),
        }
    }
}

impl CustomListTypeId {
    pub(super) fn new(list_type: ListTypeId, item_type: CustomTypeId) -> Self {
        Self {
            list_type,
            item_type,
        }
    }

    pub(crate) fn item_type(self) -> CustomTypeId {
        self.item_type
    }
}

impl ListTypeTable {
    pub(super) fn from_parts(
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

    #[cfg(test)]
    pub(super) fn entries(&self) -> impl Iterator<Item = (ListTypeId, ListStorageTypeId)> + '_ {
        self.types
            .iter()
            .copied()
            .enumerate()
            .map(|(index, type_)| (ListTypeId(index), type_))
    }

    fn get(&self, id: ListTypeId) -> ListStorageTypeId {
        self.types[id.index()]
    }

    pub(crate) fn storage_type(&self, id: ListTypeId) -> ListStorageTypeId {
        self.get(id)
    }

    pub(crate) fn value_type(
        &self,
        value: &ValueType,
        custom_types: &super::custom_type::CustomTypeTable,
    ) -> plan::ValueType {
        match value {
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
                    .map(|element| self.value_type(element, custom_types))
                    .collect(),
            ),
            ValueType::List(id) => self.list_value_type(*id, custom_types),
            ValueType::Function(type_) => {
                plan::ValueType::Function(Box::new(self.function_type(type_, custom_types)))
            }
            ValueType::Custom(id) => plan::ValueType::Custom(custom_types.value_type(*id)),
        }
    }

    pub(crate) fn function_type(
        &self,
        type_: &FunctionType,
        custom_types: &super::custom_type::CustomTypeTable,
    ) -> plan::FunctionType {
        plan::FunctionType::new(
            type_
                .argument_types()
                .iter()
                .map(|argument| self.value_type(argument, custom_types))
                .collect(),
            self.value_type(type_.return_(), custom_types),
        )
    }

    pub(crate) fn list_value_type(
        &self,
        id: ListTypeId,
        custom_types: &super::custom_type::CustomTypeTable,
    ) -> plan::ValueType {
        plan::ValueType::List(Box::new(self.item_value_type(id, custom_types)))
    }

    pub(crate) fn item_value_type(
        &self,
        id: ListTypeId,
        custom_types: &super::custom_type::CustomTypeTable,
    ) -> plan::ValueType {
        match self.storage_type(id) {
            ListStorageTypeId::Int(_) => plan::ValueType::Int,
            ListStorageTypeId::String(_) => plan::ValueType::String,
            ListStorageTypeId::BitArray(_) => plan::ValueType::BitArray,
            ListStorageTypeId::UtfCodepoint(_) => plan::ValueType::UtfCodepoint,
            ListStorageTypeId::Float(_) => plan::ValueType::Float,
            ListStorageTypeId::Bool(_) => plan::ValueType::Bool,
            ListStorageTypeId::Nil(_) => plan::ValueType::Nil,
            ListStorageTypeId::Tuple(id) => {
                plan::ValueType::Tuple(self.tuple_item_type(id, custom_types))
            }
            ListStorageTypeId::List(id) => {
                plan::ValueType::List(Box::new(self.nested_list_item_type(id, custom_types)))
            }
            ListStorageTypeId::Function(id) => {
                plan::ValueType::Function(Box::new(self.function_item_type(id, custom_types)))
            }
            ListStorageTypeId::Custom(id) => {
                plan::ValueType::Custom(custom_types.value_type(id.item_type()))
            }
        }
    }

    pub(crate) fn tuple_item_type(
        &self,
        id: TupleListTypeId,
        custom_types: &super::custom_type::CustomTypeTable,
    ) -> Vec<plan::ValueType> {
        self.tuple_items[id.item_type.0]
            .iter()
            .map(|type_| self.value_type(type_, custom_types))
            .collect()
    }

    pub(crate) fn nested_list_item_type(
        &self,
        id: ListListTypeId,
        custom_types: &super::custom_type::CustomTypeTable,
    ) -> plan::ValueType {
        self.item_value_type(id.item_type, custom_types)
    }

    pub(crate) fn function_item_type(
        &self,
        id: FunctionListTypeId,
        custom_types: &super::custom_type::CustomTypeTable,
    ) -> plan::FunctionType {
        self.function_type(&self.function_items[id.item_type.0], custom_types)
    }
}
