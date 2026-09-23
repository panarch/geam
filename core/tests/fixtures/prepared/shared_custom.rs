data::HostedModuleArtifact {
    module: data::ModuleArtifact {
        format: 4,
        program: data::ProgramTables {
            root: data::source::module_id(1),
            modules: data::Storage::Static(&[
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("handles"),
                    source_context: Some(data::source::SourceContext::from_static_block("handles.gleam", r#"

pub opaque type Handle(a) { Handle(a) }
pub fn make(value: a) { Handle(value) }
pub fn get(value: Handle(a)) { case value { Handle(item) -> item } }
"#)),
                },
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("consumer"),
                    source_context: Some(data::source::SourceContext::from_static_block("consumer.gleam", r#"

import handles
@external(erlang, "native", "retain")
fn retain(value: handles.Handle(a)) -> handles.Handle(a)
@external(erlang, "native", "increment")
fn increment(value: handles.Handle(Int)) -> handles.Handle(Int)
pub fn main() {
  let integer = handles.make(41) |> retain |> increment |> handles.get
  let callback = handles.make(fn(value) { value + 2 }) |> retain |> handles.get
  #(integer, callback(40))
}
"#)),
                },
            ]),
            main: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Tuple {
                id: data::function::TupleFunctionId(0),
                return_type: data::Storage::Static(&[
                    data::type_::ValueType::Int,
                    data::type_::ValueType::Int,
                ]),
            }),
            functions: data::function::FunctionTables {
                value_returns: data::function::ValueFunctionTables {
                    never_functions: data::Storage::Static(&[]),
                    int_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 1,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::CustomField {
                                                source: data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                },
                                                index: 0,
                                            }),
                                        },
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 1,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    2,
                                                ]),
                                            })),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntLocalId(0),
                                                right: data::graph::IntLocalId(1),
                                            }),
                                        },
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(2)),
                                ]),
                            },
                        })),
                    ]),
                    float_functions: data::Storage::Static(&[]),
                    string_functions: data::Storage::Static(&[]),
                    bit_array_functions: data::Storage::Static(&[]),
                    utf_codepoint_functions: data::Storage::Static(&[]),
                    custom_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 1,
                            },
                            body: data::function::ProfiledCustomFunctionBody {
                                _signature_shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(0),
                                    shape_id: data::type_::CustomValueShapeId(0),
                                },
                                _body_shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(0),
                                    shape_id: data::type_::CustomValueShapeId(2),
                                },
                                body: data::function::ProfiledFunctionBody {
                                    block_graph: data::graph::ProfiledBlockGraph {
                                        entry: data::graph::BlockId(0),
                                        blocks: data::Storage::Static(&[
                                            data::graph::BlockHeader {
                                                params: 0..1,
                                                instructions: 0..1,
                                                terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                            },
                                        ]),
                                        params: data::Storage::Static(&[
                                            data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                        ]),
                                        instructions: data::Storage::Static(&[
                                            data::graph::ProfiledInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(2),
                                                        },
                                                    }),
                                                    shape: data::type_::ValueShapeId(5),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                    constructor: data::type_::CustomConstructorId {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        index: 0,
                                                    },
                                                    fields: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                }),
                                            },
                                        ]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::Return(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(2),
                                            },
                                        }),
                                    ]),
                                },
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 0,
                            return_: data::graph::CustomLocal {
                                id: data::graph::CustomLocalId(0),
                                shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(0),
                                    shape_id: data::type_::CustomValueShapeId(0),
                                },
                            },
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 1,
                            return_: data::graph::CustomLocal {
                                id: data::graph::CustomLocalId(0),
                                shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(0),
                                    shape_id: data::type_::CustomValueShapeId(0),
                                },
                            },
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 1,
                            },
                            body: data::function::ProfiledCustomFunctionBody {
                                _signature_shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(1),
                                    shape_id: data::type_::CustomValueShapeId(1),
                                },
                                _body_shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(1),
                                    shape_id: data::type_::CustomValueShapeId(3),
                                },
                                body: data::function::ProfiledFunctionBody {
                                    block_graph: data::graph::ProfiledBlockGraph {
                                        entry: data::graph::BlockId(0),
                                        blocks: data::Storage::Static(&[
                                            data::graph::BlockHeader {
                                                params: 0..1,
                                                instructions: 0..1,
                                                terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                            },
                                        ]),
                                        params: data::Storage::Static(&[
                                            data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::IntFunction {
                                                    local: data::graph::IntFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                        ]),
                                        instructions: data::Storage::Static(&[
                                            data::graph::ProfiledInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(3),
                                                        },
                                                    }),
                                                    shape: data::type_::ValueShapeId(6),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                    constructor: data::type_::CustomConstructorId {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        index: 0,
                                                    },
                                                    fields: data::Storage::Static(&[
                                                        data::graph::ParamLocal::IntFunction {
                                                            local: data::graph::IntFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                            },
                                                        },
                                                    ]),
                                                }),
                                            },
                                        ]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::Return(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(3),
                                            },
                                        }),
                                    ]),
                                },
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 2,
                            return_: data::graph::CustomLocal {
                                id: data::graph::CustomLocalId(0),
                                shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(1),
                                    shape_id: data::type_::CustomValueShapeId(1),
                                },
                            },
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    external_functions: data::Storage::Static(&[]),
                    bool_functions: data::Storage::Static(&[]),
                    nil_functions: data::Storage::Static(&[]),
                    tuple_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..0,
                                            instructions: 0..12,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    41,
                                                ]),
                                            })),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Call {
                                                function: data::function::CustomFunctionId {
                                                    index: 0,
                                                    return_shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("consumer", "main", data::source::SourceSpan::new(248, 264)),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Call {
                                                function: data::function::CustomFunctionId {
                                                    index: 1,
                                                    return_shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(0),
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("consumer", "main", data::source::SourceSpan::new(268, 274)),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(2),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Call {
                                                function: data::function::CustomFunctionId {
                                                    index: 2,
                                                    return_shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(1),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(0),
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("consumer", "main", data::source::SourceSpan::new(278, 287)),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(2),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(0),
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("consumer", "main", data::source::SourceSpan::new(291, 302)),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::IntFunction {
                                                    local: data::graph::IntFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                                family: data::function::FunctionReturnFamily::Int,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(1)),
                                                    captures: data::Storage::Static(&[]),
                                                },
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(3),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(1),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Call {
                                                function: data::function::CustomFunctionId {
                                                    index: 3,
                                                    return_shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(1),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::IntFunction {
                                                        local: data::graph::IntFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        },
                                                    },
                                                ]),
                                                site: data::source::HostCallSite::from_static("consumer", "main", data::source::SourceSpan::new(320, 357)),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(4),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(1),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Call {
                                                function: data::function::CustomFunctionId {
                                                    index: 4,
                                                    return_shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(1),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(3),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(1),
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("consumer", "main", data::source::SourceSpan::new(361, 367)),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::IntFunction {
                                                    local: data::graph::IntFunctionLocalId(1),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                                family: data::function::FunctionReturnFamily::Int,
                                                kind: data::graph::FunctionInstructionKind::Call {
                                                    function: data::function::ProfiledFunctionFunctionId::Int(data::function::IntFunctionFunctionId(0)),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                            id: data::graph::CustomLocalId(4),
                                                            shape: data::type_::CustomValueShape {
                                                                type_id: data::type_::CustomTypeId(1),
                                                                shape_id: data::type_::CustomValueShapeId(1),
                                                            },
                                                        }),
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("consumer", "main", data::source::SourceSpan::new(371, 382)),
                                                },
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    40,
                                                ]),
                                            })),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::FunctionCall {
                                                function: data::graph::IntFunctionLocalId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("consumer", "main", data::source::SourceSpan::new(396, 408)),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                },
                                                shape: data::type_::ValueShapeId(4),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            ]))),
                                        },
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::TupleLocalId(0)),
                                ]),
                            },
                        })),
                    ]),
                },
                list_returns: data::function::ListFunctionTables {
                    parameter_list_functions: data::Storage::Static(&[]),
                    int_list_functions: data::Storage::Static(&[]),
                    string_list_functions: data::Storage::Static(&[]),
                    bit_array_list_functions: data::Storage::Static(&[]),
                    utf_codepoint_list_functions: data::Storage::Static(&[]),
                    custom_list_functions: data::Storage::Static(&[]),
                    external_list_functions: data::Storage::Static(&[]),
                    float_list_functions: data::Storage::Static(&[]),
                    bool_list_functions: data::Storage::Static(&[]),
                    nil_list_functions: data::Storage::Static(&[]),
                    tuple_list_functions: data::Storage::Static(&[]),
                    parameter_list_list_functions: data::Storage::Static(&[]),
                    list_list_functions: data::Storage::Static(&[]),
                    function_list_functions: data::Storage::Static(&[]),
                },
                function_returns: data::function::FunctionFunctionTables {
                    int_function_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 1,
                            },
                            body: data::function::TypedFunctionBody {
                                _shape: data::type_::FunctionShape {
                                    shape_id: data::type_::ValueShapeId(1),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    },
                                },
                                body: data::function::ProfiledFunctionBody {
                                    block_graph: data::graph::ProfiledBlockGraph {
                                        entry: data::graph::BlockId(0),
                                        blocks: data::Storage::Static(&[
                                            data::graph::BlockHeader {
                                                params: 0..1,
                                                instructions: 0..1,
                                                terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                            },
                                        ]),
                                        params: data::Storage::Static(&[
                                            data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(1),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                        ]),
                                        instructions: data::Storage::Static(&[
                                            data::graph::ProfiledInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::IntFunction {
                                                        local: data::graph::IntFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        },
                                                    },
                                                    shape: data::type_::ValueShapeId(1),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                    family: data::function::FunctionReturnFamily::Int,
                                                    kind: data::graph::FunctionInstructionKind::CustomField {
                                                        source: data::graph::CustomLocal {
                                                            id: data::graph::CustomLocalId(0),
                                                            shape: data::type_::CustomValueShape {
                                                                type_id: data::type_::CustomTypeId(1),
                                                                shape_id: data::type_::CustomValueShapeId(1),
                                                            },
                                                        },
                                                        index: 0,
                                                    },
                                                }),
                                            },
                                        ]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::Return(data::graph::IntFunctionLocalId(0)),
                                    ]),
                                },
                            },
                        })),
                    ]),
                    float_function_functions: data::Storage::Static(&[]),
                    string_function_functions: data::Storage::Static(&[]),
                    bit_array_function_functions: data::Storage::Static(&[]),
                    utf_codepoint_function_functions: data::Storage::Static(&[]),
                    custom_function_functions: data::Storage::Static(&[]),
                    external_function_functions: data::Storage::Static(&[]),
                    bool_function_functions: data::Storage::Static(&[]),
                    nil_function_functions: data::Storage::Static(&[]),
                    tuple_function_functions: data::Storage::Static(&[]),
                    generic_function_functions: data::Storage::Static(&[]),
                    never_function_functions: data::Storage::Static(&[]),
                    parameter_list_function_functions: data::Storage::Static(&[]),
                    parameter_list_list_function_functions: data::Storage::Static(&[]),
                    int_list_function_functions: data::Storage::Static(&[]),
                    string_list_function_functions: data::Storage::Static(&[]),
                    bit_array_list_function_functions: data::Storage::Static(&[]),
                    utf_codepoint_list_function_functions: data::Storage::Static(&[]),
                    custom_list_function_functions: data::Storage::Static(&[]),
                    external_list_function_functions: data::Storage::Static(&[]),
                    float_list_function_functions: data::Storage::Static(&[]),
                    bool_list_function_functions: data::Storage::Static(&[]),
                    nil_list_function_functions: data::Storage::Static(&[]),
                    tuple_list_function_functions: data::Storage::Static(&[]),
                    list_list_function_functions: data::Storage::Static(&[]),
                    function_list_function_functions: data::Storage::Static(&[]),
                    function_function_functions: data::Storage::Static(&[]),
                },
            },
            constants: data::constant::ProfiledConstantTable {
                ints: data::Storage::Static(&[]),
                strings: data::Storage::Static(&[]),
                bit_arrays: data::Storage::Static(&[]),
                customs: data::Storage::Static(&[]),
                floats: data::Storage::Static(&[]),
                bools: data::Storage::Static(&[]),
                nils: data::Storage::Static(&[]),
                tuples: data::Storage::Static(&[]),
                parameter_lists: data::Storage::Static(&[]),
                parameter_list_lists: data::Storage::Static(&[]),
                int_lists: data::Storage::Static(&[]),
                string_lists: data::Storage::Static(&[]),
                bit_array_lists: data::Storage::Static(&[]),
                utf_codepoint_lists: data::Storage::Static(&[]),
                custom_lists: data::Storage::Static(&[]),
                external_lists: data::Storage::Static(&[]),
                float_lists: data::Storage::Static(&[]),
                bool_lists: data::Storage::Static(&[]),
                nil_lists: data::Storage::Static(&[]),
                tuple_lists: data::Storage::Static(&[]),
                list_lists: data::Storage::Static(&[]),
                function_lists: data::Storage::Static(&[]),
                functions: data::Storage::Static(&[]),
            },
            function_parameters: data::function::FunctionCatalog {
                families: [
                    0..0,
                    0..2,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    2..7,
                    0..0,
                    0..0,
                    0..0,
                    7..8,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    8..9,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                ],
                functions: data::Storage::Static(&[
                    data::function::FunctionContract {
                        parameters: 0..1,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(2),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 1..2,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 2..3,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(2),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 3..4,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(2),
                        ]),
                        return_: data::type_::ValueShapeId(2),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 4..5,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(2),
                        ]),
                        return_: data::type_::ValueShapeId(2),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 5..6,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(3),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 6..7,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(3),
                        ]),
                        return_: data::type_::ValueShapeId(3),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 7..7,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(4),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 7..8,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(3),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                ]),
                parameters: data::Storage::Static(&[
                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(0),
                        },
                    }),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(0),
                        },
                    }),
                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(0),
                        },
                    }),
                    data::graph::ParamLocal::IntFunction {
                        local: data::graph::IntFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        },
                    },
                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(1),
                            shape_id: data::type_::CustomValueShapeId(1),
                        },
                    }),
                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(1),
                            shape_id: data::type_::CustomValueShapeId(1),
                        },
                    }),
                ]),
            },
            list_types: data::type_::ListTypeTable {
                types: data::Storage::Static(&[]),
                tuple_items: data::Storage::Static(&[]),
                function_items: data::Storage::Static(&[]),
            },
            custom_types: data::type_::CustomTypeTable {
                types: data::Storage::Static(&[
                    data::type_::CustomTypeDescriptor {
                        type_: data::type_::NominalTypeMetadata {
                            package: data::Text::Static("producer"),
                            module: data::Text::Static("handles"),
                            name: data::Text::Static("Handle"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                        },
                        constructor_count: 1,
                        constructors: data::Storage::Static(&[
                            data::type_::CustomConstructorDescriptor {
                                id: data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                                name: data::Text::Static("Handle"),
                                native_tag: data::Text::Static("handle"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: None,
                                        type_: data::type_::ValueType::Int,
                                        shape: data::type_::ValueShapeId(0),
                                        refinement: data::type_::FieldRefinement::Argument(0),
                                    },
                                ]),
                            },
                        ]),
                    },
                    data::type_::CustomTypeDescriptor {
                        type_: data::type_::NominalTypeMetadata {
                            package: data::Text::Static("producer"),
                            module: data::Text::Static("handles"),
                            name: data::Text::Static("Handle"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                    arguments: data::Storage::Static(&[
                                        data::type_::TypeMetadata::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                                }),
                            ]),
                        },
                        constructor_count: 1,
                        constructors: data::Storage::Static(&[
                            data::type_::CustomConstructorDescriptor {
                                id: data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(1),
                                    index: 0,
                                },
                                name: data::Text::Static("Handle"),
                                native_tag: data::Text::Static("handle"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: None,
                                        type_: data::type_::ValueType::Function(data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        }),
                                        shape: data::type_::ValueShapeId(1),
                                        refinement: data::type_::FieldRefinement::Argument(0),
                                    },
                                ]),
                            },
                        ]),
                    },
                ]),
                definitions: data::Storage::Static(&[
                    data::type_::CustomDefinition {
                        package: data::Text::Static("producer"),
                        module: data::Text::Static("handles"),
                        name: data::Text::Static("Handle"),
                        publicity: data::type_::CustomTypePublicity::Public,
                        opaque: true,
                        parameters: 1,
                        constructors: data::Storage::Static(&[
                            data::type_::ConstructorDefinition {
                                name: data::Text::Static("Handle"),
                                fields: data::Storage::Static(&[
                                    data::type_::FieldDefinition {
                                        label: None,
                                        type_: data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                                    },
                                ]),
                            },
                        ]),
                    },
                ]),
            },
            external_types: data::type_::ExternalTypeTable {
                types: data::Storage::Static(&[]),
            },
            value_shapes: data::type_::ValueShapeTable {
                shapes: data::Storage::Static(&[
                    data::type_::ValueShapeDescriptor::Int,
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                    },
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(0)),
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(1)),
                    data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                    ])),
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(2)),
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(3)),
                ]),
                shape_types: data::Storage::Static(&[
                    data::type_::ValueType::Int,
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    }),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::Int,
                        data::type_::ValueType::Int,
                    ])),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                ]),
                custom_shapes: data::Storage::Static(&[
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(0),
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        constructor: data::type_::CustomConstructorRefinement::Any,
                    },
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(1),
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        constructor: data::type_::CustomConstructorRefinement::Any,
                    },
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(0),
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        constructor: data::type_::CustomConstructorRefinement::Exact(0),
                    },
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(1),
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        constructor: data::type_::CustomConstructorRefinement::Exact(0),
                    },
                ]),
            },
        },
        entries: data::program::LibraryFunctionEntries {
            ints: data::Storage::Static(&[]),
            floats: data::Storage::Static(&[]),
            strings: data::Storage::Static(&[]),
            bit_arrays: data::Storage::Static(&[]),
            utf_codepoints: data::Storage::Static(&[]),
            customs: data::Storage::Static(&[]),
            externals: data::Storage::Static(&[]),
            bools: data::Storage::Static(&[]),
            nils: data::Storage::Static(&[]),
            tuples: data::Storage::Static(&[
                data::program::LibraryFunctionEntry {
                    function: data::function::TupleFunctionId(0),
                    inputs: data::program::LibraryInputConstructions {
                        variants: data::Storage::Static(&[]),
                        lists: data::program::LibraryListConstructions {
                            ints: data::Storage::Static(&[]),
                            floats: data::Storage::Static(&[]),
                            strings: data::Storage::Static(&[]),
                            bit_arrays: data::Storage::Static(&[]),
                            utf_codepoints: data::Storage::Static(&[]),
                            customs: data::Storage::Static(&[]),
                            externals: data::Storage::Static(&[]),
                            bools: data::Storage::Static(&[]),
                            nils: data::Storage::Static(&[]),
                            tuples: data::Storage::Static(&[]),
                            lists: data::Storage::Static(&[]),
                            functions: data::Storage::Static(&[]),
                        },
                    },
                    callables: data::Storage::Static(&[]),
                },
            ]),
            lists: data::Storage::Static(&[]),
            functions: data::Storage::Static(&[]),
        },
        exports: data::Storage::Static(&[
            data::Export {
                name: data::Text::Static("main"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::Int,
                    ]))),
                },
                slot: 0,
            },
        ]),
    },
    value_functions: data::Storage::Static(&[
        data::host::HostedFunctionMetadata {
            callable_entry: None,
            package: data::Text::Static("consumer"),
            site: data::source::HostCallSite::from_static("consumer", "retain", data::source::SourceSpan::new(54, 89)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("producer"),
                        module: data::Text::Static("handles"),
                        name: data::Text::Static("Handle"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                    package: data::Text::Static("producer"),
                    module: data::Text::Static("handles"),
                    name: data::Text::Static("Handle"),
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                    ]),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Int,
                    shape: data::type_::ValueShapeId(0),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Custom(data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(0),
                        },
                    })),
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("producer"),
                            module: data::Text::Static("handles"),
                            name: data::Text::Static("Handle"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                        }), data::type_::CustomTypeId(0)),
                    ]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Custom {
                        schema: data::host::CustomSchema {
                            package: data::Text::Static("producer"),
                            module: data::Text::Static("handles"),
                            name: data::Text::Static("Handle"),
                            parameter_count: 1,
                            constructors: data::Storage::Static(&[
                                data::host::ConstructorSchema {
                                    name: data::Text::Static("Handle"),
                                    fields: data::Storage::Static(&[
                                        data::host::FieldSchema {
                                            label: None,
                                            type_: data::host::SchemaType::Parameter(0),
                                        },
                                    ]),
                                },
                            ]),
                            shared: true,
                        },
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Parameter(0),
                        ]),
                    },
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Custom {
                    schema: data::host::CustomSchema {
                        package: data::Text::Static("producer"),
                        module: data::Text::Static("handles"),
                        name: data::Text::Static("Handle"),
                        parameter_count: 1,
                        constructors: data::Storage::Static(&[
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Handle"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(0),
                                    },
                                ]),
                            },
                        ]),
                        shared: true,
                    },
                    arguments: data::Storage::Static(&[
                        data::host::RegistrationType::Parameter(0),
                    ]),
                },
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Custom(0),
                ]),
                custom_schemas: data::Storage::Static(&[
                    data::host::CustomSchema {
                        package: data::Text::Static("producer"),
                        module: data::Text::Static("handles"),
                        name: data::Text::Static("Handle"),
                        parameter_count: 1,
                        constructors: data::Storage::Static(&[
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Handle"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(0),
                                    },
                                ]),
                            },
                        ]),
                        shared: true,
                    },
                ]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
        data::host::HostedFunctionMetadata {
            callable_entry: None,
            package: data::Text::Static("consumer"),
            site: data::source::HostCallSite::from_static("consumer", "increment", data::source::SourceSpan::new(152, 192)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("producer"),
                        module: data::Text::Static("handles"),
                        name: data::Text::Static("Handle"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                    package: data::Text::Static("producer"),
                    module: data::Text::Static("handles"),
                    name: data::Text::Static("Handle"),
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                    ]),
                })),
            },
            type_arguments: data::Storage::Static(&[]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Custom(data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(0),
                        },
                    })),
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("producer"),
                            module: data::Text::Static("handles"),
                            name: data::Text::Static("Handle"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                        }), data::type_::CustomTypeId(0)),
                    ]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(0))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 0,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Custom {
                        schema: data::host::CustomSchema {
                            package: data::Text::Static("producer"),
                            module: data::Text::Static("handles"),
                            name: data::Text::Static("Handle"),
                            parameter_count: 1,
                            constructors: data::Storage::Static(&[
                                data::host::ConstructorSchema {
                                    name: data::Text::Static("Handle"),
                                    fields: data::Storage::Static(&[
                                        data::host::FieldSchema {
                                            label: None,
                                            type_: data::host::SchemaType::Parameter(0),
                                        },
                                    ]),
                                },
                            ]),
                            shared: true,
                        },
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Int,
                        ]),
                    },
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Custom {
                    schema: data::host::CustomSchema {
                        package: data::Text::Static("producer"),
                        module: data::Text::Static("handles"),
                        name: data::Text::Static("Handle"),
                        parameter_count: 1,
                        constructors: data::Storage::Static(&[
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Handle"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(0),
                                    },
                                ]),
                            },
                        ]),
                        shared: true,
                    },
                    arguments: data::Storage::Static(&[
                        data::host::RegistrationType::Int,
                    ]),
                },
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Custom(0),
                ]),
                custom_schemas: data::Storage::Static(&[
                    data::host::CustomSchema {
                        package: data::Text::Static("producer"),
                        module: data::Text::Static("handles"),
                        name: data::Text::Static("Handle"),
                        parameter_count: 1,
                        constructors: data::Storage::Static(&[
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Handle"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(0),
                                    },
                                ]),
                            },
                        ]),
                        shared: true,
                    },
                ]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
        data::host::HostedFunctionMetadata {
            callable_entry: None,
            package: data::Text::Static("consumer"),
            site: data::source::HostCallSite::from_static("consumer", "retain", data::source::SourceSpan::new(54, 89)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("producer"),
                        module: data::Text::Static("handles"),
                        name: data::Text::Static("Handle"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                            }),
                        ]),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                    package: data::Text::Static("producer"),
                    module: data::Text::Static("handles"),
                    name: data::Text::Static("Handle"),
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                        }),
                    ]),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                    }),
                    shape: data::type_::ValueShapeId(1),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Custom(data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(1),
                            shape_id: data::type_::CustomValueShapeId(1),
                        },
                    })),
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("producer"),
                            module: data::Text::Static("handles"),
                            name: data::Text::Static("Handle"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                    arguments: data::Storage::Static(&[
                                        data::type_::TypeMetadata::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                                }),
                            ]),
                        }), data::type_::CustomTypeId(1)),
                    ]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(1))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Custom {
                        schema: data::host::CustomSchema {
                            package: data::Text::Static("producer"),
                            module: data::Text::Static("handles"),
                            name: data::Text::Static("Handle"),
                            parameter_count: 1,
                            constructors: data::Storage::Static(&[
                                data::host::ConstructorSchema {
                                    name: data::Text::Static("Handle"),
                                    fields: data::Storage::Static(&[
                                        data::host::FieldSchema {
                                            label: None,
                                            type_: data::host::SchemaType::Parameter(0),
                                        },
                                    ]),
                                },
                            ]),
                            shared: true,
                        },
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Parameter(0),
                        ]),
                    },
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Custom {
                    schema: data::host::CustomSchema {
                        package: data::Text::Static("producer"),
                        module: data::Text::Static("handles"),
                        name: data::Text::Static("Handle"),
                        parameter_count: 1,
                        constructors: data::Storage::Static(&[
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Handle"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(0),
                                    },
                                ]),
                            },
                        ]),
                        shared: true,
                    },
                    arguments: data::Storage::Static(&[
                        data::host::RegistrationType::Parameter(0),
                    ]),
                },
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Custom(0),
                ]),
                custom_schemas: data::Storage::Static(&[
                    data::host::CustomSchema {
                        package: data::Text::Static("producer"),
                        module: data::Text::Static("handles"),
                        name: data::Text::Static("Handle"),
                        parameter_count: 1,
                        constructors: data::Storage::Static(&[
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Handle"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(0),
                                    },
                                ]),
                            },
                        ]),
                        shared: true,
                    },
                ]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
    ]),
    never_functions: data::Storage::Static(&[]),
    callables: data::Storage::Static(&[]),
}
