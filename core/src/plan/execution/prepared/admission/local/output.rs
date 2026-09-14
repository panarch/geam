use super::{Address, Family};
use crate::plan::execution::graph::{self, ParamLocal, ParamSlot};
use crate::plan::execution::prepared::admission::type_::{
    FunctionRepresentation, TypeError, Types,
};
use crate::plan::execution::type_::{
    ListStorageTypeId, ValueShapeDescriptor, ValueShapeId, ValueType,
};

pub(in crate::plan::execution::prepared::admission) trait Output {
    fn admit(&self, shape: ValueShapeId, types: &Types<'_>) -> Result<(), TypeError>;
}

impl Output for std::convert::Infallible {
    fn admit(&self, _shape: ValueShapeId, _types: &Types<'_>) -> Result<(), TypeError> {
        match *self {}
    }
}

impl<Id: Copy + Into<Address>> Output for Id {
    fn admit(&self, shape: ValueShapeId, types: &Types<'_>) -> Result<(), TypeError> {
        (*self).into().admit_output(shape, types)
    }
}

impl Address {
    fn admit_output(self, shape: ValueShapeId, types: &Types<'_>) -> Result<(), TypeError> {
        if self.index != 0 || !self.family.matches(types.shape_type(shape)?, types) {
            return Err(TypeError::LocalTypeMismatch);
        }
        if let ValueShapeDescriptor::Function { arguments, return_ } =
            &types.shapes.shapes[shape.index()]
            && types.representation(arguments, *return_) != FunctionRepresentation::Value
        {
            return Err(TypeError::FunctionRepresentation);
        }
        Ok(())
    }
}

impl Output for ParamLocal {
    fn admit(&self, shape: ValueShapeId, types: &Types<'_>) -> Result<(), TypeError> {
        if Address::of(self).index != 0 {
            return Err(TypeError::LocalTypeMismatch);
        }
        types.slot(&ParamSlot {
            local: self.clone(),
            shape,
        })?;
        Ok(())
    }
}

impl Output for graph::CustomLocal {
    fn admit(&self, shape: ValueShapeId, types: &Types<'_>) -> Result<(), TypeError> {
        ParamLocal::Custom(*self).admit(shape, types)
    }
}

impl Output for graph::ExternalLocal {
    fn admit(&self, shape: ValueShapeId, types: &Types<'_>) -> Result<(), TypeError> {
        ParamLocal::External(*self).admit(shape, types)
    }
}

impl Output for graph::GenericFunctionLocal {
    fn admit(&self, shape: ValueShapeId, types: &Types<'_>) -> Result<(), TypeError> {
        ParamLocal::GenericFunction(self.clone()).admit(shape, types)
    }
}

impl Output for graph::NeverFunctionLocal {
    fn admit(&self, shape: ValueShapeId, types: &Types<'_>) -> Result<(), TypeError> {
        ParamLocal::NeverFunction(self.clone()).admit(shape, types)
    }
}

impl Output for graph::CustomFunctionLocal {
    fn admit(&self, shape: ValueShapeId, types: &Types<'_>) -> Result<(), TypeError> {
        ParamLocal::CustomFunction(self.clone()).admit(shape, types)
    }
}

impl Output for graph::ExternalFunctionLocal {
    fn admit(&self, shape: ValueShapeId, types: &Types<'_>) -> Result<(), TypeError> {
        ParamLocal::ExternalFunction(self.clone()).admit(shape, types)
    }
}

impl Output for graph::ListFunctionLocal {
    fn admit(&self, shape: ValueShapeId, types: &Types<'_>) -> Result<(), TypeError> {
        ParamLocal::ListFunction(self.clone()).admit(shape, types)
    }
}

impl Output for graph::FunctionFunctionLocal {
    fn admit(&self, shape: ValueShapeId, types: &Types<'_>) -> Result<(), TypeError> {
        ParamLocal::FunctionFunction(self.clone()).admit(shape, types)
    }
}

impl Output for graph::CoreFunctionFunctionLocal {
    fn admit(&self, shape: ValueShapeId, types: &Types<'_>) -> Result<(), TypeError> {
        graph::FunctionFunctionLocal::Core(self.clone()).admit(shape, types)
    }
}

impl Output for graph::ExternalFunctionFunctionLocal {
    fn admit(&self, shape: ValueShapeId, types: &Types<'_>) -> Result<(), TypeError> {
        graph::FunctionFunctionLocal::External(self.clone()).admit(shape, types)
    }
}

impl Family {
    fn matches(self, type_: &ValueType, types: &Types<'_>) -> bool {
        match type_ {
            ValueType::Int => self == Self::Int,
            ValueType::Float => self == Self::Float,
            ValueType::String => self == Self::String,
            ValueType::BitArray => self == Self::BitArray,
            ValueType::UtfCodepoint => self == Self::UtfCodepoint,
            ValueType::Bool => self == Self::Bool,
            ValueType::Nil => self == Self::Nil,
            ValueType::Tuple(_) => self == Self::Tuple,
            ValueType::Custom(_) => self == Self::Custom,
            ValueType::External(_) => self == Self::External,
            ValueType::List(id) => self.matches_list(types.lists.types[id.index()], false),
            ValueType::Parameter(_) => false,
            ValueType::Function(function) => match function.return_() {
                ValueType::Int => self == Self::IntFunction,
                ValueType::Float => self == Self::FloatFunction,
                ValueType::String => self == Self::StringFunction,
                ValueType::BitArray => self == Self::BitArrayFunction,
                ValueType::UtfCodepoint => self == Self::UtfCodepointFunction,
                ValueType::Bool => self == Self::BoolFunction,
                ValueType::Nil => self == Self::NilFunction,
                ValueType::Tuple(_) => self == Self::TupleFunction,
                ValueType::Custom(_) => self == Self::CustomFunction,
                ValueType::External(_) => self == Self::ExternalFunction,
                ValueType::List(id) => self.matches_list(types.lists.types[id.index()], true),
                ValueType::Parameter(_) => self == Self::NeverFunction,
                ValueType::Function(_) => matches!(
                    self,
                    Self::CoreFunctionFunction | Self::ExternalFunctionFunction
                ),
            },
        }
    }

