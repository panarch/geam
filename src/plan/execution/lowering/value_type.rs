use std::collections::HashMap;

use crate::plan;
use crate::plan::execution::{
    BoolListTypeId, FloatListTypeId, FunctionListTypeId, FunctionType, IntListTypeId,
    ListListTypeId, ListStorageTypeId, ListTypeId, ListTypeTable, NilListTypeId, StringListTypeId,
    TupleListTypeId, ValueType,
};

use super::super::value_type::ListType;

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

        let id = ListTypeId::new(self.types.len());
        self.types.push(ListType::new(item, storage(id)));
        self.ids.insert(type_, id);
        id
    }

    pub(super) fn int_list_type(&mut self) -> IntListTypeId {
        let list_type = self.intern_primitive(plan::ValueType::Int, ValueType::Int, |list_type| {
            ListStorageTypeId::Int(IntListTypeId::new(list_type))
        });
        IntListTypeId::new(list_type)
    }

    pub(super) fn string_list_type(&mut self) -> StringListTypeId {
        let list_type =
            self.intern_primitive(plan::ValueType::String, ValueType::String, |list_type| {
                ListStorageTypeId::String(StringListTypeId::new(list_type))
            });
        StringListTypeId::new(list_type)
    }

    pub(super) fn float_list_type(&mut self) -> FloatListTypeId {
        let list_type =
            self.intern_primitive(plan::ValueType::Float, ValueType::Float, |list_type| {
                ListStorageTypeId::Float(FloatListTypeId::new(list_type))
            });
        FloatListTypeId::new(list_type)
    }

    pub(super) fn bool_list_type(&mut self) -> BoolListTypeId {
        let list_type =
            self.intern_primitive(plan::ValueType::Bool, ValueType::Bool, |list_type| {
                ListStorageTypeId::Bool(BoolListTypeId::new(list_type))
            });
        BoolListTypeId::new(list_type)
    }

    pub(super) fn nil_list_type(&mut self) -> NilListTypeId {
        let list_type = self.intern_primitive(plan::ValueType::Nil, ValueType::Nil, |list_type| {
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
        self.tuple_items.push(item.clone());
        self.types.push(ListType::new(
            ValueType::Tuple(item),
            ListStorageTypeId::Tuple(id),
        ));
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
        self.types.push(ListType::new(
            ValueType::List(item_type),
            ListStorageTypeId::List(id),
        ));
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
        self.function_items.push(item.clone());
        self.types.push(ListType::new(
            ValueType::Function(Box::new(item)),
            ListStorageTypeId::Function(id),
        ));
        self.ids.insert(type_.clone(), list_type);
        self.function_ids.insert(type_, id);
        id
    }

    pub(super) fn into_table(self) -> ListTypeTable {
        ListTypeTable::from_parts(self.types, self.tuple_items, self.function_items)
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::ExecutionPlan;
    use crate::plan::{FunctionType, ValueType};

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

pub fn main() { Nil }
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
}
