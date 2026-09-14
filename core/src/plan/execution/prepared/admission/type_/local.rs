use super::{TypeError, TypeRef, Types, ValueShapeDescriptor};
use crate::plan::execution::graph::{
    FunctionFunctionLocal, ListFunctionLocal, ListLocal, ParamLocal, ParamSlot,
};
use crate::plan::execution::type_::{
    CustomValueShape, FunctionShape, FunctionType, ListStorageTypeId, ListTypeId, ValueShapeId,
    ValueType,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::plan::execution::prepared::admission) struct Slot<'slot, 'data> {
    raw: &'slot ParamSlot,
    pub(in crate::plan::execution::prepared::admission) type_: &'data ValueType,
    pub(in crate::plan::execution::prepared::admission) descriptor: &'data ValueShapeDescriptor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::plan::execution::prepared::admission) struct TupleSlot<'data> {
    pub(in crate::plan::execution::prepared::admission) slot: Slot<'data, 'data>,
    pub(in crate::plan::execution::prepared::admission) elements: &'data [ValueShapeId],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::plan::execution::prepared::admission) struct ListSlot<'data> {
    pub(in crate::plan::execution::prepared::admission) slot: Slot<'data, 'data>,
    pub(in crate::plan::execution::prepared::admission) item: ValueShapeId,
}

impl<'data> Types<'data> {
    pub(in crate::plan::execution::prepared::admission) fn tuple_slot(
        &self,
        raw: &'data ParamSlot,
    ) -> Result<TupleSlot<'data>, TypeError> {
        let ValueShapeDescriptor::Tuple(elements) = self.shape(raw.shape)? else {
            return Err(TypeError::LocalTypeMismatch);
        };
        Ok(TupleSlot {
            slot: self.slot(raw)?,
            elements,
        })
    }

    pub(in crate::plan::execution::prepared::admission) fn list_slot(
        &self,
        raw: &'data ParamSlot,
    ) -> Result<ListSlot<'data>, TypeError> {
        let ValueShapeDescriptor::List(item) = self.shape(raw.shape)? else {
            return Err(TypeError::LocalTypeMismatch);
        };
        Ok(ListSlot {
            slot: self.slot(raw)?,
            item: *item,
        })
    }

    pub(in crate::plan::execution::prepared::admission) fn slot<'slot>(
        &self,
        slot: &'slot ParamSlot,
    ) -> Result<Slot<'slot, 'data>, TypeError> {
        let type_ = self.shape_type(slot.shape)?;
        let descriptor = &self.shapes.shapes[slot.shape.index()];
        if &self.local_type(&slot.local)? != type_ {
            return Err(TypeError::LocalTypeMismatch);
        }
        let consistent = match (&slot.local, descriptor) {
            (ParamLocal::Custom(local), ValueShapeDescriptor::Custom(id)) => {
                *id == local.shape.shape_id
            }
            (ParamLocal::GenericFunction(local), _) => {
                self.flow(slot.shape, local.type_.shape.shape_id)
                    && self.flow(local.type_.shape.shape_id, slot.shape)
            }
            (ParamLocal::NeverFunction(local), _) => {
                self.flow(slot.shape, local.type_.shape.shape_id)
                    && self.flow(local.type_.shape.shape_id, slot.shape)
            }
            (
                ParamLocal::CustomFunction(local),
                ValueShapeDescriptor::Function { arguments, return_ },
            ) => {
                self.same_argument_shapes(arguments, &local.type_.arguments)
                    && matches!(&self.shapes.shapes[return_.index()], ValueShapeDescriptor::Custom(id) if *id == local.type_.return_.shape_id)
            }
            (
                ParamLocal::ExternalFunction(local),
                ValueShapeDescriptor::Function { arguments, .. },
            ) => self.same_argument_shapes(arguments, &local.type_.arguments),
            (
                ParamLocal::FunctionFunction(local),
                ValueShapeDescriptor::Function { arguments, return_ },
            ) => {
                let type_ = match local {
                    FunctionFunctionLocal::Core(local) => &local.type_,
                    FunctionFunctionLocal::External(local) => &local.type_,
                };
                self.same_argument_shapes(arguments, &type_.arguments)
                    && self.flow(*return_, type_.return_.shape_id)
                    && self.flow(type_.return_.shape_id, *return_)
            }
            _ => true,
        };
        if !consistent {
            return Err(TypeError::LocalTypeMismatch);
        }
        self.function_slot(slot, descriptor)?;
        Ok(Slot {
            raw: slot,
            type_,
            descriptor,
        })
    }

    fn same_argument_shapes(&self, actual: &[ValueShapeId], expected: &[ValueShapeId]) -> bool {
        // Nominal slot agreement has already checked arity and every referenced shape.
        actual.iter().zip(expected).all(|(actual, expected)| {
            self.flow(*actual, *expected) && self.flow(*expected, *actual)
        })
    }

    pub(in crate::plan::execution::prepared::admission) fn local_type(
        &self,
        local: &ParamLocal,
    ) -> Result<ValueType, TypeError> {
        let function = match local {
            ParamLocal::Int(_) => return Ok(ValueType::Int),
            ParamLocal::Float(_) => return Ok(ValueType::Float),
            ParamLocal::String(_) => return Ok(ValueType::String),
            ParamLocal::BitArray(_) => return Ok(ValueType::BitArray),
            ParamLocal::UtfCodepoint(_) => return Ok(ValueType::UtfCodepoint),
            ParamLocal::Bool(_) => return Ok(ValueType::Bool),
            ParamLocal::Nil(_) => return Ok(ValueType::Nil),
            ParamLocal::Custom(local) => {
                return self
                    .custom_value_shape(local.shape)
                    .map(|shape| ValueType::Custom(shape.type_id));
            }
            ParamLocal::External(local) => {
                self.externals.types.get(local.type_id.index()).ok_or(
                    TypeError::MissingExternal {
                        index: local.type_id.index(),
                    },
                )?;
                return Ok(ValueType::External(local.type_id));
            }
            ParamLocal::Tuple { type_, .. } => {
                self.walk(type_.iter().map(TypeRef::Value).collect())?;
                return Ok(ValueType::Tuple(type_.clone()));
            }
            ParamLocal::List(local) => return self.local_list_type(local).map(ValueType::List),
            ParamLocal::IntFunction { type_, .. } => {
                self.require_return(type_, &ValueType::Int)?;
                type_
            }
            ParamLocal::FloatFunction { type_, .. } => {
                self.require_return(type_, &ValueType::Float)?;
                type_
            }
            ParamLocal::StringFunction { type_, .. } => {
                self.require_return(type_, &ValueType::String)?;
                type_
            }
            ParamLocal::BitArrayFunction { type_, .. } => {
                self.require_return(type_, &ValueType::BitArray)?;
                type_
            }
            ParamLocal::UtfCodepointFunction { type_, .. } => {
                self.require_return(type_, &ValueType::UtfCodepoint)?;
                type_
            }
            ParamLocal::BoolFunction { type_, .. } => {
                self.require_return(type_, &ValueType::Bool)?;
                type_
            }
            ParamLocal::NilFunction { type_, .. } => {
                self.require_return(type_, &ValueType::Nil)?;
                type_
            }
            ParamLocal::TupleFunction { type_, .. } => {
                self.check_function(type_)?;
                if !matches!(type_.return_(), ValueType::Tuple(_)) {
                    return Err(TypeError::LocalTypeMismatch);
                }
                type_
            }
            ParamLocal::GenericFunction(local) => {
                self.function_shape(&local.type_.shape)?;
                if local.type_.type_ != local.type_.shape.type_ {
                    return Err(TypeError::LocalTypeMismatch);
                }
                &local.type_.type_
            }
            ParamLocal::NeverFunction(local) => {
                self.function_shape(&local.type_.shape)?;
                if local.type_.type_ != local.type_.shape.type_ {
                    return Err(TypeError::LocalTypeMismatch);
                }
                &local.type_.type_
            }
            ParamLocal::CustomFunction(local) => {
                self.require_return(
                    &local.type_.type_,
                    &ValueType::Custom(self.custom_value_shape(local.type_.return_)?.type_id),
                )?;
                self.argument_shapes(&local.type_.type_, &local.type_.arguments)?;
                &local.type_.type_
            }
            ParamLocal::ExternalFunction(local) => {
                self.require_return(
                    &local.type_.type_,
                    &ValueType::External(local.type_.return_),
                )?;
                self.argument_shapes(&local.type_.type_, &local.type_.arguments)?;
                &local.type_.type_
            }
            ParamLocal::ListFunction(local) => {
                let (type_, list) = match local {
                    ListFunctionLocal::Parameter {
                        type_, list_type, ..
                    } => (type_, ListStorageTypeId::Parameter(*list_type)),
                    ListFunctionLocal::ParameterList {
                        type_, list_type, ..
                    } => (type_, ListStorageTypeId::ParameterList(*list_type)),
                    ListFunctionLocal::Int {
                        type_, list_type, ..
                    } => (type_, ListStorageTypeId::Int(*list_type)),
                    ListFunctionLocal::String {
                        type_, list_type, ..
                    } => (type_, ListStorageTypeId::String(*list_type)),
                    ListFunctionLocal::BitArray {
                        type_, list_type, ..
                    } => (type_, ListStorageTypeId::BitArray(*list_type)),
                    ListFunctionLocal::UtfCodepoint {
                        type_, list_type, ..
                    } => (type_, ListStorageTypeId::UtfCodepoint(*list_type)),
                    ListFunctionLocal::Custom {
                        type_, list_type, ..
                    } => (type_, ListStorageTypeId::Custom(*list_type)),
                    ListFunctionLocal::External {
                        type_, list_type, ..
                    } => (type_, ListStorageTypeId::External(*list_type)),
                    ListFunctionLocal::Float {
                        type_, list_type, ..
                    } => (type_, ListStorageTypeId::Float(*list_type)),
                    ListFunctionLocal::Bool {
                        type_, list_type, ..
                    } => (type_, ListStorageTypeId::Bool(*list_type)),
                    ListFunctionLocal::Nil {
                        type_, list_type, ..
                    } => (type_, ListStorageTypeId::Nil(*list_type)),
                    ListFunctionLocal::Tuple {
                        type_, list_type, ..
                    } => (type_, ListStorageTypeId::Tuple(*list_type)),
                    ListFunctionLocal::List {
                        type_, list_type, ..
                    } => (type_, ListStorageTypeId::List(*list_type)),
                    ListFunctionLocal::Function {
                        type_, list_type, ..
                    } => (type_, ListStorageTypeId::Function(*list_type)),
                };
                let list_type = self.list_storage(list)?;
                self.require_return(type_, &ValueType::List(list_type))?;
                type_
            }
            ParamLocal::FunctionFunction(local) => {
                let type_ = match local {
                    FunctionFunctionLocal::Core(local) => &local.type_,
                    FunctionFunctionLocal::External(local) => &local.type_,
                };
                self.function_shape(&type_.return_)?;
                self.check_function(&type_.type_)?;
                if !matches!(type_.type_.return_(), ValueType::Function(return_) if return_ == &type_.return_.type_)
                {
                    return Err(TypeError::LocalTypeMismatch);
                }
                self.argument_shapes(&type_.type_, &type_.arguments)?;
                &type_.type_
            }
        };
        Ok(ValueType::Function(function.clone()))
    }

    pub(in crate::plan::execution::prepared::admission) fn shape_type(
        &self,
        id: ValueShapeId,
    ) -> Result<&'data ValueType, TypeError> {
        self.shapes
            .shape_types
            .get(id.index())
            .ok_or(TypeError::MissingShape { index: id.index() })
    }

    fn local_list_type(&self, local: &ListLocal) -> Result<ListTypeId, TypeError> {
        let stored = match local {
            ListLocal::Parameter { type_id, .. } => ListStorageTypeId::Parameter(*type_id),
            ListLocal::ParameterList { type_id, .. } => ListStorageTypeId::ParameterList(*type_id),
            ListLocal::Int { type_id, .. } => ListStorageTypeId::Int(*type_id),
            ListLocal::String { type_id, .. } => ListStorageTypeId::String(*type_id),
            ListLocal::BitArray { type_id, .. } => ListStorageTypeId::BitArray(*type_id),
            ListLocal::UtfCodepoint { type_id, .. } => ListStorageTypeId::UtfCodepoint(*type_id),
            ListLocal::Custom { type_id, .. } => ListStorageTypeId::Custom(*type_id),
            ListLocal::External { type_id, .. } => ListStorageTypeId::External(*type_id),
            ListLocal::Float { type_id, .. } => ListStorageTypeId::Float(*type_id),
            ListLocal::Bool { type_id, .. } => ListStorageTypeId::Bool(*type_id),
            ListLocal::Nil { type_id, .. } => ListStorageTypeId::Nil(*type_id),
            ListLocal::Tuple { type_id, .. } => ListStorageTypeId::Tuple(*type_id),
            ListLocal::List { type_id, .. } => ListStorageTypeId::List(*type_id),
            ListLocal::Function { type_id, .. } => ListStorageTypeId::Function(*type_id),
        };
        self.list_storage(stored)
    }

    pub(in crate::plan::execution::prepared::admission) fn list_storage(
        &self,
        stored: ListStorageTypeId,
    ) -> Result<ListTypeId, TypeError> {
        let id = stored.list_type();
        let expected = self
            .lists
            .types
            .get(id.index())
            .ok_or(TypeError::MissingList { index: id.index() })?;
        if expected != &stored {
            return Err(TypeError::ListStorageMismatch { index: id.index() });
        }
        Ok(id)
    }

    pub(in crate::plan::execution::prepared::admission) fn custom_value_shape(
        &self,
        shape: CustomValueShape,
    ) -> Result<&'data crate::plan::execution::type_::CustomValueShapeDescriptor, TypeError> {
        let stored = self.custom_shape_descriptor(shape.shape_id)?;
        if stored.type_id != shape.type_id {
            return Err(TypeError::LocalTypeMismatch);
        }
        Ok(stored)
    }

    fn check_function(&self, function: &FunctionType) -> Result<(), TypeError> {
        let mut roots = Vec::new();
        Self::function_roots(function, &mut roots);
        self.walk(roots)
    }

    fn require_return(
        &self,
        function: &FunctionType,
        expected: &ValueType,
    ) -> Result<(), TypeError> {
        self.check_function(function)?;
        if function.return_() != expected {
            return Err(TypeError::LocalTypeMismatch);
        }
        Ok(())
    }

    pub(in crate::plan::execution::prepared::admission) fn function_shape(
        &self,
        shape: &FunctionShape,
    ) -> Result<(), TypeError> {
        self.check_function(&shape.type_)?;
        if !matches!(self.shape_type(shape.shape_id)?, ValueType::Function(type_) if type_ == &shape.type_)
        {
            return Err(TypeError::LocalTypeMismatch);
        }
        Ok(())
    }

    fn argument_shapes(
        &self,
        type_: &FunctionType,
        shapes: &[ValueShapeId],
    ) -> Result<(), TypeError> {
        if type_.arguments.len() != shapes.len() {
            return Err(TypeError::LocalTypeMismatch);
        }
        for (type_, shape) in type_.arguments.iter().zip(shapes) {
            if type_ != self.shape_type(*shape)? {
                return Err(TypeError::LocalTypeMismatch);
            }
        }
        Ok(())
    }
}

