use super::local::{Address, LocalError, Locals};
use super::type_::Slot;
use crate::plan::execution::graph::{self, ParamLocal};

pub(super) trait Operand {
    fn read<'data>(&self, locals: &Locals<'data>) -> Result<Slot<'data, 'data>, LocalError>;
}

impl Operand for std::convert::Infallible {
    fn read<'data>(&self, _locals: &Locals<'data>) -> Result<Slot<'data, 'data>, LocalError> {
        match *self {}
    }
}

impl<Id: Copy + Into<Address>> Operand for Id {
    fn read<'data>(&self, locals: &Locals<'data>) -> Result<Slot<'data, 'data>, LocalError> {
        locals.get(*self)
    }
}

impl Operand for ParamLocal {
    fn read<'data>(&self, locals: &Locals<'data>) -> Result<Slot<'data, 'data>, LocalError> {
        locals.parameter(self)
    }
}

impl Operand for graph::CustomLocal {
    fn read<'data>(&self, locals: &Locals<'data>) -> Result<Slot<'data, 'data>, LocalError> {
        locals.parameter(&ParamLocal::Custom(*self))
    }
}

impl Operand for graph::ExternalLocal {
    fn read<'data>(&self, locals: &Locals<'data>) -> Result<Slot<'data, 'data>, LocalError> {
        locals.parameter(&ParamLocal::External(*self))
    }
}

impl Operand for graph::ListLocal {
    fn read<'data>(&self, locals: &Locals<'data>) -> Result<Slot<'data, 'data>, LocalError> {
        locals.parameter(&ParamLocal::List(self.clone()))
    }
}

impl Operand for graph::GenericFunctionLocal {
    fn read<'data>(&self, locals: &Locals<'data>) -> Result<Slot<'data, 'data>, LocalError> {
        locals.parameter(&ParamLocal::GenericFunction(self.clone()))
    }
}

impl Operand for graph::NeverFunctionLocal {
    fn read<'data>(&self, locals: &Locals<'data>) -> Result<Slot<'data, 'data>, LocalError> {
        locals.parameter(&ParamLocal::NeverFunction(self.clone()))
    }
}

impl Operand for graph::CustomFunctionLocal {
    fn read<'data>(&self, locals: &Locals<'data>) -> Result<Slot<'data, 'data>, LocalError> {
        locals.parameter(&ParamLocal::CustomFunction(self.clone()))
    }
}

impl Operand for graph::ExternalFunctionLocal {
    fn read<'data>(&self, locals: &Locals<'data>) -> Result<Slot<'data, 'data>, LocalError> {
        locals.parameter(&ParamLocal::ExternalFunction(self.clone()))
    }
}

impl Operand for graph::ListFunctionLocal {
    fn read<'data>(&self, locals: &Locals<'data>) -> Result<Slot<'data, 'data>, LocalError> {
        locals.parameter(&ParamLocal::ListFunction(self.clone()))
    }
}

impl Operand for graph::FunctionFunctionLocal {
    fn read<'data>(&self, locals: &Locals<'data>) -> Result<Slot<'data, 'data>, LocalError> {
        locals.parameter(&ParamLocal::FunctionFunction(self.clone()))
    }
}

impl Operand for graph::FunctionLocal {
    fn read<'data>(&self, locals: &Locals<'data>) -> Result<Slot<'data, 'data>, LocalError> {
        match self {
            Self::Generic(value) => value.read(locals),
            Self::Never(value) => value.read(locals),
            Self::Int(value) => value.read(locals),
            Self::Float(value) => value.read(locals),
            Self::String(value) => value.read(locals),
            Self::BitArray(value) => value.read(locals),
            Self::UtfCodepoint(value) => value.read(locals),
            Self::Custom(value) => value.read(locals),
            Self::External(value) => value.read(locals),
            Self::Bool(value) => value.read(locals),
            Self::Nil(value) => value.read(locals),
            Self::Tuple(value) => value.read(locals),
            Self::List(value) => value.read(locals),
            Self::Function(value) => value.read(locals),
        }
    }
}

impl Operand for graph::CoreFunctionFunctionLocal {
    fn read<'data>(&self, locals: &Locals<'data>) -> Result<Slot<'data, 'data>, LocalError> {
        graph::FunctionFunctionLocal::Core(self.clone()).read(locals)
    }
}

