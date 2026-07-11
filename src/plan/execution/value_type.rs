use std::collections::HashMap;

use crate::plan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ListTypeId(usize);

macro_rules! primitive_list_type_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub(crate) struct $name {
            list_type: ListTypeId,
        }
    };
}

primitive_list_type_id!(IntListTypeId);
primitive_list_type_id!(StringListTypeId);
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
pub(crate) enum ListStorageTypeId {
    Int(IntListTypeId),
    String(StringListTypeId),
    Float(FloatListTypeId),
    Bool(BoolListTypeId),
    Nil(NilListTypeId),
    Tuple(TupleListTypeId),
    List(ListListTypeId),
    Function(FunctionListTypeId),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ValueType {
    Int,
    Float,
    String,
    Bool,
    Nil,
    Tuple(Vec<ValueType>),
    List(ListTypeId),
    Function(Box<FunctionType>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct FunctionType {
    arguments: Vec<ValueType>,
    return_: Box<ValueType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ListType {
    item: ValueType,
    storage: ListStorageTypeId,
}

#[derive(Default)]
pub(crate) struct ListTypeTable {
    types: Vec<ListType>,
    tuple_items: Vec<Vec<ValueType>>,
    function_items: Vec<FunctionType>,
}

#[derive(Default)]
pub(super) struct ListTypeInterner {
    types: Vec<ListType>,
    ids: HashMap<plan::ValueType, ListTypeId>,
    tuple_ids: HashMap<plan::ValueType, TupleListTypeId>,
    list_ids: HashMap<plan::ValueType, ListListTypeId>,
    function_ids: HashMap<plan::ValueType, FunctionListTypeId>,
    tuple_items: Vec<Vec<ValueType>>,
    function_items: Vec<FunctionType>,
}

impl ListTypeId {
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
list_type_id!(FloatListTypeId);
list_type_id!(BoolListTypeId);
list_type_id!(NilListTypeId);
list_type_id!(TupleListTypeId);
list_type_id!(ListListTypeId);
list_type_id!(FunctionListTypeId);

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
    pub(crate) fn item_type(self) -> ListTypeId {
        self.item_type
    }
}

impl ListType {
    pub(crate) fn item(&self) -> &ValueType {
        &self.item
    }
}

impl ListTypeTable {
    #[cfg(test)]
    pub(super) fn entries(&self) -> impl Iterator<Item = (ListTypeId, &ListType)> {
        self.types
            .iter()
            .enumerate()
            .map(|(index, type_)| (ListTypeId(index), type_))
    }

    pub(crate) fn get(&self, id: ListTypeId) -> &ListType {
        &self.types[id.index()]
    }

    pub(crate) fn storage_type(&self, id: ListTypeId) -> ListStorageTypeId {
        self.get(id).storage
    }

    pub(crate) fn value_type(&self, value: &ValueType) -> plan::ValueType {
        match value {
            ValueType::Int => plan::ValueType::Int,
            ValueType::Float => plan::ValueType::Float,
            ValueType::String => plan::ValueType::String,
            ValueType::Bool => plan::ValueType::Bool,
            ValueType::Nil => plan::ValueType::Nil,
            ValueType::Tuple(elements) => plan::ValueType::Tuple(
                elements
                    .iter()
                    .map(|element| self.value_type(element))
                    .collect(),
            ),
            ValueType::List(id) => self.list_value_type(*id),
            ValueType::Function(type_) => {
                plan::ValueType::Function(Box::new(self.function_type(type_)))
            }
        }
    }

    pub(crate) fn function_type(&self, type_: &FunctionType) -> plan::FunctionType {
        plan::FunctionType::new(
            type_
                .argument_types()
                .iter()
                .map(|argument| self.value_type(argument))
                .collect(),
            self.value_type(type_.return_()),
        )
    }

    pub(crate) fn list_value_type(&self, id: ListTypeId) -> plan::ValueType {
        plan::ValueType::List(Box::new(self.item_value_type(id)))
    }

    pub(crate) fn item_value_type(&self, id: ListTypeId) -> plan::ValueType {
        self.value_type(self.get(id).item())
    }

    pub(crate) fn tuple_item_type(&self, id: TupleListTypeId) -> Vec<plan::ValueType> {
        self.tuple_items[id.item_type.0]
            .iter()
            .map(|type_| self.value_type(type_))
            .collect()
    }

    pub(crate) fn nested_list_item_type(&self, id: ListListTypeId) -> plan::ValueType {
        self.item_value_type(id.item_type)
    }

    pub(crate) fn function_item_type(&self, id: FunctionListTypeId) -> plan::FunctionType {
        self.function_type(&self.function_items[id.item_type.0])
    }
}

impl ListTypeInterner {
    pub(super) fn value_type(&mut self, value: plan::ValueType) -> ValueType {
        match value {
            plan::ValueType::Int => ValueType::Int,
            plan::ValueType::Float => ValueType::Float,
            plan::ValueType::String => ValueType::String,
            plan::ValueType::Bool => ValueType::Bool,
            plan::ValueType::Nil => ValueType::Nil,
            plan::ValueType::Tuple(elements) => ValueType::Tuple(
                elements
                    .into_iter()
                    .map(|element| self.value_type(element))
                    .collect(),
            ),
            plan::ValueType::List(item) => ValueType::List(self.list_type(*item)),
            plan::ValueType::Function(type_) => {
                ValueType::Function(Box::new(self.function_type(*type_)))
            }
        }
    }

    pub(super) fn function_type(&mut self, type_: plan::FunctionType) -> FunctionType {
        FunctionType::new(
            type_
                .argument_types()
                .iter()
                .cloned()
                .map(|argument| self.value_type(argument))
                .collect(),
            self.value_type(type_.return_().clone()),
        )
    }

    pub(super) fn list_type(&mut self, item: plan::ValueType) -> ListTypeId {
        match item {
            plan::ValueType::Int => self.int_list_type().list_type(),
            plan::ValueType::String => self.string_list_type().list_type(),
            plan::ValueType::Float => self.float_list_type().list_type(),
            plan::ValueType::Bool => self.bool_list_type().list_type(),
            plan::ValueType::Nil => self.nil_list_type().list_type(),
            plan::ValueType::Tuple(item) => self.tuple_list_type(item).list_type(),
            plan::ValueType::List(item) => self.list_list_type(*item).list_type(),
            plan::ValueType::Function(item) => self.function_list_type(*item).list_type(),
        }
    }

    fn intern_primitive(
        &mut self,
        plan_item: plan::ValueType,
        item: ValueType,
        storage: impl FnOnce(ListTypeId) -> ListStorageTypeId,
    ) -> ListTypeId {
        let type_ = plan::ValueType::List(Box::new(plan_item));
        if let Some(id) = self.ids.get(&type_) {
            return *id;
        }

        let id = ListTypeId(self.types.len());
        self.types.push(ListType {
            item,
            storage: storage(id),
        });
        self.ids.insert(type_, id);
        id
    }

    pub(super) fn int_list_type(&mut self) -> IntListTypeId {
        let list_type = self.intern_primitive(plan::ValueType::Int, ValueType::Int, |list_type| {
            ListStorageTypeId::Int(IntListTypeId { list_type })
        });
        IntListTypeId { list_type }
    }

    pub(super) fn string_list_type(&mut self) -> StringListTypeId {
        let list_type =
            self.intern_primitive(plan::ValueType::String, ValueType::String, |list_type| {
                ListStorageTypeId::String(StringListTypeId { list_type })
            });
        StringListTypeId { list_type }
    }

    pub(super) fn float_list_type(&mut self) -> FloatListTypeId {
        let list_type =
            self.intern_primitive(plan::ValueType::Float, ValueType::Float, |list_type| {
                ListStorageTypeId::Float(FloatListTypeId { list_type })
            });
        FloatListTypeId { list_type }
    }

    pub(super) fn bool_list_type(&mut self) -> BoolListTypeId {
        let list_type =
            self.intern_primitive(plan::ValueType::Bool, ValueType::Bool, |list_type| {
                ListStorageTypeId::Bool(BoolListTypeId { list_type })
            });
        BoolListTypeId { list_type }
    }

    pub(super) fn nil_list_type(&mut self) -> NilListTypeId {
        let list_type = self.intern_primitive(plan::ValueType::Nil, ValueType::Nil, |list_type| {
            ListStorageTypeId::Nil(NilListTypeId { list_type })
        });
        NilListTypeId { list_type }
    }

    pub(super) fn tuple_list_type(&mut self, item: Vec<plan::ValueType>) -> TupleListTypeId {
        let type_ = plan::ValueType::List(Box::new(plan::ValueType::Tuple(item.clone())));
        if let Some(id) = self.tuple_ids.get(&type_) {
            return *id;
        }

        let item = item
            .into_iter()
            .map(|type_| self.value_type(type_))
            .collect::<Vec<_>>();
        let list_type = ListTypeId(self.types.len());
        let item_type = TupleItemTypeId(self.tuple_items.len());
        let id = TupleListTypeId {
            list_type,
            item_type,
        };
        self.tuple_items.push(item.clone());
        self.types.push(ListType {
            item: ValueType::Tuple(item),
            storage: ListStorageTypeId::Tuple(id),
        });
        self.ids.insert(type_.clone(), list_type);
        self.tuple_ids.insert(type_, id);
        id
    }

    pub(super) fn list_list_type(&mut self, item: plan::ValueType) -> ListListTypeId {
        let type_ = plan::ValueType::List(Box::new(plan::ValueType::List(Box::new(item.clone()))));
        if let Some(id) = self.list_ids.get(&type_) {
            return *id;
        }

        let item_type = self.list_type(item);
        let list_type = ListTypeId(self.types.len());
        let id = ListListTypeId {
            list_type,
            item_type,
        };
        self.types.push(ListType {
            item: ValueType::List(item_type),
            storage: ListStorageTypeId::List(id),
        });
        self.ids.insert(type_.clone(), list_type);
        self.list_ids.insert(type_, id);
        id
    }

    pub(super) fn function_list_type(&mut self, item: plan::FunctionType) -> FunctionListTypeId {
        let type_ =
            plan::ValueType::List(Box::new(plan::ValueType::Function(Box::new(item.clone()))));
        if let Some(id) = self.function_ids.get(&type_) {
            return *id;
        }

        let item = self.function_type(item);
        let list_type = ListTypeId(self.types.len());
        let item_type = FunctionItemTypeId(self.function_items.len());
        let id = FunctionListTypeId {
            list_type,
            item_type,
        };
        self.function_items.push(item.clone());
        self.types.push(ListType {
            item: ValueType::Function(Box::new(item)),
            storage: ListStorageTypeId::Function(id),
        });
        self.ids.insert(type_.clone(), list_type);
        self.function_ids.insert(type_, id);
        id
    }

    pub(super) fn finish(self) -> ListTypeTable {
        ListTypeTable {
            types: self.types,
            tuple_items: self.tuple_items,
            function_items: self.function_items,
        }
    }
}
