data::HostedEntryArtifact {
    format: 3,
    program: data::ProgramTables {
        root: data::source::module_id(1),
        modules: data::Storage::Static(&[
            data::program::ExecutionModuleContext {
                module: data::Text::Static("fixture/work"),
                source_context: Some(data::source::SourceContext::from_static("src/fixture/work.gleam", "pub type Work(value)\n\n@external(erlang, \"fixture\", \"ready\")\npub fn ready(value: value) -> Work(value)\n\n@external(erlang, \"fixture\", \"map\")\npub fn map(value: Work(a), callback: fn(a) -> b) -> Work(b)\n\n@external(erlang, \"fixture\", \"flatten\")\npub fn flatten(value: Work(Work(a))) -> Work(a)\n\n@external(erlang, \"fixture\", \"all\")\npub fn all(values: List(Work(a))) -> Work(List(a))\n\npub fn then(value: Work(a), callback: fn(a) -> Work(b)) -> Work(b) {\n  flatten(map(value, callback))\n}\n")),
            },
            data::program::ExecutionModuleContext {
                module: data::Text::Static("entry"),
                source_context: Some(data::source::SourceContext::from_static("src/entry.gleam", "pub fn main() {\n  echo second([0, 42])\n  fn(value) { value }\n}\n\nfn second(items) {\n  case items {\n    [] -> 0\n    [item] -> item\n    [_, item, ..] -> item\n  }\n}\n")),
            },
            data::program::ExecutionModuleContext {
                module: data::Text::Static("entry_failure"),
                source_context: Some(data::source::SourceContext::from_static("src/entry_failure.gleam", "pub fn main() {\n  echo \"before failure\"\n  panic as \"prepared main failed\"\n}\n")),
            },
            data::program::ExecutionModuleContext {
                module: data::Text::Static("entry_work"),
                source_context: Some(data::source::SourceContext::from_static("src/entry_work.gleam", "import fixture/work\n\npub fn main() {\n  echo \"main\"\n  work.map(work.ready(41), fn(value) {\n    echo value + 1\n    fn(value: Int) { value + 1 }\n  })\n}\n")),
            },
        ]),
        main: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Function {
            id: data::function::RuntimeFunctionFunctionTarget::Core(data::function::ProfiledFunctionFunctionId::Generic(data::function::GenericFunctionFunctionId {
                index: 0,
                type_: data::type_::GenericFunctionType {
                    type_: data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                    },
                    shape: data::type_::FunctionShape {
                        shape_id: data::type_::ValueShapeId(1),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                        },
                    },
                },
            })),
            return_type: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
            },
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
                                        terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                            subject: data::graph::BoolLocalId(0),
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Bool,
                                                            positions: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::IntList,
                                                            positions: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(0),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
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
                                                            family: data::graph::StorageFamily::IntList,
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
                                        params: 1..1,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..2,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                            subject: data::graph::BoolLocalId(0),
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(0),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
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
                                                            family: data::graph::StorageFamily::IntList,
                                                            positions: data::Storage::Static(&[
                                                                0,
                                                            ]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(4),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(0),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
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
                                                            family: data::graph::StorageFamily::IntList,
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
                                        instructions: 3..4,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 3..4,
                                        instructions: 4..5,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(3),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(3),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(3),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(3),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::ListLengthEquals {
                                            value: data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            },
                                            length: 0,
                                        }),
                                    },
                                    data::graph::ProfiledInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
                                        })),
                                    },
                                    data::graph::ProfiledInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::ListLengthEquals {
                                            value: data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            },
                                            length: 1,
                                        }),
                                    },
                                    data::graph::ProfiledInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::ListIndex {
                                            list: data::graph::IntListLocalId(0),
                                            index: 0,
                                        }),
                                    },
                                    data::graph::ProfiledInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::ListIndex {
                                            list: data::graph::IntListLocalId(0),
                                            index: 1,
                                        }),
                                    },
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
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
                bool_functions: data::Storage::Static(&[]),
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
                generic_function_functions: data::Storage::Static(&[
                    data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 0,
                        },
                        body: data::function::TypedFunctionBody {
                            _shape: data::type_::FunctionShape {
                                shape_id: data::type_::ValueShapeId(1),
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                },
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..0,
                                            instructions: 0..4,
                                            terminator: data::graph::Terminator::Echo(data::graph::Echo {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                message: None,
                                                site: data::source::EchoSite::from_static("entry", "main", data::source::SourceSpan::new(18, 38)),
                                                next: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntList,
                                                                positions: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 0..0,
                                            instructions: 4..5,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::NoSign,
                                                digits: data::Storage::Static(&[]),
                                            })),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(2),
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
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Int(data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                                data::graph::IntLocalId(0),
                                                data::graph::IntLocalId(1),
                                            ])))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(0),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("entry", "main", data::source::SourceSpan::new(23, 38)),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::GenericFunction(data::graph::GenericFunctionLocal {
                                                    id: data::graph::GenericFunctionLocalId(0),
                                                    type_: data::type_::GenericFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                        },
                                                        shape: data::type_::FunctionShape {
                                                            shape_id: data::type_::ValueShapeId(1),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                            },
                                                        },
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                },
                                                family: data::function::FunctionReturnFamily::Generic,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Generic(data::function::GenericCallableId::Function {
                                                        template: 2,
                                                        substitution: data::Storage::Static(&[
                                                            data::type_::ValueShapeId(0),
                                                        ]),
                                                    }),
                                                    captures: data::Storage::Static(&[]),
                                                },
                                            }),
                                        },
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::GenericFunctionLocal {
                                        id: data::graph::GenericFunctionLocalId(0),
                                        type_: data::type_::GenericFunctionType {
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                            },
                                            shape: data::type_::FunctionShape {
                                                shape_id: data::type_::ValueShapeId(1),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                },
                                            },
                                        },
                                    }),
                                ]),
                            },
                        },
                    })),
                ]),
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
                0..1,
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
                1..2,
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
                        data::type_::ValueShapeId(3),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 1..1,
                    parameter_shapes: data::Storage::Static(&[]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
            ]),
            parameters: data::Storage::Static(&[
                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                    local: data::graph::IntListLocalId(0),
                    type_id: data::type_::IntListTypeId {
                        list_type: data::type_::ListTypeId(0),
                    },
                }),
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
                data::type_::ValueShapeDescriptor::Parameter(data::type_::parameter_id(0)),
                data::type_::ValueShapeDescriptor::Function {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                },
                data::type_::ValueShapeDescriptor::Int,
                data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(2)),
                data::type_::ValueShapeDescriptor::Bool,
            ]),
            shape_types: data::Storage::Static(&[
                data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                }),
                data::type_::ValueType::Int,
                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                data::type_::ValueType::Bool,
            ]),
            custom_shapes: data::Storage::Static(&[]),
        },
    },
    value_functions: data::Storage::Static(&[]),
    never_functions: data::Storage::Static(&[]),
}
