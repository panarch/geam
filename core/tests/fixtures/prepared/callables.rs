data::HostedModuleArtifact {
    module: data::ModuleArtifact {
        format: 4,
        program: data::ProgramTables {
            root: data::source::module_id(1),
            modules: data::Storage::Static(&[
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("support"),
                    source_context: Some(data::source::SourceContext::from_static("support.gleam", "\n@external(erlang, \"ffi\", \"make_constant\")\npub fn make_constant(value: a) -> fn() -> a\n@external(erlang, \"ffi\", \"make_adder\")\npub fn make_adder(value: Int) -> fn(Int) -> Int\n@external(erlang, \"ffi\", \"wrap\")\npub fn wrap(callback: fn(a) -> b) -> fn(a) -> b\n")),
                },
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("library"),
                    source_context: Some(data::source::SourceContext::from_static("library.gleam", "\nimport support\nfn apply(callback, value) { callback(value) }\npub fn make_native(offset: Int) -> fn(Int) -> Int { support.make_adder(offset) }\npub fn keep(adjust: fn(Int) -> Int) -> fn(Int) -> Int { adjust }\npub fn calculate(value: Int, adjust: fn(Int) -> Int) -> Int { adjust(value) }\npub fn function_list(items: List(fn(Int) -> Int)) -> List(fn(Int) -> Int) { items }\npub fn container(adjust: fn(Int) -> Int) -> #(fn(Int) -> Int, Result(fn(Int) -> Int, Nil)) { #(adjust, Ok(adjust)) }\npub fn maker() -> fn(Int) -> fn(Int) -> Int { fn(offset) { support.make_adder(offset) } }\npub fn result_function() -> fn(Int) -> Result(Int, Nil) { fn(value) { Ok(value) } }\npub fn picker() -> fn(List(Result(Int, Nil))) -> Int {\n    fn(items) { case items { [Ok(value)] -> value _ -> 0 } }\n}\npub fn run() {\n    let add = support.make_adder(40)\n    let constant = support.make_constant(2)\n    apply(add, constant())\n}\npub fn check() {\n    let a = support.make_constant(True)\n    let alias = a\n    let b = support.make_constant(True)\n    let list = support.make_constant([42])\n    let is_answer = support.wrap(fn(value) { value == 42 })\n    let increment = support.wrap(support.wrap(fn(value) { value + 1 }))\n    let nested = support.wrap(fn(value) { support.make_constant(value) })\n    a() && a == alias && a != b && list() == [42] && is_answer(increment(41)) && nested(42)() == 42\n}\n")),
                },
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("support/private"),
                    source_context: None,
                },
            ]),
            main: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Int(data::function::IntFunctionId(0))),
            functions: data::function::FunctionTables {
                value_returns: data::function::ValueFunctionTables {
                    never_functions: data::Storage::Static(&[]),
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
                                        data::graph::ProfiledInstruction {
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
                                                kind: data::graph::FunctionInstructionKind::Call {
                                                    function: data::function::ProfiledFunctionFunctionId::Int(data::function::IntFunctionFunctionId(0)),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("library", "run", data::source::SourceSpan::new(808, 830)),
                                                },
                                            }),
                                        },
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
                                                    site: data::source::HostCallSite::from_static("library", "run", data::source::SourceSpan::new(850, 874)),
                                                },
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::FunctionCall {
                                                function: data::graph::IntFunctionLocalId(1),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("library", "run", data::source::SourceSpan::new(890, 900)),
                                            }),
                                        },
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::IntFunctionId(1),
                                            site: data::source::HostCallSite::from_static("library", "run", data::source::SourceSpan::new(879, 901)),
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
                                                    positions: data::Storage::Static(&[
                                                        2,
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
                                                site: data::source::HostCallSite::from_static("library", "apply", data::source::SourceSpan::new(44, 59)),
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
                                                    1,
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
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 7,
                            return_: data::graph::IntLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 8,
                            return_: data::graph::IntLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 12,
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
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntList,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                positions: data::Storage::Static(&[
                                                                    2,
                                                                ]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BoolFunction,
                                                                positions: data::Storage::Static(&[
                                                                    0,
                                                                    1,
                                                                    3,
                                                                ]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntListFunction,
                                                                positions: data::Storage::Static(&[
                                                                    0,
                                                                ]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CoreFunctionFunction,
                                                                positions: data::Storage::Static(&[
                                                                    1,
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
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntList,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BoolFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntListFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CoreFunctionFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 0..6,
                                            instructions: 15..16,
                                            terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                                subject: data::graph::BoolLocalId(0),
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
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                positions: data::Storage::Static(&[
                                                                    0,
                                                                ]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BoolFunction,
                                                                positions: data::Storage::Static(&[
                                                                    0,
                                                                    1,
                                                                    2,
                                                                ]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntListFunction,
                                                                positions: data::Storage::Static(&[
                                                                    0,
                                                                ]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CoreFunctionFunction,
                                                                positions: data::Storage::Static(&[
                                                                    0,
                                                                ]),
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
                                                                family: data::graph::StorageFamily::Bool,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BoolFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntListFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CoreFunctionFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 6..12,
                                            instructions: 16..17,
                                            terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                                subject: data::graph::BoolLocalId(0),
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
                                                                family: data::graph::StorageFamily::Bool,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                positions: data::Storage::Static(&[
                                                                    0,
                                                                ]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BoolFunction,
                                                                positions: data::Storage::Static(&[
                                                                    2,
                                                                ]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntListFunction,
                                                                positions: data::Storage::Static(&[
                                                                    0,
                                                                ]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CoreFunctionFunction,
                                                                positions: data::Storage::Static(&[
                                                                    0,
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
                                                                family: data::graph::StorageFamily::Bool,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BoolFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntListFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CoreFunctionFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 12..16,
                                            instructions: 17..21,
                                            terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                                subject: data::graph::BoolLocalId(0),
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
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntList,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                positions: data::Storage::Static(&[
                                                                    0,
                                                                ]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BoolFunction,
                                                                positions: data::Storage::Static(&[
                                                                    0,
                                                                ]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntListFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CoreFunctionFunction,
                                                                positions: data::Storage::Static(&[
                                                                    0,
                                                                ]),
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
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntList,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BoolFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntListFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CoreFunctionFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 16..19,
                                            instructions: 21..24,
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
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BoolFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CoreFunctionFunction,
                                                                positions: data::Storage::Static(&[
                                                                    0,
                                                                ]),
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
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BoolFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CoreFunctionFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 19..20,
                                            instructions: 24..29,
                                            terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                                subject: data::graph::BoolLocalId(0),
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(6),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CoreFunctionFunction,
                                                                positions: data::Storage::Static(&[]),
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
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::CoreFunctionFunction,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 20..20,
                                            instructions: 29..30,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(7),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
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
                                            params: 20..21,
                                            instructions: 30..30,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 21..21,
                                            instructions: 30..30,
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
                                            instructions: 30..31,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(7),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
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
                                            params: 21..21,
                                            instructions: 31..31,
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
                                            instructions: 31..31,
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
                                            instructions: 31..31,
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
                                            instructions: 31..31,
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
                                            instructions: 31..31,
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
                                            instructions: 31..31,
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
                                            instructions: 31..31,
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
                                            instructions: 31..31,
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
                                            instructions: 31..31,
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
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                        },
                                        data::graph::ProfiledInstruction {
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
                                                    site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(933, 960)),
                                                },
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                        },
                                        data::graph::ProfiledInstruction {
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
                                                    site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(991, 1018)),
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
                                                    42,
                                                ]),
                                            })),
                                        },
                                        data::graph::ProfiledInstruction {
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
                                        },
                                        data::graph::ProfiledInstruction {
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
                                                    site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1034, 1061)),
                                                },
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
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
                                                    target: data::graph::FunctionTarget::Bool(data::function::BoolFunctionId(1)),
                                                    captures: data::Storage::Static(&[]),
                                                },
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
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
                                                    site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1082, 1121)),
                                                },
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
                                                    site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1155, 1192)),
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
                                                    site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1142, 1193)),
                                                },
                                            }),
                                        },
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
                                        },
                                        data::graph::ProfiledInstruction {
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
                                                    site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1211, 1267)),
                                                },
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::FunctionCall {
                                                function: data::graph::BoolFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1272, 1275)),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Equal {
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
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::NotEqual {
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
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
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
                                                site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1303, 1309)),
                                            })),
                                        },
                                        data::graph::ProfiledInstruction {
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
                                        },
                                        data::graph::ProfiledInstruction {
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
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Equal {
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
                                                    41,
                                                ]),
                                            })),
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
                                                site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1331, 1344)),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::FunctionCall {
                                                function: data::graph::BoolFunctionLocalId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1321, 1345)),
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
                                                    42,
                                                ]),
                                            })),
                                        },
                                        data::graph::ProfiledInstruction {
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
                                                    site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1349, 1359)),
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
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("library", "check", data::source::SourceSpan::new(1349, 1361)),
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
                                                    42,
                                                ]),
                                            })),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Equal {
                                                left: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                right: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(false)),
                                        },
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
                                                    42,
                                                ]),
                                            })),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Equal {
                                                left: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                right: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            }),
                                        },
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 9,
                            return_: data::graph::BoolLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 11,
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
                            index: 10,
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
                                                site: data::source::HostCallSite::from_static("library", "<anonymous:5>", data::source::SourceSpan::new(1236, 1264)),
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
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 13,
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
                    never_function_functions: data::Storage::Static(&[]),
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
                    0..0,
                    0..0,
                    6..10,
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
                    11..16,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    16..18,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    18..19,
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
                    19..20,
                ],
                functions: data::Storage::Static(&[
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
                        parameters: 7..7,
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
                        parameters: 7..8,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 8..9,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(2),
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
                        parameters: 12..13,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(3),
                        ]),
                        return_: data::type_::ValueShapeId(4),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 13..14,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(7),
                        ]),
                        return_: data::type_::ValueShapeId(7),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 14..15,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(5),
                        ]),
                        return_: data::type_::ValueShapeId(6),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 15..16,
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
                definitions: data::Storage::Static(&[]),
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
        ]),
    },
    value_functions: data::Storage::Static(&[
        data::host::HostedFunctionMetadata {
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
                        target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Int(data::function::IntFunctionId(4))),
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
                        target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Bool(data::function::BoolFunctionId(2))),
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
                        target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Bool(data::function::BoolFunctionId(3))),
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
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::Int,
                index: 3,
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
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::Int,
                index: 4,
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
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::Bool,
                index: 2,
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
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::Bool,
                index: 3,
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
            callable_entry: Some(data::host::HostCallableEntry {
                family: data::function::FunctionTableFamily::Int,
                index: 5,
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
    never_functions: data::Storage::Static(&[]),
    callables: data::Storage::Static(&[]),
}
