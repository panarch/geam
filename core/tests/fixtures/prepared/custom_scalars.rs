data::ModuleArtifact {
    format: 30,
    program: data::ProgramTables {
        root: data::source::module_id(0),
        modules: data::Storage::Static(&[
            data::program::ExecutionModuleContext {
                module: data::Text::Static("example"),
                source_context: None,
            },
        ]),
        main: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Int(data::function::IntFunctionId(0))),
        functions: data::function::FunctionTables {
            value_returns: data::function::ValueFunctionTables {
                never_functions: data::Storage::Static(&[]),
                int_functions: data::Storage::Static(&[
                    data::function::ExecutableFunction {
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
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            ]),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(12),
                                        site: data::source::HostCallSite::from_static("example", "credit", data::source::SourceSpan::new(261, 290)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
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
                                        ]),
                                    },
                                },
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
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
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                                    shape_id: data::type_::CustomValueShapeId(1),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(0),
                                                index: 1,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            ]),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(12),
                                        site: data::source::HostCallSite::from_static("example", "debit", data::source::SourceSpan::new(336, 364)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(1),
                                            },
                                        }),
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
                                        ]),
                                    },
                                },
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
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
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(2),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(0),
                                                index: 2,
                                            },
                                            fields: data::Storage::Static(&[]),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(12),
                                        site: data::source::HostCallSite::from_static("example", "ignored", data::source::SourceSpan::new(399, 421)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(2),
                                            },
                                        }),
                                    ]),
                                    transfer: data::graph::Transfer {
                                        families: data::Storage::Static(&[]),
                                    },
                                },
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 3,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..3,
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
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(4),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(3),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(1),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            ]),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(13),
                                        site: data::source::HostCallSite::from_static("example", "guarded", data::source::SourceSpan::new(716, 750)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(3),
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
                                                family: data::graph::StorageFamily::Bool,
                                                length: 0,
                                                steps: data::Storage::Static(&[]),
                                            },
                                        ]),
                                    },
                                },
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 2,
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
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(3),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(1),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            ]),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(14),
                                        site: data::source::HostCallSite::from_static("example", "aliased", data::source::SourceSpan::new(903, 934)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(3),
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
                                                family: data::graph::StorageFamily::Bool,
                                                length: 0,
                                                steps: data::Storage::Static(&[]),
                                            },
                                        ]),
                                    },
                                },
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 3,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..3,
                                        instructions: 0..5,
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
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(4),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
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
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(3),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(1),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Plus,
                                            digits: data::Storage::Static(&[
                                                11,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(3),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(1),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            ]),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(15),
                                        site: data::source::HostCallSite::from_static("example", "multiple", data::source::SourceSpan::new(1143, 1187)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(3),
                                            },
                                        }),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(3),
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
                                                family: data::graph::StorageFamily::Bool,
                                                length: 0,
                                                steps: data::Storage::Static(&[]),
                                            },
                                        ]),
                                    },
                                },
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 3,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..3,
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
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(4),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(3),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(1),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            ]),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(16),
                                        site: data::source::HostCallSite::from_static("example", "fields", data::source::SourceSpan::new(1617, 1652)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(3),
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
                                                family: data::graph::StorageFamily::Bool,
                                                length: 0,
                                                steps: data::Storage::Static(&[]),
                                            },
                                        ]),
                                    },
                                },
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 2,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..2,
                                        instructions: 0..4,
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
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(3),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(1),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
                                        })),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(17),
                                        site: data::source::HostCallSite::from_static("example", "repeated", data::source::SourceSpan::new(1855, 1893)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(3),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    ]),
                                    transfer: data::graph::Transfer {
                                        families: data::Storage::Static(&[
                                            data::graph::FamilyTransfer {
                                                family: data::graph::StorageFamily::Int,
                                                length: 2,
                                                steps: data::Storage::Static(&[
                                                    data::graph::TransferStep {
                                                        source: 1,
                                                        destination: 0,
                                                    },
                                                    data::graph::TransferStep {
                                                        source: 3,
                                                        destination: 1,
                                                    },
                                                ]),
                                            },
                                            data::graph::FamilyTransfer {
                                                family: data::graph::StorageFamily::Bool,
                                                length: 0,
                                                steps: data::Storage::Static(&[]),
                                            },
                                        ]),
                                    },
                                },
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 3,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..3,
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
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(4),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(3),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(1),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            ]),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(18),
                                        site: data::source::HostCallSite::from_static("example", "assertion", data::source::SourceSpan::new(2034, 2070)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(3),
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
                                                family: data::graph::StorageFamily::Bool,
                                                length: 0,
                                                steps: data::Storage::Static(&[]),
                                            },
                                        ]),
                                    },
                                },
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 2,
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
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(3),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(1),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            ]),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(19),
                                        site: data::source::HostCallSite::from_static("example", "panic_case", data::source::SourceSpan::new(2244, 2273)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(3),
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
                                                family: data::graph::StorageFamily::Bool,
                                                length: 0,
                                                steps: data::Storage::Static(&[]),
                                            },
                                        ]),
                                    },
                                },
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 1,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..1,
                                        instructions: 0..4,
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
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(3),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(1),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(5),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(7),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(2),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(3),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(20),
                                        site: data::source::HostCallSite::from_static("example", "nested", data::source::SourceSpan::new(2437, 2478)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(5),
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
                                                        source: 1,
                                                        destination: 0,
                                                    },
                                                ]),
                                            },
                                            data::graph::FamilyTransfer {
                                                family: data::graph::StorageFamily::Bool,
                                                length: 0,
                                                steps: data::Storage::Static(&[]),
                                            },
                                        ]),
                                    },
                                },
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 0,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..0,
                                        instructions: 0..41,
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
                                                11,
                                            ]),
                                        })),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2500, 2513)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            shape: data::type_::ValueShapeId(0),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                            shape: data::type_::ValueShapeId(0),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2518, 2530)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(5)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                            shape: data::type_::ValueShapeId(0),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(2),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2535, 2545)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(6)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(8)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Plus,
                                            digits: data::Storage::Static(&[
                                                3,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Plus,
                                            digits: data::Storage::Static(&[
                                                4,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(3),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2550, 2569)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(9)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(12)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                            shape: data::type_::ValueShapeId(0),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(4),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2574, 2587)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(13)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(16)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Plus,
                                            digits: data::Storage::Static(&[
                                                4,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                            shape: data::type_::ValueShapeId(0),
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
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(20)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(5),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2592, 2612)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(21)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(17)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(20)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(22)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Plus,
                                            digits: data::Storage::Static(&[
                                                12,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(23)),
                                            shape: data::type_::ValueShapeId(0),
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
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(24)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(6),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(22)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(23)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2617, 2636)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(25)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(21)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(24)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(26)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Plus,
                                            digits: data::Storage::Static(&[
                                                3,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(27)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Plus,
                                            digits: data::Storage::Static(&[
                                                4,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(28)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(7),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(26)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(27)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2641, 2655)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(29)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(25)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(28)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(30)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Plus,
                                            digits: data::Storage::Static(&[
                                                3,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(31)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Plus,
                                            digits: data::Storage::Static(&[
                                                9,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(3)),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(32)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(8),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(30)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(31)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(3)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2660, 2681)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(33)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(29)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(32)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(34)),
                                            shape: data::type_::ValueShapeId(0),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(35)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(10),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(34)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2686, 2695)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(36)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(33)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(35)),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(36)),
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 2,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..2,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(6),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Bind(data::graph::MatchPatternBinding {
                                                        index: 0,
                                                    }),
                                                ]),
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                    data::graph::MatchEdgeArgument::Binding(0),
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    0,
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(6),
                                                        },
                                                    }),
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
                                    data::graph::BlockHeader {
                                        params: 4..6,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(6),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Bind(data::graph::MatchPatternBinding {
                                                        index: 0,
                                                    }),
                                                ]),
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                    data::graph::MatchEdgeArgument::Binding(0),
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    0,
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(4),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 6..8,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 8..9,
                                        instructions: 2..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
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
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(8),
                                    },
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
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(8),
                                    },
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
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(2)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(2)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
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
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            3,
                                                        ]),
                                                    }),
                                                    data::graph::MatchPattern::Bind(data::graph::MatchPatternBinding {
                                                        index: 0,
                                                    }),
                                                    data::graph::MatchPattern::Bool(true),
                                                ]),
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Binding(0),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    })),
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    0,
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(7),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    }),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..3,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::GtInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(0),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
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
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 3..4,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..5,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(4),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    }),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 5..6,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
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
                                                    data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            9,
                                                        ]),
                                                    }),
                                                    data::graph::MatchPattern::Bool(true),
                                                ]),
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(5),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Binding(0),
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    0,
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(6),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    }),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 6..7,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 7..8,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 8..9,
                                        instructions: 3..3,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(4),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    }),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Mult {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Immediate(2),
                                        }),
                                    }),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            },
                                            index: 1,
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
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
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            },
                                            index: 0,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            },
                                            index: 1,
                                        }),
                                    }),
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
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            },
                                            index: 1,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(3)),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(4)),
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 2,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..2,
                                        instructions: 0..2,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(2),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
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
                                                    data::graph::MatchPattern::Discard,
                                                    data::graph::MatchPattern::Bool(true),
                                                ]),
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Binding(0),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    })),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(1),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    })),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Tuple {
                                                        local: data::graph::TupleLocalId(0),
                                                        type_: data::Storage::Static(&[
                                                            data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                                                            data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                                                        ]),
                                                    }),
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    0,
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 2,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(5),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(1),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    }),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 2,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..6,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(2),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Discard,
                                                    data::graph::MatchPattern::Bind(data::graph::MatchPatternBinding {
                                                        index: 0,
                                                    }),
                                                    data::graph::MatchPattern::Bool(true),
                                                ]),
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                    data::graph::MatchEdgeArgument::Binding(0),
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    0,
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(1),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
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
                                                            length: 2,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 6..8,
                                        instructions: 3..4,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 8..10,
                                        instructions: 4..4,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(4),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(1),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    }),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 10..12,
                                        instructions: 4..7,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 12..14,
                                        instructions: 7..7,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(4),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(1),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    }),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Tuple {
                                            local: data::graph::TupleLocalId(0),
                                            type_: data::Storage::Static(&[
                                                data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                                                data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                                            ]),
                                        },
                                        shape: data::type_::ValueShapeId(9),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(9),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            }),
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            }),
                                        ]))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(2),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::TupleIndex {
                                            tuple: data::graph::TupleLocalId(0),
                                            index: 0,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(2),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::TupleIndex {
                                            tuple: data::graph::TupleLocalId(0),
                                            index: 1,
                                        }),
                                    }),
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
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            },
                                            index: 1,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            },
                                            index: 0,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(2)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(2)),
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
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
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    }),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Bool,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    }),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Bool,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..2,
                                        instructions: 1..5,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..3,
                                        instructions: 5..8,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            },
                                            index: 2,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            },
                                            index: 0,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Div {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Immediate(3),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            },
                                            index: 1,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::IntegerRegion(data::graph::ArithmeticRegion {
                                        inputs: data::Storage::Static(&[
                                            data::graph::IntLocalId(2),
                                            data::graph::IntLocalId(1),
                                        ]),
                                        nodes: data::Storage::Static(&[
                                            data::graph::ArithmeticNode::Remainder(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Immediate(4)),
                                            data::graph::ArithmeticNode::Add(data::graph::ArithmeticOperand::Input(1), data::graph::ArithmeticOperand::Value(0)),
                                        ]),
                                        outputs: data::Storage::Static(&[
                                            data::graph::ArithmeticOutput {
                                                value: 1,
                                                slot: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                    shape: data::type_::ValueShapeId(0),
                                                },
                                            },
                                        ]),
                                        native: true,
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            },
                                            index: 0,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            },
                                            index: 1,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(3)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(2)),
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 3,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..3,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::GtInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(0),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
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
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 3..6,
                                        instructions: 0..3,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Int,
                                                            length: 2,
                                                            steps: data::Storage::Static(&[
                                                                data::graph::TransferStep {
                                                                    source: 2,
                                                                    destination: 0,
                                                                },
                                                                data::graph::TransferStep {
                                                                    source: 4,
                                                                    destination: 1,
                                                                },
                                                            ]),
                                                        },
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 6..7,
                                        instructions: 3..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
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
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Immediate(1),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            },
                                            index: 0,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(3)),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
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
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            3,
                                                        ]),
                                                    }),
                                                    data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            9,
                                                        ]),
                                                    }),
                                                    data::graph::MatchPattern::Bool(true),
                                                ]),
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[]),
                                                bindings: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    }),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..1,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..2,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            }),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "asserted", data::source::SourceSpan::new(1932, 1942)),
                                            pattern_span: data::source::SourceSpan::new(1943, 1959),
                                        }),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
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
                                                12,
                                            ]),
                                        })),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
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
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            3,
                                                        ]),
                                                    }),
                                                    data::graph::MatchPattern::Discard,
                                                    data::graph::MatchPattern::Discard,
                                                ]),
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[]),
                                                bindings: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    }),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..1,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::SourceStop(data::graph::SourceStop {
                                            kind: data::graph::SourceStopKind::Panic,
                                            message: Some(data::graph::StringLocalId(0)),
                                            site: data::source::PanicSite::from_static("example", "stop", data::source::SourceSpan::new(2138, 2160)),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..2,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(10),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("custom stop"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            },
                                            index: 1,
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
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
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(7),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(11),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(7),
                                                },
                                            },
                                            index: 0,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            },
                                            index: 0,
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                            ]),
                        },
                    },
                ]),
                float_functions: data::Storage::Static(&[]),
                string_functions: data::Storage::Static(&[]),
                bit_array_functions: data::Storage::Static(&[]),
                utf_codepoint_functions: data::Storage::Static(&[]),
                custom_functions: data::Storage::Static(&[]),
                external_functions: data::Storage::Static(&[]),
                bool_functions: data::Storage::Static(&[
                    data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 3,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..3,
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
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(4),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(3),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(1),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            ]),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::BoolFunctionId(1),
                                        site: data::source::HostCallSite::from_static("example", "boolean", data::source::SourceSpan::new(1379, 1417)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(3),
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
                                                family: data::graph::StorageFamily::Bool,
                                                length: 0,
                                                steps: data::Storage::Static(&[]),
                                            },
                                        ]),
                                    },
                                },
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
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
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            3,
                                                        ]),
                                                    }),
                                                    data::graph::MatchPattern::Discard,
                                                    data::graph::MatchPattern::Bool(true),
                                                ]),
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    })),
                                                ]),
                                                bindings: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    }),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..2,
                                        instructions: 0..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..3,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(4),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            },
                                            index: 1,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::GtInt {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Immediate(0),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            },
                                            index: 2,
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
                            ]),
                        },
                    },
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
            const CALL_GROUP_0: [data::compiled::calls::CallStart; 8] = {
                use data::compiled::calls::{CallArguments, CallCustom, CallExecution, CallInputs, CallInteger, CallNativeFailure, CallNativeInput, CallNativeOps, CallNativeReturn, CallNullary, CallOps, CallOutput, CallProgress, CallStorage, CallValues};
                enum FunctionState {
                    Int2Point0 { int0: i128 },
                    Int2Point1 { int0: i128, nullary0: CallNullary },
                    Int11Point0 {  },
                    Int11Point1 { int0: i128 },
                    Int11Point2 { int0: i128, int1: i128 },
                    Int11Point3 { int0: i128, int1: i128, int2: i128 },
                    Int11Point4 { int0: i128, int1: i128, int2: i128, int3: i128 },
                    Int11Point5 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128 },
                    Int11Point6 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128 },
                    Int11Point7 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128 },
                    Int11Point8 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128 },
                    Int11Point9 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128 },
                    Int11Point10 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128 },
                    Int11Point11 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128 },
                    Int11Point12 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128 },
                    Int11Point13 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool },
                    Int11Point14 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128 },
                    Int11Point15 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128 },
                    Int11Point16 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128 },
                    Int11Point17 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128 },
                    Int11Point18 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128 },
                    Int11Point19 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128 },
                    Int11Point20 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128 },
                    Int11Point21 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128 },
                    Int11Point22 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool },
                    Int11Point23 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128 },
                    Int11Point24 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128 },
                    Int11Point25 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128 },
                    Int11Point26 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128 },
                    Int11Point27 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool },
                    Int11Point28 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128 },
                    Int11Point29 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128 },
                    Int11Point30 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128 },
                    Int11Point31 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128 },
                    Int11Point32 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128, int28: i128 },
                    Int11Point33 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128, int28: i128, int29: i128 },
                    Int11Point34 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128, int28: i128, int29: i128, int30: i128 },
                    Int11Point35 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128, int28: i128, int29: i128, int30: i128, int31: i128 },
                    Int11Point36 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128, int28: i128, int29: i128, int30: i128, int31: i128, bool3: bool },
                    Int11Point37 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128, int28: i128, int29: i128, int30: i128, int31: i128, bool3: bool, int32: i128 },
                    Int11Point38 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128, int28: i128, int29: i128, int30: i128, int31: i128, bool3: bool, int32: i128, int33: i128 },
                    Int11Point39 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128, int28: i128, int29: i128, int30: i128, int31: i128, bool3: bool, int32: i128, int33: i128, int34: i128 },
                    Int11Point40 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128, int28: i128, int29: i128, int30: i128, int31: i128, bool3: bool, int32: i128, int33: i128, int34: i128, int35: i128 },
                    Int11Point41 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128, int28: i128, int29: i128, int30: i128, int31: i128, bool3: bool, int32: i128, int33: i128, int34: i128, int35: i128, int36: i128 },
                    Int12Point0 { int0: i128, custom0: CallCustom },
                    Int12Point1 { int0: i128, int1: i128 },
                    Int12Point2 { int0: i128, int1: i128, int2: i128 },
                    Int12Point3 { int0: i128, custom0: CallCustom },
                    Int12Point4 { int0: i128, int1: i128 },
                    Int12Point5 { int0: i128, int1: i128, int2: i128 },
                    Int12Point6 { int0: i128 },
                    Int13Point0 { custom0: CallCustom },
                    Int13Point1 { int0: i128, custom0: CallCustom },
                    Int13Point2 { int0: i128 },
                    Int13Point3 { int0: i128, int1: i128 },
                    Int13Point4 { custom0: CallCustom },
                    Int13Point5 { custom0: CallCustom },
                    Int13Point6 { int0: i128 },
                    Int13Point7 { int0: i128, int1: i128 },
                    Int13Point8 { custom0: CallCustom },
                    Int13Point9 { custom0: CallCustom, int0: i128 },
                    Int13Point10 { custom0: CallCustom },
                    Int14Point0 { custom0: CallCustom },
                    Int14Point1 { custom0: CallCustom, int0: i128 },
                    Int14Point2 { custom0: CallCustom, int0: i128, int1: i128 },
                    Int14Point3 { custom0: CallCustom, int0: i128, int1: i128, int2: i128 },
                    Int14Point4 { custom0: CallCustom, int0: i128, int1: i128, int2: i128, int3: i128 },
                    Int14Point5 { custom0: CallCustom, int0: i128, int1: i128, int2: i128, int3: i128, int4: i128 },
                    Int16Point0 { custom0: CallCustom },
                    Int16Point1 { custom0: CallCustom, bool0: bool },
                    Int16Point2 { custom0: CallCustom },
                    Int16Point3 { custom0: CallCustom, int0: i128 },
                    Int16Point4 { custom0: CallCustom, int0: i128, int1: i128 },
                    Int16Point5 { custom0: CallCustom, int0: i128, int1: i128, int2: i128 },
                    Int16Point6 { custom0: CallCustom, int0: i128, int1: i128, int2: i128, int3: i128 },
                    Int16Point7 { custom0: CallCustom },
                    Int16Point8 { custom0: CallCustom, int0: i128 },
                    Int16Point9 { custom0: CallCustom, int0: i128, int1: i128 },
                    Int16Point10 { custom0: CallCustom, int0: i128, int1: i128, int2: i128 },
                    Int17Point0 { custom0: CallCustom, int0: i128, int1: i128 },
                    Int17Point1 { custom0: CallCustom, int0: i128, int1: i128 },
                    Int17Point2 { custom0: CallCustom, int0: i128, int1: i128, int2: i128 },
                    Int17Point3 { custom0: CallCustom, int0: i128, int1: i128, int2: i128, int3: i128 },
                    Int17Point4 { custom0: CallCustom, int0: i128, int1: i128, int2: i128, int3: i128, int4: i128 },
                    Int17Point5 { int0: i128 },
                    Int20Point0 { custom0: CallCustom },
                    Int20Point1 { custom0: CallCustom, custom1: CallCustom },
                    Int20Point2 { custom0: CallCustom, custom1: CallCustom, int0: i128 },
                    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                }
                enum IntReturn {
                    Int11Call2 { int0: i128, int1: i128 },
                    Int11Call5 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128 },
                    Int11Call8 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128 },
                    Int11Call13 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool },
                    Int11Call17 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128 },
                    Int11Call22 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool },
                    Int11Call27 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool },
                    Int11Call31 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128 },
                    Int11Call36 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128, int28: i128, int29: i128, int30: i128, int31: i128, bool3: bool },
                    Int11Call39 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128, int28: i128, int29: i128, int30: i128, int31: i128, bool3: bool, int32: i128, int33: i128, int34: i128 },
                }
                impl IntReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                            Self::Int11Call2 { .. } => data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2500, 2513)),
                            Self::Int11Call5 { .. } => data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2518, 2530)),
                            Self::Int11Call8 { .. } => data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2535, 2545)),
                            Self::Int11Call13 { .. } => data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2550, 2569)),
                            Self::Int11Call17 { .. } => data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2574, 2587)),
                            Self::Int11Call22 { .. } => data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2592, 2612)),
                            Self::Int11Call27 { .. } => data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2617, 2636)),
                            Self::Int11Call31 { .. } => data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2641, 2655)),
                            Self::Int11Call36 { .. } => data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2660, 2681)),
                            Self::Int11Call39 { .. } => data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2686, 2695)),
                        }
                    }
                    fn small(self, result: i128) -> FunctionState {
                        match self {
                            Self::Int11Call2 { int0, int1 } => {
                                let int2 = result;
                                FunctionState::Int11Point3 { int0, int1, int2 }
                            },
                            Self::Int11Call5 { int0, int1, int2, int3, int4 } => {
                                let int5 = result;
                                FunctionState::Int11Point6 { int0, int1, int2, int3, int4, int5 }
                            },
                            Self::Int11Call8 { int0, int1, int2, int3, int4, int5, int6, int7 } => {
                                let int8 = result;
                                FunctionState::Int11Point9 { int0, int1, int2, int3, int4, int5, int6, int7, int8 }
                            },
                            Self::Int11Call13 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0 } => {
                                let int12 = result;
                                FunctionState::Int11Point14 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12 }
                            },
                            Self::Int11Call17 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15 } => {
                                let int16 = result;
                                FunctionState::Int11Point18 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16 }
                            },
                            Self::Int11Call22 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1 } => {
                                let int20 = result;
                                FunctionState::Int11Point23 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20 }
                            },
                            Self::Int11Call27 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2 } => {
                                let int24 = result;
                                FunctionState::Int11Point28 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24 }
                            },
                            Self::Int11Call31 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27 } => {
                                let int28 = result;
                                FunctionState::Int11Point32 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28 }
                            },
                            Self::Int11Call36 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3 } => {
                                let int32 = result;
                                FunctionState::Int11Point37 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32 }
                            },
                            Self::Int11Call39 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33, int34 } => {
                                let int35 = result;
                                FunctionState::Int11Point40 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33, int34, int35 }
                            },
                        }
                    }
                    fn resume(self, result: CallInteger) -> FunctionState {
                        if let Some(result) = result.small() {
                            return self.small(result);
                        }
                        match self {
                            Self::Int11Call2 { int0, int1 } => {
                                let int2 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 3,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2], ..CallValues::default() }) }
                            },
                            Self::Int11Call5 { int0, int1, int2, int3, int4 } => {
                                let int5 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 6,
                                    ints: 6,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5], ..CallValues::default() }) }
                            },
                            Self::Int11Call8 { int0, int1, int2, int3, int4, int5, int6, int7 } => {
                                let int8 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 9,
                                    ints: 9,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8], ..CallValues::default() }) }
                            },
                            Self::Int11Call13 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0 } => {
                                let int12 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 14,
                                    ints: 13,
                                    bools: 1,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12], bools: vec![bool0], ..CallValues::default() }) }
                            },
                            Self::Int11Call17 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15 } => {
                                let int16 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 18,
                                    ints: 17,
                                    bools: 1,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16], bools: vec![bool0], ..CallValues::default() }) }
                            },
                            Self::Int11Call22 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1 } => {
                                let int20 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 23,
                                    ints: 21,
                                    bools: 2,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20], bools: vec![bool0, bool1], ..CallValues::default() }) }
                            },
                            Self::Int11Call27 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2 } => {
                                let int24 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 28,
                                    ints: 25,
                                    bools: 3,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into(), int22.into(), int23.into(), int24], bools: vec![bool0, bool1, bool2], ..CallValues::default() }) }
                            },
                            Self::Int11Call31 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27 } => {
                                let int28 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 32,
                                    ints: 29,
                                    bools: 3,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into(), int22.into(), int23.into(), int24.into(), int25.into(), int26.into(), int27.into(), int28], bools: vec![bool0, bool1, bool2], ..CallValues::default() }) }
                            },
                            Self::Int11Call36 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3 } => {
                                let int32 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 37,
                                    ints: 33,
                                    bools: 4,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into(), int22.into(), int23.into(), int24.into(), int25.into(), int26.into(), int27.into(), int28.into(), int29.into(), int30.into(), int31.into(), int32], bools: vec![bool0, bool1, bool2, bool3], ..CallValues::default() }) }
                            },
                            Self::Int11Call39 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33, int34 } => {
                                let int35 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 40,
                                    ints: 36,
                                    bools: 4,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into(), int22.into(), int23.into(), int24.into(), int25.into(), int26.into(), int27.into(), int28.into(), int29.into(), int30.into(), int31.into(), int32.into(), int33.into(), int34.into(), int35], bools: vec![bool0, bool1, bool2, bool3], ..CallValues::default() }) }
                            },
                        }
                    }
                }
                #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                enum FunctionStep {
                    Yield(FunctionState),
                    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                    IntCall { callee: FunctionState, caller: IntReturn },
                    IntTail { callee: FunctionState },
                    Int { value: i128 },
                    IntScalarBridge { function: data::function::IntFunctionId, site: data::source::HostCallSite, input: CallNativeInput, caller: IntReturn },
                    IntBridge { function: data::function::IntFunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: IntReturn },
                }
                struct FunctionExecution {
                    active: Option<FunctionActive>,
                    pending_entry: bool,
                    integer_returns: Vec<IntReturn>,
                }
                #[allow(clippy::large_enum_variant, reason = "The suspended caller stays in its existing execution allocation.")]
                enum FunctionActive {
                    Running(FunctionState),
                    IntCall { function: data::function::IntFunctionId, site: data::source::HostCallSite, input: CallNativeInput, caller: IntReturn },
                    IntReturn { caller: IntReturn, returned: CallNativeReturn<CallInteger> },
                }
                impl FunctionExecution {
                    fn new(active: FunctionState) -> Self {
                        Self {
                            active: Some(FunctionActive::Running(active)),
                            pending_entry: false,
                            integer_returns: Vec::new(),
                        }
                    }
                }
                impl CallExecution for FunctionExecution {
                    fn restart(&mut self, target: data::compiled::CallTarget, point: usize, values: CallInputs<'_>) -> bool {
                        if self.active.is_some() { return false; }
                        let active = match target {
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(2)) => calls_int_2_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(11)) => calls_int_11_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(12)) => calls_int_12_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(13)) => calls_int_13_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(14)) => calls_int_14_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(16)) => calls_int_16_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(17)) => calls_int_17_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(20)) => calls_int_20_state(point, values),
                            _ => None,
                        };
                        let Some(active) = active else { return false; };
                        self.active = Some(FunctionActive::Running(active));
                        true
                    }
                    fn retained_bytes(&self) -> usize {
                        std::mem::size_of::<Self>() + self.integer_returns.capacity() * std::mem::size_of::<IntReturn>()
                    }
                    fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                        let Some(active) = self.active.take() else { return CallProgress::Yield(self); };
                        let mut active = match active {
                            FunctionActive::Running(active) => active,
                            FunctionActive::IntCall { function, site, input, caller } => {
                                return CallProgress::IntScalar {
                                    function, site, input,
                                    resume: Box::new(move |value| {
                                        self.active = Some(FunctionActive::Running(caller.resume(value)));
                                        self
                                    }),
                                };
                            },
                            FunctionActive::IntReturn { caller, returned } => {
                                if *budget == 0 {
                                    self.active = Some(FunctionActive::IntReturn { caller, returned });
                                    return CallProgress::Yield(self);
                                }
                                *budget -= 1;
                                caller.resume(returned.into_value())
                            },
                        };
                        loop {
                            if self.pending_entry {
                                if *budget == 0 { self.active = Some(FunctionActive::Running(active)); return CallProgress::Yield(self); }
                                *budget -= 1;
                                self.pending_entry = false;
                            }
                            match function_step(active, ops, budget) {
                                FunctionStep::Yield(active) => {
                                    self.active = Some(FunctionActive::Running(active));
                                    return CallProgress::Yield(self);
                                },
                                FunctionStep::IntCall { callee, caller } => {
                                    self.integer_returns.push(caller);
                                    active = callee;
                                },
                                FunctionStep::IntTail { callee } => {
                                    *budget -= 1;
                                    self.pending_entry = self.integer_returns.is_empty() && ops.root_tail_entry();
                                    active = callee;
                                },
                                FunctionStep::Int { value } => {
                                    if let Some(caller) = self.integer_returns.pop() {
                                        active = caller.small(value);
                                    } else {
                                        self.integer_returns.clear();
                                        return CallProgress::Complete { output: CallOutput::Int(value.into()), execution: self };
                                    }
                                },
                                FunctionStep::IntBridge { function, site, arguments, caller } => return CallProgress::Int {
                                    function, site, arguments,
                                    resume: Box::new(move |value| {
                                        self.active = Some(FunctionActive::Running(caller.resume(value)));
                                        self
                                    }),
                                },
                                FunctionStep::IntScalarBridge { function, site, input, caller } => {
                                    return CallProgress::IntScalar {
                                        function, site, input,
                                        resume: Box::new(move |value| {
                                            self.active = Some(FunctionActive::Running(caller.resume(value)));
                                            self
                                        }),
                                    };
                                },
                                FunctionStep::Canonical { target, point, values } => {
                                    match target {
                                        data::compiled::CallTarget::Int(function) => {
                                            if let Some(caller) = self.integer_returns.pop() {
                                                let site = caller.site();
                                                return CallProgress::InterpretedInt {
                                                    function, site, point, values,
                                                    resume: Box::new(move |value| {
                                                        self.active = Some(FunctionActive::Running(caller.resume(value)));
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
                    fn advance_native(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize, native: &mut CallNativeOps<'_>) -> Result<Option<CallProgress>, CallNativeFailure> {
                        let Some(active) = self.active.take() else { return Ok(Some(CallProgress::Yield(self))); };
                        let mut active = match active {
                            FunctionActive::Running(active) => active,
                            FunctionActive::IntCall { function, site, input, caller } => {
                                if let CallNativeOps::Int { function: target, native } = native && *target == function {
                                    if *budget == 0 {
                                        self.active = Some(FunctionActive::IntCall { function, site, input, caller });
                                        return Ok(Some(CallProgress::Yield(self)));
                                    }
                                    *budget -= 1;
                                    let Some(returned) = native.call(input, site)? else { return Ok(None); };
                                    if *budget == 0 {
                                        self.active = Some(FunctionActive::IntReturn { caller, returned });
                                        return Ok(Some(CallProgress::Yield(self)));
                                    }
                                    *budget -= 1;
                                    caller.resume(returned.into_value())
                                } else {
                                    return Ok(Some(CallProgress::IntScalar {
                                        function, site, input,
                                        resume: Box::new(move |value| {
                                            self.active = Some(FunctionActive::Running(caller.resume(value)));
                                            self
                                        }),
                                    }));
                                }
                            },
                            FunctionActive::IntReturn { caller, returned } => {
                                if *budget == 0 {
                                    self.active = Some(FunctionActive::IntReturn { caller, returned });
                                    return Ok(Some(CallProgress::Yield(self)));
                                }
                                *budget -= 1;
                                caller.resume(returned.into_value())
                            },
                        };
                        loop {
                            if self.pending_entry {
                                if *budget == 0 { self.active = Some(FunctionActive::Running(active)); return Ok(Some(CallProgress::Yield(self))); }
                                *budget -= 1;
                                self.pending_entry = false;
                            }
                            match function_step(active, ops, budget) {
                                FunctionStep::Yield(active) => {
                                    self.active = Some(FunctionActive::Running(active));
                                    return Ok(Some(CallProgress::Yield(self)));
                                },
                                FunctionStep::IntCall { callee, caller } => {
                                    self.integer_returns.push(caller);
                                    active = callee;
                                },
                                FunctionStep::IntTail { callee } => {
                                    *budget -= 1;
                                    self.pending_entry = self.integer_returns.is_empty() && ops.root_tail_entry();
                                    active = callee;
                                },
                                FunctionStep::Int { value } => {
                                    if let Some(caller) = self.integer_returns.pop() {
                                        active = caller.small(value);
                                    } else {
                                        self.integer_returns.clear();
                                        return Ok(Some(CallProgress::Complete { output: CallOutput::Int(value.into()), execution: self }));
                                    }
                                },
                                FunctionStep::IntBridge { function, site, arguments, caller } => return Ok(Some(CallProgress::Int {
                                    function, site, arguments,
                                    resume: Box::new(move |value| {
                                        self.active = Some(FunctionActive::Running(caller.resume(value)));
                                        self
                                    }),
                                })),
                                FunctionStep::IntScalarBridge { function, site, input, caller } => {
                                    active = {
                                        if let CallNativeOps::Int { function: target, native } = native && *target == function {
                                            if *budget == 0 {
                                                self.active = Some(FunctionActive::IntCall { function, site, input, caller });
                                                return Ok(Some(CallProgress::Yield(self)));
                                            }
                                            *budget -= 1;
                                            let Some(returned) = native.call(input, site)? else { return Ok(None); };
                                            if *budget == 0 {
                                                self.active = Some(FunctionActive::IntReturn { caller, returned });
                                                return Ok(Some(CallProgress::Yield(self)));
                                            }
                                            *budget -= 1;
                                            caller.resume(returned.into_value())
                                        } else {
                                            return Ok(Some(CallProgress::IntScalar {
                                                function, site, input,
                                                resume: Box::new(move |value| {
                                                    self.active = Some(FunctionActive::Running(caller.resume(value)));
                                                    self
                                                }),
                                            }));
                                        }
                                    };
                                },
                                FunctionStep::Canonical { target, point, values } => {
                                    match target {
                                        data::compiled::CallTarget::Int(function) => {
                                            if let Some(caller) = self.integer_returns.pop() {
                                                let site = caller.site();
                                                return Ok(Some(CallProgress::InterpretedInt {
                                                    function, site, point, values,
                                                    resume: Box::new(move |value| {
                                                        self.active = Some(FunctionActive::Running(caller.resume(value)));
                                                        self
                                                    }),
                                                }));
                                            }
                                            return Ok(Some(CallProgress::Interpreted { target, point, values }));
                                        },
                                        _ => return Ok(Some(CallProgress::Interpreted { target, point, values })),
                                    }
                                },
                            }
                        }
                    }
                }
                fn function_step(active: FunctionState, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        FunctionState::Canonical { target, point, values } => FunctionStep::Canonical { target, point, values },
                        FunctionState::Int2Point0 { int0 } => calls_int_2_run(Int2State::Point0 { int0 }, ops, budget),
                        FunctionState::Int2Point1 { int0, nullary0 } => calls_int_2_run(Int2State::Point1 { int0, nullary0 }, ops, budget),
                        FunctionState::Int11Point0 {  } => calls_int_11_run(Int11State::Point0 {  }, ops, budget),
                        FunctionState::Int11Point1 { int0 } => calls_int_11_run(Int11State::Point1 { int0 }, ops, budget),
                        FunctionState::Int11Point2 { int0, int1 } => calls_int_11_run(Int11State::Point2 { int0, int1 }, ops, budget),
                        FunctionState::Int11Point3 { int0, int1, int2 } => calls_int_11_run(Int11State::Point3 { int0, int1, int2 }, ops, budget),
                        FunctionState::Int11Point4 { int0, int1, int2, int3 } => calls_int_11_run(Int11State::Point4 { int0, int1, int2, int3 }, ops, budget),
                        FunctionState::Int11Point5 { int0, int1, int2, int3, int4 } => calls_int_11_run(Int11State::Point5 { int0, int1, int2, int3, int4 }, ops, budget),
                        FunctionState::Int11Point6 { int0, int1, int2, int3, int4, int5 } => calls_int_11_run(Int11State::Point6 { int0, int1, int2, int3, int4, int5 }, ops, budget),
                        FunctionState::Int11Point7 { int0, int1, int2, int3, int4, int5, int6 } => calls_int_11_run(Int11State::Point7 { int0, int1, int2, int3, int4, int5, int6 }, ops, budget),
                        FunctionState::Int11Point8 { int0, int1, int2, int3, int4, int5, int6, int7 } => calls_int_11_run(Int11State::Point8 { int0, int1, int2, int3, int4, int5, int6, int7 }, ops, budget),
                        FunctionState::Int11Point9 { int0, int1, int2, int3, int4, int5, int6, int7, int8 } => calls_int_11_run(Int11State::Point9 { int0, int1, int2, int3, int4, int5, int6, int7, int8 }, ops, budget),
                        FunctionState::Int11Point10 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9 } => calls_int_11_run(Int11State::Point10 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9 }, ops, budget),
                        FunctionState::Int11Point11 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10 } => calls_int_11_run(Int11State::Point11 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10 }, ops, budget),
                        FunctionState::Int11Point12 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11 } => calls_int_11_run(Int11State::Point12 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11 }, ops, budget),
                        FunctionState::Int11Point13 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0 } => calls_int_11_run(Int11State::Point13 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0 }, ops, budget),
                        FunctionState::Int11Point14 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12 } => calls_int_11_run(Int11State::Point14 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12 }, ops, budget),
                        FunctionState::Int11Point15 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13 } => calls_int_11_run(Int11State::Point15 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13 }, ops, budget),
                        FunctionState::Int11Point16 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14 } => calls_int_11_run(Int11State::Point16 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14 }, ops, budget),
                        FunctionState::Int11Point17 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15 } => calls_int_11_run(Int11State::Point17 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15 }, ops, budget),
                        FunctionState::Int11Point18 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16 } => calls_int_11_run(Int11State::Point18 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16 }, ops, budget),
                        FunctionState::Int11Point19 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17 } => calls_int_11_run(Int11State::Point19 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17 }, ops, budget),
                        FunctionState::Int11Point20 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18 } => calls_int_11_run(Int11State::Point20 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18 }, ops, budget),
                        FunctionState::Int11Point21 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19 } => calls_int_11_run(Int11State::Point21 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19 }, ops, budget),
                        FunctionState::Int11Point22 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1 } => calls_int_11_run(Int11State::Point22 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1 }, ops, budget),
                        FunctionState::Int11Point23 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20 } => calls_int_11_run(Int11State::Point23 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20 }, ops, budget),
                        FunctionState::Int11Point24 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21 } => calls_int_11_run(Int11State::Point24 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21 }, ops, budget),
                        FunctionState::Int11Point25 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22 } => calls_int_11_run(Int11State::Point25 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22 }, ops, budget),
                        FunctionState::Int11Point26 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23 } => calls_int_11_run(Int11State::Point26 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23 }, ops, budget),
                        FunctionState::Int11Point27 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2 } => calls_int_11_run(Int11State::Point27 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2 }, ops, budget),
                        FunctionState::Int11Point28 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24 } => calls_int_11_run(Int11State::Point28 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24 }, ops, budget),
                        FunctionState::Int11Point29 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25 } => calls_int_11_run(Int11State::Point29 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25 }, ops, budget),
                        FunctionState::Int11Point30 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26 } => calls_int_11_run(Int11State::Point30 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26 }, ops, budget),
                        FunctionState::Int11Point31 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27 } => calls_int_11_run(Int11State::Point31 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27 }, ops, budget),
                        FunctionState::Int11Point32 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28 } => calls_int_11_run(Int11State::Point32 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28 }, ops, budget),
                        FunctionState::Int11Point33 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29 } => calls_int_11_run(Int11State::Point33 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29 }, ops, budget),
                        FunctionState::Int11Point34 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30 } => calls_int_11_run(Int11State::Point34 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30 }, ops, budget),
                        FunctionState::Int11Point35 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31 } => calls_int_11_run(Int11State::Point35 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31 }, ops, budget),
                        FunctionState::Int11Point36 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3 } => calls_int_11_run(Int11State::Point36 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3 }, ops, budget),
                        FunctionState::Int11Point37 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32 } => calls_int_11_run(Int11State::Point37 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32 }, ops, budget),
                        FunctionState::Int11Point38 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33 } => calls_int_11_run(Int11State::Point38 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33 }, ops, budget),
                        FunctionState::Int11Point39 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33, int34 } => calls_int_11_run(Int11State::Point39 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33, int34 }, ops, budget),
                        FunctionState::Int11Point40 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33, int34, int35 } => calls_int_11_run(Int11State::Point40 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33, int34, int35 }, ops, budget),
                        FunctionState::Int11Point41 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33, int34, int35, int36 } => calls_int_11_run(Int11State::Point41 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33, int34, int35, int36 }, ops, budget),
                        FunctionState::Int12Point0 { int0, custom0 } => calls_int_12_run(Int12State::Point0 { int0, custom0 }, ops, budget),
                        FunctionState::Int12Point1 { int0, int1 } => calls_int_12_run(Int12State::Point1 { int0, int1 }, ops, budget),
                        FunctionState::Int12Point2 { int0, int1, int2 } => calls_int_12_run(Int12State::Point2 { int0, int1, int2 }, ops, budget),
                        FunctionState::Int12Point3 { int0, custom0 } => calls_int_12_run(Int12State::Point3 { int0, custom0 }, ops, budget),
                        FunctionState::Int12Point4 { int0, int1 } => calls_int_12_run(Int12State::Point4 { int0, int1 }, ops, budget),
                        FunctionState::Int12Point5 { int0, int1, int2 } => calls_int_12_run(Int12State::Point5 { int0, int1, int2 }, ops, budget),
                        FunctionState::Int12Point6 { int0 } => calls_int_12_run(Int12State::Point6 { int0 }, ops, budget),
                        FunctionState::Int13Point0 { custom0 } => calls_int_13_run(Int13State::Point0 { custom0 }, ops, budget),
                        FunctionState::Int13Point1 { int0, custom0 } => calls_int_13_run(Int13State::Point1 { int0, custom0 }, ops, budget),
                        FunctionState::Int13Point2 { int0 } => calls_int_13_run(Int13State::Point2 { int0 }, ops, budget),
                        FunctionState::Int13Point3 { int0, int1 } => calls_int_13_run(Int13State::Point3 { int0, int1 }, ops, budget),
                        FunctionState::Int13Point4 { custom0 } => calls_int_13_run(Int13State::Point4 { custom0 }, ops, budget),
                        FunctionState::Int13Point5 { custom0 } => calls_int_13_run(Int13State::Point5 { custom0 }, ops, budget),
                        FunctionState::Int13Point6 { int0 } => calls_int_13_run(Int13State::Point6 { int0 }, ops, budget),
                        FunctionState::Int13Point7 { int0, int1 } => calls_int_13_run(Int13State::Point7 { int0, int1 }, ops, budget),
                        FunctionState::Int13Point8 { custom0 } => calls_int_13_run(Int13State::Point8 { custom0 }, ops, budget),
                        FunctionState::Int13Point9 { custom0, int0 } => calls_int_13_run(Int13State::Point9 { custom0, int0 }, ops, budget),
                        FunctionState::Int13Point10 { custom0 } => calls_int_13_run(Int13State::Point10 { custom0 }, ops, budget),
                        FunctionState::Int14Point0 { custom0 } => calls_int_14_run(Int14State::Point0 { custom0 }, ops, budget),
                        FunctionState::Int14Point1 { custom0, int0 } => calls_int_14_run(Int14State::Point1 { custom0, int0 }, ops, budget),
                        FunctionState::Int14Point2 { custom0, int0, int1 } => calls_int_14_run(Int14State::Point2 { custom0, int0, int1 }, ops, budget),
                        FunctionState::Int14Point3 { custom0, int0, int1, int2 } => calls_int_14_run(Int14State::Point3 { custom0, int0, int1, int2 }, ops, budget),
                        FunctionState::Int14Point4 { custom0, int0, int1, int2, int3 } => calls_int_14_run(Int14State::Point4 { custom0, int0, int1, int2, int3 }, ops, budget),
                        FunctionState::Int14Point5 { custom0, int0, int1, int2, int3, int4 } => calls_int_14_run(Int14State::Point5 { custom0, int0, int1, int2, int3, int4 }, ops, budget),
                        FunctionState::Int16Point0 { custom0 } => calls_int_16_run(Int16State::Point0 { custom0 }, ops, budget),
                        FunctionState::Int16Point1 { custom0, bool0 } => calls_int_16_run(Int16State::Point1 { custom0, bool0 }, ops, budget),
                        FunctionState::Int16Point2 { custom0 } => calls_int_16_run(Int16State::Point2 { custom0 }, ops, budget),
                        FunctionState::Int16Point3 { custom0, int0 } => calls_int_16_run(Int16State::Point3 { custom0, int0 }, ops, budget),
                        FunctionState::Int16Point4 { custom0, int0, int1 } => calls_int_16_run(Int16State::Point4 { custom0, int0, int1 }, ops, budget),
                        FunctionState::Int16Point5 { custom0, int0, int1, int2 } => calls_int_16_run(Int16State::Point5 { custom0, int0, int1, int2 }, ops, budget),
                        FunctionState::Int16Point6 { custom0, int0, int1, int2, int3 } => calls_int_16_run(Int16State::Point6 { custom0, int0, int1, int2, int3 }, ops, budget),
                        FunctionState::Int16Point7 { custom0 } => calls_int_16_run(Int16State::Point7 { custom0 }, ops, budget),
                        FunctionState::Int16Point8 { custom0, int0 } => calls_int_16_run(Int16State::Point8 { custom0, int0 }, ops, budget),
                        FunctionState::Int16Point9 { custom0, int0, int1 } => calls_int_16_run(Int16State::Point9 { custom0, int0, int1 }, ops, budget),
                        FunctionState::Int16Point10 { custom0, int0, int1, int2 } => calls_int_16_run(Int16State::Point10 { custom0, int0, int1, int2 }, ops, budget),
                        FunctionState::Int17Point0 { custom0, int0, int1 } => calls_int_17_run(Int17State::Point0 { custom0, int0, int1 }, ops, budget),
                        FunctionState::Int17Point1 { custom0, int0, int1 } => calls_int_17_run(Int17State::Point1 { custom0, int0, int1 }, ops, budget),
                        FunctionState::Int17Point2 { custom0, int0, int1, int2 } => calls_int_17_run(Int17State::Point2 { custom0, int0, int1, int2 }, ops, budget),
                        FunctionState::Int17Point3 { custom0, int0, int1, int2, int3 } => calls_int_17_run(Int17State::Point3 { custom0, int0, int1, int2, int3 }, ops, budget),
                        FunctionState::Int17Point4 { custom0, int0, int1, int2, int3, int4 } => calls_int_17_run(Int17State::Point4 { custom0, int0, int1, int2, int3, int4 }, ops, budget),
                        FunctionState::Int17Point5 { int0 } => calls_int_17_run(Int17State::Point5 { int0 }, ops, budget),
                        FunctionState::Int20Point0 { custom0 } => calls_int_20_run(Int20State::Point0 { custom0 }, ops, budget),
                        FunctionState::Int20Point1 { custom0, custom1 } => calls_int_20_run(Int20State::Point1 { custom0, custom1 }, ops, budget),
                        FunctionState::Int20Point2 { custom0, custom1, int0 } => calls_int_20_run(Int20State::Point2 { custom0, custom1, int0 }, ops, budget),
                    }
                }
                enum Int2State {
                    Point0 { int0: i128 },
                    Point1 { int0: i128, nullary0: CallNullary },
                }
                fn calls_int_2_run(active: Int2State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int2State::Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point0 { int0 }); }
                            *budget -= 1;
                            let nullary0 = CallNullary::new(data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(0),
                                index: 2,
                            });
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point1 { int0, nullary0 }); }
                            {
                                FunctionStep::IntTail { callee: FunctionState::Int12Point0 { int0, custom0: nullary0.into() } }
                            }
                        },
                        Int2State::Point1 { int0, nullary0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point1 { int0, nullary0 }); }
                            {
                                FunctionStep::IntTail { callee: FunctionState::Int12Point0 { int0, custom0: nullary0.into() } }
                            }
                        },
                    }
                }
                enum Int11State {
                    Point0 {  },
                    Point1 { int0: i128 },
                    Point2 { int0: i128, int1: i128 },
                    Point3 { int0: i128, int1: i128, int2: i128 },
                    Point4 { int0: i128, int1: i128, int2: i128, int3: i128 },
                    Point5 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128 },
                    Point6 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128 },
                    Point7 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128 },
                    Point8 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128 },
                    Point9 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128 },
                    Point10 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128 },
                    Point11 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128 },
                    Point12 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128 },
                    Point13 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool },
                    Point14 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128 },
                    Point15 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128 },
                    Point16 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128 },
                    Point17 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128 },
                    Point18 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128 },
                    Point19 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128 },
                    Point20 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128 },
                    Point21 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128 },
                    Point22 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool },
                    Point23 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128 },
                    Point24 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128 },
                    Point25 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128 },
                    Point26 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128 },
                    Point27 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool },
                    Point28 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128 },
                    Point29 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128 },
                    Point30 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128 },
                    Point31 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128 },
                    Point32 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128, int28: i128 },
                    Point33 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128, int28: i128, int29: i128 },
                    Point34 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128, int28: i128, int29: i128, int30: i128 },
                    Point35 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128, int28: i128, int29: i128, int30: i128, int31: i128 },
                    Point36 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128, int28: i128, int29: i128, int30: i128, int31: i128, bool3: bool },
                    Point37 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128, int28: i128, int29: i128, int30: i128, int31: i128, bool3: bool, int32: i128 },
                    Point38 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128, int28: i128, int29: i128, int30: i128, int31: i128, bool3: bool, int32: i128, int33: i128 },
                    Point39 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128, int28: i128, int29: i128, int30: i128, int31: i128, bool3: bool, int32: i128, int33: i128, int34: i128 },
                    Point40 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128, int28: i128, int29: i128, int30: i128, int31: i128, bool3: bool, int32: i128, int33: i128, int34: i128, int35: i128 },
                    Point41 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, bool0: bool, int12: i128, int13: i128, int14: i128, int15: i128, int16: i128, int17: i128, int18: i128, int19: i128, bool1: bool, int20: i128, int21: i128, int22: i128, int23: i128, bool2: bool, int24: i128, int25: i128, int26: i128, int27: i128, int28: i128, int29: i128, int30: i128, int31: i128, bool3: bool, int32: i128, int33: i128, int34: i128, int35: i128, int36: i128 },
                }
                fn calls_int_11_run(mut active: Int11State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            Int11State::Point0 {  } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point0 {  }); }
                                *budget -= 1;
                                let int0 = 11_i128;
                                if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
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
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point1 { int0 }); }
                                *budget -= 1;
                                let int1 = 2_i128;
                                if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 2,
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
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point2 { int0, int1 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntBridge { function: data::function::IntFunctionId(0), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2500, 2513)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], ..CallValues::default() }), captures: None }, caller: IntReturn::Int11Call2 { int0, int1 } }
                                };
                            },
                            Int11State::Point1 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point1 { int0 }); }
                                *budget -= 1;
                                let int1 = 2_i128;
                                if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 2,
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
                                active = Int11State::Point2 { int0, int1 };
                                continue;
                            },
                            Int11State::Point2 { int0, int1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point2 { int0, int1 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntBridge { function: data::function::IntFunctionId(0), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2500, 2513)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], ..CallValues::default() }), captures: None }, caller: IntReturn::Int11Call2 { int0, int1 } }
                                };
                            },
                            Int11State::Point3 { int0, int1, int2 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point3 { int0, int1, int2 }); }
                                *budget -= 1;
                                let int3 = 7_i128;
                                if int3 < i128::from(i64::MIN) || int3 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 4,
                                    ints: 4,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into()], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point4 { int0, int1, int2, int3 }); }
                                *budget -= 1;
                                let int4 = 20_i128;
                                if int4 < i128::from(i64::MIN) || int4 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 5,
                                    ints: 5,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into()], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point5 { int0, int1, int2, int3, int4 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntBridge { function: data::function::IntFunctionId(1), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2518, 2530)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int3.into(), int4.into()], ..CallValues::default() }), captures: None }, caller: IntReturn::Int11Call5 { int0, int1, int2, int3, int4 } }
                                };
                            },
                            Int11State::Point4 { int0, int1, int2, int3 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point4 { int0, int1, int2, int3 }); }
                                *budget -= 1;
                                let int4 = 20_i128;
                                if int4 < i128::from(i64::MIN) || int4 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 5,
                                    ints: 5,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into()], ..CallValues::default() }) }; }
                                active = Int11State::Point5 { int0, int1, int2, int3, int4 };
                                continue;
                            },
                            Int11State::Point5 { int0, int1, int2, int3, int4 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point5 { int0, int1, int2, int3, int4 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntBridge { function: data::function::IntFunctionId(1), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2518, 2530)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int3.into(), int4.into()], ..CallValues::default() }), captures: None }, caller: IntReturn::Int11Call5 { int0, int1, int2, int3, int4 } }
                                };
                            },
                            Int11State::Point6 { int0, int1, int2, int3, int4, int5 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point6 { int0, int1, int2, int3, int4, int5 }); }
                                *budget -= 1;
                                let int6 = int2 + int5;
                                if int6 < i128::from(i64::MIN) || int6 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 7,
                                    ints: 7,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into()], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point7 { int0, int1, int2, int3, int4, int5, int6 }); }
                                *budget -= 1;
                                let int7 = 5_i128;
                                if int7 < i128::from(i64::MIN) || int7 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 8,
                                    ints: 8,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into()], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point8 { int0, int1, int2, int3, int4, int5, int6, int7 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntCall { callee: FunctionState::Int2Point0 { int0: int7 }, caller: IntReturn::Int11Call8 { int0, int1, int2, int3, int4, int5, int6, int7 } }
                                };
                            },
                            Int11State::Point7 { int0, int1, int2, int3, int4, int5, int6 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point7 { int0, int1, int2, int3, int4, int5, int6 }); }
                                *budget -= 1;
                                let int7 = 5_i128;
                                if int7 < i128::from(i64::MIN) || int7 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 8,
                                    ints: 8,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into()], ..CallValues::default() }) }; }
                                active = Int11State::Point8 { int0, int1, int2, int3, int4, int5, int6, int7 };
                                continue;
                            },
                            Int11State::Point8 { int0, int1, int2, int3, int4, int5, int6, int7 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point8 { int0, int1, int2, int3, int4, int5, int6, int7 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntCall { callee: FunctionState::Int2Point0 { int0: int7 }, caller: IntReturn::Int11Call8 { int0, int1, int2, int3, int4, int5, int6, int7 } }
                                };
                            },
                            Int11State::Point9 { int0, int1, int2, int3, int4, int5, int6, int7, int8 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point9 { int0, int1, int2, int3, int4, int5, int6, int7, int8 }); }
                                *budget -= 1;
                                let int9 = int6 + int8;
                                if int9 < i128::from(i64::MIN) || int9 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 10,
                                    ints: 10,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into()], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point10 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9 }); }
                                *budget -= 1;
                                let int10 = 3_i128;
                                if int10 < i128::from(i64::MIN) || int10 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 11,
                                    ints: 11,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into()], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point11 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10 }); }
                                *budget -= 1;
                                let int11 = 4_i128;
                                if int11 < i128::from(i64::MIN) || int11 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 12,
                                    ints: 12,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into()], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point12 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11 }); }
                                *budget -= 1;
                                let bool0 = true;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point13 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntBridge { function: data::function::IntFunctionId(3), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2550, 2569)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int10.into(), int11.into()], bools: vec![bool0], ..CallValues::default() }), captures: None }, caller: IntReturn::Int11Call13 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0 } }
                                };
                            },
                            Int11State::Point10 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point10 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9 }); }
                                *budget -= 1;
                                let int10 = 3_i128;
                                if int10 < i128::from(i64::MIN) || int10 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 11,
                                    ints: 11,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into()], ..CallValues::default() }) }; }
                                active = Int11State::Point11 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10 };
                                continue;
                            },
                            Int11State::Point11 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point11 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10 }); }
                                *budget -= 1;
                                let int11 = 4_i128;
                                if int11 < i128::from(i64::MIN) || int11 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 12,
                                    ints: 12,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into()], ..CallValues::default() }) }; }
                                active = Int11State::Point12 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11 };
                                continue;
                            },
                            Int11State::Point12 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point12 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11 }); }
                                *budget -= 1;
                                let bool0 = true;
                                active = Int11State::Point13 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0 };
                                continue;
                            },
                            Int11State::Point13 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point13 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntBridge { function: data::function::IntFunctionId(3), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2550, 2569)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int10.into(), int11.into()], bools: vec![bool0], ..CallValues::default() }), captures: None }, caller: IntReturn::Int11Call13 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0 } }
                                };
                            },
                            Int11State::Point14 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point14 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12 }); }
                                *budget -= 1;
                                let int13 = int9 + int12;
                                if int13 < i128::from(i64::MIN) || int13 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 15,
                                    ints: 14,
                                    bools: 1,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into()], bools: vec![bool0], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point15 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13 }); }
                                *budget -= 1;
                                let int14 = 2_i128;
                                if int14 < i128::from(i64::MIN) || int14 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 16,
                                    ints: 15,
                                    bools: 1,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into()], bools: vec![bool0], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point16 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14 }); }
                                *budget -= 1;
                                let int15 = 5_i128;
                                if int15 < i128::from(i64::MIN) || int15 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 17,
                                    ints: 16,
                                    bools: 1,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into()], bools: vec![bool0], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point17 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntBridge { function: data::function::IntFunctionId(4), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2574, 2587)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int14.into(), int15.into()], ..CallValues::default() }), captures: None }, caller: IntReturn::Int11Call17 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15 } }
                                };
                            },
                            Int11State::Point15 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point15 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13 }); }
                                *budget -= 1;
                                let int14 = 2_i128;
                                if int14 < i128::from(i64::MIN) || int14 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 16,
                                    ints: 15,
                                    bools: 1,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into()], bools: vec![bool0], ..CallValues::default() }) }; }
                                active = Int11State::Point16 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14 };
                                continue;
                            },
                            Int11State::Point16 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point16 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14 }); }
                                *budget -= 1;
                                let int15 = 5_i128;
                                if int15 < i128::from(i64::MIN) || int15 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 17,
                                    ints: 16,
                                    bools: 1,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into()], bools: vec![bool0], ..CallValues::default() }) }; }
                                active = Int11State::Point17 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15 };
                                continue;
                            },
                            Int11State::Point17 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point17 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntBridge { function: data::function::IntFunctionId(4), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2574, 2587)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int14.into(), int15.into()], ..CallValues::default() }), captures: None }, caller: IntReturn::Int11Call17 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15 } }
                                };
                            },
                            Int11State::Point18 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point18 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16 }); }
                                *budget -= 1;
                                let int17 = int13 + int16;
                                if int17 < i128::from(i64::MIN) || int17 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 19,
                                    ints: 18,
                                    bools: 1,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into()], bools: vec![bool0], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point19 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17 }); }
                                *budget -= 1;
                                let int18 = 4_i128;
                                if int18 < i128::from(i64::MIN) || int18 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 20,
                                    ints: 19,
                                    bools: 1,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into()], bools: vec![bool0], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point20 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18 }); }
                                *budget -= 1;
                                let int19 = 6_i128;
                                if int19 < i128::from(i64::MIN) || int19 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 21,
                                    ints: 20,
                                    bools: 1,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into()], bools: vec![bool0], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point21 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19 }); }
                                *budget -= 1;
                                let bool1 = true;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point22 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntBridge { function: data::function::IntFunctionId(5), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2592, 2612)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int18.into(), int19.into()], bools: vec![bool1], ..CallValues::default() }), captures: None }, caller: IntReturn::Int11Call22 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1 } }
                                };
                            },
                            Int11State::Point19 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point19 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17 }); }
                                *budget -= 1;
                                let int18 = 4_i128;
                                if int18 < i128::from(i64::MIN) || int18 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 20,
                                    ints: 19,
                                    bools: 1,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into()], bools: vec![bool0], ..CallValues::default() }) }; }
                                active = Int11State::Point20 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18 };
                                continue;
                            },
                            Int11State::Point20 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point20 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18 }); }
                                *budget -= 1;
                                let int19 = 6_i128;
                                if int19 < i128::from(i64::MIN) || int19 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 21,
                                    ints: 20,
                                    bools: 1,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into()], bools: vec![bool0], ..CallValues::default() }) }; }
                                active = Int11State::Point21 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19 };
                                continue;
                            },
                            Int11State::Point21 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point21 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19 }); }
                                *budget -= 1;
                                let bool1 = true;
                                active = Int11State::Point22 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1 };
                                continue;
                            },
                            Int11State::Point22 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point22 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntBridge { function: data::function::IntFunctionId(5), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2592, 2612)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int18.into(), int19.into()], bools: vec![bool1], ..CallValues::default() }), captures: None }, caller: IntReturn::Int11Call22 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1 } }
                                };
                            },
                            Int11State::Point23 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point23 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20 }); }
                                *budget -= 1;
                                let int21 = int17 + int20;
                                if int21 < i128::from(i64::MIN) || int21 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 24,
                                    ints: 22,
                                    bools: 2,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into()], bools: vec![bool0, bool1], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point24 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21 }); }
                                *budget -= 1;
                                let int22 = 12_i128;
                                if int22 < i128::from(i64::MIN) || int22 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 25,
                                    ints: 23,
                                    bools: 2,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into(), int22.into()], bools: vec![bool0, bool1], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point25 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22 }); }
                                *budget -= 1;
                                let int23 = 7_i128;
                                if int23 < i128::from(i64::MIN) || int23 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 26,
                                    ints: 24,
                                    bools: 2,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into(), int22.into(), int23.into()], bools: vec![bool0, bool1], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point26 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23 }); }
                                *budget -= 1;
                                let bool2 = true;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point27 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntBridge { function: data::function::IntFunctionId(6), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2617, 2636)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int22.into(), int23.into()], bools: vec![bool2], ..CallValues::default() }), captures: None }, caller: IntReturn::Int11Call27 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2 } }
                                };
                            },
                            Int11State::Point24 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point24 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21 }); }
                                *budget -= 1;
                                let int22 = 12_i128;
                                if int22 < i128::from(i64::MIN) || int22 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 25,
                                    ints: 23,
                                    bools: 2,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into(), int22.into()], bools: vec![bool0, bool1], ..CallValues::default() }) }; }
                                active = Int11State::Point25 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22 };
                                continue;
                            },
                            Int11State::Point25 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point25 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22 }); }
                                *budget -= 1;
                                let int23 = 7_i128;
                                if int23 < i128::from(i64::MIN) || int23 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 26,
                                    ints: 24,
                                    bools: 2,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into(), int22.into(), int23.into()], bools: vec![bool0, bool1], ..CallValues::default() }) }; }
                                active = Int11State::Point26 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23 };
                                continue;
                            },
                            Int11State::Point26 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point26 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23 }); }
                                *budget -= 1;
                                let bool2 = true;
                                active = Int11State::Point27 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2 };
                                continue;
                            },
                            Int11State::Point27 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point27 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntBridge { function: data::function::IntFunctionId(6), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2617, 2636)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int22.into(), int23.into()], bools: vec![bool2], ..CallValues::default() }), captures: None }, caller: IntReturn::Int11Call27 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2 } }
                                };
                            },
                            Int11State::Point28 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point28 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24 }); }
                                *budget -= 1;
                                let int25 = int21 + int24;
                                if int25 < i128::from(i64::MIN) || int25 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 29,
                                    ints: 26,
                                    bools: 3,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into(), int22.into(), int23.into(), int24.into(), int25.into()], bools: vec![bool0, bool1, bool2], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point29 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25 }); }
                                *budget -= 1;
                                let int26 = 3_i128;
                                if int26 < i128::from(i64::MIN) || int26 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 30,
                                    ints: 27,
                                    bools: 3,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into(), int22.into(), int23.into(), int24.into(), int25.into(), int26.into()], bools: vec![bool0, bool1, bool2], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point30 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26 }); }
                                *budget -= 1;
                                let int27 = 4_i128;
                                if int27 < i128::from(i64::MIN) || int27 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 31,
                                    ints: 28,
                                    bools: 3,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into(), int22.into(), int23.into(), int24.into(), int25.into(), int26.into(), int27.into()], bools: vec![bool0, bool1, bool2], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point31 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntBridge { function: data::function::IntFunctionId(7), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2641, 2655)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int26.into(), int27.into()], ..CallValues::default() }), captures: None }, caller: IntReturn::Int11Call31 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27 } }
                                };
                            },
                            Int11State::Point29 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point29 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25 }); }
                                *budget -= 1;
                                let int26 = 3_i128;
                                if int26 < i128::from(i64::MIN) || int26 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 30,
                                    ints: 27,
                                    bools: 3,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into(), int22.into(), int23.into(), int24.into(), int25.into(), int26.into()], bools: vec![bool0, bool1, bool2], ..CallValues::default() }) }; }
                                active = Int11State::Point30 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26 };
                                continue;
                            },
                            Int11State::Point30 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point30 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26 }); }
                                *budget -= 1;
                                let int27 = 4_i128;
                                if int27 < i128::from(i64::MIN) || int27 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 31,
                                    ints: 28,
                                    bools: 3,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into(), int22.into(), int23.into(), int24.into(), int25.into(), int26.into(), int27.into()], bools: vec![bool0, bool1, bool2], ..CallValues::default() }) }; }
                                active = Int11State::Point31 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27 };
                                continue;
                            },
                            Int11State::Point31 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point31 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntBridge { function: data::function::IntFunctionId(7), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2641, 2655)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int26.into(), int27.into()], ..CallValues::default() }), captures: None }, caller: IntReturn::Int11Call31 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27 } }
                                };
                            },
                            Int11State::Point32 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point32 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28 }); }
                                *budget -= 1;
                                let int29 = int25 + int28;
                                if int29 < i128::from(i64::MIN) || int29 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 33,
                                    ints: 30,
                                    bools: 3,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into(), int22.into(), int23.into(), int24.into(), int25.into(), int26.into(), int27.into(), int28.into(), int29.into()], bools: vec![bool0, bool1, bool2], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point33 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29 }); }
                                *budget -= 1;
                                let int30 = 3_i128;
                                if int30 < i128::from(i64::MIN) || int30 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 34,
                                    ints: 31,
                                    bools: 3,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into(), int22.into(), int23.into(), int24.into(), int25.into(), int26.into(), int27.into(), int28.into(), int29.into(), int30.into()], bools: vec![bool0, bool1, bool2], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point34 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30 }); }
                                *budget -= 1;
                                let int31 = 9_i128;
                                if int31 < i128::from(i64::MIN) || int31 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 35,
                                    ints: 32,
                                    bools: 3,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into(), int22.into(), int23.into(), int24.into(), int25.into(), int26.into(), int27.into(), int28.into(), int29.into(), int30.into(), int31.into()], bools: vec![bool0, bool1, bool2], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point35 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31 }); }
                                *budget -= 1;
                                let bool3 = true;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point36 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntBridge { function: data::function::IntFunctionId(8), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2660, 2681)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int30.into(), int31.into()], bools: vec![bool3], ..CallValues::default() }), captures: None }, caller: IntReturn::Int11Call36 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3 } }
                                };
                            },
                            Int11State::Point33 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point33 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29 }); }
                                *budget -= 1;
                                let int30 = 3_i128;
                                if int30 < i128::from(i64::MIN) || int30 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 34,
                                    ints: 31,
                                    bools: 3,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into(), int22.into(), int23.into(), int24.into(), int25.into(), int26.into(), int27.into(), int28.into(), int29.into(), int30.into()], bools: vec![bool0, bool1, bool2], ..CallValues::default() }) }; }
                                active = Int11State::Point34 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30 };
                                continue;
                            },
                            Int11State::Point34 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point34 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30 }); }
                                *budget -= 1;
                                let int31 = 9_i128;
                                if int31 < i128::from(i64::MIN) || int31 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 35,
                                    ints: 32,
                                    bools: 3,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into(), int22.into(), int23.into(), int24.into(), int25.into(), int26.into(), int27.into(), int28.into(), int29.into(), int30.into(), int31.into()], bools: vec![bool0, bool1, bool2], ..CallValues::default() }) }; }
                                active = Int11State::Point35 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31 };
                                continue;
                            },
                            Int11State::Point35 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point35 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31 }); }
                                *budget -= 1;
                                let bool3 = true;
                                active = Int11State::Point36 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3 };
                                continue;
                            },
                            Int11State::Point36 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point36 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntBridge { function: data::function::IntFunctionId(8), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2660, 2681)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int30.into(), int31.into()], bools: vec![bool3], ..CallValues::default() }), captures: None }, caller: IntReturn::Int11Call36 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3 } }
                                };
                            },
                            Int11State::Point37 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point37 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32 }); }
                                *budget -= 1;
                                let int33 = int29 + int32;
                                if int33 < i128::from(i64::MIN) || int33 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 38,
                                    ints: 34,
                                    bools: 4,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into(), int22.into(), int23.into(), int24.into(), int25.into(), int26.into(), int27.into(), int28.into(), int29.into(), int30.into(), int31.into(), int32.into(), int33.into()], bools: vec![bool0, bool1, bool2, bool3], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point38 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33 }); }
                                *budget -= 1;
                                let int34 = 8_i128;
                                if int34 < i128::from(i64::MIN) || int34 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 39,
                                    ints: 35,
                                    bools: 4,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into(), int22.into(), int23.into(), int24.into(), int25.into(), int26.into(), int27.into(), int28.into(), int29.into(), int30.into(), int31.into(), int32.into(), int33.into(), int34.into()], bools: vec![bool0, bool1, bool2, bool3], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point39 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33, int34 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntScalarBridge { function: data::function::IntFunctionId(10), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2686, 2695)), input: CallNativeInput::Int(int34.into()), caller: IntReturn::Int11Call39 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33, int34 } }
                                };
                            },
                            Int11State::Point38 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point38 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33 }); }
                                *budget -= 1;
                                let int34 = 8_i128;
                                if int34 < i128::from(i64::MIN) || int34 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 39,
                                    ints: 35,
                                    bools: 4,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into(), int22.into(), int23.into(), int24.into(), int25.into(), int26.into(), int27.into(), int28.into(), int29.into(), int30.into(), int31.into(), int32.into(), int33.into(), int34.into()], bools: vec![bool0, bool1, bool2, bool3], ..CallValues::default() }) }; }
                                active = Int11State::Point39 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33, int34 };
                                continue;
                            },
                            Int11State::Point39 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33, int34 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point39 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33, int34 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntScalarBridge { function: data::function::IntFunctionId(10), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2686, 2695)), input: CallNativeInput::Int(int34.into()), caller: IntReturn::Int11Call39 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33, int34 } }
                                };
                            },
                            Int11State::Point40 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33, int34, int35 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point40 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33, int34, int35 }); }
                                *budget -= 1;
                                let int36 = int33 + int35;
                                if int36 < i128::from(i64::MIN) || int36 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 41,
                                    ints: 37,
                                    bools: 4,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into(), int15.into(), int16.into(), int17.into(), int18.into(), int19.into(), int20.into(), int21.into(), int22.into(), int23.into(), int24.into(), int25.into(), int26.into(), int27.into(), int28.into(), int29.into(), int30.into(), int31.into(), int32.into(), int33.into(), int34.into(), int35.into(), int36.into()], bools: vec![bool0, bool1, bool2, bool3], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point41 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33, int34, int35, int36 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int36 }
                                };
                            },
                            Int11State::Point41 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33, int34, int35, int36 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point41 { int0, int1, int2, int3, int4, int5, int6, int7, int8, int9, int10, int11, bool0, int12, int13, int14, int15, int16, int17, int18, int19, bool1, int20, int21, int22, int23, bool2, int24, int25, int26, int27, int28, int29, int30, int31, bool3, int32, int33, int34, int35, int36 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int36 }
                                };
                            },
                        }
                    }
                }
                enum Int12State {
                    Point0 { int0: i128, custom0: CallCustom },
                    Point1 { int0: i128, int1: i128 },
                    Point2 { int0: i128, int1: i128, int2: i128 },
                    Point3 { int0: i128, custom0: CallCustom },
                    Point4 { int0: i128, int1: i128 },
                    Point5 { int0: i128, int1: i128, int2: i128 },
                    Point6 { int0: i128 },
                }
                fn calls_int_12_run(mut active: Int12State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            Int12State::Point0 { int0, custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int12Point0 { int0, custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(i128,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(0),
                                            index: 0,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Int) { return None; }
                                        let binding0 = field0.integer()?;
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(12)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into()], ..CallValues::default() }) }; };
                                    *budget -= 1;
                                    match matched {
                                        Some((binding0,)) => Int12State::Point1 { int0, int1: binding0 },
                                        None => Int12State::Point3 { int0, custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int12State::Point1 { int0, int1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int12Point1 { int0, int1 }); }
                                *budget -= 1;
                                let int2 = int0 + int1;
                                if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(12)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
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
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int12Point2 { int0, int1, int2 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int2 }
                                };
                            },
                            Int12State::Point2 { int0, int1, int2 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int12Point2 { int0, int1, int2 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int2 }
                                };
                            },
                            Int12State::Point3 { int0, custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int12Point3 { int0, custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(i128,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(0),
                                            index: 1,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Int) { return None; }
                                        let binding0 = field0.integer()?;
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(12)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into()], ..CallValues::default() }) }; };
                                    *budget -= 1;
                                    match matched {
                                        Some((binding0,)) => Int12State::Point4 { int0, int1: binding0 },
                                        None => Int12State::Point6 { int0 },
                                    }
                                };
                                continue;
                            },
                            Int12State::Point4 { int0, int1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int12Point4 { int0, int1 }); }
                                *budget -= 1;
                                let int2 = int0 - int1;
                                if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(12)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(3),
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
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int12Point5 { int0, int1, int2 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int2 }
                                };
                            },
                            Int12State::Point5 { int0, int1, int2 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int12Point5 { int0, int1, int2 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int2 }
                                };
                            },
                            Int12State::Point6 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int12Point6 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                        }
                    }
                }
                enum Int13State {
                    Point0 { custom0: CallCustom },
                    Point1 { int0: i128, custom0: CallCustom },
                    Point2 { int0: i128 },
                    Point3 { int0: i128, int1: i128 },
                    Point4 { custom0: CallCustom },
                    Point5 { custom0: CallCustom },
                    Point6 { int0: i128 },
                    Point7 { int0: i128, int1: i128 },
                    Point8 { custom0: CallCustom },
                    Point9 { custom0: CallCustom, int0: i128 },
                    Point10 { custom0: CallCustom },
                }
                fn calls_int_13_run(mut active: Int13State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            Int13State::Point0 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point0 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(i128,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(1),
                                            index: 0,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Int) { return None; }
                                        if field0.integer()? != 3_i128 { return Some(None); }
                                        let field1 = custom0.field(1)?;
                                        if !field1.matches_type(&data::type_::ValueType::Int) { return None; }
                                        let binding0 = field1.integer()?;
                                        let field2 = custom0.field(2)?;
                                        if !field2.matches_type(&data::type_::ValueType::Bool) { return None; }
                                        if !field2.boolean()? { return Some(None); }
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(13)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) }; };
                                    *budget -= 1;
                                    match matched {
                                        Some((binding0,)) => Int13State::Point1 { int0: binding0, custom0: custom0.clone() },
                                        None => Int13State::Point10 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int13State::Point1 { int0, custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point1 { int0, custom0 }); }
                                *budget -= 1;
                                active = {
                                    if int0 > 0_i128 { Int13State::Point2 { int0 } } else { Int13State::Point4 { custom0: custom0.clone() } }
                                };
                                continue;
                            },
                            Int13State::Point2 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point2 { int0 }); }
                                *budget -= 1;
                                let int1 = int0 * 2_i128;
                                if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(13)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
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
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point3 { int0, int1 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int1 }
                                };
                            },
                            Int13State::Point3 { int0, int1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point3 { int0, int1 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int1 }
                                };
                            },
                            Int13State::Point4 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point4 { custom0 }); }
                                *budget -= 1;
                                active = {
                                    Int13State::Point5 { custom0: custom0.clone() }
                                };
                                continue;
                            },
                            Int13State::Point5 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point5 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(i128,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(1),
                                            index: 0,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Int) { return None; }
                                        let binding0 = field0.integer()?;
                                        let field1 = custom0.field(1)?;
                                        if !field1.matches_type(&data::type_::ValueType::Int) { return None; }
                                        if field1.integer()? != 9_i128 { return Some(None); }
                                        let field2 = custom0.field(2)?;
                                        if !field2.matches_type(&data::type_::ValueType::Bool) { return None; }
                                        if !field2.boolean()? { return Some(None); }
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(13)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(4),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) }; };
                                    *budget -= 1;
                                    match matched {
                                        Some((binding0,)) => Int13State::Point6 { int0: binding0 },
                                        None => Int13State::Point8 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int13State::Point6 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point6 { int0 }); }
                                *budget -= 1;
                                let int1 = int0 - 1_i128;
                                if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(13)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(5),
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
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point7 { int0, int1 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int1 }
                                };
                            },
                            Int13State::Point7 { int0, int1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point7 { int0, int1 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int1 }
                                };
                            },
                            Int13State::Point8 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point8 { custom0 }); }
                                let int0 = match (|| {
                                let field = custom0.field(1)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(13)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(6),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) },
                                };
                                *budget -= 1;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point9 { custom0, int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int13State::Point9 { custom0, int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point9 { custom0, int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int13State::Point10 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point10 { custom0 }); }
                                *budget -= 1;
                                active = {
                                    Int13State::Point5 { custom0: custom0.clone() }
                                };
                                continue;
                            },
                        }
                    }
                }
                enum Int14State {
                    Point0 { custom0: CallCustom },
                    Point1 { custom0: CallCustom, int0: i128 },
                    Point2 { custom0: CallCustom, int0: i128, int1: i128 },
                    Point3 { custom0: CallCustom, int0: i128, int1: i128, int2: i128 },
                    Point4 { custom0: CallCustom, int0: i128, int1: i128, int2: i128, int3: i128 },
                    Point5 { custom0: CallCustom, int0: i128, int1: i128, int2: i128, int3: i128, int4: i128 },
                }
                fn calls_int_14_run(mut active: Int14State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            Int14State::Point0 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int14Point0 { custom0 }); }
                                let int0 = match (|| {
                                let field = custom0.field(0)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(14)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) },
                                };
                                *budget -= 1;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int14Point1 { custom0, int0 }); }
                                let int1 = match (|| {
                                let field = custom0.field(1)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(14)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into()], ..CallValues::default() }) },
                                };
                                *budget -= 1;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int14Point2 { custom0, int0, int1 }); }
                                *budget -= 1;
                                let int2 = int0 + int1;
                                if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(14)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 3,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into(), int1.into(), int2.into()], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int14Point3 { custom0, int0, int1, int2 }); }
                                let int3 = match (|| {
                                let field = custom0.field(1)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(14)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 3,
                                        ints: 3,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into(), int1.into(), int2.into()], ..CallValues::default() }) },
                                };
                                *budget -= 1;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int14Point4 { custom0, int0, int1, int2, int3 }); }
                                *budget -= 1;
                                let int4 = int2 + int3;
                                if int4 < i128::from(i64::MIN) || int4 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(14)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 5,
                                    ints: 5,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into()], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int14Point5 { custom0, int0, int1, int2, int3, int4 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int4 }
                                };
                            },
                            Int14State::Point1 { custom0, int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int14Point1 { custom0, int0 }); }
                                let int1 = match (|| {
                                let field = custom0.field(1)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(14)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into()], ..CallValues::default() }) },
                                };
                                *budget -= 1;
                                active = Int14State::Point2 { custom0, int0, int1 };
                                continue;
                            },
                            Int14State::Point2 { custom0, int0, int1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int14Point2 { custom0, int0, int1 }); }
                                *budget -= 1;
                                let int2 = int0 + int1;
                                if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(14)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 3,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into(), int1.into(), int2.into()], ..CallValues::default() }) }; }
                                active = Int14State::Point3 { custom0, int0, int1, int2 };
                                continue;
                            },
                            Int14State::Point3 { custom0, int0, int1, int2 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int14Point3 { custom0, int0, int1, int2 }); }
                                let int3 = match (|| {
                                let field = custom0.field(1)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(14)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 3,
                                        ints: 3,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into(), int1.into(), int2.into()], ..CallValues::default() }) },
                                };
                                *budget -= 1;
                                active = Int14State::Point4 { custom0, int0, int1, int2, int3 };
                                continue;
                            },
                            Int14State::Point4 { custom0, int0, int1, int2, int3 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int14Point4 { custom0, int0, int1, int2, int3 }); }
                                *budget -= 1;
                                let int4 = int2 + int3;
                                if int4 < i128::from(i64::MIN) || int4 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(14)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 5,
                                    ints: 5,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into()], ..CallValues::default() }) }; }
                                active = Int14State::Point5 { custom0, int0, int1, int2, int3, int4 };
                                continue;
                            },
                            Int14State::Point5 { custom0, int0, int1, int2, int3, int4 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int14Point5 { custom0, int0, int1, int2, int3, int4 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int4 }
                                };
                            },
                        }
                    }
                }
                enum Int16State {
                    Point0 { custom0: CallCustom },
                    Point1 { custom0: CallCustom, bool0: bool },
                    Point2 { custom0: CallCustom },
                    Point3 { custom0: CallCustom, int0: i128 },
                    Point4 { custom0: CallCustom, int0: i128, int1: i128 },
                    Point5 { custom0: CallCustom, int0: i128, int1: i128, int2: i128 },
                    Point6 { custom0: CallCustom, int0: i128, int1: i128, int2: i128, int3: i128 },
                    Point7 { custom0: CallCustom },
                    Point8 { custom0: CallCustom, int0: i128 },
                    Point9 { custom0: CallCustom, int0: i128, int1: i128 },
                    Point10 { custom0: CallCustom, int0: i128, int1: i128, int2: i128 },
                }
                fn calls_int_16_run(mut active: Int16State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            Int16State::Point0 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point0 { custom0 }); }
                                let bool0 = match (|| {
                                let field = custom0.field(2)?;
                                    if !field.matches_type(&data::type_::ValueType::Bool) { return None; }
                                    field.boolean()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(16)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) },
                                };
                                *budget -= 1;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point1 { custom0, bool0 }); }
                                *budget -= 1;
                                active = {
                                    if bool0 { Int16State::Point2 { custom0: custom0.clone() } } else { Int16State::Point7 { custom0: custom0.clone() } }
                                };
                                continue;
                            },
                            Int16State::Point1 { custom0, bool0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point1 { custom0, bool0 }); }
                                *budget -= 1;
                                active = {
                                    if bool0 { Int16State::Point2 { custom0: custom0.clone() } } else { Int16State::Point7 { custom0: custom0.clone() } }
                                };
                                continue;
                            },
                            Int16State::Point2 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point2 { custom0 }); }
                                let int0 = match (|| {
                                let field = custom0.field(0)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(16)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) },
                                };
                                *budget -= 1;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point3 { custom0, int0 }); }
                                *budget -= 1;
                                let int1 = if 3_i128 == 0 { 0_i128 } else { int0 / 3_i128 };
                                if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(16)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 2,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into(), int1.into()], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point4 { custom0, int0, int1 }); }
                                let int2 = match (|| {
                                let field = custom0.field(1)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(16)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 2,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into(), int1.into()], ..CallValues::default() }) },
                                };
                                *budget -= 1;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point5 { custom0, int0, int1, int2 }); }
                                *budget -= 1;
                                let region0 = if 4_i128 == 0 { 0_i128 } else { int2 % 4_i128 };
                                let region1 = int1 + region0;
                                let int3 = region1;
                                if int3 < i128::from(i64::MIN) || int3 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(16)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 4,
                                    ints: 4,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into(), int1.into(), int2.into(), int3.into()], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point6 { custom0, int0, int1, int2, int3 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int3 }
                                };
                            },
                            Int16State::Point3 { custom0, int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point3 { custom0, int0 }); }
                                *budget -= 1;
                                let int1 = if 3_i128 == 0 { 0_i128 } else { int0 / 3_i128 };
                                if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(16)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 2,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into(), int1.into()], ..CallValues::default() }) }; }
                                active = Int16State::Point4 { custom0, int0, int1 };
                                continue;
                            },
                            Int16State::Point4 { custom0, int0, int1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point4 { custom0, int0, int1 }); }
                                let int2 = match (|| {
                                let field = custom0.field(1)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(16)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 2,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into(), int1.into()], ..CallValues::default() }) },
                                };
                                *budget -= 1;
                                active = Int16State::Point5 { custom0, int0, int1, int2 };
                                continue;
                            },
                            Int16State::Point5 { custom0, int0, int1, int2 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point5 { custom0, int0, int1, int2 }); }
                                *budget -= 1;
                                let region0 = if 4_i128 == 0 { 0_i128 } else { int2 % 4_i128 };
                                let region1 = int1 + region0;
                                let int3 = region1;
                                if int3 < i128::from(i64::MIN) || int3 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(16)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 4,
                                    ints: 4,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into(), int1.into(), int2.into(), int3.into()], ..CallValues::default() }) }; }
                                active = Int16State::Point6 { custom0, int0, int1, int2, int3 };
                                continue;
                            },
                            Int16State::Point6 { custom0, int0, int1, int2, int3 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point6 { custom0, int0, int1, int2, int3 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int3 }
                                };
                            },
                            Int16State::Point7 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point7 { custom0 }); }
                                let int0 = match (|| {
                                let field = custom0.field(0)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(16)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) },
                                };
                                *budget -= 1;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point8 { custom0, int0 }); }
                                let int1 = match (|| {
                                let field = custom0.field(1)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(16)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 1,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into()], ..CallValues::default() }) },
                                };
                                *budget -= 1;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point9 { custom0, int0, int1 }); }
                                *budget -= 1;
                                let int2 = int0 - int1;
                                if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(16)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 3,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into(), int1.into(), int2.into()], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point10 { custom0, int0, int1, int2 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int2 }
                                };
                            },
                            Int16State::Point8 { custom0, int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point8 { custom0, int0 }); }
                                let int1 = match (|| {
                                let field = custom0.field(1)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(16)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 1,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into()], ..CallValues::default() }) },
                                };
                                *budget -= 1;
                                active = Int16State::Point9 { custom0, int0, int1 };
                                continue;
                            },
                            Int16State::Point9 { custom0, int0, int1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point9 { custom0, int0, int1 }); }
                                *budget -= 1;
                                let int2 = int0 - int1;
                                if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(16)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 3,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into(), int1.into(), int2.into()], ..CallValues::default() }) }; }
                                active = Int16State::Point10 { custom0, int0, int1, int2 };
                                continue;
                            },
                            Int16State::Point10 { custom0, int0, int1, int2 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point10 { custom0, int0, int1, int2 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int2 }
                                };
                            },
                        }
                    }
                }
                enum Int17State {
                    Point0 { custom0: CallCustom, int0: i128, int1: i128 },
                    Point1 { custom0: CallCustom, int0: i128, int1: i128 },
                    Point2 { custom0: CallCustom, int0: i128, int1: i128, int2: i128 },
                    Point3 { custom0: CallCustom, int0: i128, int1: i128, int2: i128, int3: i128 },
                    Point4 { custom0: CallCustom, int0: i128, int1: i128, int2: i128, int3: i128, int4: i128 },
                    Point5 { int0: i128 },
                }
                fn calls_int_17_run(mut active: Int17State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            Int17State::Point0 { custom0, int0, int1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int17Point0 { custom0, int0, int1 }); }
                                *budget -= 1;
                                active = {
                                    if int0 > 0_i128 { Int17State::Point1 { custom0: custom0.clone(), int0, int1 } } else { Int17State::Point5 { int0: int1 } }
                                };
                                continue;
                            },
                            Int17State::Point1 { custom0, int0, int1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int17Point1 { custom0, int0, int1 }); }
                                *budget -= 1;
                                let int2 = int0 - 1_i128;
                                if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(17)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 1,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into(), int1.into(), int2.into()], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int17Point2 { custom0, int0, int1, int2 }); }
                                let int3 = match (|| {
                                let field = custom0.field(0)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(17)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 1,
                                        ints: 3,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into(), int1.into(), int2.into()], ..CallValues::default() }) },
                                };
                                *budget -= 1;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int17Point3 { custom0, int0, int1, int2, int3 }); }
                                *budget -= 1;
                                let int4 = int1 + int3;
                                if int4 < i128::from(i64::MIN) || int4 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(17)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 3,
                                    ints: 5,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into()], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int17Point4 { custom0, int0, int1, int2, int3, int4 }); }
                                *budget -= 1;
                                active = {
                                    Int17State::Point0 { custom0: custom0.clone(), int0: int2, int1: int4 }
                                };
                                continue;
                            },
                            Int17State::Point2 { custom0, int0, int1, int2 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int17Point2 { custom0, int0, int1, int2 }); }
                                let int3 = match (|| {
                                let field = custom0.field(0)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(17)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 1,
                                        ints: 3,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into(), int1.into(), int2.into()], ..CallValues::default() }) },
                                };
                                *budget -= 1;
                                active = Int17State::Point3 { custom0, int0, int1, int2, int3 };
                                continue;
                            },
                            Int17State::Point3 { custom0, int0, int1, int2, int3 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int17Point3 { custom0, int0, int1, int2, int3 }); }
                                *budget -= 1;
                                let int4 = int1 + int3;
                                if int4 < i128::from(i64::MIN) || int4 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(17)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 3,
                                    ints: 5,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { customs: vec![custom0], ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into()], ..CallValues::default() }) }; }
                                active = Int17State::Point4 { custom0, int0, int1, int2, int3, int4 };
                                continue;
                            },
                            Int17State::Point4 { custom0, int0, int1, int2, int3, int4 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int17Point4 { custom0, int0, int1, int2, int3, int4 }); }
                                *budget -= 1;
                                active = {
                                    Int17State::Point0 { custom0: custom0.clone(), int0: int2, int1: int4 }
                                };
                                continue;
                            },
                            Int17State::Point5 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int17Point5 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                        }
                    }
                }
                enum Int20State {
                    Point0 { custom0: CallCustom },
                    Point1 { custom0: CallCustom, custom1: CallCustom },
                    Point2 { custom0: CallCustom, custom1: CallCustom, int0: i128 },
                }
                fn calls_int_20_run(mut active: Int20State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            Int20State::Point0 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int20Point0 { custom0 }); }
                                let custom1 = match (|| {
                                let field = custom0.field(0)?;
                                    if !field.matches_type(&data::type_::ValueType::Custom(data::type_::CustomTypeId(1))) { return None; }
                                    field.custom()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(20)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) },
                                };
                                *budget -= 1;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int20Point1 { custom0, custom1 }); }
                                let int0 = match (|| {
                                let field = custom1.field(0)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(20)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 2,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0, custom1], ..CallValues::default() }) },
                                };
                                *budget -= 1;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int20Point2 { custom0, custom1, int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int20State::Point1 { custom0, custom1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int20Point1 { custom0, custom1 }); }
                                let int0 = match (|| {
                                let field = custom1.field(0)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(20)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 2,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0, custom1], ..CallValues::default() }) },
                                };
                                *budget -= 1;
                                active = Int20State::Point2 { custom0, custom1, int0 };
                                continue;
                            },
                            Int20State::Point2 { custom0, custom1, int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int20Point2 { custom0, custom1, int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                        }
                    }
                }
                fn calls_int_2_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int2Point0 { int0: values.int(0)? },
                        1 => FunctionState::Int2Point1 { int0: values.int(0)?, nullary0: values.nullary(0, &[
                            data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(0),
                                index: 2,
                            },
                        ])? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_2_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point, values) { return Some(execution); }
                    let active = calls_int_2_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_11_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int11Point0 {  },
                        1 => FunctionState::Int11Point1 { int0: values.int(0)? },
                        2 => FunctionState::Int11Point2 { int0: values.int(0)?, int1: values.int(1)? },
                        3 => FunctionState::Int11Point3 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        4 => FunctionState::Int11Point4 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)? },
                        5 => FunctionState::Int11Point5 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)? },
                        6 => FunctionState::Int11Point6 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)? },
                        7 => FunctionState::Int11Point7 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)? },
                        8 => FunctionState::Int11Point8 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)? },
                        9 => FunctionState::Int11Point9 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)? },
                        10 => FunctionState::Int11Point10 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)? },
                        11 => FunctionState::Int11Point11 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)? },
                        12 => FunctionState::Int11Point12 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)? },
                        13 => FunctionState::Int11Point13 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)? },
                        14 => FunctionState::Int11Point14 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)? },
                        15 => FunctionState::Int11Point15 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)? },
                        16 => FunctionState::Int11Point16 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)? },
                        17 => FunctionState::Int11Point17 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)? },
                        18 => FunctionState::Int11Point18 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)? },
                        19 => FunctionState::Int11Point19 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)? },
                        20 => FunctionState::Int11Point20 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)?, int18: values.int(18)? },
                        21 => FunctionState::Int11Point21 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)?, int18: values.int(18)?, int19: values.int(19)? },
                        22 => FunctionState::Int11Point22 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)?, int18: values.int(18)?, int19: values.int(19)?, bool1: values.bool(1)? },
                        23 => FunctionState::Int11Point23 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)?, int18: values.int(18)?, int19: values.int(19)?, bool1: values.bool(1)?, int20: values.int(20)? },
                        24 => FunctionState::Int11Point24 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)?, int18: values.int(18)?, int19: values.int(19)?, bool1: values.bool(1)?, int20: values.int(20)?, int21: values.int(21)? },
                        25 => FunctionState::Int11Point25 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)?, int18: values.int(18)?, int19: values.int(19)?, bool1: values.bool(1)?, int20: values.int(20)?, int21: values.int(21)?, int22: values.int(22)? },
                        26 => FunctionState::Int11Point26 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)?, int18: values.int(18)?, int19: values.int(19)?, bool1: values.bool(1)?, int20: values.int(20)?, int21: values.int(21)?, int22: values.int(22)?, int23: values.int(23)? },
                        27 => FunctionState::Int11Point27 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)?, int18: values.int(18)?, int19: values.int(19)?, bool1: values.bool(1)?, int20: values.int(20)?, int21: values.int(21)?, int22: values.int(22)?, int23: values.int(23)?, bool2: values.bool(2)? },
                        28 => FunctionState::Int11Point28 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)?, int18: values.int(18)?, int19: values.int(19)?, bool1: values.bool(1)?, int20: values.int(20)?, int21: values.int(21)?, int22: values.int(22)?, int23: values.int(23)?, bool2: values.bool(2)?, int24: values.int(24)? },
                        29 => FunctionState::Int11Point29 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)?, int18: values.int(18)?, int19: values.int(19)?, bool1: values.bool(1)?, int20: values.int(20)?, int21: values.int(21)?, int22: values.int(22)?, int23: values.int(23)?, bool2: values.bool(2)?, int24: values.int(24)?, int25: values.int(25)? },
                        30 => FunctionState::Int11Point30 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)?, int18: values.int(18)?, int19: values.int(19)?, bool1: values.bool(1)?, int20: values.int(20)?, int21: values.int(21)?, int22: values.int(22)?, int23: values.int(23)?, bool2: values.bool(2)?, int24: values.int(24)?, int25: values.int(25)?, int26: values.int(26)? },
                        31 => FunctionState::Int11Point31 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)?, int18: values.int(18)?, int19: values.int(19)?, bool1: values.bool(1)?, int20: values.int(20)?, int21: values.int(21)?, int22: values.int(22)?, int23: values.int(23)?, bool2: values.bool(2)?, int24: values.int(24)?, int25: values.int(25)?, int26: values.int(26)?, int27: values.int(27)? },
                        32 => FunctionState::Int11Point32 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)?, int18: values.int(18)?, int19: values.int(19)?, bool1: values.bool(1)?, int20: values.int(20)?, int21: values.int(21)?, int22: values.int(22)?, int23: values.int(23)?, bool2: values.bool(2)?, int24: values.int(24)?, int25: values.int(25)?, int26: values.int(26)?, int27: values.int(27)?, int28: values.int(28)? },
                        33 => FunctionState::Int11Point33 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)?, int18: values.int(18)?, int19: values.int(19)?, bool1: values.bool(1)?, int20: values.int(20)?, int21: values.int(21)?, int22: values.int(22)?, int23: values.int(23)?, bool2: values.bool(2)?, int24: values.int(24)?, int25: values.int(25)?, int26: values.int(26)?, int27: values.int(27)?, int28: values.int(28)?, int29: values.int(29)? },
                        34 => FunctionState::Int11Point34 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)?, int18: values.int(18)?, int19: values.int(19)?, bool1: values.bool(1)?, int20: values.int(20)?, int21: values.int(21)?, int22: values.int(22)?, int23: values.int(23)?, bool2: values.bool(2)?, int24: values.int(24)?, int25: values.int(25)?, int26: values.int(26)?, int27: values.int(27)?, int28: values.int(28)?, int29: values.int(29)?, int30: values.int(30)? },
                        35 => FunctionState::Int11Point35 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)?, int18: values.int(18)?, int19: values.int(19)?, bool1: values.bool(1)?, int20: values.int(20)?, int21: values.int(21)?, int22: values.int(22)?, int23: values.int(23)?, bool2: values.bool(2)?, int24: values.int(24)?, int25: values.int(25)?, int26: values.int(26)?, int27: values.int(27)?, int28: values.int(28)?, int29: values.int(29)?, int30: values.int(30)?, int31: values.int(31)? },
                        36 => FunctionState::Int11Point36 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)?, int18: values.int(18)?, int19: values.int(19)?, bool1: values.bool(1)?, int20: values.int(20)?, int21: values.int(21)?, int22: values.int(22)?, int23: values.int(23)?, bool2: values.bool(2)?, int24: values.int(24)?, int25: values.int(25)?, int26: values.int(26)?, int27: values.int(27)?, int28: values.int(28)?, int29: values.int(29)?, int30: values.int(30)?, int31: values.int(31)?, bool3: values.bool(3)? },
                        37 => FunctionState::Int11Point37 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)?, int18: values.int(18)?, int19: values.int(19)?, bool1: values.bool(1)?, int20: values.int(20)?, int21: values.int(21)?, int22: values.int(22)?, int23: values.int(23)?, bool2: values.bool(2)?, int24: values.int(24)?, int25: values.int(25)?, int26: values.int(26)?, int27: values.int(27)?, int28: values.int(28)?, int29: values.int(29)?, int30: values.int(30)?, int31: values.int(31)?, bool3: values.bool(3)?, int32: values.int(32)? },
                        38 => FunctionState::Int11Point38 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)?, int18: values.int(18)?, int19: values.int(19)?, bool1: values.bool(1)?, int20: values.int(20)?, int21: values.int(21)?, int22: values.int(22)?, int23: values.int(23)?, bool2: values.bool(2)?, int24: values.int(24)?, int25: values.int(25)?, int26: values.int(26)?, int27: values.int(27)?, int28: values.int(28)?, int29: values.int(29)?, int30: values.int(30)?, int31: values.int(31)?, bool3: values.bool(3)?, int32: values.int(32)?, int33: values.int(33)? },
                        39 => FunctionState::Int11Point39 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)?, int18: values.int(18)?, int19: values.int(19)?, bool1: values.bool(1)?, int20: values.int(20)?, int21: values.int(21)?, int22: values.int(22)?, int23: values.int(23)?, bool2: values.bool(2)?, int24: values.int(24)?, int25: values.int(25)?, int26: values.int(26)?, int27: values.int(27)?, int28: values.int(28)?, int29: values.int(29)?, int30: values.int(30)?, int31: values.int(31)?, bool3: values.bool(3)?, int32: values.int(32)?, int33: values.int(33)?, int34: values.int(34)? },
                        40 => FunctionState::Int11Point40 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)?, int18: values.int(18)?, int19: values.int(19)?, bool1: values.bool(1)?, int20: values.int(20)?, int21: values.int(21)?, int22: values.int(22)?, int23: values.int(23)?, bool2: values.bool(2)?, int24: values.int(24)?, int25: values.int(25)?, int26: values.int(26)?, int27: values.int(27)?, int28: values.int(28)?, int29: values.int(29)?, int30: values.int(30)?, int31: values.int(31)?, bool3: values.bool(3)?, int32: values.int(32)?, int33: values.int(33)?, int34: values.int(34)?, int35: values.int(35)? },
                        41 => FunctionState::Int11Point41 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, bool0: values.bool(0)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)?, int15: values.int(15)?, int16: values.int(16)?, int17: values.int(17)?, int18: values.int(18)?, int19: values.int(19)?, bool1: values.bool(1)?, int20: values.int(20)?, int21: values.int(21)?, int22: values.int(22)?, int23: values.int(23)?, bool2: values.bool(2)?, int24: values.int(24)?, int25: values.int(25)?, int26: values.int(26)?, int27: values.int(27)?, int28: values.int(28)?, int29: values.int(29)?, int30: values.int(30)?, int31: values.int(31)?, bool3: values.bool(3)?, int32: values.int(32)?, int33: values.int(33)?, int34: values.int(34)?, int35: values.int(35)?, int36: values.int(36)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_11_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point, values) { return Some(execution); }
                    let active = calls_int_11_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_12_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int12Point0 { int0: values.int(0)?, custom0: values.custom(0)? },
                        1 => FunctionState::Int12Point1 { int0: values.int(0)?, int1: values.int(1)? },
                        2 => FunctionState::Int12Point2 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        3 => FunctionState::Int12Point3 { int0: values.int(0)?, custom0: values.custom(0)? },
                        4 => FunctionState::Int12Point4 { int0: values.int(0)?, int1: values.int(1)? },
                        5 => FunctionState::Int12Point5 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        6 => FunctionState::Int12Point6 { int0: values.int(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_12_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(12)), point, values) { return Some(execution); }
                    let active = calls_int_12_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_13_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int13Point0 { custom0: values.custom(0)? },
                        1 => FunctionState::Int13Point1 { int0: values.int(0)?, custom0: values.custom(0)? },
                        2 => FunctionState::Int13Point2 { int0: values.int(0)? },
                        3 => FunctionState::Int13Point3 { int0: values.int(0)?, int1: values.int(1)? },
                        4 => FunctionState::Int13Point4 { custom0: values.custom(0)? },
                        5 => FunctionState::Int13Point5 { custom0: values.custom(0)? },
                        6 => FunctionState::Int13Point6 { int0: values.int(0)? },
                        7 => FunctionState::Int13Point7 { int0: values.int(0)?, int1: values.int(1)? },
                        8 => FunctionState::Int13Point8 { custom0: values.custom(0)? },
                        9 => FunctionState::Int13Point9 { custom0: values.custom(0)?, int0: values.int(0)? },
                        10 => FunctionState::Int13Point10 { custom0: values.custom(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_13_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(13)), point, values) { return Some(execution); }
                    let active = calls_int_13_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_14_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int14Point0 { custom0: values.custom(0)? },
                        1 => FunctionState::Int14Point1 { custom0: values.custom(0)?, int0: values.int(0)? },
                        2 => FunctionState::Int14Point2 { custom0: values.custom(0)?, int0: values.int(0)?, int1: values.int(1)? },
                        3 => FunctionState::Int14Point3 { custom0: values.custom(0)?, int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        4 => FunctionState::Int14Point4 { custom0: values.custom(0)?, int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)? },
                        5 => FunctionState::Int14Point5 { custom0: values.custom(0)?, int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_14_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(14)), point, values) { return Some(execution); }
                    let active = calls_int_14_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_16_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int16Point0 { custom0: values.custom(0)? },
                        1 => FunctionState::Int16Point1 { custom0: values.custom(0)?, bool0: values.bool(0)? },
                        2 => FunctionState::Int16Point2 { custom0: values.custom(0)? },
                        3 => FunctionState::Int16Point3 { custom0: values.custom(0)?, int0: values.int(0)? },
                        4 => FunctionState::Int16Point4 { custom0: values.custom(0)?, int0: values.int(0)?, int1: values.int(1)? },
                        5 => FunctionState::Int16Point5 { custom0: values.custom(0)?, int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        6 => FunctionState::Int16Point6 { custom0: values.custom(0)?, int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)? },
                        7 => FunctionState::Int16Point7 { custom0: values.custom(0)? },
                        8 => FunctionState::Int16Point8 { custom0: values.custom(0)?, int0: values.int(0)? },
                        9 => FunctionState::Int16Point9 { custom0: values.custom(0)?, int0: values.int(0)?, int1: values.int(1)? },
                        10 => FunctionState::Int16Point10 { custom0: values.custom(0)?, int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_16_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(16)), point, values) { return Some(execution); }
                    let active = calls_int_16_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_17_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int17Point0 { custom0: values.custom(0)?, int0: values.int(0)?, int1: values.int(1)? },
                        1 => FunctionState::Int17Point1 { custom0: values.custom(0)?, int0: values.int(0)?, int1: values.int(1)? },
                        2 => FunctionState::Int17Point2 { custom0: values.custom(0)?, int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        3 => FunctionState::Int17Point3 { custom0: values.custom(0)?, int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)? },
                        4 => FunctionState::Int17Point4 { custom0: values.custom(0)?, int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)? },
                        5 => FunctionState::Int17Point5 { int0: values.int(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_17_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(17)), point, values) { return Some(execution); }
                    let active = calls_int_17_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_20_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int20Point0 { custom0: values.custom(0)? },
                        1 => FunctionState::Int20Point1 { custom0: values.custom(0)?, custom1: values.custom(1)? },
                        2 => FunctionState::Int20Point2 { custom0: values.custom(0)?, custom1: values.custom(1)?, int0: values.int(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_20_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(20)), point, values) { return Some(execution); }
                    let active = calls_int_20_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                [calls_int_2_start, calls_int_11_start, calls_int_12_start, calls_int_13_start, calls_int_14_start, calls_int_16_start, calls_int_17_start, calls_int_20_start]
            };
            const CALL_GROUP_1: [data::compiled::calls::CallStart; 1] = {
                use data::compiled::calls::{CallCustom, CallExecution, CallInputs, CallOps, CallOutput, CallProgress, CallStorage, CallValues};
                enum FunctionState {
                    Bool1Point0 { custom0: CallCustom },
                    Bool1Point1 { custom0: CallCustom },
                    Bool1Point2 { custom0: CallCustom, int0: i128 },
                    Bool1Point3 { custom0: CallCustom, int0: i128, bool0: bool },
                    Bool1Point4 { custom0: CallCustom },
                    Bool1Point5 { custom0: CallCustom, bool0: bool },
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
                    Bool { value: bool },
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
                                        return CallProgress::Complete { output: CallOutput::Bool(value), execution: self };
                                    }
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
                                        _ => return CallProgress::Interpreted { target, point, values },
                                    }
                                },
                            }
                        }
                    }
                }
                fn function_step(active: FunctionState, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        FunctionState::Bool1Point0 { custom0 } => calls_bool_1_run(Bool1State::Point0 { custom0 }, ops, budget),
                        FunctionState::Bool1Point1 { custom0 } => calls_bool_1_run(Bool1State::Point1 { custom0 }, ops, budget),
                        FunctionState::Bool1Point2 { custom0, int0 } => calls_bool_1_run(Bool1State::Point2 { custom0, int0 }, ops, budget),
                        FunctionState::Bool1Point3 { custom0, int0, bool0 } => calls_bool_1_run(Bool1State::Point3 { custom0, int0, bool0 }, ops, budget),
                        FunctionState::Bool1Point4 { custom0 } => calls_bool_1_run(Bool1State::Point4 { custom0 }, ops, budget),
                        FunctionState::Bool1Point5 { custom0, bool0 } => calls_bool_1_run(Bool1State::Point5 { custom0, bool0 }, ops, budget),
                    }
                }
                enum Bool1State {
                    Point0 { custom0: CallCustom },
                    Point1 { custom0: CallCustom },
                    Point2 { custom0: CallCustom, int0: i128 },
                    Point3 { custom0: CallCustom, int0: i128, bool0: bool },
                    Point4 { custom0: CallCustom },
                    Point5 { custom0: CallCustom, bool0: bool },
                }
                fn calls_bool_1_run(mut active: Bool1State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            Bool1State::Point0 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point0 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<()>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(1),
                                            index: 0,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Int) { return None; }
                                        if field0.integer()? != 3_i128 { return Some(None); }
                                        let field1 = custom0.field(1)?;
                                        if !field1.matches_type(&data::type_::ValueType::Int) { return None; }
                                        let field2 = custom0.field(2)?;
                                        if !field2.matches_type(&data::type_::ValueType::Bool) { return None; }
                                        if !field2.boolean()? { return Some(None); }
                                        Some(Some(()))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) }; };
                                    *budget -= 1;
                                    match matched {
                                        Some(()) => Bool1State::Point1 { custom0: custom0.clone() },
                                        None => Bool1State::Point4 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Bool1State::Point1 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point1 { custom0 }); }
                                let int0 = match (|| {
                                let field = custom0.field(1)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) },
                                };
                                *budget -= 1;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point2 { custom0, int0 }); }
                                *budget -= 1;
                                let bool0 = int0 > 0_i128;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point3 { custom0, int0, bool0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Bool { value: bool0 }
                                };
                            },
                            Bool1State::Point2 { custom0, int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point2 { custom0, int0 }); }
                                *budget -= 1;
                                let bool0 = int0 > 0_i128;
                                active = Bool1State::Point3 { custom0, int0, bool0 };
                                continue;
                            },
                            Bool1State::Point3 { custom0, int0, bool0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point3 { custom0, int0, bool0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Bool { value: bool0 }
                                };
                            },
                            Bool1State::Point4 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point4 { custom0 }); }
                                let bool0 = match (|| {
                                let field = custom0.field(2)?;
                                    if !field.matches_type(&data::type_::ValueType::Bool) { return None; }
                                    field.boolean()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) },
                                };
                                *budget -= 1;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point5 { custom0, bool0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Bool { value: bool0 }
                                };
                            },
                            Bool1State::Point5 { custom0, bool0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point5 { custom0, bool0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Bool { value: bool0 }
                                };
                            },
                        }
                    }
                }
                fn calls_bool_1_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool1Point0 { custom0: values.custom(0)? },
                        1 => FunctionState::Bool1Point1 { custom0: values.custom(0)? },
                        2 => FunctionState::Bool1Point2 { custom0: values.custom(0)?, int0: values.int(0)? },
                        3 => FunctionState::Bool1Point3 { custom0: values.custom(0)?, int0: values.int(0)?, bool0: values.bool(0)? },
                        4 => FunctionState::Bool1Point4 { custom0: values.custom(0)? },
                        5 => FunctionState::Bool1Point5 { custom0: values.custom(0)?, bool0: values.bool(0)? },
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
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 1,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
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
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(2),
                                        },
                                    }),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[]),
                            tails: data::Storage::Static(&[
                                data::compiled::TailContract {
                                    point: 1,
                                    target: data::compiled::CallTarget::Int(data::function::IntFunctionId(12)),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(2),
                                            },
                                        }),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "ignored", data::source::SourceSpan::new(399, 421)),
                                },
                            ]),
                            start: CALL_GROUP_0[0],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)),
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
                                    instruction: 3,
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
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 4,
                                    ints: 4,
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
                                    instruction: 5,
                                    ints: 5,
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
                                    instruction: 6,
                                    ints: 6,
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
                                    instruction: 7,
                                    ints: 7,
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
                                    instruction: 8,
                                    ints: 8,
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
                                    instruction: 9,
                                    ints: 9,
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
                                    instruction: 10,
                                    ints: 10,
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
                                    instruction: 11,
                                    ints: 11,
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
                                    instruction: 12,
                                    ints: 12,
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
                                    instruction: 13,
                                    ints: 12,
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
                                    instruction: 14,
                                    ints: 13,
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
                                    instruction: 15,
                                    ints: 14,
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
                                    instruction: 16,
                                    ints: 15,
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
                                    instruction: 17,
                                    ints: 16,
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
                                    instruction: 18,
                                    ints: 17,
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
                                    instruction: 19,
                                    ints: 18,
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
                                    instruction: 20,
                                    ints: 19,
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
                                    instruction: 21,
                                    ints: 20,
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
                                    instruction: 22,
                                    ints: 20,
                                    bools: 2,
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
                                    instruction: 23,
                                    ints: 21,
                                    bools: 2,
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
                                    instruction: 24,
                                    ints: 22,
                                    bools: 2,
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
                                    instruction: 25,
                                    ints: 23,
                                    bools: 2,
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
                                    instruction: 26,
                                    ints: 24,
                                    bools: 2,
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
                                    instruction: 27,
                                    ints: 24,
                                    bools: 3,
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
                                    instruction: 28,
                                    ints: 25,
                                    bools: 3,
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
                                    instruction: 29,
                                    ints: 26,
                                    bools: 3,
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
                                    instruction: 30,
                                    ints: 27,
                                    bools: 3,
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
                                    instruction: 31,
                                    ints: 28,
                                    bools: 3,
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
                                    instruction: 32,
                                    ints: 29,
                                    bools: 3,
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
                                    instruction: 33,
                                    ints: 30,
                                    bools: 3,
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
                                    instruction: 34,
                                    ints: 31,
                                    bools: 3,
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
                                    instruction: 35,
                                    ints: 32,
                                    bools: 3,
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
                                    instruction: 36,
                                    ints: 32,
                                    bools: 4,
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
                                    instruction: 37,
                                    ints: 33,
                                    bools: 4,
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
                                    instruction: 38,
                                    ints: 34,
                                    bools: 4,
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
                                    instruction: 39,
                                    ints: 35,
                                    bools: 4,
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
                                    instruction: 40,
                                    ints: 36,
                                    bools: 4,
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
                                    instruction: 41,
                                    ints: 37,
                                    bools: 4,
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
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(20)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(20)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(21)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(20)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(21)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(22)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(20)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(21)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(22)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(23)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(20)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(21)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(22)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(23)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(20)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(21)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(22)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(23)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(24)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(20)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(21)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(22)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(23)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(24)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(25)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(20)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(21)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(22)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(23)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(24)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(25)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(26)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(20)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(21)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(22)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(23)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(24)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(25)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(26)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(27)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(20)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(21)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(22)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(23)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(24)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(25)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(26)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(27)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(28)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(20)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(21)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(22)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(23)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(24)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(25)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(26)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(27)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(28)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(29)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(20)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(21)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(22)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(23)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(24)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(25)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(26)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(27)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(28)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(29)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(30)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(20)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(21)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(22)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(23)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(24)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(25)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(26)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(27)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(28)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(29)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(30)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(31)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(20)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(21)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(22)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(23)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(24)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(25)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(26)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(27)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(28)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(29)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(30)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(31)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(3)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(20)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(21)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(22)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(23)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(24)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(25)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(26)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(27)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(28)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(29)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(30)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(31)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(32)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(20)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(21)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(22)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(23)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(24)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(25)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(26)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(27)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(28)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(29)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(30)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(31)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(32)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(33)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(20)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(21)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(22)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(23)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(24)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(25)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(26)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(27)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(28)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(29)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(30)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(31)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(32)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(33)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(34)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(20)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(21)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(22)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(23)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(24)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(25)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(26)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(27)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(28)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(29)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(30)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(31)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(32)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(33)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(34)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(35)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(17)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(20)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(21)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(22)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(23)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(24)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(25)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(26)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(27)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(28)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(29)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(30)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(31)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(32)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(33)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(34)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(35)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(36)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[
                                data::compiled::CallContract {
                                    point: 2,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(0))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2500, 2513)),
                                },
                                data::compiled::CallContract {
                                    point: 5,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(1))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2518, 2530)),
                                },
                                data::compiled::CallContract {
                                    point: 8,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(2))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2535, 2545)),
                                },
                                data::compiled::CallContract {
                                    point: 13,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(3))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2550, 2569)),
                                },
                                data::compiled::CallContract {
                                    point: 17,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(16)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(4))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(15)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2574, 2587)),
                                },
                                data::compiled::CallContract {
                                    point: 22,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(20)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(5))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(18)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(19)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2592, 2612)),
                                },
                                data::compiled::CallContract {
                                    point: 27,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(24)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(6))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(22)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(23)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2617, 2636)),
                                },
                                data::compiled::CallContract {
                                    point: 31,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(28)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(7))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(26)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(27)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2641, 2655)),
                                },
                                data::compiled::CallContract {
                                    point: 36,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(32)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(8))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(30)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(31)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(3)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2660, 2681)),
                                },
                                data::compiled::CallContract {
                                    point: 39,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(35)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(10))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(34)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2686, 2695)),
                                },
                            ]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 41,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(36)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_0[1],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(12)),
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
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
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
                                    block: data::graph::BlockId(1),
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
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 0,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(3),
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
                                    block: data::graph::BlockId(3),
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
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(4),
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
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(6),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(6),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 2,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                },
                                data::compiled::ReturnContract {
                                    point: 5,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                },
                                data::compiled::ReturnContract {
                                    point: 6,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_0[2],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(13)),
                        implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                            root: false,
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
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 0,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
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
                                    int_functions: 0,
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
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(3),
                                    instruction: 0,
                                    ints: 0,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(4),
                                    instruction: 0,
                                    ints: 0,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(5),
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
                                    block: data::graph::BlockId(5),
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
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(6),
                                    instruction: 0,
                                    ints: 0,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(6),
                                    instruction: 1,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(7),
                                    instruction: 0,
                                    ints: 0,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                            ]),
                            locals: data::Storage::Static(&[
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 3,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                },
                                data::compiled::ReturnContract {
                                    point: 7,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                },
                                data::compiled::ReturnContract {
                                    point: 9,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_0[3],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(14)),
                        implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                            root: false,
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
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 2,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 3,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 4,
                                    ints: 4,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 5,
                                    ints: 5,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                            ]),
                            locals: data::Storage::Static(&[
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 5,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_0[4],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(16)),
                        implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                            root: false,
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
                                    customs: 1,
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
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
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
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 1,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 2,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 3,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 4,
                                    ints: 4,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 0,
                                    ints: 0,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 1,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
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
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 3,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                            ]),
                            locals: data::Storage::Static(&[
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 6,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                },
                                data::compiled::ReturnContract {
                                    point: 10,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_0[5],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(17)),
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
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 0,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 1,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 2,
                                    ints: 4,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 3,
                                    ints: 5,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
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
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                            ]),
                            locals: data::Storage::Static(&[
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 5,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_0[6],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(20)),
                        implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                            root: false,
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
                                    customs: 1,
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
                                    customs: 2,
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
                                    customs: 2,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                            ]),
                            locals: data::Storage::Static(&[
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(2),
                                            shape_id: data::type_::CustomValueShapeId(7),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(2),
                                            shape_id: data::type_::CustomValueShapeId(7),
                                        },
                                    }),
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(1),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(2),
                                            shape_id: data::type_::CustomValueShapeId(7),
                                        },
                                    }),
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(1),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 2,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_0[7],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)),
                        implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                            root: false,
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
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
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
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 1,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 2,
                                    ints: 1,
                                    bools: 1,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 0,
                                    ints: 0,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 1,
                                    ints: 0,
                                    bools: 1,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                            ]),
                            locals: data::Storage::Static(&[
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(4),
                                        },
                                    }),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 3,
                                    value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                },
                                data::compiled::ReturnContract {
                                    point: 5,
                                    value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_1[0],
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
                0..21,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                21..23,
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
                0..0,
                0..0,
            ],
            functions: data::Storage::Static(&[
                data::function::FunctionContract {
                    parameters: 0..2,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 2..4,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
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
                    parameters: 5..8,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(4),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 8..10,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 10..13,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(4),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 13..16,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(4),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 16..18,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 18..21,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(4),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 21..23,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 23..24,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 24..24,
                    parameter_shapes: data::Storage::Static(&[]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 24..26,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(8),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 26..27,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(6),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 27..28,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(6),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 28..30,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(6),
                        data::type_::ValueShapeId(6),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 30..31,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(6),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 31..34,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(6),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 34..35,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(6),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 35..36,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(6),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 36..37,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(11),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 37..40,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(4),
                    ]),
                    return_: data::type_::ValueShapeId(4),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 40..41,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(6),
                    ]),
                    return_: data::type_::ValueShapeId(4),
                    captures: data::Storage::Static(&[]),
                },
            ]),
            parameters: data::Storage::Static(&[
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(0),
                        shape_id: data::type_::CustomValueShapeId(6),
                    },
                }),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(1),
                        shape_id: data::type_::CustomValueShapeId(4),
                    },
                }),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(1),
                        shape_id: data::type_::CustomValueShapeId(4),
                    },
                }),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(1),
                        shape_id: data::type_::CustomValueShapeId(4),
                    },
                }),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(1),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(1),
                        shape_id: data::type_::CustomValueShapeId(4),
                    },
                }),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(1),
                        shape_id: data::type_::CustomValueShapeId(4),
                    },
                }),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(1),
                        shape_id: data::type_::CustomValueShapeId(4),
                    },
                }),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(1),
                        shape_id: data::type_::CustomValueShapeId(4),
                    },
                }),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(1),
                        shape_id: data::type_::CustomValueShapeId(4),
                    },
                }),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(2),
                        shape_id: data::type_::CustomValueShapeId(7),
                    },
                }),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(1),
                        shape_id: data::type_::CustomValueShapeId(4),
                    },
                }),
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
                        package: data::Text::Static("geam"),
                        module: data::Text::Static("example"),
                        name: data::Text::Static("Entry"),
                        arguments: data::Storage::Static(&[]),
                    },
                    native_visible: true,
                    lifetime: data::host::HostValueLifetime::LoadedOwner,
                    constructor_count: 3,
                    constructors: data::Storage::Static(&[
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(0),
                                index: 0,
                            },
                            name: data::Text::Static("Credit"),
                            native_tag: data::Text::Static("credit"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::Int,
                                    shape: data::type_::ValueShapeId(0),
                                    refinement: data::type_::FieldRefinement::Value,
                                },
                            ]),
                        },
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(0),
                                index: 1,
                            },
                            name: data::Text::Static("Debit"),
                            native_tag: data::Text::Static("debit"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::Int,
                                    shape: data::type_::ValueShapeId(0),
                                    refinement: data::type_::FieldRefinement::Value,
                                },
                            ]),
                        },
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(0),
                                index: 2,
                            },
                            name: data::Text::Static("Ignored"),
                            native_tag: data::Text::Static("ignored"),
                            fields: data::Storage::Static(&[]),
                        },
                    ]),
                },
                data::type_::CustomTypeDescriptor {
                    type_: data::type_::NominalTypeMetadata {
                        package: data::Text::Static("geam"),
                        module: data::Text::Static("example"),
                        name: data::Text::Static("Data"),
                        arguments: data::Storage::Static(&[]),
                    },
                    native_visible: true,
                    lifetime: data::host::HostValueLifetime::LoadedOwner,
                    constructor_count: 1,
                    constructors: data::Storage::Static(&[
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(1),
                                index: 0,
                            },
                            name: data::Text::Static("Data"),
                            native_tag: data::Text::Static("data"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: Some(data::Text::Static("left")),
                                    type_: data::type_::ValueType::Int,
                                    shape: data::type_::ValueShapeId(0),
                                    refinement: data::type_::FieldRefinement::Value,
                                },
                                data::type_::CustomFieldDescriptor {
                                    label: Some(data::Text::Static("right")),
                                    type_: data::type_::ValueType::Int,
                                    shape: data::type_::ValueShapeId(0),
                                    refinement: data::type_::FieldRefinement::Value,
                                },
                                data::type_::CustomFieldDescriptor {
                                    label: Some(data::Text::Static("enabled")),
                                    type_: data::type_::ValueType::Bool,
                                    shape: data::type_::ValueShapeId(4),
                                    refinement: data::type_::FieldRefinement::Value,
                                },
                            ]),
                        },
                    ]),
                },
                data::type_::CustomTypeDescriptor {
                    type_: data::type_::NominalTypeMetadata {
                        package: data::Text::Static("geam"),
                        module: data::Text::Static("example"),
                        name: data::Text::Static("Nested"),
                        arguments: data::Storage::Static(&[]),
                    },
                    native_visible: false,
                    lifetime: data::host::HostValueLifetime::LoadedOwner,
                    constructor_count: 1,
                    constructors: data::Storage::Static(&[
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(2),
                                index: 0,
                            },
                            name: data::Text::Static("Nested"),
                            native_tag: data::Text::Static("nested"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                                    shape: data::type_::ValueShapeId(6),
                                    refinement: data::type_::FieldRefinement::Custom(data::Storage::Static(&[])),
                                },
                            ]),
                        },
                    ]),
                },
            ]),
            definitions: data::Storage::Static(&[
                data::type_::CustomDefinition {
                    package: data::Text::Static("geam"),
                    module: data::Text::Static("example"),
                    name: data::Text::Static("Data"),
                    publicity: data::type_::CustomTypePublicity::Public,
                    opaque: false,
                    native_access: None,
                    retention_lifetime: data::host::HostValueLifetime::LoadedOwner,
                    parameters: 0,
                    constructors: data::Storage::Static(&[
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Data"),
                            fields: data::Storage::Static(&[
                                data::type_::FieldDefinition {
                                    label: Some(data::Text::Static("left")),
                                    type_: data::type_::TypeMetadata::Int,
                                },
                                data::type_::FieldDefinition {
                                    label: Some(data::Text::Static("right")),
                                    type_: data::type_::TypeMetadata::Int,
                                },
                                data::type_::FieldDefinition {
                                    label: Some(data::Text::Static("enabled")),
                                    type_: data::type_::TypeMetadata::Bool,
                                },
                            ]),
                        },
                    ]),
                },
                data::type_::CustomDefinition {
                    package: data::Text::Static("geam"),
                    module: data::Text::Static("example"),
                    name: data::Text::Static("Entry"),
                    publicity: data::type_::CustomTypePublicity::Public,
                    opaque: false,
                    native_access: None,
                    retention_lifetime: data::host::HostValueLifetime::LoadedOwner,
                    parameters: 0,
                    constructors: data::Storage::Static(&[
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Credit"),
                            fields: data::Storage::Static(&[
                                data::type_::FieldDefinition {
                                    label: None,
                                    type_: data::type_::TypeMetadata::Int,
                                },
                            ]),
                        },
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Debit"),
                            fields: data::Storage::Static(&[
                                data::type_::FieldDefinition {
                                    label: None,
                                    type_: data::type_::TypeMetadata::Int,
                                },
                            ]),
                        },
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Ignored"),
                            fields: data::Storage::Static(&[]),
                        },
                    ]),
                },
                data::type_::CustomDefinition {
                    package: data::Text::Static("geam"),
                    module: data::Text::Static("example"),
                    name: data::Text::Static("Nested"),
                    publicity: data::type_::CustomTypePublicity::Private,
                    opaque: false,
                    native_access: None,
                    retention_lifetime: data::host::HostValueLifetime::LoadedOwner,
                    parameters: 0,
                    constructors: data::Storage::Static(&[
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Nested"),
                            fields: data::Storage::Static(&[
                                data::type_::FieldDefinition {
                                    label: None,
                                    type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                        package: data::Text::Static("geam"),
                                        module: data::Text::Static("example"),
                                        name: data::Text::Static("Data"),
                                        arguments: data::Storage::Static(&[]),
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
            lifetimes: data::Storage::Static(&[]),
            definitions: data::Storage::Static(&[]),
        },
        value_shapes: data::type_::ValueShapeTable {
            shapes: data::Storage::Static(&[
                data::type_::ValueShapeDescriptor::Int,
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(0)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(1)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(2)),
                data::type_::ValueShapeDescriptor::Bool,
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(3)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(4)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(5)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(6)),
                data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                    data::type_::ValueShapeId(6),
                    data::type_::ValueShapeId(6),
                ])),
                data::type_::ValueShapeDescriptor::String,
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(7)),
            ]),
            shape_types: data::Storage::Static(&[
                data::type_::ValueType::Int,
                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                data::type_::ValueType::Bool,
                data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                data::type_::ValueType::Tuple(data::Storage::Static(&[
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                ])),
                data::type_::ValueType::String,
                data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
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
                    constructor: data::type_::CustomConstructorRefinement::Exact(2),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(1),
                    arguments: data::Storage::Static(&[]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(1),
                    arguments: data::Storage::Static(&[]),
                    constructor: data::type_::CustomConstructorRefinement::Any,
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(2),
                    arguments: data::Storage::Static(&[]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(0),
                    arguments: data::Storage::Static(&[]),
                    constructor: data::type_::CustomConstructorRefinement::Any,
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(2),
                    arguments: data::Storage::Static(&[]),
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
            data::program::LibraryFunctionEntry {
                function: data::function::IntFunctionId(3),
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
                function: data::function::IntFunctionId(4),
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
                function: data::function::IntFunctionId(5),
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
                function: data::function::IntFunctionId(6),
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
                function: data::function::IntFunctionId(7),
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
                function: data::function::IntFunctionId(8),
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
                function: data::function::IntFunctionId(9),
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
                function: data::function::IntFunctionId(10),
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
                function: data::function::IntFunctionId(11),
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
            name: data::Text::Static("credit"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 0,
        },
        data::Export {
            name: data::Text::Static("debit"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 1,
        },
        data::Export {
            name: data::Text::Static("ignored"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 2,
        },
        data::Export {
            name: data::Text::Static("guarded"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Bool,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 3,
        },
        data::Export {
            name: data::Text::Static("aliased"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 4,
        },
        data::Export {
            name: data::Text::Static("multiple"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Bool,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 5,
        },
        data::Export {
            name: data::Text::Static("boolean"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Bool,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            slot: 0,
        },
        data::Export {
            name: data::Text::Static("fields"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Bool,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 6,
        },
        data::Export {
            name: data::Text::Static("repeated"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 7,
        },
        data::Export {
            name: data::Text::Static("assertion"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Bool,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 8,
        },
        data::Export {
            name: data::Text::Static("panic_case"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 9,
        },
        data::Export {
            name: data::Text::Static("nested"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 10,
        },
        data::Export {
            name: data::Text::Static("main"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 11,
        },
    ]),
}
