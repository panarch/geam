use std::collections::HashMap;

use crate::plan;
use crate::plan::execution::{
    BitArrayListTypeId, BoolListTypeId, CustomConstructorId, CustomListTypeId, CustomTypeId,
    FloatListTypeId, FunctionListTypeId, FunctionType, IntListTypeId, ListListTypeId,
    ListStorageTypeId, ListTypeId, NilListTypeId, StringListTypeId, TupleListTypeId,
    UtfCodepointListTypeId, ValueType,
};

use super::super::custom_type::{
    CustomConstructorDescriptor, CustomFieldDescriptor, CustomTypeDescriptor, CustomTypeTable,
};
use super::super::value_type::ListTypeTable;

pub(super) struct TypeInterner {
    types: Vec<ListStorageTypeId>,
    ids: HashMap<plan::ValueType, ListTypeId>,
    tuple_ids: HashMap<plan::ValueType, TupleListTypeId>,
    list_ids: HashMap<plan::ValueType, ListListTypeId>,
    function_ids: HashMap<plan::ValueType, FunctionListTypeId>,
    custom_list_ids: HashMap<plan::ValueType, CustomListTypeId>,
    tuple_items: Vec<Vec<ValueType>>,
    function_items: Vec<FunctionType>,
    custom_ids: HashMap<plan::CustomType, CustomTypeId>,
    custom_types: Vec<CustomTypeDescriptor>,
}

impl TypeInterner {
    pub(super) fn new() -> Self {
        Self {
            types: Vec::new(),
            ids: HashMap::new(),
            tuple_ids: HashMap::new(),
            list_ids: HashMap::new(),
            function_ids: HashMap::new(),
            custom_list_ids: HashMap::new(),
            tuple_items: Vec::new(),
            function_items: Vec::new(),
            custom_ids: HashMap::new(),
            custom_types: Vec::new(),
        }
    }

    pub(super) fn value_type(&mut self, value: plan::ValueType) -> ValueType {
        match value {
            plan::ValueType::Int => ValueType::Int,
            plan::ValueType::Float => ValueType::Float,
            plan::ValueType::String => ValueType::String,
            plan::ValueType::BitArray => ValueType::BitArray,
            plan::ValueType::UtfCodepoint => ValueType::UtfCodepoint,
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
            plan::ValueType::Custom(type_) => ValueType::Custom(self.custom_type(type_)),
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
            plan::ValueType::BitArray => self.bit_array_list_type().list_type(),
            plan::ValueType::UtfCodepoint => self.utf_codepoint_list_type().list_type(),
            plan::ValueType::Float => self.float_list_type().list_type(),
            plan::ValueType::Bool => self.bool_list_type().list_type(),
            plan::ValueType::Nil => self.nil_list_type().list_type(),
            plan::ValueType::Tuple(item) => self.tuple_list_type(item).list_type(),
            plan::ValueType::List(item) => self.list_list_type(*item).list_type(),
            plan::ValueType::Function(item) => self.function_list_type(*item).list_type(),
            plan::ValueType::Custom(item) => self.custom_list_type(item).list_type(),
        }
    }

    fn intern_primitive(
        &mut self,
        plan_item: plan::ValueType,
        storage: impl FnOnce(ListTypeId) -> ListStorageTypeId,
    ) -> ListTypeId {
        let type_ = plan::ValueType::List(Box::new(plan_item));
        if let Some(id) = self.ids.get(&type_) {
            return *id;
        }

        let id = ListTypeId::new(self.types.len());
        self.types.push(storage(id));
        self.ids.insert(type_, id);
        id
    }

    pub(super) fn int_list_type(&mut self) -> IntListTypeId {
        let list_type = self.intern_primitive(plan::ValueType::Int, |list_type| {
            ListStorageTypeId::Int(IntListTypeId::new(list_type))
        });
        IntListTypeId::new(list_type)
    }

