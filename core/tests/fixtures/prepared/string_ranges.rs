data::ModuleArtifact {
    format: 21,
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
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::StringStartsWith {
                                                value: data::graph::StringLocalId(0),
                                                prefix: data::Text::Static("λ"),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..4,
                                        instructions: 0..2,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
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
                                        params: 4..6,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::Equal {
                                                left: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                right: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(4),
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
                                    data::graph::BlockHeader {
                                        params: 7..7,
                                        instructions: 3..4,
                                        terminator: data::graph::Terminator::SourceStop(data::graph::SourceStop {
                                            kind: data::graph::SourceStopKind::Panic,
                                            message: Some(data::graph::StringLocalId(0)),
                                            site: data::source::PanicSite::from_static("example", "count", data::source::SourceSpan::new(130, 163)),
                                        }),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::DropPrefix {
                                            value: data::graph::StringLocalId(0),
                                            prefix: data::Text::Static("λ"),
                                        }),
                                    }),
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
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static(""))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("expected lambda prefix"))),
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
                                        terminator: data::graph::Terminator::StringSwitch(data::graph::StringSwitch {
                                            subject: data::graph::StringLocalId(0),
                                            clauses: data::Storage::Static(&[
                                                (data::Text::Static("red"), data::graph::Edge {
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
                                                }),
                                                (data::Text::Static("blue"), data::graph::Edge {
                                                    target: data::graph::BlockId(2),
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
                                                }),
                                                (data::Text::Static("\n\"\\λ"), data::graph::Edge {
                                                    target: data::graph::BlockId(3),
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
                                                }),
                                            ]),
                                            fallback: data::graph::Edge {
                                                target: data::graph::BlockId(4),
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
                                        params: 1..1,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..1,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..1,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..1,
                                        instructions: 3..4,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(3)),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Minus,
                                            digits: data::Storage::Static(&[
                                                1,
                                            ]),
                                        })),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 4,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..4,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::StringStartsWith {
                                                value: data::graph::StringLocalId(0),
                                                prefix: data::Text::Static("λ"),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(20),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                        params: 4..8,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::Equal {
                                                left: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                right: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
                                                            length: 2,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(19),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
                                                            length: 2,
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 8..12,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                            subject: data::graph::BoolLocalId(0),
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
                                                            length: 1,
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
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(9),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                        params: 12..14,
                                        instructions: 1..3,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::Equal {
                                                left: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                right: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(4),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
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
                                        params: 14..16,
                                        instructions: 3..4,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::Equal {
                                                left: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                right: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(5),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                            false_: data::graph::Edge {
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
                                        params: 16..17,
                                        instructions: 4..5,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 17..17,
                                        instructions: 5..5,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(7),
                                                args: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 17..17,
                                        instructions: 5..6,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 17..17,
                                        instructions: 6..6,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(7),
                                                args: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 17..20,
                                        instructions: 6..6,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(10),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 20..23,
                                        instructions: 6..6,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(11),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 23..26,
                                        instructions: 6..6,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::StringStartsWith {
                                                value: data::graph::StringLocalId(0),
                                                prefix: data::Text::Static("λ"),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(12),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(18),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
                                                            length: 1,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 26..29,
                                        instructions: 6..7,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::Equal {
                                                left: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                right: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(13),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
                                                            length: 1,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(14),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
                                                            length: 1,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 29..31,
                                        instructions: 7..9,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 31..33,
                                        instructions: 9..9,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(15),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 33..35,
                                        instructions: 9..9,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::StringStartsWith {
                                                value: data::graph::StringLocalId(0),
                                                prefix: data::Text::Static("λ"),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(16),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(17),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                        params: 35..36,
                                        instructions: 9..10,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(3)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 36..37,
                                        instructions: 10..11,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(4)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 37..39,
                                        instructions: 11..11,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(15),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 39..42,
                                        instructions: 11..11,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(10),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 42..45,
                                        instructions: 11..11,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(11),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::DropPrefix {
                                            value: data::graph::StringLocalId(0),
                                            prefix: data::Text::Static("λ"),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("λ"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::DropPrefix {
                                            value: data::graph::StringLocalId(0),
                                            prefix: data::Text::Static("λ"),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("λ"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Immediate(2),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Minus,
                                            digits: data::Storage::Static(&[
                                                100,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::DropPrefix {
                                            value: data::graph::StringLocalId(0),
                                            prefix: data::Text::Static("λ"),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::DropPrefix {
                                            value: data::graph::StringLocalId(0),
                                            prefix: data::Text::Static("λ"),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Immediate(4),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Immediate(8),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Immediate(16),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
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
                                            test: data::graph::BoolTest::StringStartsWith {
                                                value: data::graph::StringLocalId(0),
                                                prefix: data::Text::Static("λ"),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 3..6,
                                        instructions: 0..2,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
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
                                                            length: 2,
                                                            steps: data::Storage::Static(&[
                                                                data::graph::TransferStep {
                                                                    source: 1,
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 6..9,
                                        instructions: 2..2,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::StringStartsWith {
                                                value: data::graph::StringLocalId(0),
                                                prefix: data::Text::Static("m"),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(4),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 9..12,
                                        instructions: 2..4,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
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
                                                            length: 2,
                                                            steps: data::Storage::Static(&[
                                                                data::graph::TransferStep {
                                                                    source: 1,
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 12..15,
                                        instructions: 4..5,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::Equal {
                                                left: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                right: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(5),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
                                                            length: 2,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(8),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                        params: 15..18,
                                        instructions: 5..5,
                                        terminator: data::graph::Terminator::StringSwitch(data::graph::StringSwitch {
                                            subject: data::graph::StringLocalId(1),
                                            clauses: data::Storage::Static(&[
                                                (data::Text::Static(""), data::graph::Edge {
                                                    target: data::graph::BlockId(6),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                                }),
                                            ]),
                                            fallback: data::graph::Edge {
                                                target: data::graph::BlockId(7),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 18..19,
                                        instructions: 5..5,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 19..22,
                                        instructions: 5..5,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
                                                            length: 2,
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
                                        params: 22..23,
                                        instructions: 5..5,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::DropPrefix {
                                            value: data::graph::StringLocalId(0),
                                            prefix: data::Text::Static("λ"),
                                        }),
                                    }),
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
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::DropPrefix {
                                            value: data::graph::StringLocalId(0),
                                            prefix: data::Text::Static("m"),
                                        }),
                                    }),
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
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static(""))),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
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
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::StringStartsWith {
                                                value: data::graph::StringLocalId(0),
                                                prefix: data::Text::Static("λ"),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(5),
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
                                        params: 0..1,
                                        instructions: 1..3,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::Equal {
                                                left: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                right: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
                                                            length: 1,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(3),
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
                                        params: 1..2,
                                        instructions: 3..5,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..2,
                                        instructions: 5..5,
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
                                        params: 2..2,
                                        instructions: 5..6,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..2,
                                        instructions: 6..6,
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
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("λtail"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::DropPrefix {
                                            value: data::graph::StringLocalId(0),
                                            prefix: data::Text::Static("λ"),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("tail"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::DropPrefix {
                                            value: data::graph::StringLocalId(0),
                                            prefix: data::Text::Static("λ"),
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
                                                7,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Minus,
                                            digits: data::Storage::Static(&[
                                                1,
                                            ]),
                                        })),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
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
                                            subject: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            pattern: data::graph::MatchPattern::Alias {
                                                pattern: data::Storage::Static(&data::graph::MatchPattern::StringPrefix {
                                                    prefix: data::Text::Static("λ"),
                                                    left: Some(data::graph::MatchPatternBinding {
                                                        index: 0,
                                                    }),
                                                    right: Some(data::graph::MatchPatternBinding {
                                                        index: 1,
                                                    }),
                                                }),
                                                binding: data::graph::MatchPatternBinding {
                                                    index: 2,
                                                },
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Binding(0),
                                                    data::graph::MatchEdgeArgument::Binding(1),
                                                    data::graph::MatchEdgeArgument::Binding(2),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    0,
                                                    1,
                                                    2,
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
                                                            length: 3,
                                                            steps: data::Storage::Static(&[
                                                                data::graph::TransferStep {
                                                                    source: 1,
                                                                    destination: 0,
                                                                },
                                                                data::graph::TransferStep {
                                                                    source: 2,
                                                                    destination: 1,
                                                                },
                                                                data::graph::TransferStep {
                                                                    source: 3,
                                                                    destination: 2,
                                                                },
                                                            ]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(6),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
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
                                        params: 2..6,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::NotEqual {
                                                left: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                right: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
                                                            length: 1,
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
                                                            family: data::graph::StorageFamily::Int,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
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
                                        params: 6..8,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::Equal {
                                                left: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                right: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(4),
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
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 8..9,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 9..9,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 9..9,
                                        instructions: 3..4,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 9..10,
                                        instructions: 4..5,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            message: Some(data::graph::StringLocalId(1)),
                                            site: data::source::PanicSite::from_static("example", "asserted", data::source::SourceSpan::new(1508, 1518)),
                                            pattern_span: data::source::SourceSpan::new(1519, 1550),
                                        }),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
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
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("λ"))),
                                    }),
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
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Minus,
                                            digits: data::Storage::Static(&[
                                                1,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Minus,
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
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("lambda required"))),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
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
                                        terminator: data::graph::Terminator::StringSwitch(data::graph::StringSwitch {
                                            subject: data::graph::StringLocalId(2),
                                            clauses: data::Storage::Static(&[
                                                (data::Text::Static("tail"), data::graph::Edge {
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
                                                }),
                                            ]),
                                            fallback: data::graph::Edge {
                                                target: data::graph::BlockId(2),
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
                                        params: 1..1,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..1,
                                        instructions: 3..4,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
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
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("tail"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Concatenate {
                                            left: data::graph::StringLocalId(0),
                                            right: data::graph::StringLocalId(1),
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
                                                1,
                                            ]),
                                        })),
                                    }),
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
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
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
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::StringStartsWith {
                                                value: data::graph::StringLocalId(0),
                                                prefix: data::Text::Static("λ"),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
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
                                        params: 1..2,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
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
                                        params: 2..2,
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
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
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
                            parameter_count: 0,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..0,
                                        instructions: 0..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                ]),
                                params: data::Storage::Static(&[]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("λλλ"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
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
                                        function: data::function::IntFunctionId(0),
                                        site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2203, 2221)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                            subject: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            pattern: data::graph::MatchPattern::Alias {
                                                pattern: data::Storage::Static(&data::graph::MatchPattern::String(data::Text::Static("\n\"\\λ"))),
                                                binding: data::graph::MatchPatternBinding {
                                                    index: 0,
                                                },
                                            },
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
                                                            family: data::graph::StorageFamily::String,
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
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(4),
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
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::StringSwitch(data::graph::StringSwitch {
                                            subject: data::graph::StringLocalId(0),
                                            clauses: data::Storage::Static(&[
                                                (data::Text::Static("\n\"\\λ"), data::graph::Edge {
                                                    target: data::graph::BlockId(2),
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
                                                }),
                                            ]),
                                            fallback: data::graph::Edge {
                                                target: data::graph::BlockId(3),
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
                                        params: 2..2,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..2,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..3,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            message: Some(data::graph::StringLocalId(1)),
                                            site: data::source::PanicSite::from_static("example", "assert_literal", data::source::SourceSpan::new(2272, 2282)),
                                            pattern_span: data::source::SourceSpan::new(2283, 2302),
                                        }),
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
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
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
                                                17,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Minus,
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
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("literal required"))),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
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
                                            subject: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            pattern: data::graph::MatchPattern::StringPrefix {
                                                prefix: data::Text::Static("λ"),
                                                left: None,
                                                right: None,
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[]),
                                                bindings: data::Storage::Static(&[]),
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
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(2),
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
                                        params: 1..1,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..2,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            message: Some(data::graph::StringLocalId(1)),
                                            site: data::source::PanicSite::from_static("example", "assert_prefix", data::source::SourceSpan::new(2433, 2443)),
                                            pattern_span: data::source::SourceSpan::new(2444, 2453),
                                        }),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Plus,
                                            digits: data::Storage::Static(&[
                                                19,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("prefix required"))),
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
                                            subject: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            pattern: data::graph::MatchPattern::StringPrefix {
                                                prefix: data::Text::Static("λ"),
                                                left: Some(data::graph::MatchPatternBinding {
                                                    index: 0,
                                                }),
                                                right: Some(data::graph::MatchPatternBinding {
                                                    index: 1,
                                                }),
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Binding(1),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    1,
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
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
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(6),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
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
                                        params: 2..4,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::NotEqualInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(-1),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(5),
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
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..6,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::StringSwitch(data::graph::StringSwitch {
                                            subject: data::graph::StringLocalId(0),
                                            clauses: data::Storage::Static(&[
                                                (data::Text::Static("tail"), data::graph::Edge {
                                                    target: data::graph::BlockId(3),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                                }),
                                            ]),
                                            fallback: data::graph::Edge {
                                                target: data::graph::BlockId(4),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                        params: 6..7,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 7..8,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 8..8,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 8..9,
                                        instructions: 3..4,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            message: Some(data::graph::StringLocalId(1)),
                                            site: data::source::PanicSite::from_static("example", "assert_suffix", data::source::SourceSpan::new(2548, 2558)),
                                            pattern_span: data::source::SourceSpan::new(2559, 2581),
                                        }),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
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
                                            right: data::graph::IntegerOperand::Immediate(23),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Immediate(29),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Minus,
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
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("suffix required"))),
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
                            parameter_count: 3,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..3,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            pattern: data::graph::MatchPattern::BitArray(data::graph::BitArrayPattern {
                                                segments: data::Storage::Static(&[
                                                    data::graph::BitArrayPatternSegment::Int {
                                                        pattern: data::graph::BitArrayPatternValue::Bind(data::graph::MatchIntPatternBinding {
                                                            binding: data::graph::MatchPatternBinding {
                                                                index: 0,
                                                            },
                                                            size: None,
                                                        }),
                                                        size: data::graph::BitArrayPatternSize::Fixed(8),
                                                        endianness: data::graph::Endianness::Big,
                                                        signedness: data::graph::Signedness::Unsigned,
                                                    },
                                                    data::graph::BitArrayPatternSegment::Bits {
                                                        pattern: data::graph::BitArrayBindingPattern::Discard,
                                                        size: None,
                                                        unit: 1,
                                                    },
                                                ]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Binding(0),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0))),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1))),
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    0,
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::BitArray,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(4),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
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
                                        params: 3..6,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::Equal {
                                                left: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                right: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                                target: data::graph::BlockId(3),
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
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 6..7,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 7..7,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 7..8,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "bits_with_boolean_guard", data::source::SourceSpan::new(2838, 2848)),
                                            pattern_span: data::source::SourceSpan::new(2849, 2867),
                                        }),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(5),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(5),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
                                        })),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
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
                                        instructions: 0..2,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::Equal {
                                                left: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                right: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Bool,
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
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[]),
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 3..5,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 5..5,
                                        instructions: 3..4,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Equal {
                                            left: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            right: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::NotEqual {
                                            left: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            right: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::NotEqual {
                                            left: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            right: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(false)),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::BoolLocalId(2)),
                                data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
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
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::StringStartsWith {
                                                value: data::graph::StringLocalId(0),
                                                prefix: data::Text::Static(""),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(4),
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
                                        params: 1..2,
                                        instructions: 0..3,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::Equal {
                                                left: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                right: data::graph::ParamLocal::String(data::graph::StringLocalId(3)),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
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
                                                target: data::graph::BlockId(3),
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
                                        params: 2..4,
                                        instructions: 3..4,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..4,
                                        instructions: 4..5,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..4,
                                        instructions: 5..6,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
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
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static(""))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::DropPrefix {
                                            value: data::graph::StringLocalId(0),
                                            prefix: data::Text::Static(""),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(3)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static(""))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Equal {
                                            left: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            right: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(false)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(false)),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
                            ]),
                        },
                    },
                ]),
                nil_functions: data::Storage::Static(&[]),
                tuple_functions: data::Storage::Static(&[
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
                                        instructions: 0..7,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
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
                                            function: data::function::IntFunctionId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "caller", data::source::SourceSpan::new(1811, 1829)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(1),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            shape: data::type_::ValueShapeId(1),
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
                                            data::graph::IntLocalId(2),
                                            data::graph::IntLocalId(3),
                                        ])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                            function: data::function::BoolFunctionId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "caller", data::source::SourceSpan::new(1856, 1878)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::String,
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                    data::type_::ValueType::Bool,
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                        ]))),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::TupleLocalId(0)),
                            ]),
                        },
                    },
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

            enum CompiledResume {
                Next(usize),
                Exit(data::compiled::CompiledProgress),
            }

            fn string_int_0(
                point: usize,
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                    9
                ] = [
                    |values, budget| CompiledResume::Exit(string_int_0_entry((values.ints[0], values.strings[0],), values, budget)),
                    string_int_0_resume_1,
                    string_int_0_resume_2,
                    string_int_0_resume_3,
                    string_int_0_resume_4,
                    string_int_0_resume_5,
                    string_int_0_resume_6,
                    string_int_0_resume_7,
                    string_int_0_resume_8,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn string_int_0_entry(
                inputs: (i128, data::compiled::string::StringRange,),
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (mut b0_i0, mut b0_s0,) = inputs;
                'repeat: loop {
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b0_s0]);
                        return data::compiled::CompiledProgress::Yield(0);
                    }
                    *budget -= 1;
                    if values.text(b0_s0).starts_with("λ") {
                        let (b1_i0, b1_s0,) = (b0_i0, b0_s0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b1_s0]);
                            return data::compiled::CompiledProgress::Yield(1);
                        }
                        *budget -= 1;
                        let b1_s1 = b1_s0.drop_prefix(2);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                            return data::compiled::CompiledProgress::Yield(2);
                        }
                        *budget -= 1;
                        let b1_i1 = b1_i0 + 1_i128;
                        if b1_i1 < i128::from(i64::MIN) || b1_i1 > i128::from(i64::MAX) {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                            return data::compiled::CompiledProgress::Interpreted(3);
                        }
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                            return data::compiled::CompiledProgress::Yield(3);
                        }
                        *budget -= 1;
                        {
                            (b0_i0, b0_s0,) = (b1_i1, b1_s1,);
                            continue 'repeat;
                        }
                    } else {
                        let (b2_i0, b2_s0,) = (b0_i0, b0_s0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b2_s0]);
                            return data::compiled::CompiledProgress::Yield(4);
                        }
                        *budget -= 1;
                        let b2_s1 = data::compiled::string::StringRange::literal("");
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                            return data::compiled::CompiledProgress::Yield(5);
                        }
                        *budget -= 1;
                        if values.text(b2_s0) == values.text(b2_s1) {
                            let (b3_i0,) = (b2_i0,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(6);
                            }
                            *budget -= 1;

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b3_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0));
                        } else {
                            let () = ();
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(7);
                            }
                            *budget -= 1;
                            let b4_s0 = data::compiled::string::StringRange::literal("expected lambda prefix");
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b4_s0]);
                                return data::compiled::CompiledProgress::Yield(8);
                            }

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b4_s0]);
                            return data::compiled::CompiledProgress::Interpreted(8);
                        }
                    }
                }
            }

            fn string_int_0_resume_1(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_s0,) = (values.ints[0], values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                let b1_s1 = b1_s0.drop_prefix(2);

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                CompiledResume::Next(2)
            }

            fn string_int_0_resume_2(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_s0, b1_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                let b1_i1 = b1_i0 + 1_i128;
                if b1_i1 < i128::from(i64::MIN) || b1_i1 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(3));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                CompiledResume::Next(3)
            }

            fn string_int_0_resume_3(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_i1, b1_s0, b1_s1,) = (values.ints[0], values.ints[1], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                {
                    let (b0_i0, b0_s0,) = (b1_i1, b1_s1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0]);
                    CompiledResume::Next(0)
                }
            }

            fn string_int_0_resume_4(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_s0,) = (values.ints[0], values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;
                let b2_s1 = data::compiled::string::StringRange::literal("");

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                CompiledResume::Next(5)
            }

            fn string_int_0_resume_5(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_s0, b2_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;
                if values.text(b2_s0) == values.text(b2_s1) {
                    let (b3_i0,) = (b2_i0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(6)
                } else {
                    let () = ();

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(7)
                }
            }

            fn string_int_0_resume_6(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn string_int_0_resume_7(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                }
                *budget -= 1;
                let b4_s0 = data::compiled::string::StringRange::literal("expected lambda prefix");

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b4_s0]);
                CompiledResume::Next(8)
            }

            fn string_int_0_resume_8(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_s0,) = (values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b4_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b4_s0]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(8))
            }

            fn string_int_1(
                point: usize,
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                    9
                ] = [
                    |values, budget| CompiledResume::Exit(string_int_1_entry((values.strings[0],), values, budget)),
                    string_int_1_resume_1,
                    string_int_1_resume_2,
                    string_int_1_resume_3,
                    string_int_1_resume_4,
                    string_int_1_resume_5,
                    string_int_1_resume_6,
                    string_int_1_resume_7,
                    string_int_1_resume_8,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn string_int_1_entry(
                inputs: (data::compiled::string::StringRange,),
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (b0_s0,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0]);
                    return data::compiled::CompiledProgress::Yield(0);
                }
                *budget -= 1;
                if values.text(b0_s0) == "red" {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(1);
                    }
                    *budget -= 1;
                    let b1_i0 = 7_i128;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                } else if values.text(b0_s0) == "blue" {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(3);
                    }
                    *budget -= 1;
                    let b2_i0 = 9_i128;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(4);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                } else if values.text(b0_s0) == "\n\"\\λ" {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(5);
                    }
                    *budget -= 1;
                    let b3_i0 = 11_i128;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(6);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2))
                } else {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(7);
                    }
                    *budget -= 1;
                    let b4_i0 = -1_i128;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(8);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(3))
                }
            }

            fn string_int_1_resume_1(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                let b1_i0 = 7_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(2)
            }

            fn string_int_1_resume_2(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn string_int_1_resume_3(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                let b2_i0 = 9_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(4)
            }

            fn string_int_1_resume_4(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn string_int_1_resume_5(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;
                let b3_i0 = 11_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(6)
            }

            fn string_int_1_resume_6(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2)))
            }

            fn string_int_1_resume_7(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                }
                *budget -= 1;
                let b4_i0 = -1_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b4_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(8)
            }

            fn string_int_1_resume_8(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b4_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(3)))
            }

            fn string_int_2(
                point: usize,
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                    32
                ] = [
                    |values, budget| CompiledResume::Exit(string_int_2_entry((values.ints[0], values.bools[0], values.strings[0], values.strings[1],), values, budget)),
                    string_int_2_resume_1,
                    string_int_2_resume_2,
                    string_int_2_resume_3,
                    string_int_2_resume_4,
                    string_int_2_resume_5,
                    string_int_2_resume_6,
                    string_int_2_resume_7,
                    string_int_2_resume_8,
                    string_int_2_resume_9,
                    string_int_2_resume_10,
                    string_int_2_resume_11,
                    string_int_2_resume_12,
                    string_int_2_resume_13,
                    string_int_2_resume_14,
                    string_int_2_resume_15,
                    string_int_2_resume_16,
                    string_int_2_resume_17,
                    string_int_2_resume_18,
                    string_int_2_resume_19,
                    string_int_2_resume_20,
                    string_int_2_resume_21,
                    string_int_2_resume_22,
                    string_int_2_resume_23,
                    string_int_2_resume_24,
                    string_int_2_resume_25,
                    string_int_2_resume_26,
                    string_int_2_resume_27,
                    string_int_2_resume_28,
                    string_int_2_resume_29,
                    string_int_2_resume_30,
                    string_int_2_resume_31,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn string_int_2_entry(
                inputs: (i128, bool, data::compiled::string::StringRange, data::compiled::string::StringRange,),
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (b0_i0, b0_v0, b0_s0, b0_s1,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                    return data::compiled::CompiledProgress::Yield(0);
                }
                *budget -= 1;
                let (b11_i0, b11_s0, b11_s1,) = if values.text(b0_s0).starts_with("λ") {
                    let (b1_i0, b1_v0, b1_s0, b1_s1,) = (b0_i0, b0_v0, b0_s0, b0_s1,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                        return data::compiled::CompiledProgress::Yield(1);
                    }
                    *budget -= 1;
                    let b1_s2 = b1_s0.drop_prefix(2);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;
                    let (b10_i0, b10_s0, b10_s1,) = if values.text(b1_s2) == values.text(b1_s1) {
                        let (b2_i0, b2_v0, b2_s0, b2_s1,) = (b1_i0, b1_v0, b1_s0, b1_s1,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b2_v0]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                            return data::compiled::CompiledProgress::Yield(3);
                        }
                        *budget -= 1;
                        if b2_v0 {
                            let (b3_i0, b3_s0,) = (b2_i0, b2_s0,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b3_s0]);
                                return data::compiled::CompiledProgress::Yield(4);
                            }
                            *budget -= 1;
                            let b3_s1 = data::compiled::string::StringRange::literal("λ");
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b3_s0, b3_s1]);
                                return data::compiled::CompiledProgress::Yield(5);
                            }
                            *budget -= 1;
                            let b3_s2 = b3_s0.drop_prefix(2);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b3_s0, b3_s1, b3_s2]);
                                return data::compiled::CompiledProgress::Yield(6);
                            }
                            *budget -= 1;
                            if values.text(b3_s0) == values.text(b3_s0) {
                                let (b4_i0, b4_s0,) = (b3_i0, b3_s1,);
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b4_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);

                                    values.strings.clear();
                                    values.strings.extend_from_slice(&[b4_s0]);
                                    return data::compiled::CompiledProgress::Yield(7);
                                }
                                *budget -= 1;
                                let b4_s1 = data::compiled::string::StringRange::literal("λ");
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b4_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);

                                    values.strings.clear();
                                    values.strings.extend_from_slice(&[b4_s0, b4_s1]);
                                    return data::compiled::CompiledProgress::Yield(8);
                                }
                                *budget -= 1;
                                if values.text(b4_s0) == values.text(b4_s1) {
                                    let (b5_i0,) = (b4_i0,);
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b5_i0]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);

                                        values.strings.clear();
                                        values.strings.extend_from_slice(&[]);
                                        return data::compiled::CompiledProgress::Yield(9);
                                    }
                                    *budget -= 1;
                                    let b5_i1 = b5_i0 + 2_i128;
                                    if b5_i1 < i128::from(i64::MIN) || b5_i1 > i128::from(i64::MAX) {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);

                                        values.strings.clear();
                                        values.strings.extend_from_slice(&[]);
                                        return data::compiled::CompiledProgress::Interpreted(10);
                                    }
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);

                                        values.strings.clear();
                                        values.strings.extend_from_slice(&[]);
                                        return data::compiled::CompiledProgress::Yield(10);
                                    }
                                    *budget -= 1;

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);

                                    values.strings.clear();
                                    values.strings.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0));
                                } else {
                                    let () = ();
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);

                                        values.strings.clear();
                                        values.strings.extend_from_slice(&[]);
                                        return data::compiled::CompiledProgress::Yield(11);
                                    }
                                    *budget -= 1;
                                    {
                                    };
                                }
                            } else {
                                let () = ();
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);

                                    values.strings.clear();
                                    values.strings.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Yield(14);
                                }
                                *budget -= 1;
                                {
                                };
                            };
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(12);
                            }
                            *budget -= 1;
                            let b7_i0 = -100_i128;
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b7_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(13);
                            }
                            *budget -= 1;

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b7_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1));
                        } else {
                            let (b9_i0, b9_s0, b9_s1,) = (b2_i0, b2_s0, b2_s1,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b9_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b9_s0, b9_s1]);
                                return data::compiled::CompiledProgress::Yield(15);
                            }
                            *budget -= 1;
                            let (b10_i0, b10_s0, b10_s1,) = {
                                (b9_i0, b9_s0, b9_s1,)
                            };
                            (b10_i0, b10_s0, b10_s1,)
                        }
                    } else {
                        let (b19_i0, b19_s0, b19_s1,) = (b1_i0, b1_s0, b1_s1,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b19_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b19_s0, b19_s1]);
                            return data::compiled::CompiledProgress::Yield(30);
                        }
                        *budget -= 1;
                        let (b10_i0, b10_s0, b10_s1,) = {
                            (b19_i0, b19_s0, b19_s1,)
                        };
                        (b10_i0, b10_s0, b10_s1,)
                    };
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b10_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b10_s0, b10_s1]);
                        return data::compiled::CompiledProgress::Yield(16);
                    }
                    *budget -= 1;
                    let (b11_i0, b11_s0, b11_s1,) = {
                        (b10_i0, b10_s0, b10_s1,)
                    };
                    (b11_i0, b11_s0, b11_s1,)
                } else {
                    let (b20_i0, b20_s0, b20_s1,) = (b0_i0, b0_s0, b0_s1,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b20_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b20_s0, b20_s1]);
                        return data::compiled::CompiledProgress::Yield(31);
                    }
                    *budget -= 1;
                    let (b11_i0, b11_s0, b11_s1,) = {
                        (b20_i0, b20_s0, b20_s1,)
                    };
                    (b11_i0, b11_s0, b11_s1,)
                };
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b11_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b11_s0, b11_s1]);
                    return data::compiled::CompiledProgress::Yield(17);
                }
                *budget -= 1;
                let (b15_i0, b15_s0,) = if values.text(b11_s0).starts_with("λ") {
                    let (b12_i0, b12_s0, b12_s1,) = (b11_i0, b11_s0, b11_s1,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b12_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b12_s0, b12_s1]);
                        return data::compiled::CompiledProgress::Yield(18);
                    }
                    *budget -= 1;
                    let b12_s2 = b12_s0.drop_prefix(2);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b12_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b12_s0, b12_s1, b12_s2]);
                        return data::compiled::CompiledProgress::Yield(19);
                    }
                    *budget -= 1;
                    if values.text(b12_s2) == values.text(b12_s1) {
                        let (b13_i0, b13_s0,) = (b12_i0, b12_s0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b13_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b13_s0]);
                            return data::compiled::CompiledProgress::Yield(20);
                        }
                        *budget -= 1;
                        let b13_s1 = b13_s0.drop_prefix(2);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b13_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b13_s0, b13_s1]);
                            return data::compiled::CompiledProgress::Yield(21);
                        }
                        *budget -= 1;
                        let b13_i1 = b13_i0 + 4_i128;
                        if b13_i1 < i128::from(i64::MIN) || b13_i1 > i128::from(i64::MAX) {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b13_i0, b13_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b13_s0, b13_s1]);
                            return data::compiled::CompiledProgress::Interpreted(22);
                        }
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b13_i0, b13_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b13_s0, b13_s1]);
                            return data::compiled::CompiledProgress::Yield(22);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b13_i0, b13_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b13_s0, b13_s1]);
                        return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2));
                    } else {
                        let (b14_i0, b14_s0,) = (b12_i0, b12_s0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b14_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b14_s0]);
                            return data::compiled::CompiledProgress::Yield(23);
                        }
                        *budget -= 1;
                        let (b15_i0, b15_s0,) = {
                            (b14_i0, b14_s0,)
                        };
                        (b15_i0, b15_s0,)
                    }
                } else {
                    let (b18_i0, b18_s0,) = (b11_i0, b11_s0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b18_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b18_s0]);
                        return data::compiled::CompiledProgress::Yield(29);
                    }
                    *budget -= 1;
                    let (b15_i0, b15_s0,) = {
                        (b18_i0, b18_s0,)
                    };
                    (b15_i0, b15_s0,)
                };
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b15_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b15_s0]);
                    return data::compiled::CompiledProgress::Yield(24);
                }
                *budget -= 1;
                if values.text(b15_s0).starts_with("λ") {
                    let (b16_i0,) = (b15_i0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b16_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(25);
                    }
                    *budget -= 1;
                    let b16_i1 = b16_i0 + 8_i128;
                    if b16_i1 < i128::from(i64::MIN) || b16_i1 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b16_i0, b16_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Interpreted(26);
                    }
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b16_i0, b16_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(26);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b16_i0, b16_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(3))
                } else {
                    let (b17_i0,) = (b15_i0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b17_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(27);
                    }
                    *budget -= 1;
                    let b17_i1 = b17_i0 + 16_i128;
                    if b17_i1 < i128::from(i64::MIN) || b17_i1 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b17_i0, b17_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Interpreted(28);
                    }
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b17_i0, b17_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(28);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b17_i0, b17_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(4))
                }
            }

            fn string_int_2_resume_1(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_v0, b1_s0, b1_s1,) = (values.ints[0], values.bools[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                let b1_s2 = b1_s0.drop_prefix(2);

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b1_v0]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                CompiledResume::Next(2)
            }

            fn string_int_2_resume_2(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_v0, b1_s0, b1_s1, b1_s2,) = (values.ints[0], values.bools[0], values.strings[0], values.strings[1], values.strings[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                if values.text(b1_s2) == values.text(b1_s1) {
                    let (b2_i0, b2_v0, b2_s0, b2_s1,) = (b1_i0, b1_v0, b1_s0, b1_s1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b2_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                    CompiledResume::Next(3)
                } else {
                    let (b19_i0, b19_s0, b19_s1,) = (b1_i0, b1_s0, b1_s1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b19_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b19_s0, b19_s1]);
                    CompiledResume::Next(30)
                }
            }

            fn string_int_2_resume_3(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_v0, b2_s0, b2_s1,) = (values.ints[0], values.bools[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b2_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                if b2_v0 {
                    let (b3_i0, b3_s0,) = (b2_i0, b2_s0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b3_s0]);
                    CompiledResume::Next(4)
                } else {
                    let (b9_i0, b9_s0, b9_s1,) = (b2_i0, b2_s0, b2_s1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b9_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b9_s0, b9_s1]);
                    CompiledResume::Next(15)
                }
            }

            fn string_int_2_resume_4(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0, b3_s0,) = (values.ints[0], values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b3_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;
                let b3_s1 = data::compiled::string::StringRange::literal("λ");

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b3_s0, b3_s1]);
                CompiledResume::Next(5)
            }

            fn string_int_2_resume_5(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0, b3_s0, b3_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b3_s0, b3_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;
                let b3_s2 = b3_s0.drop_prefix(2);

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b3_s0, b3_s1, b3_s2]);
                CompiledResume::Next(6)
            }

            fn string_int_2_resume_6(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0, b3_s0, b3_s1, b3_s2,) = (values.ints[0], values.strings[0], values.strings[1], values.strings[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b3_s0, b3_s1, b3_s2]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;
                if values.text(b3_s0) == values.text(b3_s0) {
                    let (b4_i0, b4_s0,) = (b3_i0, b3_s1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b4_s0]);
                    CompiledResume::Next(7)
                } else {
                    let () = ();

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(14)
                }
            }

            fn string_int_2_resume_7(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_i0, b4_s0,) = (values.ints[0], values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b4_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                }
                *budget -= 1;
                let b4_s1 = data::compiled::string::StringRange::literal("λ");

                values.ints.clear();
                values.ints.extend_from_slice(&[b4_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b4_s0, b4_s1]);
                CompiledResume::Next(8)
            }

            fn string_int_2_resume_8(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_i0, b4_s0, b4_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b4_s0, b4_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                }
                *budget -= 1;
                if values.text(b4_s0) == values.text(b4_s1) {
                    let (b5_i0,) = (b4_i0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(9)
                } else {
                    let () = ();

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(11)
                }
            }

            fn string_int_2_resume_9(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b5_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(9));
                }
                *budget -= 1;
                let b5_i1 = b5_i0 + 2_i128;
                if b5_i1 < i128::from(i64::MIN) || b5_i1 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(10));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(10)
            }

            fn string_int_2_resume_10(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b5_i0, b5_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(10));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn string_int_2_resume_11(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(11));
                }
                *budget -= 1;
                {
                    let () = ();

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(12)
                }
            }

            fn string_int_2_resume_12(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(12));
                }
                *budget -= 1;
                let b7_i0 = -100_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b7_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(13)
            }

            fn string_int_2_resume_13(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b7_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b7_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(13));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b7_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn string_int_2_resume_14(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(14));
                }
                *budget -= 1;
                {
                    let () = ();

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(12)
                }
            }

            fn string_int_2_resume_15(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b9_i0, b9_s0, b9_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b9_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b9_s0, b9_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(15));
                }
                *budget -= 1;
                {
                    let (b10_i0, b10_s0, b10_s1,) = (b9_i0, b9_s0, b9_s1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b10_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b10_s0, b10_s1]);
                    CompiledResume::Next(16)
                }
            }

            fn string_int_2_resume_16(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b10_i0, b10_s0, b10_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b10_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b10_s0, b10_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(16));
                }
                *budget -= 1;
                {
                    let (b11_i0, b11_s0, b11_s1,) = (b10_i0, b10_s0, b10_s1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b11_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b11_s0, b11_s1]);
                    CompiledResume::Next(17)
                }
            }

            fn string_int_2_resume_17(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b11_i0, b11_s0, b11_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b11_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b11_s0, b11_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(17));
                }
                *budget -= 1;
                if values.text(b11_s0).starts_with("λ") {
                    let (b12_i0, b12_s0, b12_s1,) = (b11_i0, b11_s0, b11_s1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b12_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b12_s0, b12_s1]);
                    CompiledResume::Next(18)
                } else {
                    let (b18_i0, b18_s0,) = (b11_i0, b11_s0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b18_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b18_s0]);
                    CompiledResume::Next(29)
                }
            }

            fn string_int_2_resume_18(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b12_i0, b12_s0, b12_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b12_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b12_s0, b12_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(18));
                }
                *budget -= 1;
                let b12_s2 = b12_s0.drop_prefix(2);

                values.ints.clear();
                values.ints.extend_from_slice(&[b12_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b12_s0, b12_s1, b12_s2]);
                CompiledResume::Next(19)
            }

            fn string_int_2_resume_19(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b12_i0, b12_s0, b12_s1, b12_s2,) = (values.ints[0], values.strings[0], values.strings[1], values.strings[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b12_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b12_s0, b12_s1, b12_s2]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(19));
                }
                *budget -= 1;
                if values.text(b12_s2) == values.text(b12_s1) {
                    let (b13_i0, b13_s0,) = (b12_i0, b12_s0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b13_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b13_s0]);
                    CompiledResume::Next(20)
                } else {
                    let (b14_i0, b14_s0,) = (b12_i0, b12_s0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b14_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b14_s0]);
                    CompiledResume::Next(23)
                }
            }

            fn string_int_2_resume_20(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b13_i0, b13_s0,) = (values.ints[0], values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b13_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b13_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(20));
                }
                *budget -= 1;
                let b13_s1 = b13_s0.drop_prefix(2);

                values.ints.clear();
                values.ints.extend_from_slice(&[b13_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b13_s0, b13_s1]);
                CompiledResume::Next(21)
            }

            fn string_int_2_resume_21(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b13_i0, b13_s0, b13_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b13_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b13_s0, b13_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(21));
                }
                *budget -= 1;
                let b13_i1 = b13_i0 + 4_i128;
                if b13_i1 < i128::from(i64::MIN) || b13_i1 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b13_i0, b13_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b13_s0, b13_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(22));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b13_i0, b13_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b13_s0, b13_s1]);
                CompiledResume::Next(22)
            }

            fn string_int_2_resume_22(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b13_i0, b13_i1, b13_s0, b13_s1,) = (values.ints[0], values.ints[1], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b13_i0, b13_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b13_s0, b13_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(22));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b13_i0, b13_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b13_s0, b13_s1]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2)))
            }

            fn string_int_2_resume_23(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b14_i0, b14_s0,) = (values.ints[0], values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b14_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b14_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(23));
                }
                *budget -= 1;
                {
                    let (b15_i0, b15_s0,) = (b14_i0, b14_s0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b15_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b15_s0]);
                    CompiledResume::Next(24)
                }
            }

            fn string_int_2_resume_24(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b15_i0, b15_s0,) = (values.ints[0], values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b15_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b15_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(24));
                }
                *budget -= 1;
                if values.text(b15_s0).starts_with("λ") {
                    let (b16_i0,) = (b15_i0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b16_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(25)
                } else {
                    let (b17_i0,) = (b15_i0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b17_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(27)
                }
            }

            fn string_int_2_resume_25(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b16_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b16_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(25));
                }
                *budget -= 1;
                let b16_i1 = b16_i0 + 8_i128;
                if b16_i1 < i128::from(i64::MIN) || b16_i1 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b16_i0, b16_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(26));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b16_i0, b16_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(26)
            }

            fn string_int_2_resume_26(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b16_i0, b16_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b16_i0, b16_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(26));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b16_i0, b16_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(3)))
            }

            fn string_int_2_resume_27(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b17_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b17_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(27));
                }
                *budget -= 1;
                let b17_i1 = b17_i0 + 16_i128;
                if b17_i1 < i128::from(i64::MIN) || b17_i1 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b17_i0, b17_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(28));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b17_i0, b17_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(28)
            }

            fn string_int_2_resume_28(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b17_i0, b17_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b17_i0, b17_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(28));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b17_i0, b17_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(4)))
            }

            fn string_int_2_resume_29(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b18_i0, b18_s0,) = (values.ints[0], values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b18_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b18_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(29));
                }
                *budget -= 1;
                {
                    let (b15_i0, b15_s0,) = (b18_i0, b18_s0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b15_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b15_s0]);
                    CompiledResume::Next(24)
                }
            }

            fn string_int_2_resume_30(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b19_i0, b19_s0, b19_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b19_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b19_s0, b19_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(30));
                }
                *budget -= 1;
                {
                    let (b10_i0, b10_s0, b10_s1,) = (b19_i0, b19_s0, b19_s1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b10_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b10_s0, b10_s1]);
                    CompiledResume::Next(16)
                }
            }

            fn string_int_2_resume_31(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b20_i0, b20_s0, b20_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b20_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b20_s0, b20_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(31));
                }
                *budget -= 1;
                {
                    let (b11_i0, b11_s0, b11_s1,) = (b20_i0, b20_s0, b20_s1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b11_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b11_s0, b11_s1]);
                    CompiledResume::Next(17)
                }
            }

            fn string_int_3(
                point: usize,
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                    14
                ] = [
                    |values, budget| CompiledResume::Exit(string_int_3_entry((values.ints[0], values.strings[0], values.strings[1],), values, budget)),
                    string_int_3_resume_1,
                    string_int_3_resume_2,
                    string_int_3_resume_3,
                    string_int_3_resume_4,
                    string_int_3_resume_5,
                    string_int_3_resume_6,
                    string_int_3_resume_7,
                    string_int_3_resume_8,
                    string_int_3_resume_9,
                    string_int_3_resume_10,
                    string_int_3_resume_11,
                    string_int_3_resume_12,
                    string_int_3_resume_13,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn string_int_3_entry(
                inputs: (i128, data::compiled::string::StringRange, data::compiled::string::StringRange,),
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (mut b0_i0, mut b0_s0, mut b0_s1,) = inputs;
                'repeat: loop {
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                        return data::compiled::CompiledProgress::Yield(0);
                    }
                    *budget -= 1;
                    if values.text(b0_s0).starts_with("λ") {
                        let (b1_i0, b1_s0, b1_s1,) = (b0_i0, b0_s0, b0_s1,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                            return data::compiled::CompiledProgress::Yield(1);
                        }
                        *budget -= 1;
                        let b1_s2 = b1_s0.drop_prefix(2);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                            return data::compiled::CompiledProgress::Yield(2);
                        }
                        *budget -= 1;
                        let b1_i1 = b1_i0 + 1_i128;
                        if b1_i1 < i128::from(i64::MIN) || b1_i1 > i128::from(i64::MAX) {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                            return data::compiled::CompiledProgress::Interpreted(3);
                        }
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                            return data::compiled::CompiledProgress::Yield(3);
                        }
                        *budget -= 1;
                        {
                            (b0_i0, b0_s0, b0_s1,) = (b1_i1, b1_s1, b1_s2,);
                            continue 'repeat;
                        }
                    } else {
                        let (b2_i0, b2_s0, b2_s1,) = (b0_i0, b0_s0, b0_s1,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                            return data::compiled::CompiledProgress::Yield(4);
                        }
                        *budget -= 1;
                        if values.text(b2_s0).starts_with("m") {
                            let (b3_i0, b3_s0, b3_s1,) = (b2_i0, b2_s0, b2_s1,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b3_s0, b3_s1]);
                                return data::compiled::CompiledProgress::Yield(5);
                            }
                            *budget -= 1;
                            let b3_s2 = b3_s0.drop_prefix(1);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b3_s0, b3_s1, b3_s2]);
                                return data::compiled::CompiledProgress::Yield(6);
                            }
                            *budget -= 1;
                            let b3_i1 = b3_i0 + 1_i128;
                            if b3_i1 < i128::from(i64::MIN) || b3_i1 > i128::from(i64::MAX) {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b3_s0, b3_s1, b3_s2]);
                                return data::compiled::CompiledProgress::Interpreted(7);
                            }
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b3_s0, b3_s1, b3_s2]);
                                return data::compiled::CompiledProgress::Yield(7);
                            }
                            *budget -= 1;
                            {
                                (b0_i0, b0_s0, b0_s1,) = (b3_i1, b3_s1, b3_s2,);
                                continue 'repeat;
                            }
                        } else {
                            let (b4_i0, b4_s0, b4_s1,) = (b2_i0, b2_s0, b2_s1,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b4_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b4_s0, b4_s1]);
                                return data::compiled::CompiledProgress::Yield(8);
                            }
                            *budget -= 1;
                            let b4_s2 = data::compiled::string::StringRange::literal("");
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b4_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b4_s0, b4_s1, b4_s2]);
                                return data::compiled::CompiledProgress::Yield(9);
                            }
                            *budget -= 1;
                            if values.text(b4_s0) == values.text(b4_s2) {
                                let (b5_i0, b5_s0, b5_s1,) = (b4_i0, b4_s0, b4_s1,);
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b5_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);

                                    values.strings.clear();
                                    values.strings.extend_from_slice(&[b5_s0, b5_s1]);
                                    return data::compiled::CompiledProgress::Yield(10);
                                }
                                *budget -= 1;
                                if values.text(b5_s1).is_empty() {
                                    let (b6_i0,) = (b5_i0,);
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b6_i0]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);

                                        values.strings.clear();
                                        values.strings.extend_from_slice(&[]);
                                        return data::compiled::CompiledProgress::Yield(11);
                                    }
                                    *budget -= 1;

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b6_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);

                                    values.strings.clear();
                                    values.strings.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0));
                                } else {
                                    let (b7_i0, b7_s0, b7_s1,) = (b5_i0, b5_s0, b5_s1,);
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b7_i0]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);

                                        values.strings.clear();
                                        values.strings.extend_from_slice(&[b7_s0, b7_s1]);
                                        return data::compiled::CompiledProgress::Yield(12);
                                    }
                                    *budget -= 1;
                                    {
                                        (b0_i0, b0_s0, b0_s1,) = (b7_i0, b7_s1, b7_s0,);
                                        continue 'repeat;
                                    }
                                }
                            } else {
                                let (b8_i0,) = (b4_i0,);
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b8_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);

                                    values.strings.clear();
                                    values.strings.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Yield(13);
                                }
                                *budget -= 1;

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b8_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1));
                            }
                        }
                    }
                }
            }

            fn string_int_3_resume_1(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_s0, b1_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                let b1_s2 = b1_s0.drop_prefix(2);

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                CompiledResume::Next(2)
            }

            fn string_int_3_resume_2(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_s0, b1_s1, b1_s2,) = (values.ints[0], values.strings[0], values.strings[1], values.strings[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                let b1_i1 = b1_i0 + 1_i128;
                if b1_i1 < i128::from(i64::MIN) || b1_i1 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(3));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                CompiledResume::Next(3)
            }

            fn string_int_3_resume_3(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_i1, b1_s0, b1_s1, b1_s2,) = (values.ints[0], values.ints[1], values.strings[0], values.strings[1], values.strings[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                {
                    let (b0_i0, b0_s0, b0_s1,) = (b1_i1, b1_s1, b1_s2,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                    CompiledResume::Next(0)
                }
            }

            fn string_int_3_resume_4(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_s0, b2_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;
                if values.text(b2_s0).starts_with("m") {
                    let (b3_i0, b3_s0, b3_s1,) = (b2_i0, b2_s0, b2_s1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b3_s0, b3_s1]);
                    CompiledResume::Next(5)
                } else {
                    let (b4_i0, b4_s0, b4_s1,) = (b2_i0, b2_s0, b2_s1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b4_s0, b4_s1]);
                    CompiledResume::Next(8)
                }
            }

            fn string_int_3_resume_5(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0, b3_s0, b3_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b3_s0, b3_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;
                let b3_s2 = b3_s0.drop_prefix(1);

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b3_s0, b3_s1, b3_s2]);
                CompiledResume::Next(6)
            }

            fn string_int_3_resume_6(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0, b3_s0, b3_s1, b3_s2,) = (values.ints[0], values.strings[0], values.strings[1], values.strings[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b3_s0, b3_s1, b3_s2]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;
                let b3_i1 = b3_i0 + 1_i128;
                if b3_i1 < i128::from(i64::MIN) || b3_i1 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b3_s0, b3_s1, b3_s2]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(7));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b3_s0, b3_s1, b3_s2]);
                CompiledResume::Next(7)
            }

            fn string_int_3_resume_7(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0, b3_i1, b3_s0, b3_s1, b3_s2,) = (values.ints[0], values.ints[1], values.strings[0], values.strings[1], values.strings[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b3_s0, b3_s1, b3_s2]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                }
                *budget -= 1;
                {
                    let (b0_i0, b0_s0, b0_s1,) = (b3_i1, b3_s1, b3_s2,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                    CompiledResume::Next(0)
                }
            }

            fn string_int_3_resume_8(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_i0, b4_s0, b4_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b4_s0, b4_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                }
                *budget -= 1;
                let b4_s2 = data::compiled::string::StringRange::literal("");

                values.ints.clear();
                values.ints.extend_from_slice(&[b4_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b4_s0, b4_s1, b4_s2]);
                CompiledResume::Next(9)
            }

            fn string_int_3_resume_9(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_i0, b4_s0, b4_s1, b4_s2,) = (values.ints[0], values.strings[0], values.strings[1], values.strings[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b4_s0, b4_s1, b4_s2]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(9));
                }
                *budget -= 1;
                if values.text(b4_s0) == values.text(b4_s2) {
                    let (b5_i0, b5_s0, b5_s1,) = (b4_i0, b4_s0, b4_s1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b5_s0, b5_s1]);
                    CompiledResume::Next(10)
                } else {
                    let (b8_i0,) = (b4_i0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b8_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(13)
                }
            }

            fn string_int_3_resume_10(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b5_i0, b5_s0, b5_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b5_s0, b5_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(10));
                }
                *budget -= 1;
                if values.text(b5_s1).is_empty() {
                    let (b6_i0,) = (b5_i0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b6_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(11)
                } else {
                    let (b7_i0, b7_s0, b7_s1,) = (b5_i0, b5_s0, b5_s1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b7_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b7_s0, b7_s1]);
                    CompiledResume::Next(12)
                }
            }

            fn string_int_3_resume_11(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b6_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b6_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(11));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b6_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn string_int_3_resume_12(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b7_i0, b7_s0, b7_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b7_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b7_s0, b7_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(12));
                }
                *budget -= 1;
                {
                    let (b0_i0, b0_s0, b0_s1,) = (b7_i0, b7_s1, b7_s0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                    CompiledResume::Next(0)
                }
            }

            fn string_int_3_resume_13(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b8_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b8_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(13));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b8_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn string_int_4(
                point: usize,
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                    12
                ] = [
                    |values, budget| CompiledResume::Exit(string_int_4_entry((), values, budget)),
                    string_int_4_resume_1,
                    string_int_4_resume_2,
                    string_int_4_resume_3,
                    string_int_4_resume_4,
                    string_int_4_resume_5,
                    string_int_4_resume_6,
                    string_int_4_resume_7,
                    string_int_4_resume_8,
                    string_int_4_resume_9,
                    string_int_4_resume_10,
                    string_int_4_resume_11,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn string_int_4_entry(
                inputs: (),
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let () = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return data::compiled::CompiledProgress::Yield(0);
                }
                *budget -= 1;
                let b0_s0 = data::compiled::string::StringRange::literal("λtail");
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0]);
                    return data::compiled::CompiledProgress::Yield(1);
                }
                *budget -= 1;
                if values.text(b0_s0).starts_with("λ") {
                    let (b1_s0,) = (b0_s0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0]);
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;
                    let b1_s1 = b1_s0.drop_prefix(2);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                        return data::compiled::CompiledProgress::Yield(3);
                    }
                    *budget -= 1;
                    let b1_s2 = data::compiled::string::StringRange::literal("tail");
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                        return data::compiled::CompiledProgress::Yield(4);
                    }
                    *budget -= 1;
                    if values.text(b1_s1) == values.text(b1_s2) {
                        let (b2_s0,) = (b1_s0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b2_s0]);
                            return data::compiled::CompiledProgress::Yield(5);
                        }
                        *budget -= 1;
                        let b2_s1 = b2_s0.drop_prefix(2);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                            return data::compiled::CompiledProgress::Yield(6);
                        }
                        *budget -= 1;
                        let b2_i0 = 7_i128;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                            return data::compiled::CompiledProgress::Yield(7);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                        return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0));
                    } else {
                        let () = ();
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(8);
                        }
                        *budget -= 1;
                        {
                        };
                    }
                } else {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(11);
                    }
                    *budget -= 1;
                    {
                    };
                };
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return data::compiled::CompiledProgress::Yield(9);
                }
                *budget -= 1;
                let b4_i0 = -1_i128;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return data::compiled::CompiledProgress::Yield(10);
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b4_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
            }

            fn string_int_4_resume_1(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b0_s0,) = (values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                if values.text(b0_s0).starts_with("λ") {
                    let (b1_s0,) = (b0_s0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0]);
                    CompiledResume::Next(2)
                } else {
                    let () = ();

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(11)
                }
            }

            fn string_int_4_resume_2(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_s0,) = (values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                let b1_s1 = b1_s0.drop_prefix(2);

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                CompiledResume::Next(3)
            }

            fn string_int_4_resume_3(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_s0, b1_s1,) = (values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                let b1_s2 = data::compiled::string::StringRange::literal("tail");

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                CompiledResume::Next(4)
            }

            fn string_int_4_resume_4(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_s0, b1_s1, b1_s2,) = (values.strings[0], values.strings[1], values.strings[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;
                if values.text(b1_s1) == values.text(b1_s2) {
                    let (b2_s0,) = (b1_s0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0]);
                    CompiledResume::Next(5)
                } else {
                    let () = ();

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(8)
                }
            }

            fn string_int_4_resume_5(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_s0,) = (values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;
                let b2_s1 = b2_s0.drop_prefix(2);

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                CompiledResume::Next(6)
            }

            fn string_int_4_resume_6(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_s0, b2_s1,) = (values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;
                let b2_i0 = 7_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                CompiledResume::Next(7)
            }

            fn string_int_4_resume_7(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_s0, b2_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn string_int_4_resume_8(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                }
                *budget -= 1;
                {
                    let () = ();

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(9)
                }
            }

            fn string_int_4_resume_9(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(9));
                }
                *budget -= 1;
                let b4_i0 = -1_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b4_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(10)
            }

            fn string_int_4_resume_10(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(10));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b4_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn string_int_4_resume_11(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(11));
                }
                *budget -= 1;
                {
                    let () = ();

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(9)
                }
            }

            fn string_int_5(
                point: usize,
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                    12
                ] = [
                    |values, budget| CompiledResume::Exit(string_int_5_entry((values.ints[0], values.strings[0],), values, budget)),
                    string_int_5_resume_1,
                    string_int_5_resume_2,
                    string_int_5_resume_3,
                    string_int_5_resume_4,
                    string_int_5_resume_5,
                    string_int_5_resume_6,
                    string_int_5_resume_7,
                    string_int_5_resume_8,
                    string_int_5_resume_9,
                    string_int_5_resume_10,
                    string_int_5_resume_11,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn string_int_5_entry(
                inputs: (i128, data::compiled::string::StringRange,),
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (b0_i0, b0_s0,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0]);
                    return data::compiled::CompiledProgress::Yield(0);
                }
                *budget -= 1;
                if values.text(b0_s0).starts_with("λ") {
                    let m2 = b0_s0;
                    let m0 = data::compiled::string::StringRange::literal("λ");
                    let m1 = b0_s0.drop_prefix(2);
                    let (b1_i0, b1_s0, b1_s1, b1_s2,) = (b0_i0, m0, m1, m2,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                        return data::compiled::CompiledProgress::Yield(1);
                    }
                    *budget -= 1;
                    if values.text(b1_s1) != values.text(b1_s2) {
                        let (b2_i0, b2_s0,) = (b1_i0, b1_s0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b2_s0]);
                            return data::compiled::CompiledProgress::Yield(2);
                        }
                        *budget -= 1;
                        let b2_s1 = data::compiled::string::StringRange::literal("λ");
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                            return data::compiled::CompiledProgress::Yield(3);
                        }
                        *budget -= 1;
                        if values.text(b2_s0) == values.text(b2_s1) {
                            let (b3_i0,) = (b2_i0,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(4);
                            }
                            *budget -= 1;
                            let b3_i1 = b3_i0 + 1_i128;
                            if b3_i1 < i128::from(i64::MIN) || b3_i1 > i128::from(i64::MAX) {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Interpreted(5);
                            }
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(5);
                            }
                            *budget -= 1;

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                        } else {
                            let () = ();
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(6);
                            }
                            *budget -= 1;
                            let b4_i0 = -1_i128;
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b4_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(7);
                            }
                            *budget -= 1;

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b4_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                        }
                    } else {
                        let () = ();
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(8);
                        }
                        *budget -= 1;
                        let b5_i0 = -1_i128;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b5_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(9);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b5_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2))
                    }
                } else {
                    let (b6_s0,) = (b0_s0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b6_s0]);
                        return data::compiled::CompiledProgress::Yield(10);
                    }
                    *budget -= 1;
                    let b6_s1 = data::compiled::string::StringRange::literal("lambda required");
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b6_s0, b6_s1]);
                        return data::compiled::CompiledProgress::Yield(11);
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b6_s0, b6_s1]);
                    data::compiled::CompiledProgress::Interpreted(11)
                }
            }

            fn string_int_5_resume_1(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_s0, b1_s1, b1_s2,) = (values.ints[0], values.strings[0], values.strings[1], values.strings[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                if values.text(b1_s1) != values.text(b1_s2) {
                    let (b2_i0, b2_s0,) = (b1_i0, b1_s0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0]);
                    CompiledResume::Next(2)
                } else {
                    let () = ();

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(8)
                }
            }

            fn string_int_5_resume_2(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_s0,) = (values.ints[0], values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                let b2_s1 = data::compiled::string::StringRange::literal("λ");

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                CompiledResume::Next(3)
            }

            fn string_int_5_resume_3(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_s0, b2_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                if values.text(b2_s0) == values.text(b2_s1) {
                    let (b3_i0,) = (b2_i0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(4)
                } else {
                    let () = ();

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(6)
                }
            }

            fn string_int_5_resume_4(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;
                let b3_i1 = b3_i0 + 1_i128;
                if b3_i1 < i128::from(i64::MIN) || b3_i1 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(5));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(5)
            }

            fn string_int_5_resume_5(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0, b3_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn string_int_5_resume_6(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;
                let b4_i0 = -1_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b4_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(7)
            }

            fn string_int_5_resume_7(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b4_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn string_int_5_resume_8(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                }
                *budget -= 1;
                let b5_i0 = -1_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b5_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(9)
            }

            fn string_int_5_resume_9(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b5_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(9));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b5_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2)))
            }

            fn string_int_5_resume_10(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b6_s0,) = (values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b6_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(10));
                }
                *budget -= 1;
                let b6_s1 = data::compiled::string::StringRange::literal("lambda required");

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b6_s0, b6_s1]);
                CompiledResume::Next(11)
            }

            fn string_int_5_resume_11(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b6_s0, b6_s1,) = (values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b6_s0, b6_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(11));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b6_s0, b6_s1]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(11))
            }

            fn string_int_7(
                point: usize,
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                    4
                ] = [
                    |values, budget| CompiledResume::Exit(string_int_7_entry((values.strings[0],), values, budget)),
                    string_int_7_resume_1,
                    string_int_7_resume_2,
                    string_int_7_resume_3,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn string_int_7_entry(
                inputs: (data::compiled::string::StringRange,),
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (mut b0_s0,) = inputs;
                'repeat: loop {
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b0_s0]);
                        return data::compiled::CompiledProgress::Yield(0);
                    }
                    *budget -= 1;
                    if values.text(b0_s0).starts_with("λ") {
                        let (b1_s0,) = (b0_s0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b1_s0]);
                            return data::compiled::CompiledProgress::Yield(1);
                        }
                        *budget -= 1;
                        {
                            (b0_s0,) = (b1_s0,);
                            continue 'repeat;
                        }
                    } else {
                        let () = ();
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(2);
                        }
                        *budget -= 1;
                        let b2_i0 = 0_i128;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(3);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0));
                    }
                }
            }

            fn string_int_7_resume_1(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_s0,) = (values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                {
                    let (b0_s0,) = (b1_s0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0]);
                    CompiledResume::Next(0)
                }
            }

            fn string_int_7_resume_2(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                let b2_i0 = 0_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(3)
            }

            fn string_int_7_resume_3(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn string_int_8(
                point: usize,
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                    3
                ] = [
                    |values, budget| CompiledResume::Exit(string_int_8_entry((), values, budget)),
                    string_int_8_resume_1,
                    string_int_8_resume_2,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn string_int_8_entry(
                inputs: (),
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let () = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return data::compiled::CompiledProgress::Yield(0);
                }
                *budget -= 1;
                let b0_s0 = data::compiled::string::StringRange::literal("λλλ");
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0]);
                    return data::compiled::CompiledProgress::Yield(1);
                }
                *budget -= 1;
                let b0_i0 = 4_i128;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0]);
                    return data::compiled::CompiledProgress::Yield(2);
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b0_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b0_s0]);
                data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
            }

            fn string_int_8_resume_1(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b0_s0,) = (values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                let b0_i0 = 4_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b0_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b0_s0]);
                CompiledResume::Next(2)
            }

            fn string_int_8_resume_2(
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
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b0_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b0_s0]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn string_int_9(
                point: usize,
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                    8
                ] = [
                    |values, budget| CompiledResume::Exit(string_int_9_entry((values.strings[0],), values, budget)),
                    string_int_9_resume_1,
                    string_int_9_resume_2,
                    string_int_9_resume_3,
                    string_int_9_resume_4,
                    string_int_9_resume_5,
                    string_int_9_resume_6,
                    string_int_9_resume_7,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn string_int_9_entry(
                inputs: (data::compiled::string::StringRange,),
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (b0_s0,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0]);
                    return data::compiled::CompiledProgress::Yield(0);
                }
                *budget -= 1;
                if values.text(b0_s0) == "\n\"\\λ" {
                    let m0 = b0_s0;
                    let (b1_s0,) = (m0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0]);
                        return data::compiled::CompiledProgress::Yield(1);
                    }
                    *budget -= 1;
                    if values.text(b1_s0) == "\n\"\\λ" {
                        let () = ();
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(2);
                        }
                        *budget -= 1;
                        let b2_i0 = 17_i128;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(3);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                    } else {
                        let () = ();
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(4);
                        }
                        *budget -= 1;
                        let b3_i0 = -1_i128;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b3_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(5);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                    }
                } else {
                    let (b4_s0,) = (b0_s0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b4_s0]);
                        return data::compiled::CompiledProgress::Yield(6);
                    }
                    *budget -= 1;
                    let b4_s1 = data::compiled::string::StringRange::literal("literal required");
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b4_s0, b4_s1]);
                        return data::compiled::CompiledProgress::Yield(7);
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b4_s0, b4_s1]);
                    data::compiled::CompiledProgress::Interpreted(7)
                }
            }

            fn string_int_9_resume_1(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_s0,) = (values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                if values.text(b1_s0) == "\n\"\\λ" {
                    let () = ();

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(2)
                } else {
                    let () = ();

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(4)
                }
            }

            fn string_int_9_resume_2(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                let b2_i0 = 17_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(3)
            }

            fn string_int_9_resume_3(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn string_int_9_resume_4(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;
                let b3_i0 = -1_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(5)
            }

            fn string_int_9_resume_5(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn string_int_9_resume_6(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_s0,) = (values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b4_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;
                let b4_s1 = data::compiled::string::StringRange::literal("literal required");

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b4_s0, b4_s1]);
                CompiledResume::Next(7)
            }

            fn string_int_9_resume_7(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_s0, b4_s1,) = (values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b4_s0, b4_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b4_s0, b4_s1]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(7))
            }

            fn string_int_10(
                point: usize,
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                    5
                ] = [
                    |values, budget| CompiledResume::Exit(string_int_10_entry((values.strings[0],), values, budget)),
                    string_int_10_resume_1,
                    string_int_10_resume_2,
                    string_int_10_resume_3,
                    string_int_10_resume_4,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn string_int_10_entry(
                inputs: (data::compiled::string::StringRange,),
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (b0_s0,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0]);
                    return data::compiled::CompiledProgress::Yield(0);
                }
                *budget -= 1;
                if values.text(b0_s0).starts_with("λ") {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(1);
                    }
                    *budget -= 1;
                    let b1_i0 = 19_i128;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                } else {
                    let (b2_s0,) = (b0_s0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b2_s0]);
                        return data::compiled::CompiledProgress::Yield(3);
                    }
                    *budget -= 1;
                    let b2_s1 = data::compiled::string::StringRange::literal("prefix required");
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                        return data::compiled::CompiledProgress::Yield(4);
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                    data::compiled::CompiledProgress::Interpreted(4)
                }
            }

            fn string_int_10_resume_1(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                let b1_i0 = 19_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(2)
            }

            fn string_int_10_resume_2(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn string_int_10_resume_3(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_s0,) = (values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                let b2_s1 = data::compiled::string::StringRange::literal("prefix required");

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                CompiledResume::Next(4)
            }

            fn string_int_10_resume_4(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_s0, b2_s1,) = (values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(4))
            }

            fn string_int_11(
                point: usize,
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                    11
                ] = [
                    |values, budget| CompiledResume::Exit(string_int_11_entry((values.ints[0], values.strings[0],), values, budget)),
                    string_int_11_resume_1,
                    string_int_11_resume_2,
                    string_int_11_resume_3,
                    string_int_11_resume_4,
                    string_int_11_resume_5,
                    string_int_11_resume_6,
                    string_int_11_resume_7,
                    string_int_11_resume_8,
                    string_int_11_resume_9,
                    string_int_11_resume_10,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn string_int_11_entry(
                inputs: (i128, data::compiled::string::StringRange,),
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (b0_i0, b0_s0,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0]);
                    return data::compiled::CompiledProgress::Yield(0);
                }
                *budget -= 1;
                if values.text(b0_s0).starts_with("λ") {
                    let m1 = b0_s0.drop_prefix(2);
                    let (b1_i0, b1_s0,) = (b0_i0, m1,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0]);
                        return data::compiled::CompiledProgress::Yield(1);
                    }
                    *budget -= 1;
                    if b1_i0 != -1_i128 {
                        let (b2_i0, b2_s0,) = (b1_i0, b1_s0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b2_s0]);
                            return data::compiled::CompiledProgress::Yield(2);
                        }
                        *budget -= 1;
                        if values.text(b2_s0) == "tail" {
                            let (b3_i0,) = (b2_i0,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(3);
                            }
                            *budget -= 1;
                            let b3_i1 = b3_i0 + 23_i128;
                            if b3_i1 < i128::from(i64::MIN) || b3_i1 > i128::from(i64::MAX) {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Interpreted(4);
                            }
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(4);
                            }
                            *budget -= 1;

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                        } else {
                            let (b4_i0,) = (b2_i0,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b4_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(5);
                            }
                            *budget -= 1;
                            let b4_i1 = b4_i0 + 29_i128;
                            if b4_i1 < i128::from(i64::MIN) || b4_i1 > i128::from(i64::MAX) {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Interpreted(6);
                            }
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(6);
                            }
                            *budget -= 1;

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                        }
                    } else {
                        let () = ();
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(7);
                        }
                        *budget -= 1;
                        let b5_i0 = -1_i128;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b5_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(8);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b5_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2))
                    }
                } else {
                    let (b6_s0,) = (b0_s0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b6_s0]);
                        return data::compiled::CompiledProgress::Yield(9);
                    }
                    *budget -= 1;
                    let b6_s1 = data::compiled::string::StringRange::literal("suffix required");
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b6_s0, b6_s1]);
                        return data::compiled::CompiledProgress::Yield(10);
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b6_s0, b6_s1]);
                    data::compiled::CompiledProgress::Interpreted(10)
                }
            }

            fn string_int_11_resume_1(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_s0,) = (values.ints[0], values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                if b1_i0 != -1_i128 {
                    let (b2_i0, b2_s0,) = (b1_i0, b1_s0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0]);
                    CompiledResume::Next(2)
                } else {
                    let () = ();

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(7)
                }
            }

            fn string_int_11_resume_2(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_s0,) = (values.ints[0], values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                if values.text(b2_s0) == "tail" {
                    let (b3_i0,) = (b2_i0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(3)
                } else {
                    let (b4_i0,) = (b2_i0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(5)
                }
            }

            fn string_int_11_resume_3(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                let b3_i1 = b3_i0 + 23_i128;
                if b3_i1 < i128::from(i64::MIN) || b3_i1 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(4));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(4)
            }

            fn string_int_11_resume_4(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0, b3_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn string_int_11_resume_5(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;
                let b4_i1 = b4_i0 + 29_i128;
                if b4_i1 < i128::from(i64::MIN) || b4_i1 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(6));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(6)
            }

            fn string_int_11_resume_6(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_i0, b4_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn string_int_11_resume_7(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                }
                *budget -= 1;
                let b5_i0 = -1_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b5_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(8)
            }

            fn string_int_11_resume_8(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b5_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b5_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2)))
            }

            fn string_int_11_resume_9(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b6_s0,) = (values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b6_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(9));
                }
                *budget -= 1;
                let b6_s1 = data::compiled::string::StringRange::literal("suffix required");

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b6_s0, b6_s1]);
                CompiledResume::Next(10)
            }

            fn string_int_11_resume_10(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b6_s0, b6_s1,) = (values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b6_s0, b6_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(10));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b6_s0, b6_s1]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(10))
            }

            fn bit_array_int_12(
                point: usize,
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::bit_array::BitArrayValues, &mut usize) -> CompiledResume;
                    6
                ] = [
                    |values, budget| CompiledResume::Exit(bit_array_int_12_entry((values.bools[0], values.bools[1], values.bit_arrays[0],), values, budget)),
                    bit_array_int_12_resume_1,
                    bit_array_int_12_resume_2,
                    bit_array_int_12_resume_3,
                    bit_array_int_12_resume_4,
                    bit_array_int_12_resume_5,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn bit_array_int_12_entry(
                inputs: (bool, bool, data::compiled::bit_array::BitArrayRange,),
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (b0_v0, b0_v1, b0_b0,) = inputs;
                let (b1_i0, b1_v0, b1_v1,) = 'block_1: {
                    let (b4_b0,) = 'block_4: {
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b0_v0, b0_v1]);
                            values.bit_arrays.clear();
                            values.bit_arrays.extend_from_slice(&[b0_b0]);
                            return data::compiled::CompiledProgress::Yield(0);
                        }
                        *budget -= 1;
                        let _matched = (|| -> Option<(i128,)> {
                            let mut _offset = 0_usize;
                            let _field_0 = values.integer(
                                b0_b0,
                                _offset,
                                8_usize,
                                data::graph::Endianness::Big,
                                data::graph::Signedness::Unsigned,
                            )?;
                            let _matched_0 = _field_0;
                            _offset = _offset.checked_add(8_usize)?;
                            let _length_1 = b0_b0.bit_len().checked_sub(_offset)?;
                            let _range_1 = b0_b0.slice(_offset, _length_1)?;
                            _offset = _offset.checked_add(_length_1)?;
                            if _offset != b0_b0.bit_len() { return None; }
                            Some((_matched_0,))
                        })();
                        match _matched {
                            Some((_matched_0,)) => {
                                break 'block_1 (_matched_0, b0_v0, b0_v1,);
                            },
                            None => {
                                break 'block_4 (b0_b0,);
                            },
                        }
                    };

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b4_b0]);
                    return data::compiled::CompiledProgress::Interpreted(5);
                };

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b1_v0, b1_v1]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                data::compiled::CompiledProgress::Interpreted(1)
            }

            fn bit_array_int_12_resume_1(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_v0, b1_v1,) = (values.ints[0], values.bools[0], values.bools[1],);
                let _ = budget;

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b1_v0, b1_v1]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(1))
            }

            fn bit_array_int_12_resume_2(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn bit_array_int_12_resume_3(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                let b3_i0 = 0_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Next(4)
            }

            fn bit_array_int_12_resume_4(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn bit_array_int_12_resume_5(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_b0,) = (values.bit_arrays[0],);
                let _ = budget;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[b4_b0]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(5))
            }

            fn string_bool_0(
                point: usize,
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                    7
                ] = [
                    |values, budget| CompiledResume::Exit(string_bool_0_entry((values.bools[0], values.strings[0], values.strings[1],), values, budget)),
                    string_bool_0_resume_1,
                    string_bool_0_resume_2,
                    string_bool_0_resume_3,
                    string_bool_0_resume_4,
                    string_bool_0_resume_5,
                    string_bool_0_resume_6,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn string_bool_0_entry(
                inputs: (bool, data::compiled::string::StringRange, data::compiled::string::StringRange,),
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (b0_v0, b0_s0, b0_s1,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                    return data::compiled::CompiledProgress::Yield(0);
                }
                *budget -= 1;
                let b0_v1 = values.text(b0_s0) == values.text(b0_s1);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0, b0_v1]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                    return data::compiled::CompiledProgress::Yield(1);
                }
                *budget -= 1;
                let b0_v2 = values.text(b0_s0) != values.text(b0_s1);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0, b0_v1, b0_v2]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                    return data::compiled::CompiledProgress::Yield(2);
                }
                *budget -= 1;
                if b0_v1 == b0_v0 {
                    let (b1_v0, b1_v1,) = (b0_v0, b0_v2,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0, b1_v1]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(3);
                    }
                    *budget -= 1;
                    let b1_v2 = b1_v1 != b1_v0;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0, b1_v1, b1_v2]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(4);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0, b1_v1, b1_v2]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                } else {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(5);
                    }
                    *budget -= 1;
                    let b2_v0 = false;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b2_v0]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(6);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b2_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                }
            }

            fn string_bool_0_resume_1(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b0_v0, b0_v1, b0_s0, b0_s1,) = (values.bools[0], values.bools[1], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0, b0_v1]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                let b0_v2 = values.text(b0_s0) != values.text(b0_s1);

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b0_v0, b0_v1, b0_v2]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                CompiledResume::Next(2)
            }

            fn string_bool_0_resume_2(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b0_v0, b0_v1, b0_v2, b0_s0, b0_s1,) = (values.bools[0], values.bools[1], values.bools[2], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0, b0_v1, b0_v2]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                if b0_v1 == b0_v0 {
                    let (b1_v0, b1_v1,) = (b0_v0, b0_v2,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0, b1_v1]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(3)
                } else {
                    let () = ();

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(5)
                }
            }

            fn string_bool_0_resume_3(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_v0, b1_v1,) = (values.bools[0], values.bools[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0, b1_v1]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                let b1_v2 = b1_v1 != b1_v0;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b1_v0, b1_v1, b1_v2]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(4)
            }

            fn string_bool_0_resume_4(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_v0, b1_v1, b1_v2,) = (values.bools[0], values.bools[1], values.bools[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0, b1_v1, b1_v2]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b1_v0, b1_v1, b1_v2]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn string_bool_0_resume_5(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;
                let b2_v0 = false;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b2_v0]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(6)
            }

            fn string_bool_0_resume_6(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_v0,) = (values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b2_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b2_v0]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn string_bool_1(
                point: usize,
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                    11
                ] = [
                    |values, budget| CompiledResume::Exit(string_bool_1_entry((values.strings[0],), values, budget)),
                    string_bool_1_resume_1,
                    string_bool_1_resume_2,
                    string_bool_1_resume_3,
                    string_bool_1_resume_4,
                    string_bool_1_resume_5,
                    string_bool_1_resume_6,
                    string_bool_1_resume_7,
                    string_bool_1_resume_8,
                    string_bool_1_resume_9,
                    string_bool_1_resume_10,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn string_bool_1_entry(
                inputs: (data::compiled::string::StringRange,),
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (b0_s0,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0]);
                    return data::compiled::CompiledProgress::Yield(0);
                }
                *budget -= 1;
                if values.text(b0_s0).starts_with("") {
                    let (b1_s0,) = (b0_s0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0]);
                        return data::compiled::CompiledProgress::Yield(1);
                    }
                    *budget -= 1;
                    let b1_s1 = data::compiled::string::StringRange::literal("");
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;
                    let b1_s2 = b1_s0.drop_prefix(0);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                        return data::compiled::CompiledProgress::Yield(3);
                    }
                    *budget -= 1;
                    let b1_s3 = data::compiled::string::StringRange::literal("");
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2, b1_s3]);
                        return data::compiled::CompiledProgress::Yield(4);
                    }
                    *budget -= 1;
                    if values.text(b1_s1) == values.text(b1_s3) {
                        let (b2_s0, b2_s1,) = (b1_s0, b1_s2,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                            return data::compiled::CompiledProgress::Yield(5);
                        }
                        *budget -= 1;
                        let b2_v0 = values.text(b2_s1) == values.text(b2_s0);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b2_v0]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                            return data::compiled::CompiledProgress::Yield(6);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b2_v0]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                    } else {
                        let () = ();
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(7);
                        }
                        *budget -= 1;
                        let b3_v0 = false;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b3_v0]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(8);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b3_v0]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                    }
                } else {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(9);
                    }
                    *budget -= 1;
                    let b4_v0 = false;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b4_v0]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(10);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b4_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2))
                }
            }

            fn string_bool_1_resume_1(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_s0,) = (values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                let b1_s1 = data::compiled::string::StringRange::literal("");

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                CompiledResume::Next(2)
            }

            fn string_bool_1_resume_2(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_s0, b1_s1,) = (values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                let b1_s2 = b1_s0.drop_prefix(0);

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                CompiledResume::Next(3)
            }

            fn string_bool_1_resume_3(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_s0, b1_s1, b1_s2,) = (values.strings[0], values.strings[1], values.strings[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                let b1_s3 = data::compiled::string::StringRange::literal("");

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2, b1_s3]);
                CompiledResume::Next(4)
            }

            fn string_bool_1_resume_4(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_s0, b1_s1, b1_s2, b1_s3,) = (values.strings[0], values.strings[1], values.strings[2], values.strings[3],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2, b1_s3]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;
                if values.text(b1_s1) == values.text(b1_s3) {
                    let (b2_s0, b2_s1,) = (b1_s0, b1_s2,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                    CompiledResume::Next(5)
                } else {
                    let () = ();

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(7)
                }
            }

            fn string_bool_1_resume_5(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_s0, b2_s1,) = (values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;
                let b2_v0 = values.text(b2_s1) == values.text(b2_s0);

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b2_v0]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                CompiledResume::Next(6)
            }

            fn string_bool_1_resume_6(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_v0, b2_s0, b2_s1,) = (values.bools[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b2_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b2_v0]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn string_bool_1_resume_7(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                }
                *budget -= 1;
                let b3_v0 = false;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b3_v0]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(8)
            }

            fn string_bool_1_resume_8(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_v0,) = (values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b3_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b3_v0]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn string_bool_1_resume_9(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(9));
                }
                *budget -= 1;
                let b4_v0 = false;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b4_v0]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Next(10)
            }

            fn string_bool_1_resume_10(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_v0,) = (values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b4_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(10));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b4_v0]);

                values.strings.clear();
                values.strings.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2)))
            }
            data::compiled::CompiledFunctions {
                ints: data::Storage::Static(&[
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(0),
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
                                    strings: 1,
                                    customs: 0,
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
                                    strings: 1,
                                    customs: 0,
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
                                    strings: 2,
                                    customs: 0,
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
                                    strings: 2,
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
                                    strings: 1,
                                    customs: 0,
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
                                    strings: 2,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(3),
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
                                    block: data::graph::BlockId(4),
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
                                    block: data::graph::BlockId(4),
                                    instruction: 1,
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
                            ]),
                            run: string_int_0,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(1),
                        implementation: data::compiled::CompiledImplementation::String(data::compiled::StringImplementation {
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
                                    block: data::graph::BlockId(1),
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
                                    block: data::graph::BlockId(1),
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
                                    block: data::graph::BlockId(2),
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
                                    block: data::graph::BlockId(2),
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
                                    block: data::graph::BlockId(3),
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
                                    block: data::graph::BlockId(3),
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
                                    block: data::graph::BlockId(4),
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
                                    block: data::graph::BlockId(4),
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
                            ]),
                            run: string_int_1,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(2),
                        implementation: data::compiled::CompiledImplementation::String(data::compiled::StringImplementation {
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 1,
                                    bools: 1,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 2,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 0,
                                    ints: 1,
                                    bools: 1,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 2,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 1,
                                    ints: 1,
                                    bools: 1,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 3,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 0,
                                    ints: 1,
                                    bools: 1,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 2,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(3),
                                    instruction: 0,
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
                                    block: data::graph::BlockId(3),
                                    instruction: 1,
                                    ints: 1,
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
                                    block: data::graph::BlockId(3),
                                    instruction: 2,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 3,
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
                                    strings: 1,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(4),
                                    instruction: 1,
                                    ints: 1,
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
                                    customs: 0,
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
                                    ints: 1,
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
                                    ints: 1,
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
                                    block: data::graph::BlockId(11),
                                    instruction: 0,
                                    ints: 1,
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
                                    block: data::graph::BlockId(12),
                                    instruction: 0,
                                    ints: 1,
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
                                    block: data::graph::BlockId(12),
                                    instruction: 1,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 3,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(13),
                                    instruction: 0,
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
                                    block: data::graph::BlockId(13),
                                    instruction: 1,
                                    ints: 1,
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
                                    block: data::graph::BlockId(13),
                                    instruction: 2,
                                    ints: 2,
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
                                    block: data::graph::BlockId(14),
                                    instruction: 0,
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
                                    block: data::graph::BlockId(15),
                                    instruction: 0,
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
                                    block: data::graph::BlockId(16),
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
                                    block: data::graph::BlockId(16),
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
                                    block: data::graph::BlockId(17),
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
                                    block: data::graph::BlockId(17),
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
                                    block: data::graph::BlockId(18),
                                    instruction: 0,
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
                                    block: data::graph::BlockId(19),
                                    instruction: 0,
                                    ints: 1,
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
                                    block: data::graph::BlockId(20),
                                    instruction: 0,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 2,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                            ]),
                            run: string_int_2,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(3),
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
                                    strings: 2,
                                    customs: 0,
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
                                    strings: 2,
                                    customs: 0,
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
                                    strings: 3,
                                    customs: 0,
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
                                    strings: 3,
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
                                    strings: 2,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(3),
                                    instruction: 0,
                                    ints: 1,
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
                                    block: data::graph::BlockId(3),
                                    instruction: 1,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 3,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(3),
                                    instruction: 2,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 3,
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
                                    strings: 2,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(4),
                                    instruction: 1,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 3,
                                    customs: 0,
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
                                    strings: 2,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(6),
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
                                    block: data::graph::BlockId(7),
                                    instruction: 0,
                                    ints: 1,
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
                                    block: data::graph::BlockId(8),
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
                            run: string_int_3,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(4),
                        implementation: data::compiled::CompiledImplementation::String(data::compiled::StringImplementation {
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
                                    strings: 1,
                                    customs: 0,
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
                                    strings: 1,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
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
                                    block: data::graph::BlockId(1),
                                    instruction: 2,
                                    ints: 0,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 3,
                                    customs: 0,
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
                                    strings: 1,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
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
                                    block: data::graph::BlockId(2),
                                    instruction: 2,
                                    ints: 1,
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
                                    block: data::graph::BlockId(3),
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
                                    block: data::graph::BlockId(4),
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
                                    block: data::graph::BlockId(4),
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
                                    block: data::graph::BlockId(5),
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
                            run: string_int_4,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(5),
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
                                    strings: 1,
                                    customs: 0,
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
                                    strings: 3,
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
                                    strings: 1,
                                    customs: 0,
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
                                    strings: 2,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(3),
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
                                    block: data::graph::BlockId(3),
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
                                    block: data::graph::BlockId(4),
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
                                    block: data::graph::BlockId(4),
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
                                    block: data::graph::BlockId(5),
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
                                    block: data::graph::BlockId(5),
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
                                    block: data::graph::BlockId(6),
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
                                    block: data::graph::BlockId(6),
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
                            ]),
                            run: string_int_5,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(7),
                        implementation: data::compiled::CompiledImplementation::String(data::compiled::StringImplementation {
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
                                    block: data::graph::BlockId(1),
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
                                    block: data::graph::BlockId(2),
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
                                    block: data::graph::BlockId(2),
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
                            ]),
                            run: string_int_7,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(8),
                        implementation: data::compiled::CompiledImplementation::String(data::compiled::StringImplementation {
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
                                    strings: 1,
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
                                    strings: 1,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                            ]),
                            run: string_int_8,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(9),
                        implementation: data::compiled::CompiledImplementation::String(data::compiled::StringImplementation {
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
                                    block: data::graph::BlockId(1),
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
                                    block: data::graph::BlockId(2),
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
                                    block: data::graph::BlockId(2),
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
                                    block: data::graph::BlockId(3),
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
                                    block: data::graph::BlockId(3),
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
                                    block: data::graph::BlockId(4),
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
                                    block: data::graph::BlockId(4),
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
                            ]),
                            run: string_int_9,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(10),
                        implementation: data::compiled::CompiledImplementation::String(data::compiled::StringImplementation {
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
                                    block: data::graph::BlockId(1),
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
                                    block: data::graph::BlockId(1),
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
                                    block: data::graph::BlockId(2),
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
                                    block: data::graph::BlockId(2),
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
                            ]),
                            run: string_int_10,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(11),
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
                                    strings: 1,
                                    customs: 0,
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
                                    strings: 1,
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
                                    strings: 1,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(3),
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
                                    block: data::graph::BlockId(3),
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
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(4),
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
                                    block: data::graph::BlockId(5),
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
                                    block: data::graph::BlockId(5),
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
                                    block: data::graph::BlockId(6),
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
                                    block: data::graph::BlockId(6),
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
                            ]),
                            run: string_int_11,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(12),
                        implementation: data::compiled::CompiledImplementation::BitArray(data::compiled::BitArrayImplementation {
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 0,
                                    bools: 2,
                                    bit_arrays: 1,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 0,
                                    ints: 1,
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
                                    block: data::graph::BlockId(3),
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
                                    block: data::graph::BlockId(3),
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
                                    block: data::graph::BlockId(4),
                                    instruction: 0,
                                    ints: 0,
                                    bools: 0,
                                    bit_arrays: 1,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                            ]),
                            run: bit_array_int_12,
                        }),
                    },
                ]),
                bools: data::Storage::Static(&[
                    data::compiled::CompiledFunction {
                        function: data::function::BoolFunctionId(0),
                        implementation: data::compiled::CompiledImplementation::String(data::compiled::StringImplementation {
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 0,
                                    bools: 1,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 2,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 1,
                                    ints: 0,
                                    bools: 2,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 2,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 2,
                                    ints: 0,
                                    bools: 3,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 2,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 0,
                                    ints: 0,
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
                                    block: data::graph::BlockId(1),
                                    instruction: 1,
                                    ints: 0,
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
                                    block: data::graph::BlockId(2),
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
                                    block: data::graph::BlockId(2),
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
                            ]),
                            run: string_bool_0,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::BoolFunctionId(1),
                        implementation: data::compiled::CompiledImplementation::String(data::compiled::StringImplementation {
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
                                    block: data::graph::BlockId(1),
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
                                    block: data::graph::BlockId(1),
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
                                    block: data::graph::BlockId(1),
                                    instruction: 2,
                                    ints: 0,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 3,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 3,
                                    ints: 0,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 4,
                                    customs: 0,
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
                                    strings: 2,
                                    customs: 0,
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
                                    strings: 2,
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
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(3),
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
                                    block: data::graph::BlockId(4),
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
                                    block: data::graph::BlockId(4),
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
                            ]),
                            run: string_bool_1,
                        }),
                    },
                ]),
                customs: data::Storage::Static(&[
                ]),
                int_lists: data::Storage::Static(&[
                ]),
                callbacks: data::compiled::CompiledCallbacks::interpreted(),
                native_loops: data::Storage::Static(&[
                ]),
                function_calls: data::Storage::Static(&[
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
                0..13,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                13..15,
                0..0,
                15..16,
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
                        data::type_::ValueShapeId(1),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 2..3,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 3..7,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(2),
                        data::type_::ValueShapeId(1),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 7..10,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(1),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 10..10,
                    parameter_shapes: data::Storage::Static(&[]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 10..12,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
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
                    parameters: 14..14,
                    parameter_shapes: data::Storage::Static(&[]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 14..15,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 15..16,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 16..18,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(1),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 18..21,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(5),
                        data::type_::ValueShapeId(2),
                        data::type_::ValueShapeId(2),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 21..24,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(2),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 24..25,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 25..27,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(1),
                    ]),
                    return_: data::type_::ValueShapeId(4),
                    captures: data::Storage::Static(&[]),
                },
            ]),
            parameters: data::Storage::Static(&[
                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                data::type_::ValueShapeDescriptor::String,
                data::type_::ValueShapeDescriptor::Int,
                data::type_::ValueShapeDescriptor::Bool,
                data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(1)),
                data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                    data::type_::ValueShapeId(0),
                    data::type_::ValueShapeId(1),
                    data::type_::ValueShapeId(3),
                    data::type_::ValueShapeId(2),
                ])),
                data::type_::ValueShapeDescriptor::BitArray,
            ]),
            shape_types: data::Storage::Static(&[
                data::type_::ValueType::String,
                data::type_::ValueType::Int,
                data::type_::ValueType::Bool,
                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                data::type_::ValueType::Tuple(data::Storage::Static(&[
                    data::type_::ValueType::String,
                    data::type_::ValueType::Int,
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                    data::type_::ValueType::Bool,
                ])),
                data::type_::ValueType::BitArray,
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
            data::program::LibraryFunctionEntry {
                function: data::function::IntFunctionId(12),
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
            name: data::Text::Static("count"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::String,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 0,
        },
        data::Export {
            name: data::Text::Static("select"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::String,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 1,
        },
        data::Export {
            name: data::Text::Static("aliases"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::String,
                    data::type_::TypeMetadata::String,
                    data::type_::TypeMetadata::Bool,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 2,
        },
        data::Export {
            name: data::Text::Static("alternate"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::String,
                    data::type_::TypeMetadata::String,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 3,
        },
        data::Export {
            name: data::Text::Static("same"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::String,
                    data::type_::TypeMetadata::String,
                    data::type_::TypeMetadata::Bool,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            slot: 0,
        },
        data::Export {
            name: data::Text::Static("empty_prefix"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::String,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            slot: 1,
        },
        data::Export {
            name: data::Text::Static("literal_only"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 4,
        },
        data::Export {
            name: data::Text::Static("asserted"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::String,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 5,
        },
        data::Export {
            name: data::Text::Static("caller"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::String,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                    data::type_::TypeMetadata::String,
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                    data::type_::TypeMetadata::Bool,
                ]))),
            },
            slot: 0,
        },
        data::Export {
            name: data::Text::Static("unsupported"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::String,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 6,
        },
        data::Export {
            name: data::Text::Static("spin"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::String,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 7,
        },
        data::Export {
            name: data::Text::Static("main"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 8,
        },
        data::Export {
            name: data::Text::Static("assert_literal"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::String,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 9,
        },
        data::Export {
            name: data::Text::Static("assert_prefix"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::String,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 10,
        },
        data::Export {
            name: data::Text::Static("assert_suffix"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::String,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 11,
        },
        data::Export {
            name: data::Text::Static("bits_with_boolean_guard"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::BitArray,
                    data::type_::TypeMetadata::Bool,
                    data::type_::TypeMetadata::Bool,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 12,
        },
    ]),
}
