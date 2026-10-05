data::HostedModuleArtifact {
    module: data::ModuleArtifact {
        format: 22,
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
pub fn native_predicate(value: Int) -> Bool {
    let predicate = support.wrap(fn(input) { echo input input > value })
    !predicate(value + 1)
}
"#)),
                },
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("support/private"),
                    source_context: None,
                },
            ]),
            main: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Int(data::function::IntFunctionId(0))),
            functions: data::function::FunctionTables {
                value_returns: data::function::ValueFunctionTables {
                    never_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Host(data::host::HostNeverFunctionId(0)),
                        data::function::ValueFunctionEntry::Host(data::host::HostNeverFunctionId(1)),
                    ]),
                    int_functions: data::Storage::Static(&[
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
                                            instructions: 0..5,
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
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    40,
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
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("library", "run", data::source::SourceSpan::new(1136, 1158)),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::IntFunction {
                                                    local: data::graph::IntFunctionLocalId(1),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                                family: data::function::FunctionReturnFamily::Int,
                                                kind: data::graph::FunctionInstructionKind::Call {
                                                    function: data::function::ProfiledFunctionFunctionId::Int(data::function::IntFunctionFunctionId(1)),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("library", "run", data::source::SourceSpan::new(1178, 1202)),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::FunctionCall {
                                                function: data::graph::IntFunctionLocalId(1),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("library", "run", data::source::SourceSpan::new(1218, 1228)),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::IntFunctionId(3),
                                            site: data::source::HostCallSite::from_static("library", "run", data::source::SourceSpan::new(1207, 1229)),
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
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::Int,
                                                    length: 1,
                                                    steps: data::Storage::Static(&[
                                                        data::graph::TransferStep {
                                                            source: 2,
                                                            destination: 0,
                                                        },
                                                    ]),
                                                },
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::IntFunction,
                                                    length: 1,
                                                    steps: data::Storage::Static(&[]),
                                                },
                                            ]),
                                        },
                                    },
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
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::Echo(data::graph::Echo {
                                                subject: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                message: None,
                                                site: data::source::EchoSite::from_static("library", "fail", data::source::SourceSpan::new(921, 943)),
                                                next: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::NeverFunction(data::graph::NeverFunctionLocal {
                                                            id: data::graph::NeverFunctionLocalId(0),
                                                            type_: data::type_::GenericFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                                },
                                                                shape: data::type_::FunctionShape {
                                                                    shape_id: data::type_::ValueShapeId(10),
                                                                    type_: data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                                    },
                                                                },
                                                            },
                                                        }),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 2..2,
                                            terminator: data::graph::Terminator::NeverCall(data::graph::NeverCall {
                                                function: data::graph::NeverCallTarget::Value(data::graph::NeverFunctionLocal {
                                                    id: data::graph::NeverFunctionLocalId(0),
                                                    type_: data::type_::GenericFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                        },
                                                        shape: data::type_::FunctionShape {
                                                            shape_id: data::type_::ValueShapeId(10),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                            },
                                                        },
                                                    },
                                                }),
                                                args: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::NeverFunction,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                                site: data::source::HostCallSite::from_static("library", "fail", data::source::SourceSpan::new(956, 965)),
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
                                                        return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                    },
                                                    shape: data::type_::FunctionShape {
                                                        shape_id: data::type_::ValueShapeId(10),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                        },
                                                    },
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(10),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::NeverFunction(data::graph::NeverFunctionLocal {
                                                    id: data::graph::NeverFunctionLocalId(0),
                                                    type_: data::type_::GenericFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                        },
                                                        shape: data::type_::FunctionShape {
                                                            shape_id: data::type_::ValueShapeId(10),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                            },
                                                        },
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(10),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                },
                                                family: data::function::FunctionReturnFamily::Never,
                                                kind: data::graph::FunctionInstructionKind::Call {
                                                    function: data::function::ProfiledFunctionFunctionId::Never(data::function::NeverFunctionFunctionId {
                                                        index: 0,
                                                        type_: data::type_::GenericFunctionType {
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                            },
                                                            shape: data::type_::FunctionShape {
                                                                shape_id: data::type_::ValueShapeId(10),
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                                },
                                                            },
                                                        },
                                                    }),
                                                    args: data::Storage::Static(&[]),
                                                    site: data::source::HostCallSite::from_static("library", "fail", data::source::SourceSpan::new(897, 916)),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                shape: data::type_::ValueShapeId(11),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("before callable"))),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[]),
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
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Echo(data::graph::Echo {
                                                subject: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                message: None,
                                                site: data::source::EchoSite::from_static("library", "producer", data::source::SourceSpan::new(1022, 1044)),
                                                next: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 0..0,
                                            instructions: 1..1,
                                            terminator: data::graph::Terminator::NeverCall(data::graph::NeverCall {
                                                function: data::graph::NeverCallTarget::Direct(data::function::NeverFunctionId(0)),
                                                args: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                                site: data::source::HostCallSite::from_static("library", "producer", data::source::SourceSpan::new(1057, 1074)),
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                shape: data::type_::ValueShapeId(11),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("before producer"))),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[]),
                            },
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
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
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
                                                site: data::source::HostCallSite::from_static("library", "apply", data::source::SourceSpan::new(128, 143)),
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
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
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
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 8,
                            return_: data::graph::IntLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 9,
                            return_: data::graph::IntLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 13,
                            return_: data::graph::IntLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    float_functions: data::Storage::Static(&[]),
                    string_functions: data::Storage::Static(&[]),
                    bit_array_functions: data::Storage::Static(&[]),
                    utf_codepoint_functions: data::Storage::Static(&[]),
                    custom_functions: data::Storage::Static(&[]),
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
                                            instructions: 0..15,
                                            terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                                subject: data::graph::BoolLocalId(2),
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BoolFunction {
                                                            local: data::graph::BoolFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                            },
                                                        },
                                                        data::graph::ParamLocal::BoolFunction {
                                                            local: data::graph::BoolFunctionLocalId(1),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                            },
                                                        },
                                                        data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::Int {
                                                            local: data::graph::IntListFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                            },
                                                            list_type: data::type_::IntListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                            },
                                                        }),
                                                        data::graph::ParamLocal::BoolFunction {
                                                            local: data::graph::BoolFunctionLocalId(3),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                            },
                                                        },
                                                        data::graph::ParamLocal::IntFunction {
                                                            local: data::graph::IntFunctionLocalId(2),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                            },
                                                        },
                                                        data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                                                            id: data::graph::CoreFunctionFunctionLocalId(1),
                                                            type_: data::type_::FunctionFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::Int,
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                                    })),
                                                                },
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueShapeId(0),
                                                                ]),
                                                                return_: data::type_::FunctionShape {
                                                                    shape_id: data::type_::ValueShapeId(2),
                                                                    type_: data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                                    },
                                                                },
                                                            },
                                                        })),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntList,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 2,
                                                                        destination: 0,
                                                                    },
                                                                ]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BoolFunction,
                                                                length: 3,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 3,
                                                                        destination: 2,
                                                                    },
                                                                ]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CoreFunctionFunction,
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
                                                                family: data::graph::StorageFamily::Bool,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntList,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BoolFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntListFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CoreFunctionFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 0..6,
                                            instructions: 15..15,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::Equal {
                                                    left: data::graph::ParamLocal::BoolFunction {
                                                        local: data::graph::BoolFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                        },
                                                    },
                                                    right: data::graph::ParamLocal::BoolFunction {
                                                        local: data::graph::BoolFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                        },
                                                    },
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BoolFunction {
                                                            local: data::graph::BoolFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                            },
                                                        },
                                                        data::graph::ParamLocal::BoolFunction {
                                                            local: data::graph::BoolFunctionLocalId(1),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                            },
                                                        },
                                                        data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::Int {
                                                            local: data::graph::IntListFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                            },
                                                            list_type: data::type_::IntListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                            },
                                                        }),
                                                        data::graph::ParamLocal::BoolFunction {
                                                            local: data::graph::BoolFunctionLocalId(2),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
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
                                                        data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                                                            id: data::graph::CoreFunctionFunctionLocalId(0),
                                                            type_: data::type_::FunctionFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::Int,
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                                    })),
                                                                },
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueShapeId(0),
                                                                ]),
                                                                return_: data::type_::FunctionShape {
                                                                    shape_id: data::type_::ValueShapeId(2),
                                                                    type_: data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                                    },
                                                                },
                                                            },
                                                        })),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(16),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BoolFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntListFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CoreFunctionFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 6..12,
                                            instructions: 15..15,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::NotEqual {
                                                    left: data::graph::ParamLocal::BoolFunction {
                                                        local: data::graph::BoolFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                        },
                                                    },
                                                    right: data::graph::ParamLocal::BoolFunction {
                                                        local: data::graph::BoolFunctionLocalId(1),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                        },
                                                    },
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(3),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::Int {
                                                            local: data::graph::IntListFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                            },
                                                            list_type: data::type_::IntListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                            },
                                                        }),
                                                        data::graph::ParamLocal::BoolFunction {
                                                            local: data::graph::BoolFunctionLocalId(2),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
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
                                                        data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                                                            id: data::graph::CoreFunctionFunctionLocalId(0),
                                                            type_: data::type_::FunctionFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::Int,
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                                    })),
                                                                },
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueShapeId(0),
                                                                ]),
                                                                return_: data::type_::FunctionShape {
                                                                    shape_id: data::type_::ValueShapeId(2),
                                                                    type_: data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                                    },
                                                                },
                                                            },
                                                        })),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BoolFunction,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 2,
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
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BoolFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntListFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CoreFunctionFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 12..16,
                                            instructions: 15..18,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::Equal {
                                                    left: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(0),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                        },
                                                    }),
                                                    right: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(1),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                        },
                                                    }),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(4),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BoolFunction {
                                                            local: data::graph::BoolFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
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
                                                        data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                                                            id: data::graph::CoreFunctionFunctionLocalId(0),
                                                            type_: data::type_::FunctionFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::Int,
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                                    })),
                                                                },
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueShapeId(0),
                                                                ]),
                                                                return_: data::type_::FunctionShape {
                                                                    shape_id: data::type_::ValueShapeId(2),
                                                                    type_: data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                                    },
                                                                },
                                                            },
                                                        })),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntList,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntListFunction,
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
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntList,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BoolFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntListFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CoreFunctionFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 16..19,
                                            instructions: 18..21,
                                            terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                                subject: data::graph::BoolLocalId(0),
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(5),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                                                            id: data::graph::CoreFunctionFunctionLocalId(0),
                                                            type_: data::type_::FunctionFunctionType {
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[
                                                                        data::type_::ValueType::Int,
                                                                    ]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                                    })),
                                                                },
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueShapeId(0),
                                                                ]),
                                                                return_: data::type_::FunctionShape {
                                                                    shape_id: data::type_::ValueShapeId(2),
                                                                    type_: data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                                    },
                                                                },
                                                            },
                                                        })),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BoolFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(10),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BoolFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CoreFunctionFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 19..20,
                                            instructions: 21..24,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::EqualInt {
                                                    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                                    right: data::graph::IntegerOperand::Immediate(42),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(6),
                                                    args: data::Storage::Static(&[]),
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
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CoreFunctionFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(8),
                                                    args: data::Storage::Static(&[]),
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
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CoreFunctionFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 20..20,
                                            instructions: 24..25,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(7),
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
                                            params: 20..21,
                                            instructions: 25..25,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 21..21,
                                            instructions: 25..25,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(9),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 21..21,
                                            instructions: 25..26,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(7),
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
                                            params: 21..21,
                                            instructions: 26..26,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(11),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 21..21,
                                            instructions: 26..26,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(9),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 21..21,
                                            instructions: 26..26,
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
                                            params: 21..21,
                                            instructions: 26..26,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(11),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 21..21,
                                            instructions: 26..26,
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
                                            params: 21..21,
                                            instructions: 26..26,
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
                                            params: 21..21,
                                            instructions: 26..26,
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
                                            params: 21..21,
                                            instructions: 26..26,
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
                                            params: 21..21,
                                            instructions: 26..26,
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
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BoolFunction {
                                                local: data::graph::BoolFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BoolFunction {
                                                local: data::graph::BoolFunctionLocalId(1),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::Int {
                                                local: data::graph::IntListFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                },
                                                list_type: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BoolFunction {
                                                local: data::graph::BoolFunctionLocalId(2),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(7),
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
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                                                id: data::graph::CoreFunctionFunctionLocalId(0),
                                                type_: data::type_::FunctionFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        })),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::FunctionShape {
                                                        shape_id: data::type_::ValueShapeId(2),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        },
                                                    },
                                                },
                                            })),
                                            shape: data::type_::ValueShapeId(8),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BoolFunction {
                                                local: data::graph::BoolFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BoolFunction {
                                                local: data::graph::BoolFunctionLocalId(1),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::Int {
                                                local: data::graph::IntListFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                },
                                                list_type: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BoolFunction {
                                                local: data::graph::BoolFunctionLocalId(2),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(7),
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
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                                                id: data::graph::CoreFunctionFunctionLocalId(0),
                                                type_: data::type_::FunctionFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        })),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::FunctionShape {
                                                        shape_id: data::type_::ValueShapeId(2),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        },
                                                    },
                                                },
                                            })),
                                            shape: data::type_::ValueShapeId(8),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::Int {
                                                local: data::graph::IntListFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                },
                                                list_type: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BoolFunction {
                                                local: data::graph::BoolFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(7),
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
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                                                id: data::graph::CoreFunctionFunctionLocalId(0),
                                                type_: data::type_::FunctionFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        })),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::FunctionShape {
                                                        shape_id: data::type_::ValueShapeId(2),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        },
                                                    },
                                                },
                                            })),
                                            shape: data::type_::ValueShapeId(8),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BoolFunction {
                                                local: data::graph::BoolFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(7),
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
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                                                id: data::graph::CoreFunctionFunctionLocalId(0),
                                                type_: data::type_::FunctionFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        })),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::FunctionShape {
                                                        shape_id: data::type_::ValueShapeId(2),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        },
                                                    },
                                                },
                                            })),
                                            shape: data::type_::ValueShapeId(8),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                                                id: data::graph::CoreFunctionFunctionLocalId(0),
                                                type_: data::type_::FunctionFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        })),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::FunctionShape {
                                                        shape_id: data::type_::ValueShapeId(2),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        },
                                                    },
                                                },
                                            })),
                                            shape: data::type_::ValueShapeId(8),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BoolFunction {
                                                    local: data::graph::BoolFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(4),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                                family: data::function::FunctionReturnFamily::Bool,
                                                kind: data::graph::FunctionInstructionKind::Call {
                                                    function: data::function::ProfiledFunctionFunctionId::Bool(data::function::BoolFunctionFunctionId(0)),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1261, 1288)),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BoolFunction {
                                                    local: data::graph::BoolFunctionLocalId(1),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(4),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                                family: data::function::FunctionReturnFamily::Bool,
                                                kind: data::graph::FunctionInstructionKind::Call {
                                                    function: data::function::ProfiledFunctionFunctionId::Bool(data::function::BoolFunctionFunctionId(0)),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1319, 1346)),
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
                                                    42,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(5),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Int(data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                                data::graph::IntLocalId(0),
                                            ])))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::ListFunction(data::graph::ListFunctionLocal::Int {
                                                    local: data::graph::IntListFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                    },
                                                    list_type: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                },
                                                family: data::function::FunctionReturnFamily::List,
                                                kind: data::graph::FunctionInstructionKind::Call {
                                                    function: data::function::ProfiledFunctionFunctionId::List(data::function::ProfiledListFunctionFunctionId::Int {
                                                        id: data::function::IntListFunctionFunctionId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                        },
                                                        list_type: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                        },
                                                    }),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                            local: data::graph::IntListLocalId(0),
                                                            type_id: data::type_::IntListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                            },
                                                        }),
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1362, 1389)),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BoolFunction {
                                                    local: data::graph::BoolFunctionLocalId(2),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(7),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                                family: data::function::FunctionReturnFamily::Bool,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Bool(data::function::BoolFunctionId(2)),
                                                    captures: data::Storage::Static(&[]),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BoolFunction {
                                                    local: data::graph::BoolFunctionLocalId(3),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(7),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                                family: data::function::FunctionReturnFamily::Bool,
                                                kind: data::graph::FunctionInstructionKind::Call {
                                                    function: data::function::ProfiledFunctionFunctionId::Bool(data::function::BoolFunctionFunctionId(1)),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BoolFunction {
                                                            local: data::graph::BoolFunctionLocalId(2),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                            },
                                                        },
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1410, 1449)),
                                                },
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
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(4)),
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
                                                    function: data::function::ProfiledFunctionFunctionId::Int(data::function::IntFunctionFunctionId(2)),
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
                                                    site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1483, 1520)),
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
                                                    function: data::function::ProfiledFunctionFunctionId::Int(data::function::IntFunctionFunctionId(2)),
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
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1470, 1521)),
                                                },
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
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                            })),
                                                        },
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueShapeId(0),
                                                        ]),
                                                        return_: data::type_::FunctionShape {
                                                            shape_id: data::type_::ValueShapeId(2),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                            },
                                                        },
                                                    },
                                                })),
                                                shape: data::type_::ValueShapeId(8),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    })),
                                                },
                                                family: data::function::FunctionReturnFamily::Function,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Function(data::function::ProfiledFunctionFunctionId::Int(data::function::IntFunctionFunctionId(3))),
                                                    captures: data::Storage::Static(&[]),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                                                    id: data::graph::CoreFunctionFunctionLocalId(1),
                                                    type_: data::type_::FunctionFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                            })),
                                                        },
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueShapeId(0),
                                                        ]),
                                                        return_: data::type_::FunctionShape {
                                                            shape_id: data::type_::ValueShapeId(2),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                            },
                                                        },
                                                    },
                                                })),
                                                shape: data::type_::ValueShapeId(8),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    })),
                                                },
                                                family: data::function::FunctionReturnFamily::Function,
                                                kind: data::graph::FunctionInstructionKind::Call {
                                                    function: data::function::ProfiledFunctionFunctionId::Function(data::function::FunctionFunctionFunctionId {
                                                        index: 0,
                                                        type_: data::type_::FunctionFunctionType {
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                                })),
                                                            },
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueShapeId(0),
                                                            ]),
                                                            return_: data::type_::FunctionShape {
                                                                shape_id: data::type_::ValueShapeId(2),
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
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
                                                                        arguments: data::Storage::Static(&[]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                                    })),
                                                                },
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueShapeId(0),
                                                                ]),
                                                                return_: data::type_::FunctionShape {
                                                                    shape_id: data::type_::ValueShapeId(2),
                                                                    type_: data::type_::FunctionType {
                                                                        arguments: data::Storage::Static(&[]),
                                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                                    },
                                                                },
                                                            },
                                                        })),
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1539, 1595)),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::FunctionCall {
                                                function: data::graph::BoolFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1600, 1603)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(5),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Int(data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            }, data::graph::TypedListInstruction::FunctionCall {
                                                function: data::graph::ListFunctionLocal::Int {
                                                    local: data::graph::IntListFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                                    },
                                                    list_type: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                },
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1631, 1637)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(0),
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
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(1),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(5),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Int(data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                                data::graph::IntLocalId(0),
                                            ])))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                                site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1659, 1672)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::FunctionCall {
                                                function: data::graph::BoolFunctionLocalId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1649, 1673)),
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
                                                    42,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::IntFunction {
                                                    local: data::graph::IntFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                                family: data::function::FunctionReturnFamily::Int,
                                                kind: data::graph::FunctionInstructionKind::FunctionCall {
                                                    function: data::graph::CoreFunctionFunctionLocal {
                                                        id: data::graph::CoreFunctionFunctionLocalId(0),
                                                        type_: data::type_::FunctionFunctionType {
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                                })),
                                                            },
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueShapeId(0),
                                                            ]),
                                                            return_: data::type_::FunctionShape {
                                                                shape_id: data::type_::ValueShapeId(2),
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1677, 1687)),
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
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1677, 1689)),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BoolFunction {
                                                    local: data::graph::BoolFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(7),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                                family: data::function::FunctionReturnFamily::Bool,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Bool(data::function::BoolFunctionId(3)),
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
                                                local: data::graph::ParamLocal::BoolFunction {
                                                    local: data::graph::BoolFunctionLocalId(1),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(7),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                                family: data::function::FunctionReturnFamily::Bool,
                                                kind: data::graph::FunctionInstructionKind::Call {
                                                    function: data::function::ProfiledFunctionFunctionId::Bool(data::function::BoolFunctionFunctionId(1)),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BoolFunction {
                                                            local: data::graph::BoolFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                            },
                                                        },
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("library", "native_predicate", data::source::SourceSpan::new(1764, 1816)),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::FunctionCall {
                                                function: data::graph::BoolFunctionLocalId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("library", "native_predicate", data::source::SourceSpan::new(1822, 1842)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Not(data::graph::BoolLocalId(0)))),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::BoolLocalId(1)),
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
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(3),
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
                                parameter_count: 1,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Echo(data::graph::Echo {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                message: None,
                                                site: data::source::EchoSite::from_static("library", "<anonymous:6>", data::source::SourceSpan::new(1789, 1799)),
                                                next: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..4,
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
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::GtInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            })),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 10,
                            return_: data::graph::BoolLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 12,
                            return_: data::graph::BoolLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    nil_functions: data::Storage::Static(&[]),
                    tuple_functions: data::Storage::Static(&[]),
                },
                list_returns: data::function::ListFunctionTables {
                    parameter_list_functions: data::Storage::Static(&[]),
                    int_list_functions: data::Storage::Static(&[
                        (data::function::IntListFunctionId {
                            index: 0,
                            type_id: data::type_::IntListTypeId {
                                list_type: data::type_::ListTypeId(0),
                            },
                        }, data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 11,
                            return_: data::graph::IntListLocalId(0),
                            body: ::core::marker::PhantomData,
                        }))),
                    ]),
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
                            index: 0,
                            return_: data::graph::IntFunctionLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 1,
                            return_: data::graph::IntFunctionLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 5,
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
                                        arguments: data::Storage::Static(&[]),
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
                                                function: data::function::IntFunctionFunctionId(1),
                                                site: data::source::HostCallSite::from_static("library", "<anonymous:5>", data::source::SourceSpan::new(1564, 1592)),
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
                            index: 14,
                            return_: data::graph::IntFunctionLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    float_function_functions: data::Storage::Static(&[]),
                    string_function_functions: data::Storage::Static(&[]),
                    bit_array_function_functions: data::Storage::Static(&[]),
                    utf_codepoint_function_functions: data::Storage::Static(&[]),
                    custom_function_functions: data::Storage::Static(&[]),
                    external_function_functions: data::Storage::Static(&[]),
                    bool_function_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 2,
                            return_: data::graph::BoolFunctionLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 4,
                            return_: data::graph::BoolFunctionLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    nil_function_functions: data::Storage::Static(&[]),
                    tuple_function_functions: data::Storage::Static(&[]),
                    generic_function_functions: data::Storage::Static(&[]),
                    never_function_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 7,
                            return_: data::graph::NeverFunctionLocal {
                                id: data::graph::NeverFunctionLocalId(0),
                                type_: data::type_::GenericFunctionType {
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                    },
                                    shape: data::type_::FunctionShape {
                                        shape_id: data::type_::ValueShapeId(10),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                        },
                                    },
                                },
                            },
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    parameter_list_function_functions: data::Storage::Static(&[]),
                    parameter_list_list_function_functions: data::Storage::Static(&[]),
                    int_list_function_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 3,
                            return_: data::graph::ListFunctionLocal::Int {
                                local: data::graph::IntListFunctionLocalId(0),
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[]),
                                    return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                                },
                                list_type: data::type_::IntListTypeId {
                                    list_type: data::type_::ListTypeId(0),
                                },
                            },
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
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
                            index: 6,
                            return_: data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                                id: data::graph::CoreFunctionFunctionLocalId(0),
                                type_: data::type_::FunctionFunctionType {
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        })),
                                    },
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueShapeId(0),
                                    ]),
                                    return_: data::type_::FunctionShape {
                                        shape_id: data::type_::ValueShapeId(2),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
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
                use data::compiled::calls::{BoolCallable, CallArguments, CallCapture, CallCaptureInputs, CallExecution, CallInputs, CallInteger, CallOps, CallOutput, CallProgress, CallStorage, CallValues, IntCallable};
                use data::compiled::int_list::IntList;
                enum FunctionState {
                    Int0Point0 {  },
                    Int0Point1 { int0: i128 },
                    Int0Point2 { int0: i128, int_function0: IntCallable },
                    Int0Point3 { int0: i128, int_function0: IntCallable, int1: i128 },
                    Int0Point4 { int0: i128, int_function0: IntCallable, int1: i128, int_function1: IntCallable },
                    Int0Point5 { int0: i128, int_function0: IntCallable, int1: i128, int_function1: IntCallable, int2: i128 },
                    Int3Point0 { int_function0: IntCallable, int0: i128 },
                    Int3Point1 { int_function0: IntCallable, int0: i128, int1: i128 },
                    Int4Point0 { int0: i128 },
                    Int4Point1 { int0: i128, int1: i128 },
                    Bool0Point0 {  },
                    Bool0Point1 { bool0: bool },
                    Bool0Point2 { bool0: bool, bool_function0: BoolCallable },
                    Bool0Point3 { bool0: bool, bool_function0: BoolCallable, bool1: bool },
                    Bool0Point4 { bool0: bool, bool_function0: BoolCallable, bool1: bool, bool_function1: BoolCallable },
                    Bool0Point5 { bool0: bool, bool_function0: BoolCallable, bool1: bool, bool_function1: BoolCallable, int0: i128 },
                    Bool0Point6 { bool0: bool, bool_function0: BoolCallable, bool1: bool, bool_function1: BoolCallable, int0: i128, int_list0: IntList },
                    Bool0Point7 {  },
                    Bool0Point8 { bool0: bool },
                    Bool0Point9 { bool0: bool },
                    Bool0Point10 {  },
                    Bool0Point11 {  },
                    Bool0Point12 { bool0: bool },
                    Bool0Point13 {  },
                    Bool0Point14 {  },
                    Bool0Point15 {  },
                    Bool0Point16 {  },
                    Bool0Point17 {  },
                    Bool0Point18 {  },
                    Bool0Point19 {  },
                    Bool0Point20 {  },
                    Bool0Point21 {  },
                    Bool1Point0 { int0: i128 },
                    Bool1Point1 { int0: i128, bool_function0: BoolCallable },
                    Bool1Point2 { int0: i128, bool_function0: BoolCallable, bool_function1: BoolCallable },
                    Bool1Point3 { int0: i128, bool_function0: BoolCallable, bool_function1: BoolCallable, int1: i128 },
                    Bool1Point4 { int0: i128, bool_function0: BoolCallable, bool_function1: BoolCallable, int1: i128, bool0: bool },
                    Bool1Point5 { int0: i128, bool_function0: BoolCallable, bool_function1: BoolCallable, int1: i128, bool0: bool, bool1: bool },
                    Bool2Point0 { int0: i128 },
                    Bool2Point1 { int0: i128, bool0: bool },
                    IntFunction3Point0 { int0: i128 },
                    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: CallValues },
                }
                impl FunctionState {
                    fn values(self) -> CallValues {
                        match self {
                            Self::Int0Point0 {  } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int0Point1 { int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int0Point2 { int0, int_function0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int0Point3 { int0, int_function0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int0Point4 { int0, int_function0, int1, int_function1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0, int_function1], bool_functions: vec![] }
                            },
                            Self::Int0Point5 { int0, int_function0, int1, int_function1, int2 } => {
                                CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0, int_function1], bool_functions: vec![] }
                            },
                            Self::Int3Point0 { int_function0, int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int3Point1 { int_function0, int0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int4Point0 { int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int4Point1 { int0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point0 {  } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point1 { bool0 } => {
                                CallValues { ints: vec![], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point2 { bool0, bool_function0 } => {
                                CallValues { ints: vec![], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0] }
                            },
                            Self::Bool0Point3 { bool0, bool_function0, bool1 } => {
                                CallValues { ints: vec![], bools: vec![bool0, bool1], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0] }
                            },
                            Self::Bool0Point4 { bool0, bool_function0, bool1, bool_function1 } => {
                                CallValues { ints: vec![], bools: vec![bool0, bool1], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0, bool_function1] }
                            },
                            Self::Bool0Point5 { bool0, bool_function0, bool1, bool_function1, int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![bool0, bool1], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0, bool_function1] }
                            },
                            Self::Bool0Point6 { bool0, bool_function0, bool1, bool_function1, int0, int_list0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![bool0, bool1], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![bool_function0, bool_function1] }
                            },
                            Self::Bool0Point7 {  } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point8 { bool0 } => {
                                CallValues { ints: vec![], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point9 { bool0 } => {
                                CallValues { ints: vec![], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point10 {  } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point11 {  } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point12 { bool0 } => {
                                CallValues { ints: vec![], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point13 {  } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point14 {  } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point15 {  } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point16 {  } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point17 {  } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point18 {  } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point19 {  } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point20 {  } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point21 {  } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool1Point0 { int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool1Point1 { int0, bool_function0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0] }
                            },
                            Self::Bool1Point2 { int0, bool_function0, bool_function1 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0, bool_function1] }
                            },
                            Self::Bool1Point3 { int0, bool_function0, bool_function1, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0, bool_function1] }
                            },
                            Self::Bool1Point4 { int0, bool_function0, bool_function1, int1, bool0 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0, bool_function1] }
                            },
                            Self::Bool1Point5 { int0, bool_function0, bool_function1, int1, bool0, bool1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![bool0, bool1], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0, bool_function1] }
                            },
                            Self::Bool2Point0 { int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool2Point1 { int0, bool0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::IntFunction3Point0 { int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Canonical { values, .. } => values,
                        }
                    }
                }
                enum IntReturn {
                    Int0Call4 { int0: i128, int_function0: IntCallable, int1: i128, int_function1: IntCallable },
                    Int3Call0 { int_function0: IntCallable, int0: i128 },
                }
                impl IntReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                            Self::Int0Call4 { .. } => data::source::HostCallSite::from_static("library", "run", data::source::SourceSpan::new(1218, 1228)),
                            Self::Int3Call0 { .. } => data::source::HostCallSite::from_static("library", "apply", data::source::SourceSpan::new(128, 143)),
                        }
                    }
                    fn small(self, result: i128) -> FunctionState {
                        match self {
                            Self::Int0Call4 { int0, int_function0, int1, int_function1 } => {
                                let int2 = result;
                                FunctionState::Int0Point5 { int0, int_function0, int1, int_function1, int2 }
                            },
                            Self::Int3Call0 { int_function0, int0 } => {
                                let int1 = result;
                                FunctionState::Int3Point1 { int_function0, int0, int1 }
                            },
                        }
                    }
                    fn resume(self, result: CallInteger) -> FunctionState {
                        if let Some(result) = result.small() {
                            return self.small(result);
                        }
                        match self {
                            Self::Int0Call4 { int0, int_function0, int1, int_function1 } => {
                                let int2 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 5,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 2,
                                    bool_functions: 0,
                                }, values: CallValues { ints: vec![int0.into(), int1.into(), int2], bools: vec![], int_lists: vec![], int_functions: vec![int_function0, int_function1], bool_functions: vec![] } }
                            },
                            Self::Int3Call0 { int_function0, int0 } => {
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
                                }, values: CallValues { ints: vec![int0.into(), int1], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] } }
                            },
                        }
                    }
                }
                enum BoolReturn {
                    Bool1Call3 { int0: i128, bool_function0: BoolCallable, bool_function1: BoolCallable, int1: i128 },
                }
                impl BoolReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                            Self::Bool1Call3 { .. } => data::source::HostCallSite::from_static("library", "native_predicate", data::source::SourceSpan::new(1822, 1842)),
                        }
                    }
                    fn small(self, result: bool) -> FunctionState {
                        match self {
                            Self::Bool1Call3 { int0, bool_function0, bool_function1, int1 } => {
                                let bool0 = result;
                                FunctionState::Bool1Point4 { int0, bool_function0, bool_function1, int1, bool0 }
                            },
                        }
                    }
                    fn resume(self, result: bool) -> FunctionState { self.small(result) }
                }
                enum IntFunctionReturn {
                    Int0Call1 { int0: i128 },
                    Int0Call3 { int0: i128, int_function0: IntCallable, int1: i128 },
                }
                impl IntFunctionReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                            Self::Int0Call1 { .. } => data::source::HostCallSite::from_static("library", "run", data::source::SourceSpan::new(1136, 1158)),
                            Self::Int0Call3 { .. } => data::source::HostCallSite::from_static("library", "run", data::source::SourceSpan::new(1178, 1202)),
                        }
                    }
                    fn small(self, result: IntCallable) -> FunctionState {
                        match self {
                            Self::Int0Call1 { int0 } => {
                                let int_function0 = result.with_type(data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                });
                                FunctionState::Int0Point2 { int0, int_function0 }
                            },
                            Self::Int0Call3 { int0, int_function0, int1 } => {
                                let int_function1 = result.with_type(data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                });
                                FunctionState::Int0Point4 { int0, int_function0, int1, int_function1 }
                            },
                        }
                    }
                    fn resume(self, result: IntCallable) -> FunctionState { self.small(result) }
                }
                enum BoolFunctionReturn {
                    Bool0Call1 { bool0: bool },
                    Bool0Call3 { bool0: bool, bool_function0: BoolCallable, bool1: bool },
                    Bool1Call1 { int0: i128, bool_function0: BoolCallable },
                }
                impl BoolFunctionReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                            Self::Bool0Call1 { .. } => data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1261, 1288)),
                            Self::Bool0Call3 { .. } => data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1319, 1346)),
                            Self::Bool1Call1 { .. } => data::source::HostCallSite::from_static("library", "native_predicate", data::source::SourceSpan::new(1764, 1816)),
                        }
                    }
                    fn small(self, result: BoolCallable) -> FunctionState {
                        match self {
                            Self::Bool0Call1 { bool0 } => {
                                let bool_function0 = result.with_type(data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                });
                                FunctionState::Bool0Point2 { bool0, bool_function0 }
                            },
                            Self::Bool0Call3 { bool0, bool_function0, bool1 } => {
                                let bool_function1 = result.with_type(data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                });
                                FunctionState::Bool0Point4 { bool0, bool_function0, bool1, bool_function1 }
                            },
                            Self::Bool1Call1 { int0, bool_function0 } => {
                                let bool_function1 = result.with_type(data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                });
                                FunctionState::Bool1Point2 { int0, bool_function0, bool_function1 }
                            },
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
                    IntTail { callee: FunctionState, completed: FunctionState, point: data::compiled::CompiledCheckpoint },
                    Int { value: i128, exit: data::graph::BlockGraphExitId },
                    IntBridge { function: data::function::IntFunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: IntReturn },
                    BoolCall { callee: FunctionState, caller: BoolReturn },
                    Bool { value: bool, exit: data::graph::BlockGraphExitId },
                    BoolBridge { function: data::function::BoolFunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: BoolReturn },
                    IntFunctionBridge { function: data::function::IntFunctionFunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: IntFunctionReturn },
                    BoolFunctionBridge { function: data::function::BoolFunctionFunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: BoolFunctionReturn },
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
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(3)) => calls_int_3_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(4)) => calls_int_4_state(point, values),
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)) => calls_bool_0_state(point, values),
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)) => calls_bool_1_state(point, values),
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(2)) => calls_bool_2_state(point, values),
                            data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(3)) => calls_intfunction_3_state(point, values),
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
                                FunctionStep::IntTail { callee, completed, point } => {
                                    if self.integer_returns.is_empty() {
                                        return CallProgress::Interpreted { point, values: completed.values() };
                                    }
                                    *budget -= 1;
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
                                FunctionStep::BoolCall { callee, caller } => {
                                    self.boolean_returns.push(caller);
                                    active = callee;
                                },
                                FunctionStep::Bool { value, exit } => {
                                    if let Some(caller) = self.boolean_returns.pop() {
                                        active = caller.small(value);
                                    } else {
                                        self.integer_returns.clear();
                                        self.boolean_returns.clear();
                                        self.integer_function_returns.clear();
                                        self.boolean_function_returns.clear();
                                        return CallProgress::Complete { exit, output: CallOutput::Bool(value), execution: self };
                                    }
                                },
                                FunctionStep::BoolBridge { function, site, arguments, caller } => return CallProgress::Bool {
                                    function, site, arguments,
                                    resume: Box::new(move |value| {
                                        self.active = Some(caller.resume(value));
                                        self
                                    }),
                                },
                                FunctionStep::IntFunctionBridge { function, site, arguments, caller } => return CallProgress::IntFunction {
                                    function, site, arguments,
                                    resume: Box::new(move |value| {
                                        self.active = Some(caller.resume(value));
                                        self
                                    }),
                                },
                                FunctionStep::BoolFunctionBridge { function, site, arguments, caller } => return CallProgress::BoolFunction {
                                    function, site, arguments,
                                    resume: Box::new(move |value| {
                                        self.active = Some(caller.resume(value));
                                        self
                                    }),
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
                        FunctionState::Int0Point0 {  } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point0 {  }); }
                            *budget -= 1;
                            let int0 = 40_i128;
                            if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                            FunctionStep::Next(FunctionState::Int0Point1 { int0 })
                        },
                        FunctionState::Int0Point1 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point1 { int0 }); }
                            *budget -= 1;
                            FunctionStep::IntFunctionBridge { function: data::function::IntFunctionFunctionId(0), site: data::source::HostCallSite::from_static("library", "run", data::source::SourceSpan::new(1136, 1158)), arguments: CallArguments { values: CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }, captures: None }, caller: IntFunctionReturn::Int0Call1 { int0 } }
                        },
                        FunctionState::Int0Point2 { int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point2 { int0, int_function0 }); }
                            *budget -= 1;
                            let int1 = 2_i128;
                            if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 3,
                                ints: 2,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 1,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] } }; }
                            FunctionStep::Next(FunctionState::Int0Point3 { int0, int_function0, int1 })
                        },
                        FunctionState::Int0Point3 { int0, int_function0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point3 { int0, int_function0, int1 }); }
                            *budget -= 1;
                            FunctionStep::IntFunctionBridge { function: data::function::IntFunctionFunctionId(1), site: data::source::HostCallSite::from_static("library", "run", data::source::SourceSpan::new(1178, 1202)), arguments: CallArguments { values: CallValues { ints: vec![int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }, captures: None }, caller: IntFunctionReturn::Int0Call3 { int0, int_function0, int1 } }
                        },
                        FunctionState::Int0Point4 { int0, int_function0, int1, int_function1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point4 { int0, int_function0, int1, int_function1 }); }
                            *budget -= 1;
                            let callable = &int_function1;
                            let captures = callable.captures();
                            let target = callable.target();
                            if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, ()) {
                                return FunctionStep::IntCall { callee, caller: IntReturn::Int0Call4 { int0, int_function0, int1, int_function1 } };
                            }
                            FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("library", "run", data::source::SourceSpan::new(1218, 1228)), arguments: CallArguments { values: CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }, captures: Some(captures.retain()) }, caller: IntReturn::Int0Call4 { int0, int_function0, int1, int_function1 } }
                        },
                        FunctionState::Int0Point5 { int0, int_function0, int1, int_function1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point5 { int0, int_function0, int1, int_function1, int2 }); }
                            FunctionStep::IntTail { callee: FunctionState::Int3Point0 { int_function0: int_function0.clone(), int0: int2 }, completed: FunctionState::Int0Point5 { int0, int_function0, int1, int_function1, int2 }, point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 5,
                                ints: 3,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 2,
                                bool_functions: 0,
                            } }
                        },
                        FunctionState::Int3Point0 { int_function0, int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point0 { int_function0, int0 }); }
                            *budget -= 1;
                            let callable = &int_function0;
                            let captures = callable.captures();
                            let target = callable.target();
                            if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_1(target, &captures, (int0,)) {
                                return FunctionStep::IntCall { callee, caller: IntReturn::Int3Call0 { int_function0, int0 } };
                            }
                            FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("library", "apply", data::source::SourceSpan::new(128, 143)), arguments: CallArguments { values: CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }, captures: Some(captures.retain()) }, caller: IntReturn::Int3Call0 { int_function0, int0 } }
                        },
                        FunctionState::Int3Point1 { int_function0, int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point1 { int_function0, int0, int1 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int1, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int4Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int4Point0 { int0 }); }
                            *budget -= 1;
                            let int1 = int0 + 1_i128;
                            if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(4)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int4Point1 { int0, int1 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int1, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int4Point1 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int4Point1 { int0, int1 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int1, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Bool0Point0 {  } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point0 {  }); }
                            *budget -= 1;
                            let bool0 = true;
                            FunctionStep::Next(FunctionState::Bool0Point1 { bool0 })
                        },
                        FunctionState::Bool0Point1 { bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point1 { bool0 }); }
                            *budget -= 1;
                            FunctionStep::BoolFunctionBridge { function: data::function::BoolFunctionFunctionId(0), site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1261, 1288)), arguments: CallArguments { values: CallValues { ints: vec![], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }, captures: None }, caller: BoolFunctionReturn::Bool0Call1 { bool0 } }
                        },
                        FunctionState::Bool0Point2 { bool0, bool_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point2 { bool0, bool_function0 }); }
                            *budget -= 1;
                            let bool1 = true;
                            FunctionStep::Next(FunctionState::Bool0Point3 { bool0, bool_function0, bool1 })
                        },
                        FunctionState::Bool0Point3 { bool0, bool_function0, bool1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point3 { bool0, bool_function0, bool1 }); }
                            *budget -= 1;
                            FunctionStep::BoolFunctionBridge { function: data::function::BoolFunctionFunctionId(0), site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1319, 1346)), arguments: CallArguments { values: CallValues { ints: vec![], bools: vec![bool1], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }, captures: None }, caller: BoolFunctionReturn::Bool0Call3 { bool0, bool_function0, bool1 } }
                        },
                        FunctionState::Bool0Point4 { bool0, bool_function0, bool1, bool_function1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point4 { bool0, bool_function0, bool1, bool_function1 }); }
                            *budget -= 1;
                            let int0 = 42_i128;
                            if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 5,
                                ints: 1,
                                bools: 2,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 2,
                            }, values: CallValues { ints: vec![int0.into()], bools: vec![bool0, bool1], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0, bool_function1] } }; }
                            FunctionStep::Next(FunctionState::Bool0Point5 { bool0, bool_function0, bool1, bool_function1, int0 })
                        },
                        FunctionState::Bool0Point5 { bool0, bool_function0, bool1, bool_function1, int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point5 { bool0, bool_function0, bool1, bool_function1, int0 }); }
                            *budget -= 1;
                            let int_list0 = ops.lists().value(data::type_::IntListTypeId {
                                list_type: data::type_::ListTypeId(0),
                            }, &[int0 as i64]);
                            FunctionStep::Next(FunctionState::Bool0Point6 { bool0, bool_function0, bool1, bool_function1, int0, int_list0 })
                        },
                        FunctionState::Bool0Point6 { bool0, bool_function0, bool1, bool_function1, int0, int_list0 } => {
                            FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 6,
                                ints: 1,
                                bools: 2,
                                bit_arrays: 0,
                                int_lists: 1,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 2,
                            }, values: CallValues { ints: vec![int0.into()], bools: vec![bool0, bool1], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![bool_function0, bool_function1] } }
                        },
                        FunctionState::Bool0Point7 {  } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point7 {  }); }
                            *budget -= 1;
                            let bool0 = true;
                            FunctionStep::Next(FunctionState::Bool0Point8 { bool0 })
                        },
                        FunctionState::Bool0Point8 { bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point8 { bool0 }); }
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Bool0Point9 { bool0 })
                        },
                        FunctionState::Bool0Point9 { bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point9 { bool0 }); }
                            *budget -= 1;
                            FunctionStep::Bool { value: bool0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Bool0Point10 {  } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point10 {  }); }
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Bool0Point11 {  })
                        },
                        FunctionState::Bool0Point11 {  } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point11 {  }); }
                            *budget -= 1;
                            let bool0 = false;
                            FunctionStep::Next(FunctionState::Bool0Point12 { bool0 })
                        },
                        FunctionState::Bool0Point12 { bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point12 { bool0 }); }
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Bool0Point9 { bool0 })
                        },
                        FunctionState::Bool0Point13 {  } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point13 {  }); }
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Bool0Point14 {  })
                        },
                        FunctionState::Bool0Point14 {  } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point14 {  }); }
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Bool0Point11 {  })
                        },
                        FunctionState::Bool0Point15 {  } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point15 {  }); }
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Bool0Point16 {  })
                        },
                        FunctionState::Bool0Point16 {  } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point16 {  }); }
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Bool0Point14 {  })
                        },
                        FunctionState::Bool0Point17 {  } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point17 {  }); }
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Bool0Point18 {  })
                        },
                        FunctionState::Bool0Point18 {  } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point18 {  }); }
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Bool0Point16 {  })
                        },
                        FunctionState::Bool0Point19 {  } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point19 {  }); }
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Bool0Point20 {  })
                        },
                        FunctionState::Bool0Point20 {  } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point20 {  }); }
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Bool0Point18 {  })
                        },
                        FunctionState::Bool0Point21 {  } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point21 {  }); }
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Bool0Point20 {  })
                        },
                        FunctionState::Bool1Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point0 { int0 }); }
                            *budget -= 1;
                            let bool_function0 = ops.bool_closure(data::function::BoolFunctionId(3), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                            }, vec![CallCapture::int(data::graph::IntLocalId(1), int0)]);
                            FunctionStep::Next(FunctionState::Bool1Point1 { int0, bool_function0 })
                        },
                        FunctionState::Bool1Point1 { int0, bool_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point1 { int0, bool_function0 }); }
                            *budget -= 1;
                            FunctionStep::BoolFunctionBridge { function: data::function::BoolFunctionFunctionId(1), site: data::source::HostCallSite::from_static("library", "native_predicate", data::source::SourceSpan::new(1764, 1816)), arguments: CallArguments { values: CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0.clone()] }, captures: None }, caller: BoolFunctionReturn::Bool1Call1 { int0, bool_function0 } }
                        },
                        FunctionState::Bool1Point2 { int0, bool_function0, bool_function1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point2 { int0, bool_function0, bool_function1 }); }
                            *budget -= 1;
                            let int1 = int0 + 1_i128;
                            if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 3,
                                ints: 2,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 2,
                            }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![bool_function0, bool_function1] } }; }
                            FunctionStep::Next(FunctionState::Bool1Point3 { int0, bool_function0, bool_function1, int1 })
                        },
                        FunctionState::Bool1Point3 { int0, bool_function0, bool_function1, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point3 { int0, bool_function0, bool_function1, int1 }); }
                            *budget -= 1;
                            let callable = &bool_function1;
                            let captures = callable.captures();
                            let target = callable.target();
                            if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_2(target, &captures, (int1,)) {
                                return FunctionStep::BoolCall { callee, caller: BoolReturn::Bool1Call3 { int0, bool_function0, bool_function1, int1 } };
                            }
                            FunctionStep::BoolBridge { function: target, site: data::source::HostCallSite::from_static("library", "native_predicate", data::source::SourceSpan::new(1822, 1842)), arguments: CallArguments { values: CallValues { ints: vec![int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }, captures: Some(captures.retain()) }, caller: BoolReturn::Bool1Call3 { int0, bool_function0, bool_function1, int1 } }
                        },
                        FunctionState::Bool1Point4 { int0, bool_function0, bool_function1, int1, bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point4 { int0, bool_function0, bool_function1, int1, bool0 }); }
                            *budget -= 1;
                            let bool1 = !bool0;
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point5 { int0, bool_function0, bool_function1, int1, bool0, bool1 }); }
                            *budget -= 1;
                            FunctionStep::Bool { value: bool1, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Bool1Point5 { int0, bool_function0, bool_function1, int1, bool0, bool1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point5 { int0, bool_function0, bool_function1, int1, bool0, bool1 }); }
                            *budget -= 1;
                            FunctionStep::Bool { value: bool1, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Bool2Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool2Point0 { int0 }); }
                            *budget -= 1;
                            let bool0 = int0 == 42_i128;
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool2Point1 { int0, bool0 }); }
                            *budget -= 1;
                            FunctionStep::Bool { value: bool0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Bool2Point1 { int0, bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool2Point1 { int0, bool0 }); }
                            *budget -= 1;
                            FunctionStep::Bool { value: bool0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::IntFunction3Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction3Point0 { int0 }); }
                            FunctionStep::Canonical { target: data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(3)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }
                        },
                    }
                }
                fn calls_entry_0(target: data::function::IntFunctionId, _captures: &CallCaptureInputs<'_>, inputs: ()) -> Option<FunctionState> {
                    let () = inputs;
                    match target.0 {
                        0 => Some(FunctionState::Int0Point0 {  }),
                        _ => None,
                    }
                }
                fn calls_entry_1(target: data::function::IntFunctionId, _captures: &CallCaptureInputs<'_>, inputs: (i128,)) -> Option<FunctionState> {
                    let (argument0,) = inputs;
                    match target.0 {
                        4 => Some(FunctionState::Int4Point0 { int0: argument0 }),
                        _ => None,
                    }
                }
                fn calls_entry_2(target: data::function::BoolFunctionId, _captures: &CallCaptureInputs<'_>, inputs: (i128,)) -> Option<FunctionState> {
                    let (argument0,) = inputs;
                    match target.0 {
                        1 => Some(FunctionState::Bool1Point0 { int0: argument0 }),
                        2 => Some(FunctionState::Bool2Point0 { int0: argument0 }),
                        _ => None,
                    }
                }
                fn calls_int_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int0Point0 {  },
                        1 => FunctionState::Int0Point1 { int0: values.int(0)? },
                        2 => FunctionState::Int0Point2 { int0: values.int(0)?, int_function0: values.int_function(0)? },
                        3 => FunctionState::Int0Point3 { int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)? },
                        4 => {
                            if !matches!(values.int_function_target(1)?, data::function::IntFunctionId(0)) { return None; }
                            FunctionState::Int0Point4 { int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)?, int_function1: values.int_function(1)? }
                        },
                        5 => FunctionState::Int0Point5 { int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)?, int_function1: values.int_function(1)?, int2: values.int(2)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point, values) { return Some(execution); }
                    let active = calls_int_0_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_3_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => {
                            if !matches!(values.int_function_target(0)?, data::function::IntFunctionId(4)) { return None; }
                            FunctionState::Int3Point0 { int_function0: values.int_function(0)?, int0: values.int(0)? }
                        },
                        1 => FunctionState::Int3Point1 { int_function0: values.int_function(0)?, int0: values.int(0)?, int1: values.int(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_3_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point, values) { return Some(execution); }
                    let active = calls_int_3_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_4_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int4Point0 { int0: values.int(0)? },
                        1 => FunctionState::Int4Point1 { int0: values.int(0)?, int1: values.int(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_4_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(4)), point, values) { return Some(execution); }
                    let active = calls_int_4_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_bool_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool0Point0 {  },
                        1 => FunctionState::Bool0Point1 { bool0: values.bool(0)? },
                        2 => FunctionState::Bool0Point2 { bool0: values.bool(0)?, bool_function0: values.bool_function(0)? },
                        3 => FunctionState::Bool0Point3 { bool0: values.bool(0)?, bool_function0: values.bool_function(0)?, bool1: values.bool(1)? },
                        4 => FunctionState::Bool0Point4 { bool0: values.bool(0)?, bool_function0: values.bool_function(0)?, bool1: values.bool(1)?, bool_function1: values.bool_function(1)? },
                        5 => FunctionState::Bool0Point5 { bool0: values.bool(0)?, bool_function0: values.bool_function(0)?, bool1: values.bool(1)?, bool_function1: values.bool_function(1)?, int0: values.int(0)? },
                        6 => FunctionState::Bool0Point6 { bool0: values.bool(0)?, bool_function0: values.bool_function(0)?, bool1: values.bool(1)?, bool_function1: values.bool_function(1)?, int0: values.int(0)?, int_list0: values.int_list(0)? },
                        7 => FunctionState::Bool0Point7 {  },
                        8 => FunctionState::Bool0Point8 { bool0: values.bool(0)? },
                        9 => FunctionState::Bool0Point9 { bool0: values.bool(0)? },
                        10 => FunctionState::Bool0Point10 {  },
                        11 => FunctionState::Bool0Point11 {  },
                        12 => FunctionState::Bool0Point12 { bool0: values.bool(0)? },
                        13 => FunctionState::Bool0Point13 {  },
                        14 => FunctionState::Bool0Point14 {  },
                        15 => FunctionState::Bool0Point15 {  },
                        16 => FunctionState::Bool0Point16 {  },
                        17 => FunctionState::Bool0Point17 {  },
                        18 => FunctionState::Bool0Point18 {  },
                        19 => FunctionState::Bool0Point19 {  },
                        20 => FunctionState::Bool0Point20 {  },
                        21 => FunctionState::Bool0Point21 {  },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point, values) { return Some(execution); }
                    let active = calls_bool_0_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_bool_1_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool1Point0 { int0: values.int(0)? },
                        1 => FunctionState::Bool1Point1 { int0: values.int(0)?, bool_function0: values.bool_function(0)? },
                        2 => FunctionState::Bool1Point2 { int0: values.int(0)?, bool_function0: values.bool_function(0)?, bool_function1: values.bool_function(1)? },
                        3 => {
                            if !matches!(values.bool_function_target(1)?, data::function::BoolFunctionId(1) | data::function::BoolFunctionId(2)) { return None; }
                            FunctionState::Bool1Point3 { int0: values.int(0)?, bool_function0: values.bool_function(0)?, bool_function1: values.bool_function(1)?, int1: values.int(1)? }
                        },
                        4 => FunctionState::Bool1Point4 { int0: values.int(0)?, bool_function0: values.bool_function(0)?, bool_function1: values.bool_function(1)?, int1: values.int(1)?, bool0: values.bool(0)? },
                        5 => FunctionState::Bool1Point5 { int0: values.int(0)?, bool_function0: values.bool_function(0)?, bool_function1: values.bool_function(1)?, int1: values.int(1)?, bool0: values.bool(0)?, bool1: values.bool(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_1_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)), point, values) { return Some(execution); }
                    let active = calls_bool_1_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_bool_2_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool2Point0 { int0: values.int(0)? },
                        1 => FunctionState::Bool2Point1 { int0: values.int(0)?, bool0: values.bool(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_2_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(2)), point, values) { return Some(execution); }
                    let active = calls_bool_2_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_intfunction_3_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::IntFunction3Point0 { int0: values.int(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_intfunction_3_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(3)), point, values) { return Some(execution); }
                    let active = calls_intfunction_3_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
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
                                        block: data::graph::BlockId(0),
                                        instruction: 3,
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
                                        instruction: 4,
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
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 5,
                                        ints: 3,
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
                                                arguments: data::Storage::Static(&[]),
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
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(1),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
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
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(0))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("library", "run", data::source::SourceSpan::new(1136, 1158)),
                                    },
                                    data::compiled::CallContract {
                                        point: 3,
                                        output: data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(1),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(1))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("library", "run", data::source::SourceSpan::new(1178, 1202)),
                                    },
                                    data::compiled::CallContract {
                                        point: 4,
                                        output: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        target: data::compiled::CallContractTarget::IntValue(data::graph::IntFunctionLocalId(1)),
                                        args: data::Storage::Static(&[]),
                                        site: data::source::HostCallSite::from_static("library", "run", data::source::SourceSpan::new(1218, 1228)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[]),
                                tails: data::Storage::Static(&[
                                    data::compiled::TailContract {
                                        point: 5,
                                        target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)),
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
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("library", "run", data::source::SourceSpan::new(1207, 1229)),
                                    },
                                ]),
                                start: calls_int_0_start,
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
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                        site: data::source::HostCallSite::from_static("library", "apply", data::source::SourceSpan::new(128, 143)),
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
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(4)),
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
                                start: calls_int_4_start,
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
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 0,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 1,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 3,
                                        ints: 0,
                                        bools: 2,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 1,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 4,
                                        ints: 0,
                                        bools: 2,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 2,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 5,
                                        ints: 1,
                                        bools: 2,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 2,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 6,
                                        ints: 1,
                                        bools: 2,
                                        bit_arrays: 0,
                                        int_lists: 1,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 2,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(6),
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
                                        block: data::graph::BlockId(6),
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
                                        block: data::graph::BlockId(7),
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
                                        block: data::graph::BlockId(8),
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
                                        block: data::graph::BlockId(9),
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
                                        block: data::graph::BlockId(9),
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
                                        block: data::graph::BlockId(11),
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
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(1),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(1),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(1),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
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
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 1,
                                        output: data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(0))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1261, 1288)),
                                    },
                                    data::compiled::CallContract {
                                        point: 3,
                                        output: data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(1),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(0))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1319, 1346)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 9,
                                        value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_bool_0_start,
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
                                        int_functions: 0,
                                        bool_functions: 1,
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
                                        int_functions: 0,
                                        bool_functions: 2,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 3,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 2,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 4,
                                        ints: 2,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 2,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 5,
                                        ints: 2,
                                        bools: 2,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 2,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(1),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(1),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(1),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(1),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 1,
                                        output: data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(1),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(1))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::BoolFunction {
                                                local: data::graph::BoolFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                            },
                                        ]),
                                        site: data::source::HostCallSite::from_static("library", "native_predicate", data::source::SourceSpan::new(1764, 1816)),
                                    },
                                    data::compiled::CallContract {
                                        point: 3,
                                        output: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        target: data::compiled::CallContractTarget::BoolValue(data::graph::BoolFunctionLocalId(1)),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("library", "native_predicate", data::source::SourceSpan::new(1822, 1842)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[
                                    data::compiled::CreationContract {
                                        point: 0,
                                        output: data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        target: data::graph::FunctionTarget::Bool(data::function::BoolFunctionId(3)),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
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
                                        point: 5,
                                        value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_bool_1_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(2)),
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
                                        ints: 1,
                                        bools: 1,
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
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 1,
                                        value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_bool_2_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(3)),
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
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[]),
                                tails: data::Storage::Static(&[
                                    data::compiled::TailContract {
                                        point: 0,
                                        target: data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(1)),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("library", "<anonymous:5>", data::source::SourceSpan::new(1564, 1592)),
                                    },
                                ]),
                                start: calls_intfunction_3_start,
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
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    10..16,
                    0..0,
                    0..0,
                    0..0,
                    16..17,
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
                    17..22,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    22..24,
                    0..0,
                    0..0,
                    0..0,
                    24..25,
                    0..0,
                    0..0,
                    25..26,
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
                    26..27,
                ],
                functions: data::Storage::Static(&[
                    data::function::FunctionContract {
                        parameters: 0..0,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(9),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 0..0,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(9),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 0..0,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 0..0,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 0..0,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 0..2,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
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
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                shape: data::type_::ValueShapeId(0),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 4..4,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                        parameters: 5..5,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(3),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 5..6,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(3),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 6..7,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(3),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 7..8,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(3),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                shape: data::type_::ValueShapeId(0),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 8..8,
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
                        parameters: 8..9,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(3),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::BoolFunction {
                                    local: data::graph::BoolFunctionLocalId(0),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                    },
                                },
                                shape: data::type_::ValueShapeId(7),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 9..9,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(5),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                    local: data::graph::IntListLocalId(0),
                                    type_id: data::type_::IntListTypeId {
                                        list_type: data::type_::ListTypeId(0),
                                    },
                                }),
                                shape: data::type_::ValueShapeId(5),
                            },
                        ]),
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
                        return_: data::type_::ValueShapeId(2),
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
                        parameters: 12..13,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(2),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 13..14,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(2),
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
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            })),
                                        },
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueShapeId(0),
                                        ]),
                                        return_: data::type_::FunctionShape {
                                            shape_id: data::type_::ValueShapeId(2),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    },
                                })),
                                shape: data::type_::ValueShapeId(8),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 14..15,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(3),
                        ]),
                        return_: data::type_::ValueShapeId(4),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 15..16,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(7),
                        ]),
                        return_: data::type_::ValueShapeId(7),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 16..16,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(10),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 16..17,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(5),
                        ]),
                        return_: data::type_::ValueShapeId(6),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 17..18,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(8),
                        ]),
                        return_: data::type_::ValueShapeId(8),
                        captures: data::Storage::Static(&[]),
                    },
                ]),
                parameters: data::Storage::Static(&[
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
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                    data::graph::ParamLocal::BoolFunction {
                        local: data::graph::BoolFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                        },
                    },
                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                        local: data::graph::IntListLocalId(0),
                        type_id: data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
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
                                    arguments: data::Storage::Static(&[]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                })),
                            },
                            arguments: data::Storage::Static(&[
                                data::type_::ValueShapeId(0),
                            ]),
                            return_: data::type_::FunctionShape {
                                shape_id: data::type_::ValueShapeId(2),
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                },
                            },
                        },
                    })),
                ]),
            },
            list_types: data::type_::ListTypeTable {
                types: data::Storage::Static(&[
                    data::type_::ListStorageTypeId::Int(data::type_::IntListTypeId {
                        list_type: data::type_::ListTypeId(0),
                    }),
                ]),
                tuple_items: data::Storage::Static(&[]),
                function_items: data::Storage::Static(&[]),
            },
            custom_types: data::type_::CustomTypeTable {
                types: data::Storage::Static(&[]),
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
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(0),
                    },
                    data::type_::ValueShapeDescriptor::Bool,
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(3),
                    },
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(0)),
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(5),
                    },
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(3),
                    },
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(2),
                    },
                    data::type_::ValueShapeDescriptor::Parameter(data::type_::parameter_id(0)),
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(9),
                    },
                    data::type_::ValueShapeDescriptor::String,
                ]),
                shape_types: data::Storage::Static(&[
                    data::type_::ValueType::Int,
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    }),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    }),
                    data::type_::ValueType::Bool,
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                    }),
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                    }),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                    }),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        })),
                    }),
                    data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                    }),
                    data::type_::ValueType::String,
                ]),
                custom_shapes: data::Storage::Static(&[]),
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
                data::program::LibraryFunctionEntry {
                    function: data::function::IntFunctionId(2),
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
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("check"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("fail"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 1,
            },
            data::Export {
                name: data::Text::Static("producer"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 2,
            },
            data::Export {
                name: data::Text::Static("native_predicate"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
                },
                slot: 1,
            },
        ]),
    },
    value_functions: data::Storage::Static(&[
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
                        target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Int(data::function::IntFunctionId(5))),
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
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("support"),
            site: data::source::HostCallSite::from_static("support", "make_constant", data::source::SourceSpan::new(43, 73)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
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
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[
                    data::host::HostCallableConstruction {
                        target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Int(data::function::IntFunctionId(6))),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        },
                        parameters: data::Storage::Static(&[]),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                })),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[
                    data::host::CallableRegistration {
                        package: data::Text::Static("support"),
                        module: data::Text::Static("support/private"),
                        name: data::Text::Static("constant"),
                        arguments: data::Storage::Static(&[]),
                        captures: data::Storage::Static(&[
                            data::host::RegistrationType::Parameter(0),
                        ]),
                        return_: data::host::RegistrationType::Parameter(0),
                        returns_value: true,
                    },
                ]),
                return_: data::host::RegistrationType::Function {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::host::RegistrationType::Parameter(0)),
                },
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::host::RegistrationType::Parameter(0)),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("support"),
            site: data::source::HostCallSite::from_static("support", "make_constant", data::source::SourceSpan::new(43, 73)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Bool,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Bool,
                    shape: data::type_::ValueShapeId(3),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0))),
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
                        target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Bool(data::function::BoolFunctionId(4))),
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
                ]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Bool,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                })),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[
                    data::host::CallableRegistration {
                        package: data::Text::Static("support"),
                        module: data::Text::Static("support/private"),
                        name: data::Text::Static("constant"),
                        arguments: data::Storage::Static(&[]),
                        captures: data::Storage::Static(&[
                            data::host::RegistrationType::Parameter(0),
                        ]),
                        return_: data::host::RegistrationType::Parameter(0),
                        returns_value: true,
                    },
                ]),
                return_: data::host::RegistrationType::Function {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::host::RegistrationType::Parameter(0)),
                },
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::host::RegistrationType::Parameter(0)),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("support"),
            site: data::source::HostCallSite::from_static("support", "make_constant", data::source::SourceSpan::new(43, 73)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int))),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                    shape: data::type_::ValueShapeId(5),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                        local: data::graph::IntListLocalId(0),
                        type_id: data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
                        },
                    })),
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)), data::type_::ListTypeId(0)),
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
                callables: data::Storage::Static(&[
                    data::host::HostCallableConstruction {
                        target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::List(data::function::ProfiledListFunctionId::Core(data::function::ListFunctionId::Int(data::function::IntListFunctionId {
                            index: 0,
                            type_id: data::type_::IntListTypeId {
                                list_type: data::type_::ListTypeId(0),
                            },
                        })))),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                        },
                        parameters: data::Storage::Static(&[]),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                    local: data::graph::IntListLocalId(0),
                                    type_id: data::type_::IntListTypeId {
                                        list_type: data::type_::ListTypeId(0),
                                    },
                                }),
                                shape: data::type_::ValueShapeId(5),
                            },
                        ]),
                    },
                ]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
                })),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[
                    data::host::CallableRegistration {
                        package: data::Text::Static("support"),
                        module: data::Text::Static("support/private"),
                        name: data::Text::Static("constant"),
                        arguments: data::Storage::Static(&[]),
                        captures: data::Storage::Static(&[
                            data::host::RegistrationType::Parameter(0),
                        ]),
                        return_: data::host::RegistrationType::Parameter(0),
                        returns_value: true,
                    },
                ]),
                return_: data::host::RegistrationType::Function {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::host::RegistrationType::Parameter(0)),
                },
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::host::RegistrationType::Parameter(0)),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("support"),
            site: data::source::HostCallSite::from_static("support", "wrap", data::source::SourceSpan::new(207, 240)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Int,
                    shape: data::type_::ValueShapeId(0),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Bool,
                    shape: data::type_::ValueShapeId(3),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Function {
                        local: data::graph::ParamLocal::BoolFunction {
                            local: data::graph::BoolFunctionLocalId(0),
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
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
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[
                    data::host::HostCallableConstruction {
                        target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Bool(data::function::BoolFunctionId(5))),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                        },
                        parameters: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                shape: data::type_::ValueShapeId(0),
                            },
                        ]),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::BoolFunction {
                                    local: data::graph::BoolFunctionLocalId(0),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                    },
                                },
                                shape: data::type_::ValueShapeId(7),
                            },
                        ]),
                    },
                ]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                })),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Function {
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Parameter(0),
                        ]),
                        return_: data::Storage::Static(&data::host::RegistrationType::Parameter(1)),
                    },
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[
                    data::host::CallableRegistration {
                        package: data::Text::Static("support"),
                        module: data::Text::Static("support/private"),
                        name: data::Text::Static("wrap"),
                        arguments: data::Storage::Static(&[
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
                        return_: data::host::RegistrationType::Parameter(1),
                        returns_value: true,
                    },
                ]),
                return_: data::host::RegistrationType::Function {
                    arguments: data::Storage::Static(&[
                        data::host::RegistrationType::Parameter(0),
                    ]),
                    return_: data::Storage::Static(&data::host::RegistrationType::Parameter(1)),
                },
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Function {
                        slot: 0,
                        arity: 1,
                    },
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Function {
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Parameter(0),
                        ]),
                        return_: data::Storage::Static(&data::host::RegistrationType::Parameter(1)),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("support"),
            site: data::source::HostCallSite::from_static("support", "wrap", data::source::SourceSpan::new(207, 240)),
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
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[
                    data::host::HostCallableConstruction {
                        target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Int(data::function::IntFunctionId(7))),
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
                ]),
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
                        data::type_::ValueType::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                })),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Function {
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Parameter(0),
                        ]),
                        return_: data::Storage::Static(&data::host::RegistrationType::Parameter(1)),
                    },
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[
                    data::host::CallableRegistration {
                        package: data::Text::Static("support"),
                        module: data::Text::Static("support/private"),
                        name: data::Text::Static("wrap"),
                        arguments: data::Storage::Static(&[
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
                        return_: data::host::RegistrationType::Parameter(1),
                        returns_value: true,
                    },
                ]),
                return_: data::host::RegistrationType::Function {
                    arguments: data::Storage::Static(&[
                        data::host::RegistrationType::Parameter(0),
                    ]),
                    return_: data::Storage::Static(&data::host::RegistrationType::Parameter(1)),
                },
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Function {
                        slot: 0,
                        arity: 1,
                    },
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Function {
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Parameter(0),
                        ]),
                        return_: data::Storage::Static(&data::host::RegistrationType::Parameter(1)),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("support"),
            site: data::source::HostCallSite::from_static("support", "wrap", data::source::SourceSpan::new(207, 240)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                        })),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                    })),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Int,
                    shape: data::type_::ValueShapeId(0),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                    }),
                    shape: data::type_::ValueShapeId(2),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Function {
                        local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                            id: data::graph::CoreFunctionFunctionLocalId(0),
                            type_: data::type_::FunctionFunctionType {
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    })),
                                },
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueShapeId(0),
                                ]),
                                return_: data::type_::FunctionShape {
                                    shape_id: data::type_::ValueShapeId(2),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    },
                                },
                            },
                        })),
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
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[
                    data::host::HostCallableConstruction {
                        target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Function {
                            id: data::function::RuntimeFunctionFunctionTarget::Core(data::function::ProfiledFunctionFunctionId::Int(data::function::IntFunctionFunctionId(4))),
                            return_type: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            },
                        }),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                arguments: data::Storage::Static(&[]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            })),
                        },
                        parameters: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                shape: data::type_::ValueShapeId(0),
                            },
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
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            })),
                                        },
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueShapeId(0),
                                        ]),
                                        return_: data::type_::FunctionShape {
                                            shape_id: data::type_::ValueShapeId(2),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    },
                                })),
                                shape: data::type_::ValueShapeId(8),
                            },
                        ]),
                    },
                ]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        })),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    })),
                })),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Function {
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Parameter(0),
                        ]),
                        return_: data::Storage::Static(&data::host::RegistrationType::Parameter(1)),
                    },
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[
                    data::host::CallableRegistration {
                        package: data::Text::Static("support"),
                        module: data::Text::Static("support/private"),
                        name: data::Text::Static("wrap"),
                        arguments: data::Storage::Static(&[
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
                        return_: data::host::RegistrationType::Parameter(1),
                        returns_value: true,
                    },
                ]),
                return_: data::host::RegistrationType::Function {
                    arguments: data::Storage::Static(&[
                        data::host::RegistrationType::Parameter(0),
                    ]),
                    return_: data::Storage::Static(&data::host::RegistrationType::Parameter(1)),
                },
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Function {
                        slot: 0,
                        arity: 1,
                    },
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Function {
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Parameter(0),
                        ]),
                        return_: data::Storage::Static(&data::host::RegistrationType::Parameter(1)),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("support"),
            site: data::source::HostCallSite::from_static("support", "make_stop", data::source::SourceSpan::new(293, 311)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0))),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                    shape: data::type_::ValueShapeId(9),
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
                        target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Never(data::function::NeverFunctionId(1))),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                        },
                        parameters: data::Storage::Static(&[]),
                        captures: data::Storage::Static(&[]),
                    },
                ]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                })),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[
                    data::host::CallableRegistration {
                        package: data::Text::Static("support"),
                        module: data::Text::Static("support/private"),
                        name: data::Text::Static("stop"),
                        arguments: data::Storage::Static(&[]),
                        captures: data::Storage::Static(&[]),
                        return_: data::host::RegistrationType::Parameter(0),
                        returns_value: true,
                    },
                ]),
                return_: data::host::RegistrationType::Function {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::host::RegistrationType::Parameter(0)),
                },
                layout: data::Storage::Static(&[]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::host::RegistrationType::Parameter(0)),
                    },
                ]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::Int,
                index: 5,
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
                family: data::function::FunctionTableFamily::Int,
                index: 6,
            }),
            package: data::Text::Static("support"),
            site: data::source::HostCallSite::from_static("support/private", "constant", data::source::SourceSpan::new(0, 0)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Int,
                    shape: data::type_::ValueShapeId(0),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[]),
                captures: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::Int),
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
                family: data::function::FunctionTableFamily::Bool,
                index: 4,
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
                family: data::function::FunctionTableFamily::IntList,
                index: 0,
            }),
            package: data::Text::Static("support"),
            site: data::source::HostCallSite::from_static("support/private", "constant", data::source::SourceSpan::new(0, 0)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int))),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                    shape: data::type_::ValueShapeId(5),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[]),
                captures: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                            local: data::graph::IntListLocalId(0),
                            type_id: data::type_::IntListTypeId {
                                list_type: data::type_::ListTypeId(0),
                            },
                        }),
                        shape: data::type_::ValueShapeId(5),
                    },
                ]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)), data::type_::ListTypeId(0)),
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
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(0))),
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
                family: data::function::FunctionTableFamily::Bool,
                index: 5,
            }),
            package: data::Text::Static("support"),
            site: data::source::HostCallSite::from_static("support/private", "wrap", data::source::SourceSpan::new(0, 0)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Int,
                    shape: data::type_::ValueShapeId(0),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Bool,
                    shape: data::type_::ValueShapeId(3),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                ]),
                captures: data::Storage::Static(&[
                    data::graph::ParamSlot {
                        local: data::graph::ParamLocal::BoolFunction {
                            local: data::graph::BoolFunctionLocalId(0),
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                            },
                        },
                        shape: data::type_::ValueShapeId(7),
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
                return_: data::Storage::Static(&data::type_::ValueType::Bool),
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
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::Int,
                index: 7,
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
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::IntFunction,
                index: 4,
            }),
            package: data::Text::Static("support"),
            site: data::source::HostCallSite::from_static("support/private", "wrap", data::source::SourceSpan::new(0, 0)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Int,
                    shape: data::type_::ValueShapeId(0),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                    }),
                    shape: data::type_::ValueShapeId(2),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
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
                                        arguments: data::Storage::Static(&[]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    })),
                                },
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueShapeId(0),
                                ]),
                                return_: data::type_::FunctionShape {
                                    shape_id: data::type_::ValueShapeId(2),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    },
                                },
                            },
                        })),
                        shape: data::type_::ValueShapeId(8),
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
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                })),
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
    ]),
    never_functions: data::Storage::Static(&[
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Uninhabited,
            callable_entry: None,
            package: data::Text::Static("support"),
            site: data::source::HostCallSite::from_static("support", "produce", data::source::SourceSpan::new(361, 377)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0))),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                    shape: data::type_::ValueShapeId(9),
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
                return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[]),
                captures: data::Storage::Static(&[]),
                callable: false,
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
            completion: data::host::HostFunctionCompletion::Uninhabited,
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::Never,
                index: 1,
            }),
            package: data::Text::Static("support"),
            site: data::source::HostCallSite::from_static("support/private", "stop", data::source::SourceSpan::new(0, 0)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0))),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                    shape: data::type_::ValueShapeId(9),
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
                return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
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
    callables: data::Storage::Static(&[]),
}
