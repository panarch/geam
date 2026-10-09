data::HostedModuleArtifact {
    module: data::ModuleArtifact {
        format: 29,
        program: data::ProgramTables {
            root: data::source::module_id(0),
            modules: data::Storage::Static(&[
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("string_native"),
                    source_context: Some(data::source::SourceContext::from_static_block("src/string_native.gleam", r#"
pub type Direction {
  Before
  After
}

@external(erlang, "native", "join")
fn join(value: String, direction: Direction, index: Int) -> String

@external(erlang, "native", "later")
fn later(value: String) -> String

@external(erlang, "native", "stop")
fn stop(value: String) -> String

pub fn ordinary(value: String) -> String {
  let first = join(value, Before, 1)
  let second = join(first, After, 2)
  second
}

pub fn tail(value: String) -> String {
  join(value, Before, 3)
}

fn forward(value: String, direction: Direction) -> String {
  join(value, direction, 4)
}

pub fn nested_tail(value: String) -> String {
  let first = forward(value, Before)
  let second = join(first, After, 5)
  second
}

pub fn after_continuing(value: String) -> String {
  let first = join(value, Before, 6)
  let continued = later(first)
  let second = join(continued, After, 7)
  second
}

pub fn after_never(value: String) -> String {
  let first = join(value, Before, 8)
  let stopped = stop(first)
  join(stopped, After, 9)
}

pub fn source_caller(value: String) -> #(String, String) {
  echo value
  let result = ordinary(value)
  #(result <> "done", "caller")
}

pub fn source_string_caller(value: String) -> String {
  echo value
  ordinary(value) <> "done"
}
"#)),
                },
            ]),
            main: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::String(data::function::StringFunctionId(0))),
            functions: data::function::FunctionTables {
                value_returns: data::function::ValueFunctionTables {
                    never_functions: data::Storage::Static(&[]),
                    int_functions: data::Storage::Static(&[]),
                    float_functions: data::Storage::Static(&[]),
                    string_functions: data::Storage::Static(&[
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
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[]),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(2),
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
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                                function: data::function::StringFunctionId(6),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(0),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("string_native", "ordinary", data::source::SourceSpan::new(344, 366)),
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
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[]),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(2),
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
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                                function: data::function::StringFunctionId(6),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(1),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(1),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("string_native", "ordinary", data::source::SourceSpan::new(382, 403)),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::StringLocalId(2)),
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
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[]),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    3,
                                                ]),
                                            })),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::StringFunctionId(6),
                                            site: data::source::HostCallSite::from_static("string_native", "tail", data::source::SourceSpan::new(457, 479)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                            instructions: 0..5,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[]),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                                function: data::function::StringFunctionId(7),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(0),
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("string_native", "nested_tail", data::source::SourceSpan::new(634, 656)),
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
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[]),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    5,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                                function: data::function::StringFunctionId(6),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(1),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(1),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("string_native", "nested_tail", data::source::SourceSpan::new(672, 693)),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::StringLocalId(2)),
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
                                            instructions: 0..7,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[]),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    6,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                                function: data::function::StringFunctionId(6),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(0),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("string_native", "after_continuing", data::source::SourceSpan::new(771, 793)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                                function: data::function::StringFunctionId(8),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("string_native", "after_continuing", data::source::SourceSpan::new(812, 824)),
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
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[]),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    7,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(3)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                                function: data::function::StringFunctionId(6),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(1),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(1),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("string_native", "after_continuing", data::source::SourceSpan::new(840, 865)),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::StringLocalId(3)),
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
                                            instructions: 0..6,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[]),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    8,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                                function: data::function::StringFunctionId(6),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(0),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("string_native", "after_never", data::source::SourceSpan::new(938, 960)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                                function: data::function::StringFunctionId(9),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("string_native", "after_never", data::source::SourceSpan::new(977, 988)),
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
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[]),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    9,
                                                ]),
                                            })),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::StringFunctionId(6),
                                            site: data::source::HostCallSite::from_static("string_native", "after_never", data::source::SourceSpan::new(991, 1014)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(1),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                                    family: data::graph::StorageFamily::String,
                                                    length: 1,
                                                    steps: data::Storage::Static(&[
                                                        data::graph::TransferStep {
                                                            source: 2,
                                                            destination: 0,
                                                        },
                                                    ]),
                                                },
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::Custom,
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
                                            terminator: data::graph::Terminator::Echo(data::graph::Echo {
                                                subject: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                message: None,
                                                site: data::source::EchoSite::from_static("string_native", "source_string_caller", data::source::SourceSpan::new(1213, 1223)),
                                                next: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 1..2,
                                            instructions: 0..3,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                                function: data::function::StringFunctionId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("string_native", "source_string_caller", data::source::SourceSpan::new(1226, 1241)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("done"))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(3)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Concatenate {
                                                left: data::graph::StringLocalId(1),
                                                right: data::graph::StringLocalId(2),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::StringLocalId(3)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 0,
                            return_: data::graph::StringLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
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
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(2),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    4,
                                                ]),
                                            })),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::StringFunctionId(6),
                                            site: data::source::HostCallSite::from_static("string_native", "forward", data::source::SourceSpan::new(545, 570)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(2),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[]),
                                        },
                                    },
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 1,
                            return_: data::graph::StringLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Never(data::host::HostNeverFunctionId(0))),
                    ]),
                    bit_array_functions: data::Storage::Static(&[]),
                    utf_codepoint_functions: data::Storage::Static(&[]),
                    custom_functions: data::Storage::Static(&[]),
                    external_functions: data::Storage::Static(&[]),
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
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Echo(data::graph::Echo {
                                                subject: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                message: None,
                                                site: data::source::EchoSite::from_static("string_native", "source_caller", data::source::SourceSpan::new(1079, 1089)),
                                                next: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 1..2,
                                            instructions: 0..5,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                                function: data::function::StringFunctionId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("string_native", "source_caller", data::source::SourceSpan::new(1105, 1120)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("done"))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(3)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Concatenate {
                                                left: data::graph::StringLocalId(1),
                                                right: data::graph::StringLocalId(2),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(4)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("caller"))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                        data::type_::ValueType::String,
                                                    ]),
                                                },
                                                shape: data::type_::ValueShapeId(4),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(3)),
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(4)),
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
                    int_function_functions: data::Storage::Static(&[]),
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
            compiled: {
                const CALL_GROUP_0: [data::compiled::calls::CallStart; 6] = {
                    use data::compiled::calls::{CallArguments, CallExecution, CallInputs, CallNullary, CallOps, CallOutput, CallProgress, CallStorage, CallValues, StringNativeExecution, StringNativeRequest, StringValue};
                    enum FunctionState {
                        String0Point0 { string0: StringValue },
                        String0Point1 { string0: StringValue, nullary0: CallNullary },
                        String0Point2 { string0: StringValue, nullary0: CallNullary, int0: i128 },
                        String0Point3 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue },
                        String0Point4 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue, nullary1: CallNullary },
                        String0Point5 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue, nullary1: CallNullary, int1: i128 },
                        String0Point6 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue, nullary1: CallNullary, int1: i128, string2: StringValue },
                        String1Point0 { string0: StringValue },
                        String1Point1 { string0: StringValue, nullary0: CallNullary },
                        String1Point2 { string0: StringValue, nullary0: CallNullary, int0: i128 },
                        String2Point0 { string0: StringValue },
                        String2Point1 { string0: StringValue, nullary0: CallNullary },
                        String2Point2 { string0: StringValue, nullary0: CallNullary, string1: StringValue },
                        String2Point3 { string0: StringValue, nullary0: CallNullary, string1: StringValue, nullary1: CallNullary },
                        String2Point4 { string0: StringValue, nullary0: CallNullary, string1: StringValue, nullary1: CallNullary, int0: i128 },
                        String2Point5 { string0: StringValue, nullary0: CallNullary, string1: StringValue, nullary1: CallNullary, int0: i128, string2: StringValue },
                        String3Point0 { string0: StringValue },
                        String3Point1 { string0: StringValue, nullary0: CallNullary },
                        String3Point2 { string0: StringValue, nullary0: CallNullary, int0: i128 },
                        String3Point3 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue },
                        String3Point4 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue, string2: StringValue },
                        String3Point5 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue, string2: StringValue, nullary1: CallNullary },
                        String3Point6 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue, string2: StringValue, nullary1: CallNullary, int1: i128 },
                        String3Point7 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue, string2: StringValue, nullary1: CallNullary, int1: i128, string3: StringValue },
                        String4Point0 { string0: StringValue },
                        String4Point1 { string0: StringValue, nullary0: CallNullary },
                        String4Point2 { string0: StringValue, nullary0: CallNullary, int0: i128 },
                        String4Point3 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue },
                        String4Point4 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue, string2: StringValue },
                        String4Point5 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue, string2: StringValue, nullary1: CallNullary },
                        String4Point6 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue, string2: StringValue, nullary1: CallNullary, int1: i128 },
                        String7Point0 { string0: StringValue, nullary0: CallNullary },
                        String7Point1 { string0: StringValue, nullary0: CallNullary, int0: i128 },
                    }
                    enum StringReturn {
                        String0Call2 { string0: StringValue, nullary0: CallNullary, int0: i128 },
                        String0Call5 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue, nullary1: CallNullary, int1: i128 },
                        String2Call1 { string0: StringValue, nullary0: CallNullary },
                        String2Call4 { string0: StringValue, nullary0: CallNullary, string1: StringValue, nullary1: CallNullary, int0: i128 },
                        String3Call2 { string0: StringValue, nullary0: CallNullary, int0: i128 },
                        String3Call3 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue },
                        String3Call6 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue, string2: StringValue, nullary1: CallNullary, int1: i128 },
                        String4Call2 { string0: StringValue, nullary0: CallNullary, int0: i128 },
                        String4Call3 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue },
                    }
                    impl StringReturn {
                        fn site(&self) -> data::source::HostCallSite {
                            match *self {
                                Self::String0Call2 { .. } => data::source::HostCallSite::from_static("string_native", "ordinary", data::source::SourceSpan::new(344, 366)),
                                Self::String0Call5 { .. } => data::source::HostCallSite::from_static("string_native", "ordinary", data::source::SourceSpan::new(382, 403)),
                                Self::String2Call1 { .. } => data::source::HostCallSite::from_static("string_native", "nested_tail", data::source::SourceSpan::new(634, 656)),
                                Self::String2Call4 { .. } => data::source::HostCallSite::from_static("string_native", "nested_tail", data::source::SourceSpan::new(672, 693)),
                                Self::String3Call2 { .. } => data::source::HostCallSite::from_static("string_native", "after_continuing", data::source::SourceSpan::new(771, 793)),
                                Self::String3Call3 { .. } => data::source::HostCallSite::from_static("string_native", "after_continuing", data::source::SourceSpan::new(812, 824)),
                                Self::String3Call6 { .. } => data::source::HostCallSite::from_static("string_native", "after_continuing", data::source::SourceSpan::new(840, 865)),
                                Self::String4Call2 { .. } => data::source::HostCallSite::from_static("string_native", "after_never", data::source::SourceSpan::new(938, 960)),
                                Self::String4Call3 { .. } => data::source::HostCallSite::from_static("string_native", "after_never", data::source::SourceSpan::new(977, 988)),
                            }
                        }
                        fn small(self, result: StringValue) -> FunctionState {
                            match self {
                                Self::String0Call2 { string0, nullary0, int0 } => {
                                    let string1 = result;
                                    FunctionState::String0Point3 { string0, nullary0, int0, string1 }
                                },
                                Self::String0Call5 { string0, nullary0, int0, string1, nullary1, int1 } => {
                                    let string2 = result;
                                    FunctionState::String0Point6 { string0, nullary0, int0, string1, nullary1, int1, string2 }
                                },
                                Self::String2Call1 { string0, nullary0 } => {
                                    let string1 = result;
                                    FunctionState::String2Point2 { string0, nullary0, string1 }
                                },
                                Self::String2Call4 { string0, nullary0, string1, nullary1, int0 } => {
                                    let string2 = result;
                                    FunctionState::String2Point5 { string0, nullary0, string1, nullary1, int0, string2 }
                                },
                                Self::String3Call2 { string0, nullary0, int0 } => {
                                    let string1 = result;
                                    FunctionState::String3Point3 { string0, nullary0, int0, string1 }
                                },
                                Self::String3Call3 { string0, nullary0, int0, string1 } => {
                                    let string2 = result;
                                    FunctionState::String3Point4 { string0, nullary0, int0, string1, string2 }
                                },
                                Self::String3Call6 { string0, nullary0, int0, string1, string2, nullary1, int1 } => {
                                    let string3 = result;
                                    FunctionState::String3Point7 { string0, nullary0, int0, string1, string2, nullary1, int1, string3 }
                                },
                                Self::String4Call2 { string0, nullary0, int0 } => {
                                    let string1 = result;
                                    FunctionState::String4Point3 { string0, nullary0, int0, string1 }
                                },
                                Self::String4Call3 { string0, nullary0, int0, string1 } => {
                                    let string2 = result;
                                    FunctionState::String4Point4 { string0, nullary0, int0, string1, string2 }
                                },
                            }
                        }
                        fn resume(self, result: StringValue) -> FunctionState { self.small(result) }
                    }
                    #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                    enum FunctionStep {
                        Yield(FunctionState),
                        StringNative { function: data::function::StringFunctionId, site: data::source::HostCallSite, arguments: Box<CallValues>, caller: Option<StringReturn> },
                        Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                        StringCall { callee: FunctionState, caller: StringReturn },
                        String { value: StringValue },
                        StringBridge { function: data::function::StringFunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: StringReturn },
                    }
                    struct FunctionExecution {
                        active: Option<FunctionState>,
                        native_caller: Option<StringReturn>,
                        native_result: Option<StringValue>,
                        string_returns: Vec<StringReturn>,
                    }
                    impl FunctionExecution {
                        fn new(active: FunctionState) -> Self {
                            Self {
                                active: Some(active),
                                native_caller: None,
                                native_result: None,
                                string_returns: Vec::new(),
                            }
                        }
                    }
                    impl CallExecution for FunctionExecution {
                        fn restart(&mut self, target: data::compiled::CallTarget, point: usize, values: CallInputs<'_>) -> bool {
                            if self.active.is_some() { return false; }
                            let active = match target {
                                data::compiled::CallTarget::String(data::function::StringFunctionId(0)) => calls_string_0_state(point, values),
                                data::compiled::CallTarget::String(data::function::StringFunctionId(1)) => calls_string_1_state(point, values),
                                data::compiled::CallTarget::String(data::function::StringFunctionId(2)) => calls_string_2_state(point, values),
                                data::compiled::CallTarget::String(data::function::StringFunctionId(3)) => calls_string_3_state(point, values),
                                data::compiled::CallTarget::String(data::function::StringFunctionId(4)) => calls_string_4_state(point, values),
                                data::compiled::CallTarget::String(data::function::StringFunctionId(7)) => calls_string_7_state(point, values),
                                _ => None,
                            };
                            let Some(active) = active else { return false; };
                            self.active = Some(active);
                            true
                        }
                        fn retained_bytes(&self) -> usize {
                            std::mem::size_of::<Self>() + self.string_returns.capacity() * std::mem::size_of::<StringReturn>()
                        }
                        fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                            if let Some(result) = self.native_result.take() {
                                if let Some(caller) = self.native_caller.take().or_else(|| self.string_returns.pop()) {
                                    self.active = Some(caller.small(result));
                                } else {
                                    self.string_returns.clear();
                                    return CallProgress::Complete { output: CallOutput::String(result), execution: self };
                                }
                            }
                            let Some(mut active) = self.active.take() else { return CallProgress::Yield(self); };
                            loop {
                                match function_step(active, ops, budget) {
                                    FunctionStep::StringNative { function, site, arguments, caller } => {
                                        let root_tail = caller.is_none() && self.string_returns.is_empty() && ops.root_tail_entry();
                                        self.native_caller = caller;
                                        return CallProgress::StringNative(StringNativeRequest { function, site, arguments, root_tail, execution: self });
                                    },
                                    FunctionStep::Yield(active) => {
                                        self.active = Some(active);
                                        return CallProgress::Yield(self);
                                    },
                                    FunctionStep::StringCall { callee, caller } => {
                                        self.string_returns.push(caller);
                                        active = callee;
                                    },
                                    FunctionStep::String { value } => {
                                        if let Some(caller) = self.string_returns.pop() {
                                            active = caller.small(value);
                                        } else {
                                            self.string_returns.clear();
                                            return CallProgress::Complete { output: CallOutput::String(value), execution: self };
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
                    impl StringNativeExecution for FunctionExecution {
                        fn resume_native(mut self: Box<Self>, value: StringValue) -> Box<dyn CallExecution> { self.native_result = Some(value); self }
                    }
                    fn function_step(active: FunctionState, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            FunctionState::String0Point0 { string0 } => calls_string_0_run(String0State::Point0 { string0 }, ops, budget),
                            FunctionState::String0Point1 { string0, nullary0 } => calls_string_0_run(String0State::Point1 { string0, nullary0 }, ops, budget),
                            FunctionState::String0Point2 { string0, nullary0, int0 } => calls_string_0_run(String0State::Point2 { string0, nullary0, int0 }, ops, budget),
                            FunctionState::String0Point3 { string0, nullary0, int0, string1 } => calls_string_0_run(String0State::Point3 { string0, nullary0, int0, string1 }, ops, budget),
                            FunctionState::String0Point4 { string0, nullary0, int0, string1, nullary1 } => calls_string_0_run(String0State::Point4 { string0, nullary0, int0, string1, nullary1 }, ops, budget),
                            FunctionState::String0Point5 { string0, nullary0, int0, string1, nullary1, int1 } => calls_string_0_run(String0State::Point5 { string0, nullary0, int0, string1, nullary1, int1 }, ops, budget),
                            FunctionState::String0Point6 { string0, nullary0, int0, string1, nullary1, int1, string2 } => calls_string_0_run(String0State::Point6 { string0, nullary0, int0, string1, nullary1, int1, string2 }, ops, budget),
                            FunctionState::String1Point0 { string0 } => calls_string_1_run(String1State::Point0 { string0 }, ops, budget),
                            FunctionState::String1Point1 { string0, nullary0 } => calls_string_1_run(String1State::Point1 { string0, nullary0 }, ops, budget),
                            FunctionState::String1Point2 { string0, nullary0, int0 } => calls_string_1_run(String1State::Point2 { string0, nullary0, int0 }, ops, budget),
                            FunctionState::String2Point0 { string0 } => calls_string_2_run(String2State::Point0 { string0 }, ops, budget),
                            FunctionState::String2Point1 { string0, nullary0 } => calls_string_2_run(String2State::Point1 { string0, nullary0 }, ops, budget),
                            FunctionState::String2Point2 { string0, nullary0, string1 } => calls_string_2_run(String2State::Point2 { string0, nullary0, string1 }, ops, budget),
                            FunctionState::String2Point3 { string0, nullary0, string1, nullary1 } => calls_string_2_run(String2State::Point3 { string0, nullary0, string1, nullary1 }, ops, budget),
                            FunctionState::String2Point4 { string0, nullary0, string1, nullary1, int0 } => calls_string_2_run(String2State::Point4 { string0, nullary0, string1, nullary1, int0 }, ops, budget),
                            FunctionState::String2Point5 { string0, nullary0, string1, nullary1, int0, string2 } => calls_string_2_run(String2State::Point5 { string0, nullary0, string1, nullary1, int0, string2 }, ops, budget),
                            FunctionState::String3Point0 { string0 } => calls_string_3_run(String3State::Point0 { string0 }, ops, budget),
                            FunctionState::String3Point1 { string0, nullary0 } => calls_string_3_run(String3State::Point1 { string0, nullary0 }, ops, budget),
                            FunctionState::String3Point2 { string0, nullary0, int0 } => calls_string_3_run(String3State::Point2 { string0, nullary0, int0 }, ops, budget),
                            FunctionState::String3Point3 { string0, nullary0, int0, string1 } => calls_string_3_run(String3State::Point3 { string0, nullary0, int0, string1 }, ops, budget),
                            FunctionState::String3Point4 { string0, nullary0, int0, string1, string2 } => calls_string_3_run(String3State::Point4 { string0, nullary0, int0, string1, string2 }, ops, budget),
                            FunctionState::String3Point5 { string0, nullary0, int0, string1, string2, nullary1 } => calls_string_3_run(String3State::Point5 { string0, nullary0, int0, string1, string2, nullary1 }, ops, budget),
                            FunctionState::String3Point6 { string0, nullary0, int0, string1, string2, nullary1, int1 } => calls_string_3_run(String3State::Point6 { string0, nullary0, int0, string1, string2, nullary1, int1 }, ops, budget),
                            FunctionState::String3Point7 { string0, nullary0, int0, string1, string2, nullary1, int1, string3 } => calls_string_3_run(String3State::Point7 { string0, nullary0, int0, string1, string2, nullary1, int1, string3 }, ops, budget),
                            FunctionState::String4Point0 { string0 } => calls_string_4_run(String4State::Point0 { string0 }, ops, budget),
                            FunctionState::String4Point1 { string0, nullary0 } => calls_string_4_run(String4State::Point1 { string0, nullary0 }, ops, budget),
                            FunctionState::String4Point2 { string0, nullary0, int0 } => calls_string_4_run(String4State::Point2 { string0, nullary0, int0 }, ops, budget),
                            FunctionState::String4Point3 { string0, nullary0, int0, string1 } => calls_string_4_run(String4State::Point3 { string0, nullary0, int0, string1 }, ops, budget),
                            FunctionState::String4Point4 { string0, nullary0, int0, string1, string2 } => calls_string_4_run(String4State::Point4 { string0, nullary0, int0, string1, string2 }, ops, budget),
                            FunctionState::String4Point5 { string0, nullary0, int0, string1, string2, nullary1 } => calls_string_4_run(String4State::Point5 { string0, nullary0, int0, string1, string2, nullary1 }, ops, budget),
                            FunctionState::String4Point6 { string0, nullary0, int0, string1, string2, nullary1, int1 } => calls_string_4_run(String4State::Point6 { string0, nullary0, int0, string1, string2, nullary1, int1 }, ops, budget),
                            FunctionState::String7Point0 { string0, nullary0 } => calls_string_7_run(String7State::Point0 { string0, nullary0 }, ops, budget),
                            FunctionState::String7Point1 { string0, nullary0, int0 } => calls_string_7_run(String7State::Point1 { string0, nullary0, int0 }, ops, budget),
                        }
                    }
                    enum String0State {
                        Point0 { string0: StringValue },
                        Point1 { string0: StringValue, nullary0: CallNullary },
                        Point2 { string0: StringValue, nullary0: CallNullary, int0: i128 },
                        Point3 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue },
                        Point4 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue, nullary1: CallNullary },
                        Point5 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue, nullary1: CallNullary, int1: i128 },
                        Point6 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue, nullary1: CallNullary, int1: i128, string2: StringValue },
                    }
                    fn calls_string_0_run(mut active: String0State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        loop {
                            match active {
                                String0State::Point0 { string0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String0Point0 { string0 }); }
                                    *budget -= 1;
                                    let nullary0 = CallNullary::new(data::type_::CustomConstructorId {
                                        type_id: data::type_::CustomTypeId(0),
                                        index: 0,
                                    });
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String0Point1 { string0, nullary0 }); }
                                    *budget -= 1;
                                    let int0 = 1_i128;
                                    if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0], ..CallValues::default() }) }; }
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String0Point2 { string0, nullary0, int0 }); }
                                    *budget -= 1;
                                    return {
                                        if ops.supports_string_native(data::function::StringFunctionId(6)) {
                                            return FunctionStep::StringNative { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "ordinary", data::source::SourceSpan::new(344, 366)), arguments: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0.clone()], ..CallValues::default() }), caller: Some(StringReturn::String0Call2 { string0, nullary0, int0 }) };
                                        }
                                        FunctionStep::StringBridge { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "ordinary", data::source::SourceSpan::new(344, 366)), arguments: CallArguments { values: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0.clone()], ..CallValues::default() }), captures: None }, caller: StringReturn::String0Call2 { string0, nullary0, int0 } }
                                    };
                                },
                                String0State::Point1 { string0, nullary0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String0Point1 { string0, nullary0 }); }
                                    *budget -= 1;
                                    let int0 = 1_i128;
                                    if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0], ..CallValues::default() }) }; }
                                    active = String0State::Point2 { string0, nullary0, int0 };
                                    continue;
                                },
                                String0State::Point2 { string0, nullary0, int0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String0Point2 { string0, nullary0, int0 }); }
                                    *budget -= 1;
                                    return {
                                        if ops.supports_string_native(data::function::StringFunctionId(6)) {
                                            return FunctionStep::StringNative { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "ordinary", data::source::SourceSpan::new(344, 366)), arguments: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0.clone()], ..CallValues::default() }), caller: Some(StringReturn::String0Call2 { string0, nullary0, int0 }) };
                                        }
                                        FunctionStep::StringBridge { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "ordinary", data::source::SourceSpan::new(344, 366)), arguments: CallArguments { values: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0.clone()], ..CallValues::default() }), captures: None }, caller: StringReturn::String0Call2 { string0, nullary0, int0 } }
                                    };
                                },
                                String0State::Point3 { string0, nullary0, int0, string1 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String0Point3 { string0, nullary0, int0, string1 }); }
                                    *budget -= 1;
                                    let nullary1 = CallNullary::new(data::type_::CustomConstructorId {
                                        type_id: data::type_::CustomTypeId(0),
                                        index: 1,
                                    });
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String0Point4 { string0, nullary0, int0, string1, nullary1 }); }
                                    *budget -= 1;
                                    let int1 = 2_i128;
                                    if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 5,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 2,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { nullaries: vec![nullary0, nullary1], ints: vec![int0.into(), int1.into()], strings: vec![string0, string1], ..CallValues::default() }) }; }
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String0Point5 { string0, nullary0, int0, string1, nullary1, int1 }); }
                                    *budget -= 1;
                                    return {
                                        if ops.supports_string_native(data::function::StringFunctionId(6)) {
                                            return FunctionStep::StringNative { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "ordinary", data::source::SourceSpan::new(382, 403)), arguments: Box::new(CallValues { nullaries: vec![nullary1], ints: vec![int1.into()], strings: vec![string1.clone()], ..CallValues::default() }), caller: Some(StringReturn::String0Call5 { string0, nullary0, int0, string1, nullary1, int1 }) };
                                        }
                                        FunctionStep::StringBridge { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "ordinary", data::source::SourceSpan::new(382, 403)), arguments: CallArguments { values: Box::new(CallValues { nullaries: vec![nullary1], ints: vec![int1.into()], strings: vec![string1.clone()], ..CallValues::default() }), captures: None }, caller: StringReturn::String0Call5 { string0, nullary0, int0, string1, nullary1, int1 } }
                                    };
                                },
                                String0State::Point4 { string0, nullary0, int0, string1, nullary1 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String0Point4 { string0, nullary0, int0, string1, nullary1 }); }
                                    *budget -= 1;
                                    let int1 = 2_i128;
                                    if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 5,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 2,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { nullaries: vec![nullary0, nullary1], ints: vec![int0.into(), int1.into()], strings: vec![string0, string1], ..CallValues::default() }) }; }
                                    active = String0State::Point5 { string0, nullary0, int0, string1, nullary1, int1 };
                                    continue;
                                },
                                String0State::Point5 { string0, nullary0, int0, string1, nullary1, int1 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String0Point5 { string0, nullary0, int0, string1, nullary1, int1 }); }
                                    *budget -= 1;
                                    return {
                                        if ops.supports_string_native(data::function::StringFunctionId(6)) {
                                            return FunctionStep::StringNative { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "ordinary", data::source::SourceSpan::new(382, 403)), arguments: Box::new(CallValues { nullaries: vec![nullary1], ints: vec![int1.into()], strings: vec![string1.clone()], ..CallValues::default() }), caller: Some(StringReturn::String0Call5 { string0, nullary0, int0, string1, nullary1, int1 }) };
                                        }
                                        FunctionStep::StringBridge { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "ordinary", data::source::SourceSpan::new(382, 403)), arguments: CallArguments { values: Box::new(CallValues { nullaries: vec![nullary1], ints: vec![int1.into()], strings: vec![string1.clone()], ..CallValues::default() }), captures: None }, caller: StringReturn::String0Call5 { string0, nullary0, int0, string1, nullary1, int1 } }
                                    };
                                },
                                String0State::Point6 { string0, nullary0, int0, string1, nullary1, int1, string2 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String0Point6 { string0, nullary0, int0, string1, nullary1, int1, string2 }); }
                                    *budget -= 1;
                                    return {
                                        FunctionStep::String { value: string2 }
                                    };
                                },
                            }
                        }
                    }
                    enum String1State {
                        Point0 { string0: StringValue },
                        Point1 { string0: StringValue, nullary0: CallNullary },
                        Point2 { string0: StringValue, nullary0: CallNullary, int0: i128 },
                    }
                    fn calls_string_1_run(mut active: String1State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        loop {
                            match active {
                                String1State::Point0 { string0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String1Point0 { string0 }); }
                                    *budget -= 1;
                                    let nullary0 = CallNullary::new(data::type_::CustomConstructorId {
                                        type_id: data::type_::CustomTypeId(0),
                                        index: 0,
                                    });
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String1Point1 { string0, nullary0 }); }
                                    *budget -= 1;
                                    let int0 = 3_i128;
                                    if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(1)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0], ..CallValues::default() }) }; }
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String1Point2 { string0, nullary0, int0 }); }
                                    return {
                                        if ops.supports_string_native(data::function::StringFunctionId(6)) {
                                            *budget -= 1;
                                            return FunctionStep::StringNative { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "tail", data::source::SourceSpan::new(457, 479)), arguments: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0.clone()], ..CallValues::default() }), caller: None };
                                        }
                                        FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(1)), point: data::compiled::CompiledCheckpoint {
                                            block: data::graph::BlockId(0),
                                            instruction: 2,
                                            ints: 1,
                                            bools: 0,
                                            bit_arrays: 0,
                                            int_lists: 0,
                                            strings: 1,
                                            customs: 1,
                                            custom_lists: 0,
                                            int_functions: 0,
                                            bool_functions: 0,
                                        }, values: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0], ..CallValues::default() }) }
                                    };
                                },
                                String1State::Point1 { string0, nullary0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String1Point1 { string0, nullary0 }); }
                                    *budget -= 1;
                                    let int0 = 3_i128;
                                    if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(1)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0], ..CallValues::default() }) }; }
                                    active = String1State::Point2 { string0, nullary0, int0 };
                                    continue;
                                },
                                String1State::Point2 { string0, nullary0, int0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String1Point2 { string0, nullary0, int0 }); }
                                    return {
                                        if ops.supports_string_native(data::function::StringFunctionId(6)) {
                                            *budget -= 1;
                                            return FunctionStep::StringNative { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "tail", data::source::SourceSpan::new(457, 479)), arguments: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0.clone()], ..CallValues::default() }), caller: None };
                                        }
                                        FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(1)), point: data::compiled::CompiledCheckpoint {
                                            block: data::graph::BlockId(0),
                                            instruction: 2,
                                            ints: 1,
                                            bools: 0,
                                            bit_arrays: 0,
                                            int_lists: 0,
                                            strings: 1,
                                            customs: 1,
                                            custom_lists: 0,
                                            int_functions: 0,
                                            bool_functions: 0,
                                        }, values: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0], ..CallValues::default() }) }
                                    };
                                },
                            }
                        }
                    }
                    enum String2State {
                        Point0 { string0: StringValue },
                        Point1 { string0: StringValue, nullary0: CallNullary },
                        Point2 { string0: StringValue, nullary0: CallNullary, string1: StringValue },
                        Point3 { string0: StringValue, nullary0: CallNullary, string1: StringValue, nullary1: CallNullary },
                        Point4 { string0: StringValue, nullary0: CallNullary, string1: StringValue, nullary1: CallNullary, int0: i128 },
                        Point5 { string0: StringValue, nullary0: CallNullary, string1: StringValue, nullary1: CallNullary, int0: i128, string2: StringValue },
                    }
                    fn calls_string_2_run(mut active: String2State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        loop {
                            match active {
                                String2State::Point0 { string0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String2Point0 { string0 }); }
                                    *budget -= 1;
                                    let nullary0 = CallNullary::new(data::type_::CustomConstructorId {
                                        type_id: data::type_::CustomTypeId(0),
                                        index: 0,
                                    });
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String2Point1 { string0, nullary0 }); }
                                    *budget -= 1;
                                    return {
                                        FunctionStep::StringCall { callee: FunctionState::String7Point0 { string0: string0.clone(), nullary0 }, caller: StringReturn::String2Call1 { string0, nullary0 } }
                                    };
                                },
                                String2State::Point1 { string0, nullary0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String2Point1 { string0, nullary0 }); }
                                    *budget -= 1;
                                    return {
                                        FunctionStep::StringCall { callee: FunctionState::String7Point0 { string0: string0.clone(), nullary0 }, caller: StringReturn::String2Call1 { string0, nullary0 } }
                                    };
                                },
                                String2State::Point2 { string0, nullary0, string1 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String2Point2 { string0, nullary0, string1 }); }
                                    *budget -= 1;
                                    let nullary1 = CallNullary::new(data::type_::CustomConstructorId {
                                        type_id: data::type_::CustomTypeId(0),
                                        index: 1,
                                    });
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String2Point3 { string0, nullary0, string1, nullary1 }); }
                                    *budget -= 1;
                                    let int0 = 5_i128;
                                    if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(2)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 4,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 2,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { nullaries: vec![nullary0, nullary1], ints: vec![int0.into()], strings: vec![string0, string1], ..CallValues::default() }) }; }
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String2Point4 { string0, nullary0, string1, nullary1, int0 }); }
                                    *budget -= 1;
                                    return {
                                        if ops.supports_string_native(data::function::StringFunctionId(6)) {
                                            return FunctionStep::StringNative { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "nested_tail", data::source::SourceSpan::new(672, 693)), arguments: Box::new(CallValues { nullaries: vec![nullary1], ints: vec![int0.into()], strings: vec![string1.clone()], ..CallValues::default() }), caller: Some(StringReturn::String2Call4 { string0, nullary0, string1, nullary1, int0 }) };
                                        }
                                        FunctionStep::StringBridge { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "nested_tail", data::source::SourceSpan::new(672, 693)), arguments: CallArguments { values: Box::new(CallValues { nullaries: vec![nullary1], ints: vec![int0.into()], strings: vec![string1.clone()], ..CallValues::default() }), captures: None }, caller: StringReturn::String2Call4 { string0, nullary0, string1, nullary1, int0 } }
                                    };
                                },
                                String2State::Point3 { string0, nullary0, string1, nullary1 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String2Point3 { string0, nullary0, string1, nullary1 }); }
                                    *budget -= 1;
                                    let int0 = 5_i128;
                                    if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(2)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 4,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 2,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { nullaries: vec![nullary0, nullary1], ints: vec![int0.into()], strings: vec![string0, string1], ..CallValues::default() }) }; }
                                    active = String2State::Point4 { string0, nullary0, string1, nullary1, int0 };
                                    continue;
                                },
                                String2State::Point4 { string0, nullary0, string1, nullary1, int0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String2Point4 { string0, nullary0, string1, nullary1, int0 }); }
                                    *budget -= 1;
                                    return {
                                        if ops.supports_string_native(data::function::StringFunctionId(6)) {
                                            return FunctionStep::StringNative { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "nested_tail", data::source::SourceSpan::new(672, 693)), arguments: Box::new(CallValues { nullaries: vec![nullary1], ints: vec![int0.into()], strings: vec![string1.clone()], ..CallValues::default() }), caller: Some(StringReturn::String2Call4 { string0, nullary0, string1, nullary1, int0 }) };
                                        }
                                        FunctionStep::StringBridge { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "nested_tail", data::source::SourceSpan::new(672, 693)), arguments: CallArguments { values: Box::new(CallValues { nullaries: vec![nullary1], ints: vec![int0.into()], strings: vec![string1.clone()], ..CallValues::default() }), captures: None }, caller: StringReturn::String2Call4 { string0, nullary0, string1, nullary1, int0 } }
                                    };
                                },
                                String2State::Point5 { string0, nullary0, string1, nullary1, int0, string2 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String2Point5 { string0, nullary0, string1, nullary1, int0, string2 }); }
                                    *budget -= 1;
                                    return {
                                        FunctionStep::String { value: string2 }
                                    };
                                },
                            }
                        }
                    }
                    enum String3State {
                        Point0 { string0: StringValue },
                        Point1 { string0: StringValue, nullary0: CallNullary },
                        Point2 { string0: StringValue, nullary0: CallNullary, int0: i128 },
                        Point3 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue },
                        Point4 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue, string2: StringValue },
                        Point5 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue, string2: StringValue, nullary1: CallNullary },
                        Point6 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue, string2: StringValue, nullary1: CallNullary, int1: i128 },
                        Point7 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue, string2: StringValue, nullary1: CallNullary, int1: i128, string3: StringValue },
                    }
                    fn calls_string_3_run(mut active: String3State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        loop {
                            match active {
                                String3State::Point0 { string0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String3Point0 { string0 }); }
                                    *budget -= 1;
                                    let nullary0 = CallNullary::new(data::type_::CustomConstructorId {
                                        type_id: data::type_::CustomTypeId(0),
                                        index: 0,
                                    });
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String3Point1 { string0, nullary0 }); }
                                    *budget -= 1;
                                    let int0 = 6_i128;
                                    if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(3)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0], ..CallValues::default() }) }; }
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String3Point2 { string0, nullary0, int0 }); }
                                    *budget -= 1;
                                    return {
                                        if ops.supports_string_native(data::function::StringFunctionId(6)) {
                                            return FunctionStep::StringNative { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "after_continuing", data::source::SourceSpan::new(771, 793)), arguments: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0.clone()], ..CallValues::default() }), caller: Some(StringReturn::String3Call2 { string0, nullary0, int0 }) };
                                        }
                                        FunctionStep::StringBridge { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "after_continuing", data::source::SourceSpan::new(771, 793)), arguments: CallArguments { values: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0.clone()], ..CallValues::default() }), captures: None }, caller: StringReturn::String3Call2 { string0, nullary0, int0 } }
                                    };
                                },
                                String3State::Point1 { string0, nullary0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String3Point1 { string0, nullary0 }); }
                                    *budget -= 1;
                                    let int0 = 6_i128;
                                    if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(3)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0], ..CallValues::default() }) }; }
                                    active = String3State::Point2 { string0, nullary0, int0 };
                                    continue;
                                },
                                String3State::Point2 { string0, nullary0, int0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String3Point2 { string0, nullary0, int0 }); }
                                    *budget -= 1;
                                    return {
                                        if ops.supports_string_native(data::function::StringFunctionId(6)) {
                                            return FunctionStep::StringNative { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "after_continuing", data::source::SourceSpan::new(771, 793)), arguments: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0.clone()], ..CallValues::default() }), caller: Some(StringReturn::String3Call2 { string0, nullary0, int0 }) };
                                        }
                                        FunctionStep::StringBridge { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "after_continuing", data::source::SourceSpan::new(771, 793)), arguments: CallArguments { values: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0.clone()], ..CallValues::default() }), captures: None }, caller: StringReturn::String3Call2 { string0, nullary0, int0 } }
                                    };
                                },
                                String3State::Point3 { string0, nullary0, int0, string1 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String3Point3 { string0, nullary0, int0, string1 }); }
                                    *budget -= 1;
                                    return {
                                        if ops.supports_string_native(data::function::StringFunctionId(8)) {
                                            return FunctionStep::StringNative { function: data::function::StringFunctionId(8), site: data::source::HostCallSite::from_static("string_native", "after_continuing", data::source::SourceSpan::new(812, 824)), arguments: Box::new(CallValues { strings: vec![string1.clone()], ..CallValues::default() }), caller: Some(StringReturn::String3Call3 { string0, nullary0, int0, string1 }) };
                                        }
                                        FunctionStep::StringBridge { function: data::function::StringFunctionId(8), site: data::source::HostCallSite::from_static("string_native", "after_continuing", data::source::SourceSpan::new(812, 824)), arguments: CallArguments { values: Box::new(CallValues { strings: vec![string1.clone()], ..CallValues::default() }), captures: None }, caller: StringReturn::String3Call3 { string0, nullary0, int0, string1 } }
                                    };
                                },
                                String3State::Point4 { string0, nullary0, int0, string1, string2 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String3Point4 { string0, nullary0, int0, string1, string2 }); }
                                    *budget -= 1;
                                    let nullary1 = CallNullary::new(data::type_::CustomConstructorId {
                                        type_id: data::type_::CustomTypeId(0),
                                        index: 1,
                                    });
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String3Point5 { string0, nullary0, int0, string1, string2, nullary1 }); }
                                    *budget -= 1;
                                    let int1 = 7_i128;
                                    if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(3)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 6,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 3,
                                        customs: 2,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { nullaries: vec![nullary0, nullary1], ints: vec![int0.into(), int1.into()], strings: vec![string0, string1, string2], ..CallValues::default() }) }; }
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String3Point6 { string0, nullary0, int0, string1, string2, nullary1, int1 }); }
                                    *budget -= 1;
                                    return {
                                        if ops.supports_string_native(data::function::StringFunctionId(6)) {
                                            return FunctionStep::StringNative { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "after_continuing", data::source::SourceSpan::new(840, 865)), arguments: Box::new(CallValues { nullaries: vec![nullary1], ints: vec![int1.into()], strings: vec![string2.clone()], ..CallValues::default() }), caller: Some(StringReturn::String3Call6 { string0, nullary0, int0, string1, string2, nullary1, int1 }) };
                                        }
                                        FunctionStep::StringBridge { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "after_continuing", data::source::SourceSpan::new(840, 865)), arguments: CallArguments { values: Box::new(CallValues { nullaries: vec![nullary1], ints: vec![int1.into()], strings: vec![string2.clone()], ..CallValues::default() }), captures: None }, caller: StringReturn::String3Call6 { string0, nullary0, int0, string1, string2, nullary1, int1 } }
                                    };
                                },
                                String3State::Point5 { string0, nullary0, int0, string1, string2, nullary1 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String3Point5 { string0, nullary0, int0, string1, string2, nullary1 }); }
                                    *budget -= 1;
                                    let int1 = 7_i128;
                                    if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(3)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 6,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 3,
                                        customs: 2,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { nullaries: vec![nullary0, nullary1], ints: vec![int0.into(), int1.into()], strings: vec![string0, string1, string2], ..CallValues::default() }) }; }
                                    active = String3State::Point6 { string0, nullary0, int0, string1, string2, nullary1, int1 };
                                    continue;
                                },
                                String3State::Point6 { string0, nullary0, int0, string1, string2, nullary1, int1 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String3Point6 { string0, nullary0, int0, string1, string2, nullary1, int1 }); }
                                    *budget -= 1;
                                    return {
                                        if ops.supports_string_native(data::function::StringFunctionId(6)) {
                                            return FunctionStep::StringNative { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "after_continuing", data::source::SourceSpan::new(840, 865)), arguments: Box::new(CallValues { nullaries: vec![nullary1], ints: vec![int1.into()], strings: vec![string2.clone()], ..CallValues::default() }), caller: Some(StringReturn::String3Call6 { string0, nullary0, int0, string1, string2, nullary1, int1 }) };
                                        }
                                        FunctionStep::StringBridge { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "after_continuing", data::source::SourceSpan::new(840, 865)), arguments: CallArguments { values: Box::new(CallValues { nullaries: vec![nullary1], ints: vec![int1.into()], strings: vec![string2.clone()], ..CallValues::default() }), captures: None }, caller: StringReturn::String3Call6 { string0, nullary0, int0, string1, string2, nullary1, int1 } }
                                    };
                                },
                                String3State::Point7 { string0, nullary0, int0, string1, string2, nullary1, int1, string3 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String3Point7 { string0, nullary0, int0, string1, string2, nullary1, int1, string3 }); }
                                    *budget -= 1;
                                    return {
                                        FunctionStep::String { value: string3 }
                                    };
                                },
                            }
                        }
                    }
                    enum String4State {
                        Point0 { string0: StringValue },
                        Point1 { string0: StringValue, nullary0: CallNullary },
                        Point2 { string0: StringValue, nullary0: CallNullary, int0: i128 },
                        Point3 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue },
                        Point4 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue, string2: StringValue },
                        Point5 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue, string2: StringValue, nullary1: CallNullary },
                        Point6 { string0: StringValue, nullary0: CallNullary, int0: i128, string1: StringValue, string2: StringValue, nullary1: CallNullary, int1: i128 },
                    }
                    fn calls_string_4_run(mut active: String4State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        loop {
                            match active {
                                String4State::Point0 { string0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String4Point0 { string0 }); }
                                    *budget -= 1;
                                    let nullary0 = CallNullary::new(data::type_::CustomConstructorId {
                                        type_id: data::type_::CustomTypeId(0),
                                        index: 0,
                                    });
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String4Point1 { string0, nullary0 }); }
                                    *budget -= 1;
                                    let int0 = 8_i128;
                                    if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(4)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0], ..CallValues::default() }) }; }
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String4Point2 { string0, nullary0, int0 }); }
                                    *budget -= 1;
                                    return {
                                        if ops.supports_string_native(data::function::StringFunctionId(6)) {
                                            return FunctionStep::StringNative { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "after_never", data::source::SourceSpan::new(938, 960)), arguments: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0.clone()], ..CallValues::default() }), caller: Some(StringReturn::String4Call2 { string0, nullary0, int0 }) };
                                        }
                                        FunctionStep::StringBridge { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "after_never", data::source::SourceSpan::new(938, 960)), arguments: CallArguments { values: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0.clone()], ..CallValues::default() }), captures: None }, caller: StringReturn::String4Call2 { string0, nullary0, int0 } }
                                    };
                                },
                                String4State::Point1 { string0, nullary0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String4Point1 { string0, nullary0 }); }
                                    *budget -= 1;
                                    let int0 = 8_i128;
                                    if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(4)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0], ..CallValues::default() }) }; }
                                    active = String4State::Point2 { string0, nullary0, int0 };
                                    continue;
                                },
                                String4State::Point2 { string0, nullary0, int0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String4Point2 { string0, nullary0, int0 }); }
                                    *budget -= 1;
                                    return {
                                        if ops.supports_string_native(data::function::StringFunctionId(6)) {
                                            return FunctionStep::StringNative { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "after_never", data::source::SourceSpan::new(938, 960)), arguments: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0.clone()], ..CallValues::default() }), caller: Some(StringReturn::String4Call2 { string0, nullary0, int0 }) };
                                        }
                                        FunctionStep::StringBridge { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "after_never", data::source::SourceSpan::new(938, 960)), arguments: CallArguments { values: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0.clone()], ..CallValues::default() }), captures: None }, caller: StringReturn::String4Call2 { string0, nullary0, int0 } }
                                    };
                                },
                                String4State::Point3 { string0, nullary0, int0, string1 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String4Point3 { string0, nullary0, int0, string1 }); }
                                    *budget -= 1;
                                    return {
                                        if ops.supports_string_native(data::function::StringFunctionId(9)) {
                                            return FunctionStep::StringNative { function: data::function::StringFunctionId(9), site: data::source::HostCallSite::from_static("string_native", "after_never", data::source::SourceSpan::new(977, 988)), arguments: Box::new(CallValues { strings: vec![string1.clone()], ..CallValues::default() }), caller: Some(StringReturn::String4Call3 { string0, nullary0, int0, string1 }) };
                                        }
                                        FunctionStep::StringBridge { function: data::function::StringFunctionId(9), site: data::source::HostCallSite::from_static("string_native", "after_never", data::source::SourceSpan::new(977, 988)), arguments: CallArguments { values: Box::new(CallValues { strings: vec![string1.clone()], ..CallValues::default() }), captures: None }, caller: StringReturn::String4Call3 { string0, nullary0, int0, string1 } }
                                    };
                                },
                                String4State::Point4 { string0, nullary0, int0, string1, string2 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String4Point4 { string0, nullary0, int0, string1, string2 }); }
                                    *budget -= 1;
                                    let nullary1 = CallNullary::new(data::type_::CustomConstructorId {
                                        type_id: data::type_::CustomTypeId(0),
                                        index: 1,
                                    });
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String4Point5 { string0, nullary0, int0, string1, string2, nullary1 }); }
                                    *budget -= 1;
                                    let int1 = 9_i128;
                                    if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(4)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 6,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 3,
                                        customs: 2,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { nullaries: vec![nullary0, nullary1], ints: vec![int0.into(), int1.into()], strings: vec![string0, string1, string2], ..CallValues::default() }) }; }
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String4Point6 { string0, nullary0, int0, string1, string2, nullary1, int1 }); }
                                    return {
                                        if ops.supports_string_native(data::function::StringFunctionId(6)) {
                                            *budget -= 1;
                                            return FunctionStep::StringNative { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "after_never", data::source::SourceSpan::new(991, 1014)), arguments: Box::new(CallValues { nullaries: vec![nullary1], ints: vec![int1.into()], strings: vec![string2.clone()], ..CallValues::default() }), caller: None };
                                        }
                                        FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(4)), point: data::compiled::CompiledCheckpoint {
                                            block: data::graph::BlockId(0),
                                            instruction: 6,
                                            ints: 2,
                                            bools: 0,
                                            bit_arrays: 0,
                                            int_lists: 0,
                                            strings: 3,
                                            customs: 2,
                                            custom_lists: 0,
                                            int_functions: 0,
                                            bool_functions: 0,
                                        }, values: Box::new(CallValues { nullaries: vec![nullary0, nullary1], ints: vec![int0.into(), int1.into()], strings: vec![string0, string1, string2], ..CallValues::default() }) }
                                    };
                                },
                                String4State::Point5 { string0, nullary0, int0, string1, string2, nullary1 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String4Point5 { string0, nullary0, int0, string1, string2, nullary1 }); }
                                    *budget -= 1;
                                    let int1 = 9_i128;
                                    if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(4)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 6,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 3,
                                        customs: 2,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { nullaries: vec![nullary0, nullary1], ints: vec![int0.into(), int1.into()], strings: vec![string0, string1, string2], ..CallValues::default() }) }; }
                                    active = String4State::Point6 { string0, nullary0, int0, string1, string2, nullary1, int1 };
                                    continue;
                                },
                                String4State::Point6 { string0, nullary0, int0, string1, string2, nullary1, int1 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String4Point6 { string0, nullary0, int0, string1, string2, nullary1, int1 }); }
                                    return {
                                        if ops.supports_string_native(data::function::StringFunctionId(6)) {
                                            *budget -= 1;
                                            return FunctionStep::StringNative { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "after_never", data::source::SourceSpan::new(991, 1014)), arguments: Box::new(CallValues { nullaries: vec![nullary1], ints: vec![int1.into()], strings: vec![string2.clone()], ..CallValues::default() }), caller: None };
                                        }
                                        FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(4)), point: data::compiled::CompiledCheckpoint {
                                            block: data::graph::BlockId(0),
                                            instruction: 6,
                                            ints: 2,
                                            bools: 0,
                                            bit_arrays: 0,
                                            int_lists: 0,
                                            strings: 3,
                                            customs: 2,
                                            custom_lists: 0,
                                            int_functions: 0,
                                            bool_functions: 0,
                                        }, values: Box::new(CallValues { nullaries: vec![nullary0, nullary1], ints: vec![int0.into(), int1.into()], strings: vec![string0, string1, string2], ..CallValues::default() }) }
                                    };
                                },
                            }
                        }
                    }
                    enum String7State {
                        Point0 { string0: StringValue, nullary0: CallNullary },
                        Point1 { string0: StringValue, nullary0: CallNullary, int0: i128 },
                    }
                    fn calls_string_7_run(active: String7State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            String7State::Point0 { string0, nullary0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::String7Point0 { string0, nullary0 }); }
                                *budget -= 1;
                                let int0 = 4_i128;
                                if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(7)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 1,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 1,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::String7Point1 { string0, nullary0, int0 }); }
                                {
                                    if ops.supports_string_native(data::function::StringFunctionId(6)) {
                                        *budget -= 1;
                                        return FunctionStep::StringNative { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "forward", data::source::SourceSpan::new(545, 570)), arguments: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0.clone()], ..CallValues::default() }), caller: None };
                                    }
                                    FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(7)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0], ..CallValues::default() }) }
                                }
                            },
                            String7State::Point1 { string0, nullary0, int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::String7Point1 { string0, nullary0, int0 }); }
                                {
                                    if ops.supports_string_native(data::function::StringFunctionId(6)) {
                                        *budget -= 1;
                                        return FunctionStep::StringNative { function: data::function::StringFunctionId(6), site: data::source::HostCallSite::from_static("string_native", "forward", data::source::SourceSpan::new(545, 570)), arguments: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0.clone()], ..CallValues::default() }), caller: None };
                                    }
                                    FunctionStep::Canonical { target: data::compiled::CallTarget::String(data::function::StringFunctionId(7)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 1,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { nullaries: vec![nullary0], ints: vec![int0.into()], strings: vec![string0], ..CallValues::default() }) }
                                }
                            },
                        }
                    }
                    fn calls_string_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                        let active = match point {
                            0 => FunctionState::String0Point0 { string0: values.string(0)? },
                            1 => FunctionState::String0Point1 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])? },
                            2 => FunctionState::String0Point2 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])?, int0: values.int(0)? },
                            3 => FunctionState::String0Point3 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])?, int0: values.int(0)?, string1: values.string(1)? },
                            4 => FunctionState::String0Point4 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])?, int0: values.int(0)?, string1: values.string(1)?, nullary1: values.nullary(1, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 1,
                                },
                            ])? },
                            5 => FunctionState::String0Point5 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])?, int0: values.int(0)?, string1: values.string(1)?, nullary1: values.nullary(1, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 1,
                                },
                            ])?, int1: values.int(1)? },
                            6 => FunctionState::String0Point6 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])?, int0: values.int(0)?, string1: values.string(1)?, nullary1: values.nullary(1, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 1,
                                },
                            ])?, int1: values.int(1)?, string2: values.string(2)? },
                            _ => return None,
                        };
                        Some(active)
                    }
                    fn calls_string_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                        if let Some(execution) = storage.reuse(data::compiled::CallTarget::String(data::function::StringFunctionId(0)), point, values) { return Some(execution); }
                        let active = calls_string_0_state(point, values)?;
                        Some(Box::new(FunctionExecution::new(active)))
                    }
                    fn calls_string_1_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                        let active = match point {
                            0 => FunctionState::String1Point0 { string0: values.string(0)? },
                            1 => FunctionState::String1Point1 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])? },
                            2 => FunctionState::String1Point2 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])?, int0: values.int(0)? },
                            _ => return None,
                        };
                        Some(active)
                    }
                    fn calls_string_1_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                        if let Some(execution) = storage.reuse(data::compiled::CallTarget::String(data::function::StringFunctionId(1)), point, values) { return Some(execution); }
                        let active = calls_string_1_state(point, values)?;
                        Some(Box::new(FunctionExecution::new(active)))
                    }
                    fn calls_string_2_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                        let active = match point {
                            0 => FunctionState::String2Point0 { string0: values.string(0)? },
                            1 => FunctionState::String2Point1 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])? },
                            2 => FunctionState::String2Point2 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])?, string1: values.string(1)? },
                            3 => FunctionState::String2Point3 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])?, string1: values.string(1)?, nullary1: values.nullary(1, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 1,
                                },
                            ])? },
                            4 => FunctionState::String2Point4 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])?, string1: values.string(1)?, nullary1: values.nullary(1, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 1,
                                },
                            ])?, int0: values.int(0)? },
                            5 => FunctionState::String2Point5 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])?, string1: values.string(1)?, nullary1: values.nullary(1, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 1,
                                },
                            ])?, int0: values.int(0)?, string2: values.string(2)? },
                            _ => return None,
                        };
                        Some(active)
                    }
                    fn calls_string_2_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                        if let Some(execution) = storage.reuse(data::compiled::CallTarget::String(data::function::StringFunctionId(2)), point, values) { return Some(execution); }
                        let active = calls_string_2_state(point, values)?;
                        Some(Box::new(FunctionExecution::new(active)))
                    }
                    fn calls_string_3_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                        let active = match point {
                            0 => FunctionState::String3Point0 { string0: values.string(0)? },
                            1 => FunctionState::String3Point1 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])? },
                            2 => FunctionState::String3Point2 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])?, int0: values.int(0)? },
                            3 => FunctionState::String3Point3 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])?, int0: values.int(0)?, string1: values.string(1)? },
                            4 => FunctionState::String3Point4 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])?, int0: values.int(0)?, string1: values.string(1)?, string2: values.string(2)? },
                            5 => FunctionState::String3Point5 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])?, int0: values.int(0)?, string1: values.string(1)?, string2: values.string(2)?, nullary1: values.nullary(1, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 1,
                                },
                            ])? },
                            6 => FunctionState::String3Point6 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])?, int0: values.int(0)?, string1: values.string(1)?, string2: values.string(2)?, nullary1: values.nullary(1, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 1,
                                },
                            ])?, int1: values.int(1)? },
                            7 => FunctionState::String3Point7 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])?, int0: values.int(0)?, string1: values.string(1)?, string2: values.string(2)?, nullary1: values.nullary(1, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 1,
                                },
                            ])?, int1: values.int(1)?, string3: values.string(3)? },
                            _ => return None,
                        };
                        Some(active)
                    }
                    fn calls_string_3_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                        if let Some(execution) = storage.reuse(data::compiled::CallTarget::String(data::function::StringFunctionId(3)), point, values) { return Some(execution); }
                        let active = calls_string_3_state(point, values)?;
                        Some(Box::new(FunctionExecution::new(active)))
                    }
                    fn calls_string_4_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                        let active = match point {
                            0 => FunctionState::String4Point0 { string0: values.string(0)? },
                            1 => FunctionState::String4Point1 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])? },
                            2 => FunctionState::String4Point2 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])?, int0: values.int(0)? },
                            3 => FunctionState::String4Point3 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])?, int0: values.int(0)?, string1: values.string(1)? },
                            4 => FunctionState::String4Point4 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])?, int0: values.int(0)?, string1: values.string(1)?, string2: values.string(2)? },
                            5 => FunctionState::String4Point5 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])?, int0: values.int(0)?, string1: values.string(1)?, string2: values.string(2)?, nullary1: values.nullary(1, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 1,
                                },
                            ])? },
                            6 => FunctionState::String4Point6 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                            ])?, int0: values.int(0)?, string1: values.string(1)?, string2: values.string(2)?, nullary1: values.nullary(1, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 1,
                                },
                            ])?, int1: values.int(1)? },
                            _ => return None,
                        };
                        Some(active)
                    }
                    fn calls_string_4_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                        if let Some(execution) = storage.reuse(data::compiled::CallTarget::String(data::function::StringFunctionId(4)), point, values) { return Some(execution); }
                        let active = calls_string_4_state(point, values)?;
                        Some(Box::new(FunctionExecution::new(active)))
                    }
                    fn calls_string_7_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                        let active = match point {
                            0 => FunctionState::String7Point0 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 1,
                                },
                            ])? },
                            1 => FunctionState::String7Point1 { string0: values.string(0)?, nullary0: values.nullary(0, &[
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                                data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 1,
                                },
                            ])?, int0: values.int(0)? },
                            _ => return None,
                        };
                        Some(active)
                    }
                    fn calls_string_7_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                        if let Some(execution) = storage.reuse(data::compiled::CallTarget::String(data::function::StringFunctionId(7)), point, values) { return Some(execution); }
                        let active = calls_string_7_state(point, values)?;
                        Some(Box::new(FunctionExecution::new(active)))
                    }
                    [calls_string_0_start, calls_string_1_start, calls_string_2_start, calls_string_3_start, calls_string_4_start, calls_string_7_start]
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
                            function: data::compiled::CallTarget::String(data::function::StringFunctionId(0)),
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
                                        strings: 1,
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
                                        strings: 1,
                                        customs: 1,
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
                                        strings: 1,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 3,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 4,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 2,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 5,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 2,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 6,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 3,
                                        customs: 2,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(1),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(1),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(1),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 2,
                                        output: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::String(data::function::StringFunctionId(6))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("string_native", "ordinary", data::source::SourceSpan::new(344, 366)),
                                    },
                                    data::compiled::CallContract {
                                        point: 5,
                                        output: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::String(data::function::StringFunctionId(6))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(1),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("string_native", "ordinary", data::source::SourceSpan::new(382, 403)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 6,
                                        value: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: CALL_GROUP_0[0],
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::String(data::function::StringFunctionId(1)),
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
                                        strings: 1,
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
                                        strings: 1,
                                        customs: 1,
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
                                        strings: 1,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[]),
                                tails: data::Storage::Static(&[
                                    data::compiled::TailContract {
                                        point: 2,
                                        target: data::compiled::CallTarget::String(data::function::StringFunctionId(6)),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("string_native", "tail", data::source::SourceSpan::new(457, 479)),
                                    },
                                ]),
                                start: CALL_GROUP_0[1],
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::String(data::function::StringFunctionId(2)),
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
                                        strings: 1,
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
                                        strings: 1,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 3,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 2,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 4,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 2,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 5,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 3,
                                        customs: 2,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(1),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(1),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(1),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 1,
                                        output: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::String(data::function::StringFunctionId(7))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                        ]),
                                        site: data::source::HostCallSite::from_static("string_native", "nested_tail", data::source::SourceSpan::new(634, 656)),
                                    },
                                    data::compiled::CallContract {
                                        point: 4,
                                        output: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::String(data::function::StringFunctionId(6))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(1),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("string_native", "nested_tail", data::source::SourceSpan::new(672, 693)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 5,
                                        value: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: CALL_GROUP_0[2],
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::String(data::function::StringFunctionId(3)),
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
                                        strings: 1,
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
                                        strings: 1,
                                        customs: 1,
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
                                        strings: 1,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 3,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 4,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 3,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 5,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 3,
                                        customs: 2,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 6,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 3,
                                        customs: 2,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 7,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 4,
                                        customs: 2,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(1),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(1),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(1),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(3)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 2,
                                        output: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::String(data::function::StringFunctionId(6))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("string_native", "after_continuing", data::source::SourceSpan::new(771, 793)),
                                    },
                                    data::compiled::CallContract {
                                        point: 3,
                                        output: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::String(data::function::StringFunctionId(8))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("string_native", "after_continuing", data::source::SourceSpan::new(812, 824)),
                                    },
                                    data::compiled::CallContract {
                                        point: 6,
                                        output: data::graph::ParamLocal::String(data::graph::StringLocalId(3)),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::String(data::function::StringFunctionId(6))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(1),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("string_native", "after_continuing", data::source::SourceSpan::new(840, 865)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 7,
                                        value: data::graph::ParamLocal::String(data::graph::StringLocalId(3)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: CALL_GROUP_0[3],
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::String(data::function::StringFunctionId(4)),
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
                                        strings: 1,
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
                                        strings: 1,
                                        customs: 1,
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
                                        strings: 1,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 3,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 4,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 3,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 5,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 3,
                                        customs: 2,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 6,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 3,
                                        customs: 2,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(1),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(1),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 2,
                                        output: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::String(data::function::StringFunctionId(6))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("string_native", "after_never", data::source::SourceSpan::new(938, 960)),
                                    },
                                    data::compiled::CallContract {
                                        point: 3,
                                        output: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::String(data::function::StringFunctionId(9))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("string_native", "after_never", data::source::SourceSpan::new(977, 988)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[]),
                                tails: data::Storage::Static(&[
                                    data::compiled::TailContract {
                                        point: 6,
                                        target: data::compiled::CallTarget::String(data::function::StringFunctionId(6)),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(1),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("string_native", "after_never", data::source::SourceSpan::new(991, 1014)),
                                    },
                                ]),
                                start: CALL_GROUP_0[4],
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::String(data::function::StringFunctionId(7)),
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
                                        strings: 1,
                                        customs: 1,
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
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(2),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(2),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[]),
                                tails: data::Storage::Static(&[
                                    data::compiled::TailContract {
                                        point: 1,
                                        target: data::compiled::CallTarget::String(data::function::StringFunctionId(6)),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(2),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("string_native", "forward", data::source::SourceSpan::new(545, 570)),
                                    },
                                ]),
                                start: CALL_GROUP_0[5],
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
                    0..0,
                    0..0,
                    0..10,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    10..11,
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
                            data::type_::ValueShapeId(0),
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
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 3..4,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 4..5,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 5..6,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 6..9,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(5),
                            data::type_::ValueShapeId(2),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 9..11,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(5),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 11..12,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 12..13,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 13..14,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(4),
                        captures: data::Storage::Static(&[]),
                    },
                ]),
                parameters: data::Storage::Static(&[
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(2),
                        },
                    }),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(2),
                        },
                    }),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                ]),
            },
            list_types: data::type_::ListTypeTable {
                types: data::Storage::Static(&[]),
                tuple_items: data::Storage::Static(&[]),
                function_items: data::Storage::Static(&[]),
                lifetimes: data::Storage::Static(&[]),
            },
            custom_types: data::type_::CustomTypeTable {
                types: data::Storage::Static(&[
                    data::type_::CustomTypeDescriptor {
                        type_: data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("string_native"),
                            name: data::Text::Static("Direction"),
                            arguments: data::Storage::Static(&[]),
                        },
                        native_visible: true,
                        lifetime: data::host::HostValueLifetime::LoadedOwner,
                        constructor_count: 2,
                        constructors: data::Storage::Static(&[
                            data::type_::CustomConstructorDescriptor {
                                id: data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                                name: data::Text::Static("Before"),
                                native_tag: data::Text::Static("before"),
                                fields: data::Storage::Static(&[]),
                            },
                            data::type_::CustomConstructorDescriptor {
                                id: data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 1,
                                },
                                name: data::Text::Static("After"),
                                native_tag: data::Text::Static("after"),
                                fields: data::Storage::Static(&[]),
                            },
                        ]),
                    },
                ]),
                definitions: data::Storage::Static(&[
                    data::type_::CustomDefinition {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("string_native"),
                        name: data::Text::Static("Direction"),
                        publicity: data::type_::CustomTypePublicity::Public,
                        opaque: false,
                        native_access: None,
                        retention_lifetime: data::host::HostValueLifetime::LoadedOwner,
                        parameters: 0,
                        constructors: data::Storage::Static(&[
                            data::type_::ConstructorDefinition {
                                name: data::Text::Static("Before"),
                                fields: data::Storage::Static(&[]),
                            },
                            data::type_::ConstructorDefinition {
                                name: data::Text::Static("After"),
                                fields: data::Storage::Static(&[]),
                            },
                        ]),
                    },
                ]),
            },
            external_types: data::type_::ExternalTypeTable {
                types: data::Storage::Static(&[]),
                lifetimes: data::Storage::Static(&[]),
                definitions: data::Storage::Static(&[]),
            },
            value_shapes: data::type_::ValueShapeTable {
                shapes: data::Storage::Static(&[
                    data::type_::ValueShapeDescriptor::String,
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(0)),
                    data::type_::ValueShapeDescriptor::Int,
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(1)),
                    data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                    ])),
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(2)),
                ]),
                shape_types: data::Storage::Static(&[
                    data::type_::ValueType::String,
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::Int,
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::String,
                        data::type_::ValueType::String,
                    ])),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                ]),
                custom_shapes: data::Storage::Static(&[
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(0),
                        arguments: data::Storage::Static(&[]),
                        constructor: data::type_::CustomConstructorRefinement::Exact(0),
                    },
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(0),
                        arguments: data::Storage::Static(&[]),
                        constructor: data::type_::CustomConstructorRefinement::Exact(1),
                    },
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(0),
                        arguments: data::Storage::Static(&[]),
                        constructor: data::type_::CustomConstructorRefinement::Any,
                    },
                ]),
            },
        },
        entries: data::program::LibraryFunctionEntries {
            ints: data::Storage::Static(&[]),
            floats: data::Storage::Static(&[]),
            strings: data::Storage::Static(&[
                data::program::LibraryFunctionEntry {
                    function: data::function::StringFunctionId(0),
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
                    function: data::function::StringFunctionId(1),
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
                    function: data::function::StringFunctionId(2),
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
                    function: data::function::StringFunctionId(3),
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
                    function: data::function::StringFunctionId(4),
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
                    function: data::function::StringFunctionId(5),
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
                name: data::Text::Static("ordinary"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("tail"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                },
                slot: 1,
            },
            data::Export {
                name: data::Text::Static("nested_tail"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                },
                slot: 2,
            },
            data::Export {
                name: data::Text::Static("after_continuing"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                },
                slot: 3,
            },
            data::Export {
                name: data::Text::Static("after_never"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                },
                slot: 4,
            },
            data::Export {
                name: data::Text::Static("source_caller"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                        data::type_::TypeMetadata::String,
                    ]))),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("source_string_caller"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                },
                slot: 5,
            },
        ]),
    },
    value_functions: data::Storage::Static(&[
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("string_native", "join", data::source::SourceSpan::new(77, 133)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::String,
                    data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("string_native"),
                        name: data::Text::Static("Direction"),
                        arguments: data::Storage::Static(&[]),
                    }),
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::String),
            },
            type_arguments: data::Storage::Static(&[]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::String(data::graph::StringLocalId(0)),
                    data::host::HostCallParameter::Custom(data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(2),
                        },
                    })),
                    data::host::HostCallParameter::Int(data::graph::IntLocalId(0)),
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
                            module: data::Text::Static("string_native"),
                            name: data::Text::Static("Direction"),
                            arguments: data::Storage::Static(&[]),
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
                    data::type_::ValueType::String,
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::Int,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::String),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 0,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::String,
                    data::host::RegistrationType::Custom {
                        schema: data::host::CustomSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("string_native"),
                            name: data::Text::Static("Direction"),
                            parameter_count: 0,
                            lifetime: data::host::HostValueLifetime::LoadedOwner,
                            constructors: data::Storage::Static(&[
                                data::host::ConstructorSchema {
                                    name: data::Text::Static("Before"),
                                    fields: data::Storage::Static(&[]),
                                },
                                data::host::ConstructorSchema {
                                    name: data::Text::Static("After"),
                                    fields: data::Storage::Static(&[]),
                                },
                            ]),
                            access: data::host::HostCustomAccess::Declared,
                        },
                        arguments: data::Storage::Static(&[]),
                    },
                    data::host::RegistrationType::Int,
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::String,
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::String(0),
                    data::host::RegistrationParameter::Custom(0),
                    data::host::RegistrationParameter::Int(0),
                ]),
                custom_schemas: data::Storage::Static(&[
                    data::host::CustomSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("string_native"),
                        name: data::Text::Static("Direction"),
                        parameter_count: 0,
                        lifetime: data::host::HostValueLifetime::LoadedOwner,
                        constructors: data::Storage::Static(&[
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Before"),
                                fields: data::Storage::Static(&[]),
                            },
                            data::host::ConstructorSchema {
                                name: data::Text::Static("After"),
                                fields: data::Storage::Static(&[]),
                            },
                        ]),
                        access: data::host::HostCustomAccess::Declared,
                    },
                ]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("string_native", "later", data::source::SourceSpan::new(182, 205)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::String,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::String),
            },
            type_arguments: data::Storage::Static(&[]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::String(data::graph::StringLocalId(0)),
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
                parameter_count: 0,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::String,
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::String,
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::String(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                restorations: data::Storage::Static(&[]),
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
            completion: data::host::HostFunctionCompletion::Never,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("string_native", "stop", data::source::SourceSpan::new(253, 275)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::String,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::String),
            },
            type_arguments: data::Storage::Static(&[]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::String(data::graph::StringLocalId(0)),
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
                parameter_count: 0,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::String,
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::String,
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::String(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
    ]),
    callables: data::Storage::Static(&[]),
}