    pub(super) fn string_list_type(&mut self) -> StringListTypeId {
        let list_type = self.intern_primitive(plan::ValueType::String, |list_type| {
            ListStorageTypeId::String(StringListTypeId::new(list_type))
        });
        StringListTypeId::new(list_type)
    }

    pub(super) fn bit_array_list_type(&mut self) -> BitArrayListTypeId {
        let list_type = self.intern_primitive(plan::ValueType::BitArray, |list_type| {
            ListStorageTypeId::BitArray(BitArrayListTypeId::new(list_type))
        });
        BitArrayListTypeId::new(list_type)
    }

    pub(super) fn utf_codepoint_list_type(&mut self) -> UtfCodepointListTypeId {
        let list_type = self.intern_primitive(plan::ValueType::UtfCodepoint, |list_type| {
            ListStorageTypeId::UtfCodepoint(UtfCodepointListTypeId::new(list_type))
        });
        UtfCodepointListTypeId::new(list_type)
    }

    pub(super) fn float_list_type(&mut self) -> FloatListTypeId {
        let list_type = self.intern_primitive(plan::ValueType::Float, |list_type| {
            ListStorageTypeId::Float(FloatListTypeId::new(list_type))
        });
        FloatListTypeId::new(list_type)
    }

    pub(super) fn bool_list_type(&mut self) -> BoolListTypeId {
        let list_type = self.intern_primitive(plan::ValueType::Bool, |list_type| {
            ListStorageTypeId::Bool(BoolListTypeId::new(list_type))
        });
        BoolListTypeId::new(list_type)
    }

    pub(super) fn nil_list_type(&mut self) -> NilListTypeId {
        let list_type = self.intern_primitive(plan::ValueType::Nil, |list_type| {
            ListStorageTypeId::Nil(NilListTypeId::new(list_type))
        });
        NilListTypeId::new(list_type)
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
        let list_type = ListTypeId::new(self.types.len());
        let id = TupleListTypeId::new(list_type, self.tuple_items.len());
        self.tuple_items.push(item);
        self.types.push(ListStorageTypeId::Tuple(id));
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
        let list_type = ListTypeId::new(self.types.len());
        let id = ListListTypeId::new(list_type, item_type);
        self.types.push(ListStorageTypeId::List(id));
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
        let list_type = ListTypeId::new(self.types.len());
        let id = FunctionListTypeId::new(list_type, self.function_items.len());
        self.function_items.push(item);
        self.types.push(ListStorageTypeId::Function(id));
        self.ids.insert(type_.clone(), list_type);
        self.function_ids.insert(type_, id);
        id
    }

    pub(super) fn custom_list_type(&mut self, item: plan::CustomType) -> CustomListTypeId {
        let type_ = plan::ValueType::List(Box::new(plan::ValueType::Custom(item.clone())));
        if let Some(id) = self.custom_list_ids.get(&type_) {
            return *id;
        }

        let item_type = self.custom_type(item);
        let list_type = ListTypeId::new(self.types.len());
        let id = CustomListTypeId::new(list_type, item_type);
        self.types.push(ListStorageTypeId::Custom(id));
        self.ids.insert(type_.clone(), list_type);
        self.custom_list_ids.insert(type_, id);
        id
    }

    pub(super) fn custom_type(&mut self, type_: plan::CustomType) -> CustomTypeId {
        if let Some(id) = self.custom_ids.get(&type_) {
            return *id;
        }

        let id = CustomTypeId::new(self.custom_types.len());
        self.custom_ids.insert(type_.clone(), id);
        self.custom_types.push(CustomTypeDescriptor::new(type_));
        id
    }

    pub(super) fn custom_constructor(
        &mut self,
        constructor: plan::CustomConstructor,
    ) -> CustomConstructorId {
        let (type_, name, index, fields) = constructor.into_parts();
        let type_id = self.custom_type(type_);
        let id = CustomConstructorId::new(type_id, index);
        if self.custom_types[type_id.index()].has_constructor(index) {
            return id;
        }

        let fields = fields
            .into_iter()
            .map(|field| {
                let (label, type_) = field.into_parts();
                CustomFieldDescriptor::new(label, self.value_type(type_))
            })
            .collect();
        self.custom_types[type_id.index()]
            .insert_constructor(CustomConstructorDescriptor::new(id, name, fields));
        id
    }