impl Operand for graph::ExternalFunctionFunctionLocal {
    fn read<'data>(&self, locals: &Locals<'data>) -> Result<Slot<'data, 'data>, LocalError> {
        graph::FunctionFunctionLocal::External(self.clone()).read(locals)
    }
}

impl Operand for graph::StoredListLocal {
    fn read<'data>(&self, locals: &Locals<'data>) -> Result<Slot<'data, 'data>, LocalError> {
        match self {
            Self::ParameterList(value) => value.read(locals),
            Self::Int(value) => value.read(locals),
            Self::String(value) => value.read(locals),
            Self::BitArray(value) => value.read(locals),
            Self::UtfCodepoint(value) => value.read(locals),
            Self::Custom(value) => value.read(locals),
            Self::External(value) => value.read(locals),
            Self::Float(value) => value.read(locals),
            Self::Bool(value) => value.read(locals),
            Self::Nil(value) => value.read(locals),
            Self::Tuple(value) => value.read(locals),
            Self::List(value) => value.read(locals),
            Self::Function(value) => value.read(locals),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Address, LocalError, Locals, Operand, ParamLocal, graph};
    use crate::plan::execution::graph::ParamSlot;
    use crate::plan::execution::prepared::admission::type_::Types;
    use crate::plan::execution::storage::Table;
    use crate::plan::execution::type_::{
        CustomTypeTable, ExternalFunctionType, ExternalListTypeId, ExternalTypeId,
        ExternalTypeTable, FunctionFunctionType, FunctionShape, FunctionType, ListStorageTypeId,
        ListTypeId, ListTypeTable, NominalTypeMetadata, ValueShapeDescriptor, ValueShapeId,
        ValueShapeTable, ValueType,
    };

    #[test]
    fn every_plain_operand_family_preserves_its_admitted_local_and_metadata() {
        let source = r#"
pub type Box(a) { Box(a) }
fn capture(value) { fn() { echo value 42 } }
fn unchanged(value) { value }
fn stop(_value: Int) -> a { panic }
fn codepoint() { let assert <<value:utf8_codepoint>> = <<65>> value }
pub fn main() {
  let _ = #(capture(42), capture(1.5), capture("text"), capture(True), capture(Nil),
    capture(<<1>>), capture(codepoint()), capture(#(1, True)), capture(Box(7)))
  let _ = #(capture([42]), capture([1.5]), capture(["text"]), capture([True]), capture([Nil]),
    capture([<<1>>]), capture([codepoint()]), capture([#(1, True)]), capture([Box(7)]),
    capture([]), capture([[]]), capture([[42]]), capture([unchanged]))
  let _ = #(capture(fn() { 42 }), capture(fn() { 1.5 }), capture(fn() { "text" }),
    capture(fn() { True }), capture(fn() { Nil }), capture(fn() { <<1>> }),
    capture(fn() { codepoint() }), capture(fn() { #(1, True) }),
    capture(fn() { Box(7) }), capture(unchanged), capture(stop))
  let _ = #(capture(fn() { [42] }), capture(fn() { [1.5] }), capture(fn() { ["text"] }),
    capture(fn() { [True] }), capture(fn() { [Nil] }), capture(fn() { [<<1>>] }),
    capture(fn() { [codepoint()] }), capture(fn() { [#(1, True)] }),
    capture(fn() { [Box(7)] }), capture(fn() { [] }), capture(fn() { [[]] }),
    capture(fn() { [[42]] }), capture(fn() { [unchanged] }))
  let _ = capture(fn() { unchanged })
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
            let graph = function.body().function_body().block_graph();
            let parameters = graph.blocks().next().unwrap().params();
            assert_eq!(parameters.len(), 1);
            assert_readers(&parameters[0], &types);
        }
    }

    #[test]
    fn external_operands_keep_the_same_read_contract_without_a_payload_or_provider() {
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
            assert_readers(slot, &types);
        }
    }

    fn assert_readers<'data>(slot: &'data ParamSlot, types: &Types<'data>) {
        let mut readers: Vec<Box<dyn Operand>> = vec![Box::new(slot.local.clone())];
        match &slot.local {
            ParamLocal::Int(local) => readers.push(Box::new(*local)),
            ParamLocal::Float(local) => readers.push(Box::new(*local)),
            ParamLocal::String(local) => readers.push(Box::new(*local)),
            ParamLocal::BitArray(local) => readers.push(Box::new(*local)),
            ParamLocal::UtfCodepoint(local) => readers.push(Box::new(*local)),
            ParamLocal::Bool(local) => readers.push(Box::new(*local)),
            ParamLocal::Nil(local) => readers.push(Box::new(*local)),
            ParamLocal::Tuple { local, .. } => readers.push(Box::new(*local)),
            ParamLocal::Custom(local) => {
                readers.push(Box::new(local.id));
                readers.push(Box::new(*local));
            }
            ParamLocal::External(local) => {
                readers.push(Box::new(local.id));
                readers.push(Box::new(*local));
            }
            ParamLocal::List(local) => {
                readers.push(Box::new(local.clone()));
                match local {
                    graph::ListLocal::Parameter { local, .. } => {
                        readers.push(Box::new(*local));
                    }
                    graph::ListLocal::ParameterList { local, .. } => {
                        readers.push(Box::new(*local));
                        readers.push(Box::new(graph::StoredListLocal::ParameterList(*local)));
                    }
                    graph::ListLocal::Int { local, .. } => {
                        readers.push(Box::new(*local));
                        readers.push(Box::new(graph::StoredListLocal::Int(*local)));
                    }
                    graph::ListLocal::String { local, .. } => {
                        readers.push(Box::new(*local));
                        readers.push(Box::new(graph::StoredListLocal::String(*local)));
                    }
                    graph::ListLocal::BitArray { local, .. } => {
                        readers.push(Box::new(*local));
                        readers.push(Box::new(graph::StoredListLocal::BitArray(*local)));
                    }
                    graph::ListLocal::UtfCodepoint { local, .. } => {
                        readers.push(Box::new(*local));
                        readers.push(Box::new(graph::StoredListLocal::UtfCodepoint(*local)));
                    }
                    graph::ListLocal::Custom { local, .. } => {
                        readers.push(Box::new(*local));
                        readers.push(Box::new(graph::StoredListLocal::Custom(*local)));
                    }
                    graph::ListLocal::External { local, .. } => {
                        readers.push(Box::new(*local));
                        readers.push(Box::new(graph::StoredListLocal::External(*local)));
                    }
                    graph::ListLocal::Float { local, .. } => {
                        readers.push(Box::new(*local));
                        readers.push(Box::new(graph::StoredListLocal::Float(*local)));
                    }
                    graph::ListLocal::Bool { local, .. } => {
                        readers.push(Box::new(*local));
                        readers.push(Box::new(graph::StoredListLocal::Bool(*local)));
                    }
                    graph::ListLocal::Nil { local, .. } => {
                        readers.push(Box::new(*local));
                        readers.push(Box::new(graph::StoredListLocal::Nil(*local)));
                    }
                    graph::ListLocal::Tuple { local, .. } => {
                        readers.push(Box::new(*local));
                        readers.push(Box::new(graph::StoredListLocal::Tuple(*local)));
                    }
                    graph::ListLocal::List { local, .. } => {
                        readers.push(Box::new(*local));
                        readers.push(Box::new(graph::StoredListLocal::List(*local)));
                    }
                    graph::ListLocal::Function { local, .. } => {
                        readers.push(Box::new(*local));
                        readers.push(Box::new(graph::StoredListLocal::Function(*local)));
                    }
                }
            }
            ParamLocal::IntFunction { local, .. } => {
                readers.push(Box::new(*local));
                readers.push(Box::new(graph::FunctionLocal::Int(*local)));
            }
            ParamLocal::FloatFunction { local, .. } => {
                readers.push(Box::new(*local));
                readers.push(Box::new(graph::FunctionLocal::Float(*local)));
            }
            ParamLocal::StringFunction { local, .. } => {
                readers.push(Box::new(*local));
                readers.push(Box::new(graph::FunctionLocal::String(*local)));
            }
            ParamLocal::BitArrayFunction { local, .. } => {
                readers.push(Box::new(*local));
                readers.push(Box::new(graph::FunctionLocal::BitArray(*local)));
            }
            ParamLocal::UtfCodepointFunction { local, .. } => {
                readers.push(Box::new(*local));
                readers.push(Box::new(graph::FunctionLocal::UtfCodepoint(*local)));
            }
            ParamLocal::BoolFunction { local, .. } => {
                readers.push(Box::new(*local));
                readers.push(Box::new(graph::FunctionLocal::Bool(*local)));
            }
            ParamLocal::NilFunction { local, .. } => {
                readers.push(Box::new(*local));
                readers.push(Box::new(graph::FunctionLocal::Nil(*local)));
            }
            ParamLocal::TupleFunction { local, .. } => {
                readers.push(Box::new(*local));
                readers.push(Box::new(graph::FunctionLocal::Tuple(*local)));
            }
            ParamLocal::GenericFunction(local) => {
                readers.push(Box::new(local.id));
                readers.push(Box::new(local.clone()));
                readers.push(Box::new(graph::FunctionLocal::Generic(local.clone())));
            }

            ParamLocal::NeverFunction(local) => {
                readers.push(Box::new(local.id));
                readers.push(Box::new(local.clone()));
                readers.push(Box::new(graph::FunctionLocal::Never(local.clone())));
            }

            ParamLocal::CustomFunction(local) => {
                readers.push(Box::new(local.id));
                readers.push(Box::new(local.clone()));
                readers.push(Box::new(graph::FunctionLocal::Custom(local.clone())));
            }

            ParamLocal::ExternalFunction(local) => {
                readers.push(Box::new(local.id));
                readers.push(Box::new(local.clone()));
                readers.push(Box::new(graph::FunctionLocal::External(local.clone())));
            }
            ParamLocal::ListFunction(local) => {
                readers.push(Box::new(local.clone()));
                readers.push(Box::new(graph::FunctionLocal::List(local.clone())));
                match local {
                    graph::ListFunctionLocal::Parameter { local, .. } => {
                        readers.push(Box::new(*local))
                    }
                    graph::ListFunctionLocal::ParameterList { local, .. } => {
                        readers.push(Box::new(*local))
                    }
                    graph::ListFunctionLocal::Int { local, .. } => readers.push(Box::new(*local)),
                    graph::ListFunctionLocal::String { local, .. } => {
                        readers.push(Box::new(*local))
                    }
                    graph::ListFunctionLocal::BitArray { local, .. } => {
                        readers.push(Box::new(*local))
                    }
                    graph::ListFunctionLocal::UtfCodepoint { local, .. } => {
                        readers.push(Box::new(*local))
                    }
                    graph::ListFunctionLocal::Custom { local, .. } => {
                        readers.push(Box::new(*local))
                    }
                    graph::ListFunctionLocal::External { local, .. } => {
                        readers.push(Box::new(*local))
                    }
                    graph::ListFunctionLocal::Float { local, .. } => readers.push(Box::new(*local)),
                    graph::ListFunctionLocal::Bool { local, .. } => readers.push(Box::new(*local)),
                    graph::ListFunctionLocal::Nil { local, .. } => readers.push(Box::new(*local)),
                    graph::ListFunctionLocal::Tuple { local, .. } => readers.push(Box::new(*local)),
                    graph::ListFunctionLocal::List { local, .. } => readers.push(Box::new(*local)),
                    graph::ListFunctionLocal::Function { local, .. } => {
                        readers.push(Box::new(*local))
                    }
                }
            }
            ParamLocal::FunctionFunction(local) => {
                readers.push(Box::new(local.clone()));
                readers.push(Box::new(graph::FunctionLocal::Function(local.clone())));
                match local {
                    graph::FunctionFunctionLocal::Core(local) => {
                        readers.push(Box::new(local.id));
                        readers.push(Box::new(local.clone()));
                    }
                    graph::FunctionFunctionLocal::External(local) => {
                        readers.push(Box::new(local.id));
                        readers.push(Box::new(local.clone()));
                    }
                }
            }
        }
        let mut locals = Locals::default();
        let missing = Address::of(&slot.local);
        for reader in &readers {
            assert_eq!(reader.read(&locals), Err(LocalError::Missing(missing)));
        }
        locals.define(slot, types).unwrap();
        for reader in readers {
            let actual = reader.read(&locals).unwrap();
            assert_eq!(*actual, *slot);
            assert_eq!(actual.type_, types.shape_type(slot.shape).unwrap());
            assert_eq!(actual.descriptor, types.shape(slot.shape).unwrap());
            assert!(std::ptr::eq(&*actual, slot));
        }
    }
}