    fn matches_list(self, type_: ListStorageTypeId, function: bool) -> bool {
        let (list, callable) = match type_ {
            ListStorageTypeId::Parameter(_) => (Self::ParameterList, Self::ParameterListFunction),
            ListStorageTypeId::ParameterList(_) => {
                (Self::ParameterListList, Self::ParameterListListFunction)
            }
            ListStorageTypeId::Int(_) => (Self::IntList, Self::IntListFunction),
            ListStorageTypeId::String(_) => (Self::StringList, Self::StringListFunction),
            ListStorageTypeId::BitArray(_) => (Self::BitArrayList, Self::BitArrayListFunction),
            ListStorageTypeId::UtfCodepoint(_) => {
                (Self::UtfCodepointList, Self::UtfCodepointListFunction)
            }
            ListStorageTypeId::Custom(_) => (Self::CustomList, Self::CustomListFunction),
            ListStorageTypeId::External(_) => (Self::ExternalList, Self::ExternalListFunction),
            ListStorageTypeId::Float(_) => (Self::FloatList, Self::FloatListFunction),
            ListStorageTypeId::Bool(_) => (Self::BoolList, Self::BoolListFunction),
            ListStorageTypeId::Nil(_) => (Self::NilList, Self::NilListFunction),
            ListStorageTypeId::Tuple(_) => (Self::TupleList, Self::TupleListFunction),
            ListStorageTypeId::List(_) => (Self::ListList, Self::ListListFunction),
            ListStorageTypeId::Function(_) => (Self::FunctionList, Self::FunctionListFunction),
        };
        self == if function { callable } else { list }
    }
}

#[cfg(test)]
mod tests {
    use super::{Output, ParamLocal, TypeError, Types, graph};
    use crate::plan::execution::type_::ValueShapeId;