    pub(super) fn into_tables(self) -> (ListTypeTable, CustomTypeTable) {
        (
            ListTypeTable::from_parts(self.types, self.tuple_items, self.function_items),
            CustomTypeTable::new(self.custom_types),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{
        CustomConstructorId, CustomListTypeId, CustomTypeId, ExecutionPlan, ListStorageTypeId,
        ListTypeId, ValueType as ExecutionValueType,
    };
    use crate::plan::{CustomType, CustomTypeName, FunctionType, ValueType};

    #[test]
    fn lowering_interns_recursive_list_types_child_first_and_deduplicates_them() {
        let source = r#"
fn preserve(
  int_list: List(Int),
  nested: List(List(Int)),
  deep: List(List(List(Int))),
  duplicate: List(Int),
  functions: List(fn(List(Int)) -> List(List(Int))),
) {
  #(int_list, nested, deep, duplicate, functions)
}

pub fn main() {
  preserve([], [], [], [], [])
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);
        let entries = plan
            .list_types
            .entries()
            .map(|(id, _)| (id.index(), plan.list_value_type(id)))
            .collect::<Vec<_>>();

        assert_eq!(
            entries,
            vec![
                (0, ValueType::List(Box::new(ValueType::Int))),
                (
                    1,
                    ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Int)))),
                ),
                (
                    2,
                    ValueType::List(Box::new(ValueType::List(Box::new(ValueType::List(
                        Box::new(ValueType::Int),
                    ))))),
                ),
                (
                    3,
                    ValueType::List(Box::new(ValueType::Function(Box::new(FunctionType::new(
                        vec![ValueType::List(Box::new(ValueType::Int))],
                        ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Int)))),
                    ),)))),
                ),
            ]
        );
    }

    #[test]
    fn lowering_materializes_compound_list_items_from_canonical_storage_entries() {
        let source = r#"
fn ints() -> List(Int) { [] }
fn tuples() -> List(#(List(Int), fn(List(Int)) -> List(List(Int)))) { [] }
fn lists() -> List(List(Int)) { [] }
fn functions() -> List(fn(List(Int)) -> List(List(Int))) { [] }
pub fn main() { Nil }
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);
        let tuple = plan.tuple_list_function_id(0).type_id();
        let list = plan.list_list_function_id(0).type_id();
        let function = plan.function_list_function_id(0).type_id();
        let int_list = ValueType::List(Box::new(ValueType::Int));
        let nested_list = ValueType::List(Box::new(int_list.clone()));
        let function_item = FunctionType::new(vec![int_list.clone()], nested_list.clone());
        let tuple_item = vec![
            int_list.clone(),
            ValueType::Function(Box::new(function_item.clone())),
        ];

        assert_eq!(
            plan.list_storage_type(tuple.list_type()),
            ListStorageTypeId::Tuple(tuple)
        );
        assert_eq!(plan.tuple_list_item_type(tuple), tuple_item.clone());
        assert_eq!(
            plan.list_value_type(tuple.list_type()),
            ValueType::List(Box::new(ValueType::Tuple(tuple_item)))
        );

        assert_eq!(
            plan.list_storage_type(list.list_type()),
            ListStorageTypeId::List(list)
        );
        assert_eq!(plan.nested_list_item_type(list), ValueType::Int);
        assert_eq!(plan.list_value_type(list.list_type()), nested_list);

        assert_eq!(
            plan.list_storage_type(function.list_type()),
            ListStorageTypeId::Function(function)
        );
        assert_eq!(
            plan.function_list_item_type(function),
            function_item.clone()
        );
        assert_eq!(
            plan.list_value_type(function.list_type()),
            ValueType::List(Box::new(ValueType::Function(Box::new(function_item))))
        );
    }

    #[test]
    fn lowering_registers_used_recursive_custom_types_and_interns_custom_lists() {
        let source = r#"
pub type Box(value) {
  Empty
  Full(value, Box(value))
}

pub type Left {
  Left(Right)
}

pub type Right {
  Right(Left)
  Stop
}

fn inspect(value: Box(Int), strings: List(Box(String)), left: Left) {
  case left {
    Left(Right(_)) -> Nil
    Left(Stop) -> Nil
    _ -> Nil
  }
}

pub fn main() {
  inspect(Full(1, Empty), [Full("one", Empty)], Left(Stop))
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        let string_empty =
            plan.custom_constructor(CustomConstructorId::new(CustomTypeId::new(0), 0));
        assert_eq!(string_empty.name(), &ecow::EcoString::from("Empty"));
        assert_eq!(string_empty.fields().len(), 0);
        let string_full =
            plan.custom_constructor(CustomConstructorId::new(CustomTypeId::new(0), 1));
        assert_eq!(string_full.name(), &ecow::EcoString::from("Full"));
        assert_eq!(
            string_full
                .fields()
                .iter()
                .map(|field| field.type_().clone())
                .collect::<Vec<_>>(),
            vec![
                ExecutionValueType::String,
                ExecutionValueType::Custom(CustomTypeId::new(0)),
            ],
        );

        let int_full = plan.custom_constructor(CustomConstructorId::new(CustomTypeId::new(1), 1));
        assert_eq!(
            int_full
                .fields()
                .iter()
                .map(|field| field.type_().clone())
                .collect::<Vec<_>>(),
            vec![
                ExecutionValueType::Int,
                ExecutionValueType::Custom(CustomTypeId::new(1)),
            ],
        );
        assert_eq!(
            plan.list_storage_type(ListTypeId::new(0)),
            ListStorageTypeId::Custom(CustomListTypeId::new(
                ListTypeId::new(0),
                CustomTypeId::new(0),
            )),
        );

        let left = plan.custom_constructor(CustomConstructorId::new(CustomTypeId::new(2), 0));
        let right = plan.custom_constructor(CustomConstructorId::new(CustomTypeId::new(3), 0));
        assert_eq!(
            left.fields()[0].type_(),
            &ExecutionValueType::Custom(CustomTypeId::new(3)),
        );
        assert_eq!(
            right.fields()[0].type_(),
            &ExecutionValueType::Custom(CustomTypeId::new(2)),
        );
        assert_eq!(
            plan.custom_constructor(CustomConstructorId::new(CustomTypeId::new(3), 1))
                .fields()
                .len(),
            0,
        );
        assert_eq!(
            plan.custom_value_type(CustomTypeId::new(1)),
            CustomType::new(
                CustomTypeName::new("geam".into(), "main".into(), "Box".into()),
                vec![ValueType::Int],
            ),
        );
    }

    #[test]
    fn lowering_registers_only_finitely_used_non_regular_recursive_types() {
        let source = r#"
pub type Grow(value) {
  Stop
  Grow(Grow(List(value)))
}

pub fn main() -> Grow(Int) {
  Grow(Stop)
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        assert_eq!(plan.custom_types.len(), 2);
        assert_eq!(
            plan.custom_value_type(CustomTypeId::new(0)),
            CustomType::new(
                CustomTypeName::new("geam".into(), "main".into(), "Grow".into()),
                vec![ValueType::Int],
            ),
        );
        let root = plan.custom_constructor(CustomConstructorId::new(CustomTypeId::new(0), 1));
        assert_eq!(
            root.fields()[0].type_(),
            &ExecutionValueType::Custom(CustomTypeId::new(1)),
        );
        let nested = plan.custom_constructor(CustomConstructorId::new(CustomTypeId::new(1), 0));
        assert_eq!(nested.name(), "Stop");
        assert_eq!(nested.fields().len(), 0);
    }
}
