data::HostedModuleArtifact {
    module: data::ModuleArtifact {
        format: 20,
        program: data::ProgramTables {
            root: data::source::module_id(0),
            modules: data::Storage::Static(&[
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("function_values"),
                    source_context: Some(data::source::SourceContext::from_static_block("src/function_values.gleam", r#"
pub type Holder(item) {
  Held(fn(item) -> String)
}

@external(erlang, "native", "keep")
fn keep(function: fn(item) -> String) -> fn(item) -> String

@external(erlang, "native", "has_callback")
fn has_callback(function: fn(item) -> String) -> Bool

@external(erlang, "native", "keep_holder")
fn keep_holder(value: Holder(item)) -> Holder(item)

@external(erlang, "native", "keep_compound")
fn keep_compound(
  function: fn(List(item)) -> List(item),
) -> fn(List(item)) -> List(item)

pub fn main() {
  let label = "retained"
  let function = fn(_) { label }
  let alias = keep(function)
  let assert False = has_callback(alias)
  let assert Held(field) = keep_holder(Held(function))
  alias == function && field == function
}

pub fn concrete() {
  let label = "retained"
  let function = fn(_: Int) { label }
  let alias = keep(function)
  #(alias == function, has_callback(alias), alias(7))
}

pub fn compound() {
  let function = fn(items) { items }
  let alias = keep_compound(function)
  #(alias == function, alias([]) == [])
}
"#)),
                },
            ]),
            main: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Bool(data::function::BoolFunctionId(0))),
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
                                            params: 0..2,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::StringLocalId(0)),
                                ]),
                            },
                        })),
                    ]),
                    bit_array_functions: data::Storage::Static(&[]),
                    utf_codepoint_functions: data::Storage::Static(&[]),
                    custom_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 2,
                            return_: data::graph::CustomLocal {
                                id: data::graph::CustomLocalId(0),
                                shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(0),
                                    shape_id: data::type_::CustomValueShapeId(0),
                                },
                            },
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    external_functions: data::Storage::Static(&[]),
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
                                            instructions: 0..4,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                pattern: data::graph::MatchPattern::Bool(false),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                                                            id: data::graph::GenericFunctionLocalId(0),
                                                            type_: data::type_::GenericFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                                },
                                                                shape: data::type_::FunctionShape {
                                                                    shape_id: data::type_::ValueShapeId(2),
                                                                    type_: data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[
                                                                            data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                        ]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::String),
                                                                    },
                                                                },
                                                            },
                                                        })),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                                                            id: data::graph::GenericFunctionLocalId(1),
                                                            type_: data::type_::GenericFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                                },
                                                                shape: data::type_::FunctionShape {
                                                                    shape_id: data::type_::ValueShapeId(2),
                                                                    type_: data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[
                                                                            data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                        ]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::String),
                                                                    },
                                                                },
                                                            },
                                                        })),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(8),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::GenericFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 4..7,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::Equal {
                                                    left: data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                                                        id: data::graph::GenericFunctionLocalId(1),
                                                        type_: data::type_::GenericFunctionType {
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                            shape: data::type_::FunctionShape {
                                                                shape_id: data::type_::ValueShapeId(2),
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                                },
                                                            },
                                                        },
                                                    }),
                                                    right: data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                                                        id: data::graph::GenericFunctionLocalId(0),
                                                        type_: data::type_::GenericFunctionType {
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                            shape: data::type_::FunctionShape {
                                                                shape_id: data::type_::ValueShapeId(2),
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                                },
                                                            },
                                                        },
                                                    }),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                                                            id: data::graph::GenericFunctionLocalId(0),
                                                            type_: data::type_::GenericFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                                },
                                                                shape: data::type_::FunctionShape {
                                                                    shape_id: data::type_::ValueShapeId(2),
                                                                    type_: data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[
                                                                            data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                        ]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::String),
                                                                    },
                                                                },
                                                            },
                                                        }),
                                                        data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                                                            id: data::graph::GenericFunctionLocalId(2),
                                                            type_: data::type_::GenericFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                                },
                                                                shape: data::type_::FunctionShape {
                                                                    shape_id: data::type_::ValueShapeId(2),
                                                                    type_: data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[
                                                                            data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                        ]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::String),
                                                                    },
                                                                },
                                                            },
                                                        }),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Custom,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::GenericFunction,
                                                                length: 2,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 2,
                                                                        destination: 1,
                                                                    },
                                                                ]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(7),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Custom,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::GenericFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..4,
                                            instructions: 7..7,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::Equal {
                                                    left: data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                                                        id: data::graph::GenericFunctionLocalId(1),
                                                        type_: data::type_::GenericFunctionType {
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                            shape: data::type_::FunctionShape {
                                                                shape_id: data::type_::ValueShapeId(2),
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                                },
                                                            },
                                                        },
                                                    }),
                                                    right: data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                                                        id: data::graph::GenericFunctionLocalId(0),
                                                        type_: data::type_::GenericFunctionType {
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                            shape: data::type_::FunctionShape {
                                                                shape_id: data::type_::ValueShapeId(2),
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                                },
                                                            },
                                                        },
                                                    }),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(3),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::GenericFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(5),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::GenericFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 4..4,
                                            instructions: 7..8,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(4),
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
                                            params: 4..5,
                                            instructions: 8..8,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 5..5,
                                            instructions: 8..8,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(6),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 5..5,
                                            instructions: 8..9,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(4),
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
                                            params: 5..5,
                                            instructions: 9..9,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(6),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 5..6,
                                            instructions: 9..9,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("function_values", "main", data::source::SourceSpan::new(591, 601)),
                                                pattern_span: data::source::SourceSpan::new(602, 607),
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                                                id: data::graph::GenericFunctionLocalId(0),
                                                type_: data::type_::GenericFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::String),
                                                    },
                                                    shape: data::type_::FunctionShape {
                                                        shape_id: data::type_::ValueShapeId(2),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::String),
                                                        },
                                                    },
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                                                id: data::graph::GenericFunctionLocalId(1),
                                                type_: data::type_::GenericFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::String),
                                                    },
                                                    shape: data::type_::FunctionShape {
                                                        shape_id: data::type_::ValueShapeId(2),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::String),
                                                        },
                                                    },
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                                                id: data::graph::GenericFunctionLocalId(0),
                                                type_: data::type_::GenericFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::String),
                                                    },
                                                    shape: data::type_::FunctionShape {
                                                        shape_id: data::type_::ValueShapeId(2),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::String),
                                                        },
                                                    },
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                                                id: data::graph::GenericFunctionLocalId(1),
                                                type_: data::type_::GenericFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::String),
                                                    },
                                                    shape: data::type_::FunctionShape {
                                                        shape_id: data::type_::ValueShapeId(2),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::String),
                                                        },
                                                    },
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("retained"))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                                                    id: data::graph::GenericFunctionLocalId(0),
                                                    type_: data::type_::GenericFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::String),
                                                        },
                                                        shape: data::type_::FunctionShape {
                                                            shape_id: data::type_::ValueShapeId(2),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                        },
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                                family: data::function::FunctionReturnFamily::Generic,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Generic(data::function::GenericCallableId::Function {
                                                        template: 7,
                                                        substitution: data::Storage::Static(&[
                                                            data::type_::ValueShapeId(0),
                                                        ]),
                                                    }),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::String {
                                                            target: data::graph::StringLocalId(0),
                                                            source: data::graph::StringLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                                                    id: data::graph::GenericFunctionLocalId(1),
                                                    type_: data::type_::GenericFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::String),
                                                        },
                                                        shape: data::type_::FunctionShape {
                                                            shape_id: data::type_::ValueShapeId(2),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                        },
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                                family: data::function::FunctionReturnFamily::Generic,
                                                kind: data::graph::FunctionInstructionKind::Call {
                                                    function: data::function::ProfiledFunctionFunctionId::Generic(data::function::GenericFunctionFunctionId {
                                                        index: 0,
                                                        type_: data::type_::GenericFunctionType {
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                            shape: data::type_::FunctionShape {
                                                                shape_id: data::type_::ValueShapeId(2),
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                                },
                                                            },
                                                        },
                                                    }),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                                                            id: data::graph::GenericFunctionLocalId(0),
                                                            type_: data::type_::GenericFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                                },
                                                                shape: data::type_::FunctionShape {
                                                                    shape_id: data::type_::ValueShapeId(2),
                                                                    type_: data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[
                                                                            data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                        ]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::String),
                                                                    },
                                                                },
                                                            },
                                                        }),
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("function_values", "main", data::source::SourceSpan::new(574, 588)),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                                function: data::function::BoolFunctionId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                                                        id: data::graph::GenericFunctionLocalId(1),
                                                        type_: data::type_::GenericFunctionType {
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                            shape: data::type_::FunctionShape {
                                                                shape_id: data::type_::ValueShapeId(2),
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                                },
                                                            },
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_values", "main", data::source::SourceSpan::new(610, 629)),
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
                                                shape: data::type_::ValueShapeId(4),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                                                        id: data::graph::GenericFunctionLocalId(0),
                                                        type_: data::type_::GenericFunctionType {
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                            shape: data::type_::FunctionShape {
                                                                shape_id: data::type_::ValueShapeId(2),
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                                },
                                                            },
                                                        },
                                                    }),
                                                ]),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(5),
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
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(1),
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_values", "main", data::source::SourceSpan::new(657, 684)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                                                    id: data::graph::GenericFunctionLocalId(2),
                                                    type_: data::type_::GenericFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::String),
                                                        },
                                                        shape: data::type_::FunctionShape {
                                                            shape_id: data::type_::ValueShapeId(2),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                        },
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                                family: data::function::FunctionReturnFamily::Generic,
                                                kind: data::graph::FunctionInstructionKind::CustomField {
                                                    source: data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(1),
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
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(3),
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
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 1,
                            return_: data::graph::BoolLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 4,
                            return_: data::graph::BoolLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
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
                                            instructions: 0..8,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("retained"))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::StringFunction {
                                                    local: data::graph::StringFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::String),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(7),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                                family: data::function::FunctionReturnFamily::String,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::String(data::function::StringFunctionId(0)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::String {
                                                            target: data::graph::StringLocalId(0),
                                                            source: data::graph::StringLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::StringFunction {
                                                    local: data::graph::StringFunctionLocalId(1),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::String),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(7),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                                family: data::function::FunctionReturnFamily::String,
                                                kind: data::graph::FunctionInstructionKind::Call {
                                                    function: data::function::ProfiledFunctionFunctionId::String(data::function::StringFunctionFunctionId(0)),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::StringFunction {
                                                            local: data::graph::StringFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::String),
                                                            },
                                                        },
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("function_values", "concrete", data::source::SourceSpan::new(826, 840)),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Equal {
                                                left: data::graph::ParamLocal::StringFunction {
                                                    local: data::graph::StringFunctionLocalId(1),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::String),
                                                    },
                                                },
                                                right: data::graph::ParamLocal::StringFunction {
                                                    local: data::graph::StringFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::String),
                                                    },
                                                },
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                                function: data::function::BoolFunctionId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::StringFunction {
                                                        local: data::graph::StringFunctionLocalId(1),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::String),
                                                        },
                                                    },
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_values", "concrete", data::source::SourceSpan::new(864, 883)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(6),
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
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::FunctionCall {
                                                function: data::graph::StringFunctionLocalId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_values", "concrete", data::source::SourceSpan::new(885, 893)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::String,
                                                    ]),
                                                },
                                                shape: data::type_::ValueShapeId(8),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            ]))),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::TupleLocalId(0)),
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
                                            instructions: 0..8,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::Parameter {
                                                    local: data::graph::ParameterListFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                    },
                                                    list_type: data::type_::ParameterListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item: data::type_::parameter_id(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(10),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                },
                                                family: data::function::FunctionReturnFamily::List,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::List(data::function::ListFunctionId::Parameter(data::function::ParameterListFunctionId {
                                                        index: 0,
                                                        type_id: data::type_::ParameterListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item: data::type_::parameter_id(0),
                                                        },
                                                    })),
                                                    captures: data::Storage::Static(&[]),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::Parameter {
                                                    local: data::graph::ParameterListFunctionLocalId(1),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                    },
                                                    list_type: data::type_::ParameterListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item: data::type_::parameter_id(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(10),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                },
                                                family: data::function::FunctionReturnFamily::List,
                                                kind: data::graph::FunctionInstructionKind::Call {
                                                    function: data::function::ProfiledFunctionFunctionId::List(data::function::ProfiledListFunctionFunctionId::Parameter {
                                                        id: data::function::ParameterListFunctionFunctionId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                        },
                                                        list_type: data::type_::ParameterListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item: data::type_::parameter_id(0),
                                                        },
                                                    }),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::Parameter {
                                                            local: data::graph::ParameterListFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                            },
                                                            list_type: data::type_::ParameterListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                                item: data::type_::parameter_id(0),
                                                            },
                                                        }),
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("function_values", "compound", data::source::SourceSpan::new(969, 992)),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Equal {
                                                left: data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::Parameter {
                                                    local: data::graph::ParameterListFunctionLocalId(1),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                    },
                                                    list_type: data::type_::ParameterListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item: data::type_::parameter_id(0),
                                                    },
                                                }),
                                                right: data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::Parameter {
                                                    local: data::graph::ParameterListFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                    },
                                                    list_type: data::type_::ParameterListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item: data::type_::parameter_id(0),
                                                    },
                                                }),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                                                    local: data::graph::ParameterListLocalId(0),
                                                    type_id: data::type_::ParameterListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item: data::type_::parameter_id(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(9),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Parameter(data::type_::ParameterListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item: data::type_::parameter_id(0),
                                            }, data::graph::ParameterListInstruction::Empty)),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                                                    local: data::graph::ParameterListLocalId(1),
                                                    type_id: data::type_::ParameterListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item: data::type_::parameter_id(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(9),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Parameter(data::type_::ParameterListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item: data::type_::parameter_id(0),
                                            }, data::graph::ParameterListInstruction::FunctionCall {
                                                function: data::graph::ListFunctionLocal::Parameter {
                                                    local: data::graph::ParameterListFunctionLocalId(1),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                    },
                                                    list_type: data::type_::ParameterListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item: data::type_::parameter_id(0),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                                                        local: data::graph::ParameterListLocalId(0),
                                                        type_id: data::type_::ParameterListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item: data::type_::parameter_id(0),
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("function_values", "compound", data::source::SourceSpan::new(1016, 1025)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                                                    local: data::graph::ParameterListLocalId(2),
                                                    type_id: data::type_::ParameterListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item: data::type_::parameter_id(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(9),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Parameter(data::type_::ParameterListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item: data::type_::parameter_id(0),
                                            }, data::graph::ParameterListInstruction::Empty)),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Equal {
                                                left: data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                                                    local: data::graph::ParameterListLocalId(1),
                                                    type_id: data::type_::ParameterListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item: data::type_::parameter_id(0),
                                                    },
                                                }),
                                                right: data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                                                    local: data::graph::ParameterListLocalId(2),
                                                    type_id: data::type_::ParameterListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item: data::type_::parameter_id(0),
                                                    },
                                                }),
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
                                                shape: data::type_::ValueShapeId(11),
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
                    parameter_list_functions: data::Storage::Static(&[
                        (data::function::ParameterListFunctionId {
                            index: 0,
                            type_id: data::type_::ParameterListTypeId {
                                list_type: data::type_::ListTypeId(0),
                                item: data::type_::parameter_id(0),
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
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                                                local: data::graph::ParameterListLocalId(0),
                                                type_id: data::type_::ParameterListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item: data::type_::parameter_id(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(9),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::ParameterListLocalId(0)),
                                ]),
                            },
                        }))),
                    ]),
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
                    string_function_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 3,
                            return_: data::graph::StringFunctionLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    bit_array_function_functions: data::Storage::Static(&[]),
                    utf_codepoint_function_functions: data::Storage::Static(&[]),
                    custom_function_functions: data::Storage::Static(&[]),
                    external_function_functions: data::Storage::Static(&[]),
                    bool_function_functions: data::Storage::Static(&[]),
                    nil_function_functions: data::Storage::Static(&[]),
                    tuple_function_functions: data::Storage::Static(&[]),
                    generic_function_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 0,
                            return_: data::graph::GenericFunctionLocal {
                                id: data::graph::GenericFunctionLocalId(0),
                                type_: data::type_::GenericFunctionType {
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::String),
                                    },
                                    shape: data::type_::FunctionShape {
                                        shape_id: data::type_::ValueShapeId(2),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::String),
                                        },
                                    },
                                },
                            },
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    never_function_functions: data::Storage::Static(&[]),
                    parameter_list_function_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 5,
                            return_: data::graph::ListFunctionLocal::Parameter {
                                local: data::graph::ParameterListFunctionLocalId(0),
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                },
                                list_type: data::type_::ParameterListTypeId {
                                    list_type: data::type_::ListTypeId(0),
                                    item: data::type_::parameter_id(0),
                                },
                            },
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
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
            compiled: data::compiled::CompiledFunctions::interpreted(),
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
                    0..1,
                    0..0,
                    0..0,
                    1..2,
                    0..0,
                    2..5,
                    0..0,
                    5..7,
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
                    0..0,
                    8..9,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    9..10,
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
                ],
                functions: data::Storage::Static(&[
                    data::function::FunctionContract {
                        parameters: 0..1,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(6),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                shape: data::type_::ValueShapeId(1),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 1..2,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(5),
                        ]),
                        return_: data::type_::ValueShapeId(5),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 2..2,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(3),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 2..3,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(2),
                        ]),
                        return_: data::type_::ValueShapeId(3),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 3..4,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(7),
                        ]),
                        return_: data::type_::ValueShapeId(3),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 4..4,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(8),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 4..4,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(11),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 4..5,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(9),
                        ]),
                        return_: data::type_::ValueShapeId(9),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 5..6,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(7),
                        ]),
                        return_: data::type_::ValueShapeId(7),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 6..7,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(2),
                        ]),
                        return_: data::type_::ValueShapeId(2),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 7..8,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(10),
                        ]),
                        return_: data::type_::ValueShapeId(10),
                        captures: data::Storage::Static(&[]),
                    },
                ]),
                parameters: data::Storage::Static(&[
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(0),
                        },
                    }),
                    data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                        id: data::graph::GenericFunctionLocalId(0),
                        type_: data::type_::GenericFunctionType {
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::String),
                            },
                            shape: data::type_::FunctionShape {
                                shape_id: data::type_::ValueShapeId(2),
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                },
                            },
                        },
                    }),
                    data::graph::ParamLocal::StringFunction {
                        local: data::graph::StringFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::String),
                        },
                    },
                    data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                        local: data::graph::ParameterListLocalId(0),
                        type_id: data::type_::ParameterListTypeId {
                            list_type: data::type_::ListTypeId(0),
                            item: data::type_::parameter_id(0),
                        },
                    }),
                    data::graph::ParamLocal::StringFunction {
                        local: data::graph::StringFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::String),
                        },
                    },
                    data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                        id: data::graph::GenericFunctionLocalId(0),
                        type_: data::type_::GenericFunctionType {
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::String),
                            },
                            shape: data::type_::FunctionShape {
                                shape_id: data::type_::ValueShapeId(2),
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                },
                            },
                        },
                    }),
                    data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::Parameter {
                        local: data::graph::ParameterListFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                        },
                        list_type: data::type_::ParameterListTypeId {
                            list_type: data::type_::ListTypeId(0),
                            item: data::type_::parameter_id(0),
                        },
                    }),
                ]),
            },
            list_types: data::type_::ListTypeTable {
                types: data::Storage::Static(&[
                    data::type_::ListStorageTypeId::Parameter(data::type_::ParameterListTypeId {
                        list_type: data::type_::ListTypeId(0),
                        item: data::type_::parameter_id(0),
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
                            module: data::Text::Static("function_values"),
                            name: data::Text::Static("Holder"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                            ]),
                        },
                        constructor_count: 1,
                        constructors: data::Storage::Static(&[
                            data::type_::CustomConstructorDescriptor {
                                id: data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                                name: data::Text::Static("Held"),
                                native_tag: data::Text::Static("held"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: None,
                                        type_: data::type_::ValueType::Function(data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::String),
                                        }),
                                        shape: data::type_::ValueShapeId(2),
                                        refinement: data::type_::FieldRefinement::Function {
                                            arguments: data::Storage::Static(&[
                                                data::type_::FieldRefinement::Argument(0),
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
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_values"),
                        name: data::Text::Static("Holder"),
                        publicity: data::type_::CustomTypePublicity::Public,
                        opaque: false,
                        parameters: 1,
                        constructors: data::Storage::Static(&[
                            data::type_::ConstructorDefinition {
                                name: data::Text::Static("Held"),
                                fields: data::Storage::Static(&[
                                    data::type_::FieldDefinition {
                                        label: None,
                                        type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                            arguments: data::Storage::Static(&[
                                                data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                                        }),
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
                    data::type_::ValueShapeDescriptor::Parameter(data::type_::parameter_id(0)),
                    data::type_::ValueShapeDescriptor::String,
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                    },
                    data::type_::ValueShapeDescriptor::Bool,
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(1)),
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(0)),
                    data::type_::ValueShapeDescriptor::Int,
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(6),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                    },
                    data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                        data::type_::ValueShapeId(3),
                        data::type_::ValueShapeId(3),
                        data::type_::ValueShapeId(1),
                    ])),
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(0)),
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(9),
                        ]),
                        return_: data::type_::ValueShapeId(9),
                    },
                    data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                        data::type_::ValueShapeId(3),
                        data::type_::ValueShapeId(3),
                    ])),
                ]),
                shape_types: data::Storage::Static(&[
                    data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                    data::type_::ValueType::String,
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::String),
                    }),
                    data::type_::ValueType::Bool,
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::Int,
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::String),
                    }),
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::Bool,
                        data::type_::ValueType::Bool,
                        data::type_::ValueType::String,
                    ])),
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::List(data::type_::ListTypeId(0)),
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                    }),
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::Bool,
                        data::type_::ValueType::Bool,
                    ])),
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
                        type_id: data::type_::CustomTypeId(0),
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
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
            ]),
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
                data::program::LibraryFunctionEntry {
                    function: data::function::TupleFunctionId(1),
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
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("concrete"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                        data::type_::TypeMetadata::Bool,
                        data::type_::TypeMetadata::Bool,
                        data::type_::TypeMetadata::String,
                    ]))),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("compound"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                        data::type_::TypeMetadata::Bool,
                        data::type_::TypeMetadata::Bool,
                    ]))),
                },
                slot: 1,
            },
        ]),
    },
    value_functions: data::Storage::Static(&[
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_values", "keep", data::source::SourceSpan::new(90, 127)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                    shape: data::type_::ValueShapeId(0),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                        id: data::graph::GenericFunctionLocalId(0),
                        type_: data::type_::GenericFunctionType {
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::String),
                            },
                            shape: data::type_::FunctionShape {
                                shape_id: data::type_::ValueShapeId(2),
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::String),
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
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::String),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::String),
                })),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::FunctionValue {
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Parameter(0),
                        ]),
                        return_: data::Storage::Static(&data::host::RegistrationType::String),
                    },
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::FunctionValue {
                    arguments: data::Storage::Static(&[
                        data::host::RegistrationType::Parameter(0),
                    ]),
                    return_: data::Storage::Static(&data::host::RegistrationType::String),
                },
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
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
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_values", "has_callback", data::source::SourceSpan::new(195, 240)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                    shape: data::type_::ValueShapeId(0),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                        id: data::graph::GenericFunctionLocalId(0),
                        type_: data::type_::GenericFunctionType {
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::String),
                            },
                            shape: data::type_::FunctionShape {
                                shape_id: data::type_::ValueShapeId(2),
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::String),
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
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::String),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Bool),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::FunctionValue {
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Parameter(0),
                        ]),
                        return_: data::Storage::Static(&data::host::RegistrationType::String),
                    },
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Bool,
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
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
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_values", "keep_holder", data::source::SourceSpan::new(293, 328)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_values"),
                        name: data::Text::Static("Holder"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                        ]),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                    package: data::Text::Static("application"),
                    module: data::Text::Static("function_values"),
                    name: data::Text::Static("Holder"),
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                    ]),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
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
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_values"),
                            name: data::Text::Static("Holder"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
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
                            package: data::Text::Static("application"),
                            module: data::Text::Static("function_values"),
                            name: data::Text::Static("Holder"),
                            parameter_count: 1,
                            constructors: data::Storage::Static(&[
                                data::host::ConstructorSchema {
                                    name: data::Text::Static("Held"),
                                    fields: data::Storage::Static(&[
                                        data::host::FieldSchema {
                                            label: None,
                                            type_: data::host::SchemaType::FunctionValue {
                                                arguments: data::Storage::Static(&[
                                                    data::host::SchemaType::Parameter(0),
                                                ]),
                                                return_: data::Storage::Static(&data::host::SchemaType::String),
                                            },
                                        },
                                    ]),
                                },
                            ]),
                            shared: false,
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
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_values"),
                        name: data::Text::Static("Holder"),
                        parameter_count: 1,
                        constructors: data::Storage::Static(&[
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Held"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::FunctionValue {
                                            arguments: data::Storage::Static(&[
                                                data::host::SchemaType::Parameter(0),
                                            ]),
                                            return_: data::Storage::Static(&data::host::SchemaType::String),
                                        },
                                    },
                                ]),
                            },
                        ]),
                        shared: false,
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
                        package: data::Text::Static("application"),
                        module: data::Text::Static("function_values"),
                        name: data::Text::Static("Holder"),
                        parameter_count: 1,
                        constructors: data::Storage::Static(&[
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Held"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::FunctionValue {
                                            arguments: data::Storage::Static(&[
                                                data::host::SchemaType::Parameter(0),
                                            ]),
                                            return_: data::Storage::Static(&data::host::SchemaType::String),
                                        },
                                    },
                                ]),
                            },
                        ]),
                        shared: false,
                    },
                ]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
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
            site: data::source::HostCallSite::from_static("function_values", "keep", data::source::SourceSpan::new(90, 127)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Int,
                    shape: data::type_::ValueShapeId(6),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::StringFunction {
                        local: data::graph::StringFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
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
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::String),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::String),
                })),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::FunctionValue {
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Parameter(0),
                        ]),
                        return_: data::Storage::Static(&data::host::RegistrationType::String),
                    },
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::FunctionValue {
                    arguments: data::Storage::Static(&[
                        data::host::RegistrationType::Parameter(0),
                    ]),
                    return_: data::Storage::Static(&data::host::RegistrationType::String),
                },
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
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
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_values", "has_callback", data::source::SourceSpan::new(195, 240)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Int,
                    shape: data::type_::ValueShapeId(6),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::StringFunction {
                        local: data::graph::StringFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
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
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::String),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Bool),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::FunctionValue {
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Parameter(0),
                        ]),
                        return_: data::Storage::Static(&data::host::RegistrationType::String),
                    },
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Bool,
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
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
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("function_values", "keep_compound", data::source::SourceSpan::new(391, 452)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)))),
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0))))),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)))),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0))))),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                    shape: data::type_::ValueShapeId(0),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::Parameter {
                        local: data::graph::ParameterListFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                        },
                        list_type: data::type_::ParameterListTypeId {
                            list_type: data::type_::ListTypeId(0),
                            item: data::type_::parameter_id(0),
                        },
                    })),
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)))), data::type_::ListTypeId(0)),
                    ]),
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
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::List(data::type_::ListTypeId(0)),
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::List(data::type_::ListTypeId(0)),
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                })),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::FunctionValue {
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::List(data::Storage::Static(&data::host::RegistrationType::Parameter(0))),
                        ]),
                        return_: data::Storage::Static(&data::host::RegistrationType::List(data::Storage::Static(&data::host::RegistrationType::Parameter(0)))),
                    },
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::FunctionValue {
                    arguments: data::Storage::Static(&[
                        data::host::RegistrationType::List(data::Storage::Static(&data::host::RegistrationType::Parameter(0))),
                    ]),
                    return_: data::Storage::Static(&data::host::RegistrationType::List(data::Storage::Static(&data::host::RegistrationType::Parameter(0)))),
                },
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
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
    never_functions: data::Storage::Static(&[]),
    callables: data::Storage::Static(&[]),
}
