data::HostedModuleArtifact {
    module: data::ModuleArtifact {
        format: 20,
        program: data::ProgramTables {
            root: data::source::module_id(1),
            modules: data::Storage::Static(&[
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("fixture/work"),
                    source_context: Some(data::source::SourceContext::from_static_block("src/fixture/work.gleam", r#"
pub type Work(value)

@external(erlang, "fixture", "ready")
pub fn ready(value: value) -> Work(value)

@external(erlang, "fixture", "map")
pub fn map(value: Work(a), callback: fn(a) -> b) -> Work(b)

@external(erlang, "fixture", "flatten")
pub fn flatten(value: Work(Work(a))) -> Work(a)

@external(erlang, "fixture", "all")
pub fn all(values: List(Work(a))) -> Work(List(a))

pub fn then(value: Work(a), callback: fn(a) -> Work(b)) -> Work(b) {
  flatten(map(value, callback))
}
"#)),
                },
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("app"),
                    source_context: Some(data::source::SourceContext::from_static_block("src/app.gleam", r#"
import fixture/work

fn identity(value) {
  value
}

pub fn make(seed: Int) -> work.Work(Int) {
  let original = work.ready(seed)
  let same = identity(original)
  let assert [same_list] = identity([same])
  let same_closure = identity(fn() { same_list })()
  let assert [same_closure_list] = identity(fn() { [same_closure] })()
  let offset = 2
  use value <- work.map(same_closure_list)
  value + offset
}

pub fn collect(seed: Int) -> work.Work(List(Int)) {
  let shared = make(seed)
  work.all([shared, shared])
}

pub fn keep(value: work.Work(Int)) -> work.Work(Int) {
  value
}

pub fn failure() -> work.Work(Int) {
  use _ <- work.map(work.ready(0))
  panic as "prepared work failed"
}

pub type Captured {
  Captured(callback: fn(Int) -> Int)
}

fn chain(depth: Int, previous: fn(Int) -> Int) -> fn(Int) -> Int {
  case depth {
    0 -> previous
    _ -> chain(depth - 1, fn(value) { previous(value) + 1 })
  }
}

pub fn capture(depth: Int) -> Captured {
  Captured(chain(depth, fn(value) { value }))
}

pub fn extend(value: Captured, depth: Int) -> Captured {
  Captured(chain(depth, value.callback))
}

pub fn identities(value: Captured) -> #(Bool, Bool) {
  let callback = value.callback
  #(callback == value.callback, callback == fn(input) { callback(input) })
}

pub fn invoke(value: Captured) -> work.Work(Int) {
  use initial <- work.map(work.ready(1))
  value.callback(initial)
}
"#)),
                },
            ]),
            main: data::function::ProfiledRuntimeFunctionId::External(data::function::ExternalFunctionId {
                index: 0,
                return_type: data::type_::ExternalTypeId(0),
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
                                            params: 0..2,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(2)),
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
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::SourceStop(data::graph::SourceStop {
                                                kind: data::graph::SourceStopKind::Panic,
                                                message: Some(data::graph::StringLocalId(0)),
                                                site: data::source::PanicSite::from_static("app", "<anonymous:3>", data::source::SourceSpan::new(659, 690)),
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                shape: data::type_::ValueShapeId(10),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("prepared work failed"))),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[]),
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
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
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
                                            params: 0..2,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
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
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::FunctionCall {
                                                function: data::graph::IntFunctionLocalId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("app", "<anonymous:6>", data::source::SourceSpan::new(1255, 1270)),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
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
                                            params: 0..2,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(7),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                                shape: data::type_::ValueShapeId(5),
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
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(0),
                                                        },
                                                    },
                                                    index: 0,
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::FunctionCall {
                                                function: data::graph::IntFunctionLocalId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("app", "<anonymous:7>", data::source::SourceSpan::new(1371, 1394)),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
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
                                            params: 0..2,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
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
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::FunctionCall {
                                                function: data::graph::IntFunctionLocalId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("app", "<anonymous:4>", data::source::SourceSpan::new(892, 907)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
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
                                    shape_id: data::type_::CustomValueShapeId(1),
                                },
                                body: data::function::ProfiledFunctionBody {
                                    block_graph: data::graph::ProfiledBlockGraph {
                                        entry: data::graph::BlockId(0),
                                        blocks: data::Storage::Static(&[
                                            data::graph::BlockHeader {
                                                params: 0..1,
                                                instructions: 0..3,
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
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                                    shape: data::type_::ValueShapeId(5),
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
                                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(2)),
                                                        captures: data::Storage::Static(&[]),
                                                    },
                                                }),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                                    shape: data::type_::ValueShapeId(5),
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
                                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                                        site: data::source::HostCallSite::from_static("app", "capture", data::source::SourceSpan::new(974, 1007)),
                                                    },
                                                }),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(1),
                                                        },
                                                    }),
                                                    shape: data::type_::ValueShapeId(6),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                    constructor: data::type_::CustomConstructorId {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        index: 0,
                                                    },
                                                    fields: data::Storage::Static(&[
                                                        data::graph::ParamLocal::IntFunction {
                                                            local: data::graph::IntFunctionLocalId(1),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                            },
                                                        },
                                                    ]),
                                                }),
                                            }),
                                        ]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::Return(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(1),
                                            },
                                        }),
                                    ]),
                                },
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledCustomFunctionBody {
                                _signature_shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(0),
                                    shape_id: data::type_::CustomValueShapeId(0),
                                },
                                _body_shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(0),
                                    shape_id: data::type_::CustomValueShapeId(1),
                                },
                                body: data::function::ProfiledFunctionBody {
                                    block_graph: data::graph::ProfiledBlockGraph {
                                        entry: data::graph::BlockId(0),
                                        blocks: data::Storage::Static(&[
                                            data::graph::BlockHeader {
                                                params: 0..2,
                                                instructions: 0..3,
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
                                                shape: data::type_::ValueShapeId(7),
                                            },
                                            data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                        ]),
                                        instructions: data::Storage::Static(&[
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                                    shape: data::type_::ValueShapeId(5),
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
                                                                type_id: data::type_::CustomTypeId(0),
                                                                shape_id: data::type_::CustomValueShapeId(0),
                                                            },
                                                        },
                                                        index: 0,
                                                    },
                                                }),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                                    shape: data::type_::ValueShapeId(5),
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
                                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                                        site: data::source::HostCallSite::from_static("app", "extend", data::source::SourceSpan::new(1080, 1108)),
                                                    },
                                                }),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(1),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(1),
                                                        },
                                                    }),
                                                    shape: data::type_::ValueShapeId(6),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                    constructor: data::type_::CustomConstructorId {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        index: 0,
                                                    },
                                                    fields: data::Storage::Static(&[
                                                        data::graph::ParamLocal::IntFunction {
                                                            local: data::graph::IntFunctionLocalId(1),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                            },
                                                        },
                                                    ]),
                                                }),
                                            }),
                                        ]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::Return(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(1),
                                            },
                                        }),
                                    ]),
                                },
                            },
                        })),
                    ]),
                    external_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 1,
                            },
                            body: data::function::ProfiledExternalFunctionBody {
                                _signature_type: data::type_::ExternalTypeId(0),
                                _body_type: data::type_::ExternalTypeId(0),
                                body: data::function::ProfiledFunctionBody {
                                    block_graph: data::graph::ProfiledBlockGraph {
                                        entry: data::graph::BlockId(0),
                                        blocks: data::Storage::Static(&[
                                            data::graph::BlockHeader {
                                                params: 0..1,
                                                instructions: 0..4,
                                                terminator: data::graph::Terminator::Match(data::graph::Match {
                                                    subject: data::graph::ParamLocal::List(data::graph::ListLocal::External {
                                                        local: data::graph::ExternalListLocalId(1),
                                                        type_id: data::type_::ExternalListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::ExternalTypeId(0),
                                                        },
                                                    }),
                                                    pattern: data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                        elements: data::Storage::Static(&[
                                                            data::graph::MatchPattern::Bind(data::graph::MatchPatternBinding {
                                                                index: 0,
                                                            }),
                                                        ]),
                                                        tail: None,
                                                    }),
                                                    success: data::graph::MatchEdge {
                                                        target: data::graph::BlockId(1),
                                                        args: data::Storage::Static(&[
                                                            data::graph::MatchEdgeArgument::Binding(0),
                                                        ]),
                                                        bindings: data::Storage::Static(&[
                                                            0,
                                                        ]),
                                                        transfer: data::graph::Transfer {
                                                            families: data::Storage::Static(&[
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::Int,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::External,
                                                                    length: 1,
                                                                    steps: data::Storage::Static(&[
                                                                        data::graph::TransferStep {
                                                                            source: 2,
                                                                            destination: 0,
                                                                        },
                                                                    ]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::ExternalList,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                            ]),
                                                        },
                                                    },
                                                    failure: data::graph::Edge {
                                                        target: data::graph::BlockId(4),
                                                        args: data::Storage::Static(&[
                                                            data::graph::ParamLocal::List(data::graph::ListLocal::External {
                                                                local: data::graph::ExternalListLocalId(1),
                                                                type_id: data::type_::ExternalListTypeId {
                                                                    list_type: data::type_::ListTypeId(0),
                                                                    item_type: data::type_::ExternalTypeId(0),
                                                                },
                                                            }),
                                                        ]),
                                                        transfer: data::graph::Transfer {
                                                            families: data::Storage::Static(&[
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::Int,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::External,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::ExternalList,
                                                                    length: 1,
                                                                    steps: data::Storage::Static(&[
                                                                        data::graph::TransferStep {
                                                                            source: 1,
                                                                            destination: 0,
                                                                        },
                                                                    ]),
                                                                },
                                                            ]),
                                                        },
                                                    },
                                                }),
                                            },
                                            data::graph::BlockHeader {
                                                params: 1..2,
                                                instructions: 4..10,
                                                terminator: data::graph::Terminator::Match(data::graph::Match {
                                                    subject: data::graph::ParamLocal::List(data::graph::ListLocal::External {
                                                        local: data::graph::ExternalListLocalId(0),
                                                        type_id: data::type_::ExternalListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::ExternalTypeId(0),
                                                        },
                                                    }),
                                                    pattern: data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                        elements: data::Storage::Static(&[
                                                            data::graph::MatchPattern::Bind(data::graph::MatchPatternBinding {
                                                                index: 0,
                                                            }),
                                                        ]),
                                                        tail: None,
                                                    }),
                                                    success: data::graph::MatchEdge {
                                                        target: data::graph::BlockId(2),
                                                        args: data::Storage::Static(&[
                                                            data::graph::MatchEdgeArgument::Binding(0),
                                                        ]),
                                                        bindings: data::Storage::Static(&[
                                                            0,
                                                        ]),
                                                        transfer: data::graph::Transfer {
                                                            families: data::Storage::Static(&[
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::External,
                                                                    length: 1,
                                                                    steps: data::Storage::Static(&[
                                                                        data::graph::TransferStep {
                                                                            source: 2,
                                                                            destination: 0,
                                                                        },
                                                                    ]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::ExternalList,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::ExternalFunction,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::ExternalListFunction,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                            ]),
                                                        },
                                                    },
                                                    failure: data::graph::Edge {
                                                        target: data::graph::BlockId(3),
                                                        args: data::Storage::Static(&[
                                                            data::graph::ParamLocal::List(data::graph::ListLocal::External {
                                                                local: data::graph::ExternalListLocalId(0),
                                                                type_id: data::type_::ExternalListTypeId {
                                                                    list_type: data::type_::ListTypeId(0),
                                                                    item_type: data::type_::ExternalTypeId(0),
                                                                },
                                                            }),
                                                        ]),
                                                        transfer: data::graph::Transfer {
                                                            families: data::Storage::Static(&[
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::External,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::ExternalFunction,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::ExternalListFunction,
                                                                    length: 0,
                                                                    steps: data::Storage::Static(&[]),
                                                                },
                                                            ]),
                                                        },
                                                    },
                                                }),
                                            },
                                            data::graph::BlockHeader {
                                                params: 2..3,
                                                instructions: 10..12,
                                                terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                            },
                                            data::graph::BlockHeader {
                                                params: 3..4,
                                                instructions: 12..12,
                                                terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                    subject: data::graph::ParamLocal::List(data::graph::ListLocal::External {
                                                        local: data::graph::ExternalListLocalId(0),
                                                        type_id: data::type_::ExternalListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::ExternalTypeId(0),
                                                        },
                                                    }),
                                                    message: None,
                                                    site: data::source::PanicSite::from_static("app", "make", data::source::SourceSpan::new(260, 270)),
                                                    pattern_span: data::source::SourceSpan::new(271, 290),
                                                }),
                                            },
                                            data::graph::BlockHeader {
                                                params: 4..5,
                                                instructions: 12..12,
                                                terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                    subject: data::graph::ParamLocal::List(data::graph::ListLocal::External {
                                                        local: data::graph::ExternalListLocalId(0),
                                                        type_id: data::type_::ExternalListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::ExternalTypeId(0),
                                                        },
                                                    }),
                                                    message: None,
                                                    site: data::source::PanicSite::from_static("app", "make", data::source::SourceSpan::new(164, 174)),
                                                    pattern_span: data::source::SourceSpan::new(175, 186),
                                                }),
                                            },
                                        ]),
                                        params: data::Storage::Static(&[
                                            data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                    id: data::graph::ExternalLocalId(0),
                                                    type_id: data::type_::ExternalTypeId(0),
                                                }),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                    id: data::graph::ExternalLocalId(0),
                                                    type_id: data::type_::ExternalTypeId(0),
                                                }),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::External {
                                                    local: data::graph::ExternalListLocalId(0),
                                                    type_id: data::type_::ExternalListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item_type: data::type_::ExternalTypeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::External {
                                                    local: data::graph::ExternalListLocalId(0),
                                                    type_id: data::type_::ExternalListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item_type: data::type_::ExternalTypeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                        ]),
                                        instructions: data::Storage::Static(&[
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                        id: data::graph::ExternalLocalId(0),
                                                        type_id: data::type_::ExternalTypeId(0),
                                                    }),
                                                    shape: data::type_::ValueShapeId(1),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::External(data::graph::ExternalInstruction::Call {
                                                    function: data::function::ExternalFunctionId {
                                                        index: 5,
                                                        return_type: data::type_::ExternalTypeId(0),
                                                    },
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("app", "make", data::source::SourceSpan::new(113, 129)),
                                                }),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                        id: data::graph::ExternalLocalId(1),
                                                        type_id: data::type_::ExternalTypeId(0),
                                                    }),
                                                    shape: data::type_::ValueShapeId(1),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::External(data::graph::ExternalInstruction::Call {
                                                    function: data::function::ExternalFunctionId {
                                                        index: 6,
                                                        return_type: data::type_::ExternalTypeId(0),
                                                    },
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                            id: data::graph::ExternalLocalId(0),
                                                            type_id: data::type_::ExternalTypeId(0),
                                                        }),
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("app", "make", data::source::SourceSpan::new(143, 161)),
                                                }),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::List(data::graph::ListLocal::External {
                                                        local: data::graph::ExternalListLocalId(0),
                                                        type_id: data::type_::ExternalListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::ExternalTypeId(0),
                                                        },
                                                    }),
                                                    shape: data::type_::ValueShapeId(2),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::ExternalList(data::graph::ExternalListInstruction {
                                                    type_id: data::type_::ExternalListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item_type: data::type_::ExternalTypeId(0),
                                                    },
                                                    instruction: data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                                        data::graph::ExternalLocal {
                                                            id: data::graph::ExternalLocalId(1),
                                                            type_id: data::type_::ExternalTypeId(0),
                                                        },
                                                    ])),
                                                }),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::List(data::graph::ListLocal::External {
                                                        local: data::graph::ExternalListLocalId(1),
                                                        type_id: data::type_::ExternalListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::ExternalTypeId(0),
                                                        },
                                                    }),
                                                    shape: data::type_::ValueShapeId(2),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::ExternalList(data::graph::ExternalListInstruction {
                                                    type_id: data::type_::ExternalListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item_type: data::type_::ExternalTypeId(0),
                                                    },
                                                    instruction: data::graph::TypedListInstruction::Call {
                                                        function: data::function::ExternalListFunctionId {
                                                            index: 0,
                                                            type_id: data::type_::ExternalListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                                item_type: data::type_::ExternalTypeId(0),
                                                            },
                                                        },
                                                        args: data::Storage::Static(&[
                                                            data::graph::ParamLocal::List(data::graph::ListLocal::External {
                                                                local: data::graph::ExternalListLocalId(0),
                                                                type_id: data::type_::ExternalListTypeId {
                                                                    list_type: data::type_::ListTypeId(0),
                                                                    item_type: data::type_::ExternalTypeId(0),
                                                                },
                                                            }),
                                                        ]),
                                                        site: data::source::HostCallSite::from_static("app", "make", data::source::SourceSpan::new(189, 205)),
                                                    },
                                                }),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                        id: data::graph::ExternalFunctionLocalId(0),
                                                        type_: data::type_::ExternalFunctionType {
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                            },
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::type_::ExternalTypeId(0),
                                                        },
                                                    }),
                                                    shape: data::type_::ValueShapeId(3),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::ExternalFunction(data::graph::ExternalFunctionInstruction {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                    },
                                                    family: data::function::FunctionReturnFamily::External,
                                                    kind: data::graph::ExternalFunctionInstructionKind::Closure {
                                                        target: data::graph::ExternalFunctionTarget::Value(data::function::ExternalFunctionId {
                                                            index: 7,
                                                            return_type: data::type_::ExternalTypeId(0),
                                                        }),
                                                        captures: data::Storage::Static(&[
                                                            data::graph::FunctionCapture::External {
                                                                target: data::graph::ExternalLocal {
                                                                    id: data::graph::ExternalLocalId(0),
                                                                    type_id: data::type_::ExternalTypeId(0),
                                                                },
                                                                source: data::graph::ExternalLocal {
                                                                    id: data::graph::ExternalLocalId(0),
                                                                    type_id: data::type_::ExternalTypeId(0),
                                                                },
                                                            },
                                                        ]),
                                                    },
                                                }),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                        id: data::graph::ExternalFunctionLocalId(1),
                                                        type_: data::type_::ExternalFunctionType {
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                            },
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::type_::ExternalTypeId(0),
                                                        },
                                                    }),
                                                    shape: data::type_::ValueShapeId(3),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::ExternalFunction(data::graph::ExternalFunctionInstruction {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                    },
                                                    family: data::function::FunctionReturnFamily::External,
                                                    kind: data::graph::ExternalFunctionInstructionKind::Call {
                                                        function: data::graph::ExternalFunctionCallTarget::Function(data::function::ExternalFunctionFunctionId {
                                                            index: 0,
                                                            type_: data::type_::ExternalFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                },
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::type_::ExternalTypeId(0),
                                                            },
                                                        }),
                                                        args: data::Storage::Static(&[
                                                            data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                                id: data::graph::ExternalFunctionLocalId(0),
                                                                type_: data::type_::ExternalFunctionType {
                                                                    type_: data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                    },
                                                                    arguments: data::Storage::Static(&[]),
                                                                    return_: data::type_::ExternalTypeId(0),
                                                                },
                                                            }),
                                                        ]),
                                                        site: data::source::HostCallSite::from_static("app", "make", data::source::SourceSpan::new(227, 255)),
                                                    },
                                                }),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                        id: data::graph::ExternalLocalId(1),
                                                        type_id: data::type_::ExternalTypeId(0),
                                                    }),
                                                    shape: data::type_::ValueShapeId(1),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::External(data::graph::ExternalInstruction::FunctionCall {
                                                    function: data::graph::ExternalFunctionLocal {
                                                        id: data::graph::ExternalFunctionLocalId(1),
                                                        type_: data::type_::ExternalFunctionType {
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                            },
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::type_::ExternalTypeId(0),
                                                        },
                                                    },
                                                    args: data::Storage::Static(&[]),
                                                    site: data::source::HostCallSite::from_static("app", "make", data::source::SourceSpan::new(227, 257)),
                                                }),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::External {
                                                        local: data::graph::ExternalListFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                        },
                                                        list_type: data::type_::ExternalListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::ExternalTypeId(0),
                                                        },
                                                    }),
                                                    shape: data::type_::ValueShapeId(4),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::ExternalFunction(data::graph::ExternalFunctionInstruction {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                    },
                                                    family: data::function::FunctionReturnFamily::List,
                                                    kind: data::graph::ExternalFunctionInstructionKind::Closure {
                                                        target: data::graph::ExternalFunctionTarget::List(data::function::ExternalListFunctionId {
                                                            index: 1,
                                                            type_id: data::type_::ExternalListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                                item_type: data::type_::ExternalTypeId(0),
                                                            },
                                                        }),
                                                        captures: data::Storage::Static(&[
                                                            data::graph::FunctionCapture::External {
                                                                target: data::graph::ExternalLocal {
                                                                    id: data::graph::ExternalLocalId(0),
                                                                    type_id: data::type_::ExternalTypeId(0),
                                                                },
                                                                source: data::graph::ExternalLocal {
                                                                    id: data::graph::ExternalLocalId(1),
                                                                    type_id: data::type_::ExternalTypeId(0),
                                                                },
                                                            },
                                                        ]),
                                                    },
                                                }),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::External {
                                                        local: data::graph::ExternalListFunctionLocalId(1),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                        },
                                                        list_type: data::type_::ExternalListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::ExternalTypeId(0),
                                                        },
                                                    }),
                                                    shape: data::type_::ValueShapeId(4),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::ExternalFunction(data::graph::ExternalFunctionInstruction {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                    },
                                                    family: data::function::FunctionReturnFamily::List,
                                                    kind: data::graph::ExternalFunctionInstructionKind::Call {
                                                        function: data::graph::ExternalFunctionCallTarget::ListFunction {
                                                            id: data::function::ExternalListFunctionFunctionId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                            },
                                                            list_type: data::type_::ExternalListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                                item_type: data::type_::ExternalTypeId(0),
                                                            },
                                                        },
                                                        args: data::Storage::Static(&[
                                                            data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::External {
                                                                local: data::graph::ExternalListFunctionLocalId(0),
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                                },
                                                                list_type: data::type_::ExternalListTypeId {
                                                                    list_type: data::type_::ListTypeId(0),
                                                                    item_type: data::type_::ExternalTypeId(0),
                                                                },
                                                            }),
                                                        ]),
                                                        site: data::source::HostCallSite::from_static("app", "make", data::source::SourceSpan::new(293, 326)),
                                                    },
                                                }),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::List(data::graph::ListLocal::External {
                                                        local: data::graph::ExternalListLocalId(0),
                                                        type_id: data::type_::ExternalListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::ExternalTypeId(0),
                                                        },
                                                    }),
                                                    shape: data::type_::ValueShapeId(2),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::ExternalList(data::graph::ExternalListInstruction {
                                                    type_id: data::type_::ExternalListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item_type: data::type_::ExternalTypeId(0),
                                                    },
                                                    instruction: data::graph::TypedListInstruction::FunctionCall {
                                                        function: data::graph::ExternalListFunctionLocalId(1),
                                                        args: data::Storage::Static(&[]),
                                                        site: data::source::HostCallSite::from_static("app", "make", data::source::SourceSpan::new(293, 328)),
                                                    },
                                                }),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    shape: data::type_::ValueShapeId(0),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                    sign: data::Sign::Plus,
                                                    digits: data::Storage::Static(&[
                                                        2,
                                                    ]),
                                                })),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                                    shape: data::type_::ValueShapeId(5),
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
                                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(0)),
                                                        captures: data::Storage::Static(&[
                                                            data::graph::FunctionCapture::Int {
                                                                target: data::graph::IntLocalId(1),
                                                                source: data::graph::IntLocalId(0),
                                                            },
                                                        ]),
                                                    },
                                                }),
                                            }),
                                        ]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::TailCall {
                                            function: data::source::FunctionCallTarget {
                                                function: 8,
                                                site: data::source::HostCallSite::from_static("app", "make", data::source::SourceSpan::new(348, 405)),
                                            },
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                    id: data::graph::ExternalLocalId(0),
                                                    type_id: data::type_::ExternalTypeId(0),
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
                                            ]),
                                            transfer: data::graph::Transfer {
                                                families: data::Storage::Static(&[
                                                    data::graph::FamilyTransfer {
                                                        family: data::graph::StorageFamily::Int,
                                                        length: 0,
                                                        steps: data::Storage::Static(&[]),
                                                    },
                                                ]),
                                            },
                                        },
                                    ]),
                                },
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 1,
                            },
                            body: data::function::ProfiledExternalFunctionBody {
                                _signature_type: data::type_::ExternalTypeId(1),
                                _body_type: data::type_::ExternalTypeId(1),
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
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                        id: data::graph::ExternalLocalId(0),
                                                        type_id: data::type_::ExternalTypeId(0),
                                                    }),
                                                    shape: data::type_::ValueShapeId(1),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::External(data::graph::ExternalInstruction::Call {
                                                    function: data::function::ExternalFunctionId {
                                                        index: 0,
                                                        return_type: data::type_::ExternalTypeId(0),
                                                    },
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("app", "collect", data::source::SourceSpan::new(476, 486)),
                                                }),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::List(data::graph::ListLocal::External {
                                                        local: data::graph::ExternalListLocalId(0),
                                                        type_id: data::type_::ExternalListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::ExternalTypeId(0),
                                                        },
                                                    }),
                                                    shape: data::type_::ValueShapeId(2),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::ExternalList(data::graph::ExternalListInstruction {
                                                    type_id: data::type_::ExternalListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item_type: data::type_::ExternalTypeId(0),
                                                    },
                                                    instruction: data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                                        data::graph::ExternalLocal {
                                                            id: data::graph::ExternalLocalId(0),
                                                            type_id: data::type_::ExternalTypeId(0),
                                                        },
                                                        data::graph::ExternalLocal {
                                                            id: data::graph::ExternalLocalId(0),
                                                            type_id: data::type_::ExternalTypeId(0),
                                                        },
                                                    ])),
                                                }),
                                            }),
                                        ]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::TailCall {
                                            function: data::source::FunctionCallTarget {
                                                function: 9,
                                                site: data::source::HostCallSite::from_static("app", "collect", data::source::SourceSpan::new(489, 515)),
                                            },
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::List(data::graph::ListLocal::External {
                                                    local: data::graph::ExternalListLocalId(0),
                                                    type_id: data::type_::ExternalListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item_type: data::type_::ExternalTypeId(0),
                                                    },
                                                }),
                                            ]),
                                            transfer: data::graph::Transfer {
                                                families: data::Storage::Static(&[
                                                    data::graph::FamilyTransfer {
                                                        family: data::graph::StorageFamily::Int,
                                                        length: 0,
                                                        steps: data::Storage::Static(&[]),
                                                    },
                                                    data::graph::FamilyTransfer {
                                                        family: data::graph::StorageFamily::External,
                                                        length: 0,
                                                        steps: data::Storage::Static(&[]),
                                                    },
                                                ]),
                                            },
                                        },
                                    ]),
                                },
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 1,
                            },
                            body: data::function::ProfiledExternalFunctionBody {
                                _signature_type: data::type_::ExternalTypeId(0),
                                _body_type: data::type_::ExternalTypeId(0),
                                body: data::function::ProfiledFunctionBody {
                                    block_graph: data::graph::ProfiledBlockGraph {
                                        entry: data::graph::BlockId(0),
                                        blocks: data::Storage::Static(&[
                                            data::graph::BlockHeader {
                                                params: 0..1,
                                                instructions: 0..0,
                                                terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                            },
                                        ]),
                                        params: data::Storage::Static(&[
                                            data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                    id: data::graph::ExternalLocalId(0),
                                                    type_id: data::type_::ExternalTypeId(0),
                                                }),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                        ]),
                                        instructions: data::Storage::Static(&[]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::Return(data::graph::ExternalLocal {
                                            id: data::graph::ExternalLocalId(0),
                                            type_id: data::type_::ExternalTypeId(0),
                                        }),
                                    ]),
                                },
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledExternalFunctionBody {
                                _signature_type: data::type_::ExternalTypeId(0),
                                _body_type: data::type_::ExternalTypeId(0),
                                body: data::function::ProfiledFunctionBody {
                                    block_graph: data::graph::ProfiledBlockGraph {
                                        entry: data::graph::BlockId(0),
                                        blocks: data::Storage::Static(&[
                                            data::graph::BlockHeader {
                                                params: 0..0,
                                                instructions: 0..3,
                                                terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                            },
                                        ]),
                                        params: data::Storage::Static(&[]),
                                        instructions: data::Storage::Static(&[
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    shape: data::type_::ValueShapeId(0),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                    sign: data::Sign::NoSign,
                                                    digits: data::Storage::Static(&[]),
                                                })),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                        id: data::graph::ExternalLocalId(0),
                                                        type_id: data::type_::ExternalTypeId(0),
                                                    }),
                                                    shape: data::type_::ValueShapeId(1),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::External(data::graph::ExternalInstruction::Call {
                                                    function: data::function::ExternalFunctionId {
                                                        index: 5,
                                                        return_type: data::type_::ExternalTypeId(0),
                                                    },
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("app", "failure", data::source::SourceSpan::new(642, 655)),
                                                }),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                                    shape: data::type_::ValueShapeId(5),
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
                                            }),
                                        ]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::TailCall {
                                            function: data::source::FunctionCallTarget {
                                                function: 8,
                                                site: data::source::HostCallSite::from_static("app", "failure", data::source::SourceSpan::new(624, 690)),
                                            },
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                    id: data::graph::ExternalLocalId(0),
                                                    type_id: data::type_::ExternalTypeId(0),
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
                                            ]),
                                            transfer: data::graph::Transfer {
                                                families: data::Storage::Static(&[
                                                    data::graph::FamilyTransfer {
                                                        family: data::graph::StorageFamily::Int,
                                                        length: 0,
                                                        steps: data::Storage::Static(&[]),
                                                    },
                                                ]),
                                            },
                                        },
                                    ]),
                                },
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 1,
                            },
                            body: data::function::ProfiledExternalFunctionBody {
                                _signature_type: data::type_::ExternalTypeId(0),
                                _body_type: data::type_::ExternalTypeId(0),
                                body: data::function::ProfiledFunctionBody {
                                    block_graph: data::graph::ProfiledBlockGraph {
                                        entry: data::graph::BlockId(0),
                                        blocks: data::Storage::Static(&[
                                            data::graph::BlockHeader {
                                                params: 0..1,
                                                instructions: 0..3,
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
                                                shape: data::type_::ValueShapeId(7),
                                            },
                                        ]),
                                        instructions: data::Storage::Static(&[
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    shape: data::type_::ValueShapeId(0),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                    sign: data::Sign::Plus,
                                                    digits: data::Storage::Static(&[
                                                        1,
                                                    ]),
                                                })),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                        id: data::graph::ExternalLocalId(0),
                                                        type_id: data::type_::ExternalTypeId(0),
                                                    }),
                                                    shape: data::type_::ValueShapeId(1),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::External(data::graph::ExternalInstruction::Call {
                                                    function: data::function::ExternalFunctionId {
                                                        index: 5,
                                                        return_type: data::type_::ExternalTypeId(0),
                                                    },
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("app", "invoke", data::source::SourceSpan::new(1354, 1367)),
                                                }),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                                    shape: data::type_::ValueShapeId(5),
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
                                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(4)),
                                                        captures: data::Storage::Static(&[
                                                            data::graph::FunctionCapture::Custom {
                                                                target: data::graph::CustomLocal {
                                                                    id: data::graph::CustomLocalId(0),
                                                                    shape: data::type_::CustomValueShape {
                                                                        type_id: data::type_::CustomTypeId(0),
                                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                                    },
                                                                },
                                                                source: data::graph::CustomLocal {
                                                                    id: data::graph::CustomLocalId(0),
                                                                    shape: data::type_::CustomValueShape {
                                                                        type_id: data::type_::CustomTypeId(0),
                                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                                    },
                                                                },
                                                            },
                                                        ]),
                                                    },
                                                }),
                                            }),
                                        ]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::TailCall {
                                            function: data::source::FunctionCallTarget {
                                                function: 8,
                                                site: data::source::HostCallSite::from_static("app", "invoke", data::source::SourceSpan::new(1330, 1394)),
                                            },
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                    id: data::graph::ExternalLocalId(0),
                                                    type_id: data::type_::ExternalTypeId(0),
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
                                            ]),
                                            transfer: data::graph::Transfer {
                                                families: data::Storage::Static(&[
                                                    data::graph::FamilyTransfer {
                                                        family: data::graph::StorageFamily::Int,
                                                        length: 0,
                                                        steps: data::Storage::Static(&[]),
                                                    },
                                                    data::graph::FamilyTransfer {
                                                        family: data::graph::StorageFamily::Custom,
                                                        length: 0,
                                                        steps: data::Storage::Static(&[]),
                                                    },
                                                ]),
                                            },
                                        },
                                    ]),
                                },
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 0,
                            return_: data::graph::ExternalLocal {
                                id: data::graph::ExternalLocalId(0),
                                type_id: data::type_::ExternalTypeId(0),
                            },
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 1,
                            },
                            body: data::function::ProfiledExternalFunctionBody {
                                _signature_type: data::type_::ExternalTypeId(0),
                                _body_type: data::type_::ExternalTypeId(0),
                                body: data::function::ProfiledFunctionBody {
                                    block_graph: data::graph::ProfiledBlockGraph {
                                        entry: data::graph::BlockId(0),
                                        blocks: data::Storage::Static(&[
                                            data::graph::BlockHeader {
                                                params: 0..1,
                                                instructions: 0..0,
                                                terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                            },
                                        ]),
                                        params: data::Storage::Static(&[
                                            data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                    id: data::graph::ExternalLocalId(0),
                                                    type_id: data::type_::ExternalTypeId(0),
                                                }),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                        ]),
                                        instructions: data::Storage::Static(&[]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::Return(data::graph::ExternalLocal {
                                            id: data::graph::ExternalLocalId(0),
                                            type_id: data::type_::ExternalTypeId(0),
                                        }),
                                    ]),
                                },
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledExternalFunctionBody {
                                _signature_type: data::type_::ExternalTypeId(0),
                                _body_type: data::type_::ExternalTypeId(0),
                                body: data::function::ProfiledFunctionBody {
                                    block_graph: data::graph::ProfiledBlockGraph {
                                        entry: data::graph::BlockId(0),
                                        blocks: data::Storage::Static(&[
                                            data::graph::BlockHeader {
                                                params: 0..1,
                                                instructions: 0..0,
                                                terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                            },
                                        ]),
                                        params: data::Storage::Static(&[
                                            data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                    id: data::graph::ExternalLocalId(0),
                                                    type_id: data::type_::ExternalTypeId(0),
                                                }),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                        ]),
                                        instructions: data::Storage::Static(&[]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::Return(data::graph::ExternalLocal {
                                            id: data::graph::ExternalLocalId(0),
                                            type_id: data::type_::ExternalTypeId(0),
                                        }),
                                    ]),
                                },
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 1,
                            return_: data::graph::ExternalLocal {
                                id: data::graph::ExternalLocalId(0),
                                type_id: data::type_::ExternalTypeId(0),
                            },
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 2,
                            return_: data::graph::ExternalLocal {
                                id: data::graph::ExternalLocalId(0),
                                type_id: data::type_::ExternalTypeId(1),
                            },
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    bool_functions: data::Storage::Static(&[]),
                    nil_functions: data::Storage::Static(&[]),
                    tuple_functions: data::Storage::Static(&[
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
                                            instructions: 0..6,
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
                                            shape: data::type_::ValueShapeId(7),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                                shape: data::type_::ValueShapeId(5),
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
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(0),
                                                        },
                                                    },
                                                    index: 0,
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                                shape: data::type_::ValueShapeId(5),
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
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(0),
                                                        },
                                                    },
                                                    index: 0,
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(8),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Equal {
                                                left: data::graph::ParamLocal::IntFunction {
                                                    local: data::graph::IntFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                right: data::graph::ParamLocal::IntFunction {
                                                    local: data::graph::IntFunctionLocalId(1),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::IntFunction {
                                                    local: data::graph::IntFunctionLocalId(2),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(5),
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
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(3)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::IntFunction {
                                                            target: data::graph::IntFunctionLocalId(0),
                                                            source: data::graph::IntFunctionLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                shape: data::type_::ValueShapeId(8),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Equal {
                                                left: data::graph::ParamLocal::IntFunction {
                                                    local: data::graph::IntFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                right: data::graph::ParamLocal::IntFunction {
                                                    local: data::graph::IntFunctionLocalId(2),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::Bool,
                                                    ]),
                                                },
                                                shape: data::type_::ValueShapeId(9),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            ]))),
                                        }),
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
                    external_list_functions: data::Storage::Static(&[
                        (data::function::ExternalListFunctionId {
                            index: 0,
                            type_id: data::type_::ExternalListTypeId {
                                list_type: data::type_::ListTypeId(0),
                                item_type: data::type_::ExternalTypeId(0),
                            },
                        }, data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 1,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::External {
                                                local: data::graph::ExternalListLocalId(0),
                                                type_id: data::type_::ExternalListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::ExternalTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::ExternalListLocalId(0)),
                                ]),
                            },
                        }))),
                        (data::function::ExternalListFunctionId {
                            index: 1,
                            type_id: data::type_::ExternalListTypeId {
                                list_type: data::type_::ListTypeId(0),
                                item_type: data::type_::ExternalTypeId(0),
                            },
                        }, data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
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
                                            local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                id: data::graph::ExternalLocalId(0),
                                                type_id: data::type_::ExternalTypeId(0),
                                            }),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::External {
                                                    local: data::graph::ExternalListLocalId(0),
                                                    type_id: data::type_::ExternalListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item_type: data::type_::ExternalTypeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::ExternalList(data::graph::ExternalListInstruction {
                                                type_id: data::type_::ExternalListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::ExternalTypeId(0),
                                                },
                                                instruction: data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                                    data::graph::ExternalLocal {
                                                        id: data::graph::ExternalLocalId(0),
                                                        type_id: data::type_::ExternalTypeId(0),
                                                    },
                                                ])),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::ExternalListLocalId(0)),
                                ]),
                            },
                        }))),
                    ]),
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
                                parameter_count: 2,
                            },
                            body: data::function::TypedFunctionBody {
                                _shape: data::type_::FunctionShape {
                                    shape_id: data::type_::ValueShapeId(5),
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
                                                params: 0..2,
                                                instructions: 0..0,
                                                terminator: data::graph::Terminator::IntSwitch(data::graph::IntSwitch {
                                                    subject: data::graph::IntLocalId(0),
                                                    clauses: data::Storage::Static(&[
                                                        (data::graph::IntegerLiteral {
                                                            sign: data::Sign::NoSign,
                                                            digits: data::Storage::Static(&[]),
                                                        }, data::graph::Edge {
                                                            target: data::graph::BlockId(1),
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
                                                            transfer: data::graph::Transfer {
                                                                families: data::Storage::Static(&[
                                                                    data::graph::FamilyTransfer {
                                                                        family: data::graph::StorageFamily::Int,
                                                                        length: 0,
                                                                        steps: data::Storage::Static(&[]),
                                                                    },
                                                                ]),
                                                            },
                                                        }),
                                                    ]),
                                                    fallback: data::graph::Edge {
                                                        target: data::graph::BlockId(2),
                                                        args: data::Storage::Static(&[
                                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                                        transfer: data::graph::Transfer {
                                                            families: data::Storage::Static(&[]),
                                                        },
                                                    },
                                                }),
                                            },
                                            data::graph::BlockHeader {
                                                params: 2..3,
                                                instructions: 0..0,
                                                terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                            },
                                            data::graph::BlockHeader {
                                                params: 3..5,
                                                instructions: 0..2,
                                                terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                    edge: data::graph::Edge {
                                                        target: data::graph::BlockId(0),
                                                        args: data::Storage::Static(&[
                                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                            data::graph::ParamLocal::IntFunction {
                                                                local: data::graph::IntFunctionLocalId(1),
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::Int,
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                                },
                                                            },
                                                        ]),
                                                        transfer: data::graph::Transfer {
                                                            families: data::Storage::Static(&[
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::Int,
                                                                    length: 1,
                                                                    steps: data::Storage::Static(&[
                                                                        data::graph::TransferStep {
                                                                            source: 1,
                                                                            destination: 0,
                                                                        },
                                                                    ]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::IntFunction,
                                                                    length: 1,
                                                                    steps: data::Storage::Static(&[
                                                                        data::graph::TransferStep {
                                                                            source: 1,
                                                                            destination: 0,
                                                                        },
                                                                    ]),
                                                                },
                                                            ]),
                                                        },
                                                    },
                                                }),
                                            },
                                        ]),
                                        params: data::Storage::Static(&[
                                            data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
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
                                                shape: data::type_::ValueShapeId(5),
                                            },
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
                                                shape: data::type_::ValueShapeId(5),
                                            },
                                            data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
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
                                                shape: data::type_::ValueShapeId(5),
                                            },
                                        ]),
                                        instructions: data::Storage::Static(&[
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    shape: data::type_::ValueShapeId(0),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                                    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                    right: data::graph::IntegerOperand::Immediate(1),
                                                }),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                                    shape: data::type_::ValueShapeId(5),
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
                                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(5)),
                                                        captures: data::Storage::Static(&[
                                                            data::graph::FunctionCapture::IntFunction {
                                                                target: data::graph::IntFunctionLocalId(0),
                                                                source: data::graph::IntFunctionLocalId(0),
                                                            },
                                                        ]),
                                                    },
                                                }),
                                            }),
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
                    external_function_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 1,
                            },
                            body: data::function::ProfiledExternalFunctionFunctionBody {
                                _shape: data::type_::FunctionShape {
                                    shape_id: data::type_::ValueShapeId(3),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[]),
                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                    },
                                },
                                _type: data::type_::ExternalFunctionType {
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[]),
                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                    },
                                    arguments: data::Storage::Static(&[]),
                                    return_: data::type_::ExternalTypeId(0),
                                },
                                body: data::function::ProfiledFunctionBody {
                                    block_graph: data::graph::ProfiledBlockGraph {
                                        entry: data::graph::BlockId(0),
                                        blocks: data::Storage::Static(&[
                                            data::graph::BlockHeader {
                                                params: 0..1,
                                                instructions: 0..0,
                                                terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                            },
                                        ]),
                                        params: data::Storage::Static(&[
                                            data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                    id: data::graph::ExternalFunctionLocalId(0),
                                                    type_: data::type_::ExternalFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                        },
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::type_::ExternalTypeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                        ]),
                                        instructions: data::Storage::Static(&[]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::Return(data::graph::ExternalFunctionLocal {
                                            id: data::graph::ExternalFunctionLocalId(0),
                                            type_: data::type_::ExternalFunctionType {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                },
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::type_::ExternalTypeId(0),
                                            },
                                        }),
                                    ]),
                                },
                            },
                        })),
                    ]),
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
                    external_list_function_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 1,
                            },
                            body: data::function::TypedFunctionBody {
                                _shape: data::type_::FunctionShape {
                                    shape_id: data::type_::ValueShapeId(4),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[]),
                                        return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                    },
                                },
                                body: data::function::ProfiledFunctionBody {
                                    block_graph: data::graph::ProfiledBlockGraph {
                                        entry: data::graph::BlockId(0),
                                        blocks: data::Storage::Static(&[
                                            data::graph::BlockHeader {
                                                params: 0..1,
                                                instructions: 0..0,
                                                terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                            },
                                        ]),
                                        params: data::Storage::Static(&[
                                            data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::External {
                                                    local: data::graph::ExternalListFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                    },
                                                    list_type: data::type_::ExternalListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item_type: data::type_::ExternalTypeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(4),
                                            },
                                        ]),
                                        instructions: data::Storage::Static(&[]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::Return(data::graph::ListFunctionLocal::External {
                                            local: data::graph::ExternalListFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                            },
                                            list_type: data::type_::ExternalListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::ExternalTypeId(0),
                                            },
                                        }),
                                    ]),
                                },
                            },
                        })),
                    ]),
                    float_list_function_functions: data::Storage::Static(&[]),
                    bool_list_function_functions: data::Storage::Static(&[]),
                    nil_list_function_functions: data::Storage::Static(&[]),
                    tuple_list_function_functions: data::Storage::Static(&[]),
                    list_list_function_functions: data::Storage::Static(&[]),
                    function_list_function_functions: data::Storage::Static(&[]),
                    function_function_functions: data::Storage::Static(&[]),
                },
            },
            compiled: {

                enum CompiledResume {
                    Exit(data::compiled::CompiledProgress),
                }
                use data::compiled::calls::{BoolCallable, CallArguments, CallCapture, CallCaptureInputs, CallExecution, CallInputs, CallInteger, CallOps, CallOutput, CallProgress, CallStorage, CallValues, IntCallable};
                enum FunctionState {
                    Int0Point0 { int0: i128, int1: i128 },
                    Int0Point1 { int0: i128, int1: i128, int2: i128 },
                    Int2Point0 { int0: i128 },
                    Int3Point0 { int0: i128, int_function0: IntCallable },
                    Int3Point1 { int0: i128, int_function0: IntCallable, int1: i128 },
                    Int5Point0 { int0: i128, int_function0: IntCallable },
                    Int5Point1 { int0: i128, int_function0: IntCallable, int1: i128 },
                    Int5Point2 { int0: i128, int_function0: IntCallable, int1: i128, int2: i128 },
                    IntFunction0Point0 { int0: i128, int_function0: IntCallable },
                    IntFunction0Point1 { int_function0: IntCallable },
                    IntFunction0Point2 { int0: i128, int_function0: IntCallable },
                    IntFunction0Point3 { int0: i128, int_function0: IntCallable, int1: i128 },
                    IntFunction0Point4 { int0: i128, int_function0: IntCallable, int1: i128, int_function1: IntCallable },
                    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: CallValues },
                }
                enum IntReturn {
                    Int3Call0 { int0: i128, int_function0: IntCallable },
                    Int5Call0 { int0: i128, int_function0: IntCallable },
                }
                impl IntReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                            Self::Int3Call0 { .. } => data::source::HostCallSite::from_static("app", "<anonymous:6>", data::source::SourceSpan::new(1255, 1270)),
                            Self::Int5Call0 { .. } => data::source::HostCallSite::from_static("app", "<anonymous:4>", data::source::SourceSpan::new(892, 907)),
                        }
                    }
                    fn small(self, result: i128) -> FunctionState {
                        match self {
                            Self::Int3Call0 { int0, int_function0 } => {
                                let int1 = result;
                                FunctionState::Int3Point1 { int0, int_function0, int1 }
                            },
                            Self::Int5Call0 { int0, int_function0 } => {
                                let int1 = result;
                                FunctionState::Int5Point1 { int0, int_function0, int1 }
                            },
                        }
                    }
                    fn resume(self, result: CallInteger) -> FunctionState {
                        if let Some(result) = result.small() {
                            return self.small(result);
                        }
                        match self {
                            Self::Int3Call0 { int0, int_function0 } => {
                                let int1 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 1,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 1,
                                    bool_functions: 0,
                                }, values: CallValues { ints: vec![int0.into(), int1], bools: vec![], int_functions: vec![int_function0], bool_functions: vec![] } }
                            },
                            Self::Int5Call0 { int0, int_function0 } => {
                                let int1 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(5)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 1,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 1,
                                    bool_functions: 0,
                                }, values: CallValues { ints: vec![int0.into(), int1], bools: vec![], int_functions: vec![int_function0], bool_functions: vec![] } }
                            },
                        }
                    }
                }
                enum BoolReturn {
                }
                impl BoolReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                        }
                    }
                    fn small(self, result: bool) -> FunctionState {
                        let _ = result;
                        match self {
                        }
                    }
                    fn resume(self, result: bool) -> FunctionState { self.small(result) }
                }
                enum IntFunctionReturn {
                }
                impl IntFunctionReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                        }
                    }
                    fn small(self, result: IntCallable) -> FunctionState {
                        let _ = result;
                        match self {
                        }
                    }
                    fn resume(self, result: IntCallable) -> FunctionState { self.small(result) }
                }
                enum BoolFunctionReturn {
                }
                impl BoolFunctionReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                        }
                    }
                    fn small(self, result: BoolCallable) -> FunctionState {
                        let _ = result;
                        match self {
                        }
                    }
                    fn resume(self, result: BoolCallable) -> FunctionState { self.small(result) }
                }
                #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                enum FunctionStep {
                    Next(FunctionState),
                    Yield(FunctionState),
                    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: CallValues },
                    IntCall { callee: FunctionState, caller: IntReturn },
                    Int { value: i128, exit: data::graph::BlockGraphExitId },
                    IntBridge { function: data::function::IntFunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: IntReturn },
                    IntFunction { value: IntCallable, exit: data::graph::BlockGraphExitId },
                }
                struct FunctionExecution {
                    active: Option<FunctionState>,
                    integer_returns: Vec<IntReturn>,
                    boolean_returns: Vec<BoolReturn>,
                    integer_function_returns: Vec<IntFunctionReturn>,
                    boolean_function_returns: Vec<BoolFunctionReturn>,
                }
                impl FunctionExecution {
                    fn new(active: FunctionState) -> Self {
                        Self {
                            active: Some(active),
                            integer_returns: Vec::new(),
                            boolean_returns: Vec::new(),
                            integer_function_returns: Vec::new(),
                            boolean_function_returns: Vec::new(),
                        }
                    }
                }
                impl CallExecution for FunctionExecution {
                    fn restart(&mut self, target: data::compiled::CallTarget, point: usize, values: CallInputs<'_>) -> bool {
                        if self.active.is_some() { return false; }
                        let active = match target {
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(0)) => calls_int_0_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(2)) => calls_int_2_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(3)) => calls_int_3_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(5)) => calls_int_5_state(point, values),
                            data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(0)) => calls_intfunction_0_state(point, values),
                            _ => None,
                        };
                        let Some(active) = active else { return false; };
                        self.active = Some(active);
                        true
                    }
                    fn retained_bytes(&self) -> usize {
                        std::mem::size_of::<Self>() + self.integer_returns.capacity() * std::mem::size_of::<IntReturn>() + self.boolean_returns.capacity() * std::mem::size_of::<BoolReturn>() + self.integer_function_returns.capacity() * std::mem::size_of::<IntFunctionReturn>() + self.boolean_function_returns.capacity() * std::mem::size_of::<BoolFunctionReturn>()
                    }
                    fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                        let Some(mut active) = self.active.take() else { return CallProgress::Yield(self); };
                        loop {
                            match function_step(active, ops, budget) {
                                FunctionStep::Next(next) => active = next,
                                FunctionStep::Yield(active) => {
                                    self.active = Some(active);
                                    return CallProgress::Yield(self);
                                },
                                FunctionStep::IntCall { callee, caller } => {
                                    self.integer_returns.push(caller);
                                    active = callee;
                                },
                                FunctionStep::Int { value, exit } => {
                                    if let Some(caller) = self.integer_returns.pop() {
                                        active = caller.small(value);
                                    } else {
                                        self.integer_returns.clear();
                                        self.boolean_returns.clear();
                                        self.integer_function_returns.clear();
                                        self.boolean_function_returns.clear();
                                        return CallProgress::Complete { exit, output: CallOutput::Int(value.into()), execution: self };
                                    }
                                },
                                FunctionStep::IntBridge { function, site, arguments, caller } => return CallProgress::Int {
                                    function, site, arguments,
                                    resume: Box::new(move |value| {
                                        self.active = Some(caller.resume(value));
                                        self
                                    }),
                                },
                                FunctionStep::IntFunction { value, exit } => {
                                    if let Some(caller) = self.integer_function_returns.pop() {
                                        active = caller.small(value);
                                    } else {
                                        self.integer_returns.clear();
                                        self.boolean_returns.clear();
                                        self.integer_function_returns.clear();
                                        self.boolean_function_returns.clear();
                                        return CallProgress::Complete { exit, output: CallOutput::IntFunction(value), execution: self };
                                    }
                                },
                                FunctionStep::Canonical { target, point, values } => {
                                    match target {
                                        data::compiled::CallTarget::Int(function) => {
                                            if let Some(caller) = self.integer_returns.pop() {
                                                let site = caller.site();
                                                return CallProgress::InterpretedInt {
                                                    function, site, point, values,
                                                    resume: Box::new(move |value| {
                                                        self.active = Some(caller.resume(value));
                                                        self
                                                    }),
                                                };
                                            }
                                            return CallProgress::Interpreted { point, values };
                                        },
                                        data::compiled::CallTarget::Bool(function) => {
                                            if let Some(caller) = self.boolean_returns.pop() {
                                                let site = caller.site();
                                                return CallProgress::InterpretedBool {
                                                    function, site, point, values,
                                                    resume: Box::new(move |value| {
                                                        self.active = Some(caller.resume(value));
                                                        self
                                                    }),
                                                };
                                            }
                                            return CallProgress::Interpreted { point, values };
                                        },
                                        data::compiled::CallTarget::IntFunction(function) => {
                                            if let Some(caller) = self.integer_function_returns.pop() {
                                                let site = caller.site();
                                                return CallProgress::InterpretedIntFunction {
                                                    function, site, point, values,
                                                    resume: Box::new(move |value| {
                                                        self.active = Some(caller.resume(value));
                                                        self
                                                    }),
                                                };
                                            }
                                            return CallProgress::Interpreted { point, values };
                                        },
                                        data::compiled::CallTarget::BoolFunction(function) => {
                                            if let Some(caller) = self.boolean_function_returns.pop() {
                                                let site = caller.site();
                                                return CallProgress::InterpretedBoolFunction {
                                                    function, site, point, values,
                                                    resume: Box::new(move |value| {
                                                        self.active = Some(caller.resume(value));
                                                        self
                                                    }),
                                                };
                                            }
                                            return CallProgress::Interpreted { point, values };
                                        },
                                    }
                                },
                            }
                        }
                    }
                }
                fn function_step(active: FunctionState, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        FunctionState::Canonical { target, point, values } => FunctionStep::Canonical { target, point, values },
                        FunctionState::Int0Point0 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point0 { int0, int1 }); }
                            *budget -= 1;
                            let int2 = int0 + int1;
                            if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 1,
                                ints: 3,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point1 { int0, int1, int2 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int2, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int0Point1 { int0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point1 { int0, int1, int2 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int2, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int2Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point0 { int0 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int3Point0 { int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point0 { int0, int_function0 }); }
                            *budget -= 1;
                            let callable = &int_function0;
                            let captures = callable.captures();
                            let target = callable.target();
                            if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (int0,)) {
                                return FunctionStep::IntCall { callee, caller: IntReturn::Int3Call0 { int0, int_function0 } };
                            }
                            FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("app", "<anonymous:6>", data::source::SourceSpan::new(1255, 1270)), arguments: CallArguments { values: CallValues { ints: vec![int0.into()], bools: vec![], int_functions: vec![], bool_functions: vec![] }, captures: Some(captures.retain()) }, caller: IntReturn::Int3Call0 { int0, int_function0 } }
                        },
                        FunctionState::Int3Point1 { int0, int_function0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point1 { int0, int_function0, int1 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int1, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int5Point0 { int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int5Point0 { int0, int_function0 }); }
                            *budget -= 1;
                            let callable = &int_function0;
                            let captures = callable.captures();
                            let target = callable.target();
                            if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (int0,)) {
                                return FunctionStep::IntCall { callee, caller: IntReturn::Int5Call0 { int0, int_function0 } };
                            }
                            FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("app", "<anonymous:4>", data::source::SourceSpan::new(892, 907)), arguments: CallArguments { values: CallValues { ints: vec![int0.into()], bools: vec![], int_functions: vec![], bool_functions: vec![] }, captures: Some(captures.retain()) }, caller: IntReturn::Int5Call0 { int0, int_function0 } }
                        },
                        FunctionState::Int5Point1 { int0, int_function0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int5Point1 { int0, int_function0, int1 }); }
                            *budget -= 1;
                            let int2 = int1 + 1_i128;
                            if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(5)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 2,
                                ints: 3,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 1,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![], int_functions: vec![int_function0], bool_functions: vec![] } }; }
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int5Point2 { int0, int_function0, int1, int2 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int2, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int5Point2 { int0, int_function0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int5Point2 { int0, int_function0, int1, int2 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int2, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::IntFunction0Point0 { int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction0Point0 { int0, int_function0 }); }
                            *budget -= 1;
                            match int0 {
                                0_i128 => FunctionStep::Next(FunctionState::IntFunction0Point1 { int_function0: int_function0.clone() }),
                                _ => FunctionStep::Next(FunctionState::IntFunction0Point2 { int0, int_function0: int_function0.clone() }),
                            }
                        },
                        FunctionState::IntFunction0Point1 { int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction0Point1 { int_function0 }); }
                            *budget -= 1;
                            FunctionStep::IntFunction { value: int_function0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::IntFunction0Point2 { int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction0Point2 { int0, int_function0 }); }
                            *budget -= 1;
                            let int1 = int0 - 1_i128;
                            if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(2),
                                instruction: 1,
                                ints: 2,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 1,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_functions: vec![int_function0], bool_functions: vec![] } }; }
                            FunctionStep::Next(FunctionState::IntFunction0Point3 { int0, int_function0, int1 })
                        },
                        FunctionState::IntFunction0Point3 { int0, int_function0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction0Point3 { int0, int_function0, int1 }); }
                            *budget -= 1;
                            let int_function1 = ops.int_closure(data::function::IntFunctionId(5), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            }, vec![CallCapture::int_function(data::graph::IntFunctionLocalId(0), int_function0.clone())]);
                            FunctionStep::Next(FunctionState::IntFunction0Point4 { int0, int_function0, int1, int_function1 })
                        },
                        FunctionState::IntFunction0Point4 { int0, int_function0, int1, int_function1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction0Point4 { int0, int_function0, int1, int_function1 }); }
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::IntFunction0Point0 { int0: int1, int_function0: int_function1.clone() })
                        },
                    }
                }
                fn calls_entry_0(target: data::function::IntFunctionId, captures: &CallCaptureInputs<'_>, inputs: (i128,)) -> Option<FunctionState> {
                    let (argument0,) = inputs;
                    match target.0 {
                        0 => Some(FunctionState::Int0Point0 { int0: argument0, int1: captures.int(data::graph::IntLocalId(1))? }),
                        2 => Some(FunctionState::Int2Point0 { int0: argument0 }),
                        3 => Some(FunctionState::Int3Point0 { int0: argument0, int_function0: captures.int_function(data::graph::IntFunctionLocalId(0))? }),
                        5 => Some(FunctionState::Int5Point0 { int0: argument0, int_function0: captures.int_function(data::graph::IntFunctionLocalId(0))? }),
                        _ => None,
                    }
                }
                fn calls_int_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int0Point0 { int0: values.int(0)?, int1: values.int(1)? },
                        1 => FunctionState::Int0Point1 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point, values) { return Some(execution); }
                    let active = calls_int_0_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_2_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int2Point0 { int0: values.int(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_2_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point, values) { return Some(execution); }
                    let active = calls_int_2_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_3_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => {
                            if !matches!(values.int_function_target(0)?, data::function::IntFunctionId(0) | data::function::IntFunctionId(2) | data::function::IntFunctionId(3) | data::function::IntFunctionId(5)) { return None; }
                            FunctionState::Int3Point0 { int0: values.int(0)?, int_function0: values.int_function(0)? }
                        },
                        1 => FunctionState::Int3Point1 { int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_3_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point, values) { return Some(execution); }
                    let active = calls_int_3_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_5_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => {
                            if !matches!(values.int_function_target(0)?, data::function::IntFunctionId(0) | data::function::IntFunctionId(2) | data::function::IntFunctionId(3) | data::function::IntFunctionId(5)) { return None; }
                            FunctionState::Int5Point0 { int0: values.int(0)?, int_function0: values.int_function(0)? }
                        },
                        1 => FunctionState::Int5Point1 { int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)? },
                        2 => FunctionState::Int5Point2 { int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_5_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(5)), point, values) { return Some(execution); }
                    let active = calls_int_5_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_intfunction_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::IntFunction0Point0 { int0: values.int(0)?, int_function0: values.int_function(0)? },
                        1 => FunctionState::IntFunction0Point1 { int_function0: values.int_function(0)? },
                        2 => FunctionState::IntFunction0Point2 { int0: values.int(0)?, int_function0: values.int_function(0)? },
                        3 => FunctionState::IntFunction0Point3 { int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)? },
                        4 => FunctionState::IntFunction0Point4 { int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)?, int_function1: values.int_function(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_intfunction_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(0)), point, values) { return Some(execution); }
                    let active = calls_intfunction_0_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }

                fn string_int_1(
                    point: usize,
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {

                    const RESUME: [
                        fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                        2
                    ] = [
                        |values, budget| CompiledResume::Exit(string_int_1_entry((values.ints[0],), values, budget)),
                        string_int_1_resume_1,
                    ];
                    let CompiledResume::Exit(progress) = RESUME[point](values, budget);
                    progress
                }

                fn string_int_1_entry(
                    inputs: (i128,),
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {
                    let (b0_i0,) = inputs;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(0);
                    }
                    *budget -= 1;
                    let b0_s0 = data::compiled::string::StringRange::literal("prepared work failed");
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b0_s0]);
                        return data::compiled::CompiledProgress::Yield(1);
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0]);
                    data::compiled::CompiledProgress::Interpreted(1)
                }

                fn string_int_1_resume_1(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b0_i0, b0_s0,) = (values.ints[0], values.strings[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b0_s0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(1))
                }
                data::compiled::CompiledFunctions {
                    ints: data::Storage::Static(&[
                        data::compiled::CompiledFunction {
                            function: data::function::IntFunctionId(1),
                            implementation: data::compiled::CompiledImplementation::String(data::compiled::StringImplementation {
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                run: string_int_1,
                            }),
                        },
                    ]),
                    bools: data::Storage::Static(&[
                    ]),
                    customs: data::Storage::Static(&[
                    ]),
                    int_lists: data::Storage::Static(&[
                    ]),
                    callbacks: data::compiled::CompiledCallbacks::interpreted(),
                    function_calls: data::Storage::Static(&[
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: false,
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 3,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 1,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_int_0_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: false,
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 0,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_int_2_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: true,
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 0,
                                        output: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        target: data::compiled::CallContractTarget::IntValue(data::graph::IntFunctionLocalId(0)),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("app", "<anonymous:6>", data::source::SourceSpan::new(1255, 1270)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 1,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_int_3_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(5)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: true,
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 3,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 0,
                                        output: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        target: data::compiled::CallContractTarget::IntValue(data::graph::IntFunctionLocalId(0)),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("app", "<anonymous:4>", data::source::SourceSpan::new(892, 907)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 2,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_int_5_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(0)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: true,
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 1,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 2,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 2,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                    data::Storage::Static(&[
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
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(1),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[
                                    data::compiled::CreationContract {
                                        point: 3,
                                        output: data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(1),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(5)),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                        reference: false,
                                        captures: data::Storage::Static(&[
                                            data::graph::FunctionCapture::IntFunction {
                                                target: data::graph::IntFunctionLocalId(0),
                                                source: data::graph::IntFunctionLocalId(0),
                                            },
                                        ]),
                                    },
                                ]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 1,
                                        value: data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_intfunction_0_start,
                            })),
                        },
                    ]),
                }
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
                    0..6,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    6..8,
                    8..18,
                    0..0,
                    0..0,
                    18..19,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    19..21,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    21..22,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    22..23,
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
                    23..24,
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
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                shape: data::type_::ValueShapeId(0),
                            },
                        ]),
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
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 3..4,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[
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
                                shape: data::type_::ValueShapeId(5),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 4..5,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                    id: data::graph::CustomLocalId(0),
                                    shape: data::type_::CustomValueShape {
                                        type_id: data::type_::CustomTypeId(0),
                                        shape_id: data::type_::CustomValueShapeId(0),
                                    },
                                }),
                                shape: data::type_::ValueShapeId(7),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 5..6,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[
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
                                shape: data::type_::ValueShapeId(5),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 6..7,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(7),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 7..9,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(7),
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(7),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 9..10,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 10..11,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(11),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 11..12,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 12..12,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 12..13,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(7),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 13..14,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 14..15,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 15..15,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                    id: data::graph::ExternalLocalId(0),
                                    type_id: data::type_::ExternalTypeId(0),
                                }),
                                shape: data::type_::ValueShapeId(1),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 15..17,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                            data::type_::ValueShapeId(5),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 17..18,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(2),
                        ]),
                        return_: data::type_::ValueShapeId(11),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 18..19,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(7),
                        ]),
                        return_: data::type_::ValueShapeId(9),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 19..20,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(2),
                        ]),
                        return_: data::type_::ValueShapeId(2),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 20..20,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(2),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                    id: data::graph::ExternalLocalId(0),
                                    type_id: data::type_::ExternalTypeId(0),
                                }),
                                shape: data::type_::ValueShapeId(1),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 20..22,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(5),
                        ]),
                        return_: data::type_::ValueShapeId(5),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 22..23,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(3),
                        ]),
                        return_: data::type_::ValueShapeId(3),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 23..24,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(4),
                        ]),
                        return_: data::type_::ValueShapeId(4),
                        captures: data::Storage::Static(&[]),
                    },
                ]),
                parameters: data::Storage::Static(&[
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(0),
                        },
                    }),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    }),
                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(0),
                        },
                    }),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    }),
                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
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
                    data::graph::ParamLocal::List(data::graph::ListLocal::External {
                        local: data::graph::ExternalListLocalId(0),
                        type_id: data::type_::ExternalListTypeId {
                            list_type: data::type_::ListTypeId(0),
                            item_type: data::type_::ExternalTypeId(0),
                        },
                    }),
                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(0),
                        },
                    }),
                    data::graph::ParamLocal::List(data::graph::ListLocal::External {
                        local: data::graph::ExternalListLocalId(0),
                        type_id: data::type_::ExternalListTypeId {
                            list_type: data::type_::ListTypeId(0),
                            item_type: data::type_::ExternalTypeId(0),
                        },
                    }),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::IntFunction {
                        local: data::graph::IntFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        },
                    },
                    data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                        id: data::graph::ExternalFunctionLocalId(0),
                        type_: data::type_::ExternalFunctionType {
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[]),
                                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                            },
                            arguments: data::Storage::Static(&[]),
                            return_: data::type_::ExternalTypeId(0),
                        },
                    }),
                    data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::External {
                        local: data::graph::ExternalListFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                        },
                        list_type: data::type_::ExternalListTypeId {
                            list_type: data::type_::ListTypeId(0),
                            item_type: data::type_::ExternalTypeId(0),
                        },
                    }),
                ]),
            },
            list_types: data::type_::ListTypeTable {
                types: data::Storage::Static(&[
                    data::type_::ListStorageTypeId::External(data::type_::ExternalListTypeId {
                        list_type: data::type_::ListTypeId(0),
                        item_type: data::type_::ExternalTypeId(0),
                    }),
                    data::type_::ListStorageTypeId::Int(data::type_::IntListTypeId {
                        list_type: data::type_::ListTypeId(1),
                    }),
                ]),
                tuple_items: data::Storage::Static(&[]),
                function_items: data::Storage::Static(&[]),
            },
            custom_types: data::type_::CustomTypeTable {
                types: data::Storage::Static(&[
                    data::type_::CustomTypeDescriptor {
                        type_: data::type_::NominalTypeMetadata {
                            package: data::Text::Static("app"),
                            module: data::Text::Static("app"),
                            name: data::Text::Static("Captured"),
                            arguments: data::Storage::Static(&[]),
                        },
                        constructor_count: 1,
                        constructors: data::Storage::Static(&[
                            data::type_::CustomConstructorDescriptor {
                                id: data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                                name: data::Text::Static("Captured"),
                                native_tag: data::Text::Static("captured"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: Some(data::Text::Static("callback")),
                                        type_: data::type_::ValueType::Function(data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        }),
                                        shape: data::type_::ValueShapeId(5),
                                        refinement: data::type_::FieldRefinement::Function {
                                            arguments: data::Storage::Static(&[
                                                data::type_::FieldRefinement::Value,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::FieldRefinement::Value),
                                        },
                                    },
                                ]),
                            },
                        ]),
                    },
                ]),
                definitions: data::Storage::Static(&[
                    data::type_::CustomDefinition {
                        package: data::Text::Static("app"),
                        module: data::Text::Static("app"),
                        name: data::Text::Static("Captured"),
                        publicity: data::type_::CustomTypePublicity::Public,
                        opaque: false,
                        parameters: 0,
                        constructors: data::Storage::Static(&[
                            data::type_::ConstructorDefinition {
                                name: data::Text::Static("Captured"),
                                fields: data::Storage::Static(&[
                                    data::type_::FieldDefinition {
                                        label: Some(data::Text::Static("callback")),
                                        type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                            arguments: data::Storage::Static(&[
                                                data::type_::TypeMetadata::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                                        }),
                                    },
                                ]),
                            },
                        ]),
                    },
                ]),
            },
            external_types: data::type_::ExternalTypeTable {
                types: data::Storage::Static(&[
                    data::type_::NominalTypeMetadata {
                        package: data::Text::Static("work_fixture"),
                        module: data::Text::Static("fixture/work"),
                        name: data::Text::Static("Work"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                    },
                    data::type_::NominalTypeMetadata {
                        package: data::Text::Static("work_fixture"),
                        module: data::Text::Static("fixture/work"),
                        name: data::Text::Static("Work"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                        ]),
                    },
                ]),
            },
            value_shapes: data::type_::ValueShapeTable {
                shapes: data::Storage::Static(&[
                    data::type_::ValueShapeDescriptor::Int,
                    data::type_::ValueShapeDescriptor::External(data::type_::ExternalTypeId(0)),
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(1)),
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(1),
                    },
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(2),
                    },
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                    },
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(1)),
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(0)),
                    data::type_::ValueShapeDescriptor::Bool,
                    data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                        data::type_::ValueShapeId(8),
                        data::type_::ValueShapeId(8),
                    ])),
                    data::type_::ValueShapeDescriptor::String,
                    data::type_::ValueShapeDescriptor::External(data::type_::ExternalTypeId(1)),
                ]),
                shape_types: data::Storage::Static(&[
                    data::type_::ValueType::Int,
                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                    }),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                    }),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    }),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::Bool,
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::Bool,
                        data::type_::ValueType::Bool,
                    ])),
                    data::type_::ValueType::String,
                    data::type_::ValueType::External(data::type_::ExternalTypeId(1)),
                ]),
                custom_shapes: data::Storage::Static(&[
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(0),
                        arguments: data::Storage::Static(&[]),
                        constructor: data::type_::CustomConstructorRefinement::Any,
                    },
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(0),
                        arguments: data::Storage::Static(&[]),
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
            customs: data::Storage::Static(&[
                data::program::LibraryFunctionEntry {
                    function: data::function::CustomFunctionId {
                        index: 0,
                        return_shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(0),
                        },
                    },
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
                data::program::LibraryFunctionEntry {
                    function: data::function::CustomFunctionId {
                        index: 1,
                        return_shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(0),
                        },
                    },
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
            externals: data::Storage::Static(&[
                data::program::LibraryFunctionEntry {
                    function: data::function::ExternalFunctionId {
                        index: 0,
                        return_type: data::type_::ExternalTypeId(0),
                    },
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
                data::program::LibraryFunctionEntry {
                    function: data::function::ExternalFunctionId {
                        index: 1,
                        return_type: data::type_::ExternalTypeId(1),
                    },
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
                data::program::LibraryFunctionEntry {
                    function: data::function::ExternalFunctionId {
                        index: 2,
                        return_type: data::type_::ExternalTypeId(0),
                    },
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
                data::program::LibraryFunctionEntry {
                    function: data::function::ExternalFunctionId {
                        index: 3,
                        return_type: data::type_::ExternalTypeId(0),
                    },
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
                data::program::LibraryFunctionEntry {
                    function: data::function::ExternalFunctionId {
                        index: 4,
                        return_type: data::type_::ExternalTypeId(0),
                    },
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
                name: data::Text::Static("make"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("work_fixture"),
                        module: data::Text::Static("fixture/work"),
                        name: data::Text::Static("Work"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                    })),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("collect"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("work_fixture"),
                        module: data::Text::Static("fixture/work"),
                        name: data::Text::Static("Work"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                        ]),
                    })),
                },
                slot: 1,
            },
            data::Export {
                name: data::Text::Static("keep"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("work_fixture"),
                            module: data::Text::Static("fixture/work"),
                            name: data::Text::Static("Work"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                        }),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("work_fixture"),
                        module: data::Text::Static("fixture/work"),
                        name: data::Text::Static("Work"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                    })),
                },
                slot: 2,
            },
            data::Export {
                name: data::Text::Static("failure"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("work_fixture"),
                        module: data::Text::Static("fixture/work"),
                        name: data::Text::Static("Work"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                    })),
                },
                slot: 3,
            },
            data::Export {
                name: data::Text::Static("capture"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("app"),
                        module: data::Text::Static("app"),
                        name: data::Text::Static("Captured"),
                        arguments: data::Storage::Static(&[]),
                    })),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("extend"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("app"),
                            module: data::Text::Static("app"),
                            name: data::Text::Static("Captured"),
                            arguments: data::Storage::Static(&[]),
                        }),
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("app"),
                        module: data::Text::Static("app"),
                        name: data::Text::Static("Captured"),
                        arguments: data::Storage::Static(&[]),
                    })),
                },
                slot: 1,
            },
            data::Export {
                name: data::Text::Static("identities"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("app"),
                            module: data::Text::Static("app"),
                            name: data::Text::Static("Captured"),
                            arguments: data::Storage::Static(&[]),
                        }),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                        data::type_::TypeMetadata::Bool,
                        data::type_::TypeMetadata::Bool,
                    ]))),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("invoke"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("app"),
                            module: data::Text::Static("app"),
                            name: data::Text::Static("Captured"),
                            arguments: data::Storage::Static(&[]),
                        }),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("work_fixture"),
                        module: data::Text::Static("fixture/work"),
                        name: data::Text::Static("Work"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                    })),
                },
                slot: 4,
            },
        ]),
    },
    value_functions: data::Storage::Static(&[
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("work_fixture"),
            site: data::source::HostCallSite::from_static("fixture/work", "ready", data::source::SourceSpan::new(60, 86)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                    package: data::Text::Static("work_fixture"),
                    module: data::Text::Static("fixture/work"),
                    name: data::Text::Static("Work"),
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
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("work_fixture"),
                            module: data::Text::Static("fixture/work"),
                            name: data::Text::Static("Work"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                        }), data::type_::ExternalTypeId(0)),
                    ]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Int,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::External {
                    schema: data::host::ExternalSchema {
                        package: data::Text::Static("work_fixture"),
                        module: data::Text::Static("fixture/work"),
                        name: data::Text::Static("Work"),
                        parameter_count: 1,
                    },
                    arguments: data::Storage::Static(&[
                        data::host::RegistrationType::Parameter(0),
                    ]),
                },
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("work_fixture"),
                        module: data::Text::Static("fixture/work"),
                        name: data::Text::Static("Work"),
                        parameter_count: 1,
                    },
                ]),
                constructions: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("work_fixture"),
            site: data::source::HostCallSite::from_static("fixture/work", "map", data::source::SourceSpan::new(139, 187)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("work_fixture"),
                        module: data::Text::Static("fixture/work"),
                        name: data::Text::Static("Work"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                    }),
                    data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                    package: data::Text::Static("work_fixture"),
                    module: data::Text::Static("fixture/work"),
                    name: data::Text::Static("Work"),
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
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Int,
                    shape: data::type_::ValueShapeId(0),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::External(data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    })),
                    data::host::HostCallParameter::Function {
                        local: data::graph::ParamLocal::IntFunction {
                            local: data::graph::IntFunctionLocalId(0),
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            },
                        },
                        arity: 1,
                    },
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("work_fixture"),
                            module: data::Text::Static("fixture/work"),
                            name: data::Text::Static("Work"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                        }), data::type_::ExternalTypeId(0)),
                    ]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("work_fixture"),
                            module: data::Text::Static("fixture/work"),
                            name: data::Text::Static("Work"),
                            parameter_count: 1,
                        },
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Parameter(1),
                        ]),
                    },
                    data::host::RegistrationType::Function {
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Parameter(1),
                        ]),
                        return_: data::Storage::Static(&data::host::RegistrationType::Parameter(0)),
                    },
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::External {
                    schema: data::host::ExternalSchema {
                        package: data::Text::Static("work_fixture"),
                        module: data::Text::Static("fixture/work"),
                        name: data::Text::Static("Work"),
                        parameter_count: 1,
                    },
                    arguments: data::Storage::Static(&[
                        data::host::RegistrationType::Parameter(0),
                    ]),
                },
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::External(0),
                    data::host::RegistrationParameter::Function {
                        slot: 0,
                        arity: 1,
                    },
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("work_fixture"),
                        module: data::Text::Static("fixture/work"),
                        name: data::Text::Static("Work"),
                        parameter_count: 1,
                    },
                ]),
                constructions: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("work_fixture"),
            site: data::source::HostCallSite::from_static("fixture/work", "all", data::source::SourceSpan::new(325, 358)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("work_fixture"),
                        module: data::Text::Static("fixture/work"),
                        name: data::Text::Static("Work"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                    }))),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                    package: data::Text::Static("work_fixture"),
                    module: data::Text::Static("fixture/work"),
                    name: data::Text::Static("Work"),
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
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
                    data::host::HostCallParameter::List(data::graph::ParamLocal::List(data::graph::ListLocal::External {
                        local: data::graph::ExternalListLocalId(0),
                        type_id: data::type_::ExternalListTypeId {
                            list_type: data::type_::ListTypeId(0),
                            item_type: data::type_::ExternalTypeId(0),
                        },
                    })),
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)), data::type_::ListTypeId(1)),
                        (data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("work_fixture"),
                            module: data::Text::Static("fixture/work"),
                            name: data::Text::Static("Work"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                        }))), data::type_::ListTypeId(0)),
                    ]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("work_fixture"),
                            module: data::Text::Static("fixture/work"),
                            name: data::Text::Static("Work"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                        }), data::type_::ExternalTypeId(0)),
                        (data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("work_fixture"),
                            module: data::Text::Static("fixture/work"),
                            name: data::Text::Static("Work"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                            ]),
                        }), data::type_::ExternalTypeId(1)),
                    ]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(1))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::List(data::Storage::Static(&data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("work_fixture"),
                            module: data::Text::Static("fixture/work"),
                            name: data::Text::Static("Work"),
                            parameter_count: 1,
                        },
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Parameter(0),
                        ]),
                    })),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::External {
                    schema: data::host::ExternalSchema {
                        package: data::Text::Static("work_fixture"),
                        module: data::Text::Static("fixture/work"),
                        name: data::Text::Static("Work"),
                        parameter_count: 1,
                    },
                    arguments: data::Storage::Static(&[
                        data::host::RegistrationType::List(data::Storage::Static(&data::host::RegistrationType::Parameter(0))),
                    ]),
                },
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::List(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("work_fixture"),
                        module: data::Text::Static("fixture/work"),
                        name: data::Text::Static("Work"),
                        parameter_count: 1,
                    },
                ]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::List(data::Storage::Static(&data::host::RegistrationType::Parameter(0))),
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
    ]),
    never_functions: data::Storage::Static(&[]),
    callables: data::Storage::Static(&[]),
}