    #[test]
    fn return_outputs_preserve_each_source_family_and_reject_other_shapes() {
        let source = r#"
pub type Box(a) { Box(a) }
fn capture(value) { fn() { echo value 42 } }
fn unchanged(value) { value }
fn stop(_value: Int) -> a { panic }
fn point() { let assert <<value:utf8_codepoint>> = <<65>> value }
pub fn main() {
  let _ = #(capture(42), capture(1.5), capture("text"), capture(True), capture(Nil),
    capture(<<1>>), capture(point()), capture(#(1, True)), capture(Box(7)))
  let _ = #(capture([42]), capture([1.5]), capture(["text"]), capture([True]), capture([Nil]),
    capture([<<1>>]), capture([point()]), capture([#(1, True)]), capture([Box(7)]),
    capture([]), capture([[]]), capture([[42]]), capture([unchanged]))
  let _ = #(capture(fn() { 42 }), capture(fn() { 1.5 }), capture(fn() { "text" }),
    capture(fn() { True }), capture(fn() { Nil }), capture(fn() { <<1>> }),
    capture(fn() { point() }), capture(fn() { #(1, True) }), capture(fn() { Box(7) }),
    capture(unchanged), capture(stop))
  let _ = #(capture(fn() { [42] }), capture(fn() { [1.5] }), capture(fn() { ["text"] }),
    capture(fn() { [True] }), capture(fn() { [Nil] }), capture(fn() { [<<1>>] }),
    capture(fn() { [point()] }), capture(fn() { [#(1, True)] }), capture(fn() { [Box(7)] }),
    capture(fn() { [] }), capture(fn() { [[]] }), capture(fn() { [[42]] }),
    capture(fn() { [unchanged] }), capture(fn() { unchanged }))
  42
}
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
        let functions = &plan
            .program
            .functions
            .function_returns
            .int_function_functions;
        assert_eq!(functions.len(), 47);
        for function in functions.iter() {
            let block = function
                .body()
                .function_body()
                .block_graph()
                .blocks()
                .next()
                .unwrap();
            assert_eq!(block.params().len(), 1);
            let slot = &block.params()[0];
            assert_eq!(slot.local.admit(slot.shape, &types), Ok(()));
            let output = typed_output(&slot.local);
            assert_eq!(output.admit(slot.shape, &types), Ok(()));
            assert_eq!(
                graph::IntLocalId(1).admit(slot.shape, &types),
                Err(TypeError::LocalTypeMismatch)
            );
            assert_eq!(
                ParamLocal::Int(graph::IntLocalId(1)).admit(slot.shape, &types),
                Err(TypeError::LocalTypeMismatch)
            );
        }
        assert_eq!(
            graph::IntLocalId(0).admit(ValueShapeId(usize::MAX), &types),
            Err(TypeError::MissingShape { index: usize::MAX })
        );
        for (index, type_) in types.shape_types().iter().enumerate() {
            if type_ != &crate::plan::execution::type_::ValueType::Int {
                assert_eq!(
                    graph::IntLocalId(0).admit(ValueShapeId(index), &types),
                    Err(TypeError::LocalTypeMismatch)
                );
            }
        }
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()),
            Ok(crate::Value::Int(42.into()))
        );
    }

    #[test]
    fn external_outputs_keep_nominal_list_and_nested_callable_metadata() {
        use crate::plan::execution::graph::ParamSlot;
        use crate::plan::execution::storage::Table;
        use crate::plan::execution::type_::{
            CustomTypeTable, ExternalFunctionType, ExternalListTypeId, ExternalTypeId,
            ExternalTypeTable, FunctionFunctionType, FunctionShape, FunctionType,
            ListStorageTypeId, ListTypeId, ListTypeTable, NominalTypeMetadata,
            ValueShapeDescriptor, ValueShapeTable, ValueType,
        };
        let key = ExternalTypeId(0);
        let key_list = ExternalListTypeId::new(ListTypeId(0), key);
        let lists = ListTypeTable {
            types: vec![ListStorageTypeId::External(key_list)].into(),
            tuple_items: Table::Static(&[]),
            function_items: Table::Static(&[]),
        };
        let customs = CustomTypeTable {
            definitions: Table::Static(&[]),
            types: Table::Static(&[]),
        };
        let externals = ExternalTypeTable {
            types: vec![NominalTypeMetadata {
                package: "app".into(),
                module: "main".into(),
                name: "Key".into(),
                arguments: Table::Static(&[]),
            }]
            .into(),
        };
        let key_function = FunctionType::new(vec![], ValueType::External(key));
        let list_function = FunctionType::new(vec![], ValueType::List(ListTypeId(0)));
        let outer_function = FunctionType::new(vec![], ValueType::Function(key_function.clone()));
        let shapes = ValueShapeTable {
            shapes: vec![
                ValueShapeDescriptor::External(key),
                ValueShapeDescriptor::List(ValueShapeId(0)),
                ValueShapeDescriptor::Function {
                    arguments: Table::Static(&[]),
                    return_: ValueShapeId(0),
                },
                ValueShapeDescriptor::Function {
                    arguments: Table::Static(&[]),
                    return_: ValueShapeId(2),
                },
                ValueShapeDescriptor::Function {
                    arguments: Table::Static(&[]),
                    return_: ValueShapeId(1),
                },
            ]
            .into(),
            shape_types: vec![
                ValueType::External(key),
                ValueType::List(ListTypeId(0)),
                ValueType::Function(key_function.clone()),
                ValueType::Function(outer_function.clone()),
                ValueType::Function(list_function.clone()),
            ]
            .into(),
            custom_shapes: Table::Static(&[]),
        };
        let types = Types::admit(&lists, &customs, &externals, &shapes).unwrap();
        let slots = [
            ParamSlot::new(
                ParamLocal::External(graph::ExternalLocal {
                    id: graph::ExternalLocalId(0),
                    type_id: key,
                }),
                ValueShapeId(0),
            ),
            ParamSlot::new(
                ParamLocal::List(graph::ListLocal::External {
                    local: graph::ExternalListLocalId(0),
                    type_id: key_list,
                }),
                ValueShapeId(1),
            ),
            ParamSlot::new(
                ParamLocal::ExternalFunction(graph::ExternalFunctionLocal {
                    id: graph::ExternalFunctionLocalId(0),
                    type_: ExternalFunctionType {
                        type_: key_function.clone(),
                        arguments: Table::Static(&[]),
                        return_: key,
                    },
                }),
                ValueShapeId(2),
            ),
            ParamSlot::new(
                ParamLocal::FunctionFunction(graph::FunctionFunctionLocal::External(
                    graph::ExternalFunctionFunctionLocal {
                        id: graph::ExternalFunctionFunctionLocalId(0),
                        type_: FunctionFunctionType {
                            type_: outer_function,
                            arguments: Table::Static(&[]),
                            return_: FunctionShape {
                                type_: key_function,
                                shape_id: ValueShapeId(2),
                            },
                        },
                    },
                )),
                ValueShapeId(3),
            ),
            ParamSlot::new(
                ParamLocal::ListFunction(graph::ListFunctionLocal::External {
                    local: graph::ExternalListFunctionLocalId(0),
                    type_: list_function,
                    list_type: key_list,
                }),
                ValueShapeId(4),
            ),
        ];
        for slot in &slots {
            assert_eq!(slot.local.admit(slot.shape, &types), Ok(()));
            assert_eq!(typed_output(&slot.local).admit(slot.shape, &types), Ok(()));
            assert_eq!(
                graph::IntLocalId(0).admit(slot.shape, &types),
                Err(TypeError::LocalTypeMismatch)
            );
        }
        assert_eq!(
            typed_output(&slots[0].local).admit(ValueShapeId(1), &types),
            Err(TypeError::LocalTypeMismatch)
        );
    }

    #[test]
    fn ordinary_function_outputs_do_not_admit_symbolic_callable_storage() {
        let source = "fn discard(value) { echo value 42 } pub fn main() { echo 42 discard }";
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
        let functions = &plan
            .program
            .functions
            .function_returns
            .generic_function_functions;
        assert_eq!(functions.len(), 1);
        let function = functions[0].body().function_body();
        let locals: Vec<_> = function
            .block_graph()
            .blocks()
            .flat_map(|block| block.instructions())
            .filter_map(|instruction| match &instruction.output().local {
                ParamLocal::GenericFunction(local) => Some(local),
                _ => None,
            })
            .collect();
        assert_eq!(locals.len(), 1);
        let local = locals[0];
        let shape = local.type_.shape.shape_id;
        assert_eq!(local.admit(shape, &types), Ok(()));
        assert_eq!(
            graph::IntFunctionLocalId(0).admit(shape, &types),
            Err(TypeError::FunctionRepresentation)
        );
    }

    fn typed_output(local: &ParamLocal) -> Box<dyn Output> {
        match local {
            ParamLocal::Int(id) => Box::new(*id),
            ParamLocal::Float(id) => Box::new(*id),
            ParamLocal::String(id) => Box::new(*id),
            ParamLocal::BitArray(id) => Box::new(*id),
            ParamLocal::UtfCodepoint(id) => Box::new(*id),
            ParamLocal::Bool(id) => Box::new(*id),
            ParamLocal::Nil(id) => Box::new(*id),
            ParamLocal::Tuple { local, .. } => Box::new(*local),
            ParamLocal::Custom(local) => Box::new(*local),
            ParamLocal::External(local) => Box::new(*local),
            ParamLocal::GenericFunction(local) => Box::new(local.clone()),
            ParamLocal::NeverFunction(local) => Box::new(local.clone()),
            ParamLocal::CustomFunction(local) => Box::new(local.clone()),
            ParamLocal::ExternalFunction(local) => Box::new(local.clone()),
            ParamLocal::ListFunction(local) => Box::new(local.clone()),
            ParamLocal::FunctionFunction(graph::FunctionFunctionLocal::Core(local)) => {
                Box::new(local.clone())
            }
            ParamLocal::FunctionFunction(graph::FunctionFunctionLocal::External(local)) => {
                Box::new(local.clone())
            }
            ParamLocal::IntFunction { local, .. } => Box::new(*local),
            ParamLocal::FloatFunction { local, .. } => Box::new(*local),
            ParamLocal::StringFunction { local, .. } => Box::new(*local),
            ParamLocal::BitArrayFunction { local, .. } => Box::new(*local),
            ParamLocal::UtfCodepointFunction { local, .. } => Box::new(*local),
            ParamLocal::BoolFunction { local, .. } => Box::new(*local),
            ParamLocal::NilFunction { local, .. } => Box::new(*local),
            ParamLocal::TupleFunction { local, .. } => Box::new(*local),
            ParamLocal::List(local) => match local {
                graph::ListLocal::Parameter { local, .. } => Box::new(*local),
                graph::ListLocal::ParameterList { local, .. } => Box::new(*local),
                graph::ListLocal::Int { local, .. } => Box::new(*local),
                graph::ListLocal::String { local, .. } => Box::new(*local),
                graph::ListLocal::BitArray { local, .. } => Box::new(*local),
                graph::ListLocal::UtfCodepoint { local, .. } => Box::new(*local),
                graph::ListLocal::Custom { local, .. } => Box::new(*local),
                graph::ListLocal::External { local, .. } => Box::new(*local),
                graph::ListLocal::Float { local, .. } => Box::new(*local),
                graph::ListLocal::Bool { local, .. } => Box::new(*local),
                graph::ListLocal::Nil { local, .. } => Box::new(*local),
                graph::ListLocal::Tuple { local, .. } => Box::new(*local),
                graph::ListLocal::List { local, .. } => Box::new(*local),
                graph::ListLocal::Function { local, .. } => Box::new(*local),
            },
        }
    }
}
