mod definition;
mod flow;
mod inhabitation;
pub(super) use inhabitation::FunctionRepresentation;
mod local;
pub(super) use local::{ListSlot, Slot, TupleSlot};
mod refinement;

use crate::plan::execution::type_::{
    CustomConstructorId, CustomConstructorRefinement, CustomTypeTable, ExternalTypeTable,
    FunctionType, ListStorageTypeId, ListTypeId, ListTypeTable, TypeMetadata, ValueShapeDescriptor,
    ValueShapeId, ValueShapeTable, ValueType,
};
use std::collections::{BTreeMap, HashSet};

pub(super) struct Types<'data> {
    pub(super) lists: &'data ListTypeTable,
    pub(super) customs: &'data CustomTypeTable,
    externals: &'data ExternalTypeTable,
    pub(super) shapes: &'data ValueShapeTable,
    declarations: BTreeMap<
        (&'data str, &'data str, &'data str),
        &'data crate::plan::execution::type_::custom::CustomDefinition,
    >,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum TypeError {
    MissingList {
        index: usize,
    },
    MissingCustom {
        index: usize,
    },
    MissingExternal {
        index: usize,
    },
    MissingTupleItem {
        index: usize,
    },
    MissingFunctionItem {
        index: usize,
    },
    MisplacedList {
        index: usize,
        stored: usize,
    },
    ParameterListItem {
        index: usize,
    },
    ConstructorOwner {
        type_index: usize,
        index: usize,
    },
    ConstructorOrder {
        type_index: usize,
    },
    ConstructorIndex {
        type_index: usize,
        index: usize,
        count: usize,
    },
    MissingConstructor {
        type_index: usize,
        index: usize,
    },
    RecursiveType,
    RecursiveMetadata,
    MissingShape {
        index: usize,
    },
    MissingCustomShape {
        index: usize,
    },
    ShapeTableLength {
        shapes: usize,
        types: usize,
    },
    ShapeType {
        index: usize,
    },
    CustomShapeArguments {
        index: usize,
    },
    RecursiveShape,
    LocalTypeMismatch,
    ListStorageMismatch {
        index: usize,
    },
    FieldShape,
    Definition,
    DefinitionParameter {
        index: usize,
        parameters: usize,
    },
    MissingDefinition {
        package: String,
        module: String,
        name: String,
    },
    FunctionRepresentation,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum TypeNode {
    Value(*const ValueType),
    List(usize),
    Metadata(*const TypeMetadata),
    Shape(usize),
}

enum Visit<'data> {
    Enter(TypeRef<'data>),
    Leave(TypeNode),
}

enum TypeRef<'data> {
    Value(&'data ValueType),
    List(ListTypeId),
    Metadata(&'data TypeMetadata),
    Shape(usize),
}

impl<'data> Types<'data> {
    pub(super) fn admit(
        lists: &'data ListTypeTable,
        customs: &'data CustomTypeTable,
        externals: &'data ExternalTypeTable,
        shapes: &'data ValueShapeTable,
    ) -> Result<Self, TypeError> {
        let tables = Self {
            lists,
            customs,
            externals,
            shapes,
            declarations: customs
                .definitions
                .iter()
                .chain(std::iter::once(
                    &crate::plan::execution::type_::custom::definition::RESULT,
                ))
                .map(|definition| (definition.identity(), definition))
                .collect(),
        };
        tables.validate()?;
        Ok(tables)
    }

    fn validate(&self) -> Result<(), TypeError> {
        self.definitions()?;
        let mut roots = Vec::new();
        for (index, list) in self.lists.types.iter().enumerate() {
            let stored = list.list_type().index();
            if index != stored {
                return Err(TypeError::MisplacedList { index, stored });
            }
            roots.push(TypeRef::List(ListTypeId(index)));
        }
        for items in self.lists.tuple_items.iter() {
            roots.extend(items.iter().map(TypeRef::Value));
        }
        for function in self.lists.function_items.iter() {
            Self::function_roots(function, &mut roots);
        }
        for (type_index, descriptor) in self.customs.types.iter().enumerate() {
            roots.extend(descriptor.type_.arguments.iter().map(TypeRef::Metadata));
            let mut previous = None;
            for constructor in descriptor.constructors.iter() {
                if constructor.id.index >= descriptor.constructor_count {
                    return Err(TypeError::ConstructorIndex {
                        type_index,
                        index: constructor.id.index,
                        count: descriptor.constructor_count,
                    });
                }
                if constructor.id.type_id.index() != type_index {
                    return Err(TypeError::ConstructorOwner {
                        type_index,
                        index: constructor.id.index,
                    });
                }
                if previous.is_some_and(|index| index >= constructor.id.index) {
                    return Err(TypeError::ConstructorOrder { type_index });
                }
                previous = Some(constructor.id.index);
                roots.extend(
                    constructor
                        .fields
                        .iter()
                        .map(|field| TypeRef::Value(&field.type_)),
                );
                roots.extend(
                    constructor
                        .fields
                        .iter()
                        .map(|field| TypeRef::Shape(field.shape.index())),
                );
            }
        }
        for descriptor in self.externals.types.iter() {
            roots.extend(descriptor.arguments.iter().map(TypeRef::Metadata));
        }
        if self.shapes.shapes.len() != self.shapes.shape_types.len() {
            return Err(TypeError::ShapeTableLength {
                shapes: self.shapes.shapes.len(),
                types: self.shapes.shape_types.len(),
            });
        }
        roots.extend(self.shapes.shape_types.iter().map(TypeRef::Value));
        roots.extend((0..self.shapes.shapes.len()).map(TypeRef::Shape));
        for (index, shape) in self.shapes.custom_shapes.iter().enumerate() {
            let descriptor =
                self.customs
                    .types
                    .get(shape.type_id.index())
                    .ok_or(TypeError::MissingCustom {
                        index: shape.type_id.index(),
                    })?;
            if shape.arguments.len() != descriptor.type_.arguments.len() {
                return Err(TypeError::CustomShapeArguments { index });
            }
            roots.extend(shape.arguments.iter().map(|id| TypeRef::Shape(id.index())));
            // A divergent function can describe a return constructor that is
            // never materialized. Only actual constructor uses index the sparse table.
            if let CustomConstructorRefinement::Exact(index) = shape.constructor
                && index >= descriptor.constructor_count
            {
                return Err(TypeError::ConstructorIndex {
                    type_index: shape.type_id.index(),
                    index,
                    count: descriptor.constructor_count,
                });
            }
        }
        self.walk(roots)?;
        for (index, shape) in self.shapes.custom_shapes.iter().enumerate() {
            let declared = &self.customs.types[shape.type_id.index()].type_.arguments;
            if !shape
                .arguments
                .iter()
                .zip(declared.iter())
                .all(|(id, metadata)| {
                    self.metadata_matches_value(metadata, &self.shapes.shape_types[id.index()])
                })
            {
                return Err(TypeError::CustomShapeArguments { index });
            }
        }
        for (index, descriptor) in self.shapes.shapes.iter().enumerate() {
            if !self.shape_matches(descriptor, &self.shapes.shape_types[index]) {
                return Err(TypeError::ShapeType { index });
            }
        }
        for descriptor in self.customs.types.iter() {
            self.custom_definition(descriptor)?;
            for constructor in descriptor.constructors.iter() {
                for field in constructor.fields.iter() {
                    if self.shapes.shape_types[field.shape.index()] != field.type_
                        || !self.is_nominal_shape(field.shape)
                    {
                        return Err(TypeError::FieldShape);
                    }
                }
            }
        }
        Ok(())
    }

    pub(super) fn value(&self, value: &ValueType) -> Result<(), TypeError> {
        self.walk(vec![TypeRef::Value(value)])
    }

    pub(super) fn external_types(
        &self,
    ) -> &'data [crate::plan::execution::type_::NominalTypeMetadata] {
        &self.externals.types
    }

    pub(super) fn metadata(&self, value: &TypeMetadata) -> Result<(), TypeError> {
        self.walk(vec![TypeRef::Metadata(value)])
    }

    pub(super) fn shape(&self, id: ValueShapeId) -> Result<&'data ValueShapeDescriptor, TypeError> {
        self.shapes
            .shapes
            .get(id.index())
            .ok_or(TypeError::MissingShape { index: id.index() })
    }

    pub(super) fn shape_types(&self) -> &'data [ValueType] {
        &self.shapes.shape_types
    }

    pub(super) fn matches(
        &self,
        metadata: &TypeMetadata,
        value: &ValueType,
    ) -> Result<bool, TypeError> {
        self.metadata(metadata)?;
        self.value(value)?;
        Ok(self.metadata_matches_value(metadata, value))
    }

    pub(super) fn constructor(&self, id: CustomConstructorId) -> Result<(), TypeError> {
        self.constructor_descriptor(id).map(|_| ())
    }

    pub(super) fn custom_type(
        &self,
        id: crate::plan::execution::type_::CustomTypeId,
    ) -> Result<&'data crate::plan::execution::type_::CustomTypeDescriptor, TypeError> {
        self.customs
            .types
            .get(id.index())
            .ok_or(TypeError::MissingCustom { index: id.index() })
    }

    pub(super) fn custom_shape_descriptor(
        &self,
        id: crate::plan::execution::type_::CustomValueShapeId,
    ) -> Result<&'data crate::plan::execution::type_::CustomValueShapeDescriptor, TypeError> {
        self.shapes
            .custom_shapes
            .get(id.0)
            .ok_or(TypeError::MissingCustomShape { index: id.0 })
    }

    pub(super) fn constructor_descriptor(
        &self,
        id: CustomConstructorId,
    ) -> Result<&'data crate::plan::execution::type_::CustomConstructorDescriptor, TypeError> {
        let descriptor = self.custom_type(id.type_id)?;
        descriptor
            .constructors
            .binary_search_by_key(&id.index, |constructor| constructor.id.index)
            .map(|index| &descriptor.constructors[index])
            .map_err(|_| TypeError::MissingConstructor {
                type_index: id.type_id.index(),
                index: id.index,
            })
    }

    // Nominal recursion ends at the named type. Structural cycles in inline or
    // indexed types are not source types and must not reach recursive runtime reads.
    fn walk<'value>(&'value self, roots: Vec<TypeRef<'value>>) -> Result<(), TypeError> {
        let mut stack: Vec<_> = roots.into_iter().map(Visit::Enter).collect();
        let mut children = Vec::new();
        let mut active = HashSet::new();
        let mut completed = HashSet::new();
        while let Some(visit) = stack.pop() {
            let value = match visit {
                Visit::Enter(value) => value,
                Visit::Leave(key) => {
                    active.remove(&key);
                    completed.insert(key);
                    continue;
                }
            };
            let (key, cycle) = match &value {
                TypeRef::Value(value) => (TypeNode::Value(*value), TypeError::RecursiveType),
                TypeRef::List(id) => (TypeNode::List(id.index()), TypeError::RecursiveType),
                TypeRef::Metadata(value) => {
                    (TypeNode::Metadata(*value), TypeError::RecursiveMetadata)
                }
                TypeRef::Shape(index) => (TypeNode::Shape(*index), TypeError::RecursiveShape),
            };
            if completed.contains(&key) {
                continue;
            }
            if !active.insert(key) {
                return Err(cycle);
            }
            stack.push(Visit::Leave(key));
            match value {
                TypeRef::Value(value) => self.value_children(value, &mut children)?,
                TypeRef::List(id) => self.list_children(id, &mut children)?,
                TypeRef::Metadata(value) => Self::metadata_children(value, &mut children),
                TypeRef::Shape(index) => self.shape_children(index, &mut children)?,
            }
            stack.extend(children.drain(..).map(Visit::Enter));
        }
        Ok(())
    }

    fn value_children<'value>(
        &'value self,
        value: &'value ValueType,
        stack: &mut Vec<TypeRef<'value>>,
    ) -> Result<(), TypeError> {
        match value {
            ValueType::Parameter(_)
            | ValueType::Int
            | ValueType::Float
            | ValueType::String
            | ValueType::BitArray
            | ValueType::UtfCodepoint
            | ValueType::Bool
            | ValueType::Nil => {}
            ValueType::Tuple(items) => stack.extend(items.iter().map(TypeRef::Value)),
            ValueType::List(id) => stack.push(TypeRef::List(*id)),
            ValueType::Function(function) => Self::function_roots(function, stack),
            ValueType::Custom(id) => {
                self.customs
                    .types
                    .get(id.index())
                    .ok_or(TypeError::MissingCustom { index: id.index() })?;
            }
            ValueType::External(id) => {
                self.externals
                    .types
                    .get(id.index())
                    .ok_or(TypeError::MissingExternal { index: id.index() })?;
            }
        }
        Ok(())
    }

    fn function_roots<'value>(function: &'value FunctionType, stack: &mut Vec<TypeRef<'value>>) {
        stack.extend(function.arguments.iter().map(TypeRef::Value));
        stack.push(TypeRef::Value(&function.return_));
    }

    fn list_children<'value>(
        &'value self,
        id: ListTypeId,
        stack: &mut Vec<TypeRef<'value>>,
    ) -> Result<(), TypeError> {
        let list = self
            .lists
            .types
            .get(id.index())
            .ok_or(TypeError::MissingList { index: id.index() })?;
        match list {
            ListStorageTypeId::Parameter(_)
            | ListStorageTypeId::Int(_)
            | ListStorageTypeId::String(_)
            | ListStorageTypeId::BitArray(_)
            | ListStorageTypeId::UtfCodepoint(_)
            | ListStorageTypeId::Float(_)
            | ListStorageTypeId::Bool(_)
            | ListStorageTypeId::Nil(_) => {}
            ListStorageTypeId::Tuple(item) => {
                let items = self.lists.tuple_items.get(item.item_type.0).ok_or(
                    TypeError::MissingTupleItem {
                        index: item.item_type.0,
                    },
                )?;
                stack.extend(items.iter().map(TypeRef::Value));
            }
            ListStorageTypeId::Function(item) => {
                let function = self.lists.function_items.get(item.item_type.0).ok_or(
                    TypeError::MissingFunctionItem {
                        index: item.item_type.0,
                    },
                )?;
                Self::function_roots(function, stack);
            }
            ListStorageTypeId::ParameterList(item) => {
                let inner = self.lists.types.get(item.item_type.list_type.index());
                if inner != Some(&ListStorageTypeId::Parameter(item.item_type)) {
                    return Err(TypeError::ParameterListItem { index: id.index() });
                }
                stack.push(TypeRef::List(item.item_type.list_type));
            }
            ListStorageTypeId::List(item) => stack.push(TypeRef::List(item.item_type)),
            ListStorageTypeId::Custom(item) => {
                self.customs
                    .types
                    .get(item.item_type.index())
                    .ok_or(TypeError::MissingCustom {
                        index: item.item_type.index(),
                    })?;
            }
            ListStorageTypeId::External(item) => {
                self.externals.types.get(item.item_type.index()).ok_or(
                    TypeError::MissingExternal {
                        index: item.item_type.index(),
                    },
                )?;
            }
        }
        Ok(())
    }

    fn metadata_children<'value>(value: &'value TypeMetadata, stack: &mut Vec<TypeRef<'value>>) {
        match value {
            TypeMetadata::Parameter(_)
            | TypeMetadata::Int
            | TypeMetadata::Float
            | TypeMetadata::String
            | TypeMetadata::BitArray
            | TypeMetadata::UtfCodepoint
            | TypeMetadata::Bool
            | TypeMetadata::Nil => {}
            TypeMetadata::Tuple(items) => stack.extend(items.iter().map(TypeRef::Metadata)),
            TypeMetadata::List(item) => stack.push(TypeRef::Metadata(item)),
            TypeMetadata::Function(function) => {
                stack.extend(function.arguments.iter().map(TypeRef::Metadata));
                stack.push(TypeRef::Metadata(&function.return_));
            }
            TypeMetadata::Custom(nominal) | TypeMetadata::External(nominal) => {
                stack.extend(nominal.arguments.iter().map(TypeRef::Metadata));
            }
        }
    }

    fn shape_children<'value>(
        &'value self,
        index: usize,
        stack: &mut Vec<TypeRef<'value>>,
    ) -> Result<(), TypeError> {
        let shape = self
            .shapes
            .shapes
            .get(index)
            .ok_or(TypeError::MissingShape { index })?;
        match shape {
            ValueShapeDescriptor::Parameter(_)
            | ValueShapeDescriptor::Int
            | ValueShapeDescriptor::Float
            | ValueShapeDescriptor::String
            | ValueShapeDescriptor::BitArray
            | ValueShapeDescriptor::UtfCodepoint
            | ValueShapeDescriptor::Bool
            | ValueShapeDescriptor::Nil => {}
            ValueShapeDescriptor::Tuple(items) => {
                stack.extend(items.iter().map(|id| TypeRef::Shape(id.index())))
            }
            ValueShapeDescriptor::List(item) => stack.push(TypeRef::Shape(item.index())),
            ValueShapeDescriptor::Function { arguments, return_ } => {
                stack.extend(arguments.iter().map(|id| TypeRef::Shape(id.index())));
                stack.push(TypeRef::Shape(return_.index()));
            }
            ValueShapeDescriptor::Custom(id) => {
                let shape = self
                    .shapes
                    .custom_shapes
                    .get(id.0)
                    .ok_or(TypeError::MissingCustomShape { index: id.0 })?;
                stack.extend(shape.arguments.iter().map(|id| TypeRef::Shape(id.index())));
            }
            ValueShapeDescriptor::External(id) => {
                self.externals
                    .types
                    .get(id.index())
                    .ok_or(TypeError::MissingExternal { index: id.index() })?;
            }
        }
        Ok(())
    }

    fn shape_matches(&self, shape: &ValueShapeDescriptor, type_: &ValueType) -> bool {
        match (shape, type_) {
            (ValueShapeDescriptor::Parameter(a), ValueType::Parameter(b)) => a == b,
            (ValueShapeDescriptor::Int, ValueType::Int)
            | (ValueShapeDescriptor::Float, ValueType::Float)
            | (ValueShapeDescriptor::String, ValueType::String)
            | (ValueShapeDescriptor::BitArray, ValueType::BitArray)
            | (ValueShapeDescriptor::UtfCodepoint, ValueType::UtfCodepoint)
            | (ValueShapeDescriptor::Bool, ValueType::Bool)
            | (ValueShapeDescriptor::Nil, ValueType::Nil) => true,
            (ValueShapeDescriptor::Tuple(shapes), ValueType::Tuple(types)) => {
                shapes.len() == types.len()
                    && shapes
                        .iter()
                        .zip(types.iter())
                        .all(|(id, type_)| &self.shapes.shape_types[id.index()] == type_)
            }
            (ValueShapeDescriptor::List(shape), ValueType::List(id)) => {
                self.list_item_matches(*id, &self.shapes.shape_types[shape.index()])
            }
            (
                ValueShapeDescriptor::Function { arguments, return_ },
                ValueType::Function(function),
            ) => {
                arguments.len() == function.arguments.len()
                    && arguments
                        .iter()
                        .zip(function.arguments.iter())
                        .all(|(id, type_)| &self.shapes.shape_types[id.index()] == type_)
                    && &self.shapes.shape_types[return_.index()] == function.return_()
            }
            (ValueShapeDescriptor::Custom(id), ValueType::Custom(type_id)) => {
                self.shapes.custom_shapes[id.0].type_id == *type_id
            }
            (ValueShapeDescriptor::External(a), ValueType::External(b)) => a == b,
            _ => false,
        }
    }

    fn list_item_matches(&self, id: ListTypeId, item: &ValueType) -> bool {
        match (&self.lists.types[id.index()], item) {
            (ListStorageTypeId::Parameter(a), ValueType::Parameter(b)) => a.item == *b,
            (ListStorageTypeId::Int(_), ValueType::Int)
            | (ListStorageTypeId::Float(_), ValueType::Float)
            | (ListStorageTypeId::String(_), ValueType::String)
            | (ListStorageTypeId::BitArray(_), ValueType::BitArray)
            | (ListStorageTypeId::UtfCodepoint(_), ValueType::UtfCodepoint)
            | (ListStorageTypeId::Bool(_), ValueType::Bool)
            | (ListStorageTypeId::Nil(_), ValueType::Nil) => true,
            (ListStorageTypeId::Tuple(a), ValueType::Tuple(b)) => {
                self.lists.tuple_items[a.item_type.0].as_ref() == b.as_ref()
            }
            (ListStorageTypeId::Function(a), ValueType::Function(b)) => {
                &self.lists.function_items[a.item_type.0] == b
            }
            (ListStorageTypeId::List(a), ValueType::List(b)) => a.item_type == *b,
            (ListStorageTypeId::ParameterList(a), ValueType::List(b)) => {
                a.item_type.list_type == *b
            }
            (ListStorageTypeId::Custom(a), ValueType::Custom(b)) => a.item_type == *b,
            (ListStorageTypeId::External(a), ValueType::External(b)) => a.item_type == *b,
            _ => false,
        }
    }

    pub(super) fn metadata_matches_value(
        &self,
        metadata: &TypeMetadata,
        value: &ValueType,
    ) -> bool {
        match (metadata, value) {
            (TypeMetadata::Parameter(a), ValueType::Parameter(b)) => a == b,
            (TypeMetadata::Int, ValueType::Int)
            | (TypeMetadata::Float, ValueType::Float)
            | (TypeMetadata::String, ValueType::String)
            | (TypeMetadata::BitArray, ValueType::BitArray)
            | (TypeMetadata::UtfCodepoint, ValueType::UtfCodepoint)
            | (TypeMetadata::Bool, ValueType::Bool)
            | (TypeMetadata::Nil, ValueType::Nil) => true,
            (TypeMetadata::Tuple(a), ValueType::Tuple(b)) => {
                a.len() == b.len()
                    && a.iter()
                        .zip(b.iter())
                        .all(|(a, b)| self.metadata_matches_value(a, b))
            }
            (TypeMetadata::List(a), ValueType::List(b)) => self.metadata_matches_list_item(a, *b),
            (TypeMetadata::Function(a), ValueType::Function(b)) => {
                a.arguments.len() == b.arguments.len()
                    && a.arguments
                        .iter()
                        .zip(b.arguments.iter())
                        .all(|(a, b)| self.metadata_matches_value(a, b))
                    && self.metadata_matches_value(&a.return_, &b.return_)
            }
            (TypeMetadata::Custom(a), ValueType::Custom(b)) => {
                a == &self.customs.types[b.index()].type_
            }
            (TypeMetadata::External(a), ValueType::External(b)) => {
                a == &self.externals.types[b.index()]
            }
            _ => false,
        }
    }

    fn metadata_matches_list_item(&self, metadata: &TypeMetadata, id: ListTypeId) -> bool {
        match (metadata, &self.lists.types[id.index()]) {
            (TypeMetadata::Parameter(a), ListStorageTypeId::Parameter(b)) => *a == b.item,
            (TypeMetadata::Int, ListStorageTypeId::Int(_))
            | (TypeMetadata::Float, ListStorageTypeId::Float(_))
            | (TypeMetadata::String, ListStorageTypeId::String(_))
            | (TypeMetadata::BitArray, ListStorageTypeId::BitArray(_))
            | (TypeMetadata::UtfCodepoint, ListStorageTypeId::UtfCodepoint(_))
            | (TypeMetadata::Bool, ListStorageTypeId::Bool(_))
            | (TypeMetadata::Nil, ListStorageTypeId::Nil(_)) => true,
            (TypeMetadata::Tuple(a), ListStorageTypeId::Tuple(b)) => {
                let b = &self.lists.tuple_items[b.item_type.0];
                a.len() == b.len()
                    && a.iter()
                        .zip(b.iter())
                        .all(|(a, b)| self.metadata_matches_value(a, b))
            }
            (TypeMetadata::Function(a), ListStorageTypeId::Function(b)) => {
                let b = &self.lists.function_items[b.item_type.0];
                a.arguments.len() == b.arguments.len()
                    && a.arguments
                        .iter()
                        .zip(b.arguments.iter())
                        .all(|(a, b)| self.metadata_matches_value(a, b))
                    && self.metadata_matches_value(&a.return_, &b.return_)
            }
            (TypeMetadata::List(a), ListStorageTypeId::List(b)) => {
                self.metadata_matches_list_item(a, b.item_type)
            }
            (TypeMetadata::List(a), ListStorageTypeId::ParameterList(b)) => {
                self.metadata_matches_list_item(a, b.item_type.list_type)
            }
            (TypeMetadata::Custom(a), ListStorageTypeId::Custom(b)) => {
                a == &self.customs.types[b.item_type.index()].type_
            }
            (TypeMetadata::External(a), ListStorageTypeId::External(b)) => {
                a == &self.externals.types[b.item_type.index()]
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CustomConstructorId, CustomTypeTable, ExternalTypeTable, ListStorageTypeId, ListTypeId,
        ListTypeTable, TypeError, TypeMetadata, Types, ValueShapeDescriptor, ValueShapeId,
        ValueShapeTable, ValueType,
    };
    use crate::plan::execution::storage::{Node, Table};
    use crate::plan::execution::type_::list::{FunctionItemTypeId, TupleItemTypeId};
    use crate::plan::execution::type_::{
        CustomListTypeId, CustomTypeId, CustomValueShapeId, ExternalListTypeId, ExternalTypeId,
        FunctionListTypeId, FunctionMetadata, IntListTypeId, ListListTypeId,
        ParameterListListTypeId, ParameterListTypeId, TupleListTypeId,
    };

    static LISTS: ListTypeTable = ListTypeTable {
        types: Table::Static(&[]),
        tuple_items: Table::Static(&[]),
        function_items: Table::Static(&[]),
    };
    static CUSTOMS: CustomTypeTable = CustomTypeTable {
        definitions: Table::Static(&[]),
        types: Table::Static(&[]),
    };
    static EXTERNALS: ExternalTypeTable = ExternalTypeTable {
        types: Table::Static(&[]),
    };
    static SHAPES: ValueShapeTable = ValueShapeTable {
        shapes: Table::Static(&[]),
        shape_types: Table::Static(&[]),
        custom_shapes: Table::Static(&[]),
    };

    #[test]
    fn admits_real_lowered_primitive_nested_generic_and_recursive_nominal_types() {
        let sources = [
            "pub fn main() { #(1, 1.5, \"text\", True, Nil, <<1, 2>>) }",
            "pub fn main() { #([1], [1.5], [\"text\"], [True], [Nil], [<<1>>], [[1]], [#(1, True)]) }",
            "fn make(x) { [x] } pub fn main() { #(make(1), make(\"text\"), [fn(x) { x + 1 }]) }",
            "pub type Chain { End Link(Chain) } pub fn main() { Link(Link(End)) }",
            "pub type Boxed(a) { Boxed(a) } pub fn main() { #(Boxed([1]), [Boxed(fn(x) { x + 1 })]) }",
            "pub type Choice { First Second(String) Third(Int) } pub fn main() { Third(3) }",
        ];
        for source in sources {
            let module =
                crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
            let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(module).unwrap());
            let common = &plan.program.common;
            assert_eq!(
                Types::admit(
                    &common.list_types,
                    &common.custom_types,
                    &common.external_types,
                    &common.value_shapes
                )
                .map(|_| ()),
                Ok(()),
                "{source}"
            );
        }
    }

    #[test]
    fn rejects_unknown_nominal_and_list_references() {
        let types = Types::admit(&LISTS, &CUSTOMS, &EXTERNALS, &SHAPES).unwrap();
        for (value, expected) in [
            (
                ValueType::Custom(CustomTypeId(3)),
                TypeError::MissingCustom { index: 3 },
            ),
            (
                ValueType::External(ExternalTypeId(4)),
                TypeError::MissingExternal { index: 4 },
            ),
            (
                ValueType::List(ListTypeId(5)),
                TypeError::MissingList { index: 5 },
            ),
        ] {
            assert_eq!(types.value(&value), Err(expected));
        }
        assert_eq!(
            types.constructor(CustomConstructorId {
                type_id: CustomTypeId(2),
                index: 1
            }),
            Err(TypeError::MissingCustom { index: 2 })
        );
    }

    #[test]
    fn rejects_inline_cycles_without_recursing_or_executing() {
        static VALUE: ValueType = ValueType::Tuple(Table::Static(std::slice::from_ref(&VALUE)));
        static METADATA: TypeMetadata = TypeMetadata::List(Node::Static(&METADATA));
        let types = Types::admit(&LISTS, &CUSTOMS, &EXTERNALS, &SHAPES).unwrap();
        assert_eq!(types.value(&VALUE), Err(TypeError::RecursiveType));
        assert_eq!(types.metadata(&METADATA), Err(TypeError::RecursiveMetadata));
    }

    #[test]
    fn admits_shared_metadata_nodes_without_treating_them_as_cycles() {
        static LEAF: TypeMetadata = TypeMetadata::Int;
        static ROOT: TypeMetadata = TypeMetadata::Tuple(Table::Static(&[
            TypeMetadata::List(Node::Static(&LEAF)),
            TypeMetadata::Function(FunctionMetadata {
                arguments: Table::Static(&[TypeMetadata::String]),
                return_: Node::Static(&LEAF),
            }),
        ]));
        let types = Types::admit(&LISTS, &CUSTOMS, &EXTERNALS, &SHAPES).unwrap();
        assert_eq!(types.metadata(&ROOT), Ok(()));
    }

    #[test]
    fn rejects_indexed_cycles_and_incorrect_list_row_identity() {
        for (items, expected) in [
            (
                vec![ListStorageTypeId::List(ListListTypeId {
                    list_type: ListTypeId(0),
                    item_type: ListTypeId(0),
                })],
                TypeError::RecursiveType,
            ),
            (
                vec![ListStorageTypeId::Int(IntListTypeId {
                    list_type: ListTypeId(1),
                })],
                TypeError::MisplacedList {
                    index: 0,
                    stored: 1,
                },
            ),
        ] {
            let lists = ListTypeTable {
                types: items.into(),
                ..LISTS.clone()
            };
            assert_eq!(
                Types::admit(&lists, &CUSTOMS, &EXTERNALS, &SHAPES).map(|_| ()),
                Err(expected)
            );
        }
    }

    #[test]
    fn rejects_missing_list_item_descriptors_and_incorrect_parameter_items() {
        let cases = [
            (
                ListStorageTypeId::Tuple(TupleListTypeId {
                    list_type: ListTypeId(0),
                    item_type: TupleItemTypeId(9),
                }),
                TypeError::MissingTupleItem { index: 9 },
            ),
            (
                ListStorageTypeId::Function(FunctionListTypeId {
                    list_type: ListTypeId(0),
                    item_type: FunctionItemTypeId(8),
                }),
                TypeError::MissingFunctionItem { index: 8 },
            ),
            (
                ListStorageTypeId::ParameterList(ParameterListListTypeId {
                    list_type: ListTypeId(0),
                    item_type: ParameterListTypeId {
                        list_type: ListTypeId(1),
                        item: crate::plan::TypeParameterId(0),
                    },
                }),
                TypeError::ParameterListItem { index: 0 },
            ),
            (
                ListStorageTypeId::Custom(CustomListTypeId {
                    list_type: ListTypeId(0),
                    item_type: CustomTypeId(7),
                }),
                TypeError::MissingCustom { index: 7 },
            ),
            (
                ListStorageTypeId::External(ExternalListTypeId {
                    list_type: ListTypeId(0),
                    item_type: ExternalTypeId(6),
                }),
                TypeError::MissingExternal { index: 6 },
            ),
        ];
        for (item, expected) in cases {
            let lists = ListTypeTable {
                types: vec![item].into(),
                ..LISTS.clone()
            };
            assert_eq!(
                Types::admit(&lists, &CUSTOMS, &EXTERNALS, &SHAPES).map(|_| ()),
                Err(expected)
            );
        }
    }

    #[test]
    fn rejects_shape_counts_links_cycles_and_type_disagreement() {
        let cases = [
            (
                vec![ValueShapeDescriptor::Int],
                vec![],
                TypeError::ShapeTableLength {
                    shapes: 1,
                    types: 0,
                },
            ),
            (
                vec![ValueShapeDescriptor::Tuple(vec![ValueShapeId(1)].into())],
                vec![ValueType::Tuple(vec![].into())],
                TypeError::MissingShape { index: 1 },
            ),
            (
                vec![ValueShapeDescriptor::Tuple(vec![ValueShapeId(0)].into())],
                vec![ValueType::Tuple(vec![].into())],
                TypeError::RecursiveShape,
            ),
            (
                vec![ValueShapeDescriptor::Custom(CustomValueShapeId(2))],
                vec![ValueType::Nil],
                TypeError::MissingCustomShape { index: 2 },
            ),
            (
                vec![ValueShapeDescriptor::External(ExternalTypeId(3))],
                vec![ValueType::Nil],
                TypeError::MissingExternal { index: 3 },
            ),
            (
                vec![ValueShapeDescriptor::Int],
                vec![ValueType::String],
                TypeError::ShapeType { index: 0 },
            ),
        ];
        for (shapes, shape_types, expected) in cases {
            let shapes = ValueShapeTable {
                shapes: shapes.into(),
                shape_types: shape_types.into(),
                custom_shapes: Table::Static(&[]),
            };
            assert_eq!(
                Types::admit(&LISTS, &CUSTOMS, &EXTERNALS, &shapes).map(|_| ()),
                Err(expected)
            );
        }
    }

    #[test]
    fn rejects_malformed_sparse_constructors_and_custom_shape_arguments() {
        use crate::plan::execution::type_::{
            CustomConstructorRefinement, CustomTypeDescriptor, CustomValueShapeDescriptor,
        };

        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            "pub type Choice(a) { First(a) Second(a) }\npub fn main() { #(First(42), Second(7), [True]) }",
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        assert_eq!(
            Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes
            )
            .err(),
            None,
        );
        assert_eq!(common.custom_types.types.len(), 1);
        assert_eq!(common.custom_types.types[0].constructors.len(), 2);
        type ConstructorChange = fn(&mut CustomTypeDescriptor);
        let mutations: [(ConstructorChange, TypeError); 3] = [
            (
                |descriptor| {
                    let mut constructors = descriptor.constructors.clone().into_vec();
                    constructors[0].id.index = 2;
                    descriptor.constructors = constructors.into();
                },
                TypeError::ConstructorIndex {
                    type_index: 0,
                    index: 2,
                    count: 2,
                },
            ),
            (
                |descriptor| {
                    let mut constructors = descriptor.constructors.clone().into_vec();
                    constructors[0].id.type_id = CustomTypeId(99);
                    descriptor.constructors = constructors.into();
                },
                TypeError::ConstructorOwner {
                    type_index: 0,
                    index: 0,
                },
            ),
            (
                |descriptor| {
                    let mut constructors = descriptor.constructors.clone().into_vec();
                    constructors.reverse();
                    descriptor.constructors = constructors.into();
                },
                TypeError::ConstructorOrder { type_index: 0 },
            ),
        ];
        for (mutate, error) in mutations {
            let mut customs = common.custom_types.as_ref().clone();
            let mut rows = customs.types.into_vec();
            mutate(&mut rows[0]);
            customs.types = rows.into();
            assert_eq!(
                Types::admit(
                    &common.list_types,
                    &customs,
                    &common.external_types,
                    &common.value_shapes
                )
                .err(),
                Some(error),
            );
        }
        type ShapeChange = fn(&mut CustomValueShapeDescriptor);
        let mutations: [(ShapeChange, TypeError); 3] = [
            (
                |shape| shape.type_id = CustomTypeId(99),
                TypeError::MissingCustom { index: 99 },
            ),
            (
                |shape| shape.arguments = Table::Static(&[]),
                TypeError::CustomShapeArguments { index: 0 },
            ),
            (
                |shape| shape.constructor = CustomConstructorRefinement::Exact(2),
                TypeError::ConstructorIndex {
                    type_index: 0,
                    index: 2,
                    count: 2,
                },
            ),
        ];
        for (mutate, error) in mutations {
            let mut shapes = ValueShapeTable {
                shapes: common.value_shapes.shapes.clone(),
                shape_types: common.value_shapes.shape_types.clone(),
                custom_shapes: common.value_shapes.custom_shapes.clone(),
            };
            let mut rows = shapes.custom_shapes.into_vec();
            mutate(&mut rows[0]);
            shapes.custom_shapes = rows.into();
            assert_eq!(
                Types::admit(
                    &common.list_types,
                    &common.custom_types,
                    &common.external_types,
                    &shapes
                )
                .err(),
                Some(error),
            );
        }

        let bool_shape = ValueShapeId(
            common
                .value_shapes
                .shape_types
                .iter()
                .position(|type_| type_ == &ValueType::Bool)
                .unwrap(),
        );
        let mut shapes = ValueShapeTable {
            shapes: common.value_shapes.shapes.clone(),
            shape_types: common.value_shapes.shape_types.clone(),
            custom_shapes: common.value_shapes.custom_shapes.clone(),
        };
        let mut customs = shapes.custom_shapes.into_vec();
        customs[0].arguments = vec![bool_shape].into();
        shapes.custom_shapes = customs.into();
        assert_eq!(
            Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &shapes
            )
            .err(),
            Some(TypeError::CustomShapeArguments { index: 0 }),
        );

        let mut customs = common.custom_types.as_ref().clone();
        let mut descriptors = customs.types.into_vec();
        let mut constructors = descriptors[0].constructors.clone().into_vec();
        let mut fields = constructors[0].fields.clone().into_vec();
        fields[0].shape = bool_shape;
        constructors[0].fields = fields.into();
        descriptors[0].constructors = constructors.into();
        customs.types = descriptors.into();
        assert_eq!(
            Types::admit(
                &common.list_types,
                &customs,
                &common.external_types,
                &common.value_shapes
            )
            .err(),
            Some(TypeError::FieldShape),
        );
    }

    #[test]
    fn list_shapes_must_preserve_the_storage_item_family() {
        let lists = ListTypeTable {
            types: vec![ListStorageTypeId::Int(IntListTypeId {
                list_type: ListTypeId(0),
            })]
            .into(),
            ..LISTS.clone()
        };
        for (item, value, expected) in [
            (ValueShapeDescriptor::Int, ValueType::Int, None),
            (
                ValueShapeDescriptor::Bool,
                ValueType::Bool,
                Some(TypeError::ShapeType { index: 1 }),
            ),
        ] {
            let shapes = ValueShapeTable {
                shapes: vec![item, ValueShapeDescriptor::List(ValueShapeId(0))].into(),
                shape_types: vec![value, ValueType::List(ListTypeId(0))].into(),
                custom_shapes: Table::Static(&[]),
            };
            assert_eq!(
                Types::admit(&lists, &CUSTOMS, &EXTERNALS, &shapes).err(),
                expected
            );
        }
    }
}
