data::HostedModuleArtifact {
    module: data::ModuleArtifact {
        format: 2,
        program: data::ProgramTables {
            root: data::source::module_id(1),
            modules: data::Storage::Static(&[
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("fixture/work"),
                    source_context: Some(data::source::SourceContext::from_static("src/fixture/work.gleam", "pub type Work(value)\n\n@external(erlang, \"fixture\", \"ready\")\npub fn ready(value: value) -> Work(value)\n\n@external(erlang, \"fixture\", \"map\")\npub fn map(value: Work(a), callback: fn(a) -> b) -> Work(b)\n\n@external(erlang, \"fixture\", \"flatten\")\npub fn flatten(value: Work(Work(a))) -> Work(a)\n\n@external(erlang, \"fixture\", \"all\")\npub fn all(values: List(Work(a))) -> Work(List(a))\n\npub fn then(value: Work(a), callback: fn(a) -> Work(b)) -> Work(b) {\n  flatten(map(value, callback))\n}\n")),
                },
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("app"),
                    source_context: Some(data::source::SourceContext::from_static("src/app.gleam", "import fixture/work\n\nfn identity(value) {\n  value\n}\n\npub fn make(seed: Int) -> work.Work(Int) {\n  let original = work.ready(seed)\n  let same = identity(original)\n  let assert [same_list] = identity([same])\n  let same_closure = identity(fn() { same_list })()\n  let assert [same_closure_list] = identity(fn() { [same_closure] })()\n  let offset = 2\n  use value <- work.map(same_closure_list)\n  value + offset\n}\n\npub fn collect(seed: Int) -> work.Work(List(Int)) {\n  let shared = make(seed)\n  work.all([shared, shared])\n}\n\npub fn keep(value: work.Work(Int)) -> work.Work(Int) {\n  value\n}\n\npub fn failure() -> work.Work(Int) {\n  use _ <- work.map(work.ready(0))\n  panic as \"prepared work failed\"\n}\n\npub type Captured {\n  Captured(callback: fn(Int) -> Int)\n}\n\nfn chain(depth: Int, previous: fn(Int) -> Int) -> fn(Int) -> Int {\n  case depth {\n    0 -> previous\n    _ -> chain(depth - 1, fn(value) { previous(value) + 1 })\n  }\n}\n\npub fn capture(depth: Int) -> Captured {\n  Captured(chain(depth, fn(value) { value }))\n}\n\npub fn extend(value: Captured, depth: Int) -> Captured {\n  Captured(chain(depth, value.callback))\n}\n\npub fn identities(value: Captured) -> #(Bool, Bool) {\n  let callback = value.callback\n  #(callback == value.callback, callback == fn(input) { callback(input) })\n}\n\npub fn invoke(value: Captured) -> work.Work(Int) {\n  use initial <- work.map(work.ready(1))\n  value.callback(initial)\n}\n")),
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
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                shape: data::type_::ValueShapeId(10),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("prepared work failed"))),
                                        },
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
                                        data::graph::ProfiledInstruction {
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
                                        },
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
                                        },
                                        data::graph::ProfiledInstruction {
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
                                        },
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
                                            instructions: 0..3,
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
                                        data::graph::ProfiledInstruction {
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
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    1,
                                                ]),
                                            })),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntLocalId(1),
                                                right: data::graph::IntLocalId(2),
                                            }),
                                        },
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(3)),
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
                                            },
                                            data::graph::ProfiledInstruction {
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
                                            },
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
                                            },
                                            data::graph::ProfiledInstruction {
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
                                            },
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
                                                                    positions: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::External,
                                                                    positions: data::Storage::Static(&[
                                                                        2,
                                                                    ]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::ExternalList,
                                                                    positions: data::Storage::Static(&[]),
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
                                                                    positions: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::External,
                                                                    positions: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::ExternalList,
                                                                    positions: data::Storage::Static(&[
                                                                        1,
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
                                                                    positions: data::Storage::Static(&[
                                                                        2,
                                                                    ]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::ExternalList,
                                                                    positions: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::ExternalFunction,
                                                                    positions: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::ExternalListFunction,
                                                                    positions: data::Storage::Static(&[]),
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
                                                                    positions: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::ExternalList,
                                                                    positions: data::Storage::Static(&[
                                                                        0,
                                                                    ]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::ExternalFunction,
                                                                    positions: data::Storage::Static(&[]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::ExternalListFunction,
                                                                    positions: data::Storage::Static(&[]),
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
                                            data::graph::ProfiledInstruction {
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
                                            },
                                            data::graph::ProfiledInstruction {
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
                                            },
                                            data::graph::ProfiledInstruction {
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
                                            },
                                            data::graph::ProfiledInstruction {
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
                                            },
                                            data::graph::ProfiledInstruction {
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
                                            },
                                            data::graph::ProfiledInstruction {
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
                                            },
                                            data::graph::ProfiledInstruction {
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
                                            },
                                            data::graph::ProfiledInstruction {
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
                                            },
                                            data::graph::ProfiledInstruction {
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
                                            },
                                            data::graph::ProfiledInstruction {
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
                                            },
                                            data::graph::ProfiledInstruction {
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
                                            },
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
                                                        positions: data::Storage::Static(&[]),
                                                    },
                                                    data::graph::FamilyTransfer {
                                                        family: data::graph::StorageFamily::External,
                                                        positions: data::Storage::Static(&[
                                                            0,
                                                        ]),
                                                    },
                                                    data::graph::FamilyTransfer {
                                                        family: data::graph::StorageFamily::IntFunction,
                                                        positions: data::Storage::Static(&[
                                                            0,
                                                        ]),
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
                                            data::graph::ProfiledInstruction {
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
                                            },
                                            data::graph::ProfiledInstruction {
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
                                            },
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
                                                        positions: data::Storage::Static(&[]),
                                                    },
                                                    data::graph::FamilyTransfer {
                                                        family: data::graph::StorageFamily::External,
                                                        positions: data::Storage::Static(&[]),
                                                    },
                                                    data::graph::FamilyTransfer {
                                                        family: data::graph::StorageFamily::ExternalList,
                                                        positions: data::Storage::Static(&[
                                                            0,
                                                        ]),
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
                                            data::graph::ProfiledInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    shape: data::type_::ValueShapeId(0),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                    sign: data::Sign::NoSign,
                                                    digits: data::Storage::Static(&[]),
                                                })),
                                            },
                                            data::graph::ProfiledInstruction {
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
                                            },
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
                                                        positions: data::Storage::Static(&[]),
                                                    },
                                                    data::graph::FamilyTransfer {
                                                        family: data::graph::StorageFamily::External,
                                                        positions: data::Storage::Static(&[
                                                            0,
                                                        ]),
                                                    },
                                                    data::graph::FamilyTransfer {
                                                        family: data::graph::StorageFamily::IntFunction,
                                                        positions: data::Storage::Static(&[
                                                            0,
                                                        ]),
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
                                            data::graph::ProfiledInstruction {
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
                                            },
                                            data::graph::ProfiledInstruction {
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
                                            },
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
                                                        positions: data::Storage::Static(&[]),
                                                    },
                                                    data::graph::FamilyTransfer {
                                                        family: data::graph::StorageFamily::Custom,
                                                        positions: data::Storage::Static(&[]),
                                                    },
                                                    data::graph::FamilyTransfer {
                                                        family: data::graph::StorageFamily::External,
                                                        positions: data::Storage::Static(&[
                                                            0,
                                                        ]),
                                                    },
                                                    data::graph::FamilyTransfer {
                                                        family: data::graph::StorageFamily::IntFunction,
                                                        positions: data::Storage::Static(&[
                                                            0,
                                                        ]),
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
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(8),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Equal {
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
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
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
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                shape: data::type_::ValueShapeId(8),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Equal {
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
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
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
                                        data::graph::ProfiledInstruction {
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
                                        },
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
                                                                        positions: data::Storage::Static(&[]),
                                                                    },
                                                                    data::graph::FamilyTransfer {
                                                                        family: data::graph::StorageFamily::IntFunction,
                                                                        positions: data::Storage::Static(&[
                                                                            0,
                                                                        ]),
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
                                                            families: data::Storage::Static(&[
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::Int,
                                                                    positions: data::Storage::Static(&[
                                                                        0,
                                                                    ]),
                                                                },
                                                                data::graph::FamilyTransfer {
                                                                    family: data::graph::StorageFamily::IntFunction,
                                                                    positions: data::Storage::Static(&[
                                                                        0,
                                                                    ]),
                                                                },
                                                            ]),
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
                                                instructions: 0..3,
                                                terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
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
                                            data::graph::ProfiledInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    shape: data::type_::ValueShapeId(0),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                    sign: data::Sign::Plus,
                                                    digits: data::Storage::Static(&[
                                                        1,
                                                    ]),
                                                })),
                                            },
                                            data::graph::ProfiledInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                    shape: data::type_::ValueShapeId(0),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                                    left: data::graph::IntLocalId(0),
                                                    right: data::graph::IntLocalId(1),
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
                                            },
                                        ]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::Return(data::graph::IntFunctionLocalId(0)),
                                        data::function::FunctionExit::TailCall {
                                            function: data::source::FunctionCallTarget {
                                                function: data::function::IntFunctionFunctionId(0),
                                                site: data::source::HostCallSite::from_static("app", "chain", data::source::SourceSpan::new(863, 914)),
                                            },
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
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
                                                        positions: data::Storage::Static(&[
                                                            2,
                                                        ]),
                                                    },
                                                    data::graph::FamilyTransfer {
                                                        family: data::graph::StorageFamily::IntFunction,
                                                        positions: data::Storage::Static(&[
                                                            1,
                                                        ]),
                                                    },
                                                ]),
                                            },
                                        },
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
                        },
                    },
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
                        },
                    },
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
                        },
                    },
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
                        },
                    },
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
                        },
                    },
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
                        },
                    },
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
                        },
                    },
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
                        },
                    },
                },
            ]),
            lists: data::Storage::Static(&[]),
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
}
