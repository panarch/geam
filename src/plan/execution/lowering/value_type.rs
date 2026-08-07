use std::collections::HashMap;

use crate::plan;
use crate::plan::execution::type_::{
    BitArrayListTypeId, BoolListTypeId, CustomConstructorId, CustomFunctionType, CustomListTypeId,
    CustomTypeId, ExternalFunctionType, ExternalListTypeId, ExternalTypeId, FloatListTypeId,
    FunctionFunctionType, FunctionListTypeId, FunctionType, GenericFunctionType, IntListTypeId,
    ListListTypeId, ListStorageTypeId, ListTypeId, NilListTypeId, ParameterListListTypeId,
    ParameterListTypeId, StringListTypeId, TupleListTypeId, UtfCodepointListTypeId, ValueType,
};
use crate::plan::execution::type_::{
    CustomConstructorRefinement, CustomValueShape, CustomValueShapeId, FunctionShape, ValueShapeId,
};

use super::super::type_::{
    CustomConstructorDescriptor, CustomFieldDescriptor, CustomTypeDescriptor, CustomTypeTable,
    CustomValueShapeDescriptor, ExternalTypeTable, ListTypeTable, ValueShapeDescriptor,
    ValueShapeTable,
};
use super::specialization::{
    SpecializedCustomConstructor, SpecializedCustomValueShape, SpecializedExternalValueShape,
    SpecializedFunctionShape, SpecializedValueShape, StoredValueShape,
};

pub(super) struct TypeInterner {
    types: Vec<ListStorageTypeId>,
    ids: HashMap<SpecializedValueShape, ListTypeId>,
    tuple_ids: HashMap<SpecializedValueShape, TupleListTypeId>,
    parameter_list_ids: HashMap<plan::TypeParameterId, ParameterListListTypeId>,
    list_ids: HashMap<StoredValueShape, ListListTypeId>,
    function_ids: HashMap<SpecializedValueShape, FunctionListTypeId>,
    custom_list_ids: HashMap<plan::CustomType, CustomListTypeId>,
    external_list_ids: HashMap<plan::ExternalType, ExternalListTypeId>,
    tuple_items: Vec<Vec<ValueType>>,
    function_items: Vec<FunctionType>,
    custom_ids: HashMap<plan::CustomType, CustomTypeId>,
    custom_types: Vec<CustomTypeDescriptor>,
    external_ids: HashMap<plan::ExternalType, ExternalTypeId>,
    external_types: Vec<plan::ExternalType>,
    shape_ids: HashMap<SpecializedValueShape, ValueShapeId>,
    shapes: Vec<ValueShapeDescriptor>,
    shape_types: Vec<ValueType>,
    custom_shape_ids: HashMap<SpecializedCustomValueShape, CustomValueShapeId>,
    custom_shapes: Vec<CustomValueShapeDescriptor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum NestedListTypeId {
    Parameter(ParameterListListTypeId),
    Stored(ListListTypeId),
}

impl NestedListTypeId {
    pub(super) fn list_type(self) -> ListTypeId {
        match self {
            Self::Parameter(id) => id.list_type(),
            Self::Stored(id) => id.list_type(),
        }
    }
}

impl TypeInterner {
    pub(super) fn new() -> Self {
        Self {
            types: Vec::new(),
            ids: HashMap::new(),
            tuple_ids: HashMap::new(),
            parameter_list_ids: HashMap::new(),
            list_ids: HashMap::new(),
            function_ids: HashMap::new(),
            custom_list_ids: HashMap::new(),
            external_list_ids: HashMap::new(),
            tuple_items: Vec::new(),
            function_items: Vec::new(),
            custom_ids: HashMap::new(),
            custom_types: Vec::new(),
            external_ids: HashMap::new(),
            external_types: Vec::new(),
            shape_ids: HashMap::new(),
            shapes: Vec::new(),
            shape_types: Vec::new(),
            custom_shape_ids: HashMap::new(),
            custom_shapes: Vec::new(),
        }
    }

