use super::{
    CustomTypeId, CustomTypeTable, ExternalTypeId, ExternalTypeTable, FunctionType, ValueType,
};
use crate::plan;
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ListTypeId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntListTypeId {
    pub list_type: ListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringListTypeId {
    pub list_type: ListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitArrayListTypeId {
    pub list_type: ListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UtfCodepointListTypeId {
    pub list_type: ListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FloatListTypeId {
    pub list_type: ListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoolListTypeId {
    pub list_type: ListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NilListTypeId {
    pub list_type: ListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParameterListTypeId {
    pub list_type: ListTypeId,
    pub item: plan::TypeParameterId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TupleItemTypeId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionItemTypeId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TupleListTypeId {
    pub list_type: ListTypeId,
    pub item_type: TupleItemTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ListListTypeId {
    pub list_type: ListTypeId,
    pub item_type: ListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParameterListListTypeId {
    pub list_type: ListTypeId,
    pub item_type: ParameterListTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionListTypeId {
    pub list_type: ListTypeId,
    pub item_type: FunctionItemTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomListTypeId {
    pub list_type: ListTypeId,
    pub item_type: CustomTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExternalListTypeId {
    pub list_type: ListTypeId,
    pub item_type: ExternalTypeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ListStorageTypeId {
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

#[derive(Clone)]
pub struct ListTypeTable {
    pub types: Table<ListStorageTypeId>,
    pub tuple_items: Table<Table<ValueType>>,
    pub function_items: Table<FunctionType>,
}

impl ListStorageTypeId {
    pub(in crate::plan::execution) fn list_type(self) -> ListTypeId {
        match self {
            Self::Parameter(id) => id.list_type,
            Self::Int(id) => id.list_type,
            Self::String(id) => id.list_type,
            Self::BitArray(id) => id.list_type,
            Self::UtfCodepoint(id) => id.list_type,
            Self::Float(id) => id.list_type,
            Self::Bool(id) => id.list_type,
            Self::Nil(id) => id.list_type,
            Self::Tuple(id) => id.list_type,
            Self::ParameterList(id) => id.list_type,
            Self::List(id) => id.list_type,
            Self::Function(id) => id.list_type,
            Self::Custom(id) => id.list_type,
            Self::External(id) => id.list_type,
        }
    }
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
            types: types.into(),
            tuple_items: tuple_items.into_iter().map(Table::from).collect(),
            function_items: function_items.into(),
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

impl Default for ListTypeTable {
    fn default() -> Self {
        Self::from_parts(Vec::new(), Vec::new(), Vec::new())
    }
}

impl Emit for ListTypeId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("type_::ListTypeId", &[field_0]);
    }
}

impl Emit for IntListTypeId {
    fn emit(&self, output: &mut Rust) {
        let Self { list_type } = self;
        output.structure("type_::IntListTypeId", &[("list_type", list_type)]);
    }
}

impl Emit for StringListTypeId {
    fn emit(&self, output: &mut Rust) {
        let Self { list_type } = self;
        output.structure("type_::StringListTypeId", &[("list_type", list_type)]);
    }
}

impl Emit for BitArrayListTypeId {
    fn emit(&self, output: &mut Rust) {
        let Self { list_type } = self;
        output.structure("type_::BitArrayListTypeId", &[("list_type", list_type)]);
    }
}

impl Emit for UtfCodepointListTypeId {
    fn emit(&self, output: &mut Rust) {
        let Self { list_type } = self;
        output.structure("type_::UtfCodepointListTypeId", &[("list_type", list_type)]);
    }
}

impl Emit for FloatListTypeId {
    fn emit(&self, output: &mut Rust) {
        let Self { list_type } = self;
        output.structure("type_::FloatListTypeId", &[("list_type", list_type)]);
    }
}

impl Emit for BoolListTypeId {
    fn emit(&self, output: &mut Rust) {
        let Self { list_type } = self;
        output.structure("type_::BoolListTypeId", &[("list_type", list_type)]);
    }
}

impl Emit for NilListTypeId {
    fn emit(&self, output: &mut Rust) {
        let Self { list_type } = self;
        output.structure("type_::NilListTypeId", &[("list_type", list_type)]);
    }
}

impl Emit for ParameterListTypeId {
    fn emit(&self, output: &mut Rust) {
        let Self { list_type, item } = self;
        output.structure(
            "type_::ParameterListTypeId",
            &[("list_type", list_type), ("item", item)],
        );
    }
}

impl Emit for TupleItemTypeId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("type_::TupleItemTypeId", &[field_0]);
    }
}

impl Emit for FunctionItemTypeId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("type_::FunctionItemTypeId", &[field_0]);
    }
}

impl Emit for TupleListTypeId {
    fn emit(&self, output: &mut Rust) {
        let Self {
            list_type,
            item_type,
        } = self;
        output.structure(
            "type_::TupleListTypeId",
            &[("list_type", list_type), ("item_type", item_type)],
        );
    }
}

impl Emit for ListListTypeId {
    fn emit(&self, output: &mut Rust) {
        let Self {
            list_type,
            item_type,
        } = self;
        output.structure(
            "type_::ListListTypeId",
            &[("list_type", list_type), ("item_type", item_type)],
        );
    }
}

impl Emit for ParameterListListTypeId {
    fn emit(&self, output: &mut Rust) {
        let Self {
            list_type,
            item_type,
        } = self;
        output.structure(
            "type_::ParameterListListTypeId",
            &[("list_type", list_type), ("item_type", item_type)],
        );
    }
}

impl Emit for FunctionListTypeId {
    fn emit(&self, output: &mut Rust) {
        let Self {
            list_type,
            item_type,
        } = self;
        output.structure(
            "type_::FunctionListTypeId",
            &[("list_type", list_type), ("item_type", item_type)],
        );
    }
}

impl Emit for CustomListTypeId {
    fn emit(&self, output: &mut Rust) {
        let Self {
            list_type,
            item_type,
        } = self;
        output.structure(
            "type_::CustomListTypeId",
            &[("list_type", list_type), ("item_type", item_type)],
        );
    }
}

impl Emit for ExternalListTypeId {
    fn emit(&self, output: &mut Rust) {
        let Self {
            list_type,
            item_type,
        } = self;
        output.structure(
            "type_::ExternalListTypeId",
            &[("list_type", list_type), ("item_type", item_type)],
        );
    }
}

impl Emit for ListStorageTypeId {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Parameter(field_0) => {
                output.call("type_::ListStorageTypeId::Parameter", &[field_0])
            }
            Self::Int(field_0) => output.call("type_::ListStorageTypeId::Int", &[field_0]),
            Self::String(field_0) => output.call("type_::ListStorageTypeId::String", &[field_0]),
            Self::BitArray(field_0) => {
                output.call("type_::ListStorageTypeId::BitArray", &[field_0])
            }
            Self::UtfCodepoint(field_0) => {
                output.call("type_::ListStorageTypeId::UtfCodepoint", &[field_0])
            }
            Self::Float(field_0) => output.call("type_::ListStorageTypeId::Float", &[field_0]),
            Self::Bool(field_0) => output.call("type_::ListStorageTypeId::Bool", &[field_0]),
            Self::Nil(field_0) => output.call("type_::ListStorageTypeId::Nil", &[field_0]),
            Self::Tuple(field_0) => output.call("type_::ListStorageTypeId::Tuple", &[field_0]),
            Self::ParameterList(field_0) => {
                output.call("type_::ListStorageTypeId::ParameterList", &[field_0])
            }
            Self::List(field_0) => output.call("type_::ListStorageTypeId::List", &[field_0]),
            Self::Function(field_0) => {
                output.call("type_::ListStorageTypeId::Function", &[field_0])
            }
            Self::Custom(field_0) => output.call("type_::ListStorageTypeId::Custom", &[field_0]),
            Self::External(field_0) => {
                output.call("type_::ListStorageTypeId::External", &[field_0])
            }
        }
    }
}

impl Emit for ListTypeTable {
    fn emit(&self, output: &mut Rust) {
        let Self {
            types,
            tuple_items,
            function_items,
        } = self;
        output.structure(
            "type_::ListTypeTable",
            &[
                ("types", types),
                ("tuple_items", tuple_items),
                ("function_items", function_items),
            ],
        );
    }
}
