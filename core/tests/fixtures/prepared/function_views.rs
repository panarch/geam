data::HostedModuleArtifact {
    module: data::ModuleArtifact {
        format: 28,
        program: data::ProgramTables {
            root: data::source::module_id(0),
            modules: data::Storage::Static(&[
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("function_views"),
                    source_context: Some(data::source::SourceContext::from_static_block("src/function_views.gleam", r#"
pub type Erased

pub type Handler(a, b) {
  Handler(fn(a) -> b)
}

pub type Tree(a) {
  Leaf(a)
  Branch(List(Tree(a)))
}

pub type Empty {
  Again(Empty)
}

@external(erlang, "gleam@function", "identity")
fn coerce(value: a) -> b

@external(erlang, "fixture", "tick")
fn tick() -> Int

fn increment(a: Int) -> Int {
  a + 1
}

fn input(value: Int) -> Erased {
  coerce(value)
}

fn output(value: Erased) -> Int {
  coerce(value)
}

pub fn run() -> Bool {
  let bias = 2
  let original = fn(a: Int) { a + bias }
  let view: fn(Erased) -> Erased = coerce(original)
  let restored: fn(Int) -> Int = coerce(view)
  let handler: Handler(Erased, Erased) = coerce(Handler(increment))
  let Handler(callback) = handler
  let tree: Tree(fn(Erased) -> Erased) = coerce(Branch([Leaf(increment)]))
  let assert Branch([Leaf(leaf)]) = tree
  let nested: fn(Erased) -> fn(Erased) -> Erased =
    coerce(fn(a: Int) { fn(b: Int) { a + b } })
  let raw: String = coerce(<<255, 0, 195>>)
  let append = fn(a: String) { a <> raw }
  let byte_view: fn(BitArray) -> BitArray = coerce(append)
  let restored_bytes: fn(String) -> String = coerce(byte_view)
  output(view(input(40))) == 42
  && restored == original
  && restored(40) == 42
  && output(callback(input(41))) == 42
  && output(leaf(input(41))) == 42
  && output(nested(input(20))(input(22))) == 42
  && byte_view(<<255, 0, 195>>) == <<255, 0, 195, 255, 0, 195>>
  && restored_bytes == append
  && restored_bytes(raw) == raw <> raw
}

pub fn invalid_input() -> Bool {
  let view: fn(Erased) -> Erased = coerce(fn(a: Int) { tick() + a })
  let wrong: Erased = coerce("wrong")
  output(view(wrong)) == 42
}

pub fn stopped() -> Bool {
  let view: fn(Erased) -> Empty =
    coerce(fn(a: Int) {
      let _ = tick()
      panic as "view source stopped"
    })
  let _ = view(input(42))
  True
}
"#)),
                },
            ]),
            main: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Bool(data::function::BoolFunctionId(0))),
            functions: data::function::FunctionTables {
                value_returns: data::function::ValueFunctionTables {
                    never_functions: data::Storage::Static(&[
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
                                            terminator: data::graph::Terminator::SourceStop(data::graph::SourceStop {
                                                kind: data::graph::SourceStopKind::Panic,
                                                message: Some(data::graph::StringLocalId(0)),
                                                site: data::source::PanicSite::from_static("function_views", "<anonymous:5>", data::source::SourceSpan::new(1758, 1788)),
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(7),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("function_views", "<anonymous:5>", data::source::SourceSpan::new(1745, 1751)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                shape: data::type_::ValueShapeId(16),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("view source stopped"))),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostNeverFunctionId(0)),
                    ]),
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
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(1),
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
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
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
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::IntFunctionId(6),
                                            site: data::source::HostCallSite::from_static("function_views", "output", data::source::SourceSpan::new(416, 429)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                id: data::graph::ExternalLocalId(0),
                                                type_id: data::type_::ExternalTypeId(0),
                                            }),
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[]),
                                        },
                                    },
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
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(7),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("function_views", "<anonymous:4>", data::source::SourceSpan::new(1563, 1569)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(2)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 3,
                            return_: data::graph::IntLocalId(0),
                            body: ::core::marker::PhantomData,
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
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(2)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 23,
                            return_: data::graph::IntLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 24,
                            return_: data::graph::IntLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    float_functions: data::Storage::Static(&[]),
                    string_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 15,
                            return_: data::graph::StringLocalId(0),
                            body: ::core::marker::PhantomData,
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
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(16),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(16),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                shape: data::type_::ValueShapeId(16),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Concatenate {
                                                left: data::graph::StringLocalId(0),
                                                right: data::graph::StringLocalId(1),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::StringLocalId(2)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 19,
                            return_: data::graph::StringLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    bit_array_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 17,
                            return_: data::graph::BitArrayLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    utf_codepoint_functions: data::Storage::Static(&[]),
                    custom_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 4,
                            return_: data::graph::CustomLocal {
                                id: data::graph::CustomLocalId(0),
                                shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(1),
                                    shape_id: data::type_::CustomValueShapeId(0),
                                },
                            },
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 6,
                            return_: data::graph::CustomLocal {
                                id: data::graph::CustomLocalId(0),
                                shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(3),
                                    shape_id: data::type_::CustomValueShapeId(2),
                                },
                            },
                            body: ::core::marker::PhantomData,
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
                                                instructions: 0..0,
                                                terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                            },
                                        ]),
                                        params: data::Storage::Static(&[
                                            data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                        ]),
                                        instructions: data::Storage::Static(&[]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::TailCall {
                                            function: data::source::FunctionCallTarget {
                                                function: 8,
                                                site: data::source::HostCallSite::from_static("function_views", "input", data::source::SourceSpan::new(363, 376)),
                                            },
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            ]),
                                            transfer: data::graph::Transfer {
                                                families: data::Storage::Static(&[]),
                                            },
                                        },
                                    ]),
                                },
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 20,
                            return_: data::graph::ExternalLocal {
                                id: data::graph::ExternalLocalId(0),
                                type_id: data::type_::ExternalTypeId(0),
                            },
                            body: ::core::marker::PhantomData,
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
                            index: 5,
                            return_: data::graph::ExternalLocal {
                                id: data::graph::ExternalLocalId(0),
                                type_id: data::type_::ExternalTypeId(0),
                            },
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 7,
                            return_: data::graph::ExternalLocal {
                                id: data::graph::ExternalLocalId(0),
                                type_id: data::type_::ExternalTypeId(0),
                            },
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 12,
                            return_: data::graph::ExternalLocal {
                                id: data::graph::ExternalLocalId(0),
                                type_id: data::type_::ExternalTypeId(0),
                            },
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 13,
                            return_: data::graph::ExternalLocal {
                                id: data::graph::ExternalLocalId(0),
                                type_id: data::type_::ExternalTypeId(0),
                            },
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 14,
                            return_: data::graph::ExternalLocal {
                                id: data::graph::ExternalLocalId(0),
                                type_id: data::type_::ExternalTypeId(0),
                            },
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 22,
                            return_: data::graph::ExternalLocal {
                                id: data::graph::ExternalLocalId(0),
                                type_id: data::type_::ExternalTypeId(0),
                            },
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    bool_functions: data::Storage::Static(&[
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
                                            instructions: 0..13,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(4),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        shape_id: data::type_::CustomValueShapeId(2),
                                                    },
                                                }),
                                                pattern: data::graph::MatchPattern::Custom {
                                                    constructor: data::type_::CustomConstructorId {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        index: 1,
                                                    },
                                                    fields: data::Storage::Static(&[
                                                        data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                            elements: data::Storage::Static(&[
                                                                data::graph::MatchPattern::Custom {
                                                                    constructor: data::type_::CustomConstructorId {
                                                                        type_id: data::type_::CustomTypeId(3),
                                                                        index: 0,
                                                                    },
                                                                    fields: data::Storage::Static(&[
                                                                        data::graph::MatchPattern::Bind(data::graph::MatchPatternBinding {
                                                                            index: 0,
                                                                        }),
                                                                    ]),
                                                                },
                                                            ]),
                                                            tail: None,
                                                        }),
                                                    ]),
                                                },
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Binding(0),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::IntFunction {
                                                            local: data::graph::IntFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                            },
                                                        }),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                            id: data::graph::ExternalFunctionLocalId(0),
                                                            type_: data::type_::ExternalFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                },
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueShapeId(0),
                                                                ]),
                                                                return_: data::type_::ExternalTypeId(0),
                                                            },
                                                        })),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::IntFunction {
                                                            local: data::graph::IntFunctionLocalId(1),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                            },
                                                        }),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                            id: data::graph::ExternalFunctionLocalId(1),
                                                            type_: data::type_::ExternalFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                },
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueShapeId(0),
                                                                ]),
                                                                return_: data::type_::ExternalTypeId(0),
                                                            },
                                                        })),
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
                                                                family: data::graph::StorageFamily::Custom,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CustomList,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                length: 2,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::ExternalFunction,
                                                                length: 3,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 2,
                                                                        destination: 0,
                                                                    },
                                                                    data::graph::TransferStep {
                                                                        source: 2,
                                                                        destination: 1,
                                                                    },
                                                                ]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(29),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                            id: data::graph::CustomLocalId(4),
                                                            shape: data::type_::CustomValueShape {
                                                                type_id: data::type_::CustomTypeId(3),
                                                                shape_id: data::type_::CustomValueShapeId(2),
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
                                                                family: data::graph::StorageFamily::Custom,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 4,
                                                                        destination: 0,
                                                                    },
                                                                ]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CustomList,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::ExternalFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 0..5,
                                            instructions: 13..27,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::EqualInt {
                                                    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(4)),
                                                    right: data::graph::IntegerOperand::Immediate(42),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
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
                                                        data::graph::ParamLocal::IntFunction {
                                                            local: data::graph::IntFunctionLocalId(1),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                            },
                                                        },
                                                        data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                            id: data::graph::ExternalFunctionLocalId(2),
                                                            type_: data::type_::ExternalFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                },
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueShapeId(0),
                                                                ]),
                                                                return_: data::type_::ExternalTypeId(0),
                                                            },
                                                        }),
                                                        data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                            id: data::graph::ExternalFunctionLocalId(0),
                                                            type_: data::type_::ExternalFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                },
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueShapeId(0),
                                                                ]),
                                                                return_: data::type_::ExternalTypeId(0),
                                                            },
                                                        }),
                                                        data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::External(data::graph::ExternalFunctionFunctionLocal {
                                                            id: data::graph::ExternalFunctionFunctionLocalId(0),
                                                            type_: data::type_::FunctionFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[
                                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                        ]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                    })),
                                                                },
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueShapeId(0),
                                                                ]),
                                                                return_: data::type_::FunctionShape {
                                                                    shape_id: data::type_::ValueShapeId(3),
                                                                    type_: data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[
                                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                        ]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                    },
                                                                },
                                                            },
                                                        })),
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::StringFunction {
                                                            local: data::graph::StringFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::String,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                        },
                                                        data::graph::ParamLocal::BitArrayFunction {
                                                            local: data::graph::BitArrayFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::BitArray,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                            },
                                                        },
                                                        data::graph::ParamLocal::StringFunction {
                                                            local: data::graph::StringFunctionLocalId(1),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::String,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
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
                                                                family: data::graph::StorageFamily::BitArray,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::External,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::ExternalFunction,
                                                                length: 2,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 2,
                                                                        destination: 0,
                                                                    },
                                                                    data::graph::TransferStep {
                                                                        source: 2,
                                                                        destination: 1,
                                                                    },
                                                                ]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CoreFunctionFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(28),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BitArray,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::External,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::StringFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BitArrayFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::ExternalFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CoreFunctionFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::ExternalFunctionFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 5..14,
                                            instructions: 27..27,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::Equal {
                                                    left: data::graph::ParamLocal::IntFunction {
                                                        local: data::graph::IntFunctionLocalId(1),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        },
                                                    },
                                                    right: data::graph::ParamLocal::IntFunction {
                                                        local: data::graph::IntFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        },
                                                    },
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(3),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::IntFunction {
                                                            local: data::graph::IntFunctionLocalId(1),
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
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                },
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueShapeId(0),
                                                                ]),
                                                                return_: data::type_::ExternalTypeId(0),
                                                            },
                                                        }),
                                                        data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                            id: data::graph::ExternalFunctionLocalId(1),
                                                            type_: data::type_::ExternalFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                },
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueShapeId(0),
                                                                ]),
                                                                return_: data::type_::ExternalTypeId(0),
                                                            },
                                                        }),
                                                        data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::External(data::graph::ExternalFunctionFunctionLocal {
                                                            id: data::graph::ExternalFunctionFunctionLocalId(0),
                                                            type_: data::type_::FunctionFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[
                                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                        ]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                    })),
                                                                },
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueShapeId(0),
                                                                ]),
                                                                return_: data::type_::FunctionShape {
                                                                    shape_id: data::type_::ValueShapeId(3),
                                                                    type_: data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[
                                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                        ]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                    },
                                                                },
                                                            },
                                                        })),
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::StringFunction {
                                                            local: data::graph::StringFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::String,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                        },
                                                        data::graph::ParamLocal::BitArrayFunction {
                                                            local: data::graph::BitArrayFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::BitArray,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                            },
                                                        },
                                                        data::graph::ParamLocal::StringFunction {
                                                            local: data::graph::StringFunctionLocalId(1),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::String,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
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
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(26),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::StringFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BitArrayFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::ExternalFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::ExternalFunctionFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 14..22,
                                            instructions: 27..29,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::EqualInt {
                                                    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                                    right: data::graph::IntegerOperand::Immediate(42),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(4),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                            id: data::graph::ExternalFunctionLocalId(0),
                                                            type_: data::type_::ExternalFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                },
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueShapeId(0),
                                                                ]),
                                                                return_: data::type_::ExternalTypeId(0),
                                                            },
                                                        }),
                                                        data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                            id: data::graph::ExternalFunctionLocalId(1),
                                                            type_: data::type_::ExternalFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                },
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueShapeId(0),
                                                                ]),
                                                                return_: data::type_::ExternalTypeId(0),
                                                            },
                                                        }),
                                                        data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::External(data::graph::ExternalFunctionFunctionLocal {
                                                            id: data::graph::ExternalFunctionFunctionLocalId(0),
                                                            type_: data::type_::FunctionFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[
                                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                        ]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                    })),
                                                                },
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueShapeId(0),
                                                                ]),
                                                                return_: data::type_::FunctionShape {
                                                                    shape_id: data::type_::ValueShapeId(3),
                                                                    type_: data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[
                                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                        ]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                    },
                                                                },
                                                            },
                                                        })),
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::StringFunction {
                                                            local: data::graph::StringFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::String,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                        },
                                                        data::graph::ParamLocal::BitArrayFunction {
                                                            local: data::graph::BitArrayFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::BitArray,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                            },
                                                        },
                                                        data::graph::ParamLocal::StringFunction {
                                                            local: data::graph::StringFunctionLocalId(1),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::String,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
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
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(24),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::StringFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BitArrayFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::ExternalFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::ExternalFunctionFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 22..29,
                                            instructions: 29..33,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::EqualInt {
                                                    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                                    right: data::graph::IntegerOperand::Immediate(42),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(5),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                            id: data::graph::ExternalFunctionLocalId(1),
                                                            type_: data::type_::ExternalFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                },
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueShapeId(0),
                                                                ]),
                                                                return_: data::type_::ExternalTypeId(0),
                                                            },
                                                        }),
                                                        data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::External(data::graph::ExternalFunctionFunctionLocal {
                                                            id: data::graph::ExternalFunctionFunctionLocalId(0),
                                                            type_: data::type_::FunctionFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[
                                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                        ]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                    })),
                                                                },
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueShapeId(0),
                                                                ]),
                                                                return_: data::type_::FunctionShape {
                                                                    shape_id: data::type_::ValueShapeId(3),
                                                                    type_: data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[
                                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                        ]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                    },
                                                                },
                                                            },
                                                        })),
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::StringFunction {
                                                            local: data::graph::StringFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::String,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                        },
                                                        data::graph::ParamLocal::BitArrayFunction {
                                                            local: data::graph::BitArrayFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::BitArray,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                            },
                                                        },
                                                        data::graph::ParamLocal::StringFunction {
                                                            local: data::graph::StringFunctionLocalId(1),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::String,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
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
                                                                family: data::graph::StorageFamily::External,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::ExternalFunction,
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
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(22),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::External,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::StringFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BitArrayFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::ExternalFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::ExternalFunctionFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 29..35,
                                            instructions: 33..37,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::EqualInt {
                                                    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                                    right: data::graph::IntegerOperand::Immediate(42),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(6),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::External(data::graph::ExternalFunctionFunctionLocal {
                                                            id: data::graph::ExternalFunctionFunctionLocalId(0),
                                                            type_: data::type_::FunctionFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[
                                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                        ]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                    })),
                                                                },
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueShapeId(0),
                                                                ]),
                                                                return_: data::type_::FunctionShape {
                                                                    shape_id: data::type_::ValueShapeId(3),
                                                                    type_: data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[
                                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                        ]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                    },
                                                                },
                                                            },
                                                        })),
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::StringFunction {
                                                            local: data::graph::StringFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::String,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                        },
                                                        data::graph::ParamLocal::BitArrayFunction {
                                                            local: data::graph::BitArrayFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::BitArray,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                            },
                                                        },
                                                        data::graph::ParamLocal::StringFunction {
                                                            local: data::graph::StringFunctionLocalId(1),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::String,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
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
                                                                family: data::graph::StorageFamily::External,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::ExternalFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(20),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::External,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::StringFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BitArrayFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::ExternalFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::ExternalFunctionFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 35..40,
                                            instructions: 37..44,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::EqualInt {
                                                    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
                                                    right: data::graph::IntegerOperand::Immediate(42),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(7),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::StringFunction {
                                                            local: data::graph::StringFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::String,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                        },
                                                        data::graph::ParamLocal::BitArrayFunction {
                                                            local: data::graph::BitArrayFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::BitArray,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                            },
                                                        },
                                                        data::graph::ParamLocal::StringFunction {
                                                            local: data::graph::StringFunctionLocalId(1),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::String,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
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
                                                                family: data::graph::StorageFamily::ExternalFunctionFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(18),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::External,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::StringFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BitArrayFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::ExternalFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::ExternalFunctionFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 40..44,
                                            instructions: 44..56,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::Equal {
                                                    left: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                                    right: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(2)),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(8),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::StringFunction {
                                                            local: data::graph::StringFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::String,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                        },
                                                        data::graph::ParamLocal::StringFunction {
                                                            local: data::graph::StringFunctionLocalId(1),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::String,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
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
                                                                family: data::graph::StorageFamily::BitArray,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BitArrayFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(16),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BitArray,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::StringFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BitArrayFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 44..47,
                                            instructions: 56..56,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::Equal {
                                                    left: data::graph::ParamLocal::StringFunction {
                                                        local: data::graph::StringFunctionLocalId(1),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::String,
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::String),
                                                        },
                                                    },
                                                    right: data::graph::ParamLocal::StringFunction {
                                                        local: data::graph::StringFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::String,
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::String),
                                                        },
                                                    },
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(9),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::StringFunction {
                                                            local: data::graph::StringFunctionLocalId(1),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::String,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                        },
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::StringFunction,
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
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(14),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::StringFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 47..49,
                                            instructions: 56..58,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::Equal {
                                                    left: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    right: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(10),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::StringFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(12),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::StringFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 49..49,
                                            instructions: 58..59,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(11),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 49..50,
                                            instructions: 59..59,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 50..50,
                                            instructions: 59..59,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(13),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 50..50,
                                            instructions: 59..60,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(11),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 50..50,
                                            instructions: 60..60,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(15),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 50..50,
                                            instructions: 60..60,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(13),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 50..50,
                                            instructions: 60..60,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(17),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 50..50,
                                            instructions: 60..60,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(15),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 50..50,
                                            instructions: 60..60,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(19),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 50..50,
                                            instructions: 60..60,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(17),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 50..50,
                                            instructions: 60..60,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(21),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 50..50,
                                            instructions: 60..60,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(19),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 50..50,
                                            instructions: 60..60,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(23),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 50..50,
                                            instructions: 60..60,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(21),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 50..50,
                                            instructions: 60..60,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(25),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 50..50,
                                            instructions: 60..60,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(23),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 50..50,
                                            instructions: 60..60,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(27),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 50..50,
                                            instructions: 60..60,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(25),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 50..50,
                                            instructions: 60..60,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(27),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 50..51,
                                            instructions: 60..60,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        shape_id: data::type_::CustomValueShapeId(2),
                                                    },
                                                }),
                                                message: None,
                                                site: data::source::PanicSite::from_static("function_views", "run", data::source::SourceSpan::new(789, 799)),
                                                pattern_span: data::source::SourceSpan::new(800, 820),
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                id: data::graph::ExternalFunctionLocalId(0),
                                                type_: data::type_::ExternalFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::ExternalTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(3),
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
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                id: data::graph::ExternalFunctionLocalId(1),
                                                type_: data::type_::ExternalFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::ExternalTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(1),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                id: data::graph::ExternalFunctionLocalId(2),
                                                type_: data::type_::ExternalFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::ExternalTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(3),
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
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(1),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                id: data::graph::ExternalFunctionLocalId(0),
                                                type_: data::type_::ExternalFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::ExternalTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                id: data::graph::ExternalFunctionLocalId(1),
                                                type_: data::type_::ExternalFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::ExternalTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::External(data::graph::ExternalFunctionFunctionLocal {
                                                id: data::graph::ExternalFunctionFunctionLocalId(0),
                                                type_: data::type_::FunctionFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                        })),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::FunctionShape {
                                                        shape_id: data::type_::ValueShapeId(3),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                        },
                                                    },
                                                },
                                            })),
                                            shape: data::type_::ValueShapeId(14),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(16),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::StringFunction {
                                                local: data::graph::StringFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArrayFunction {
                                                local: data::graph::BitArrayFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::BitArray,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(18),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::StringFunction {
                                                local: data::graph::StringFunctionLocalId(1),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(17),
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
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                id: data::graph::ExternalFunctionLocalId(0),
                                                type_: data::type_::ExternalFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::ExternalTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                id: data::graph::ExternalFunctionLocalId(1),
                                                type_: data::type_::ExternalFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::ExternalTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::External(data::graph::ExternalFunctionFunctionLocal {
                                                id: data::graph::ExternalFunctionFunctionLocalId(0),
                                                type_: data::type_::FunctionFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                        })),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::FunctionShape {
                                                        shape_id: data::type_::ValueShapeId(3),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                        },
                                                    },
                                                },
                                            })),
                                            shape: data::type_::ValueShapeId(14),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(16),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::StringFunction {
                                                local: data::graph::StringFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArrayFunction {
                                                local: data::graph::BitArrayFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::BitArray,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(18),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::StringFunction {
                                                local: data::graph::StringFunctionLocalId(1),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                id: data::graph::ExternalFunctionLocalId(0),
                                                type_: data::type_::ExternalFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::ExternalTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                id: data::graph::ExternalFunctionLocalId(1),
                                                type_: data::type_::ExternalFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::ExternalTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::External(data::graph::ExternalFunctionFunctionLocal {
                                                id: data::graph::ExternalFunctionFunctionLocalId(0),
                                                type_: data::type_::FunctionFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                        })),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::FunctionShape {
                                                        shape_id: data::type_::ValueShapeId(3),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                        },
                                                    },
                                                },
                                            })),
                                            shape: data::type_::ValueShapeId(14),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(16),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::StringFunction {
                                                local: data::graph::StringFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArrayFunction {
                                                local: data::graph::BitArrayFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::BitArray,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(18),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::StringFunction {
                                                local: data::graph::StringFunctionLocalId(1),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                id: data::graph::ExternalFunctionLocalId(0),
                                                type_: data::type_::ExternalFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::ExternalTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::External(data::graph::ExternalFunctionFunctionLocal {
                                                id: data::graph::ExternalFunctionFunctionLocalId(0),
                                                type_: data::type_::FunctionFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                        })),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::FunctionShape {
                                                        shape_id: data::type_::ValueShapeId(3),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                        },
                                                    },
                                                },
                                            })),
                                            shape: data::type_::ValueShapeId(14),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(16),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::StringFunction {
                                                local: data::graph::StringFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArrayFunction {
                                                local: data::graph::BitArrayFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::BitArray,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(18),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::StringFunction {
                                                local: data::graph::StringFunctionLocalId(1),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::External(data::graph::ExternalFunctionFunctionLocal {
                                                id: data::graph::ExternalFunctionFunctionLocalId(0),
                                                type_: data::type_::FunctionFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                        })),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::FunctionShape {
                                                        shape_id: data::type_::ValueShapeId(3),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                        },
                                                    },
                                                },
                                            })),
                                            shape: data::type_::ValueShapeId(14),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(16),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::StringFunction {
                                                local: data::graph::StringFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArrayFunction {
                                                local: data::graph::BitArrayFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::BitArray,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(18),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::StringFunction {
                                                local: data::graph::StringFunctionLocalId(1),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(16),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::StringFunction {
                                                local: data::graph::StringFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArrayFunction {
                                                local: data::graph::BitArrayFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::BitArray,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(18),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::StringFunction {
                                                local: data::graph::StringFunctionLocalId(1),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(16),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::StringFunction {
                                                local: data::graph::StringFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::StringFunction {
                                                local: data::graph::StringFunctionLocalId(1),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(16),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::StringFunction {
                                                local: data::graph::StringFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(19),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    shape_id: data::type_::CustomValueShapeId(2),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(1),
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
                                                shape: data::type_::ValueShapeId(2),
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
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                    id: data::graph::ExternalFunctionLocalId(0),
                                                    type_: data::type_::ExternalFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                        },
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueShapeId(0),
                                                        ]),
                                                        return_: data::type_::ExternalTypeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::ExternalFunction(data::graph::ExternalFunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                },
                                                family: data::function::FunctionReturnFamily::External,
                                                kind: data::graph::ExternalFunctionInstructionKind::Call {
                                                    function: data::graph::ExternalFunctionCallTarget::Function(data::function::ExternalFunctionFunctionId {
                                                        index: 0,
                                                        type_: data::type_::ExternalFunctionType {
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                            },
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueShapeId(0),
                                                            ]),
                                                            return_: data::type_::ExternalTypeId(0),
                                                        },
                                                    }),
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
                                                    site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(547, 563)),
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
                                                shape: data::type_::ValueShapeId(2),
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
                                                        data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                            id: data::graph::ExternalFunctionLocalId(0),
                                                            type_: data::type_::ExternalFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                },
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueShapeId(0),
                                                                ]),
                                                                return_: data::type_::ExternalTypeId(0),
                                                            },
                                                        }),
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(597, 609)),
                                                },
                                            }),
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
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                                family: data::function::FunctionReturnFamily::Int,
                                                kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(1))),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(3),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(8),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::ParamLocal::IntFunction {
                                                        local: data::graph::IntFunctionLocalId(2),
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
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(9),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Call {
                                                function: data::function::CustomFunctionId {
                                                    index: 0,
                                                    return_shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(3),
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(651, 677)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                    id: data::graph::ExternalFunctionLocalId(1),
                                                    type_: data::type_::ExternalFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                        },
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueShapeId(0),
                                                        ]),
                                                        return_: data::type_::ExternalTypeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                },
                                                family: data::function::FunctionReturnFamily::External,
                                                kind: data::graph::FunctionInstructionKind::CustomField {
                                                    source: data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(1),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
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
                                                    local: data::graph::IntFunctionLocalId(3),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                                family: data::function::FunctionReturnFamily::Int,
                                                kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(1))),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(2),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(4),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(10),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::ParamLocal::IntFunction {
                                                        local: data::graph::IntFunctionLocalId(3),
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
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                    local: data::graph::CustomListLocalId(0),
                                                    type_id: data::type_::CustomListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item_type: data::type_::CustomTypeId(2),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(11),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::CustomTypeId(2),
                                            }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                                data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(2),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(4),
                                                    },
                                                },
                                            ])))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(3),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(5),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(12),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                        local: data::graph::CustomListLocalId(0),
                                                        type_id: data::type_::CustomListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::CustomTypeId(2),
                                                        },
                                                    }),
                                                ]),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(4),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        shape_id: data::type_::CustomValueShapeId(2),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Call {
                                                function: data::function::CustomFunctionId {
                                                    index: 1,
                                                    return_shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        shape_id: data::type_::CustomValueShapeId(2),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(3),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(2),
                                                            shape_id: data::type_::CustomValueShapeId(5),
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(753, 786)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                                                    id: data::graph::CoreFunctionFunctionLocalId(0),
                                                    type_: data::type_::FunctionFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                            })),
                                                        },
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueShapeId(1),
                                                        ]),
                                                        return_: data::type_::FunctionShape {
                                                            shape_id: data::type_::ValueShapeId(2),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                            },
                                                        },
                                                    },
                                                })),
                                                shape: data::type_::ValueShapeId(13),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    })),
                                                },
                                                family: data::function::FunctionReturnFamily::Function,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Function(data::function::ProfiledFunctionFunctionId::Int(data::function::IntFunctionFunctionId(1))),
                                                    captures: data::Storage::Static(&[]),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::External(data::graph::ExternalFunctionFunctionLocal {
                                                    id: data::graph::ExternalFunctionFunctionLocalId(0),
                                                    type_: data::type_::FunctionFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                            })),
                                                        },
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueShapeId(0),
                                                        ]),
                                                        return_: data::type_::FunctionShape {
                                                            shape_id: data::type_::ValueShapeId(3),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                            },
                                                        },
                                                    },
                                                })),
                                                shape: data::type_::ValueShapeId(14),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                    })),
                                                },
                                                family: data::function::FunctionReturnFamily::Function,
                                                kind: data::graph::FunctionInstructionKind::Call {
                                                    function: data::function::ProfiledFunctionFunctionId::Function(data::function::FunctionFunctionFunctionId {
                                                        index: 0,
                                                        type_: data::type_::FunctionFunctionType {
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                })),
                                                            },
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueShapeId(0),
                                                            ]),
                                                            return_: data::type_::FunctionShape {
                                                                shape_id: data::type_::ValueShapeId(3),
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                },
                                                            },
                                                        },
                                                    }),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                                                            id: data::graph::CoreFunctionFunctionLocalId(0),
                                                            type_: data::type_::FunctionFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::Int,
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[
                                                                            data::type_::ValueType::Int,
                                                                        ]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                                    })),
                                                                },
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueShapeId(1),
                                                                ]),
                                                                return_: data::type_::FunctionShape {
                                                                    shape_id: data::type_::ValueShapeId(2),
                                                                    type_: data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[
                                                                            data::type_::ValueType::Int,
                                                                        ]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                                    },
                                                                },
                                                            },
                                                        })),
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(883, 926)),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    255,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::NoSign,
                                                digits: data::Storage::Static(&[]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    195,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                shape: data::type_::ValueShapeId(15),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::BitArray(data::graph::BitArrayInstruction::Value(data::Storage::Static(&[
                                                data::graph::BitArraySegment::Int {
                                                    value: data::graph::IntLocalId(0),
                                                    bit_size: 8,
                                                    endianness: data::graph::Endianness::Big,
                                                },
                                                data::graph::BitArraySegment::Int {
                                                    value: data::graph::IntLocalId(1),
                                                    bit_size: 8,
                                                    endianness: data::graph::Endianness::Big,
                                                },
                                                data::graph::BitArraySegment::Int {
                                                    value: data::graph::IntLocalId(2),
                                                    bit_size: 8,
                                                    endianness: data::graph::Endianness::Big,
                                                },
                                            ]))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                shape: data::type_::ValueShapeId(16),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                                function: data::function::StringFunctionId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(947, 970)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::StringFunction {
                                                    local: data::graph::StringFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::String,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::String),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                                family: data::function::FunctionReturnFamily::String,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::String(data::function::StringFunctionId(1)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::String {
                                                            target: data::graph::StringLocalId(1),
                                                            source: data::graph::StringLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BitArrayFunction {
                                                    local: data::graph::BitArrayFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::BitArray,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(18),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::BitArray,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                },
                                                family: data::function::FunctionReturnFamily::BitArray,
                                                kind: data::graph::FunctionInstructionKind::Call {
                                                    function: data::function::ProfiledFunctionFunctionId::BitArray(data::function::BitArrayFunctionFunctionId(0)),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::StringFunction {
                                                            local: data::graph::StringFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::String,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                        },
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(1057, 1071)),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::StringFunction {
                                                    local: data::graph::StringFunctionLocalId(1),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::String,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::String),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                                family: data::function::FunctionReturnFamily::String,
                                                kind: data::graph::FunctionInstructionKind::Call {
                                                    function: data::function::ProfiledFunctionFunctionId::String(data::function::StringFunctionFunctionId(0)),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BitArrayFunction {
                                                            local: data::graph::BitArrayFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::BitArray,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                            },
                                                        },
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(1117, 1134)),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    40,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                    id: data::graph::ExternalLocalId(0),
                                                    type_id: data::type_::ExternalTypeId(0),
                                                }),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::External(data::graph::ExternalInstruction::Call {
                                                function: data::function::ExternalFunctionId {
                                                    index: 0,
                                                    return_type: data::type_::ExternalTypeId(0),
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(1149, 1158)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                    id: data::graph::ExternalLocalId(1),
                                                    type_id: data::type_::ExternalTypeId(0),
                                                }),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::External(data::graph::ExternalInstruction::FunctionCall {
                                                function: data::graph::ExternalFunctionLocal {
                                                    id: data::graph::ExternalFunctionLocalId(1),
                                                    type_: data::type_::ExternalFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                        },
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueShapeId(0),
                                                        ]),
                                                        return_: data::type_::ExternalTypeId(0),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                        id: data::graph::ExternalLocalId(0),
                                                        type_id: data::type_::ExternalTypeId(0),
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(1144, 1159)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                        id: data::graph::ExternalLocalId(1),
                                                        type_id: data::type_::ExternalTypeId(0),
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(1137, 1160)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    40,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::FunctionCall {
                                                function: data::graph::IntFunctionLocalId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(1198, 1210)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    41,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                    id: data::graph::ExternalLocalId(0),
                                                    type_id: data::type_::ExternalTypeId(0),
                                                }),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::External(data::graph::ExternalInstruction::Call {
                                                function: data::function::ExternalFunctionId {
                                                    index: 0,
                                                    return_type: data::type_::ExternalTypeId(0),
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(1238, 1247)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                    id: data::graph::ExternalLocalId(1),
                                                    type_id: data::type_::ExternalTypeId(0),
                                                }),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::External(data::graph::ExternalInstruction::FunctionCall {
                                                function: data::graph::ExternalFunctionLocal {
                                                    id: data::graph::ExternalFunctionLocalId(0),
                                                    type_: data::type_::ExternalFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                        },
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueShapeId(0),
                                                        ]),
                                                        return_: data::type_::ExternalTypeId(0),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                        id: data::graph::ExternalLocalId(0),
                                                        type_id: data::type_::ExternalTypeId(0),
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(1229, 1248)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                        id: data::graph::ExternalLocalId(1),
                                                        type_id: data::type_::ExternalTypeId(0),
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(1222, 1249)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    41,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                    id: data::graph::ExternalLocalId(0),
                                                    type_id: data::type_::ExternalTypeId(0),
                                                }),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::External(data::graph::ExternalInstruction::Call {
                                                function: data::function::ExternalFunctionId {
                                                    index: 0,
                                                    return_type: data::type_::ExternalTypeId(0),
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(1273, 1282)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                    id: data::graph::ExternalLocalId(1),
                                                    type_id: data::type_::ExternalTypeId(0),
                                                }),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::External(data::graph::ExternalInstruction::FunctionCall {
                                                function: data::graph::ExternalFunctionLocal {
                                                    id: data::graph::ExternalFunctionLocalId(0),
                                                    type_: data::type_::ExternalFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                        },
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueShapeId(0),
                                                        ]),
                                                        return_: data::type_::ExternalTypeId(0),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                        id: data::graph::ExternalLocalId(0),
                                                        type_id: data::type_::ExternalTypeId(0),
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(1268, 1283)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                        id: data::graph::ExternalLocalId(1),
                                                        type_id: data::type_::ExternalTypeId(0),
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(1261, 1284)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    20,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                    id: data::graph::ExternalLocalId(0),
                                                    type_id: data::type_::ExternalTypeId(0),
                                                }),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::External(data::graph::ExternalInstruction::Call {
                                                function: data::function::ExternalFunctionId {
                                                    index: 0,
                                                    return_type: data::type_::ExternalTypeId(0),
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(1310, 1319)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                    id: data::graph::ExternalFunctionLocalId(0),
                                                    type_: data::type_::ExternalFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                        },
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueShapeId(0),
                                                        ]),
                                                        return_: data::type_::ExternalTypeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::ExternalFunction(data::graph::ExternalFunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                },
                                                family: data::function::FunctionReturnFamily::External,
                                                kind: data::graph::ExternalFunctionInstructionKind::FunctionCall {
                                                    function: data::graph::ExternalFunctionFunctionLocal {
                                                        id: data::graph::ExternalFunctionFunctionLocalId(0),
                                                        type_: data::type_::FunctionFunctionType {
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                })),
                                                            },
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueShapeId(0),
                                                            ]),
                                                            return_: data::type_::FunctionShape {
                                                                shape_id: data::type_::ValueShapeId(3),
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                            id: data::graph::ExternalLocalId(0),
                                                            type_id: data::type_::ExternalTypeId(0),
                                                        }),
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(1303, 1320)),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    22,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                    id: data::graph::ExternalLocalId(1),
                                                    type_id: data::type_::ExternalTypeId(0),
                                                }),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::External(data::graph::ExternalInstruction::Call {
                                                function: data::function::ExternalFunctionId {
                                                    index: 0,
                                                    return_type: data::type_::ExternalTypeId(0),
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(1321, 1330)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                    id: data::graph::ExternalLocalId(2),
                                                    type_id: data::type_::ExternalTypeId(0),
                                                }),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::External(data::graph::ExternalInstruction::FunctionCall {
                                                function: data::graph::ExternalFunctionLocal {
                                                    id: data::graph::ExternalFunctionLocalId(0),
                                                    type_: data::type_::ExternalFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                        },
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueShapeId(0),
                                                        ]),
                                                        return_: data::type_::ExternalTypeId(0),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                        id: data::graph::ExternalLocalId(1),
                                                        type_id: data::type_::ExternalTypeId(0),
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(1303, 1331)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                        id: data::graph::ExternalLocalId(2),
                                                        type_id: data::type_::ExternalTypeId(0),
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(1296, 1332)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    255,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::NoSign,
                                                digits: data::Storage::Static(&[]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    195,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                shape: data::type_::ValueShapeId(15),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::BitArray(data::graph::BitArrayInstruction::Value(data::Storage::Static(&[
                                                data::graph::BitArraySegment::Int {
                                                    value: data::graph::IntLocalId(0),
                                                    bit_size: 8,
                                                    endianness: data::graph::Endianness::Big,
                                                },
                                                data::graph::BitArraySegment::Int {
                                                    value: data::graph::IntLocalId(1),
                                                    bit_size: 8,
                                                    endianness: data::graph::Endianness::Big,
                                                },
                                                data::graph::BitArraySegment::Int {
                                                    value: data::graph::IntLocalId(2),
                                                    bit_size: 8,
                                                    endianness: data::graph::Endianness::Big,
                                                },
                                            ]))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                                shape: data::type_::ValueShapeId(15),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::BitArray(data::graph::BitArrayInstruction::FunctionCall {
                                                function: data::graph::BitArrayFunctionLocalId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(1344, 1370)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    255,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::NoSign,
                                                digits: data::Storage::Static(&[]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    195,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    255,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::NoSign,
                                                digits: data::Storage::Static(&[]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    195,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(2)),
                                                shape: data::type_::ValueShapeId(15),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::BitArray(data::graph::BitArrayInstruction::Value(data::Storage::Static(&[
                                                data::graph::BitArraySegment::Int {
                                                    value: data::graph::IntLocalId(3),
                                                    bit_size: 8,
                                                    endianness: data::graph::Endianness::Big,
                                                },
                                                data::graph::BitArraySegment::Int {
                                                    value: data::graph::IntLocalId(4),
                                                    bit_size: 8,
                                                    endianness: data::graph::Endianness::Big,
                                                },
                                                data::graph::BitArraySegment::Int {
                                                    value: data::graph::IntLocalId(5),
                                                    bit_size: 8,
                                                    endianness: data::graph::Endianness::Big,
                                                },
                                                data::graph::BitArraySegment::Int {
                                                    value: data::graph::IntLocalId(6),
                                                    bit_size: 8,
                                                    endianness: data::graph::Endianness::Big,
                                                },
                                                data::graph::BitArraySegment::Int {
                                                    value: data::graph::IntLocalId(7),
                                                    bit_size: 8,
                                                    endianness: data::graph::Endianness::Big,
                                                },
                                                data::graph::BitArraySegment::Int {
                                                    value: data::graph::IntLocalId(8),
                                                    bit_size: 8,
                                                    endianness: data::graph::Endianness::Big,
                                                },
                                            ]))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(16),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::FunctionCall {
                                                function: data::graph::StringFunctionLocalId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(1438, 1457)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                shape: data::type_::ValueShapeId(16),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Concatenate {
                                                left: data::graph::StringLocalId(0),
                                                right: data::graph::StringLocalId(0),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(19),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(19),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(false)),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
                                ]),
                            },
                        })),
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
                                            instructions: 0..7,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[]),
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
                                                shape: data::type_::ValueShapeId(2),
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
                                                    captures: data::Storage::Static(&[]),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                    id: data::graph::ExternalFunctionLocalId(0),
                                                    type_: data::type_::ExternalFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                        },
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueShapeId(0),
                                                        ]),
                                                        return_: data::type_::ExternalTypeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::ExternalFunction(data::graph::ExternalFunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                },
                                                family: data::function::FunctionReturnFamily::External,
                                                kind: data::graph::ExternalFunctionInstructionKind::Call {
                                                    function: data::graph::ExternalFunctionCallTarget::Function(data::function::ExternalFunctionFunctionId {
                                                        index: 0,
                                                        type_: data::type_::ExternalFunctionType {
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                            },
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueShapeId(0),
                                                            ]),
                                                            return_: data::type_::ExternalTypeId(0),
                                                        },
                                                    }),
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
                                                    site: data::source::HostCallSite::from_static("function_views", "invalid_input", data::source::SourceSpan::new(1543, 1576)),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                shape: data::type_::ValueShapeId(16),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("wrong"))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                    id: data::graph::ExternalLocalId(0),
                                                    type_id: data::type_::ExternalTypeId(0),
                                                }),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::External(data::graph::ExternalInstruction::Call {
                                                function: data::function::ExternalFunctionId {
                                                    index: 1,
                                                    return_type: data::type_::ExternalTypeId(0),
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "invalid_input", data::source::SourceSpan::new(1599, 1614)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                    id: data::graph::ExternalLocalId(1),
                                                    type_id: data::type_::ExternalTypeId(0),
                                                }),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::External(data::graph::ExternalInstruction::FunctionCall {
                                                function: data::graph::ExternalFunctionLocal {
                                                    id: data::graph::ExternalFunctionLocalId(0),
                                                    type_: data::type_::ExternalFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                        },
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueShapeId(0),
                                                        ]),
                                                        return_: data::type_::ExternalTypeId(0),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                        id: data::graph::ExternalLocalId(0),
                                                        type_id: data::type_::ExternalTypeId(0),
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "invalid_input", data::source::SourceSpan::new(1624, 1635)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                        id: data::graph::ExternalLocalId(1),
                                                        type_id: data::type_::ExternalTypeId(0),
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "invalid_input", data::source::SourceSpan::new(1617, 1636)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(19),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::EqualInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(42),
                                            })),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
                                ]),
                            },
                        })),
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
                                            instructions: 0..4,
                                            terminator: data::graph::Terminator::NeverCall(data::graph::NeverCall {
                                                function: data::graph::NeverCallTarget::Value(data::graph::NeverFunctionLocal {
                                                    id: data::graph::NeverFunctionLocalId(1),
                                                    type_: data::type_::GenericFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(4))),
                                                        },
                                                        shape: data::type_::FunctionShape {
                                                            shape_id: data::type_::ValueShapeId(21),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(4))),
                                                            },
                                                        },
                                                    },
                                                }),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                        id: data::graph::ExternalLocalId(0),
                                                        type_id: data::type_::ExternalTypeId(0),
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
                                                            family: data::graph::StorageFamily::NeverFunction,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                                site: data::source::HostCallSite::from_static("function_views", "stopped", data::source::SourceSpan::new(1806, 1821)),
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::NeverFunction(data::graph::NeverFunctionLocal {
                                                    id: data::graph::NeverFunctionLocalId(0),
                                                    type_: data::type_::GenericFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                        },
                                                        shape: data::type_::FunctionShape {
                                                            shape_id: data::type_::ValueShapeId(23),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                            },
                                                        },
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(23),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                },
                                                family: data::function::FunctionReturnFamily::Never,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Never(data::function::NeverFunctionId(0)),
                                                    captures: data::Storage::Static(&[]),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::NeverFunction(data::graph::NeverFunctionLocal {
                                                    id: data::graph::NeverFunctionLocalId(1),
                                                    type_: data::type_::GenericFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(4))),
                                                        },
                                                        shape: data::type_::FunctionShape {
                                                            shape_id: data::type_::ValueShapeId(21),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(4))),
                                                            },
                                                        },
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(21),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(4))),
                                                },
                                                family: data::function::FunctionReturnFamily::Never,
                                                kind: data::graph::FunctionInstructionKind::Call {
                                                    function: data::function::ProfiledFunctionFunctionId::Never(data::function::NeverFunctionFunctionId {
                                                        index: 0,
                                                        type_: data::type_::GenericFunctionType {
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(4))),
                                                            },
                                                            shape: data::type_::FunctionShape {
                                                                shape_id: data::type_::ValueShapeId(21),
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(4))),
                                                                },
                                                            },
                                                        },
                                                    }),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::NeverFunction(data::graph::NeverFunctionLocal {
                                                            id: data::graph::NeverFunctionLocalId(0),
                                                            type_: data::type_::GenericFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::Int,
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                                },
                                                                shape: data::type_::FunctionShape {
                                                                    shape_id: data::type_::ValueShapeId(23),
                                                                    type_: data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[
                                                                            data::type_::ValueType::Int,
                                                                        ]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                                    },
                                                                },
                                                            },
                                                        }),
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("function_views", "stopped", data::source::SourceSpan::new(1711, 1795)),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    42,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                    id: data::graph::ExternalLocalId(0),
                                                    type_id: data::type_::ExternalTypeId(0),
                                                }),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::External(data::graph::ExternalInstruction::Call {
                                                function: data::function::ExternalFunctionId {
                                                    index: 0,
                                                    return_type: data::type_::ExternalTypeId(0),
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_views", "stopped", data::source::SourceSpan::new(1811, 1820)),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[]),
                            },
                        })),
                    ]),
                    nil_functions: data::Storage::Static(&[]),
                    tuple_functions: data::Storage::Static(&[]),
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
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 2,
                            return_: data::graph::IntFunctionLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 1,
                            },
                            body: data::function::TypedFunctionBody {
                                _shape: data::type_::FunctionShape {
                                    shape_id: data::type_::ValueShapeId(2),
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
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(1),
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
                                                    shape: data::type_::ValueShapeId(2),
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
                                        data::function::FunctionExit::Return(data::graph::IntFunctionLocalId(0)),
                                    ]),
                                },
                            },
                        })),
                    ]),
                    float_function_functions: data::Storage::Static(&[]),
                    string_function_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 18,
                            return_: data::graph::StringFunctionLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    bit_array_function_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 16,
                            return_: data::graph::BitArrayFunctionLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    utf_codepoint_function_functions: data::Storage::Static(&[]),
                    custom_function_functions: data::Storage::Static(&[]),
                    external_function_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 0,
                            return_: data::graph::ExternalFunctionLocal {
                                id: data::graph::ExternalFunctionLocalId(0),
                                type_: data::type_::ExternalFunctionType {
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                    },
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueShapeId(0),
                                    ]),
                                    return_: data::type_::ExternalTypeId(0),
                                },
                            },
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 9,
                            return_: data::graph::ExternalFunctionLocal {
                                id: data::graph::ExternalFunctionLocalId(0),
                                type_: data::type_::ExternalFunctionType {
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                    },
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueShapeId(0),
                                    ]),
                                    return_: data::type_::ExternalTypeId(0),
                                },
                            },
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 10,
                            return_: data::graph::ExternalFunctionLocal {
                                id: data::graph::ExternalFunctionLocalId(0),
                                type_: data::type_::ExternalFunctionType {
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                    },
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueShapeId(0),
                                    ]),
                                    return_: data::type_::ExternalTypeId(0),
                                },
                            },
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 11,
                            return_: data::graph::ExternalFunctionLocal {
                                id: data::graph::ExternalFunctionLocalId(0),
                                type_: data::type_::ExternalFunctionType {
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                    },
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueShapeId(0),
                                    ]),
                                    return_: data::type_::ExternalTypeId(0),
                                },
                            },
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    bool_function_functions: data::Storage::Static(&[]),
                    nil_function_functions: data::Storage::Static(&[]),
                    tuple_function_functions: data::Storage::Static(&[]),
                    generic_function_functions: data::Storage::Static(&[]),
                    never_function_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 21,
                            return_: data::graph::NeverFunctionLocal {
                                id: data::graph::NeverFunctionLocalId(0),
                                type_: data::type_::GenericFunctionType {
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(4))),
                                    },
                                    shape: data::type_::FunctionShape {
                                        shape_id: data::type_::ValueShapeId(21),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(4))),
                                        },
                                    },
                                },
                            },
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
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
                    function_function_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 8,
                            return_: data::graph::FunctionFunctionLocal::External(data::graph::ExternalFunctionFunctionLocal {
                                id: data::graph::ExternalFunctionFunctionLocalId(0),
                                type_: data::type_::FunctionFunctionType {
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                        })),
                                    },
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueShapeId(0),
                                    ]),
                                    return_: data::type_::FunctionShape {
                                        shape_id: data::type_::ValueShapeId(3),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                        },
                                    },
                                },
                            }),
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                },
            },
            compiled: {
                const CALL_GROUP_0: [data::compiled::calls::CallStart; 3] = {
                    use data::compiled::calls::{CallExecution, CallInputs, CallInteger, CallOps, CallOutput, CallProgress, CallStorage, CallValues};
                    enum FunctionState {
                        Int0Point0 { int0: i128, int1: i128 },
                        Int0Point1 { int0: i128, int1: i128, int2: i128 },
                        Int1Point0 { int0: i128 },
                        Int1Point1 { int0: i128, int1: i128 },
                        Int5Point0 { int0: i128, int1: i128 },
                        Int5Point1 { int0: i128, int1: i128, int2: i128 },
                    }
                    enum IntReturn {
                    }
                    impl IntReturn {
                        fn site(&self) -> data::source::HostCallSite {
                            match *self {
                            }
                        }
                        fn small(self, result: i128) -> FunctionState {
                            let _ = result;
                            match self {
                            }
                        }
                        fn resume(self, result: CallInteger) -> FunctionState {
                            if let Some(result) = result.small() {
                                return self.small(result);
                            }
                            match self {
                            }
                        }
                    }
                    #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                    enum FunctionStep {
                        Yield(FunctionState),
                        Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                        Int { value: i128 },
                    }
                    struct FunctionExecution {
                        active: Option<FunctionState>,
                        integer_returns: Vec<IntReturn>,
                    }
                    impl FunctionExecution {
                        fn new(active: FunctionState) -> Self {
                            Self {
                                active: Some(active),
                                integer_returns: Vec::new(),
                            }
                        }
                    }
                    impl CallExecution for FunctionExecution {
                        fn restart(&mut self, target: data::compiled::CallTarget, point: usize, values: CallInputs<'_>) -> bool {
                            if self.active.is_some() { return false; }
                            let active = match target {
                                data::compiled::CallTarget::Int(data::function::IntFunctionId(0)) => calls_int_0_state(point, values),
                                data::compiled::CallTarget::Int(data::function::IntFunctionId(1)) => calls_int_1_state(point, values),
                                data::compiled::CallTarget::Int(data::function::IntFunctionId(5)) => calls_int_5_state(point, values),
                                _ => None,
                            };
                            let Some(active) = active else { return false; };
                            self.active = Some(active);
                            true
                        }
                        fn retained_bytes(&self) -> usize {
                            std::mem::size_of::<Self>() + self.integer_returns.capacity() * std::mem::size_of::<IntReturn>()
                        }
                        fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                            let Some(mut active) = self.active.take() else { return CallProgress::Yield(self); };
                            loop {
                                match function_step(active, ops, budget) {
                                    FunctionStep::Yield(active) => {
                                        self.active = Some(active);
                                        return CallProgress::Yield(self);
                                    },
                                    FunctionStep::Int { value } => {
                                        if let Some(caller) = self.integer_returns.pop() {
                                            active = caller.small(value);
                                        } else {
                                            self.integer_returns.clear();
                                            return CallProgress::Complete { output: CallOutput::Int(value.into()), execution: self };
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
                                                return CallProgress::Interpreted { target, point, values };
                                            },
                                            _ => return CallProgress::Interpreted { target, point, values },
                                        }
                                    },
                                }
                            }
                        }
                    }
                    fn function_step(active: FunctionState, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            FunctionState::Int0Point0 { int0, int1 } => calls_int_0_run(Int0State::Point0 { int0, int1 }, ops, budget),
                            FunctionState::Int0Point1 { int0, int1, int2 } => calls_int_0_run(Int0State::Point1 { int0, int1, int2 }, ops, budget),
                            FunctionState::Int1Point0 { int0 } => calls_int_1_run(Int1State::Point0 { int0 }, ops, budget),
                            FunctionState::Int1Point1 { int0, int1 } => calls_int_1_run(Int1State::Point1 { int0, int1 }, ops, budget),
                            FunctionState::Int5Point0 { int0, int1 } => calls_int_5_run(Int5State::Point0 { int0, int1 }, ops, budget),
                            FunctionState::Int5Point1 { int0, int1, int2 } => calls_int_5_run(Int5State::Point1 { int0, int1, int2 }, ops, budget),
                        }
                    }
                    enum Int0State {
                        Point0 { int0: i128, int1: i128 },
                        Point1 { int0: i128, int1: i128, int2: i128 },
                    }
                    fn calls_int_0_run(active: Int0State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            Int0State::Point0 { int0, int1 } => {
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
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into()], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point1 { int0, int1, int2 }); }
                                *budget -= 1;
                                {
                                    FunctionStep::Int { value: int2 }
                                }
                            },
                            Int0State::Point1 { int0, int1, int2 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point1 { int0, int1, int2 }); }
                                *budget -= 1;
                                {
                                    FunctionStep::Int { value: int2 }
                                }
                            },
                        }
                    }
                    enum Int1State {
                        Point0 { int0: i128 },
                        Point1 { int0: i128, int1: i128 },
                    }
                    fn calls_int_1_run(active: Int1State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            Int1State::Point0 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point0 { int0 }); }
                                *budget -= 1;
                                let int1 = int0 + 1_i128;
                                if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(1)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 1,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point1 { int0, int1 }); }
                                *budget -= 1;
                                {
                                    FunctionStep::Int { value: int1 }
                                }
                            },
                            Int1State::Point1 { int0, int1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point1 { int0, int1 }); }
                                *budget -= 1;
                                {
                                    FunctionStep::Int { value: int1 }
                                }
                            },
                        }
                    }
                    enum Int5State {
                        Point0 { int0: i128, int1: i128 },
                        Point1 { int0: i128, int1: i128, int2: i128 },
                    }
                    fn calls_int_5_run(active: Int5State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            Int5State::Point0 { int0, int1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int5Point0 { int0, int1 }); }
                                *budget -= 1;
                                let int2 = int1 + int0;
                                if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(5)), point: data::compiled::CompiledCheckpoint {
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
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into()], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int5Point1 { int0, int1, int2 }); }
                                *budget -= 1;
                                {
                                    FunctionStep::Int { value: int2 }
                                }
                            },
                            Int5State::Point1 { int0, int1, int2 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int5Point1 { int0, int1, int2 }); }
                                *budget -= 1;
                                {
                                    FunctionStep::Int { value: int2 }
                                }
                            },
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
                    fn calls_int_1_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                        let active = match point {
                            0 => FunctionState::Int1Point0 { int0: values.int(0)? },
                            1 => FunctionState::Int1Point1 { int0: values.int(0)?, int1: values.int(1)? },
                            _ => return None,
                        };
                        Some(active)
                    }
                    fn calls_int_1_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                        if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(1)), point, values) { return Some(execution); }
                        let active = calls_int_1_state(point, values)?;
                        Some(Box::new(FunctionExecution::new(active)))
                    }
                    fn calls_int_5_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                        let active = match point {
                            0 => FunctionState::Int5Point0 { int0: values.int(0)?, int1: values.int(1)? },
                            1 => FunctionState::Int5Point1 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                            _ => return None,
                        };
                        Some(active)
                    }
                    fn calls_int_5_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                        if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(5)), point, values) { return Some(execution); }
                        let active = calls_int_5_state(point, values)?;
                        Some(Box::new(FunctionExecution::new(active)))
                    }
                    [calls_int_0_start, calls_int_1_start, calls_int_5_start]
                };
                const CALL_GROUP_1: [data::compiled::calls::CallStart; 1] = {
                    use data::compiled::calls::{BitArrayCallable, CallArguments, CallCapture, CallExecution, CallInputs, CallOps, CallOutput, CallProgress, CallStorage, CallValues, IntCallable, StringCallable, StringValue};
                    enum FunctionState {
                        Bool0Point0 {  },
                        Bool0Point1 { int0: i128 },
                        Bool0Point2 { int0: i128, int_function0: IntCallable },
                        Bool0Point3 { string0: StringValue, string_function0: StringCallable, bit_array_function0: BitArrayCallable, string_function1: StringCallable },
                        Bool0Point4 { string0: StringValue, string_function0: StringCallable, bit_array_function0: BitArrayCallable, string_function1: StringCallable, int0: i128 },
                        Bool0Point5 { string0: StringValue, string_function0: StringCallable, bit_array_function0: BitArrayCallable, string_function1: StringCallable, int0: i128, int1: i128 },
                        Bool0Point6 { string0: StringValue, string_function0: StringCallable, bit_array_function0: BitArrayCallable, string_function1: StringCallable, int0: i128, int1: i128, int2: i128 },
                        Bool0Point7 { string0: StringValue, string_function0: StringCallable, string_function1: StringCallable },
                        Bool0Point8 { string0: StringValue, string_function0: StringCallable },
                        Bool0Point9 { string0: StringValue, string_function0: StringCallable, string1: StringValue },
                        Bool0Point10 {  },
                        Bool0Point11 { bool0: bool },
                        Bool0Point12 { bool0: bool },
                        Bool0Point13 {  },
                        Bool0Point14 {  },
                        Bool0Point15 { bool0: bool },
                        Bool0Point16 {  },
                        Bool0Point17 {  },
                        Bool0Point18 {  },
                        Bool0Point19 {  },
                        Bool0Point20 {  },
                        Bool0Point21 {  },
                        Bool0Point22 {  },
                        Bool0Point23 {  },
                        Bool0Point24 {  },
                        Bool0Point25 {  },
                        Bool0Point26 {  },
                        Bool0Point27 {  },
                        Bool0Point28 {  },
                        Bool0Point29 {  },
                        Bool0Point30 {  },
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
                    enum StringReturn {
                        Bool0Call8 { string0: StringValue, string_function0: StringCallable },
                    }
                    impl StringReturn {
                        fn site(&self) -> data::source::HostCallSite {
                            match *self {
                                Self::Bool0Call8 { .. } => data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(1438, 1457)),
                            }
                        }
                        fn small(self, result: StringValue) -> FunctionState {
                            match self {
                                Self::Bool0Call8 { string0, string_function0 } => {
                                    let string1 = result;
                                    FunctionState::Bool0Point9 { string0, string_function0, string1 }
                                },
                            }
                        }
                        fn resume(self, result: StringValue) -> FunctionState { self.small(result) }
                    }
                    #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                    enum FunctionStep {
                        Yield(FunctionState),
                        Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                        Bool { value: bool },
                        StringBridge { function: data::function::StringFunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: StringReturn },
                    }
                    struct FunctionExecution {
                        active: Option<FunctionState>,
                        boolean_returns: Vec<BoolReturn>,
                        string_returns: Vec<StringReturn>,
                    }
                    impl FunctionExecution {
                        fn new(active: FunctionState) -> Self {
                            Self {
                                active: Some(active),
                                boolean_returns: Vec::new(),
                                string_returns: Vec::new(),
                            }
                        }
                    }
                    impl CallExecution for FunctionExecution {
                        fn restart(&mut self, target: data::compiled::CallTarget, point: usize, values: CallInputs<'_>) -> bool {
                            if self.active.is_some() { return false; }
                            let active = match target {
                                data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)) => calls_bool_0_state(point, values),
                                _ => None,
                            };
                            let Some(active) = active else { return false; };
                            self.active = Some(active);
                            true
                        }
                        fn retained_bytes(&self) -> usize {
                            std::mem::size_of::<Self>() + self.boolean_returns.capacity() * std::mem::size_of::<BoolReturn>() + self.string_returns.capacity() * std::mem::size_of::<StringReturn>()
                        }
                        fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                            let Some(mut active) = self.active.take() else { return CallProgress::Yield(self); };
                            loop {
                                match function_step(active, ops, budget) {
                                    FunctionStep::Yield(active) => {
                                        self.active = Some(active);
                                        return CallProgress::Yield(self);
                                    },
                                    FunctionStep::Bool { value } => {
                                        if let Some(caller) = self.boolean_returns.pop() {
                                            active = caller.small(value);
                                        } else {
                                            self.boolean_returns.clear();
                                            self.string_returns.clear();
                                            return CallProgress::Complete { output: CallOutput::Bool(value), execution: self };
                                        }
                                    },
                                    FunctionStep::StringBridge { function, site, arguments, caller } => return CallProgress::String {
                                        function, site, arguments,
                                        resume: Box::new(move |value| {
                                            self.active = Some(caller.resume(value));
                                            self
                                        }),
                                    },
                                    FunctionStep::Canonical { target, point, values } => {
                                        match target {
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
                                                return CallProgress::Interpreted { target, point, values };
                                            },
                                            data::compiled::CallTarget::String(function) => {
                                                if let Some(caller) = self.string_returns.pop() {
                                                    let site = caller.site();
                                                    return CallProgress::InterpretedString {
                                                        function, site, point, values,
                                                        resume: Box::new(move |value| {
                                                            self.active = Some(caller.resume(value));
                                                            self
                                                        }),
                                                    };
                                                }
                                                return CallProgress::Interpreted { target, point, values };
                                            },
                                            _ => return CallProgress::Interpreted { target, point, values },
                                        }
                                    },
                                }
                            }
                        }
                    }
                    fn function_step(active: FunctionState, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            FunctionState::Bool0Point0 {  } => calls_bool_0_run(Bool0State::Point0 {  }, ops, budget),
                            FunctionState::Bool0Point1 { int0 } => calls_bool_0_run(Bool0State::Point1 { int0 }, ops, budget),
                            FunctionState::Bool0Point2 { int0, int_function0 } => calls_bool_0_run(Bool0State::Point2 { int0, int_function0 }, ops, budget),
                            FunctionState::Bool0Point3 { string0, string_function0, bit_array_function0, string_function1 } => calls_bool_0_run(Bool0State::Point3 { string0, string_function0, bit_array_function0, string_function1 }, ops, budget),
                            FunctionState::Bool0Point4 { string0, string_function0, bit_array_function0, string_function1, int0 } => calls_bool_0_run(Bool0State::Point4 { string0, string_function0, bit_array_function0, string_function1, int0 }, ops, budget),
                            FunctionState::Bool0Point5 { string0, string_function0, bit_array_function0, string_function1, int0, int1 } => calls_bool_0_run(Bool0State::Point5 { string0, string_function0, bit_array_function0, string_function1, int0, int1 }, ops, budget),
                            FunctionState::Bool0Point6 { string0, string_function0, bit_array_function0, string_function1, int0, int1, int2 } => calls_bool_0_run(Bool0State::Point6 { string0, string_function0, bit_array_function0, string_function1, int0, int1, int2 }, ops, budget),
                            FunctionState::Bool0Point7 { string0, string_function0, string_function1 } => calls_bool_0_run(Bool0State::Point7 { string0, string_function0, string_function1 }, ops, budget),
                            FunctionState::Bool0Point8 { string0, string_function0 } => calls_bool_0_run(Bool0State::Point8 { string0, string_function0 }, ops, budget),
                            FunctionState::Bool0Point9 { string0, string_function0, string1 } => calls_bool_0_run(Bool0State::Point9 { string0, string_function0, string1 }, ops, budget),
                            FunctionState::Bool0Point10 {  } => calls_bool_0_run(Bool0State::Point10 {  }, ops, budget),
                            FunctionState::Bool0Point11 { bool0 } => calls_bool_0_run(Bool0State::Point11 { bool0 }, ops, budget),
                            FunctionState::Bool0Point12 { bool0 } => calls_bool_0_run(Bool0State::Point12 { bool0 }, ops, budget),
                            FunctionState::Bool0Point13 {  } => calls_bool_0_run(Bool0State::Point13 {  }, ops, budget),
                            FunctionState::Bool0Point14 {  } => calls_bool_0_run(Bool0State::Point14 {  }, ops, budget),
                            FunctionState::Bool0Point15 { bool0 } => calls_bool_0_run(Bool0State::Point15 { bool0 }, ops, budget),
                            FunctionState::Bool0Point16 {  } => calls_bool_0_run(Bool0State::Point16 {  }, ops, budget),
                            FunctionState::Bool0Point17 {  } => calls_bool_0_run(Bool0State::Point17 {  }, ops, budget),
                            FunctionState::Bool0Point18 {  } => calls_bool_0_run(Bool0State::Point18 {  }, ops, budget),
                            FunctionState::Bool0Point19 {  } => calls_bool_0_run(Bool0State::Point19 {  }, ops, budget),
                            FunctionState::Bool0Point20 {  } => calls_bool_0_run(Bool0State::Point20 {  }, ops, budget),
                            FunctionState::Bool0Point21 {  } => calls_bool_0_run(Bool0State::Point21 {  }, ops, budget),
                            FunctionState::Bool0Point22 {  } => calls_bool_0_run(Bool0State::Point22 {  }, ops, budget),
                            FunctionState::Bool0Point23 {  } => calls_bool_0_run(Bool0State::Point23 {  }, ops, budget),
                            FunctionState::Bool0Point24 {  } => calls_bool_0_run(Bool0State::Point24 {  }, ops, budget),
                            FunctionState::Bool0Point25 {  } => calls_bool_0_run(Bool0State::Point25 {  }, ops, budget),
                            FunctionState::Bool0Point26 {  } => calls_bool_0_run(Bool0State::Point26 {  }, ops, budget),
                            FunctionState::Bool0Point27 {  } => calls_bool_0_run(Bool0State::Point27 {  }, ops, budget),
                            FunctionState::Bool0Point28 {  } => calls_bool_0_run(Bool0State::Point28 {  }, ops, budget),
                            FunctionState::Bool0Point29 {  } => calls_bool_0_run(Bool0State::Point29 {  }, ops, budget),
                            FunctionState::Bool0Point30 {  } => calls_bool_0_run(Bool0State::Point30 {  }, ops, budget),
                        }
                    }
                    enum Bool0State {
                        Point0 {  },
                        Point1 { int0: i128 },
                        Point2 { int0: i128, int_function0: IntCallable },
                        Point3 { string0: StringValue, string_function0: StringCallable, bit_array_function0: BitArrayCallable, string_function1: StringCallable },
                        Point4 { string0: StringValue, string_function0: StringCallable, bit_array_function0: BitArrayCallable, string_function1: StringCallable, int0: i128 },
                        Point5 { string0: StringValue, string_function0: StringCallable, bit_array_function0: BitArrayCallable, string_function1: StringCallable, int0: i128, int1: i128 },
                        Point6 { string0: StringValue, string_function0: StringCallable, bit_array_function0: BitArrayCallable, string_function1: StringCallable, int0: i128, int1: i128, int2: i128 },
                        Point7 { string0: StringValue, string_function0: StringCallable, string_function1: StringCallable },
                        Point8 { string0: StringValue, string_function0: StringCallable },
                        Point9 { string0: StringValue, string_function0: StringCallable, string1: StringValue },
                        Point10 {  },
                        Point11 { bool0: bool },
                        Point12 { bool0: bool },
                        Point13 {  },
                        Point14 {  },
                        Point15 { bool0: bool },
                        Point16 {  },
                        Point17 {  },
                        Point18 {  },
                        Point19 {  },
                        Point20 {  },
                        Point21 {  },
                        Point22 {  },
                        Point23 {  },
                        Point24 {  },
                        Point25 {  },
                        Point26 {  },
                        Point27 {  },
                        Point28 {  },
                        Point29 {  },
                        Point30 {  },
                    }
                    fn calls_bool_0_run(mut active: Bool0State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        loop {
                            match active {
                                Bool0State::Point0 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point0 {  }); }
                                    *budget -= 1;
                                    let int0 = 2_i128;
                                    if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { ints: vec![int0.into()], ..CallValues::default() }) }; }
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point1 { int0 }); }
                                    *budget -= 1;
                                    let int_function0 = ops.int_closure(data::function::IntFunctionId(0), data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    }, vec![CallCapture::int(data::graph::IntLocalId(1), int0)]);
                                    return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { ints: vec![int0.into()], int_functions: vec![int_function0], ..CallValues::default() }) };
                                },
                                Bool0State::Point1 { int0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point1 { int0 }); }
                                    *budget -= 1;
                                    let int_function0 = ops.int_closure(data::function::IntFunctionId(0), data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    }, vec![CallCapture::int(data::graph::IntLocalId(1), int0)]);
                                    active = Bool0State::Point2 { int0, int_function0 };
                                    continue;
                                },
                                Bool0State::Point2 { int0, int_function0 } => {
                                    return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { ints: vec![int0.into()], int_functions: vec![int_function0], ..CallValues::default() }) };
                                },
                                Bool0State::Point3 { string0, string_function0, bit_array_function0, string_function1 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point3 { string0, string_function0, bit_array_function0, string_function1 }); }
                                    *budget -= 1;
                                    let int0 = 255_i128;
                                    if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(7),
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
                                    }, values: Box::new(CallValues { ints: vec![int0.into()], strings: vec![string0], string_functions: vec![string_function0, string_function1], bit_array_functions: vec![bit_array_function0], ..CallValues::default() }) }; }
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point4 { string0, string_function0, bit_array_function0, string_function1, int0 }); }
                                    *budget -= 1;
                                    let int1 = 0_i128;
                                    if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(7),
                                        instruction: 2,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], strings: vec![string0], string_functions: vec![string_function0, string_function1], bit_array_functions: vec![bit_array_function0], ..CallValues::default() }) }; }
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point5 { string0, string_function0, bit_array_function0, string_function1, int0, int1 }); }
                                    *budget -= 1;
                                    let int2 = 195_i128;
                                    if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(7),
                                        instruction: 3,
                                        ints: 3,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into()], strings: vec![string0], string_functions: vec![string_function0, string_function1], bit_array_functions: vec![bit_array_function0], ..CallValues::default() }) }; }
                                    return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(7),
                                        instruction: 3,
                                        ints: 3,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into()], strings: vec![string0], string_functions: vec![string_function0, string_function1], bit_array_functions: vec![bit_array_function0], ..CallValues::default() }) };
                                },
                                Bool0State::Point4 { string0, string_function0, bit_array_function0, string_function1, int0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point4 { string0, string_function0, bit_array_function0, string_function1, int0 }); }
                                    *budget -= 1;
                                    let int1 = 0_i128;
                                    if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(7),
                                        instruction: 2,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], strings: vec![string0], string_functions: vec![string_function0, string_function1], bit_array_functions: vec![bit_array_function0], ..CallValues::default() }) }; }
                                    active = Bool0State::Point5 { string0, string_function0, bit_array_function0, string_function1, int0, int1 };
                                    continue;
                                },
                                Bool0State::Point5 { string0, string_function0, bit_array_function0, string_function1, int0, int1 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point5 { string0, string_function0, bit_array_function0, string_function1, int0, int1 }); }
                                    *budget -= 1;
                                    let int2 = 195_i128;
                                    if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(7),
                                        instruction: 3,
                                        ints: 3,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into()], strings: vec![string0], string_functions: vec![string_function0, string_function1], bit_array_functions: vec![bit_array_function0], ..CallValues::default() }) }; }
                                    active = Bool0State::Point6 { string0, string_function0, bit_array_function0, string_function1, int0, int1, int2 };
                                    continue;
                                },
                                Bool0State::Point6 { string0, string_function0, bit_array_function0, string_function1, int0, int1, int2 } => {
                                    return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(7),
                                        instruction: 3,
                                        ints: 3,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into()], strings: vec![string0], string_functions: vec![string_function0, string_function1], bit_array_functions: vec![bit_array_function0], ..CallValues::default() }) };
                                },
                                Bool0State::Point7 { string0, string_function0, string_function1 } => {
                                    return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(8),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { strings: vec![string0], string_functions: vec![string_function0, string_function1], ..CallValues::default() }) };
                                },
                                Bool0State::Point8 { string0, string_function0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point8 { string0, string_function0 }); }
                                    *budget -= 1;
                                    return {
                                        let callable = &string_function0;
                                        let captures = callable.captures();
                                        let target = callable.target();
                                        FunctionStep::StringBridge { function: target, site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(1438, 1457)), arguments: CallArguments { values: Box::new(CallValues { strings: vec![string0.clone()], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: StringReturn::Bool0Call8 { string0, string_function0 } }
                                    };
                                },
                                Bool0State::Point9 { string0, string_function0, string1 } => {
                                    return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(9),
                                        instruction: 1,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { strings: vec![string0, string1], string_functions: vec![string_function0], ..CallValues::default() }) };
                                },
                                Bool0State::Point10 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point10 {  }); }
                                    *budget -= 1;
                                    let bool0 = true;
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point11 { bool0 }); }
                                    *budget -= 1;
                                    active = {
                                        Bool0State::Point12 { bool0 }
                                    };
                                    continue;
                                },
                                Bool0State::Point11 { bool0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point11 { bool0 }); }
                                    *budget -= 1;
                                    active = {
                                        Bool0State::Point12 { bool0 }
                                    };
                                    continue;
                                },
                                Bool0State::Point12 { bool0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point12 { bool0 }); }
                                    *budget -= 1;
                                    return {
                                        FunctionStep::Bool { value: bool0 }
                                    };
                                },
                                Bool0State::Point13 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point13 {  }); }
                                    *budget -= 1;
                                    active = {
                                        Bool0State::Point14 {  }
                                    };
                                    continue;
                                },
                                Bool0State::Point14 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point14 {  }); }
                                    *budget -= 1;
                                    let bool0 = false;
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point15 { bool0 }); }
                                    *budget -= 1;
                                    active = {
                                        Bool0State::Point12 { bool0 }
                                    };
                                    continue;
                                },
                                Bool0State::Point15 { bool0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point15 { bool0 }); }
                                    *budget -= 1;
                                    active = {
                                        Bool0State::Point12 { bool0 }
                                    };
                                    continue;
                                },
                                Bool0State::Point16 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point16 {  }); }
                                    *budget -= 1;
                                    active = {
                                        Bool0State::Point17 {  }
                                    };
                                    continue;
                                },
                                Bool0State::Point17 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point17 {  }); }
                                    *budget -= 1;
                                    active = {
                                        Bool0State::Point14 {  }
                                    };
                                    continue;
                                },
                                Bool0State::Point18 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point18 {  }); }
                                    *budget -= 1;
                                    active = {
                                        Bool0State::Point19 {  }
                                    };
                                    continue;
                                },
                                Bool0State::Point19 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point19 {  }); }
                                    *budget -= 1;
                                    active = {
                                        Bool0State::Point17 {  }
                                    };
                                    continue;
                                },
                                Bool0State::Point20 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point20 {  }); }
                                    *budget -= 1;
                                    active = {
                                        Bool0State::Point21 {  }
                                    };
                                    continue;
                                },
                                Bool0State::Point21 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point21 {  }); }
                                    *budget -= 1;
                                    active = {
                                        Bool0State::Point19 {  }
                                    };
                                    continue;
                                },
                                Bool0State::Point22 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point22 {  }); }
                                    *budget -= 1;
                                    active = {
                                        Bool0State::Point23 {  }
                                    };
                                    continue;
                                },
                                Bool0State::Point23 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point23 {  }); }
                                    *budget -= 1;
                                    active = {
                                        Bool0State::Point21 {  }
                                    };
                                    continue;
                                },
                                Bool0State::Point24 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point24 {  }); }
                                    *budget -= 1;
                                    active = {
                                        Bool0State::Point25 {  }
                                    };
                                    continue;
                                },
                                Bool0State::Point25 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point25 {  }); }
                                    *budget -= 1;
                                    active = {
                                        Bool0State::Point23 {  }
                                    };
                                    continue;
                                },
                                Bool0State::Point26 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point26 {  }); }
                                    *budget -= 1;
                                    active = {
                                        Bool0State::Point27 {  }
                                    };
                                    continue;
                                },
                                Bool0State::Point27 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point27 {  }); }
                                    *budget -= 1;
                                    active = {
                                        Bool0State::Point25 {  }
                                    };
                                    continue;
                                },
                                Bool0State::Point28 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point28 {  }); }
                                    *budget -= 1;
                                    active = {
                                        Bool0State::Point29 {  }
                                    };
                                    continue;
                                },
                                Bool0State::Point29 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point29 {  }); }
                                    *budget -= 1;
                                    active = {
                                        Bool0State::Point27 {  }
                                    };
                                    continue;
                                },
                                Bool0State::Point30 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point30 {  }); }
                                    *budget -= 1;
                                    active = {
                                        Bool0State::Point29 {  }
                                    };
                                    continue;
                                },
                            }
                        }
                    }
                    fn calls_bool_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                        let active = match point {
                            0 => FunctionState::Bool0Point0 {  },
                            1 => FunctionState::Bool0Point1 { int0: values.int(0)? },
                            2 => FunctionState::Bool0Point2 { int0: values.int(0)?, int_function0: values.int_function(0)? },
                            3 => FunctionState::Bool0Point3 { string0: values.string(0)?, string_function0: values.string_function(0)?, bit_array_function0: values.bit_array_function(0)?, string_function1: values.string_function(1)? },
                            4 => FunctionState::Bool0Point4 { string0: values.string(0)?, string_function0: values.string_function(0)?, bit_array_function0: values.bit_array_function(0)?, string_function1: values.string_function(1)?, int0: values.int(0)? },
                            5 => FunctionState::Bool0Point5 { string0: values.string(0)?, string_function0: values.string_function(0)?, bit_array_function0: values.bit_array_function(0)?, string_function1: values.string_function(1)?, int0: values.int(0)?, int1: values.int(1)? },
                            6 => FunctionState::Bool0Point6 { string0: values.string(0)?, string_function0: values.string_function(0)?, bit_array_function0: values.bit_array_function(0)?, string_function1: values.string_function(1)?, int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                            7 => FunctionState::Bool0Point7 { string0: values.string(0)?, string_function0: values.string_function(0)?, string_function1: values.string_function(1)? },
                            8 => {
                                return None
                            },
                            9 => FunctionState::Bool0Point9 { string0: values.string(0)?, string_function0: values.string_function(0)?, string1: values.string(1)? },
                            10 => FunctionState::Bool0Point10 {  },
                            11 => FunctionState::Bool0Point11 { bool0: values.bool(0)? },
                            12 => FunctionState::Bool0Point12 { bool0: values.bool(0)? },
                            13 => FunctionState::Bool0Point13 {  },
                            14 => FunctionState::Bool0Point14 {  },
                            15 => FunctionState::Bool0Point15 { bool0: values.bool(0)? },
                            16 => FunctionState::Bool0Point16 {  },
                            17 => FunctionState::Bool0Point17 {  },
                            18 => FunctionState::Bool0Point18 {  },
                            19 => FunctionState::Bool0Point19 {  },
                            20 => FunctionState::Bool0Point20 {  },
                            21 => FunctionState::Bool0Point21 {  },
                            22 => FunctionState::Bool0Point22 {  },
                            23 => FunctionState::Bool0Point23 {  },
                            24 => FunctionState::Bool0Point24 {  },
                            25 => FunctionState::Bool0Point25 {  },
                            26 => FunctionState::Bool0Point26 {  },
                            27 => FunctionState::Bool0Point27 {  },
                            28 => FunctionState::Bool0Point28 {  },
                            29 => FunctionState::Bool0Point29 {  },
                            30 => FunctionState::Bool0Point30 {  },
                            _ => return None,
                        };
                        Some(active)
                    }
                    fn calls_bool_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                        if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point, values) { return Some(execution); }
                        let active = calls_bool_0_state(point, values)?;
                        Some(Box::new(FunctionExecution::new(active)))
                    }
                    [calls_bool_0_start]
                };
                const CALL_GROUP_2: [data::compiled::calls::CallStart; 1] = {
                    use data::compiled::calls::{CallExecution, CallInputs, CallOps, CallProgress, CallStorage, CallValues, IntCallable};
                    enum FunctionState {
                        Bool1Point0 {  },
                        Bool1Point1 { int_function0: IntCallable },
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
                    #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                    enum FunctionStep {
                        Yield(FunctionState),
                        Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                    }
                    struct FunctionExecution {
                        active: Option<FunctionState>,
                        boolean_returns: Vec<BoolReturn>,
                    }
                    impl FunctionExecution {
                        fn new(active: FunctionState) -> Self {
                            Self {
                                active: Some(active),
                                boolean_returns: Vec::new(),
                            }
                        }
                    }
                    impl CallExecution for FunctionExecution {
                        fn restart(&mut self, target: data::compiled::CallTarget, point: usize, values: CallInputs<'_>) -> bool {
                            if self.active.is_some() { return false; }
                            let active = match target {
                                data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)) => calls_bool_1_state(point, values),
                                _ => None,
                            };
                            let Some(active) = active else { return false; };
                            self.active = Some(active);
                            true
                        }
                        fn retained_bytes(&self) -> usize {
                            std::mem::size_of::<Self>() + self.boolean_returns.capacity() * std::mem::size_of::<BoolReturn>()
                        }
                        fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                            let Some(active) = self.active.take() else { return CallProgress::Yield(self); };
                            match function_step(active, ops, budget) {
                                FunctionStep::Yield(active) => {
                                    self.active = Some(active);
                                    CallProgress::Yield(self)
                                },
                                FunctionStep::Canonical { target, point, values } => {
                                    match target {
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
                                            CallProgress::Interpreted { target, point, values }
                                        },
                                        _ => CallProgress::Interpreted { target, point, values },
                                    }
                                },
                            }
                        }
                    }
                    fn function_step(active: FunctionState, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            FunctionState::Bool1Point0 {  } => calls_bool_1_run(Bool1State::Point0 {  }, ops, budget),
                            FunctionState::Bool1Point1 { int_function0 } => calls_bool_1_run(Bool1State::Point1 { int_function0 }, ops, budget),
                        }
                    }
                    enum Bool1State {
                        Point0 {  },
                        Point1 { int_function0: IntCallable },
                    }
                    fn calls_bool_1_run(active: Bool1State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            Bool1State::Point0 {  } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point0 {  }); }
                                *budget -= 1;
                                let int_function0 = ops.int_closure(data::function::IntFunctionId(3), data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                }, vec![]);
                                FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 1,
                                    ints: 0,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 1,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { int_functions: vec![int_function0], ..CallValues::default() }) }
                            },
                            Bool1State::Point1 { int_function0 } => {
                                FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 1,
                                    ints: 0,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 1,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { int_functions: vec![int_function0], ..CallValues::default() }) }
                            },
                        }
                    }
                    fn calls_bool_1_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                        let active = match point {
                            0 => FunctionState::Bool1Point0 {  },
                            1 => FunctionState::Bool1Point1 { int_function0: values.int_function(0)? },
                            _ => return None,
                        };
                        Some(active)
                    }
                    fn calls_bool_1_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                        if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)), point, values) { return Some(execution); }
                        let active = calls_bool_1_state(point, values)?;
                        Some(Box::new(FunctionExecution::new(active)))
                    }
                    [calls_bool_1_start]
                };
                const CALL_GROUP_3: [data::compiled::calls::CallStart; 1] = {
                    use data::compiled::calls::{CallCapture, CallExecution, CallInputs, CallOps, CallOutput, CallProgress, CallStorage, IntCallable};
                    enum FunctionState {
                        IntFunction1Point0 { int0: i128 },
                        IntFunction1Point1 { int0: i128, int_function0: IntCallable },
                    }
                    enum IntFunctionReturn {
                    }
                    impl IntFunctionReturn {
                        fn small(self, result: IntCallable) -> FunctionState {
                            let _ = result;
                            match self {
                            }
                        }
                    }
                    #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                    enum FunctionStep {
                        Yield(FunctionState),
                        IntFunction { value: IntCallable },
                    }
                    struct FunctionExecution {
                        active: Option<FunctionState>,
                        integer_function_returns: Vec<IntFunctionReturn>,
                    }
                    impl FunctionExecution {
                        fn new(active: FunctionState) -> Self {
                            Self {
                                active: Some(active),
                                integer_function_returns: Vec::new(),
                            }
                        }
                    }
                    impl CallExecution for FunctionExecution {
                        fn restart(&mut self, target: data::compiled::CallTarget, point: usize, values: CallInputs<'_>) -> bool {
                            if self.active.is_some() { return false; }
                            let active = match target {
                                data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(1)) => calls_intfunction_1_state(point, values),
                                _ => None,
                            };
                            let Some(active) = active else { return false; };
                            self.active = Some(active);
                            true
                        }
                        fn retained_bytes(&self) -> usize {
                            std::mem::size_of::<Self>() + self.integer_function_returns.capacity() * std::mem::size_of::<IntFunctionReturn>()
                        }
                        fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                            let Some(mut active) = self.active.take() else { return CallProgress::Yield(self); };
                            loop {
                                match function_step(active, ops, budget) {
                                    FunctionStep::Yield(active) => {
                                        self.active = Some(active);
                                        return CallProgress::Yield(self);
                                    },
                                    FunctionStep::IntFunction { value } => {
                                        if let Some(caller) = self.integer_function_returns.pop() {
                                            active = caller.small(value);
                                        } else {
                                            self.integer_function_returns.clear();
                                            return CallProgress::Complete { output: CallOutput::IntFunction(value), execution: self };
                                        }
                                    },
                                }
                            }
                        }
                    }
                    fn function_step(active: FunctionState, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            FunctionState::IntFunction1Point0 { int0 } => calls_intfunction_1_run(IntFunction1State::Point0 { int0 }, ops, budget),
                            FunctionState::IntFunction1Point1 { int0, int_function0 } => calls_intfunction_1_run(IntFunction1State::Point1 { int0, int_function0 }, ops, budget),
                        }
                    }
                    enum IntFunction1State {
                        Point0 { int0: i128 },
                        Point1 { int0: i128, int_function0: IntCallable },
                    }
                    fn calls_intfunction_1_run(active: IntFunction1State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            IntFunction1State::Point0 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction1Point0 { int0 }); }
                                *budget -= 1;
                                let int_function0 = ops.int_closure(data::function::IntFunctionId(5), data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                }, vec![CallCapture::int(data::graph::IntLocalId(1), int0)]);
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction1Point1 { int0, int_function0 }); }
                                *budget -= 1;
                                {
                                    FunctionStep::IntFunction { value: int_function0 }
                                }
                            },
                            IntFunction1State::Point1 { int0, int_function0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction1Point1 { int0, int_function0 }); }
                                *budget -= 1;
                                {
                                    FunctionStep::IntFunction { value: int_function0 }
                                }
                            },
                        }
                    }
                    fn calls_intfunction_1_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                        let active = match point {
                            0 => FunctionState::IntFunction1Point0 { int0: values.int(0)? },
                            1 => FunctionState::IntFunction1Point1 { int0: values.int(0)?, int_function0: values.int_function(0)? },
                            _ => return None,
                        };
                        Some(active)
                    }
                    fn calls_intfunction_1_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                        if let Some(execution) = storage.reuse(data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(1)), point, values) { return Some(execution); }
                        let active = calls_intfunction_1_state(point, values)?;
                        Some(Box::new(FunctionExecution::new(active)))
                    }
                    [calls_intfunction_1_start]
                };
                data::compiled::CompiledFunctions {
                    ints: data::Storage::Static(&[
                    ]),
                    bools: data::Storage::Static(&[
                    ]),
                    customs: data::Storage::Static(&[
                    ]),
                    int_lists: data::Storage::Static(&[
                    ]),
                    callbacks: data::compiled::CompiledCallbacks::interpreted(),
                    native_loops: data::Storage::Static(&[
                    ]),
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
                                start: CALL_GROUP_0[0],
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(1)),
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
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 1,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: CALL_GROUP_0[1],
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(5)),
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
                                start: CALL_GROUP_0[2],
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: true,
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 0,
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
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
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
                                        block: data::graph::BlockId(7),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(7),
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
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(7),
                                        instruction: 2,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(7),
                                        instruction: 3,
                                        ints: 3,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(8),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(9),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(9),
                                        instruction: 1,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(10),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(10),
                                        instruction: 1,
                                        ints: 0,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(11),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(12),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(13),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(13),
                                        instruction: 1,
                                        ints: 0,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(14),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(15),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(16),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(17),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(18),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(19),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(20),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(21),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(22),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(23),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(24),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(25),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(26),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(27),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(28),
                                        instruction: 0,
                                        ints: 0,
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
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::StringFunction {
                                            local: data::graph::StringFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::String,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                            },
                                        },
                                        data::graph::ParamLocal::BitArrayFunction {
                                            local: data::graph::BitArrayFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::BitArray,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                            },
                                        },
                                        data::graph::ParamLocal::StringFunction {
                                            local: data::graph::StringFunctionLocalId(1),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::String,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                            },
                                        },
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::StringFunction {
                                            local: data::graph::StringFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::String,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                            },
                                        },
                                        data::graph::ParamLocal::BitArrayFunction {
                                            local: data::graph::BitArrayFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::BitArray,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                            },
                                        },
                                        data::graph::ParamLocal::StringFunction {
                                            local: data::graph::StringFunctionLocalId(1),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::String,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::StringFunction {
                                            local: data::graph::StringFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::String,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                            },
                                        },
                                        data::graph::ParamLocal::BitArrayFunction {
                                            local: data::graph::BitArrayFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::BitArray,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                            },
                                        },
                                        data::graph::ParamLocal::StringFunction {
                                            local: data::graph::StringFunctionLocalId(1),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::String,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::StringFunction {
                                            local: data::graph::StringFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::String,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                            },
                                        },
                                        data::graph::ParamLocal::BitArrayFunction {
                                            local: data::graph::BitArrayFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::BitArray,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                            },
                                        },
                                        data::graph::ParamLocal::StringFunction {
                                            local: data::graph::StringFunctionLocalId(1),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::String,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::StringFunction {
                                            local: data::graph::StringFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::String,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                            },
                                        },
                                        data::graph::ParamLocal::StringFunction {
                                            local: data::graph::StringFunctionLocalId(1),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::String,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                            },
                                        },
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::StringFunction {
                                            local: data::graph::StringFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::String,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                            },
                                        },
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::StringFunction {
                                            local: data::graph::StringFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::String,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                            },
                                        },
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 8,
                                        output: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        target: data::compiled::CallContractTarget::StringValue(data::graph::StringFunctionLocalId(0)),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("function_views", "run", data::source::SourceSpan::new(1438, 1457)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[
                                    data::compiled::CreationContract {
                                        point: 1,
                                        output: data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(0)),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                        reference: false,
                                        captures: data::Storage::Static(&[
                                            data::graph::FunctionCapture::Int {
                                                target: data::graph::IntLocalId(1),
                                                source: data::graph::IntLocalId(0),
                                            },
                                        ]),
                                    },
                                ]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 12,
                                        value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: CALL_GROUP_1[0],
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: true,
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 0,
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
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[]),
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
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[
                                    data::compiled::CreationContract {
                                        point: 0,
                                        output: data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(3)),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                        reference: false,
                                        captures: data::Storage::Static(&[]),
                                    },
                                ]),
                                returns: data::Storage::Static(&[]),
                                tails: data::Storage::Static(&[]),
                                start: CALL_GROUP_2[0],
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(1)),
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
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[
                                    data::compiled::CreationContract {
                                        point: 0,
                                        output: data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
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
                                            data::graph::FunctionCapture::Int {
                                                target: data::graph::IntLocalId(1),
                                                source: data::graph::IntLocalId(0),
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
                                start: CALL_GROUP_3[0],
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
                    0..2,
                    2..10,
                    0..0,
                    10..13,
                    13..14,
                    0..0,
                    14..16,
                    16..25,
                    25..28,
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
                    28..30,
                    0..0,
                    30..31,
                    31..32,
                    0..0,
                    0..0,
                    32..36,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    36..37,
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
                    37..38,
                ],
                functions: data::Storage::Static(&[
                    data::function::FunctionContract {
                        parameters: 0..1,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(22),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 1..2,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(20),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::NeverFunction(data::graph::NeverFunctionLocal {
                                    id: data::graph::NeverFunctionLocalId(0),
                                    type_: data::type_::GenericFunctionType {
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                        },
                                        shape: data::type_::FunctionShape {
                                            shape_id: data::type_::ValueShapeId(23),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                            },
                                        },
                                    },
                                }),
                                shape: data::type_::ValueShapeId(23),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 2..3,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                shape: data::type_::ValueShapeId(1),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 3..4,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 4..5,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 5..6,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 6..7,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                    id: data::graph::ExternalFunctionLocalId(0),
                                    type_: data::type_::ExternalFunctionType {
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                        },
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueShapeId(0),
                                        ]),
                                        return_: data::type_::ExternalTypeId(0),
                                    },
                                }),
                                shape: data::type_::ValueShapeId(3),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 7..8,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                shape: data::type_::ValueShapeId(1),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 8..9,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 9..9,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 9..10,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(15),
                        ]),
                        return_: data::type_::ValueShapeId(16),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 10..11,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(16),
                        ]),
                        return_: data::type_::ValueShapeId(16),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                shape: data::type_::ValueShapeId(16),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 11..12,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(16),
                        ]),
                        return_: data::type_::ValueShapeId(16),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::BitArrayFunction {
                                    local: data::graph::BitArrayFunctionLocalId(0),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::BitArray,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                    },
                                },
                                shape: data::type_::ValueShapeId(18),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 12..13,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(15),
                        ]),
                        return_: data::type_::ValueShapeId(15),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::StringFunction {
                                    local: data::graph::StringFunctionLocalId(0),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::String,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::String),
                                    },
                                },
                                shape: data::type_::ValueShapeId(17),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 13..14,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(8),
                        ]),
                        return_: data::type_::ValueShapeId(9),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 14..15,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(12),
                        ]),
                        return_: data::type_::ValueShapeId(6),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 15..16,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 16..17,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(16),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 17..18,
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
                                shape: data::type_::ValueShapeId(2),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 18..19,
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
                                shape: data::type_::ValueShapeId(2),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 19..20,
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
                                shape: data::type_::ValueShapeId(2),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 20..21,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                                    id: data::graph::CoreFunctionFunctionLocalId(0),
                                    type_: data::type_::FunctionFunctionType {
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            })),
                                        },
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueShapeId(1),
                                        ]),
                                        return_: data::type_::FunctionShape {
                                            shape_id: data::type_::ValueShapeId(2),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    },
                                })),
                                shape: data::type_::ValueShapeId(13),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 21..22,
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
                                shape: data::type_::ValueShapeId(2),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 22..23,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::External(data::graph::ExternalFunctionFunctionLocal {
                                    id: data::graph::ExternalFunctionFunctionLocalId(0),
                                    type_: data::type_::FunctionFunctionType {
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                            })),
                                        },
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueShapeId(0),
                                        ]),
                                        return_: data::type_::FunctionShape {
                                            shape_id: data::type_::ValueShapeId(3),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                            },
                                        },
                                    },
                                })),
                                shape: data::type_::ValueShapeId(14),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 23..24,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 24..24,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(19),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 24..24,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(19),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 24..24,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(19),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 24..25,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(3),
                        ]),
                        return_: data::type_::ValueShapeId(2),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 25..26,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(2),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 26..27,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(18),
                        ]),
                        return_: data::type_::ValueShapeId(17),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 27..28,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(17),
                        ]),
                        return_: data::type_::ValueShapeId(18),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 28..29,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(2),
                        ]),
                        return_: data::type_::ValueShapeId(3),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 29..30,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(3),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                                    id: data::graph::CoreFunctionFunctionLocalId(0),
                                    type_: data::type_::FunctionFunctionType {
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            })),
                                        },
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueShapeId(1),
                                        ]),
                                        return_: data::type_::FunctionShape {
                                            shape_id: data::type_::ValueShapeId(2),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    },
                                })),
                                shape: data::type_::ValueShapeId(13),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 30..31,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(3),
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
                                shape: data::type_::ValueShapeId(2),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 31..32,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(3),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                    id: data::graph::ExternalFunctionLocalId(0),
                                    type_: data::type_::ExternalFunctionType {
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                        },
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueShapeId(0),
                                        ]),
                                        return_: data::type_::ExternalTypeId(0),
                                    },
                                }),
                                shape: data::type_::ValueShapeId(3),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 32..33,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(23),
                        ]),
                        return_: data::type_::ValueShapeId(21),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 33..34,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(13),
                        ]),
                        return_: data::type_::ValueShapeId(14),
                        captures: data::Storage::Static(&[]),
                    },
                ]),
                parameters: data::Storage::Static(&[
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    }),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    }),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    }),
                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(3),
                        },
                    }),
                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(2),
                            shape_id: data::type_::CustomValueShapeId(5),
                        },
                    }),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    }),
                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    }),
                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    }),
                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    }),
                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    }),
                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    }),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                        id: data::graph::ExternalFunctionLocalId(0),
                        type_: data::type_::ExternalFunctionType {
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                            },
                            arguments: data::Storage::Static(&[
                                data::type_::ValueShapeId(0),
                            ]),
                            return_: data::type_::ExternalTypeId(0),
                        },
                    }),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::BitArrayFunction {
                        local: data::graph::BitArrayFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::BitArray,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                        },
                    },
                    data::graph::ParamLocal::StringFunction {
                        local: data::graph::StringFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::String,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::String),
                        },
                    },
                    data::graph::ParamLocal::IntFunction {
                        local: data::graph::IntFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        },
                    },
                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    }),
                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    }),
                    data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    }),
                    data::graph::ParamLocal::NeverFunction(data::graph::NeverFunctionLocal {
                        id: data::graph::NeverFunctionLocalId(0),
                        type_: data::type_::GenericFunctionType {
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                            },
                            shape: data::type_::FunctionShape {
                                shape_id: data::type_::ValueShapeId(23),
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                },
                            },
                        },
                    }),
                    data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                        id: data::graph::CoreFunctionFunctionLocalId(0),
                        type_: data::type_::FunctionFunctionType {
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                })),
                            },
                            arguments: data::Storage::Static(&[
                                data::type_::ValueShapeId(1),
                            ]),
                            return_: data::type_::FunctionShape {
                                shape_id: data::type_::ValueShapeId(2),
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                },
                            },
                        },
                    })),
                ]),
            },
            list_types: data::type_::ListTypeTable {
                types: data::Storage::Static(&[
                    data::type_::ListStorageTypeId::Custom(data::type_::CustomListTypeId {
                        list_type: data::type_::ListTypeId(0),
                        item_type: data::type_::CustomTypeId(2),
                    }),
                    data::type_::ListStorageTypeId::Custom(data::type_::CustomListTypeId {
                        list_type: data::type_::ListTypeId(1),
                        item_type: data::type_::CustomTypeId(3),
                    }),
                ]),
                tuple_items: data::Storage::Static(&[]),
                function_items: data::Storage::Static(&[]),
            },
            custom_types: data::type_::CustomTypeTable {
                types: data::Storage::Static(&[
                    data::type_::CustomTypeDescriptor {
                        type_: data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Handler"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
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
                                name: data::Text::Static("Handler"),
                                native_tag: data::Text::Static("handler"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: None,
                                        type_: data::type_::ValueType::Function(data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        }),
                                        shape: data::type_::ValueShapeId(2),
                                        refinement: data::type_::FieldRefinement::Function {
                                            arguments: data::Storage::Static(&[
                                                data::type_::FieldRefinement::Argument(0),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::FieldRefinement::Argument(1)),
                                        },
                                    },
                                ]),
                            },
                        ]),
                    },
                    data::type_::CustomTypeDescriptor {
                        type_: data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Handler"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                    package: data::Text::Static("application"),
                                    module: data::Text::Static("function_views"),
                                    name: data::Text::Static("Erased"),
                                    arguments: data::Storage::Static(&[]),
                                }),
                                data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                    package: data::Text::Static("application"),
                                    module: data::Text::Static("function_views"),
                                    name: data::Text::Static("Erased"),
                                    arguments: data::Storage::Static(&[]),
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
                                name: data::Text::Static("Handler"),
                                native_tag: data::Text::Static("handler"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: None,
                                        type_: data::type_::ValueType::Function(data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                        }),
                                        shape: data::type_::ValueShapeId(3),
                                        refinement: data::type_::FieldRefinement::Function {
                                            arguments: data::Storage::Static(&[
                                                data::type_::FieldRefinement::Argument(0),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::FieldRefinement::Argument(1)),
                                        },
                                    },
                                ]),
                            },
                        ]),
                    },
                    data::type_::CustomTypeDescriptor {
                        type_: data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Tree"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                    arguments: data::Storage::Static(&[
                                        data::type_::TypeMetadata::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                                }),
                            ]),
                        },
                        constructor_count: 2,
                        constructors: data::Storage::Static(&[
                            data::type_::CustomConstructorDescriptor {
                                id: data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(2),
                                    index: 0,
                                },
                                name: data::Text::Static("Leaf"),
                                native_tag: data::Text::Static("leaf"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: None,
                                        type_: data::type_::ValueType::Function(data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        }),
                                        shape: data::type_::ValueShapeId(2),
                                        refinement: data::type_::FieldRefinement::Argument(0),
                                    },
                                ]),
                            },
                            data::type_::CustomConstructorDescriptor {
                                id: data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(2),
                                    index: 1,
                                },
                                name: data::Text::Static("Branch"),
                                native_tag: data::Text::Static("branch"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: None,
                                        type_: data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                        shape: data::type_::ValueShapeId(5),
                                        refinement: data::type_::FieldRefinement::List(data::Storage::Static(&data::type_::FieldRefinement::Custom(data::Storage::Static(&[
                                            data::type_::FieldRefinement::Argument(0),
                                        ])))),
                                    },
                                ]),
                            },
                        ]),
                    },
                    data::type_::CustomTypeDescriptor {
                        type_: data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Tree"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                    arguments: data::Storage::Static(&[
                                        data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                            package: data::Text::Static("application"),
                                            module: data::Text::Static("function_views"),
                                            name: data::Text::Static("Erased"),
                                            arguments: data::Storage::Static(&[]),
                                        }),
                                    ]),
                                    return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                        package: data::Text::Static("application"),
                                        module: data::Text::Static("function_views"),
                                        name: data::Text::Static("Erased"),
                                        arguments: data::Storage::Static(&[]),
                                    })),
                                }),
                            ]),
                        },
                        constructor_count: 2,
                        constructors: data::Storage::Static(&[
                            data::type_::CustomConstructorDescriptor {
                                id: data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(3),
                                    index: 0,
                                },
                                name: data::Text::Static("Leaf"),
                                native_tag: data::Text::Static("leaf"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: None,
                                        type_: data::type_::ValueType::Function(data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                        }),
                                        shape: data::type_::ValueShapeId(3),
                                        refinement: data::type_::FieldRefinement::Argument(0),
                                    },
                                ]),
                            },
                            data::type_::CustomConstructorDescriptor {
                                id: data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(3),
                                    index: 1,
                                },
                                name: data::Text::Static("Branch"),
                                native_tag: data::Text::Static("branch"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: None,
                                        type_: data::type_::ValueType::List(data::type_::ListTypeId(1)),
                                        shape: data::type_::ValueShapeId(7),
                                        refinement: data::type_::FieldRefinement::List(data::Storage::Static(&data::type_::FieldRefinement::Custom(data::Storage::Static(&[
                                            data::type_::FieldRefinement::Argument(0),
                                        ])))),
                                    },
                                ]),
                            },
                        ]),
                    },
                    data::type_::CustomTypeDescriptor {
                        type_: data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Empty"),
                            arguments: data::Storage::Static(&[]),
                        },
                        constructor_count: 1,
                        constructors: data::Storage::Static(&[]),
                    },
                ]),
                definitions: data::Storage::Static(&[
                    data::type_::CustomDefinition {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Empty"),
                        publicity: data::type_::CustomTypePublicity::Public,
                        opaque: false,
                        parameters: 0,
                        constructors: data::Storage::Static(&[
                            data::type_::ConstructorDefinition {
                                name: data::Text::Static("Again"),
                                fields: data::Storage::Static(&[
                                    data::type_::FieldDefinition {
                                        label: None,
                                        type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                            package: data::Text::Static("application"),
                                            module: data::Text::Static("function_views"),
                                            name: data::Text::Static("Empty"),
                                            arguments: data::Storage::Static(&[]),
                                        }),
                                    },
                                ]),
                            },
                        ]),
                    },
                    data::type_::CustomDefinition {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Handler"),
                        publicity: data::type_::CustomTypePublicity::Public,
                        opaque: false,
                        parameters: 2,
                        constructors: data::Storage::Static(&[
                            data::type_::ConstructorDefinition {
                                name: data::Text::Static("Handler"),
                                fields: data::Storage::Static(&[
                                    data::type_::FieldDefinition {
                                        label: None,
                                        type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                            arguments: data::Storage::Static(&[
                                                data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::TypeMetadata::Parameter(data::type_::parameter_id(1))),
                                        }),
                                    },
                                ]),
                            },
                        ]),
                    },
                    data::type_::CustomDefinition {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Tree"),
                        publicity: data::type_::CustomTypePublicity::Public,
                        opaque: false,
                        parameters: 1,
                        constructors: data::Storage::Static(&[
                            data::type_::ConstructorDefinition {
                                name: data::Text::Static("Leaf"),
                                fields: data::Storage::Static(&[
                                    data::type_::FieldDefinition {
                                        label: None,
                                        type_: data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                                    },
                                ]),
                            },
                            data::type_::ConstructorDefinition {
                                name: data::Text::Static("Branch"),
                                fields: data::Storage::Static(&[
                                    data::type_::FieldDefinition {
                                        label: None,
                                        type_: data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                            package: data::Text::Static("application"),
                                            module: data::Text::Static("function_views"),
                                            name: data::Text::Static("Tree"),
                                            arguments: data::Storage::Static(&[
                                                data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                                            ]),
                                        }))),
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
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
            },
            value_shapes: data::type_::ValueShapeTable {
                shapes: data::Storage::Static(&[
                    data::type_::ValueShapeDescriptor::External(data::type_::ExternalTypeId(0)),
                    data::type_::ValueShapeDescriptor::Int,
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                    },
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                    },
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(1)),
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(4)),
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(2)),
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(6)),
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(3)),
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(0)),
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(4)),
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(10)),
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(5)),
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(2),
                    },
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(3),
                    },
                    data::type_::ValueShapeDescriptor::BitArray,
                    data::type_::ValueShapeDescriptor::String,
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(16),
                        ]),
                        return_: data::type_::ValueShapeId(16),
                    },
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(15),
                        ]),
                        return_: data::type_::ValueShapeId(15),
                    },
                    data::type_::ValueShapeDescriptor::Bool,
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(6)),
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(20),
                    },
                    data::type_::ValueShapeDescriptor::Parameter(data::type_::parameter_id(0)),
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(22),
                    },
                ]),
                shape_types: data::Storage::Static(&[
                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                    data::type_::ValueType::Int,
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    }),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                    }),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(3)),
                    data::type_::ValueType::List(data::type_::ListTypeId(1)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        })),
                    }),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                        })),
                    }),
                    data::type_::ValueType::BitArray,
                    data::type_::ValueType::String,
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::String,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::String),
                    }),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::BitArray,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                    }),
                    data::type_::ValueType::Bool,
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(4)),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(4))),
                    }),
                    data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                    }),
                ]),
                custom_shapes: data::Storage::Static(&[
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(1),
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(0),
                        ]),
                        constructor: data::type_::CustomConstructorRefinement::Any,
                    },
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(2),
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(2),
                        ]),
                        constructor: data::type_::CustomConstructorRefinement::Any,
                    },
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(3),
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(3),
                        ]),
                        constructor: data::type_::CustomConstructorRefinement::Any,
                    },
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(0),
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                            data::type_::ValueShapeId(1),
                        ]),
                        constructor: data::type_::CustomConstructorRefinement::Exact(0),
                    },
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(2),
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(2),
                        ]),
                        constructor: data::type_::CustomConstructorRefinement::Exact(0),
                    },
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(2),
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(2),
                        ]),
                        constructor: data::type_::CustomConstructorRefinement::Exact(1),
                    },
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(4),
                        arguments: data::Storage::Static(&[]),
                        constructor: data::type_::CustomConstructorRefinement::Any,
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
            bools: data::Storage::Static(&[
                data::program::LibraryFunctionEntry {
                    function: data::function::BoolFunctionId(0),
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
                    function: data::function::BoolFunctionId(1),
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
                    function: data::function::BoolFunctionId(2),
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
            nils: data::Storage::Static(&[]),
            tuples: data::Storage::Static(&[]),
            lists: data::Storage::Static(&[]),
            functions: data::Storage::Static(&[]),
        },
        exports: data::Storage::Static(&[
            data::Export {
                name: data::Text::Static("run"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("invalid_input"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
                },
                slot: 1,
            },
            data::Export {
                name: data::Text::Static("stopped"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
                },
                slot: 2,
            },
        ]),
    },
    value_functions: data::Storage::Static(&[
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        arguments: data::Storage::Static(&[]),
                    })),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        })),
                    }),
                    shape: data::type_::ValueShapeId(3),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                    }),
                    shape: data::type_::ValueShapeId(2),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::IntFunction {
                        local: data::graph::IntFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        },
                    }),
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
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }), data::type_::ExternalTypeId(0)),
                    ]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[
                        data::host::NativeConversionId(0),
                        data::host::NativeConversionId(1),
                    ]),
                    nodes: data::Storage::Static(&[
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                        package: data::Text::Static("application"),
                                        module: data::Text::Static("function_views"),
                                        name: data::Text::Static("Erased"),
                                        arguments: data::Storage::Static(&[]),
                                    }),
                                ]),
                                return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                    package: data::Text::Static("application"),
                                    module: data::Text::Static("function_views"),
                                    name: data::Text::Static("Erased"),
                                    arguments: data::Storage::Static(&[]),
                                })),
                            }),
                            kind: data::host::NativeConversionKind::Function(data::Storage::Static(&[
                                data::host::NativeFunctionView {
                                    source: data::type_::FunctionMetadata {
                                        arguments: data::Storage::Static(&[
                                            data::type_::TypeMetadata::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                                    },
                                    target: data::function::ProfiledRuntimeFunctionId::External(data::function::ExternalFunctionId {
                                        index: 2,
                                        return_type: data::type_::ExternalTypeId(0),
                                    }),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                    },
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
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                    ]),
                                    host: 1,
                                    host_value: true,
                                },
                            ])),
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                            kind: data::host::NativeConversionKind::External {
                                rule: 0,
                            },
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Int,
                            kind: data::host::NativeConversionKind::Int,
                        },
                    ]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                })),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::External,
                index: 2,
            }),
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        arguments: data::Storage::Static(&[]),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                    package: data::Text::Static("application"),
                    module: data::Text::Static("function_views"),
                    name: data::Text::Static("Erased"),
                    arguments: data::Storage::Static(&[]),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        })),
                    }),
                    shape: data::type_::ValueShapeId(3),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                    }),
                    shape: data::type_::ValueShapeId(2),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    })),
                ]),
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
                        shape: data::type_::ValueShapeId(2),
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
                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: Some(data::host::HostNativeView {
                parent: 0,
                source: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                parent_value: true,
                arguments: data::Storage::Static(&[
                    data::host::NativeConversionId(2),
                ]),
                return_: data::host::NativeConversionId(1),
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        })),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
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
                    shape: data::type_::ValueShapeId(2),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        })),
                    }),
                    shape: data::type_::ValueShapeId(3),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                        id: data::graph::ExternalFunctionLocalId(0),
                        type_: data::type_::ExternalFunctionType {
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                            },
                            arguments: data::Storage::Static(&[
                                data::type_::ValueShapeId(0),
                            ]),
                            return_: data::type_::ExternalTypeId(0),
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
                    entries: data::Storage::Static(&[]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }), data::type_::ExternalTypeId(0)),
                    ]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[
                        data::host::NativeConversionId(0),
                        data::host::NativeConversionId(1),
                    ]),
                    nodes: data::Storage::Static(&[
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                            }),
                            kind: data::host::NativeConversionKind::Function(data::Storage::Static(&[
                                data::host::NativeFunctionView {
                                    source: data::type_::FunctionMetadata {
                                        arguments: data::Storage::Static(&[
                                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                                package: data::Text::Static("application"),
                                                module: data::Text::Static("function_views"),
                                                name: data::Text::Static("Erased"),
                                                arguments: data::Storage::Static(&[]),
                                            }),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                            package: data::Text::Static("application"),
                                            module: data::Text::Static("function_views"),
                                            name: data::Text::Static("Erased"),
                                            arguments: data::Storage::Static(&[]),
                                        })),
                                    },
                                    target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Int(data::function::IntFunctionId(4))),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    },
                                    captures: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                id: data::graph::ExternalFunctionLocalId(0),
                                                type_: data::type_::ExternalFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::ExternalTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                    ]),
                                    host: 3,
                                    host_value: true,
                                },
                            ])),
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                            kind: data::host::NativeConversionKind::External {
                                rule: 0,
                            },
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Int,
                            kind: data::host::NativeConversionKind::Int,
                        },
                    ]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                })),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::Int,
                index: 4,
            }),
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                    }),
                    shape: data::type_::ValueShapeId(2),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        })),
                    }),
                    shape: data::type_::ValueShapeId(3),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                ]),
                captures: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                            id: data::graph::ExternalFunctionLocalId(0),
                            type_: data::type_::ExternalFunctionType {
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                },
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueShapeId(0),
                                ]),
                                return_: data::type_::ExternalTypeId(0),
                            },
                        }),
                        shape: data::type_::ValueShapeId(3),
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
                    data::type_::ValueType::Int,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Int),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: Some(data::host::HostNativeView {
                parent: 2,
                source: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        arguments: data::Storage::Static(&[]),
                    })),
                },
                parent_value: true,
                arguments: data::Storage::Static(&[
                    data::host::NativeConversionId(1),
                ]),
                return_: data::host::NativeConversionId(2),
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Handler"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                            data::type_::TypeMetadata::Int,
                        ]),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                    package: data::Text::Static("application"),
                    module: data::Text::Static("function_views"),
                    name: data::Text::Static("Handler"),
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }),
                        data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }),
                    ]),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Handler"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                        ]),
                    }),
                    shape: data::type_::ValueShapeId(9),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Handler"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                            data::type_::TypeMetadata::Int,
                        ]),
                    }),
                    shape: data::type_::ValueShapeId(8),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(3),
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
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Handler"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                                data::type_::TypeMetadata::Int,
                            ]),
                        }), data::type_::CustomTypeId(0)),
                        (data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Handler"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                    package: data::Text::Static("application"),
                                    module: data::Text::Static("function_views"),
                                    name: data::Text::Static("Erased"),
                                    arguments: data::Storage::Static(&[]),
                                }),
                                data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                    package: data::Text::Static("application"),
                                    module: data::Text::Static("function_views"),
                                    name: data::Text::Static("Erased"),
                                    arguments: data::Storage::Static(&[]),
                                }),
                            ]),
                        }), data::type_::CustomTypeId(1)),
                    ]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }), data::type_::ExternalTypeId(0)),
                    ]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[
                        data::host::NativeConversionId(0),
                        data::host::NativeConversionId(1),
                    ]),
                    nodes: data::Storage::Static(&[
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Handler"),
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                        package: data::Text::Static("application"),
                                        module: data::Text::Static("function_views"),
                                        name: data::Text::Static("Erased"),
                                        arguments: data::Storage::Static(&[]),
                                    }),
                                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                        package: data::Text::Static("application"),
                                        module: data::Text::Static("function_views"),
                                        name: data::Text::Static("Erased"),
                                        arguments: data::Storage::Static(&[]),
                                    }),
                                ]),
                            }),
                            kind: data::host::NativeConversionKind::CustomView(data::Storage::Static(&[
                                data::host::NativeCustomView {
                                    source: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                        package: data::Text::Static("application"),
                                        module: data::Text::Static("function_views"),
                                        name: data::Text::Static("Handler"),
                                        arguments: data::Storage::Static(&[
                                            data::type_::TypeMetadata::Int,
                                            data::type_::TypeMetadata::Int,
                                        ]),
                                    }),
                                    constructors: data::Storage::Static(&[
                                        data::host::NativeConstructor {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(1),
                                                index: 0,
                                            },
                                            tag: data::Text::Static("handler"),
                                            fields: data::Storage::Static(&[
                                                data::host::NativeConversionId(2),
                                            ]),
                                        },
                                    ]),
                                },
                            ])),
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                            kind: data::host::NativeConversionKind::External {
                                rule: 0,
                            },
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                        package: data::Text::Static("application"),
                                        module: data::Text::Static("function_views"),
                                        name: data::Text::Static("Erased"),
                                        arguments: data::Storage::Static(&[]),
                                    }),
                                ]),
                                return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                    package: data::Text::Static("application"),
                                    module: data::Text::Static("function_views"),
                                    name: data::Text::Static("Erased"),
                                    arguments: data::Storage::Static(&[]),
                                })),
                            }),
                            kind: data::host::NativeConversionKind::Function(data::Storage::Static(&[
                                data::host::NativeFunctionView {
                                    source: data::type_::FunctionMetadata {
                                        arguments: data::Storage::Static(&[
                                            data::type_::TypeMetadata::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                                    },
                                    target: data::function::ProfiledRuntimeFunctionId::External(data::function::ExternalFunctionId {
                                        index: 3,
                                        return_type: data::type_::ExternalTypeId(0),
                                    }),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                    },
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
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                    ]),
                                    host: 5,
                                    host_value: true,
                                },
                            ])),
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Int,
                            kind: data::host::NativeConversionKind::Int,
                        },
                    ]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(1))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::External,
                index: 3,
            }),
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        arguments: data::Storage::Static(&[]),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                    package: data::Text::Static("application"),
                    module: data::Text::Static("function_views"),
                    name: data::Text::Static("Erased"),
                    arguments: data::Storage::Static(&[]),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Handler"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                        ]),
                    }),
                    shape: data::type_::ValueShapeId(9),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Handler"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                            data::type_::TypeMetadata::Int,
                        ]),
                    }),
                    shape: data::type_::ValueShapeId(8),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    })),
                ]),
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
                        shape: data::type_::ValueShapeId(2),
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
                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: Some(data::host::HostNativeView {
                parent: 4,
                source: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                parent_value: true,
                arguments: data::Storage::Static(&[
                    data::host::NativeConversionId(3),
                ]),
                return_: data::host::NativeConversionId(1),
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Tree"),
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
                    package: data::Text::Static("application"),
                    module: data::Text::Static("function_views"),
                    name: data::Text::Static("Tree"),
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                    package: data::Text::Static("application"),
                                    module: data::Text::Static("function_views"),
                                    name: data::Text::Static("Erased"),
                                    arguments: data::Storage::Static(&[]),
                                }),
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            })),
                        }),
                    ]),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Tree"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                        package: data::Text::Static("application"),
                                        module: data::Text::Static("function_views"),
                                        name: data::Text::Static("Erased"),
                                        arguments: data::Storage::Static(&[]),
                                    }),
                                ]),
                                return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                    package: data::Text::Static("application"),
                                    module: data::Text::Static("function_views"),
                                    name: data::Text::Static("Erased"),
                                    arguments: data::Storage::Static(&[]),
                                })),
                            }),
                        ]),
                    }),
                    shape: data::type_::ValueShapeId(6),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Tree"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                            }),
                        ]),
                    }),
                    shape: data::type_::ValueShapeId(12),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(2),
                            shape_id: data::type_::CustomValueShapeId(5),
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
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Tree"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                    arguments: data::Storage::Static(&[
                                        data::type_::TypeMetadata::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                                }),
                            ]),
                        }), data::type_::CustomTypeId(2)),
                        (data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Tree"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                    arguments: data::Storage::Static(&[
                                        data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                            package: data::Text::Static("application"),
                                            module: data::Text::Static("function_views"),
                                            name: data::Text::Static("Erased"),
                                            arguments: data::Storage::Static(&[]),
                                        }),
                                    ]),
                                    return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                        package: data::Text::Static("application"),
                                        module: data::Text::Static("function_views"),
                                        name: data::Text::Static("Erased"),
                                        arguments: data::Storage::Static(&[]),
                                    })),
                                }),
                            ]),
                        }), data::type_::CustomTypeId(3)),
                    ]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }), data::type_::ExternalTypeId(0)),
                    ]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[
                        data::host::NativeConversionId(0),
                        data::host::NativeConversionId(1),
                    ]),
                    nodes: data::Storage::Static(&[
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Tree"),
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                        arguments: data::Storage::Static(&[
                                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                                package: data::Text::Static("application"),
                                                module: data::Text::Static("function_views"),
                                                name: data::Text::Static("Erased"),
                                                arguments: data::Storage::Static(&[]),
                                            }),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                            package: data::Text::Static("application"),
                                            module: data::Text::Static("function_views"),
                                            name: data::Text::Static("Erased"),
                                            arguments: data::Storage::Static(&[]),
                                        })),
                                    }),
                                ]),
                            }),
                            kind: data::host::NativeConversionKind::CustomView(data::Storage::Static(&[
                                data::host::NativeCustomView {
                                    source: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                        package: data::Text::Static("application"),
                                        module: data::Text::Static("function_views"),
                                        name: data::Text::Static("Tree"),
                                        arguments: data::Storage::Static(&[
                                            data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::TypeMetadata::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                                            }),
                                        ]),
                                    }),
                                    constructors: data::Storage::Static(&[
                                        data::host::NativeConstructor {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(3),
                                                index: 0,
                                            },
                                            tag: data::Text::Static("leaf"),
                                            fields: data::Storage::Static(&[
                                                data::host::NativeConversionId(2),
                                            ]),
                                        },
                                        data::host::NativeConstructor {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(3),
                                                index: 1,
                                            },
                                            tag: data::Text::Static("branch"),
                                            fields: data::Storage::Static(&[
                                                data::host::NativeConversionId(3),
                                            ]),
                                        },
                                    ]),
                                },
                            ])),
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                            kind: data::host::NativeConversionKind::External {
                                rule: 0,
                            },
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                        package: data::Text::Static("application"),
                                        module: data::Text::Static("function_views"),
                                        name: data::Text::Static("Erased"),
                                        arguments: data::Storage::Static(&[]),
                                    }),
                                ]),
                                return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                    package: data::Text::Static("application"),
                                    module: data::Text::Static("function_views"),
                                    name: data::Text::Static("Erased"),
                                    arguments: data::Storage::Static(&[]),
                                })),
                            }),
                            kind: data::host::NativeConversionKind::Function(data::Storage::Static(&[
                                data::host::NativeFunctionView {
                                    source: data::type_::FunctionMetadata {
                                        arguments: data::Storage::Static(&[
                                            data::type_::TypeMetadata::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                                    },
                                    target: data::function::ProfiledRuntimeFunctionId::External(data::function::ExternalFunctionId {
                                        index: 4,
                                        return_type: data::type_::ExternalTypeId(0),
                                    }),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                    },
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
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                    ]),
                                    host: 7,
                                    host_value: true,
                                },
                            ])),
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Tree"),
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                        arguments: data::Storage::Static(&[
                                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                                package: data::Text::Static("application"),
                                                module: data::Text::Static("function_views"),
                                                name: data::Text::Static("Erased"),
                                                arguments: data::Storage::Static(&[]),
                                            }),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                            package: data::Text::Static("application"),
                                            module: data::Text::Static("function_views"),
                                            name: data::Text::Static("Erased"),
                                            arguments: data::Storage::Static(&[]),
                                        })),
                                    }),
                                ]),
                            }))),
                            kind: data::host::NativeConversionKind::List {
                                storage: data::type_::ListTypeId(1),
                                item: data::host::NativeConversionId(0),
                            },
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Int,
                            kind: data::host::NativeConversionKind::Int,
                        },
                    ]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(3))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::External,
                index: 4,
            }),
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        arguments: data::Storage::Static(&[]),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                    package: data::Text::Static("application"),
                    module: data::Text::Static("function_views"),
                    name: data::Text::Static("Erased"),
                    arguments: data::Storage::Static(&[]),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Tree"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                        package: data::Text::Static("application"),
                                        module: data::Text::Static("function_views"),
                                        name: data::Text::Static("Erased"),
                                        arguments: data::Storage::Static(&[]),
                                    }),
                                ]),
                                return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                    package: data::Text::Static("application"),
                                    module: data::Text::Static("function_views"),
                                    name: data::Text::Static("Erased"),
                                    arguments: data::Storage::Static(&[]),
                                })),
                            }),
                        ]),
                    }),
                    shape: data::type_::ValueShapeId(6),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Tree"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                            }),
                        ]),
                    }),
                    shape: data::type_::ValueShapeId(12),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    })),
                ]),
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
                        shape: data::type_::ValueShapeId(2),
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
                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: Some(data::host::HostNativeView {
                parent: 6,
                source: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                parent_value: true,
                arguments: data::Storage::Static(&[
                    data::host::NativeConversionId(4),
                ]),
                return_: data::host::NativeConversionId(1),
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                        })),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        })),
                    })),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                    package: data::Text::Static("application"),
                                    module: data::Text::Static("function_views"),
                                    name: data::Text::Static("Erased"),
                                    arguments: data::Storage::Static(&[]),
                                }),
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            })),
                        })),
                    }),
                    shape: data::type_::ValueShapeId(14),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                        })),
                    }),
                    shape: data::type_::ValueShapeId(13),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                        id: data::graph::CoreFunctionFunctionLocalId(0),
                        type_: data::type_::FunctionFunctionType {
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                })),
                            },
                            arguments: data::Storage::Static(&[
                                data::type_::ValueShapeId(1),
                            ]),
                            return_: data::type_::FunctionShape {
                                shape_id: data::type_::ValueShapeId(2),
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                },
                            },
                        },
                    }))),
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
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }), data::type_::ExternalTypeId(0)),
                    ]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[
                        data::host::NativeConversionId(0),
                        data::host::NativeConversionId(1),
                    ]),
                    nodes: data::Storage::Static(&[
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                        package: data::Text::Static("application"),
                                        module: data::Text::Static("function_views"),
                                        name: data::Text::Static("Erased"),
                                        arguments: data::Storage::Static(&[]),
                                    }),
                                ]),
                                return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                    arguments: data::Storage::Static(&[
                                        data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                            package: data::Text::Static("application"),
                                            module: data::Text::Static("function_views"),
                                            name: data::Text::Static("Erased"),
                                            arguments: data::Storage::Static(&[]),
                                        }),
                                    ]),
                                    return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                        package: data::Text::Static("application"),
                                        module: data::Text::Static("function_views"),
                                        name: data::Text::Static("Erased"),
                                        arguments: data::Storage::Static(&[]),
                                    })),
                                })),
                            }),
                            kind: data::host::NativeConversionKind::Function(data::Storage::Static(&[
                                data::host::NativeFunctionView {
                                    source: data::type_::FunctionMetadata {
                                        arguments: data::Storage::Static(&[
                                            data::type_::TypeMetadata::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                            arguments: data::Storage::Static(&[
                                                data::type_::TypeMetadata::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                                        })),
                                    },
                                    target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Function {
                                        id: data::function::RuntimeFunctionFunctionTarget::External(data::graph::ExternalFunctionCallTarget::Function(data::function::ExternalFunctionFunctionId {
                                            index: 1,
                                            type_: data::type_::ExternalFunctionType {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                },
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueShapeId(0),
                                                ]),
                                                return_: data::type_::ExternalTypeId(0),
                                            },
                                        })),
                                        return_type: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                        },
                                    }),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                        })),
                                    },
                                    captures: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                                                id: data::graph::CoreFunctionFunctionLocalId(0),
                                                type_: data::type_::FunctionFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        })),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(1),
                                                    ]),
                                                    return_: data::type_::FunctionShape {
                                                        shape_id: data::type_::ValueShapeId(2),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        },
                                                    },
                                                },
                                            })),
                                            shape: data::type_::ValueShapeId(13),
                                        },
                                    ]),
                                    host: 9,
                                    host_value: true,
                                },
                                data::host::NativeFunctionView {
                                    source: data::type_::FunctionMetadata {
                                        arguments: data::Storage::Static(&[
                                            data::type_::TypeMetadata::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                                    },
                                    target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Function {
                                        id: data::function::RuntimeFunctionFunctionTarget::External(data::graph::ExternalFunctionCallTarget::Function(data::function::ExternalFunctionFunctionId {
                                            index: 2,
                                            type_: data::type_::ExternalFunctionType {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                },
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueShapeId(0),
                                                ]),
                                                return_: data::type_::ExternalTypeId(0),
                                            },
                                        })),
                                        return_type: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                        },
                                    }),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                        })),
                                    },
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
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                    ]),
                                    host: 10,
                                    host_value: true,
                                },
                                data::host::NativeFunctionView {
                                    source: data::type_::FunctionMetadata {
                                        arguments: data::Storage::Static(&[
                                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                                package: data::Text::Static("application"),
                                                module: data::Text::Static("function_views"),
                                                name: data::Text::Static("Erased"),
                                                arguments: data::Storage::Static(&[]),
                                            }),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                            package: data::Text::Static("application"),
                                            module: data::Text::Static("function_views"),
                                            name: data::Text::Static("Erased"),
                                            arguments: data::Storage::Static(&[]),
                                        })),
                                    },
                                    target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Function {
                                        id: data::function::RuntimeFunctionFunctionTarget::External(data::graph::ExternalFunctionCallTarget::Function(data::function::ExternalFunctionFunctionId {
                                            index: 3,
                                            type_: data::type_::ExternalFunctionType {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                },
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueShapeId(0),
                                                ]),
                                                return_: data::type_::ExternalTypeId(0),
                                            },
                                        })),
                                        return_type: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                        },
                                    }),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                        })),
                                    },
                                    captures: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                                                id: data::graph::ExternalFunctionLocalId(0),
                                                type_: data::type_::ExternalFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::ExternalTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                    ]),
                                    host: 11,
                                    host_value: true,
                                },
                            ])),
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                            kind: data::host::NativeConversionKind::External {
                                rule: 0,
                            },
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Int,
                            kind: data::host::NativeConversionKind::Int,
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                        package: data::Text::Static("application"),
                                        module: data::Text::Static("function_views"),
                                        name: data::Text::Static("Erased"),
                                        arguments: data::Storage::Static(&[]),
                                    }),
                                ]),
                                return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                    package: data::Text::Static("application"),
                                    module: data::Text::Static("function_views"),
                                    name: data::Text::Static("Erased"),
                                    arguments: data::Storage::Static(&[]),
                                })),
                            }),
                            kind: data::host::NativeConversionKind::Function(data::Storage::Static(&[
                                data::host::NativeFunctionView {
                                    source: data::type_::FunctionMetadata {
                                        arguments: data::Storage::Static(&[
                                            data::type_::TypeMetadata::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                            arguments: data::Storage::Static(&[
                                                data::type_::TypeMetadata::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                                        })),
                                    },
                                    target: data::function::ProfiledRuntimeFunctionId::External(data::function::ExternalFunctionId {
                                        index: 5,
                                        return_type: data::type_::ExternalTypeId(0),
                                    }),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                    },
                                    captures: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                                                id: data::graph::CoreFunctionFunctionLocalId(0),
                                                type_: data::type_::FunctionFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        })),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(1),
                                                    ]),
                                                    return_: data::type_::FunctionShape {
                                                        shape_id: data::type_::ValueShapeId(2),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        },
                                                    },
                                                },
                                            })),
                                            shape: data::type_::ValueShapeId(13),
                                        },
                                    ]),
                                    host: 12,
                                    host_value: true,
                                },
                                data::host::NativeFunctionView {
                                    source: data::type_::FunctionMetadata {
                                        arguments: data::Storage::Static(&[
                                            data::type_::TypeMetadata::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                                    },
                                    target: data::function::ProfiledRuntimeFunctionId::External(data::function::ExternalFunctionId {
                                        index: 6,
                                        return_type: data::type_::ExternalTypeId(0),
                                    }),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                    },
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
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                    ]),
                                    host: 13,
                                    host_value: true,
                                },
                                data::host::NativeFunctionView {
                                    source: data::type_::FunctionMetadata {
                                        arguments: data::Storage::Static(&[
                                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                                package: data::Text::Static("application"),
                                                module: data::Text::Static("function_views"),
                                                name: data::Text::Static("Erased"),
                                                arguments: data::Storage::Static(&[]),
                                            }),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                            arguments: data::Storage::Static(&[
                                                data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                                    package: data::Text::Static("application"),
                                                    module: data::Text::Static("function_views"),
                                                    name: data::Text::Static("Erased"),
                                                    arguments: data::Storage::Static(&[]),
                                                }),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                                package: data::Text::Static("application"),
                                                module: data::Text::Static("function_views"),
                                                name: data::Text::Static("Erased"),
                                                arguments: data::Storage::Static(&[]),
                                            })),
                                        })),
                                    },
                                    target: data::function::ProfiledRuntimeFunctionId::External(data::function::ExternalFunctionId {
                                        index: 7,
                                        return_type: data::type_::ExternalTypeId(0),
                                    }),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                    },
                                    captures: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::External(data::graph::ExternalFunctionFunctionLocal {
                                                id: data::graph::ExternalFunctionFunctionLocalId(0),
                                                type_: data::type_::FunctionFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                        })),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::FunctionShape {
                                                        shape_id: data::type_::ValueShapeId(3),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                                        },
                                                    },
                                                },
                                            })),
                                            shape: data::type_::ValueShapeId(14),
                                        },
                                    ]),
                                    host: 14,
                                    host_value: true,
                                },
                            ])),
                        },
                    ]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        })),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                    })),
                })),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::ExternalFunction,
                index: 1,
            }),
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        arguments: data::Storage::Static(&[]),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        arguments: data::Storage::Static(&[]),
                    })),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                    package: data::Text::Static("application"),
                                    module: data::Text::Static("function_views"),
                                    name: data::Text::Static("Erased"),
                                    arguments: data::Storage::Static(&[]),
                                }),
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            })),
                        })),
                    }),
                    shape: data::type_::ValueShapeId(14),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                        })),
                    }),
                    shape: data::type_::ValueShapeId(13),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    })),
                ]),
                captures: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                            id: data::graph::CoreFunctionFunctionLocalId(0),
                            type_: data::type_::FunctionFunctionType {
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    })),
                                },
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueShapeId(1),
                                ]),
                                return_: data::type_::FunctionShape {
                                    shape_id: data::type_::ValueShapeId(2),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    },
                                },
                            },
                        })),
                        shape: data::type_::ValueShapeId(13),
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
                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                })),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: Some(data::host::HostNativeView {
                parent: 8,
                source: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                    })),
                },
                parent_value: true,
                arguments: data::Storage::Static(&[
                    data::host::NativeConversionId(2),
                ]),
                return_: data::host::NativeConversionId(3),
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::ExternalFunction,
                index: 2,
            }),
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        arguments: data::Storage::Static(&[]),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        arguments: data::Storage::Static(&[]),
                    })),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                    package: data::Text::Static("application"),
                                    module: data::Text::Static("function_views"),
                                    name: data::Text::Static("Erased"),
                                    arguments: data::Storage::Static(&[]),
                                }),
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            })),
                        })),
                    }),
                    shape: data::type_::ValueShapeId(14),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                        })),
                    }),
                    shape: data::type_::ValueShapeId(13),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    })),
                ]),
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
                        shape: data::type_::ValueShapeId(2),
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
                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                })),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: Some(data::host::HostNativeView {
                parent: 8,
                source: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                parent_value: true,
                arguments: data::Storage::Static(&[
                    data::host::NativeConversionId(2),
                ]),
                return_: data::host::NativeConversionId(3),
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::ExternalFunction,
                index: 3,
            }),
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        arguments: data::Storage::Static(&[]),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        arguments: data::Storage::Static(&[]),
                    })),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                    package: data::Text::Static("application"),
                                    module: data::Text::Static("function_views"),
                                    name: data::Text::Static("Erased"),
                                    arguments: data::Storage::Static(&[]),
                                }),
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            })),
                        })),
                    }),
                    shape: data::type_::ValueShapeId(14),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                        })),
                    }),
                    shape: data::type_::ValueShapeId(13),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    })),
                ]),
                captures: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::ExternalFunction(data::graph::ExternalFunctionLocal {
                            id: data::graph::ExternalFunctionLocalId(0),
                            type_: data::type_::ExternalFunctionType {
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                },
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueShapeId(0),
                                ]),
                                return_: data::type_::ExternalTypeId(0),
                            },
                        }),
                        shape: data::type_::ValueShapeId(3),
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
                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                })),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: Some(data::host::HostNativeView {
                parent: 8,
                source: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        arguments: data::Storage::Static(&[]),
                    })),
                },
                parent_value: true,
                arguments: data::Storage::Static(&[
                    data::host::NativeConversionId(1),
                ]),
                return_: data::host::NativeConversionId(3),
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::External,
                index: 5,
            }),
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        arguments: data::Storage::Static(&[]),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                    package: data::Text::Static("application"),
                    module: data::Text::Static("function_views"),
                    name: data::Text::Static("Erased"),
                    arguments: data::Storage::Static(&[]),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                    package: data::Text::Static("application"),
                                    module: data::Text::Static("function_views"),
                                    name: data::Text::Static("Erased"),
                                    arguments: data::Storage::Static(&[]),
                                }),
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            })),
                        })),
                    }),
                    shape: data::type_::ValueShapeId(14),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                        })),
                    }),
                    shape: data::type_::ValueShapeId(13),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    })),
                ]),
                captures: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                            id: data::graph::CoreFunctionFunctionLocalId(0),
                            type_: data::type_::FunctionFunctionType {
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    })),
                                },
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueShapeId(1),
                                ]),
                                return_: data::type_::FunctionShape {
                                    shape_id: data::type_::ValueShapeId(2),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    },
                                },
                            },
                        })),
                        shape: data::type_::ValueShapeId(13),
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
                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: Some(data::host::HostNativeView {
                parent: 8,
                source: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                    })),
                },
                parent_value: true,
                arguments: data::Storage::Static(&[
                    data::host::NativeConversionId(2),
                ]),
                return_: data::host::NativeConversionId(1),
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::External,
                index: 6,
            }),
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        arguments: data::Storage::Static(&[]),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                    package: data::Text::Static("application"),
                    module: data::Text::Static("function_views"),
                    name: data::Text::Static("Erased"),
                    arguments: data::Storage::Static(&[]),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                    package: data::Text::Static("application"),
                                    module: data::Text::Static("function_views"),
                                    name: data::Text::Static("Erased"),
                                    arguments: data::Storage::Static(&[]),
                                }),
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            })),
                        })),
                    }),
                    shape: data::type_::ValueShapeId(14),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                        })),
                    }),
                    shape: data::type_::ValueShapeId(13),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    })),
                ]),
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
                        shape: data::type_::ValueShapeId(2),
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
                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: Some(data::host::HostNativeView {
                parent: 8,
                source: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                parent_value: true,
                arguments: data::Storage::Static(&[
                    data::host::NativeConversionId(2),
                ]),
                return_: data::host::NativeConversionId(1),
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::External,
                index: 7,
            }),
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        arguments: data::Storage::Static(&[]),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                    package: data::Text::Static("application"),
                    module: data::Text::Static("function_views"),
                    name: data::Text::Static("Erased"),
                    arguments: data::Storage::Static(&[]),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                    package: data::Text::Static("application"),
                                    module: data::Text::Static("function_views"),
                                    name: data::Text::Static("Erased"),
                                    arguments: data::Storage::Static(&[]),
                                }),
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            })),
                        })),
                    }),
                    shape: data::type_::ValueShapeId(14),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                        })),
                    }),
                    shape: data::type_::ValueShapeId(13),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    })),
                ]),
                captures: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::External(data::graph::ExternalFunctionFunctionLocal {
                            id: data::graph::ExternalFunctionFunctionLocalId(0),
                            type_: data::type_::FunctionFunctionType {
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                    })),
                                },
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueShapeId(0),
                                ]),
                                return_: data::type_::FunctionShape {
                                    shape_id: data::type_::ValueShapeId(3),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
                                    },
                                },
                            },
                        })),
                        shape: data::type_::ValueShapeId(14),
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
                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: Some(data::host::HostNativeView {
                parent: 8,
                source: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        })),
                    })),
                },
                parent_value: true,
                arguments: data::Storage::Static(&[
                    data::host::NativeConversionId(1),
                ]),
                return_: data::host::NativeConversionId(1),
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::BitArray,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::String),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::String,
                    shape: data::type_::ValueShapeId(16),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::BitArray,
                    shape: data::type_::ValueShapeId(15),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0))),
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
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }), data::type_::ExternalTypeId(0)),
                    ]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[
                        data::host::NativeConversionId(0),
                        data::host::NativeConversionId(1),
                    ]),
                    nodes: data::Storage::Static(&[
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::String,
                            kind: data::host::NativeConversionKind::String,
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                            kind: data::host::NativeConversionKind::External {
                                rule: 0,
                            },
                        },
                    ]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::BitArray,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::String),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::String,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::BitArray,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::BitArray),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::BitArray,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::BitArray),
                    }),
                    shape: data::type_::ValueShapeId(18),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::String,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                    }),
                    shape: data::type_::ValueShapeId(17),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::StringFunction {
                        local: data::graph::StringFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::String,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::String),
                        },
                    }),
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
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }), data::type_::ExternalTypeId(0)),
                    ]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[
                        data::host::NativeConversionId(0),
                        data::host::NativeConversionId(1),
                    ]),
                    nodes: data::Storage::Static(&[
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::BitArray,
                                ]),
                                return_: data::Storage::Static(&data::type_::TypeMetadata::BitArray),
                            }),
                            kind: data::host::NativeConversionKind::Function(data::Storage::Static(&[
                                data::host::NativeFunctionView {
                                    source: data::type_::FunctionMetadata {
                                        arguments: data::Storage::Static(&[
                                            data::type_::TypeMetadata::String,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                                    },
                                    target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::BitArray(data::function::BitArrayFunctionId(0))),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::BitArray,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                    },
                                    captures: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::StringFunction {
                                                local: data::graph::StringFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                    ]),
                                    host: 17,
                                    host_value: true,
                                },
                            ])),
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                            kind: data::host::NativeConversionKind::External {
                                rule: 0,
                            },
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::String,
                            kind: data::host::NativeConversionKind::String,
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::BitArray,
                            kind: data::host::NativeConversionKind::BitArray,
                        },
                    ]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::String,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::String),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::BitArray,
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                })),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::BitArray,
                index: 0,
            }),
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::BitArray,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::BitArray),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::BitArray,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::BitArray),
                    }),
                    shape: data::type_::ValueShapeId(18),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::String,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                    }),
                    shape: data::type_::ValueShapeId(17),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0))),
                ]),
                captures: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::StringFunction {
                            local: data::graph::StringFunctionLocalId(0),
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::String,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::String),
                            },
                        },
                        shape: data::type_::ValueShapeId(17),
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
                    data::type_::ValueType::BitArray,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::BitArray),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: Some(data::host::HostNativeView {
                parent: 16,
                source: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                },
                parent_value: true,
                arguments: data::Storage::Static(&[
                    data::host::NativeConversionId(2),
                ]),
                return_: data::host::NativeConversionId(3),
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::BitArray,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::BitArray),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::String,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                    }),
                    shape: data::type_::ValueShapeId(17),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::BitArray,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::BitArray),
                    }),
                    shape: data::type_::ValueShapeId(18),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::BitArrayFunction {
                        local: data::graph::BitArrayFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::BitArray,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                        },
                    }),
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
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }), data::type_::ExternalTypeId(0)),
                    ]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[
                        data::host::NativeConversionId(0),
                        data::host::NativeConversionId(1),
                    ]),
                    nodes: data::Storage::Static(&[
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::String,
                                ]),
                                return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                            }),
                            kind: data::host::NativeConversionKind::Function(data::Storage::Static(&[
                                data::host::NativeFunctionView {
                                    source: data::type_::FunctionMetadata {
                                        arguments: data::Storage::Static(&[
                                            data::type_::TypeMetadata::BitArray,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::TypeMetadata::BitArray),
                                    },
                                    target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::String(data::function::StringFunctionId(2))),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::String,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::String),
                                    },
                                    captures: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArrayFunction {
                                                local: data::graph::BitArrayFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::BitArray,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(18),
                                        },
                                    ]),
                                    host: 19,
                                    host_value: true,
                                },
                            ])),
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                            kind: data::host::NativeConversionKind::External {
                                rule: 0,
                            },
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::BitArray,
                            kind: data::host::NativeConversionKind::BitArray,
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::String,
                            kind: data::host::NativeConversionKind::String,
                        },
                    ]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::BitArray,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::String,
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::String),
                })),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::String,
                index: 2,
            }),
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::String,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::String),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::String,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                    }),
                    shape: data::type_::ValueShapeId(17),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::BitArray,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::BitArray),
                    }),
                    shape: data::type_::ValueShapeId(18),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::String(data::graph::StringLocalId(0))),
                ]),
                captures: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::BitArrayFunction {
                            local: data::graph::BitArrayFunctionLocalId(0),
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::BitArray,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                            },
                        },
                        shape: data::type_::ValueShapeId(18),
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
                    data::type_::ValueType::String,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::String),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: Some(data::host::HostNativeView {
                parent: 18,
                source: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::BitArray,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::BitArray),
                },
                parent_value: true,
                arguments: data::Storage::Static(&[
                    data::host::NativeConversionId(2),
                ]),
                return_: data::host::NativeConversionId(3),
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::String,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                    package: data::Text::Static("application"),
                    module: data::Text::Static("function_views"),
                    name: data::Text::Static("Erased"),
                    arguments: data::Storage::Static(&[]),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        arguments: data::Storage::Static(&[]),
                    }),
                    shape: data::type_::ValueShapeId(0),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::String,
                    shape: data::type_::ValueShapeId(16),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::String(data::graph::StringLocalId(0))),
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
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }), data::type_::ExternalTypeId(0)),
                    ]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[
                        data::host::NativeConversionId(0),
                        data::host::NativeConversionId(0),
                    ]),
                    nodes: data::Storage::Static(&[
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                            kind: data::host::NativeConversionKind::External {
                                rule: 0,
                            },
                        },
                    ]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::String,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0))),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Empty"),
                        arguments: data::Storage::Static(&[]),
                    })),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Empty"),
                            arguments: data::Storage::Static(&[]),
                        })),
                    }),
                    shape: data::type_::ValueShapeId(21),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0))),
                    }),
                    shape: data::type_::ValueShapeId(23),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::NeverFunction(data::graph::NeverFunctionLocal {
                        id: data::graph::NeverFunctionLocalId(0),
                        type_: data::type_::GenericFunctionType {
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                            },
                            shape: data::type_::FunctionShape {
                                shape_id: data::type_::ValueShapeId(23),
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                },
                            },
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
                    entries: data::Storage::Static(&[]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }), data::type_::ExternalTypeId(0)),
                    ]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[
                        data::host::NativeConversionId(0),
                        data::host::NativeConversionId(1),
                    ]),
                    nodes: data::Storage::Static(&[
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                        package: data::Text::Static("application"),
                                        module: data::Text::Static("function_views"),
                                        name: data::Text::Static("Erased"),
                                        arguments: data::Storage::Static(&[]),
                                    }),
                                ]),
                                return_: data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                    package: data::Text::Static("application"),
                                    module: data::Text::Static("function_views"),
                                    name: data::Text::Static("Empty"),
                                    arguments: data::Storage::Static(&[]),
                                })),
                            }),
                            kind: data::host::NativeConversionKind::Function(data::Storage::Static(&[
                                data::host::NativeFunctionView {
                                    source: data::type_::FunctionMetadata {
                                        arguments: data::Storage::Static(&[
                                            data::type_::TypeMetadata::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0))),
                                    },
                                    target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Never(data::function::NeverFunctionId(1))),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(4))),
                                    },
                                    captures: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::NeverFunction(data::graph::NeverFunctionLocal {
                                                id: data::graph::NeverFunctionLocalId(0),
                                                type_: data::type_::GenericFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                    },
                                                    shape: data::type_::FunctionShape {
                                                        shape_id: data::type_::ValueShapeId(23),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                        },
                                                    },
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(23),
                                        },
                                    ]),
                                    host: 0,
                                    host_value: false,
                                },
                            ])),
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                            kind: data::host::NativeConversionKind::External {
                                rule: 0,
                            },
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Int,
                            kind: data::host::NativeConversionKind::Int,
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Empty"),
                                arguments: data::Storage::Static(&[]),
                            }),
                            kind: data::host::NativeConversionKind::Exact,
                        },
                    ]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(4))),
                })),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                    package: data::Text::Static("application"),
                    module: data::Text::Static("function_views"),
                    name: data::Text::Static("Erased"),
                    arguments: data::Storage::Static(&[]),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        arguments: data::Storage::Static(&[]),
                    }),
                    shape: data::type_::ValueShapeId(0),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Int,
                    shape: data::type_::ValueShapeId(1),
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
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }), data::type_::ExternalTypeId(0)),
                    ]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[
                        data::host::NativeConversionId(0),
                        data::host::NativeConversionId(0),
                    ]),
                    nodes: data::Storage::Static(&[
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                            kind: data::host::NativeConversionKind::External {
                                rule: 0,
                            },
                        },
                    ]),
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
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        arguments: data::Storage::Static(&[]),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Int,
                    shape: data::type_::ValueShapeId(1),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        arguments: data::Storage::Static(&[]),
                    }),
                    shape: data::type_::ValueShapeId(0),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    })),
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
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            arguments: data::Storage::Static(&[]),
                        }), data::type_::ExternalTypeId(0)),
                    ]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[
                        data::host::NativeConversionId(0),
                        data::host::NativeConversionId(1),
                    ]),
                    nodes: data::Storage::Static(&[
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Int,
                            kind: data::host::NativeConversionKind::Int,
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                            kind: data::host::NativeConversionKind::External {
                                rule: 0,
                            },
                        },
                    ]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Int),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "tick", data::source::SourceSpan::new(269, 278)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            type_arguments: data::Storage::Static(&[]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[]),
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
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::Int),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 0,
                parameters: data::Storage::Static(&[]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Int,
                layout: data::Storage::Static(&[]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
    ]),
    never_functions: data::Storage::Static(&[
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Uninhabited,
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::Never,
                index: 1,
            }),
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_views", "coerce", data::source::SourceSpan::new(206, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        arguments: data::Storage::Static(&[]),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                    package: data::Text::Static("application"),
                    module: data::Text::Static("function_views"),
                    name: data::Text::Static("Empty"),
                    arguments: data::Storage::Static(&[]),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("function_views"),
                                name: data::Text::Static("Erased"),
                                arguments: data::Storage::Static(&[]),
                            }),
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Empty"),
                            arguments: data::Storage::Static(&[]),
                        })),
                    }),
                    shape: data::type_::ValueShapeId(21),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0))),
                    }),
                    shape: data::type_::ValueShapeId(23),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(0),
                    })),
                ]),
                captures: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::NeverFunction(data::graph::NeverFunctionLocal {
                            id: data::graph::NeverFunctionLocalId(0),
                            type_: data::type_::GenericFunctionType {
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                },
                                shape: data::type_::FunctionShape {
                                    shape_id: data::type_::ValueShapeId(23),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                    },
                                },
                            },
                        }),
                        shape: data::type_::ValueShapeId(23),
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
                    data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(4))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_views"),
                        name: data::Text::Static("Erased"),
                        parameter_count: 0,
                    },
                ]),
                native_rules: Some(data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_views"),
                            name: data::Text::Static("Erased"),
                            parameter_count: 0,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                ])),
                native_sources: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                ]),
            }),
            native_view: Some(data::host::HostNativeView {
                parent: 21,
                source: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0))),
                },
                parent_value: true,
                arguments: data::Storage::Static(&[
                    data::host::NativeConversionId(2),
                ]),
                return_: data::host::NativeConversionId(3),
            }),
        },
    ]),
    callables: data::Storage::Static(&[]),
}