    pub(super) fn value_type(&mut self, value: &SpecializedValueShape) -> ValueType {
        match value {
            SpecializedValueShape::Parameter(parameter) => ValueType::Parameter(*parameter),
            SpecializedValueShape::Int => ValueType::Int,
            SpecializedValueShape::Float => ValueType::Float,
            SpecializedValueShape::String => ValueType::String,
            SpecializedValueShape::BitArray => ValueType::BitArray,
            SpecializedValueShape::UtfCodepoint => ValueType::UtfCodepoint,
            SpecializedValueShape::Bool => ValueType::Bool,
            SpecializedValueShape::Nil => ValueType::Nil,
            SpecializedValueShape::Tuple(elements) => ValueType::Tuple(
                elements
                    .iter()
                    .map(|element| self.value_type(element))
                    .collect(),
            ),
            SpecializedValueShape::List(item) => ValueType::List(self.list_type(item)),
            SpecializedValueShape::Function(type_) => {
                ValueType::Function(Box::new(self.function_type(type_)))
            }
            SpecializedValueShape::Custom(shape) => ValueType::Custom(self.custom_type(shape)),
            SpecializedValueShape::External(shape) => {
                ValueType::External(self.external_type(shape))
            }
        }
    }

    pub(super) fn value_shape(&mut self, shape: &SpecializedValueShape) -> ValueShapeId {
        if let Some(id) = self.shape_ids.get(shape) {
            return *id;
        }

        let key = shape.clone();
        let descriptor = match shape {
            SpecializedValueShape::Parameter(parameter) => {
                ValueShapeDescriptor::Parameter(*parameter)
            }
            SpecializedValueShape::Int => ValueShapeDescriptor::Int,
            SpecializedValueShape::Float => ValueShapeDescriptor::Float,
            SpecializedValueShape::String => ValueShapeDescriptor::String,
            SpecializedValueShape::BitArray => ValueShapeDescriptor::BitArray,
            SpecializedValueShape::UtfCodepoint => ValueShapeDescriptor::UtfCodepoint,
            SpecializedValueShape::Bool => ValueShapeDescriptor::Bool,
            SpecializedValueShape::Nil => ValueShapeDescriptor::Nil,
            SpecializedValueShape::Tuple(elements) => ValueShapeDescriptor::Tuple(
                elements
                    .iter()
                    .map(|element| self.value_shape(element))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
            SpecializedValueShape::List(item) => ValueShapeDescriptor::List(self.value_shape(item)),
            SpecializedValueShape::Function(type_) => ValueShapeDescriptor::Function {
                arguments: type_
                    .arguments()
                    .iter()
                    .map(|argument| self.value_shape(argument))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
                return_: self.value_shape(type_.return_()),
            },
            SpecializedValueShape::Custom(shape) => {
                ValueShapeDescriptor::Custom(self.custom_shape_id(shape))
            }
            SpecializedValueShape::External(shape) => {
                ValueShapeDescriptor::External(self.external_type(shape))
            }
        };
        let nominal = self.value_type(&key);
        let id = ValueShapeId::new(self.shapes.len());
        self.shapes.push(descriptor);
        self.shape_types.push(nominal);
        self.shape_ids.insert(key, id);
        id
    }

    pub(super) fn custom_value_shape(
        &mut self,
        shape: &SpecializedCustomValueShape,
    ) -> CustomValueShape {
        let type_id = self.custom_type(shape);
        let shape_id = self.custom_shape_id(shape);
        CustomValueShape::new(type_id, shape_id)
    }

    pub(super) fn external_function_type(
        &mut self,
        arguments: &[SpecializedValueShape],
        return_: &SpecializedExternalValueShape,
    ) -> ExternalFunctionType {
        let shape = SpecializedFunctionShape::new(
            arguments.to_vec(),
            SpecializedValueShape::External(return_.clone()),
        );
        let nominal = self.function_type(&shape);
        let arguments = arguments
            .iter()
            .map(|argument| self.value_shape(argument))
            .collect();
        let return_ = self.external_type(return_);
        ExternalFunctionType::from_shapes(nominal, arguments, return_)
    }

    pub(super) fn function_shape(&mut self, shape: &SpecializedFunctionShape) -> FunctionShape {
        let type_ = self.function_type(shape);
        let shape_id = self.value_shape(&SpecializedValueShape::Function(Box::new(shape.clone())));
        FunctionShape::new(shape_id, type_)
    }

    fn custom_shape_id(&mut self, shape: &SpecializedCustomValueShape) -> CustomValueShapeId {
        if let Some(id) = self.custom_shape_ids.get(shape) {
            return *id;
        }

        let key = shape.clone();
        let type_id = self.custom_type(shape);
        let arguments = shape
            .arguments()
            .iter()
            .map(|argument| self.value_shape(argument))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let constructor = match shape.constructor() {
            plan::CustomConstructorRefinement::Any => CustomConstructorRefinement::Any,
            plan::CustomConstructorRefinement::Exact(index) => {
                CustomConstructorRefinement::Exact(index)
            }
        };
        let id = CustomValueShapeId::new(self.custom_shapes.len());
        self.custom_shapes.push(CustomValueShapeDescriptor::new(
            type_id,
            arguments,
            constructor,
        ));
        self.custom_shape_ids.insert(key, id);
        id
    }

    pub(super) fn function_type(&mut self, type_: &SpecializedFunctionShape) -> FunctionType {
        FunctionType::new(
            type_
                .arguments()
                .iter()
                .map(|argument| self.value_type(argument))
                .collect(),
            self.value_type(type_.return_()),
        )
    }

    pub(super) fn custom_function_type(
        &mut self,
        arguments: &[SpecializedValueShape],
        return_: &SpecializedCustomValueShape,
    ) -> CustomFunctionType {
        let shape = SpecializedFunctionShape::new(
            arguments.to_vec(),
            SpecializedValueShape::Custom(return_.clone()),
        );
        let nominal = self.function_type(&shape);
        CustomFunctionType::from_shapes(
            nominal,
            arguments
                .iter()
                .map(|argument| self.value_shape(argument))
                .collect(),
            self.custom_value_shape(return_),
        )
    }

    pub(super) fn generic_function_type(
        &mut self,
        shape: &SpecializedFunctionShape,
    ) -> GenericFunctionType {
        let nominal = self.function_type(shape);
        let shape = self.function_shape(shape);
        GenericFunctionType::from_shapes(nominal, shape)
    }

    pub(super) fn function_function_type(
        &mut self,
        arguments: &[SpecializedValueShape],
        return_: &SpecializedFunctionShape,
    ) -> FunctionFunctionType {
        let shape = SpecializedFunctionShape::new(
            arguments.to_vec(),
            SpecializedValueShape::Function(Box::new(return_.clone())),
        );
        let nominal = self.function_type(&shape);
        FunctionFunctionType::from_shapes(
            nominal,
            arguments
                .iter()
                .map(|argument| self.value_shape(argument))
                .collect(),
            self.function_shape(return_),
        )
    }

    pub(super) fn list_type(&mut self, item: &SpecializedValueShape) -> ListTypeId {
        match item {
            SpecializedValueShape::Parameter(parameter) => {
                self.parameter_list_type(*parameter).list_type()
            }
            SpecializedValueShape::Int => self.int_list_type().list_type(),
            SpecializedValueShape::String => self.string_list_type().list_type(),
            SpecializedValueShape::BitArray => self.bit_array_list_type().list_type(),
            SpecializedValueShape::UtfCodepoint => self.utf_codepoint_list_type().list_type(),
            SpecializedValueShape::Float => self.float_list_type().list_type(),
            SpecializedValueShape::Bool => self.bool_list_type().list_type(),
            SpecializedValueShape::Nil => self.nil_list_type().list_type(),
            SpecializedValueShape::Tuple(item) => self.tuple_list_type(item).list_type(),
            SpecializedValueShape::List(item) => self.list_list_type(item).list_type(),
            SpecializedValueShape::Function(item) => self.function_list_type(item).list_type(),
            SpecializedValueShape::Custom(item) => self.custom_list_type(item).list_type(),
            SpecializedValueShape::External(item) => self.external_list_type(item).list_type(),
        }
    }

    fn intern_primitive(
        &mut self,
        item: SpecializedValueShape,
        storage: impl FnOnce(ListTypeId) -> ListStorageTypeId,
    ) -> ListTypeId {
        let type_ = SpecializedValueShape::List(Box::new(item));
        if let Some(id) = self.ids.get(&type_) {
            return *id;
        }

        let id = ListTypeId::new(self.types.len());
        self.types.push(storage(id));
        self.ids.insert(type_, id);
        id
    }

    pub(super) fn parameter_list_type(
        &mut self,
        parameter: plan::TypeParameterId,
    ) -> ParameterListTypeId {
        let list_type =
            self.intern_primitive(SpecializedValueShape::Parameter(parameter), |list_type| {
                ListStorageTypeId::Parameter(ParameterListTypeId::new(list_type, parameter))
            });
        ParameterListTypeId::new(list_type, parameter)
    }

    pub(super) fn int_list_type(&mut self) -> IntListTypeId {
        let list_type = self.intern_primitive(SpecializedValueShape::Int, |list_type| {
            ListStorageTypeId::Int(IntListTypeId::new(list_type))
        });
        IntListTypeId::new(list_type)
    }

    pub(super) fn string_list_type(&mut self) -> StringListTypeId {
        let list_type = self.intern_primitive(SpecializedValueShape::String, |list_type| {
            ListStorageTypeId::String(StringListTypeId::new(list_type))
        });
        StringListTypeId::new(list_type)
    }

    pub(super) fn bit_array_list_type(&mut self) -> BitArrayListTypeId {
        let list_type = self.intern_primitive(SpecializedValueShape::BitArray, |list_type| {
            ListStorageTypeId::BitArray(BitArrayListTypeId::new(list_type))
        });
        BitArrayListTypeId::new(list_type)
    }

    pub(super) fn utf_codepoint_list_type(&mut self) -> UtfCodepointListTypeId {
        let list_type = self.intern_primitive(SpecializedValueShape::UtfCodepoint, |list_type| {
            ListStorageTypeId::UtfCodepoint(UtfCodepointListTypeId::new(list_type))
        });
        UtfCodepointListTypeId::new(list_type)
    }

    pub(super) fn float_list_type(&mut self) -> FloatListTypeId {
        let list_type = self.intern_primitive(SpecializedValueShape::Float, |list_type| {
            ListStorageTypeId::Float(FloatListTypeId::new(list_type))
        });
        FloatListTypeId::new(list_type)
    }

    pub(super) fn bool_list_type(&mut self) -> BoolListTypeId {
        let list_type = self.intern_primitive(SpecializedValueShape::Bool, |list_type| {
            ListStorageTypeId::Bool(BoolListTypeId::new(list_type))
        });
        BoolListTypeId::new(list_type)
    }

    pub(super) fn nil_list_type(&mut self) -> NilListTypeId {
        let list_type = self.intern_primitive(SpecializedValueShape::Nil, |list_type| {
            ListStorageTypeId::Nil(NilListTypeId::new(list_type))
        });
        NilListTypeId::new(list_type)
    }

    pub(super) fn tuple_list_type(&mut self, item: &[SpecializedValueShape]) -> TupleListTypeId {
        let type_ = SpecializedValueShape::List(Box::new(SpecializedValueShape::Tuple(
            item.to_vec().into_boxed_slice(),
        )));
        if let Some(id) = self.tuple_ids.get(&type_) {
            return *id;
        }

        let item = item
            .iter()
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

    pub(super) fn list_list_type(&mut self, item: &SpecializedValueShape) -> NestedListTypeId {
        match item.storage_representation() {
            super::specialization::StorageRepresentation::Parameter(parameter) => {
                NestedListTypeId::Parameter(self.parameter_list_list_type(parameter))
            }
            super::specialization::StorageRepresentation::Stored(item) => {
                NestedListTypeId::Stored(self.stored_list_list_type(&item))
            }
        }
    }

    pub(super) fn parameter_list_list_type(
        &mut self,
        parameter: plan::TypeParameterId,
    ) -> ParameterListListTypeId {
        if let Some(id) = self.parameter_list_ids.get(&parameter) {
            return *id;
        }
        let item = SpecializedValueShape::Parameter(parameter);
        let type_ =
            SpecializedValueShape::List(Box::new(SpecializedValueShape::List(Box::new(item))));
        let item_type = self.parameter_list_type(parameter);
        let list_type = ListTypeId::new(self.types.len());
        let type_id = ParameterListListTypeId::new(list_type, item_type);
        self.types.push(ListStorageTypeId::ParameterList(type_id));
        self.ids.insert(type_.clone(), list_type);
        self.parameter_list_ids.insert(parameter, type_id);
        type_id
    }

    pub(super) fn stored_list_list_type(&mut self, item: &StoredValueShape) -> ListListTypeId {
        if let Some(id) = self.list_ids.get(item) {
            return *id;
        }

        self.register_stored_list_list_type(item)
    }

    fn register_stored_list_list_type(&mut self, item: &StoredValueShape) -> ListListTypeId {
        let specialized = item.to_specialized();
        let type_ = SpecializedValueShape::List(Box::new(SpecializedValueShape::List(Box::new(
            specialized,
        ))));
        let item_type = match item {
            StoredValueShape::Int => self.int_list_type().list_type(),
            StoredValueShape::String => self.string_list_type().list_type(),
            StoredValueShape::BitArray => self.bit_array_list_type().list_type(),
            StoredValueShape::UtfCodepoint => self.utf_codepoint_list_type().list_type(),
            StoredValueShape::Custom(item) => self.custom_list_type(item).list_type(),
            StoredValueShape::External(item) => self.external_list_type(item).list_type(),
            StoredValueShape::Float => self.float_list_type().list_type(),
            StoredValueShape::Bool => self.bool_list_type().list_type(),
            StoredValueShape::Nil => self.nil_list_type().list_type(),
            StoredValueShape::Tuple(item) => self.tuple_list_type(item).list_type(),
            StoredValueShape::List(item) => self.list_list_type(item).list_type(),
            StoredValueShape::Function(item) => self.function_list_type(item).list_type(),
        };
        let list_type = ListTypeId::new(self.types.len());
        let id = ListListTypeId::new(list_type, item_type);
        self.types.push(ListStorageTypeId::List(id));
        self.ids.insert(type_.clone(), list_type);
        self.list_ids.insert(item.clone(), id);
        id
    }

    pub(super) fn function_list_type(
        &mut self,
        item: &SpecializedFunctionShape,
    ) -> FunctionListTypeId {
        let type_ = SpecializedValueShape::List(Box::new(SpecializedValueShape::Function(
            Box::new(item.clone()),
        )));
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

    pub(super) fn custom_list_type(
        &mut self,
        item: &SpecializedCustomValueShape,
    ) -> CustomListTypeId {
        let type_ = item.to_module_shape().type_().clone();
        if let Some(id) = self.custom_list_ids.get(&type_) {
            return *id;
        }

        let item_type = self.custom_type(item);
        let list_type = ListTypeId::new(self.types.len());
        let id = CustomListTypeId::new(list_type, item_type);
        self.types.push(ListStorageTypeId::Custom(id));
        self.custom_list_ids.insert(type_, id);
        id
    }

    pub(super) fn external_list_type(
        &mut self,
        item: &SpecializedExternalValueShape,
    ) -> ExternalListTypeId {
        let type_ = item.to_module_shape().type_().clone();
        if let Some(id) = self.external_list_ids.get(&type_) {
            return *id;
        }

        let item_type = self.external_type(item);
        let list_type = ListTypeId::new(self.types.len());
        let id = ExternalListTypeId::new(list_type, item_type);
        self.types.push(ListStorageTypeId::External(id));
        self.external_list_ids.insert(type_, id);
        id
    }

    pub(super) fn custom_type(&mut self, shape: &SpecializedCustomValueShape) -> CustomTypeId {
        let type_ = shape.to_module_shape().type_().clone();
        if let Some(id) = self.custom_ids.get(&type_) {
            return *id;
        }

        let id = CustomTypeId::new(self.custom_types.len());
        self.custom_ids.insert(type_.clone(), id);
        self.custom_types.push(CustomTypeDescriptor::new(type_));
        id
    }

    pub(super) fn external_type(
        &mut self,
        shape: &SpecializedExternalValueShape,
    ) -> ExternalTypeId {
        let type_ = shape.to_module_shape().type_().clone();
        if let Some(id) = self.external_ids.get(&type_) {
            return *id;
        }

        let id = ExternalTypeId::new(self.external_types.len());
        self.external_ids.insert(type_.clone(), id);
        self.external_types.push(type_);
        id
    }

    pub(super) fn custom_constructor(
        &mut self,
        constructor: SpecializedCustomConstructor,
    ) -> CustomConstructorId {
        let (type_, name, index, fields) = constructor.into_parts();
        let type_id = self.custom_type(&type_);
        let id = CustomConstructorId::new(type_id, index);
        if self.custom_types[type_id.index()].has_constructor(index) {
            return id;
        }

        let fields = fields
            .into_iter()
            .map(|field| {
                let (label, type_) = field.into_parts();
                CustomFieldDescriptor::new(label, self.value_type(&type_))
            })
            .collect();
        self.custom_types[type_id.index()]
            .insert_constructor(CustomConstructorDescriptor::new(id, name, fields));
        id
    }

    pub(super) fn into_tables(
        self,
    ) -> (
        ListTypeTable,
        CustomTypeTable,
        ExternalTypeTable,
        ValueShapeTable,
    ) {
        (
            ListTypeTable::from_parts(self.types, self.tuple_items, self.function_items),
            CustomTypeTable::new(self.custom_types),
            ExternalTypeTable::new(self.external_types),
            ValueShapeTable::new(self.shapes, self.shape_types, self.custom_shapes),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::ExecutionPlan;
    use crate::plan::execution::type_::{
        CustomConstructorId, CustomConstructorRefinement, CustomListTypeId, CustomTypeId,
        CustomValueShapeDescriptor, CustomValueShapeId, ListStorageTypeId, ListTypeId,
        ValueShapeDescriptor, ValueShapeId, ValueShapeTable, ValueType as ExecutionValueType,
    };
    use crate::plan::{CustomType, CustomTypeName, FunctionType, ValueType};

    #[test]
    fn lowering_preserves_exact_and_widened_custom_shapes() {
        let exact = execution_plan(
            r#"
pub type Choice { First(Int) Second(Int) }
pub fn main() { First(1) }
"#,
        );
        let widened = execution_plan(
            r#"
pub type Choice { First(Int) Second(Int) }
pub fn main() {
  let flag = True
  case flag { True -> First(1) False -> Second(2) }
}
"#,
        );

        let exact_shape = exact
            .custom_function(exact.custom_function_id(0))
            .body()
            .body_shape()
            .shape_id();
        let widened_shape = widened
            .custom_function(widened.custom_function_id(0))
            .body()
            .body_shape()
            .shape_id();
        assert_eq!(
            exact
                .program
                .common
                .value_shapes
                .custom(exact_shape)
                .constructor(),
            CustomConstructorRefinement::Exact(0),
        );
        assert_eq!(
            widened
                .program
                .common
                .value_shapes
                .custom(widened_shape)
                .constructor(),
            CustomConstructorRefinement::Any,
        );
    }

    #[test]
    fn lowering_preserves_nested_shapes_without_splitting_nominal_list_storage() {
        let plan = execution_plan(
            r#"
pub type Choice { First(Int) Second(Int) }
pub type Wrapper(a) { Wrapper(value: a) }

pub fn main() {
  let flag = True
  Wrapper(#(
    Wrapper(First(1)),
    [First(2)],
    case flag { True -> [First(3)] False -> [Second(4)] },
    fn() { First(5) },
  ))
}
"#,
        );
        assert_value_shapes(
            &plan.program.common.value_shapes,
            &[
                ValueShapeDescriptor::Custom(CustomValueShapeId::new(0)),
                ValueShapeDescriptor::Custom(CustomValueShapeId::new(1)),
                ValueShapeDescriptor::List(ValueShapeId::new(0)),
                ValueShapeDescriptor::Function {
                    arguments: Vec::new().into_boxed_slice(),
                    return_: ValueShapeId::new(0),
                },
                ValueShapeDescriptor::Tuple(
                    vec![
                        ValueShapeId::new(1),
                        ValueShapeId::new(2),
                        ValueShapeId::new(2),
                        ValueShapeId::new(3),
                    ]
                    .into_boxed_slice(),
                ),
                ValueShapeDescriptor::Custom(CustomValueShapeId::new(3)),
                ValueShapeDescriptor::Custom(CustomValueShapeId::new(4)),
                ValueShapeDescriptor::List(ValueShapeId::new(5)),
                ValueShapeDescriptor::Function {
                    arguments: Vec::new().into_boxed_slice(),
                    return_: ValueShapeId::new(5),
                },
                ValueShapeDescriptor::Tuple(
                    vec![
                        ValueShapeId::new(6),
                        ValueShapeId::new(7),
                        ValueShapeId::new(2),
                        ValueShapeId::new(8),
                    ]
                    .into_boxed_slice(),
                ),
                ValueShapeDescriptor::Bool,
                ValueShapeDescriptor::Int,
                ValueShapeDescriptor::Custom(CustomValueShapeId::new(5)),
                ValueShapeDescriptor::Custom(CustomValueShapeId::new(6)),
            ],
        );
        assert_custom_shapes(
            &plan.program.common.value_shapes,
            &[
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(2),
                    Vec::new().into_boxed_slice(),
                    CustomConstructorRefinement::Any,
                ),
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(1),
                    vec![ValueShapeId::new(0)].into_boxed_slice(),
                    CustomConstructorRefinement::Any,
                ),
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(0),
                    vec![ValueShapeId::new(4)].into_boxed_slice(),
                    CustomConstructorRefinement::Any,
                ),
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(2),
                    Vec::new().into_boxed_slice(),
                    CustomConstructorRefinement::Exact(0),
                ),
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(1),
                    vec![ValueShapeId::new(5)].into_boxed_slice(),
                    CustomConstructorRefinement::Exact(0),
                ),
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(0),
                    vec![ValueShapeId::new(9)].into_boxed_slice(),
                    CustomConstructorRefinement::Exact(0),
                ),
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(2),
                    Vec::new().into_boxed_slice(),
                    CustomConstructorRefinement::Exact(1),
                ),
            ],
        );
    }

    #[test]
    fn lowering_preserves_refinements_through_projections_calls_and_captures() {
        let plan = execution_plan(
            r#"
pub type Choice { First(Int) Second(Int) }
pub type Wrapper(a) { Wrapper(value: a, label: String) }
pub type Factory(a) { Factory(make: fn() -> a, label: String) }

fn direct() { First(3) }

pub fn main() {
  let tuple_value = #(First(1)).0
  let record_value = Wrapper(First(2), "record").value
  let captured = First(4)
  let closure = fn() { captured }
  let constructor = First
  let captured_constructor = fn() { constructor(6) }
  let factory = Factory(fn() { First(5) }, "factory")
  Wrapper(#(
    tuple_value,
    record_value,
    direct(),
    closure(),
    factory.make(),
    captured_constructor(),
  ), "result")
}
"#,
        );
        assert_value_shapes(
            &plan.program.common.value_shapes,
            &[
                ValueShapeDescriptor::Custom(CustomValueShapeId::new(0)),
                ValueShapeDescriptor::Tuple(vec![ValueShapeId::new(0); 6].into_boxed_slice()),
                ValueShapeDescriptor::Custom(CustomValueShapeId::new(2)),
                ValueShapeDescriptor::Tuple(
                    vec![
                        ValueShapeId::new(2),
                        ValueShapeId::new(2),
                        ValueShapeId::new(0),
                        ValueShapeId::new(2),
                        ValueShapeId::new(2),
                        ValueShapeId::new(2),
                    ]
                    .into_boxed_slice(),
                ),
                ValueShapeDescriptor::Int,
                ValueShapeDescriptor::Tuple(vec![ValueShapeId::new(2)].into_boxed_slice()),
                ValueShapeDescriptor::String,
                ValueShapeDescriptor::Custom(CustomValueShapeId::new(4)),
                ValueShapeDescriptor::Function {
                    arguments: Vec::new().into_boxed_slice(),
                    return_: ValueShapeId::new(0),
                },
                ValueShapeDescriptor::Function {
                    arguments: vec![ValueShapeId::new(4)].into_boxed_slice(),
                    return_: ValueShapeId::new(0),
                },
                ValueShapeDescriptor::Custom(CustomValueShapeId::new(5)),
                ValueShapeDescriptor::Custom(CustomValueShapeId::new(3)),
                ValueShapeDescriptor::Function {
                    arguments: vec![ValueShapeId::new(4)].into_boxed_slice(),
                    return_: ValueShapeId::new(2),
                },
            ],
        );
        assert_custom_shapes(
            &plan.program.common.value_shapes,
            &[
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(1),
                    Vec::new().into_boxed_slice(),
                    CustomConstructorRefinement::Any,
                ),
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(0),
                    vec![ValueShapeId::new(1)].into_boxed_slice(),
                    CustomConstructorRefinement::Any,
                ),
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(1),
                    Vec::new().into_boxed_slice(),
                    CustomConstructorRefinement::Exact(0),
                ),
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(0),
                    vec![ValueShapeId::new(3)].into_boxed_slice(),
                    CustomConstructorRefinement::Exact(0),
                ),
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(2),
                    vec![ValueShapeId::new(2)].into_boxed_slice(),
                    CustomConstructorRefinement::Exact(0),
                ),
                CustomValueShapeDescriptor::new(
                    CustomTypeId::new(3),
                    vec![ValueShapeId::new(2)].into_boxed_slice(),
                    CustomConstructorRefinement::Exact(0),
                ),
            ],
        );
    }

    #[test]
    fn lowering_preserves_refinements_through_function_returning_function_types() {
        let plan = execution_plan(
            r#"
pub type Choice { First(Int) Second(Int) }
pub type Boxed(a) { Boxed(value: a) }

pub fn main() { Boxed(fn() { fn() { First(1) } }).value }
"#,
        );
        let main = plan.function_function_function_id(0);
        let type_ = main.type_();
        assert_eq!(type_.argument_shapes(), []);
        let returned = type_.return_shape();
        let (arguments, return_) =
            function_shape(&plan.program.common.value_shapes, returned.shape_id());
        assert_eq!(arguments, &[]);
        let custom = custom_shape(&plan.program.common.value_shapes, return_);
        assert_eq!(
            plan.program
                .common
                .value_shapes
                .custom(custom)
                .constructor(),
            CustomConstructorRefinement::Exact(0),
        );
    }

    #[test]
    #[should_panic(expected = "expected a function value shape")]
    fn function_shape_fixture_guard_rejects_int_shape() {
        let table = ValueShapeTable::new(
            vec![ValueShapeDescriptor::Int],
            vec![ExecutionValueType::Int],
            Vec::new(),
        );

        let _ = function_shape(&table, ValueShapeId::new(0));
    }

    #[test]
    #[should_panic(expected = "expected a custom value shape")]
    fn custom_shape_fixture_guard_rejects_int_shape() {
        let table = ValueShapeTable::new(
            vec![ValueShapeDescriptor::Int],
            vec![ExecutionValueType::Int],
            Vec::new(),
        );

        let _ = custom_shape(&table, ValueShapeId::new(0));
    }

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
            .program
            .common
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
pub fn main() {
  let _ = #(ints, tuples, lists, functions)
  Nil
}
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
  inspect(Full(1, Empty), [Full("one", Empty)], Left(Right(Left(Stop))))
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        let int_empty = plan.custom_constructor(CustomConstructorId::new(CustomTypeId::new(0), 0));
        assert_eq!(int_empty.name(), &ecow::EcoString::from("Empty"));
        assert_eq!(int_empty.fields().len(), 0);
        let int_full = plan.custom_constructor(CustomConstructorId::new(CustomTypeId::new(0), 1));
        assert_eq!(int_full.name(), &ecow::EcoString::from("Full"));
        assert_eq!(
            int_full
                .fields()
                .iter()
                .map(|field| field.type_().clone())
                .collect::<Vec<_>>(),
            vec![
                ExecutionValueType::Int,
                ExecutionValueType::Custom(CustomTypeId::new(0)),
            ],
        );

        let string_full =
            plan.custom_constructor(CustomConstructorId::new(CustomTypeId::new(1), 1));
        assert_eq!(
            string_full
                .fields()
                .iter()
                .map(|field| field.type_().clone())
                .collect::<Vec<_>>(),
            vec![
                ExecutionValueType::String,
                ExecutionValueType::Custom(CustomTypeId::new(1)),
            ],
        );
        assert_eq!(
            plan.list_storage_type(ListTypeId::new(0)),
            ListStorageTypeId::Custom(CustomListTypeId::new(
                ListTypeId::new(0),
                CustomTypeId::new(1),
            )),
        );

        assert_eq!(
            plan.custom_value_type(CustomTypeId::new(2)),
            CustomType::new(
                CustomTypeName::new("geam".into(), "main".into(), "Right".into()),
                Vec::new(),
            ),
        );
        assert_eq!(
            plan.custom_value_type(CustomTypeId::new(3)),
            CustomType::new(
                CustomTypeName::new("geam".into(), "main".into(), "Left".into()),
                Vec::new(),
            ),
        );

        let right = plan.custom_constructor(CustomConstructorId::new(CustomTypeId::new(2), 0));
        let left = plan.custom_constructor(CustomConstructorId::new(CustomTypeId::new(3), 0));
        assert_eq!(
            left.fields()[0].type_(),
            &ExecutionValueType::Custom(CustomTypeId::new(2)),
        );
        assert_eq!(
            right.fields()[0].type_(),
            &ExecutionValueType::Custom(CustomTypeId::new(3)),
        );
        assert_eq!(
            plan.custom_constructor(CustomConstructorId::new(CustomTypeId::new(2), 1))
                .fields()
                .len(),
            0,
        );
        assert_eq!(
            plan.custom_value_type(CustomTypeId::new(0)),
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

        assert_eq!(plan.program.common.custom_types.len(), 2);
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

    fn execution_plan(source: &str) -> ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module)
    }

    fn assert_value_shapes(table: &ValueShapeTable, expected: &[ValueShapeDescriptor]) {
        for (index, expected) in expected.iter().enumerate() {
            assert_eq!(
                table.get(ValueShapeId::new(index)),
                expected,
                "shape {index}"
            );
        }
        assert!(
            std::panic::catch_unwind(|| table.get(ValueShapeId::new(expected.len()))).is_err(),
            "unexpected trailing value shape",
        );
    }

    fn assert_custom_shapes(table: &ValueShapeTable, expected: &[CustomValueShapeDescriptor]) {
        for (index, expected) in expected.iter().enumerate() {
            assert_eq!(
                table.custom(CustomValueShapeId::new(index)),
                expected,
                "custom shape {index}",
            );
        }
        assert!(
            std::panic::catch_unwind(|| { table.custom(CustomValueShapeId::new(expected.len())) })
                .is_err(),
            "unexpected trailing custom value shape",
        );
    }

    fn function_shape(
        table: &ValueShapeTable,
        shape: ValueShapeId,
    ) -> (&[ValueShapeId], ValueShapeId) {
        match table.get(shape) {
            ValueShapeDescriptor::Function { arguments, return_ } => (arguments, *return_),
            _ => panic!("expected a function value shape"),
        }
    }

    fn custom_shape(table: &ValueShapeTable, shape: ValueShapeId) -> CustomValueShapeId {
        match table.get(shape) {
            ValueShapeDescriptor::Custom(custom) => *custom,
            _ => panic!("expected a custom value shape"),
        }
    }
}