impl std::ops::Deref for Slot<'_, '_> {
    type Target = ParamSlot;

    fn deref(&self) -> &Self::Target {
        self.raw
    }
}

#[cfg(test)]
mod tests {
    use super::{FunctionType, ParamLocal, ParamSlot, TypeError, Types, ValueShapeId, ValueType};
    use crate::plan::execution::function::FunctionBodyOwner;
    use crate::plan::execution::graph::{
        IntFunctionLocalId, IntLocalId, ProfiledBlockGraph, StringLocalId, TupleLocalId,
    };
    use crate::plan::execution::storage::{Node, Table};
    use crate::plan::execution::type_::{
        CustomTypeTable, ExternalTypeTable, ListTypeTable, ValueShapeDescriptor, ValueShapeTable,
    };
    use std::convert::Infallible;

    #[test]
    fn admits_primitive_and_nested_slots_in_tuple_returning_bodies() {
        for source in [
            "pub fn main() { #(1, 1.5, \"text\", True, Nil, <<1, 2>>) }",
            "pub fn main() { #([1], [1.5], [\"text\"], [True], [Nil], [<<1>>], [[1]], [#(1, True)]) }",
            "fn make(x) { [x] } pub fn main() { #(make(1), make(\"text\"), [fn(x) { x + 1 }]) }",
        ] {
            let module =
                crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
            let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(module).unwrap());
            let common = &plan.program.common;
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            let functions = &plan.program.functions.value_returns.tuple_functions;
            assert_eq!(functions.len(), 1);
            assert_graph_slots(&types, functions[0].body().function_body().block_graph());
        }
    }

    #[test]
    fn admits_captured_parameters_in_integer_closure_bodies() {
        let source = "fn make(x) { fn(y) { x + y } } pub fn main() { make(2)(4) }";
        let module = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(module).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let functions = &plan.program.functions.value_returns.int_functions;
        assert_eq!(functions.len(), 2);
        for function in functions.iter() {
            assert_graph_slots(&types, function.body().function_body().block_graph());
        }
    }

    #[test]
    fn admits_recursive_and_sparse_custom_constructor_slots() {
        for source in [
            "pub type Chain { End Link(Chain) } pub fn main() { Link(Link(End)) }",
            "pub type Choice { First Second(String) Third(Int) } pub fn main() { Third(3) }",
        ] {
            let module =
                crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
            let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(module).unwrap());
            let common = &plan.program.common;
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            let functions = &plan.program.functions.value_returns.custom_functions;
            assert_eq!(functions.len(), 1);
            assert_graph_slots(&types, functions[0].body().function_body().block_graph());
        }
    }

    #[test]
    fn rejects_missing_or_disagreeing_slot_shapes_and_function_returns() {
        let lists = ListTypeTable {
            types: Table::Static(&[]),
            tuple_items: Table::Static(&[]),
            function_items: Table::Static(&[]),
        };
        let customs = CustomTypeTable {
            definitions: Table::Static(&[]),
            types: Table::Static(&[]),
        };
        let externals = ExternalTypeTable {
            types: Table::Static(&[]),
        };
        let shapes = ValueShapeTable {
            shapes: Table::Static(&[ValueShapeDescriptor::Int]),
            shape_types: Table::Static(&[ValueType::Int]),
            custom_shapes: Table::Static(&[]),
        };
        let types = Types::admit(&lists, &customs, &externals, &shapes).unwrap();
        let raw = ParamSlot {
            local: ParamLocal::Int(IntLocalId(0)),
            shape: ValueShapeId(0),
        };
        let slot = types.slot(&raw).unwrap();
        assert_eq!(*slot, raw);
        assert_eq!(slot.type_, &ValueType::Int);
        assert_eq!(slot.descriptor, &ValueShapeDescriptor::Int);
        assert!(std::ptr::eq(&*slot, &raw));
        assert!(std::ptr::eq(slot.type_, &shapes.shape_types[0]));
        assert!(std::ptr::eq(slot.descriptor, &shapes.shapes[0]));
        assert_eq!(
            types.slot(&ParamSlot {
                local: ParamLocal::Int(IntLocalId(0)),
                shape: ValueShapeId(1)
            }),
            Err(TypeError::MissingShape { index: 1 })
        );
        assert_eq!(
            types.slot(&ParamSlot {
                local: ParamLocal::String(StringLocalId(0)),
                shape: ValueShapeId(0)
            }),
            Err(TypeError::LocalTypeMismatch)
        );
        let wrong = ParamLocal::IntFunction {
            local: IntFunctionLocalId(0),
            type_: FunctionType {
                arguments: Table::Static(&[]),
                return_: Node::Static(&ValueType::String),
            },
        };
        assert_eq!(types.local_type(&wrong), Err(TypeError::LocalTypeMismatch));
    }

    #[test]
    fn rejects_cyclic_function_metadata_and_preserves_static_tuple_fields() {
        static RECURSIVE: ValueType = ValueType::Function(FunctionType {
            arguments: Table::Static(&[]),
            return_: Node::Static(&RECURSIVE),
        });
        let lists = ListTypeTable {
            types: Table::Static(&[]),
            tuple_items: Table::Static(&[]),
            function_items: Table::Static(&[]),
        };
        let customs = CustomTypeTable {
            definitions: Table::Static(&[]),
            types: Table::Static(&[]),
        };
        let externals = ExternalTypeTable {
            types: Table::Static(&[]),
        };
        let shapes = ValueShapeTable {
            shapes: Table::Static(&[]),
            shape_types: Table::Static(&[]),
            custom_shapes: Table::Static(&[]),
        };
        let types = Types::admit(&lists, &customs, &externals, &shapes).unwrap();
        let recursive = ParamLocal::IntFunction {
            local: IntFunctionLocalId(0),
            type_: FunctionType {
                arguments: Table::Static(&[]),
                return_: Node::Static(&RECURSIVE),
            },
        };
        assert_eq!(types.local_type(&recursive), Err(TypeError::RecursiveType));
        static FIELDS: &[ValueType] = &[
            ValueType::Int,
            ValueType::Tuple(Table::Static(&[ValueType::Bool])),
        ];
        let local = ParamLocal::Tuple {
            local: TupleLocalId(0),
            type_: Table::Static(FIELDS),
        };
        assert_eq!(
            types.local_type(&local),
            Ok(ValueType::Tuple(Table::Static(FIELDS)))
        );
    }

    #[test]
    fn callable_locals_reject_disagreeing_signatures_before_slot_admission() {
        use crate::plan::execution::graph;
        let source = r#"
pub type Box { Box(Int) }
fn identity(value) { value }
fn stop() -> a { panic }
pub fn main() { #(Box, identity, stop, fn() { fn() { 42 } }, fn() { #(42) }) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let wrong = FunctionType::new(vec![], ValueType::Int);
        for local in [
            ParamLocal::FloatFunction {
                local: graph::FloatFunctionLocalId(0),
                type_: wrong.clone(),
            },
            ParamLocal::StringFunction {
                local: graph::StringFunctionLocalId(0),
                type_: wrong.clone(),
            },
            ParamLocal::BitArrayFunction {
                local: graph::BitArrayFunctionLocalId(0),
                type_: wrong.clone(),
            },
            ParamLocal::UtfCodepointFunction {
                local: graph::UtfCodepointFunctionLocalId(0),
                type_: wrong.clone(),
            },
            ParamLocal::BoolFunction {
                local: graph::BoolFunctionLocalId(0),
                type_: wrong.clone(),
            },
            ParamLocal::NilFunction {
                local: graph::NilFunctionLocalId(0),
                type_: wrong.clone(),
            },
            ParamLocal::TupleFunction {
                local: graph::TupleFunctionLocalId(0),
                type_: wrong.clone(),
            },
        ] {
            assert_eq!(types.local_type(&local), Err(TypeError::LocalTypeMismatch));
        }
        let graph = plan.program.functions.value_returns.tuple_functions[0]
            .body()
            .block_graph();
        let mut checked = [0; 5];
        for instruction in graph.blocks().flat_map(|block| block.instructions()) {
            let slot = instruction.output();
            let mut local = slot.local.clone();
            match &mut local {
                ParamLocal::CustomFunction(value) => {
                    value.type_.arguments = Table::Static(&[]);
                    checked[0] += 1;
                }
                ParamLocal::GenericFunction(value) => {
                    value.type_.type_ = wrong.clone();
                    checked[1] += 1;
                }
                ParamLocal::NeverFunction(value) => {
                    value.type_.type_ = wrong.clone();
                    checked[2] += 1;
                }
                ParamLocal::FunctionFunction(graph::FunctionFunctionLocal::Core(value)) => {
                    value.type_.type_ = wrong.clone();
                    checked[3] += 1;
                }
                ParamLocal::TupleFunction { type_, .. } => {
                    *type_ = wrong.clone();
                    checked[4] += 1;
                }
                _ => continue,
            }
            assert_eq!(*types.slot(slot).unwrap(), *slot);
            assert_eq!(types.local_type(&local), Err(TypeError::LocalTypeMismatch));
            assert_eq!(
                types.slot(&ParamSlot {
                    local,
                    shape: slot.shape
                }),
                Err(TypeError::LocalTypeMismatch)
            );
        }
        assert_eq!(checked, [1, 1, 1, 1, 1]);
    }

    #[test]
    fn local_metadata_rejects_missing_links_and_disagreeing_callable_fields() {
        use crate::plan::execution::graph;
        use crate::plan::execution::type_::{
            ExternalTypeId, IntListTypeId, ListStorageTypeId, ListTypeId,
        };

        let source = r#"
pub type Box { Box(Int) }
pub type Key
@external(erlang, "native", "key") fn key() -> Key
fn identity(value) { value }
fn stop() -> a { panic }
pub fn main() {
  #(Box, identity, stop, fn(value: Int) { fn() { value } },
    fn(value: Int) { echo value key() }, fn() { [42] })
}
"#;
        let (program, _, _) = super::super::super::tests::lowered_native(source);
        let common = &program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let missing = ValueType::List(ListTypeId(99));
        for local in [
            ParamLocal::External(graph::ExternalLocal {
                id: graph::ExternalLocalId(0),
                type_id: ExternalTypeId(99),
            }),
            ParamLocal::Tuple {
                local: TupleLocalId(0),
                type_: vec![ValueType::External(ExternalTypeId(99))].into(),
            },
        ] {
            assert_eq!(
                types.local_type(&local),
                Err(TypeError::MissingExternal { index: 99 })
            );
        }
        assert_eq!(
            types.local_type(&ParamLocal::TupleFunction {
                local: graph::TupleFunctionLocalId(0),
                type_: FunctionType::new(
                    vec![missing.clone()],
                    ValueType::Tuple(Table::Static(&[]))
                ),
            }),
            Err(TypeError::MissingList { index: 99 })
        );
        assert_eq!(
            types.local_type(&ParamLocal::IntFunction {
                local: graph::IntFunctionLocalId(0),
                type_: FunctionType::new(vec![missing.clone()], ValueType::Int),
            }),
            Err(TypeError::MissingList { index: 99 }),
        );
        let function_shape = ValueShapeId(
            common
                .value_shapes
                .shapes
                .iter()
                .position(|shape| {
                    matches!(
                        shape,
                        crate::plan::execution::type_::ValueShapeDescriptor::Function { .. }
                    )
                })
                .unwrap(),
        );
        assert_eq!(
            types.function_shape(&crate::plan::execution::type_::FunctionShape {
                shape_id: function_shape,
                type_: FunctionType::new(vec![], missing.clone()),
            }),
            Err(TypeError::MissingList { index: 99 }),
        );
        assert_eq!(
            types.function_shape(&crate::plan::execution::type_::FunctionShape {
                shape_id: function_shape,
                type_: FunctionType::new(Vec::new(), ValueType::Bool),
            }),
            Err(TypeError::LocalTypeMismatch),
        );
        assert_eq!(
            types.argument_shapes(
                &FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
                &[function_shape]
            ),
            Err(TypeError::LocalTypeMismatch),
        );
        let graphs = program
            .functions
            .value_returns
            .tuple_functions
            .iter()
            .map(|entry| {
                crate::plan::execution::prepared::admission::tests::graph_body(entry).block_graph()
            })
            .collect::<Vec<_>>();
        assert_eq!(graphs.len(), 1);
        let graph = graphs[0];
        let mut checked = [0; 6];
        for instruction in graph.blocks().flat_map(|block| block.instructions()) {
            let slot = instruction.output();
            let cases = match &slot.local {
                ParamLocal::CustomFunction(value) => {
                    checked[0] += 1;
                    let mut missing_return = value.clone();
                    missing_return.type_.return_.shape_id =
                        crate::plan::execution::type_::CustomValueShapeId(99);
                    let mut wrong_return = value.clone();
                    wrong_return.type_.type_.return_ = Box::new(ValueType::Bool).into();
                    let mut missing_argument = value.clone();
                    missing_argument.type_.arguments = Table::Static(&[ValueShapeId(99)]);
                    vec![
                        (
                            ParamLocal::CustomFunction(missing_return),
                            TypeError::MissingCustomShape { index: 99 },
                        ),
                        (
                            ParamLocal::CustomFunction(wrong_return),
                            TypeError::LocalTypeMismatch,
                        ),
                        (
                            ParamLocal::CustomFunction(missing_argument),
                            TypeError::MissingShape { index: 99 },
                        ),
                    ]
                }
                ParamLocal::GenericFunction(value) => {
                    checked[1] += 1;
                    let mut missing = value.clone();
                    missing.type_.shape.shape_id = ValueShapeId(99);
                    vec![(
                        ParamLocal::GenericFunction(missing),
                        TypeError::MissingShape { index: 99 },
                    )]
                }
                ParamLocal::NeverFunction(value) => {
                    checked[2] += 1;
                    let mut missing = value.clone();
                    missing.type_.shape.shape_id = ValueShapeId(99);
                    vec![(
                        ParamLocal::NeverFunction(missing),
                        TypeError::MissingShape { index: 99 },
                    )]
                }
                ParamLocal::FunctionFunction(graph::FunctionFunctionLocal::Core(value)) => {
                    checked[3] += 1;
                    let mut missing_shape = value.clone();
                    missing_shape.type_.return_.shape_id = ValueShapeId(99);
                    let mut missing_type = value.clone();
                    missing_type.type_.type_.arguments = vec![missing.clone()].into();
                    let mut missing_argument = value.clone();
                    missing_argument.type_.arguments = Table::Static(&[ValueShapeId(99)]);
                    vec![
                        (
                            ParamLocal::FunctionFunction(graph::FunctionFunctionLocal::Core(
                                missing_shape,
                            )),
                            TypeError::MissingShape { index: 99 },
                        ),
                        (
                            ParamLocal::FunctionFunction(graph::FunctionFunctionLocal::Core(
                                missing_type,
                            )),
                            TypeError::MissingList { index: 99 },
                        ),
                        (
                            ParamLocal::FunctionFunction(graph::FunctionFunctionLocal::Core(
                                missing_argument,
                            )),
                            TypeError::MissingShape { index: 99 },
                        ),
                    ]
                }
                ParamLocal::ExternalFunction(value) => {
                    checked[4] += 1;
                    let mut wrong_return = value.clone();
                    wrong_return.type_.type_.return_ = Box::new(ValueType::Bool).into();
                    let mut missing_argument = value.clone();
                    missing_argument.type_.arguments = Table::Static(&[ValueShapeId(99)]);
                    vec![
                        (
                            ParamLocal::ExternalFunction(wrong_return),
                            TypeError::LocalTypeMismatch,
                        ),
                        (
                            ParamLocal::ExternalFunction(missing_argument),
                            TypeError::MissingShape { index: 99 },
                        ),
                    ]
                }
                ParamLocal::ListFunction(graph::ListFunctionLocal::Int {
                    type_,
                    list_type,
                    local,
                }) => {
                    checked[5] += 1;
                    let missing_list = graph::ListFunctionLocal::Int {
                        type_: type_.clone(),
                        list_type: IntListTypeId {
                            list_type: ListTypeId(99),
                        },
                        local: *local,
                    };
                    let wrong_return = graph::ListFunctionLocal::Int {
                        type_: FunctionType::new(vec![], ValueType::Bool),
                        list_type: *list_type,
                        local: *local,
                    };
                    assert_eq!(
                        types.list_storage(ListStorageTypeId::Bool(
                            crate::plan::execution::type_::BoolListTypeId {
                                list_type: list_type.list_type
                            }
                        )),
                        Err(TypeError::ListStorageMismatch {
                            index: list_type.list_type.index()
                        })
                    );
                    vec![
                        (
                            ParamLocal::ListFunction(missing_list),
                            TypeError::MissingList { index: 99 },
                        ),
                        (
                            ParamLocal::ListFunction(wrong_return),
                            TypeError::LocalTypeMismatch,
                        ),
                    ]
                }
                _ => continue,
            };
            assert_eq!(*types.slot(slot).unwrap(), *slot);
            for (local, expected) in cases {
                assert_eq!(types.local_type(&local), Err(expected));
            }
        }
        assert_eq!(checked, [1; 6]);
    }

    #[test]
    fn custom_locals_preserve_nominal_identity_and_exact_constructor_refinements() {
        use crate::plan::execution::type_::{CustomTypeId, CustomValueShapeId};
        let source =
            "pub type Choice { Left(Int) Right(Int) } pub fn main() { #(Left(1), Right(2)) }";
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let graph = plan.program.functions.value_returns.tuple_functions[0]
            .body()
            .block_graph();
        let locals: Vec<_> = graph
            .blocks()
            .flat_map(|block| block.instructions())
            .filter_map(|instruction| match instruction.output().local {
                ParamLocal::Custom(local) => Some((instruction.output(), local)),
                _ => None,
            })
            .collect();
        assert_eq!(locals.len(), 2);
        let (left_slot, left) = locals[0];
        let (_, right) = locals[1];
        assert_eq!(
            types.slot(&ParamSlot {
                local: ParamLocal::Custom(right),
                shape: left_slot.shape,
            }),
            Err(TypeError::LocalTypeMismatch)
        );
        let mut wrong_owner = left;
        wrong_owner.shape.type_id = CustomTypeId(99);
        assert_eq!(
            types.local_type(&ParamLocal::Custom(wrong_owner)),
            Err(TypeError::LocalTypeMismatch)
        );
        let mut missing = left;
        missing.shape.shape_id = CustomValueShapeId(99);
        assert_eq!(
            types.local_type(&ParamLocal::Custom(missing)),
            Err(TypeError::MissingCustomShape { index: 99 })
        );
        assert_eq!(*types.slot(left_slot).unwrap(), *left_slot);
    }

    fn assert_graph_slots(types: &Types<'_>, graph: &ProfiledBlockGraph<Infallible>) {
        for block in graph.blocks() {
            for parameter in block.params() {
                assert_eq!(*types.slot(parameter).unwrap(), *parameter);
            }
            for instruction in block.instructions() {
                assert_eq!(
                    *types.slot(instruction.output()).unwrap(),
                    *instruction.output()
                );
            }
        }
    }
}
