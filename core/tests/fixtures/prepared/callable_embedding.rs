data::HostedModuleArtifact {
    module: data::ModuleArtifact {
        format: 5,
        program: data::ProgramTables {
            root: data::source::module_id(1),
            modules: data::Storage::Static(&[
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("support"),
                    source_context: Some(data::source::SourceContext::from_static_block("support.gleam", r#"

@external(erlang, "ffi", "make_constant")
pub fn make_constant(value: a) -> fn() -> a
@external(erlang, "ffi", "make_adder")
pub fn make_adder(value: Int) -> fn(Int) -> Int
@external(erlang, "ffi", "wrap")
pub fn wrap(callback: fn(a) -> b) -> fn(a) -> b
@external(erlang, "ffi", "make_stop")
pub fn make_stop() -> fn() -> a
@external(erlang, "ffi", "produce")
pub fn produce() -> a
"#)),
                },
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("library"),
                    source_context: Some(data::source::SourceContext::from_static_block("library.gleam", r#"

import support
pub type Never
pub fn call_never(callback: fn() -> Never) { let _ = callback() 42 }
fn apply(callback, value) { callback(value) }
pub fn make_native(offset: Int) -> fn(Int) -> Int { support.make_adder(offset) }
pub fn keep(adjust: fn(Int) -> Int) -> fn(Int) -> Int { adjust }
pub fn calculate(value: Int, adjust: fn(Int) -> Int) -> Int { adjust(value) }
pub fn function_list(items: List(fn(Int) -> Int)) -> List(fn(Int) -> Int) { items }
pub fn container(adjust: fn(Int) -> Int) -> #(fn(Int) -> Int, Result(fn(Int) -> Int, Nil)) { #(adjust, Ok(adjust)) }
pub fn maker() -> fn(Int) -> fn(Int) -> Int { fn(offset) { support.make_adder(offset) } }
pub fn result_function() -> fn(Int) -> Result(Int, Nil) { fn(value) { Ok(value) } }
pub fn picker() -> fn(List(Result(Int, Nil))) -> Int {
    fn(items) { case items { [Ok(value)] -> value _ -> 0 } }
}
pub fn fail() {
    let stopped = support.make_stop()
    echo "before callable"
    let _ = stopped()
    echo "unreachable"
    42
}
pub fn producer() {
    echo "before producer"
    let _ = support.produce()
    echo "unreachable"
    42
}
pub fn run() {
    let add = support.make_adder(40)
    let constant = support.make_constant(2)
    apply(add, constant())
}
pub fn check() {
    let a = support.make_constant(True)
    let alias = a
    let b = support.make_constant(True)
    let list = support.make_constant([42])
    let is_answer = support.wrap(fn(value) { value == 42 })
    let increment = support.wrap(support.wrap(fn(value) { value + 1 }))
    let nested = support.wrap(fn(value) { support.make_constant(value) })
    a() && a == alias && a != b && list() == [42] && is_answer(increment(41)) && nested(42)() == 42
}
"#)),
                },
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("support/private"),
                    source_context: None,
                },
            ]),
            main: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Function {
                id: data::function::RuntimeFunctionFunctionTarget::Core(data::function::ProfiledFunctionFunctionId::Int(data::function::IntFunctionFunctionId(0))),
                return_type: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                },
            }),
            functions: data::function::FunctionTables {
                value_returns: data::function::ValueFunctionTables {
                    never_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Host(data::host::HostNeverFunctionId(0)),
                    ]),
                    int_functions: data::Storage::Static(&[
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
                                            shape: data::type_::ValueShapeId(1),
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
                                                site: data::source::HostCallSite::from_static("library", "calculate", data::source::SourceSpan::new(354, 367)),
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
                                            params: 0..1,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::NeverCall(data::graph::NeverCall {
                                                function: data::graph::NeverCallTarget::Value(data::graph::NeverFunctionLocal {
                                                    id: data::graph::NeverFunctionLocalId(0),
                                                    type_: data::type_::GenericFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(2))),
                                                        },
                                                        shape: data::type_::FunctionShape {
                                                            shape_id: data::type_::ValueShapeId(12),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(2))),
                                                            },
                                                        },
                                                    },
                                                }),
                                                args: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::NeverFunction,
                                                            positions: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                                site: data::source::HostCallSite::from_static("library", "call_never", data::source::SourceSpan::new(84, 94)),
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::NeverFunction(data::graph::NeverFunctionLocal {
                                                id: data::graph::NeverFunctionLocalId(0),
                                                type_: data::type_::GenericFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(2))),
                                                    },
                                                    shape: data::type_::FunctionShape {
                                                        shape_id: data::type_::ValueShapeId(12),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(2))),
                                                        },
                                                    },
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(12),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 0,
                            return_: data::graph::IntLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 2,
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
                                            params: 0..1,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                                subject: data::graph::BoolLocalId(0),
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                            local: data::graph::CustomListLocalId(0),
                                                            type_id: data::type_::CustomListTypeId {
                                                                list_type: data::type_::ListTypeId(1),
                                                                item_type: data::type_::CustomTypeId(1),
                                                            },
                                                        }),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CustomList,
                                                                positions: data::Storage::Static(&[
                                                                    0,
                                                                ]),
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
                                                                family: data::graph::StorageFamily::Bool,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CustomList,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 1..2,
                                            instructions: 1..2,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(1),
                                                    },
                                                }),
                                                pattern: data::graph::MatchPattern::Custom {
                                                    constructor: data::type_::CustomConstructorId {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        index: 0,
                                                    },
                                                    fields: data::Storage::Static(&[
                                                        data::graph::MatchPattern::Bind(data::graph::MatchPatternBinding {
                                                            index: 0,
                                                        }),
                                                    ]),
                                                },
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
                                                                family: data::graph::StorageFamily::Int,
                                                                positions: data::Storage::Static(&[
                                                                    0,
                                                                ]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Custom,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CustomList,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(3),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Custom,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CustomList,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..3,
                                            instructions: 2..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 3..3,
                                            instructions: 2..2,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(4),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 3..3,
                                            instructions: 2..3,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 3..3,
                                            instructions: 3..3,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(4),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(0),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(1),
                                                    item_type: data::type_::CustomTypeId(1),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(9),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(0),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(1),
                                                    item_type: data::type_::CustomTypeId(1),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(9),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::ListLengthEquals {
                                                value: data::graph::ListLocal::Custom {
                                                    local: data::graph::CustomListLocalId(0),
                                                    type_id: data::type_::CustomListTypeId {
                                                        list_type: data::type_::ListTypeId(1),
                                                        item_type: data::type_::CustomTypeId(1),
                                                    },
                                                },
                                                length: 1,
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(1),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(8),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::ListIndex {
                                                list: data::graph::CustomListLocalId(0),
                                                index: 0,
                                            }),
                                        },
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
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                ]),
                            },
                        })),
                    ]),
                    float_functions: data::Storage::Static(&[]),
                    string_functions: data::Storage::Static(&[]),
                    bit_array_functions: data::Storage::Static(&[]),
                    utf_codepoint_functions: data::Storage::Static(&[]),
                    custom_functions: data::Storage::Static(&[]),
                    external_functions: data::Storage::Static(&[]),
                    bool_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 1,
                            return_: data::graph::BoolLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
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
                                            instructions: 0..2,
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
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
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
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Function(data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        }),
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                    ]),
                                                },
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
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
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                }),
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
                    function_list_functions: data::Storage::Static(&[
                        (data::function::FunctionListFunctionId {
                            index: 0,
                            type_id: data::type_::FunctionListTypeId {
                                list_type: data::type_::ListTypeId(0),
                                item_type: data::type_::FunctionItemTypeId(0),
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
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Function {
                                                local: data::graph::FunctionListLocalId(0),
                                                type_id: data::type_::FunctionListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::FunctionItemTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::FunctionListLocalId(0)),
                                ]),
                            },
                        }))),
                    ]),
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
                                        data::function::FunctionExit::TailCall {
                                            function: data::source::FunctionCallTarget {
                                                function: data::function::IntFunctionFunctionId(3),
                                                site: data::source::HostCallSite::from_static("library", "make_native", data::source::SourceSpan::new(198, 224)),
                                            },
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            ]),
                                            transfer: data::graph::Transfer {
                                                families: data::Storage::Static(&[
                                                    data::graph::FamilyTransfer {
                                                        family: data::graph::StorageFamily::Int,
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
                                                instructions: 0..0,
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
                                        instructions: data::Storage::Static(&[]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::Return(data::graph::IntFunctionLocalId(0)),
                                    ]),
                                },
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::TypedFunctionBody {
                                _shape: data::type_::FunctionShape {
                                    shape_id: data::type_::ValueShapeId(10),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::List(data::type_::ListTypeId(1)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    },
                                },
                                body: data::function::ProfiledFunctionBody {
                                    block_graph: data::graph::ProfiledBlockGraph {
                                        entry: data::graph::BlockId(0),
                                        blocks: data::Storage::Static(&[
                                            data::graph::BlockHeader {
                                                params: 0..0,
                                                instructions: 0..1,
                                                terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                            },
                                        ]),
                                        params: data::Storage::Static(&[]),
                                        instructions: data::Storage::Static(&[
                                            data::graph::ProfiledInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::IntFunction {
                                                        local: data::graph::IntFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::List(data::type_::ListTypeId(1)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        },
                                                    },
                                                    shape: data::type_::ValueShapeId(10),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::List(data::type_::ListTypeId(1)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                    family: data::function::FunctionReturnFamily::Int,
                                                    kind: data::graph::FunctionInstructionKind::Closure {
                                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(4)),
                                                        captures: data::Storage::Static(&[]),
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
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 3,
                            return_: data::graph::IntFunctionLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
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
                                        data::function::FunctionExit::TailCall {
                                            function: data::source::FunctionCallTarget {
                                                function: data::function::IntFunctionFunctionId(3),
                                                site: data::source::HostCallSite::from_static("library", "<anonymous:0>", data::source::SourceSpan::new(630, 656)),
                                            },
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            ]),
                                            transfer: data::graph::Transfer {
                                                families: data::Storage::Static(&[
                                                    data::graph::FamilyTransfer {
                                                        family: data::graph::StorageFamily::Int,
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
                    function_function_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionFunctionFunctionBody {
                                _shape: data::type_::FunctionShape {
                                    shape_id: data::type_::ValueShapeId(7),
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
                                },
                                _type: data::type_::FunctionFunctionType {
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
                                        data::type_::ValueShapeId(0),
                                    ]),
                                    return_: data::type_::FunctionShape {
                                        shape_id: data::type_::ValueShapeId(1),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                    },
                                },
                                body: data::function::ProfiledFunctionBody {
                                    block_graph: data::graph::ProfiledBlockGraph {
                                        entry: data::graph::BlockId(0),
                                        blocks: data::Storage::Static(&[
                                            data::graph::BlockHeader {
                                                params: 0..0,
                                                instructions: 0..1,
                                                terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                            },
                                        ]),
                                        params: data::Storage::Static(&[]),
                                        instructions: data::Storage::Static(&[
                                            data::graph::ProfiledInstruction {
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
                                                                data::type_::ValueShapeId(0),
                                                            ]),
                                                            return_: data::type_::FunctionShape {
                                                                shape_id: data::type_::ValueShapeId(1),
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::Int,
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                                },
                                                            },
                                                        },
                                                    })),
                                                    shape: data::type_::ValueShapeId(7),
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
                                                        target: data::graph::FunctionTarget::Function(data::function::ProfiledFunctionFunctionId::Int(data::function::IntFunctionFunctionId(4))),
                                                        captures: data::Storage::Static(&[]),
                                                    },
                                                }),
                                            },
                                        ]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::Return(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
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
                                                    data::type_::ValueShapeId(0),
                                                ]),
                                                return_: data::type_::FunctionShape {
                                                    shape_id: data::type_::ValueShapeId(1),
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
                            },
                        })),
                    ]),
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
                    0..1,
                    1..6,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    6..7,
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
                    8..9,
                    9..14,
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
                    14..15,
                ],
                functions: data::Storage::Static(&[
                    data::function::FunctionContract {
                        parameters: 0..0,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(11),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 0..2,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 2..3,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(12),
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
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                shape: data::type_::ValueShapeId(0),
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
                    },
                    data::function::FunctionContract {
                        parameters: 5..6,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(9),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 6..6,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(3),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                shape: data::type_::ValueShapeId(3),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 6..7,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(14),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 7..8,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(4),
                        ]),
                        return_: data::type_::ValueShapeId(4),
                        captures: data::Storage::Static(&[]),
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
                        parameters: 9..10,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 10..10,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(10),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 10..11,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 11..12,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 12..12,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(7),
                        captures: data::Storage::Static(&[]),
                    },
                ]),
                parameters: data::Storage::Static(&[
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
                    data::graph::ParamLocal::NeverFunction(data::graph::NeverFunctionLocal {
                        id: data::graph::NeverFunctionLocalId(0),
                        type_: data::type_::GenericFunctionType {
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[]),
                                return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(2))),
                            },
                            shape: data::type_::FunctionShape {
                                shape_id: data::type_::ValueShapeId(12),
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(2))),
                                },
                            },
                        },
                    }),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                        local: data::graph::CustomListLocalId(0),
                        type_id: data::type_::CustomListTypeId {
                            list_type: data::type_::ListTypeId(1),
                            item_type: data::type_::CustomTypeId(1),
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
                    data::graph::ParamLocal::List(data::graph::ListLocal::Function {
                        local: data::graph::FunctionListLocalId(0),
                        type_id: data::type_::FunctionListTypeId {
                            list_type: data::type_::ListTypeId(0),
                            item_type: data::type_::FunctionItemTypeId(0),
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
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                ]),
            },
            list_types: data::type_::ListTypeTable {
                types: data::Storage::Static(&[
                    data::type_::ListStorageTypeId::Function(data::type_::FunctionListTypeId {
                        list_type: data::type_::ListTypeId(0),
                        item_type: data::type_::FunctionItemTypeId(0),
                    }),
                    data::type_::ListStorageTypeId::Custom(data::type_::CustomListTypeId {
                        list_type: data::type_::ListTypeId(1),
                        item_type: data::type_::CustomTypeId(1),
                    }),
                ]),
                tuple_items: data::Storage::Static(&[]),
                function_items: data::Storage::Static(&[
                    data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    },
                ]),
            },
            custom_types: data::type_::CustomTypeTable {
                types: data::Storage::Static(&[
                    data::type_::CustomTypeDescriptor {
                        type_: data::type_::NominalTypeMetadata {
                            package: data::Text::Static(""),
                            module: data::Text::Static("gleam"),
                            name: data::Text::Static("Result"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                    arguments: data::Storage::Static(&[
                                        data::type_::TypeMetadata::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                                }),
                                data::type_::TypeMetadata::Nil,
                            ]),
                        },
                        constructor_count: 2,
                        constructors: data::Storage::Static(&[
                            data::type_::CustomConstructorDescriptor {
                                id: data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                                name: data::Text::Static("Ok"),
                                native_tag: data::Text::Static("ok"),
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
                    data::type_::CustomTypeDescriptor {
                        type_: data::type_::NominalTypeMetadata {
                            package: data::Text::Static(""),
                            module: data::Text::Static("gleam"),
                            name: data::Text::Static("Result"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                                data::type_::TypeMetadata::Nil,
                            ]),
                        },
                        constructor_count: 2,
                        constructors: data::Storage::Static(&[
                            data::type_::CustomConstructorDescriptor {
                                id: data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(1),
                                    index: 0,
                                },
                                name: data::Text::Static("Ok"),
                                native_tag: data::Text::Static("ok"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: None,
                                        type_: data::type_::ValueType::Int,
                                        shape: data::type_::ValueShapeId(0),
                                        refinement: data::type_::FieldRefinement::Argument(0),
                                    },
                                ]),
                            },
                            data::type_::CustomConstructorDescriptor {
                                id: data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(1),
                                    index: 1,
                                },
                                name: data::Text::Static("Error"),
                                native_tag: data::Text::Static("error"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: None,
                                        type_: data::type_::ValueType::Nil,
                                        shape: data::type_::ValueShapeId(2),
                                        refinement: data::type_::FieldRefinement::Argument(1),
                                    },
                                ]),
                            },
                        ]),
                    },
                    data::type_::CustomTypeDescriptor {
                        type_: data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("library"),
                            name: data::Text::Static("Never"),
                            arguments: data::Storage::Static(&[]),
                        },
                        constructor_count: 0,
                        constructors: data::Storage::Static(&[]),
                    },
                ]),
                definitions: data::Storage::Static(&[
                    data::type_::CustomDefinition {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("library"),
                        name: data::Text::Static("Never"),
                        publicity: data::type_::CustomTypePublicity::Public,
                        opaque: false,
                        parameters: 0,
                        constructors: data::Storage::Static(&[]),
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
                    data::type_::ValueShapeDescriptor::Nil,
                    data::type_::ValueShapeDescriptor::Bool,
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(1)),
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(0)),
                    data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                        data::type_::ValueShapeId(1),
                        data::type_::ValueShapeId(5),
                    ])),
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                    },
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(1)),
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(8)),
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(9),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                    },
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(2)),
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(11),
                    },
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(3)),
                    data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                        data::type_::ValueShapeId(1),
                        data::type_::ValueShapeId(13),
                    ])),
                ]),
                shape_types: data::Storage::Static(&[
                    data::type_::ValueType::Int,
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    }),
                    data::type_::ValueType::Nil,
                    data::type_::ValueType::Bool,
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::Function(data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        }),
                        data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    ])),
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
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                    data::type_::ValueType::List(data::type_::ListTypeId(1)),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::List(data::type_::ListTypeId(1)),
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    }),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(2))),
                    }),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::Function(data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        }),
                        data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    ])),
                ]),
                custom_shapes: data::Storage::Static(&[
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(0),
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                            data::type_::ValueShapeId(2),
                        ]),
                        constructor: data::type_::CustomConstructorRefinement::Exact(0),
                    },
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(1),
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(2),
                        ]),
                        constructor: data::type_::CustomConstructorRefinement::Any,
                    },
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(2),
                        arguments: data::Storage::Static(&[]),
                        constructor: data::type_::CustomConstructorRefinement::Any,
                    },
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(0),
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                            data::type_::ValueShapeId(2),
                        ]),
                        constructor: data::type_::CustomConstructorRefinement::Any,
                    },
                ]),
            },
        },
        entries: data::program::LibraryFunctionEntries {
            ints: data::Storage::Static(&[
                data::program::LibraryFunctionEntry {
                    function: data::function::IntFunctionId(0),
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
                    function: data::function::IntFunctionId(1),
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
                    callables: data::Storage::Static(&[
                        data::program::LibraryCallable {
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
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
                        data::program::LibraryCallable {
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
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
                },
            ]),
            lists: data::Storage::Static(&[
                data::program::LibraryFunctionEntry {
                    function: data::function::LibraryListFunctionId::Function(data::function::FunctionListFunctionId {
                        index: 0,
                        type_id: data::type_::FunctionListTypeId {
                            list_type: data::type_::ListTypeId(0),
                            item_type: data::type_::FunctionItemTypeId(0),
                        },
                    }),
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
                            functions: data::Storage::Static(&[
                                data::type_::FunctionListTypeId {
                                    list_type: data::type_::ListTypeId(0),
                                    item_type: data::type_::FunctionItemTypeId(0),
                                },
                            ]),
                        },
                    },
                    callables: data::Storage::Static(&[
                        data::program::LibraryCallable {
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
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
                },
            ]),
            functions: data::Storage::Static(&[
                data::program::LibraryFunctionEntry {
                    function: data::program::LibraryCallableEntry {
                        function: data::function::RuntimeFunctionFunctionTarget::Core(data::function::ProfiledFunctionFunctionId::Int(data::function::IntFunctionFunctionId(0))),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
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
                    callables: data::Storage::Static(&[
                        data::program::LibraryCallable {
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
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
                },
                data::program::LibraryFunctionEntry {
                    function: data::program::LibraryCallableEntry {
                        function: data::function::RuntimeFunctionFunctionTarget::Core(data::function::ProfiledFunctionFunctionId::Int(data::function::IntFunctionFunctionId(1))),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
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
                    callables: data::Storage::Static(&[
                        data::program::LibraryCallable {
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
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
                },
                data::program::LibraryFunctionEntry {
                    function: data::program::LibraryCallableEntry {
                        function: data::function::RuntimeFunctionFunctionTarget::Core(data::function::ProfiledFunctionFunctionId::Function(data::function::FunctionFunctionFunctionId {
                            index: 0,
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
                                    data::type_::ValueShapeId(0),
                                ]),
                                return_: data::type_::FunctionShape {
                                    shape_id: data::type_::ValueShapeId(1),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    },
                                },
                            },
                        })),
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
                    callables: data::Storage::Static(&[
                        data::program::LibraryCallable {
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
                            callables: data::Storage::Static(&[
                                data::program::LibraryCallable {
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
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
                        },
                    ]),
                },
                data::program::LibraryFunctionEntry {
                    function: data::program::LibraryCallableEntry {
                        function: data::function::RuntimeFunctionFunctionTarget::Core(data::function::ProfiledFunctionFunctionId::Int(data::function::IntFunctionFunctionId(2))),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::List(data::type_::ListTypeId(1)),
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
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
                    callables: data::Storage::Static(&[
                        data::program::LibraryCallable {
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::List(data::type_::ListTypeId(1)),
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            },
                            inputs: data::program::LibraryInputConstructions {
                                variants: data::Storage::Static(&[
                                    [
                                        data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(1),
                                            index: 0,
                                        },
                                        data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(1),
                                            index: 1,
                                        },
                                    ],
                                ]),
                                lists: data::program::LibraryListConstructions {
                                    ints: data::Storage::Static(&[]),
                                    floats: data::Storage::Static(&[]),
                                    strings: data::Storage::Static(&[]),
                                    bit_arrays: data::Storage::Static(&[]),
                                    utf_codepoints: data::Storage::Static(&[]),
                                    customs: data::Storage::Static(&[
                                        data::type_::CustomListTypeId {
                                            list_type: data::type_::ListTypeId(1),
                                            item_type: data::type_::CustomTypeId(1),
                                        },
                                    ]),
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
                },
            ]),
        },
        exports: data::Storage::Static(&[
            data::Export {
                name: data::Text::Static("make_native"),
                signature: data::type_::FunctionMetadata {
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
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("keep"),
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
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                    })),
                },
                slot: 1,
            },
            data::Export {
                name: data::Text::Static("calculate"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                        }),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("function_list"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                        }))),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                    })))),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("container"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                        }),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                        data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                        }),
                        data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static(""),
                            module: data::Text::Static("gleam"),
                            name: data::Text::Static("Result"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                    arguments: data::Storage::Static(&[
                                        data::type_::TypeMetadata::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                                }),
                                data::type_::TypeMetadata::Nil,
                            ]),
                        }),
                    ]))),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("maker"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                        })),
                    })),
                },
                slot: 2,
            },
            data::Export {
                name: data::Text::Static("picker"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                package: data::Text::Static(""),
                                module: data::Text::Static("gleam"),
                                name: data::Text::Static("Result"),
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::Int,
                                    data::type_::TypeMetadata::Nil,
                                ]),
                            }))),
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                    })),
                },
                slot: 3,
            },
            data::Export {
                name: data::Text::Static("call_never"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("library"),
                                name: data::Text::Static("Never"),
                                arguments: data::Storage::Static(&[]),
                            })),
                        }),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 1,
            },
        ]),
    },
    value_functions: data::Storage::Static(&[
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::Int,
                index: 2,
            }),
            package: data::Text::Static("support"),
            site: data::source::HostCallSite::from_static("support/private", "add", data::source::SourceSpan::new(0, 0)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            type_arguments: data::Storage::Static(&[]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Int(data::graph::IntLocalId(0)),
                ]),
                captures: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                        shape: data::type_::ValueShapeId(0),
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
                parameter_count: 0,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Int,
                ]),
                captures: data::Storage::Static(&[
                    data::host::RegistrationType::Int,
                ]),
                callable: true,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Int,
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Int(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::Bool,
                index: 0,
            }),
            package: data::Text::Static("support"),
            site: data::source::HostCallSite::from_static("support/private", "constant", data::source::SourceSpan::new(0, 0)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Bool,
                    shape: data::type_::ValueShapeId(3),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[]),
                captures: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::Bool),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[]),
                captures: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                ]),
                callable: true,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::Int,
                index: 3,
            }),
            package: data::Text::Static("support"),
            site: data::source::HostCallSite::from_static("support/private", "wrap", data::source::SourceSpan::new(0, 0)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
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
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
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
                        shape: data::type_::ValueShapeId(1),
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
                    data::host::RegistrationType::Parameter(0),
                ]),
                captures: data::Storage::Static(&[
                    data::host::RegistrationType::Function {
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Parameter(0),
                        ]),
                        return_: data::Storage::Static(&data::host::RegistrationType::Parameter(1)),
                    },
                ]),
                callable: true,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(1),
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("support"),
            site: data::source::HostCallSite::from_static("support", "make_adder", data::source::SourceSpan::new(126, 155)),
            signature: data::type_::FunctionMetadata {
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
            type_arguments: data::Storage::Static(&[]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Int(data::graph::IntLocalId(0)),
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
                callables: data::Storage::Static(&[
                    data::host::HostCallableConstruction {
                        target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Int(data::function::IntFunctionId(2))),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        },
                        parameters: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                shape: data::type_::ValueShapeId(0),
                            },
                        ]),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                shape: data::type_::ValueShapeId(0),
                            },
                        ]),
                    },
                ]),
            },
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
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 0,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Int,
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[
                    data::host::CallableRegistration {
                        package: data::Text::Static("support"),
                        module: data::Text::Static("support/private"),
                        name: data::Text::Static("add"),
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Int,
                        ]),
                        captures: data::Storage::Static(&[
                            data::host::RegistrationType::Int,
                        ]),
                        return_: data::host::RegistrationType::Int,
                        returns_value: true,
                    },
                ]),
                return_: data::host::RegistrationType::Function {
                    arguments: data::Storage::Static(&[
                        data::host::RegistrationType::Int,
                    ]),
                    return_: data::Storage::Static(&data::host::RegistrationType::Int),
                },
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Int(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Function {
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Int,
                        ]),
                        return_: data::Storage::Static(&data::host::RegistrationType::Int),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
    ]),
    never_functions: data::Storage::Static(&[
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Uninhabited,
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::Never,
                index: 0,
            }),
            package: data::Text::Static("support"),
            site: data::source::HostCallSite::from_static("support/private", "stop", data::source::SourceSpan::new(0, 0)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                    package: data::Text::Static("application"),
                    module: data::Text::Static("library"),
                    name: data::Text::Static("Never"),
                    arguments: data::Storage::Static(&[]),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("library"),
                        name: data::Text::Static("Never"),
                        arguments: data::Storage::Static(&[]),
                    }),
                    shape: data::type_::ValueShapeId(11),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[]),
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
                            module: data::Text::Static("library"),
                            name: data::Text::Static("Never"),
                            arguments: data::Storage::Static(&[]),
                        }), data::type_::CustomTypeId(2)),
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
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(2))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[]),
                captures: data::Storage::Static(&[]),
                callable: true,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Parameter(0),
                layout: data::Storage::Static(&[]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
    ]),
    callables: data::Storage::Static(&[
        data::program::LibraryNativeConstruction {
            declaration: data::host::CallableRegistration {
                package: data::Text::Static("support"),
                module: data::Text::Static("support/private"),
                name: data::Text::Static("stop"),
                arguments: data::Storage::Static(&[]),
                captures: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Custom {
                    schema: data::host::CustomSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("library"),
                        name: data::Text::Static("Never"),
                        parameter_count: 0,
                        constructors: data::Storage::Static(&[]),
                        shared: false,
                    },
                    arguments: data::Storage::Static(&[]),
                },
                returns_value: true,
            },
            construction: data::host::HostCallableConstruction {
                target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Never(data::function::NeverFunctionId(0))),
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(2))),
                },
                parameters: data::Storage::Static(&[]),
                captures: data::Storage::Static(&[]),
            },
            invocation: data::program::LibraryCallable {
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(2))),
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
            captures: data::program::LibraryInputConstructions {
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
        },
        data::program::LibraryNativeConstruction {
            declaration: data::host::CallableRegistration {
                package: data::Text::Static("support"),
                module: data::Text::Static("support/private"),
                name: data::Text::Static("add"),
                arguments: data::Storage::Static(&[
                    data::host::RegistrationType::Int,
                ]),
                captures: data::Storage::Static(&[
                    data::host::RegistrationType::Int,
                ]),
                return_: data::host::RegistrationType::Int,
                returns_value: true,
            },
            construction: data::host::HostCallableConstruction {
                target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Int(data::function::IntFunctionId(2))),
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                },
                parameters: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                        shape: data::type_::ValueShapeId(0),
                    },
                ]),
                captures: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                        shape: data::type_::ValueShapeId(0),
                    },
                ]),
            },
            invocation: data::program::LibraryCallable {
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
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
            captures: data::program::LibraryInputConstructions {
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
        },
        data::program::LibraryNativeConstruction {
            declaration: data::host::CallableRegistration {
                package: data::Text::Static("support"),
                module: data::Text::Static("support/private"),
                name: data::Text::Static("constant"),
                arguments: data::Storage::Static(&[]),
                captures: data::Storage::Static(&[
                    data::host::RegistrationType::Bool,
                ]),
                return_: data::host::RegistrationType::Bool,
                returns_value: true,
            },
            construction: data::host::HostCallableConstruction {
                target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Bool(data::function::BoolFunctionId(0))),
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                },
                parameters: data::Storage::Static(&[]),
                captures: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                        shape: data::type_::ValueShapeId(3),
                    },
                ]),
            },
            invocation: data::program::LibraryCallable {
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
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
            captures: data::program::LibraryInputConstructions {
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
        },
        data::program::LibraryNativeConstruction {
            declaration: data::host::CallableRegistration {
                package: data::Text::Static("support"),
                module: data::Text::Static("support/private"),
                name: data::Text::Static("wrap"),
                arguments: data::Storage::Static(&[
                    data::host::RegistrationType::Int,
                ]),
                captures: data::Storage::Static(&[
                    data::host::RegistrationType::Function {
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Int,
                        ]),
                        return_: data::Storage::Static(&data::host::RegistrationType::Int),
                    },
                ]),
                return_: data::host::RegistrationType::Int,
                returns_value: true,
            },
            construction: data::host::HostCallableConstruction {
                target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Int(data::function::IntFunctionId(3))),
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                },
                parameters: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                        shape: data::type_::ValueShapeId(0),
                    },
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
                        shape: data::type_::ValueShapeId(1),
                    },
                ]),
            },
            invocation: data::program::LibraryCallable {
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
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
            captures: data::program::LibraryInputConstructions {
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
        },
    ]),
}
