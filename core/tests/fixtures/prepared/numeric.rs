data::ModuleArtifact {
    format: 24,
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
                                        terminator: data::graph::Terminator::IntSwitch(data::graph::IntSwitch {
                                            subject: data::graph::IntLocalId(0),
                                            clauses: data::Storage::Static(&[
                                                (data::graph::IntegerLiteral {
                                                    sign: data::Sign::NoSign,
                                                    digits: data::Storage::Static(&[]),
                                                }, data::graph::Edge {
                                                    target: data::graph::BlockId(1),
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
                                                        ]),
                                                    },
                                                }),
                                            ]),
                                            fallback: data::graph::Edge {
                                                target: data::graph::BlockId(2),
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
                                        params: 2..3,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 3..5,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::EqualInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
                                                right: data::graph::IntegerOperand::Immediate(0),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Int,
                                                            length: 2,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(4),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Int,
                                                            length: 2,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 5..7,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
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
                                                                    source: 3,
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
                                        params: 7..9,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
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
                                                                    source: 3,
                                                                    destination: 1,
                                                                },
                                                            ]),
                                                        },
                                                    ]),
                                                },
                                            },
                                        }),
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
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Remainder {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Immediate(2),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::IntegerRegion(data::graph::ArithmeticRegion {
                                        inputs: data::Storage::Static(&[
                                            data::graph::IntLocalId(0),
                                            data::graph::IntLocalId(1),
                                        ]),
                                        nodes: data::Storage::Static(&[
                                            data::graph::ArithmeticNode::Subtract(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Immediate(1)),
                                            data::graph::ArithmeticNode::Multiply(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Immediate(3)),
                                            data::graph::ArithmeticNode::Add(data::graph::ArithmeticOperand::Input(1), data::graph::ArithmeticOperand::Value(1)),
                                            data::graph::ArithmeticNode::Add(data::graph::ArithmeticOperand::Value(2), data::graph::ArithmeticOperand::Immediate(1)),
                                        ]),
                                        outputs: data::Storage::Static(&[
                                            data::graph::ArithmeticOutput {
                                                value: 0,
                                                slot: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                    shape: data::type_::ValueShapeId(0),
                                                },
                                            },
                                            data::graph::ArithmeticOutput {
                                                value: 3,
                                                slot: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                    shape: data::type_::ValueShapeId(0),
                                                },
                                            },
                                        ]),
                                        native: true,
                                    }),
                                    data::graph::ProfiledInstruction::IntegerRegion(data::graph::ArithmeticRegion {
                                        inputs: data::Storage::Static(&[
                                            data::graph::IntLocalId(0),
                                            data::graph::IntLocalId(1),
                                        ]),
                                        nodes: data::Storage::Static(&[
                                            data::graph::ArithmeticNode::Subtract(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Immediate(1)),
                                            data::graph::ArithmeticNode::Multiply(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Immediate(2)),
                                            data::graph::ArithmeticNode::Add(data::graph::ArithmeticOperand::Input(1), data::graph::ArithmeticOperand::Value(1)),
                                            data::graph::ArithmeticNode::Subtract(data::graph::ArithmeticOperand::Value(2), data::graph::ArithmeticOperand::Immediate(1)),
                                        ]),
                                        outputs: data::Storage::Static(&[
                                            data::graph::ArithmeticOutput {
                                                value: 0,
                                                slot: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                    shape: data::type_::ValueShapeId(0),
                                                },
                                            },
                                            data::graph::ArithmeticOutput {
                                                value: 3,
                                                slot: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                    shape: data::type_::ValueShapeId(0),
                                                },
                                            },
                                        ]),
                                        native: true,
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
                                            test: data::graph::BoolTest::LtEqInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(0),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
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
                                                                    source: 2,
                                                                    destination: 1,
                                                                },
                                                            ]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(4),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..7,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                            subject: data::graph::BoolLocalId(0),
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                        params: 7..9,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 9..11,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 11..15,
                                        instructions: 2..4,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Int,
                                                            length: 3,
                                                            steps: data::Storage::Static(&[
                                                                data::graph::TransferStep {
                                                                    source: 3,
                                                                    destination: 0,
                                                                },
                                                                data::graph::TransferStep {
                                                                    source: 2,
                                                                    destination: 1,
                                                                },
                                                            ]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Bool,
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
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
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
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
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
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Immediate(1),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Not(data::graph::BoolLocalId(0)))),
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
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::IntSwitch(data::graph::IntSwitch {
                                            subject: data::graph::IntLocalId(0),
                                            clauses: data::Storage::Static(&[
                                                (data::graph::IntegerLiteral {
                                                    sign: data::Sign::NoSign,
                                                    digits: data::Storage::Static(&[]),
                                                }, data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                }),
                                                (data::graph::IntegerLiteral {
                                                    sign: data::Sign::Plus,
                                                    digits: data::Storage::Static(&[
                                                        1,
                                                    ]),
                                                }, data::graph::Edge {
                                                    target: data::graph::BlockId(5),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                }),
                                            ]),
                                            fallback: data::graph::Edge {
                                                target: data::graph::BlockId(6),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..2,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::GtInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(0),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(4),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..3,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 3..4,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..4,
                                        instructions: 3..4,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..5,
                                        instructions: 4..4,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
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
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Minus,
                                            digits: data::Storage::Static(&[
                                                3,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Immediate(42),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Negate(data::graph::IntLocalId(0))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
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
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                            subject: data::graph::BoolLocalId(0),
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
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
                                        params: 3..4,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..5,
                                        instructions: 2..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
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
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Div {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Negate(data::graph::IntLocalId(0))),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
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
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                            subject: data::graph::BoolLocalId(0),
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
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
                                                                    source: 2,
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
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
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
                                                                    source: 2,
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 3..5,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 5..7,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
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
                                        shape: data::type_::ValueShapeId(1),
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
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::IntegerRegion(data::graph::ArithmeticRegion {
                                        inputs: data::Storage::Static(&[
                                            data::graph::IntLocalId(0),
                                            data::graph::IntLocalId(1),
                                        ]),
                                        nodes: data::Storage::Static(&[
                                            data::graph::ArithmeticNode::Multiply(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Input(1)),
                                            data::graph::ArithmeticNode::Add(data::graph::ArithmeticOperand::Value(0), data::graph::ArithmeticOperand::Input(0)),
                                            data::graph::ArithmeticNode::Subtract(data::graph::ArithmeticOperand::Value(1), data::graph::ArithmeticOperand::Input(1)),
                                            data::graph::ArithmeticNode::Negate(data::graph::ArithmeticOperand::Value(2)),
                                        ]),
                                        outputs: data::Storage::Static(&[
                                            data::graph::ArithmeticOutput {
                                                value: 3,
                                                slot: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                    shape: data::type_::ValueShapeId(0),
                                                },
                                            },
                                        ]),
                                        native: true,
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Div {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Remainder {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
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
                            parameter_count: 3,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..3,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                            subject: data::graph::BoolLocalId(0),
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
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
                                        params: 3..4,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..5,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
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
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::IntegerRegion(data::graph::ArithmeticRegion {
                                        inputs: data::Storage::Static(&[
                                            data::graph::IntLocalId(0),
                                            data::graph::IntLocalId(1),
                                        ]),
                                        nodes: data::Storage::Static(&[
                                            data::graph::ArithmeticNode::Divide(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Input(1)),
                                            data::graph::ArithmeticNode::Remainder(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Input(1)),
                                            data::graph::ArithmeticNode::Add(data::graph::ArithmeticOperand::Value(0), data::graph::ArithmeticOperand::Value(1)),
                                        ]),
                                        outputs: data::Storage::Static(&[
                                            data::graph::ArithmeticOutput {
                                                value: 2,
                                                slot: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                    shape: data::type_::ValueShapeId(0),
                                                },
                                            },
                                        ]),
                                        native: true,
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Negate(data::graph::IntLocalId(0))),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
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
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::EqualInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(5),
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
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                        params: 4..7,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                            subject: data::graph::BoolLocalId(0),
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                                target: data::graph::BlockId(4),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Int,
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 7..9,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 9..10,
                                        instructions: 2..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 10..12,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
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
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(false)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Mult {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(2)),
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
                                                target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(10)),
                                                captures: data::Storage::Static(&[
                                                    data::graph::FunctionCapture::Int {
                                                        target: data::graph::IntLocalId(1),
                                                        source: data::graph::IntLocalId(1),
                                                    },
                                                ]),
                                            },
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::FunctionCall {
                                            function: data::graph::IntFunctionLocalId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "captured", data::source::SourceSpan::new(1895, 1911)),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(2)),
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
                                        instructions: 0..12,
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
                                                12,
                                            ]),
                                        })),
                                    }),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2174, 2191)),
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
                                                3,
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
                                                7,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2194, 2217)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(6)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                            shape: data::type_::ValueShapeId(0),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(2),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2220, 2229)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(7)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(9)),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(10)),
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
                                        terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                            subject: data::graph::BoolLocalId(0),
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..3,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 3..4,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::IntegerRegion(data::graph::ArithmeticRegion {
                                        inputs: data::Storage::Static(&[
                                            data::graph::IntLocalId(0),
                                        ]),
                                        nodes: data::Storage::Static(&[
                                            data::graph::ArithmeticNode::Add(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Immediate(1)),
                                            data::graph::ArithmeticNode::Add(data::graph::ArithmeticOperand::Value(0), data::graph::ArithmeticOperand::Immediate(2)),
                                        ]),
                                        outputs: data::Storage::Static(&[]),
                                        native: true,
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Negate(data::graph::IntLocalId(0))),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
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
                                        params: 0..2,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::GtEqInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(0),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
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
                                    data::graph::BlockHeader {
                                        params: 4..6,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
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
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
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
                                            test: data::graph::BoolTest::LtInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(0),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..3,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 3..5,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::GtEqInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(10),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                        params: 5..6,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 6..6,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Not(data::graph::BoolLocalId(0)))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(false)),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::BoolLocalId(1)),
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
                            parameter_count: 3,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..3,
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
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(3),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Int(data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                            data::graph::IntLocalId(0),
                                            data::graph::IntLocalId(1),
                                        ])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "caller", data::source::SourceSpan::new(2064, 2092)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(7),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "caller", data::source::SourceSpan::new(2118, 2144)),
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
                                                    data::type_::ValueType::Int,
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
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
            use data::compiled::calls::{BoolCallable, CallArguments, CallCapture, CallCaptureInputs, CallExecution, CallInputs, CallInteger, CallOps, CallOutput, CallProgress, CallStorage, CallValues, IntCallable};
            enum FunctionState {
                Int0Point0 { int0: i128, int1: i128 },
                Int0Point1 { int0: i128 },
                Int0Point2 { int0: i128, int1: i128 },
                Int0Point3 { int0: i128, int1: i128, int2: i128 },
                Int0Point4 { int0: i128, int1: i128 },
                Int0Point5 { int0: i128, int1: i128, int2: i128, int3: i128 },
                Int0Point6 { int0: i128, int1: i128 },
                Int0Point7 { int0: i128, int1: i128, int2: i128, int3: i128 },
                Int1Point0 { int0: i128, int1: i128, bool0: bool, int2: i128 },
                Int1Point1 { int0: i128, bool0: bool, int1: i128 },
                Int1Point2 { int0: i128, int1: i128 },
                Int1Point3 { int0: i128, int1: i128, int2: i128 },
                Int1Point4 { int0: i128, int1: i128 },
                Int1Point5 { int0: i128, int1: i128, int2: i128 },
                Int1Point6 { int0: i128, int1: i128, bool0: bool, int2: i128 },
                Int1Point7 { int0: i128, int1: i128, bool0: bool, int2: i128, int3: i128 },
                Int1Point8 { int0: i128, int1: i128, bool0: bool, int2: i128, int3: i128, bool1: bool },
                Int2Point0 { int0: i128 },
                Int2Point1 {  },
                Int2Point2 { int0: i128 },
                Int2Point3 { int0: i128 },
                Int2Point4 { int0: i128 },
                Int2Point5 { int0: i128, int1: i128 },
                Int2Point6 { int0: i128 },
                Int2Point7 { int0: i128, int1: i128 },
                Int2Point8 {  },
                Int2Point9 { int0: i128 },
                Int2Point10 { int0: i128 },
                Int3Point0 { int0: i128, int1: i128, bool0: bool },
                Int3Point1 { int0: i128, int1: i128, bool0: bool, int2: i128 },
                Int3Point2 { int0: i128 },
                Int3Point3 { int0: i128, int1: i128 },
                Int3Point4 { int0: i128 },
                Int4Point0 { int0: i128, int1: i128, bool0: bool },
                Int4Point1 { int0: i128, int1: i128, bool0: bool, int2: i128 },
                Int4Point2 { int0: i128, int1: i128 },
                Int4Point3 { int0: i128, int1: i128, int2: i128 },
                Int4Point4 { int0: i128, int1: i128 },
                Int4Point5 { int0: i128, int1: i128, int2: i128 },
                Int5Point0 { int0: i128, int1: i128, bool0: bool },
                Int5Point1 { int0: i128, int1: i128, bool0: bool, int2: i128 },
                Int5Point2 { int0: i128 },
                Int5Point3 { int0: i128 },
                Int5Point4 { int0: i128, int1: i128 },
                Int6Point0 { int0: i128, int1: i128 },
                Int6Point1 { int0: i128, int1: i128 },
                Int6Point2 { int0: i128, int1: i128, bool0: bool },
                Int6Point3 { bool0: bool, int0: i128, int1: i128 },
                Int6Point4 { int0: i128, int1: i128 },
                Int6Point5 { int0: i128, int1: i128, int2: i128 },
                Int6Point6 { int0: i128 },
                Int6Point7 { int0: i128, int1: i128 },
                Int6Point8 { int0: i128, int1: i128, bool0: bool },
                Int7Point0 { int0: i128, int1: i128 },
                Int7Point1 { int0: i128, int1: i128, int_function0: IntCallable },
                Int7Point2 { int0: i128, int1: i128, int_function0: IntCallable, int2: i128 },
                Int8Point0 {  },
                Int8Point1 { int0: i128 },
                Int8Point2 { int0: i128, int1: i128 },
                Int8Point3 { int0: i128, int1: i128, int2: i128 },
                Int8Point4 { int0: i128, int1: i128, int2: i128, int3: i128 },
                Int8Point5 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128 },
                Int8Point6 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, bool0: bool },
                Int8Point7 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, bool0: bool, int5: i128 },
                Int8Point8 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, bool0: bool, int5: i128, int6: i128 },
                Int8Point9 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, bool0: bool, int5: i128, int6: i128, int7: i128 },
                Int8Point10 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, bool0: bool, int5: i128, int6: i128, int7: i128, int8: i128 },
                Int8Point11 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, bool0: bool, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128 },
                Int8Point12 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, bool0: bool, int5: i128, int6: i128, int7: i128, int8: i128, int9: i128, int10: i128 },
                Int9Point0 { int0: i128, bool0: bool },
                Int9Point1 { int0: i128, bool0: bool },
                Int9Point2 { int0: i128 },
                Int9Point3 { int0: i128 },
                Int9Point4 { int0: i128, int1: i128 },
                Int10Point0 { int0: i128, int1: i128 },
                Int10Point1 { int0: i128, int1: i128 },
                Int10Point2 { int0: i128, int1: i128, int2: i128 },
                Int10Point3 { int0: i128, int1: i128 },
                Int10Point4 { int0: i128, int1: i128, int2: i128 },
                Bool0Point0 { int0: i128, bool0: bool },
                Bool0Point1 { bool0: bool },
                Bool0Point2 { bool0: bool, bool1: bool },
                Bool0Point3 { int0: i128, bool0: bool },
                Bool0Point4 { bool0: bool },
                Bool0Point5 {  },
                Bool0Point6 { bool0: bool },
                Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: CallValues },
            }
            enum IntReturn {
                Int7Call1 { int0: i128, int1: i128, int_function0: IntCallable },
                Int8Call2 { int0: i128, int1: i128 },
                Int8Call7 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, bool0: bool, int5: i128 },
                Int8Call10 { int0: i128, int1: i128, int2: i128, int3: i128, int4: i128, bool0: bool, int5: i128, int6: i128, int7: i128, int8: i128 },
            }
            impl IntReturn {
                fn site(&self) -> data::source::HostCallSite {
                    match *self {
                        Self::Int7Call1 { .. } => data::source::HostCallSite::from_static("example", "captured", data::source::SourceSpan::new(1895, 1911)),
                        Self::Int8Call2 { .. } => data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2174, 2191)),
                        Self::Int8Call7 { .. } => data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2194, 2217)),
                        Self::Int8Call10 { .. } => data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2220, 2229)),
                    }
                }
                fn small(self, result: i128) -> FunctionState {
                    match self {
                        Self::Int7Call1 { int0, int1, int_function0 } => {
                            let int2 = result;
                            FunctionState::Int7Point2 { int0, int1, int_function0, int2 }
                        },
                        Self::Int8Call2 { int0, int1 } => {
                            let int2 = result;
                            FunctionState::Int8Point3 { int0, int1, int2 }
                        },
                        Self::Int8Call7 { int0, int1, int2, int3, int4, bool0, int5 } => {
                            let int6 = result;
                            FunctionState::Int8Point8 { int0, int1, int2, int3, int4, bool0, int5, int6 }
                        },
                        Self::Int8Call10 { int0, int1, int2, int3, int4, bool0, int5, int6, int7, int8 } => {
                            let int9 = result;
                            FunctionState::Int8Point11 { int0, int1, int2, int3, int4, bool0, int5, int6, int7, int8, int9 }
                        },
                    }
                }
                fn resume(self, result: CallInteger) -> FunctionState {
                    if let Some(result) = result.small() {
                        return self.small(result);
                    }
                    match self {
                        Self::Int7Call1 { int0, int1, int_function0 } => {
                            let int2 = result;
                            FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(7)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 2,
                                ints: 3,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 1,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into(), int1.into(), int2], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] } }
                        },
                        Self::Int8Call2 { int0, int1 } => {
                            let int2 = result;
                            FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(8)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: CallValues { ints: vec![int0.into(), int1.into(), int2], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }
                        },
                        Self::Int8Call7 { int0, int1, int2, int3, int4, bool0, int5 } => {
                            let int6 = result;
                            FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(8)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 8,
                                ints: 7,
                                bools: 1,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }
                        },
                        Self::Int8Call10 { int0, int1, int2, int3, int4, bool0, int5, int6, int7, int8 } => {
                            let int9 = result;
                            FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(8)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 11,
                                ints: 10,
                                bools: 1,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }
                        },
                    }
                }
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
            enum IntFunctionReturn {
            }
            impl IntFunctionReturn {
                fn site(&self) -> data::source::HostCallSite {
                    match *self {
                    }
                }
                fn small(self, result: IntCallable) -> FunctionState {
                    let _ = result;
                    match self {
                    }
                }
                fn resume(self, result: IntCallable) -> FunctionState { self.small(result) }
            }
            enum BoolFunctionReturn {
            }
            impl BoolFunctionReturn {
                fn site(&self) -> data::source::HostCallSite {
                    match *self {
                    }
                }
                fn small(self, result: BoolCallable) -> FunctionState {
                    let _ = result;
                    match self {
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
                Int { value: i128, exit: data::graph::BlockGraphExitId },
                IntBridge { function: data::function::IntFunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: IntReturn },
                Bool { value: bool, exit: data::graph::BlockGraphExitId },
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
                        data::compiled::CallTarget::Int(data::function::IntFunctionId(1)) => calls_int_1_state(point, values),
                        data::compiled::CallTarget::Int(data::function::IntFunctionId(2)) => calls_int_2_state(point, values),
                        data::compiled::CallTarget::Int(data::function::IntFunctionId(3)) => calls_int_3_state(point, values),
                        data::compiled::CallTarget::Int(data::function::IntFunctionId(4)) => calls_int_4_state(point, values),
                        data::compiled::CallTarget::Int(data::function::IntFunctionId(5)) => calls_int_5_state(point, values),
                        data::compiled::CallTarget::Int(data::function::IntFunctionId(6)) => calls_int_6_state(point, values),
                        data::compiled::CallTarget::Int(data::function::IntFunctionId(7)) => calls_int_7_state(point, values),
                        data::compiled::CallTarget::Int(data::function::IntFunctionId(8)) => calls_int_8_state(point, values),
                        data::compiled::CallTarget::Int(data::function::IntFunctionId(9)) => calls_int_9_state(point, values),
                        data::compiled::CallTarget::Int(data::function::IntFunctionId(10)) => calls_int_10_state(point, values),
                        data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)) => calls_bool_0_state(point, values),
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
                    FunctionState::Int0Point0 { int0, int1 } => {
                        let values = ops.numeric();
                        let progress = numeric_int_0_entry((int0, int1,), values, budget);
                        calls_int_0_numeric(progress, values)
                    },
                    FunctionState::Int0Point1 { int0 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_0(1, values, budget);
                        calls_int_0_numeric(progress, values)
                    },
                    FunctionState::Int0Point2 { int0, int1 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_0(2, values, budget);
                        calls_int_0_numeric(progress, values)
                    },
                    FunctionState::Int0Point3 { int0, int1, int2 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1, int2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_0(3, values, budget);
                        calls_int_0_numeric(progress, values)
                    },
                    FunctionState::Int0Point4 { int0, int1 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_0(4, values, budget);
                        calls_int_0_numeric(progress, values)
                    },
                    FunctionState::Int0Point5 { int0, int1, int2, int3 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1, int2, int3]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_0(5, values, budget);
                        calls_int_0_numeric(progress, values)
                    },
                    FunctionState::Int0Point6 { int0, int1 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_0(6, values, budget);
                        calls_int_0_numeric(progress, values)
                    },
                    FunctionState::Int0Point7 { int0, int1, int2, int3 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1, int2, int3]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_0(7, values, budget);
                        calls_int_0_numeric(progress, values)
                    },
                    FunctionState::Int1Point0 { int0, int1, bool0, int2 } => {
                        let values = ops.numeric();
                        let progress = numeric_int_1_entry((int0, int1, int2, bool0,), values, budget);
                        calls_int_1_numeric(progress, values)
                    },
                    FunctionState::Int1Point1 { int0, bool0, int1 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[bool0]);
                        let progress = numeric_int_1(1, values, budget);
                        calls_int_1_numeric(progress, values)
                    },
                    FunctionState::Int1Point2 { int0, int1 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_1(2, values, budget);
                        calls_int_1_numeric(progress, values)
                    },
                    FunctionState::Int1Point3 { int0, int1, int2 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1, int2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_1(3, values, budget);
                        calls_int_1_numeric(progress, values)
                    },
                    FunctionState::Int1Point4 { int0, int1 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_1(4, values, budget);
                        calls_int_1_numeric(progress, values)
                    },
                    FunctionState::Int1Point5 { int0, int1, int2 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1, int2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_1(5, values, budget);
                        calls_int_1_numeric(progress, values)
                    },
                    FunctionState::Int1Point6 { int0, int1, bool0, int2 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1, int2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[bool0]);
                        let progress = numeric_int_1(6, values, budget);
                        calls_int_1_numeric(progress, values)
                    },
                    FunctionState::Int1Point7 { int0, int1, bool0, int2, int3 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1, int2, int3]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[bool0]);
                        let progress = numeric_int_1(7, values, budget);
                        calls_int_1_numeric(progress, values)
                    },
                    FunctionState::Int1Point8 { int0, int1, bool0, int2, int3, bool1 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1, int2, int3]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[bool0, bool1]);
                        let progress = numeric_int_1(8, values, budget);
                        calls_int_1_numeric(progress, values)
                    },
                    FunctionState::Int2Point0 { int0 } => {
                        let values = ops.numeric();
                        let progress = numeric_int_2_entry((int0,), values, budget);
                        calls_int_2_numeric(progress, values)
                    },
                    FunctionState::Int2Point1 {  } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_2(1, values, budget);
                        calls_int_2_numeric(progress, values)
                    },
                    FunctionState::Int2Point2 { int0 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_2(2, values, budget);
                        calls_int_2_numeric(progress, values)
                    },
                    FunctionState::Int2Point3 { int0 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_2(3, values, budget);
                        calls_int_2_numeric(progress, values)
                    },
                    FunctionState::Int2Point4 { int0 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_2(4, values, budget);
                        calls_int_2_numeric(progress, values)
                    },
                    FunctionState::Int2Point5 { int0, int1 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_2(5, values, budget);
                        calls_int_2_numeric(progress, values)
                    },
                    FunctionState::Int2Point6 { int0 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_2(6, values, budget);
                        calls_int_2_numeric(progress, values)
                    },
                    FunctionState::Int2Point7 { int0, int1 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_2(7, values, budget);
                        calls_int_2_numeric(progress, values)
                    },
                    FunctionState::Int2Point8 {  } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_2(8, values, budget);
                        calls_int_2_numeric(progress, values)
                    },
                    FunctionState::Int2Point9 { int0 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_2(9, values, budget);
                        calls_int_2_numeric(progress, values)
                    },
                    FunctionState::Int2Point10 { int0 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_2(10, values, budget);
                        calls_int_2_numeric(progress, values)
                    },
                    FunctionState::Int3Point0 { int0, int1, bool0 } => {
                        let values = ops.numeric();
                        let progress = numeric_int_3_entry((int0, int1, bool0,), values, budget);
                        calls_int_3_numeric(progress, values)
                    },
                    FunctionState::Int3Point1 { int0, int1, bool0, int2 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1, int2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[bool0]);
                        let progress = numeric_int_3(1, values, budget);
                        calls_int_3_numeric(progress, values)
                    },
                    FunctionState::Int3Point2 { int0 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_3(2, values, budget);
                        calls_int_3_numeric(progress, values)
                    },
                    FunctionState::Int3Point3 { int0, int1 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_3(3, values, budget);
                        calls_int_3_numeric(progress, values)
                    },
                    FunctionState::Int3Point4 { int0 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_3(4, values, budget);
                        calls_int_3_numeric(progress, values)
                    },
                    FunctionState::Int4Point0 { int0, int1, bool0 } => {
                        let values = ops.numeric();
                        let progress = numeric_int_4_entry((int0, int1, bool0,), values, budget);
                        calls_int_4_numeric(progress, values)
                    },
                    FunctionState::Int4Point1 { int0, int1, bool0, int2 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1, int2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[bool0]);
                        let progress = numeric_int_4(1, values, budget);
                        calls_int_4_numeric(progress, values)
                    },
                    FunctionState::Int4Point2 { int0, int1 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_4(2, values, budget);
                        calls_int_4_numeric(progress, values)
                    },
                    FunctionState::Int4Point3 { int0, int1, int2 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1, int2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_4(3, values, budget);
                        calls_int_4_numeric(progress, values)
                    },
                    FunctionState::Int4Point4 { int0, int1 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_4(4, values, budget);
                        calls_int_4_numeric(progress, values)
                    },
                    FunctionState::Int4Point5 { int0, int1, int2 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1, int2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_4(5, values, budget);
                        calls_int_4_numeric(progress, values)
                    },
                    FunctionState::Int5Point0 { int0, int1, bool0 } => {
                        let values = ops.numeric();
                        let progress = numeric_int_5_entry((int0, int1, bool0,), values, budget);
                        calls_int_5_numeric(progress, values)
                    },
                    FunctionState::Int5Point1 { int0, int1, bool0, int2 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1, int2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[bool0]);
                        let progress = numeric_int_5(1, values, budget);
                        calls_int_5_numeric(progress, values)
                    },
                    FunctionState::Int5Point2 { int0 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_5(2, values, budget);
                        calls_int_5_numeric(progress, values)
                    },
                    FunctionState::Int5Point3 { int0 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_5(3, values, budget);
                        calls_int_5_numeric(progress, values)
                    },
                    FunctionState::Int5Point4 { int0, int1 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_5(4, values, budget);
                        calls_int_5_numeric(progress, values)
                    },
                    FunctionState::Int6Point0 { int0, int1 } => {
                        let values = ops.numeric();
                        let progress = numeric_int_6_entry((int0, int1,), values, budget);
                        calls_int_6_numeric(progress, values)
                    },
                    FunctionState::Int6Point1 { int0, int1 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_6(1, values, budget);
                        calls_int_6_numeric(progress, values)
                    },
                    FunctionState::Int6Point2 { int0, int1, bool0 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[bool0]);
                        let progress = numeric_int_6(2, values, budget);
                        calls_int_6_numeric(progress, values)
                    },
                    FunctionState::Int6Point3 { bool0, int0, int1 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[bool0]);
                        let progress = numeric_int_6(3, values, budget);
                        calls_int_6_numeric(progress, values)
                    },
                    FunctionState::Int6Point4 { int0, int1 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_6(4, values, budget);
                        calls_int_6_numeric(progress, values)
                    },
                    FunctionState::Int6Point5 { int0, int1, int2 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1, int2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_6(5, values, budget);
                        calls_int_6_numeric(progress, values)
                    },
                    FunctionState::Int6Point6 { int0 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_6(6, values, budget);
                        calls_int_6_numeric(progress, values)
                    },
                    FunctionState::Int6Point7 { int0, int1 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_6(7, values, budget);
                        calls_int_6_numeric(progress, values)
                    },
                    FunctionState::Int6Point8 { int0, int1, bool0 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[bool0]);
                        let progress = numeric_int_6(8, values, budget);
                        calls_int_6_numeric(progress, values)
                    },
                    FunctionState::Int7Point0 { int0, int1 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int7Point0 { int0, int1 }); }
                        *budget -= 1;
                        let int_function0 = ops.int_closure(data::function::IntFunctionId(10), data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        }, vec![CallCapture::int(data::graph::IntLocalId(1), int1)]);
                        FunctionStep::Next(FunctionState::Int7Point1 { int0, int1, int_function0 })
                    },
                    FunctionState::Int7Point1 { int0, int1, int_function0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int7Point1 { int0, int1, int_function0 }); }
                        *budget -= 1;
                        let callable = &int_function0;
                        let captures = callable.captures();
                        let target = callable.target();
                        if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (int0,)) {
                            return FunctionStep::IntCall { callee, caller: IntReturn::Int7Call1 { int0, int1, int_function0 } };
                        }
                        FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("example", "captured", data::source::SourceSpan::new(1895, 1911)), arguments: CallArguments { values: CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }, captures: Some(captures.retain()) }, caller: IntReturn::Int7Call1 { int0, int1, int_function0 } }
                    },
                    FunctionState::Int7Point2 { int0, int1, int_function0, int2 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int7Point2 { int0, int1, int_function0, int2 }); }
                        *budget -= 1;
                        FunctionStep::Int { value: int2, exit: data::graph::BlockGraphExitId(0) }
                    },
                    FunctionState::Int8Point0 {  } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point0 {  }); }
                        *budget -= 1;
                        let int0 = 12_i128;
                        if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(8)), point: data::compiled::CompiledCheckpoint {
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
                        FunctionStep::Next(FunctionState::Int8Point1 { int0 })
                    },
                    FunctionState::Int8Point1 { int0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point1 { int0 }); }
                        *budget -= 1;
                        let int1 = 0_i128;
                        if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(8)), point: data::compiled::CompiledCheckpoint {
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
                        }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int8Point2 { int0, int1 })
                    },
                    FunctionState::Int8Point2 { int0, int1 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point2 { int0, int1 }); }
                        *budget -= 1;
                        FunctionStep::IntCall { callee: FunctionState::Int0Point0 { int0, int1 }, caller: IntReturn::Int8Call2 { int0, int1 } }
                    },
                    FunctionState::Int8Point3 { int0, int1, int2 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point3 { int0, int1, int2 }); }
                        *budget -= 1;
                        let int3 = 3_i128;
                        if int3 < i128::from(i64::MIN) || int3 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(8)), point: data::compiled::CompiledCheckpoint {
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
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int8Point4 { int0, int1, int2, int3 })
                    },
                    FunctionState::Int8Point4 { int0, int1, int2, int3 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point4 { int0, int1, int2, int3 }); }
                        *budget -= 1;
                        let int4 = 7_i128;
                        if int4 < i128::from(i64::MIN) || int4 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(8)), point: data::compiled::CompiledCheckpoint {
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
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int8Point5 { int0, int1, int2, int3, int4 })
                    },
                    FunctionState::Int8Point5 { int0, int1, int2, int3, int4 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point5 { int0, int1, int2, int3, int4 }); }
                        *budget -= 1;
                        let bool0 = true;
                        FunctionStep::Next(FunctionState::Int8Point6 { int0, int1, int2, int3, int4, bool0 })
                    },
                    FunctionState::Int8Point6 { int0, int1, int2, int3, int4, bool0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point6 { int0, int1, int2, int3, int4, bool0 }); }
                        *budget -= 1;
                        let int5 = 11_i128;
                        if int5 < i128::from(i64::MIN) || int5 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(8)), point: data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 7,
                            ints: 6,
                            bools: 1,
                            bit_arrays: 0,
                            int_lists: 0,
                            strings: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into()], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int8Point7 { int0, int1, int2, int3, int4, bool0, int5 })
                    },
                    FunctionState::Int8Point7 { int0, int1, int2, int3, int4, bool0, int5 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point7 { int0, int1, int2, int3, int4, bool0, int5 }); }
                        *budget -= 1;
                        FunctionStep::IntCall { callee: FunctionState::Int1Point0 { int0: int3, int1: int4, bool0, int2: int5 }, caller: IntReturn::Int8Call7 { int0, int1, int2, int3, int4, bool0, int5 } }
                    },
                    FunctionState::Int8Point8 { int0, int1, int2, int3, int4, bool0, int5, int6 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point8 { int0, int1, int2, int3, int4, bool0, int5, int6 }); }
                        *budget -= 1;
                        let int7 = int2 + int6;
                        if int7 < i128::from(i64::MIN) || int7 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(8)), point: data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 9,
                            ints: 8,
                            bools: 1,
                            bit_arrays: 0,
                            int_lists: 0,
                            strings: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into()], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int8Point9 { int0, int1, int2, int3, int4, bool0, int5, int6, int7 })
                    },
                    FunctionState::Int8Point9 { int0, int1, int2, int3, int4, bool0, int5, int6, int7 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point9 { int0, int1, int2, int3, int4, bool0, int5, int6, int7 }); }
                        *budget -= 1;
                        let int8 = 1_i128;
                        if int8 < i128::from(i64::MIN) || int8 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(8)), point: data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 10,
                            ints: 9,
                            bools: 1,
                            bit_arrays: 0,
                            int_lists: 0,
                            strings: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into()], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int8Point10 { int0, int1, int2, int3, int4, bool0, int5, int6, int7, int8 })
                    },
                    FunctionState::Int8Point10 { int0, int1, int2, int3, int4, bool0, int5, int6, int7, int8 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point10 { int0, int1, int2, int3, int4, bool0, int5, int6, int7, int8 }); }
                        *budget -= 1;
                        FunctionStep::IntCall { callee: FunctionState::Int2Point0 { int0: int8 }, caller: IntReturn::Int8Call10 { int0, int1, int2, int3, int4, bool0, int5, int6, int7, int8 } }
                    },
                    FunctionState::Int8Point11 { int0, int1, int2, int3, int4, bool0, int5, int6, int7, int8, int9 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point11 { int0, int1, int2, int3, int4, bool0, int5, int6, int7, int8, int9 }); }
                        *budget -= 1;
                        let int10 = int7 + int9;
                        if int10 < i128::from(i64::MIN) || int10 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(8)), point: data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 12,
                            ints: 11,
                            bools: 1,
                            bit_arrays: 0,
                            int_lists: 0,
                            strings: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into()], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point12 { int0, int1, int2, int3, int4, bool0, int5, int6, int7, int8, int9, int10 }); }
                        *budget -= 1;
                        FunctionStep::Int { value: int10, exit: data::graph::BlockGraphExitId(0) }
                    },
                    FunctionState::Int8Point12 { int0, int1, int2, int3, int4, bool0, int5, int6, int7, int8, int9, int10 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point12 { int0, int1, int2, int3, int4, bool0, int5, int6, int7, int8, int9, int10 }); }
                        *budget -= 1;
                        FunctionStep::Int { value: int10, exit: data::graph::BlockGraphExitId(0) }
                    },
                    FunctionState::Int9Point0 { int0, bool0 } => {
                        let values = ops.numeric();
                        let progress = numeric_int_9_entry((int0, bool0,), values, budget);
                        calls_int_9_numeric(progress, values)
                    },
                    FunctionState::Int9Point1 { int0, bool0 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[bool0]);
                        let progress = numeric_int_9(1, values, budget);
                        calls_int_9_numeric(progress, values)
                    },
                    FunctionState::Int9Point2 { int0 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_9(2, values, budget);
                        calls_int_9_numeric(progress, values)
                    },
                    FunctionState::Int9Point3 { int0 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_9(3, values, budget);
                        calls_int_9_numeric(progress, values)
                    },
                    FunctionState::Int9Point4 { int0, int1 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_9(4, values, budget);
                        calls_int_9_numeric(progress, values)
                    },
                    FunctionState::Int10Point0 { int0, int1 } => {
                        let values = ops.numeric();
                        let progress = numeric_int_10_entry((int0, int1,), values, budget);
                        calls_int_10_numeric(progress, values)
                    },
                    FunctionState::Int10Point1 { int0, int1 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_10(1, values, budget);
                        calls_int_10_numeric(progress, values)
                    },
                    FunctionState::Int10Point2 { int0, int1, int2 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1, int2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_10(2, values, budget);
                        calls_int_10_numeric(progress, values)
                    },
                    FunctionState::Int10Point3 { int0, int1 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_10(3, values, budget);
                        calls_int_10_numeric(progress, values)
                    },
                    FunctionState::Int10Point4 { int0, int1, int2 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0, int1, int2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_int_10(4, values, budget);
                        calls_int_10_numeric(progress, values)
                    },
                    FunctionState::Bool0Point0 { int0, bool0 } => {
                        let values = ops.numeric();
                        let progress = numeric_bool_0_entry((int0, bool0,), values, budget);
                        calls_bool_0_numeric(progress, values)
                    },
                    FunctionState::Bool0Point1 { bool0 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[bool0]);
                        let progress = numeric_bool_0(1, values, budget);
                        calls_bool_0_numeric(progress, values)
                    },
                    FunctionState::Bool0Point2 { bool0, bool1 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[bool0, bool1]);
                        let progress = numeric_bool_0(2, values, budget);
                        calls_bool_0_numeric(progress, values)
                    },
                    FunctionState::Bool0Point3 { int0, bool0 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[int0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[bool0]);
                        let progress = numeric_bool_0(3, values, budget);
                        calls_bool_0_numeric(progress, values)
                    },
                    FunctionState::Bool0Point4 { bool0 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[bool0]);
                        let progress = numeric_bool_0(4, values, budget);
                        calls_bool_0_numeric(progress, values)
                    },
                    FunctionState::Bool0Point5 {  } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        let progress = numeric_bool_0(5, values, budget);
                        calls_bool_0_numeric(progress, values)
                    },
                    FunctionState::Bool0Point6 { bool0 } => {
                        let values = ops.numeric();
                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[bool0]);
                        let progress = numeric_bool_0(6, values, budget);
                        calls_bool_0_numeric(progress, values)
                    },
                }
            }
            fn calls_entry_0(target: data::function::IntFunctionId, captures: &CallCaptureInputs<'_>, inputs: (i128,)) -> Option<FunctionState> {
                let (argument0,) = inputs;
                match target.0 {
                    2 => Some(FunctionState::Int2Point0 { int0: argument0 }),
                    10 => Some(FunctionState::Int10Point0 { int0: argument0, int1: captures.int(data::graph::IntLocalId(1))? }),
                    _ => None,
                }
            }
            fn calls_int_0_numeric(progress: data::compiled::CompiledProgress, values: &data::compiled::numeric::NumericValues) -> FunctionStep {
                const STATES: [fn(&data::compiled::numeric::NumericValues) -> FunctionState; 8] = [
                    |values| FunctionState::Int0Point0 { int0: values.ints[0], int1: values.ints[1] },
                    |values| FunctionState::Int0Point1 { int0: values.ints[0] },
                    |values| FunctionState::Int0Point2 { int0: values.ints[0], int1: values.ints[1] },
                    |values| FunctionState::Int0Point3 { int0: values.ints[0], int1: values.ints[1], int2: values.ints[2] },
                    |values| FunctionState::Int0Point4 { int0: values.ints[0], int1: values.ints[1] },
                    |values| FunctionState::Int0Point5 { int0: values.ints[0], int1: values.ints[1], int2: values.ints[2], int3: values.ints[3] },
                    |values| FunctionState::Int0Point6 { int0: values.ints[0], int1: values.ints[1] },
                    |values| FunctionState::Int0Point7 { int0: values.ints[0], int1: values.ints[1], int2: values.ints[2], int3: values.ints[3] },
                ];
                const RETURNS: [fn(data::graph::BlockGraphExitId, &data::compiled::numeric::NumericValues) -> FunctionStep; 1] = [
                    |exit, values| FunctionStep::Int { value: values.ints[data::graph::IntLocalId(0).0], exit },
                ];
                match progress {
                    data::compiled::CompiledProgress::Yield(point) => FunctionStep::Yield(STATES[point](values)),
                    data::compiled::CompiledProgress::Interpreted(point) => {
                        const POINTS: [data::compiled::CompiledCheckpoint; 8] = [
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
                                block: data::graph::BlockId(1),
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
                                block: data::graph::BlockId(2),
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
                                block: data::graph::BlockId(4),
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
                                block: data::graph::BlockId(4),
                                instruction: 1,
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
                        ];
                        FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: POINTS[point], values: CallValues { ints: values.ints.iter().copied().map(Into::into).collect(), bools: values.bools.clone(), int_lists: Vec::new(), int_functions: Vec::new(), bool_functions: Vec::new() } }
                    },
                    data::compiled::CompiledProgress::Complete(exit) => RETURNS[exit.0](exit, values),
                }
            }
            fn calls_int_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                let active = match point {
                    0 => FunctionState::Int0Point0 { int0: values.int(0)?, int1: values.int(1)? },
                    1 => FunctionState::Int0Point1 { int0: values.int(0)? },
                    2 => FunctionState::Int0Point2 { int0: values.int(0)?, int1: values.int(1)? },
                    3 => FunctionState::Int0Point3 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                    4 => FunctionState::Int0Point4 { int0: values.int(0)?, int1: values.int(1)? },
                    5 => FunctionState::Int0Point5 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)? },
                    6 => FunctionState::Int0Point6 { int0: values.int(0)?, int1: values.int(1)? },
                    7 => FunctionState::Int0Point7 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)? },
                    _ => return None,
                };
                Some(active)
            }
            fn calls_int_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point, values) { return Some(execution); }
                let active = calls_int_0_state(point, values)?;
                Some(Box::new(FunctionExecution::new(active)))
            }
            fn calls_int_1_numeric(progress: data::compiled::CompiledProgress, values: &data::compiled::numeric::NumericValues) -> FunctionStep {
                const STATES: [fn(&data::compiled::numeric::NumericValues) -> FunctionState; 9] = [
                    |values| FunctionState::Int1Point0 { int0: values.ints[0], int1: values.ints[1], int2: values.ints[2], bool0: values.bools[0] },
                    |values| FunctionState::Int1Point1 { int0: values.ints[0], int1: values.ints[1], bool0: values.bools[0] },
                    |values| FunctionState::Int1Point2 { int0: values.ints[0], int1: values.ints[1] },
                    |values| FunctionState::Int1Point3 { int0: values.ints[0], int1: values.ints[1], int2: values.ints[2] },
                    |values| FunctionState::Int1Point4 { int0: values.ints[0], int1: values.ints[1] },
                    |values| FunctionState::Int1Point5 { int0: values.ints[0], int1: values.ints[1], int2: values.ints[2] },
                    |values| FunctionState::Int1Point6 { int0: values.ints[0], int1: values.ints[1], int2: values.ints[2], bool0: values.bools[0] },
                    |values| FunctionState::Int1Point7 { int0: values.ints[0], int1: values.ints[1], int2: values.ints[2], int3: values.ints[3], bool0: values.bools[0] },
                    |values| FunctionState::Int1Point8 { int0: values.ints[0], int1: values.ints[1], int2: values.ints[2], int3: values.ints[3], bool0: values.bools[0], bool1: values.bools[1] },
                ];
                const RETURNS: [fn(data::graph::BlockGraphExitId, &data::compiled::numeric::NumericValues) -> FunctionStep; 2] = [
                    |exit, values| FunctionStep::Int { value: values.ints[data::graph::IntLocalId(2).0], exit },
                    |exit, values| FunctionStep::Int { value: values.ints[data::graph::IntLocalId(2).0], exit },
                ];
                match progress {
                    data::compiled::CompiledProgress::Yield(point) => FunctionStep::Yield(STATES[point](values)),
                    data::compiled::CompiledProgress::Interpreted(point) => {
                        const POINTS: [data::compiled::CompiledCheckpoint; 9] = [
                            data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 0,
                                ints: 3,
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
                                block: data::graph::BlockId(1),
                                instruction: 0,
                                ints: 2,
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
                                block: data::graph::BlockId(2),
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
                                block: data::graph::BlockId(2),
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
                                ints: 3,
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
                                instruction: 1,
                                ints: 4,
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
                                instruction: 2,
                                ints: 4,
                                bools: 2,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 0,
                            },
                        ];
                        FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(1)), point: POINTS[point], values: CallValues { ints: values.ints.iter().copied().map(Into::into).collect(), bools: values.bools.clone(), int_lists: Vec::new(), int_functions: Vec::new(), bool_functions: Vec::new() } }
                    },
                    data::compiled::CompiledProgress::Complete(exit) => RETURNS[exit.0](exit, values),
                }
            }
            fn calls_int_1_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                let active = match point {
                    0 => FunctionState::Int1Point0 { int0: values.int(0)?, int1: values.int(1)?, bool0: values.bool(0)?, int2: values.int(2)? },
                    1 => FunctionState::Int1Point1 { int0: values.int(0)?, bool0: values.bool(0)?, int1: values.int(1)? },
                    2 => FunctionState::Int1Point2 { int0: values.int(0)?, int1: values.int(1)? },
                    3 => FunctionState::Int1Point3 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                    4 => FunctionState::Int1Point4 { int0: values.int(0)?, int1: values.int(1)? },
                    5 => FunctionState::Int1Point5 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                    6 => FunctionState::Int1Point6 { int0: values.int(0)?, int1: values.int(1)?, bool0: values.bool(0)?, int2: values.int(2)? },
                    7 => FunctionState::Int1Point7 { int0: values.int(0)?, int1: values.int(1)?, bool0: values.bool(0)?, int2: values.int(2)?, int3: values.int(3)? },
                    8 => FunctionState::Int1Point8 { int0: values.int(0)?, int1: values.int(1)?, bool0: values.bool(0)?, int2: values.int(2)?, int3: values.int(3)?, bool1: values.bool(1)? },
                    _ => return None,
                };
                Some(active)
            }
            fn calls_int_1_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(1)), point, values) { return Some(execution); }
                let active = calls_int_1_state(point, values)?;
                Some(Box::new(FunctionExecution::new(active)))
            }
            fn calls_int_2_numeric(progress: data::compiled::CompiledProgress, values: &data::compiled::numeric::NumericValues) -> FunctionStep {
                const STATES: [fn(&data::compiled::numeric::NumericValues) -> FunctionState; 11] = [
                    |values| FunctionState::Int2Point0 { int0: values.ints[0] },
                    |_values| FunctionState::Int2Point1 {  },
                    |values| FunctionState::Int2Point2 { int0: values.ints[0] },
                    |values| FunctionState::Int2Point3 { int0: values.ints[0] },
                    |values| FunctionState::Int2Point4 { int0: values.ints[0] },
                    |values| FunctionState::Int2Point5 { int0: values.ints[0], int1: values.ints[1] },
                    |values| FunctionState::Int2Point6 { int0: values.ints[0] },
                    |values| FunctionState::Int2Point7 { int0: values.ints[0], int1: values.ints[1] },
                    |_values| FunctionState::Int2Point8 {  },
                    |values| FunctionState::Int2Point9 { int0: values.ints[0] },
                    |values| FunctionState::Int2Point10 { int0: values.ints[0] },
                ];
                const RETURNS: [fn(data::graph::BlockGraphExitId, &data::compiled::numeric::NumericValues) -> FunctionStep; 2] = [
                    |exit, values| FunctionStep::Int { value: values.ints[data::graph::IntLocalId(1).0], exit },
                    |exit, values| FunctionStep::Int { value: values.ints[data::graph::IntLocalId(1).0], exit },
                ];
                match progress {
                    data::compiled::CompiledProgress::Yield(point) => FunctionStep::Yield(STATES[point](values)),
                    data::compiled::CompiledProgress::Interpreted(point) => {
                        const POINTS: [data::compiled::CompiledCheckpoint; 11] = [
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
                        ];
                        FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: POINTS[point], values: CallValues { ints: values.ints.iter().copied().map(Into::into).collect(), bools: values.bools.clone(), int_lists: Vec::new(), int_functions: Vec::new(), bool_functions: Vec::new() } }
                    },
                    data::compiled::CompiledProgress::Complete(exit) => RETURNS[exit.0](exit, values),
                }
            }
            fn calls_int_2_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                let active = match point {
                    0 => FunctionState::Int2Point0 { int0: values.int(0)? },
                    1 => FunctionState::Int2Point1 {  },
                    2 => FunctionState::Int2Point2 { int0: values.int(0)? },
                    3 => FunctionState::Int2Point3 { int0: values.int(0)? },
                    4 => FunctionState::Int2Point4 { int0: values.int(0)? },
                    5 => FunctionState::Int2Point5 { int0: values.int(0)?, int1: values.int(1)? },
                    6 => FunctionState::Int2Point6 { int0: values.int(0)? },
                    7 => FunctionState::Int2Point7 { int0: values.int(0)?, int1: values.int(1)? },
                    8 => FunctionState::Int2Point8 {  },
                    9 => FunctionState::Int2Point9 { int0: values.int(0)? },
                    10 => FunctionState::Int2Point10 { int0: values.int(0)? },
                    _ => return None,
                };
                Some(active)
            }
            fn calls_int_2_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point, values) { return Some(execution); }
                let active = calls_int_2_state(point, values)?;
                Some(Box::new(FunctionExecution::new(active)))
            }
            fn calls_int_3_numeric(progress: data::compiled::CompiledProgress, values: &data::compiled::numeric::NumericValues) -> FunctionStep {
                const STATES: [fn(&data::compiled::numeric::NumericValues) -> FunctionState; 5] = [
                    |values| FunctionState::Int3Point0 { int0: values.ints[0], int1: values.ints[1], bool0: values.bools[0] },
                    |values| FunctionState::Int3Point1 { int0: values.ints[0], int1: values.ints[1], int2: values.ints[2], bool0: values.bools[0] },
                    |values| FunctionState::Int3Point2 { int0: values.ints[0] },
                    |values| FunctionState::Int3Point3 { int0: values.ints[0], int1: values.ints[1] },
                    |values| FunctionState::Int3Point4 { int0: values.ints[0] },
                ];
                const RETURNS: [fn(data::graph::BlockGraphExitId, &data::compiled::numeric::NumericValues) -> FunctionStep; 2] = [
                    |exit, values| FunctionStep::Int { value: values.ints[data::graph::IntLocalId(1).0], exit },
                    |exit, values| FunctionStep::Int { value: values.ints[data::graph::IntLocalId(0).0], exit },
                ];
                match progress {
                    data::compiled::CompiledProgress::Yield(point) => FunctionStep::Yield(STATES[point](values)),
                    data::compiled::CompiledProgress::Interpreted(point) => {
                        const POINTS: [data::compiled::CompiledCheckpoint; 5] = [
                            data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 0,
                                ints: 2,
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
                                instruction: 1,
                                ints: 3,
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
                                block: data::graph::BlockId(1),
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
                                block: data::graph::BlockId(1),
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
                        ];
                        FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point: POINTS[point], values: CallValues { ints: values.ints.iter().copied().map(Into::into).collect(), bools: values.bools.clone(), int_lists: Vec::new(), int_functions: Vec::new(), bool_functions: Vec::new() } }
                    },
                    data::compiled::CompiledProgress::Complete(exit) => RETURNS[exit.0](exit, values),
                }
            }
            fn calls_int_3_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                let active = match point {
                    0 => FunctionState::Int3Point0 { int0: values.int(0)?, int1: values.int(1)?, bool0: values.bool(0)? },
                    1 => FunctionState::Int3Point1 { int0: values.int(0)?, int1: values.int(1)?, bool0: values.bool(0)?, int2: values.int(2)? },
                    2 => FunctionState::Int3Point2 { int0: values.int(0)? },
                    3 => FunctionState::Int3Point3 { int0: values.int(0)?, int1: values.int(1)? },
                    4 => FunctionState::Int3Point4 { int0: values.int(0)? },
                    _ => return None,
                };
                Some(active)
            }
            fn calls_int_3_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point, values) { return Some(execution); }
                let active = calls_int_3_state(point, values)?;
                Some(Box::new(FunctionExecution::new(active)))
            }
            fn calls_int_4_numeric(progress: data::compiled::CompiledProgress, values: &data::compiled::numeric::NumericValues) -> FunctionStep {
                const STATES: [fn(&data::compiled::numeric::NumericValues) -> FunctionState; 6] = [
                    |values| FunctionState::Int4Point0 { int0: values.ints[0], int1: values.ints[1], bool0: values.bools[0] },
                    |values| FunctionState::Int4Point1 { int0: values.ints[0], int1: values.ints[1], int2: values.ints[2], bool0: values.bools[0] },
                    |values| FunctionState::Int4Point2 { int0: values.ints[0], int1: values.ints[1] },
                    |values| FunctionState::Int4Point3 { int0: values.ints[0], int1: values.ints[1], int2: values.ints[2] },
                    |values| FunctionState::Int4Point4 { int0: values.ints[0], int1: values.ints[1] },
                    |values| FunctionState::Int4Point5 { int0: values.ints[0], int1: values.ints[1], int2: values.ints[2] },
                ];
                const RETURNS: [fn(data::graph::BlockGraphExitId, &data::compiled::numeric::NumericValues) -> FunctionStep; 2] = [
                    |exit, values| FunctionStep::Int { value: values.ints[data::graph::IntLocalId(2).0], exit },
                    |exit, values| FunctionStep::Int { value: values.ints[data::graph::IntLocalId(2).0], exit },
                ];
                match progress {
                    data::compiled::CompiledProgress::Yield(point) => FunctionStep::Yield(STATES[point](values)),
                    data::compiled::CompiledProgress::Interpreted(point) => {
                        const POINTS: [data::compiled::CompiledCheckpoint; 6] = [
                            data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 0,
                                ints: 2,
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
                                instruction: 1,
                                ints: 3,
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
                                block: data::graph::BlockId(2),
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
                        ];
                        FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(4)), point: POINTS[point], values: CallValues { ints: values.ints.iter().copied().map(Into::into).collect(), bools: values.bools.clone(), int_lists: Vec::new(), int_functions: Vec::new(), bool_functions: Vec::new() } }
                    },
                    data::compiled::CompiledProgress::Complete(exit) => RETURNS[exit.0](exit, values),
                }
            }
            fn calls_int_4_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                let active = match point {
                    0 => FunctionState::Int4Point0 { int0: values.int(0)?, int1: values.int(1)?, bool0: values.bool(0)? },
                    1 => FunctionState::Int4Point1 { int0: values.int(0)?, int1: values.int(1)?, bool0: values.bool(0)?, int2: values.int(2)? },
                    2 => FunctionState::Int4Point2 { int0: values.int(0)?, int1: values.int(1)? },
                    3 => FunctionState::Int4Point3 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                    4 => FunctionState::Int4Point4 { int0: values.int(0)?, int1: values.int(1)? },
                    5 => FunctionState::Int4Point5 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                    _ => return None,
                };
                Some(active)
            }
            fn calls_int_4_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(4)), point, values) { return Some(execution); }
                let active = calls_int_4_state(point, values)?;
                Some(Box::new(FunctionExecution::new(active)))
            }
            fn calls_int_5_numeric(progress: data::compiled::CompiledProgress, values: &data::compiled::numeric::NumericValues) -> FunctionStep {
                const STATES: [fn(&data::compiled::numeric::NumericValues) -> FunctionState; 5] = [
                    |values| FunctionState::Int5Point0 { int0: values.ints[0], int1: values.ints[1], bool0: values.bools[0] },
                    |values| FunctionState::Int5Point1 { int0: values.ints[0], int1: values.ints[1], int2: values.ints[2], bool0: values.bools[0] },
                    |values| FunctionState::Int5Point2 { int0: values.ints[0] },
                    |values| FunctionState::Int5Point3 { int0: values.ints[0] },
                    |values| FunctionState::Int5Point4 { int0: values.ints[0], int1: values.ints[1] },
                ];
                const RETURNS: [fn(data::graph::BlockGraphExitId, &data::compiled::numeric::NumericValues) -> FunctionStep; 2] = [
                    |exit, values| FunctionStep::Int { value: values.ints[data::graph::IntLocalId(0).0], exit },
                    |exit, values| FunctionStep::Int { value: values.ints[data::graph::IntLocalId(1).0], exit },
                ];
                match progress {
                    data::compiled::CompiledProgress::Yield(point) => FunctionStep::Yield(STATES[point](values)),
                    data::compiled::CompiledProgress::Interpreted(point) => {
                        const POINTS: [data::compiled::CompiledCheckpoint; 5] = [
                            data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 0,
                                ints: 2,
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
                                instruction: 1,
                                ints: 3,
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
                                block: data::graph::BlockId(1),
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
                        ];
                        FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(5)), point: POINTS[point], values: CallValues { ints: values.ints.iter().copied().map(Into::into).collect(), bools: values.bools.clone(), int_lists: Vec::new(), int_functions: Vec::new(), bool_functions: Vec::new() } }
                    },
                    data::compiled::CompiledProgress::Complete(exit) => RETURNS[exit.0](exit, values),
                }
            }
            fn calls_int_5_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                let active = match point {
                    0 => FunctionState::Int5Point0 { int0: values.int(0)?, int1: values.int(1)?, bool0: values.bool(0)? },
                    1 => FunctionState::Int5Point1 { int0: values.int(0)?, int1: values.int(1)?, bool0: values.bool(0)?, int2: values.int(2)? },
                    2 => FunctionState::Int5Point2 { int0: values.int(0)? },
                    3 => FunctionState::Int5Point3 { int0: values.int(0)? },
                    4 => FunctionState::Int5Point4 { int0: values.int(0)?, int1: values.int(1)? },
                    _ => return None,
                };
                Some(active)
            }
            fn calls_int_5_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(5)), point, values) { return Some(execution); }
                let active = calls_int_5_state(point, values)?;
                Some(Box::new(FunctionExecution::new(active)))
            }
            fn calls_int_6_numeric(progress: data::compiled::CompiledProgress, values: &data::compiled::numeric::NumericValues) -> FunctionStep {
                const STATES: [fn(&data::compiled::numeric::NumericValues) -> FunctionState; 9] = [
                    |values| FunctionState::Int6Point0 { int0: values.ints[0], int1: values.ints[1] },
                    |values| FunctionState::Int6Point1 { int0: values.ints[0], int1: values.ints[1] },
                    |values| FunctionState::Int6Point2 { int0: values.ints[0], int1: values.ints[1], bool0: values.bools[0] },
                    |values| FunctionState::Int6Point3 { int0: values.ints[0], int1: values.ints[1], bool0: values.bools[0] },
                    |values| FunctionState::Int6Point4 { int0: values.ints[0], int1: values.ints[1] },
                    |values| FunctionState::Int6Point5 { int0: values.ints[0], int1: values.ints[1], int2: values.ints[2] },
                    |values| FunctionState::Int6Point6 { int0: values.ints[0] },
                    |values| FunctionState::Int6Point7 { int0: values.ints[0], int1: values.ints[1] },
                    |values| FunctionState::Int6Point8 { int0: values.ints[0], int1: values.ints[1], bool0: values.bools[0] },
                ];
                const RETURNS: [fn(data::graph::BlockGraphExitId, &data::compiled::numeric::NumericValues) -> FunctionStep; 2] = [
                    |exit, values| FunctionStep::Int { value: values.ints[data::graph::IntLocalId(2).0], exit },
                    |exit, values| FunctionStep::Int { value: values.ints[data::graph::IntLocalId(0).0], exit },
                ];
                match progress {
                    data::compiled::CompiledProgress::Yield(point) => FunctionStep::Yield(STATES[point](values)),
                    data::compiled::CompiledProgress::Interpreted(point) => {
                        const POINTS: [data::compiled::CompiledCheckpoint; 9] = [
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
                                ints: 2,
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
                                block: data::graph::BlockId(2),
                                instruction: 0,
                                ints: 2,
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
                            data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(5),
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
                                block: data::graph::BlockId(5),
                                instruction: 1,
                                ints: 2,
                                bools: 1,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 0,
                            },
                        ];
                        FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(6)), point: POINTS[point], values: CallValues { ints: values.ints.iter().copied().map(Into::into).collect(), bools: values.bools.clone(), int_lists: Vec::new(), int_functions: Vec::new(), bool_functions: Vec::new() } }
                    },
                    data::compiled::CompiledProgress::Complete(exit) => RETURNS[exit.0](exit, values),
                }
            }
            fn calls_int_6_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                let active = match point {
                    0 => FunctionState::Int6Point0 { int0: values.int(0)?, int1: values.int(1)? },
                    1 => FunctionState::Int6Point1 { int0: values.int(0)?, int1: values.int(1)? },
                    2 => FunctionState::Int6Point2 { int0: values.int(0)?, int1: values.int(1)?, bool0: values.bool(0)? },
                    3 => FunctionState::Int6Point3 { bool0: values.bool(0)?, int0: values.int(0)?, int1: values.int(1)? },
                    4 => FunctionState::Int6Point4 { int0: values.int(0)?, int1: values.int(1)? },
                    5 => FunctionState::Int6Point5 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                    6 => FunctionState::Int6Point6 { int0: values.int(0)? },
                    7 => FunctionState::Int6Point7 { int0: values.int(0)?, int1: values.int(1)? },
                    8 => FunctionState::Int6Point8 { int0: values.int(0)?, int1: values.int(1)?, bool0: values.bool(0)? },
                    _ => return None,
                };
                Some(active)
            }
            fn calls_int_6_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(6)), point, values) { return Some(execution); }
                let active = calls_int_6_state(point, values)?;
                Some(Box::new(FunctionExecution::new(active)))
            }
            fn calls_int_7_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                let active = match point {
                    0 => FunctionState::Int7Point0 { int0: values.int(0)?, int1: values.int(1)? },
                    1 => {
                        if !matches!(values.int_function_target(0)?, data::function::IntFunctionId(2) | data::function::IntFunctionId(10)) { return None; }
                        FunctionState::Int7Point1 { int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)? }
                    },
                    2 => FunctionState::Int7Point2 { int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)?, int2: values.int(2)? },
                    _ => return None,
                };
                Some(active)
            }
            fn calls_int_7_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(7)), point, values) { return Some(execution); }
                let active = calls_int_7_state(point, values)?;
                Some(Box::new(FunctionExecution::new(active)))
            }
            fn calls_int_8_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                let active = match point {
                    0 => FunctionState::Int8Point0 {  },
                    1 => FunctionState::Int8Point1 { int0: values.int(0)? },
                    2 => FunctionState::Int8Point2 { int0: values.int(0)?, int1: values.int(1)? },
                    3 => FunctionState::Int8Point3 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                    4 => FunctionState::Int8Point4 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)? },
                    5 => FunctionState::Int8Point5 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)? },
                    6 => FunctionState::Int8Point6 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, bool0: values.bool(0)? },
                    7 => FunctionState::Int8Point7 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, bool0: values.bool(0)?, int5: values.int(5)? },
                    8 => FunctionState::Int8Point8 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, bool0: values.bool(0)?, int5: values.int(5)?, int6: values.int(6)? },
                    9 => FunctionState::Int8Point9 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, bool0: values.bool(0)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)? },
                    10 => FunctionState::Int8Point10 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, bool0: values.bool(0)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)? },
                    11 => FunctionState::Int8Point11 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, bool0: values.bool(0)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)? },
                    12 => FunctionState::Int8Point12 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, int4: values.int(4)?, bool0: values.bool(0)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)? },
                    _ => return None,
                };
                Some(active)
            }
            fn calls_int_8_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(8)), point, values) { return Some(execution); }
                let active = calls_int_8_state(point, values)?;
                Some(Box::new(FunctionExecution::new(active)))
            }
            fn calls_int_9_numeric(progress: data::compiled::CompiledProgress, values: &data::compiled::numeric::NumericValues) -> FunctionStep {
                const STATES: [fn(&data::compiled::numeric::NumericValues) -> FunctionState; 5] = [
                    |values| FunctionState::Int9Point0 { int0: values.ints[0], bool0: values.bools[0] },
                    |values| FunctionState::Int9Point1 { int0: values.ints[0], bool0: values.bools[0] },
                    |values| FunctionState::Int9Point2 { int0: values.ints[0] },
                    |values| FunctionState::Int9Point3 { int0: values.ints[0] },
                    |values| FunctionState::Int9Point4 { int0: values.ints[0], int1: values.ints[1] },
                ];
                const RETURNS: [fn(data::graph::BlockGraphExitId, &data::compiled::numeric::NumericValues) -> FunctionStep; 2] = [
                    |exit, values| FunctionStep::Int { value: values.ints[data::graph::IntLocalId(0).0], exit },
                    |exit, values| FunctionStep::Int { value: values.ints[data::graph::IntLocalId(1).0], exit },
                ];
                match progress {
                    data::compiled::CompiledProgress::Yield(point) => FunctionStep::Yield(STATES[point](values)),
                    data::compiled::CompiledProgress::Interpreted(point) => {
                        const POINTS: [data::compiled::CompiledCheckpoint; 5] = [
                            data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 0,
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
                            data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(1),
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
                        ];
                        FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(9)), point: POINTS[point], values: CallValues { ints: values.ints.iter().copied().map(Into::into).collect(), bools: values.bools.clone(), int_lists: Vec::new(), int_functions: Vec::new(), bool_functions: Vec::new() } }
                    },
                    data::compiled::CompiledProgress::Complete(exit) => RETURNS[exit.0](exit, values),
                }
            }
            fn calls_int_9_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                let active = match point {
                    0 => FunctionState::Int9Point0 { int0: values.int(0)?, bool0: values.bool(0)? },
                    1 => FunctionState::Int9Point1 { int0: values.int(0)?, bool0: values.bool(0)? },
                    2 => FunctionState::Int9Point2 { int0: values.int(0)? },
                    3 => FunctionState::Int9Point3 { int0: values.int(0)? },
                    4 => FunctionState::Int9Point4 { int0: values.int(0)?, int1: values.int(1)? },
                    _ => return None,
                };
                Some(active)
            }
            fn calls_int_9_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(9)), point, values) { return Some(execution); }
                let active = calls_int_9_state(point, values)?;
                Some(Box::new(FunctionExecution::new(active)))
            }
            fn calls_int_10_numeric(progress: data::compiled::CompiledProgress, values: &data::compiled::numeric::NumericValues) -> FunctionStep {
                const STATES: [fn(&data::compiled::numeric::NumericValues) -> FunctionState; 5] = [
                    |values| FunctionState::Int10Point0 { int0: values.ints[0], int1: values.ints[1] },
                    |values| FunctionState::Int10Point1 { int0: values.ints[0], int1: values.ints[1] },
                    |values| FunctionState::Int10Point2 { int0: values.ints[0], int1: values.ints[1], int2: values.ints[2] },
                    |values| FunctionState::Int10Point3 { int0: values.ints[0], int1: values.ints[1] },
                    |values| FunctionState::Int10Point4 { int0: values.ints[0], int1: values.ints[1], int2: values.ints[2] },
                ];
                const RETURNS: [fn(data::graph::BlockGraphExitId, &data::compiled::numeric::NumericValues) -> FunctionStep; 2] = [
                    |exit, values| FunctionStep::Int { value: values.ints[data::graph::IntLocalId(2).0], exit },
                    |exit, values| FunctionStep::Int { value: values.ints[data::graph::IntLocalId(2).0], exit },
                ];
                match progress {
                    data::compiled::CompiledProgress::Yield(point) => FunctionStep::Yield(STATES[point](values)),
                    data::compiled::CompiledProgress::Interpreted(point) => {
                        const POINTS: [data::compiled::CompiledCheckpoint; 5] = [
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
                                block: data::graph::BlockId(2),
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
                        ];
                        FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(10)), point: POINTS[point], values: CallValues { ints: values.ints.iter().copied().map(Into::into).collect(), bools: values.bools.clone(), int_lists: Vec::new(), int_functions: Vec::new(), bool_functions: Vec::new() } }
                    },
                    data::compiled::CompiledProgress::Complete(exit) => RETURNS[exit.0](exit, values),
                }
            }
            fn calls_int_10_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                let active = match point {
                    0 => FunctionState::Int10Point0 { int0: values.int(0)?, int1: values.int(1)? },
                    1 => FunctionState::Int10Point1 { int0: values.int(0)?, int1: values.int(1)? },
                    2 => FunctionState::Int10Point2 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                    3 => FunctionState::Int10Point3 { int0: values.int(0)?, int1: values.int(1)? },
                    4 => FunctionState::Int10Point4 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                    _ => return None,
                };
                Some(active)
            }
            fn calls_int_10_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(10)), point, values) { return Some(execution); }
                let active = calls_int_10_state(point, values)?;
                Some(Box::new(FunctionExecution::new(active)))
            }
            fn calls_bool_0_numeric(progress: data::compiled::CompiledProgress, values: &data::compiled::numeric::NumericValues) -> FunctionStep {
                const STATES: [fn(&data::compiled::numeric::NumericValues) -> FunctionState; 7] = [
                    |values| FunctionState::Bool0Point0 { int0: values.ints[0], bool0: values.bools[0] },
                    |values| FunctionState::Bool0Point1 { bool0: values.bools[0] },
                    |values| FunctionState::Bool0Point2 { bool0: values.bools[0], bool1: values.bools[1] },
                    |values| FunctionState::Bool0Point3 { int0: values.ints[0], bool0: values.bools[0] },
                    |values| FunctionState::Bool0Point4 { bool0: values.bools[0] },
                    |_values| FunctionState::Bool0Point5 {  },
                    |values| FunctionState::Bool0Point6 { bool0: values.bools[0] },
                ];
                const RETURNS: [fn(data::graph::BlockGraphExitId, &data::compiled::numeric::NumericValues) -> FunctionStep; 3] = [
                    |exit, values| FunctionStep::Bool { value: values.bools[data::graph::BoolLocalId(1).0], exit },
                    |exit, values| FunctionStep::Bool { value: values.bools[data::graph::BoolLocalId(0).0], exit },
                    |exit, values| FunctionStep::Bool { value: values.bools[data::graph::BoolLocalId(0).0], exit },
                ];
                match progress {
                    data::compiled::CompiledProgress::Yield(point) => FunctionStep::Yield(STATES[point](values)),
                    data::compiled::CompiledProgress::Interpreted(point) => {
                        const POINTS: [data::compiled::CompiledCheckpoint; 7] = [
                            data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 0,
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
                            data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(1),
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
                                block: data::graph::BlockId(1),
                                instruction: 1,
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
                                block: data::graph::BlockId(2),
                                instruction: 0,
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
                            data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(3),
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
                        ];
                        FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: POINTS[point], values: CallValues { ints: values.ints.iter().copied().map(Into::into).collect(), bools: values.bools.clone(), int_lists: Vec::new(), int_functions: Vec::new(), bool_functions: Vec::new() } }
                    },
                    data::compiled::CompiledProgress::Complete(exit) => RETURNS[exit.0](exit, values),
                }
            }
            fn calls_bool_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                let active = match point {
                    0 => FunctionState::Bool0Point0 { int0: values.int(0)?, bool0: values.bool(0)? },
                    1 => FunctionState::Bool0Point1 { bool0: values.bool(0)? },
                    2 => FunctionState::Bool0Point2 { bool0: values.bool(0)?, bool1: values.bool(1)? },
                    3 => FunctionState::Bool0Point3 { int0: values.int(0)?, bool0: values.bool(0)? },
                    4 => FunctionState::Bool0Point4 { bool0: values.bool(0)? },
                    5 => FunctionState::Bool0Point5 {  },
                    6 => FunctionState::Bool0Point6 { bool0: values.bool(0)? },
                    _ => return None,
                };
                Some(active)
            }
            fn calls_bool_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point, values) { return Some(execution); }
                let active = calls_bool_0_state(point, values)?;
                Some(Box::new(FunctionExecution::new(active)))
            }

            fn numeric_int_0(
                point: usize,
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::numeric::NumericValues, &mut usize) -> CompiledResume;
                    8
                ] = [
                    |values, budget| CompiledResume::Exit(numeric_int_0_entry((values.ints[0], values.ints[1],), values, budget)),
                    numeric_int_0_resume_1,
                    numeric_int_0_resume_2,
                    numeric_int_0_resume_3,
                    numeric_int_0_resume_4,
                    numeric_int_0_resume_5,
                    numeric_int_0_resume_6,
                    numeric_int_0_resume_7,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn numeric_int_0_entry(
                inputs: (i128, i128,),
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (mut b0_i0, mut b0_i1,) = inputs;
                'repeat: loop {
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(0);
                    }
                    *budget -= 1;
                    if b0_i0 == 0_i128 {
                        let (b1_i0,) = (b0_i1,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(1);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0));
                    } else {
                        let (b2_i0, b2_i1,) = (b0_i0, b0_i1,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(2);
                        }
                        *budget -= 1;
                        let b2_i2 = if 2_i128 == 0 { 0_i128 } else { b2_i0 % 2_i128 };
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(3);
                        }
                        *budget -= 1;
                        if b2_i2 == 0_i128 {
                            let (b3_i0, b3_i1,) = (b2_i0, b2_i1,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(4);
                            }
                            *budget -= 1;
                            let _r0_n0 = b3_i0 - 1_i128;
                            let _r0_n1 = b3_i0 * 3_i128;
                            let _r0_n2 = b3_i1 + _r0_n1;
                            let _r0_n3 = _r0_n2 + 1_i128;
                            let b3_i2 = _r0_n0;
                            let b3_i3 = _r0_n3;
                            if b3_i2 < i128::from(i64::MIN) || b3_i2 > i128::from(i64::MAX) || b3_i3 < i128::from(i64::MIN) || b3_i3 > i128::from(i64::MAX) {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Interpreted(5);
                            }
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(5);
                            }
                            *budget -= 1;
                            {
                                (b0_i0, b0_i1,) = (b3_i2, b3_i3,);
                                continue 'repeat;
                            }
                        } else {
                            let (b4_i0, b4_i1,) = (b2_i0, b2_i1,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(6);
                            }
                            *budget -= 1;
                            let _r0_n0 = b4_i0 - 1_i128;
                            let _r0_n1 = b4_i0 * 2_i128;
                            let _r0_n2 = b4_i1 + _r0_n1;
                            let _r0_n3 = _r0_n2 - 1_i128;
                            let b4_i2 = _r0_n0;
                            let b4_i3 = _r0_n3;
                            if b4_i2 < i128::from(i64::MIN) || b4_i2 > i128::from(i64::MAX) || b4_i3 < i128::from(i64::MIN) || b4_i3 > i128::from(i64::MAX) {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Interpreted(7);
                            }
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(7);
                            }
                            *budget -= 1;
                            {
                                (b0_i0, b0_i1,) = (b4_i2, b4_i3,);
                                continue 'repeat;
                            }
                        }
                    }
                }
            }

            fn numeric_int_0_resume_1(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn numeric_int_0_resume_2(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                let b2_i2 = if 2_i128 == 0 { 0_i128 } else { b2_i0 % 2_i128 };

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Next(3)
            }

            fn numeric_int_0_resume_3(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_i1, b2_i2,) = (values.ints[0], values.ints[1], values.ints[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                if b2_i2 == 0_i128 {
                    let (b3_i0, b3_i1,) = (b2_i0, b2_i1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(4)
                } else {
                    let (b4_i0, b4_i1,) = (b2_i0, b2_i1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(6)
                }
            }

            fn numeric_int_0_resume_4(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0, b3_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;
                let _r0_n0 = b3_i0 - 1_i128;
                let _r0_n1 = b3_i0 * 3_i128;
                let _r0_n2 = b3_i1 + _r0_n1;
                let _r0_n3 = _r0_n2 + 1_i128;
                let b3_i2 = _r0_n0;
                let b3_i3 = _r0_n3;
                if b3_i2 < i128::from(i64::MIN) || b3_i2 > i128::from(i64::MAX) || b3_i3 < i128::from(i64::MIN) || b3_i3 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(5));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Next(5)
            }

            fn numeric_int_0_resume_5(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0, b3_i1, b3_i2, b3_i3,) = (values.ints[0], values.ints[1], values.ints[2], values.ints[3],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;
                {
                    let (b0_i0, b0_i1,) = (b3_i2, b3_i3,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(0)
                }
            }

            fn numeric_int_0_resume_6(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_i0, b4_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;
                let _r0_n0 = b4_i0 - 1_i128;
                let _r0_n1 = b4_i0 * 2_i128;
                let _r0_n2 = b4_i1 + _r0_n1;
                let _r0_n3 = _r0_n2 - 1_i128;
                let b4_i2 = _r0_n0;
                let b4_i3 = _r0_n3;
                if b4_i2 < i128::from(i64::MIN) || b4_i2 > i128::from(i64::MAX) || b4_i3 < i128::from(i64::MIN) || b4_i3 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(7));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Next(7)
            }

            fn numeric_int_0_resume_7(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_i0, b4_i1, b4_i2, b4_i3,) = (values.ints[0], values.ints[1], values.ints[2], values.ints[3],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                }
                *budget -= 1;
                {
                    let (b0_i0, b0_i1,) = (b4_i2, b4_i3,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(0)
                }
            }

            fn numeric_int_1(
                point: usize,
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::numeric::NumericValues, &mut usize) -> CompiledResume;
                    9
                ] = [
                    |values, budget| CompiledResume::Exit(numeric_int_1_entry((values.ints[0], values.ints[1], values.ints[2], values.bools[0],), values, budget)),
                    numeric_int_1_resume_1,
                    numeric_int_1_resume_2,
                    numeric_int_1_resume_3,
                    numeric_int_1_resume_4,
                    numeric_int_1_resume_5,
                    numeric_int_1_resume_6,
                    numeric_int_1_resume_7,
                    numeric_int_1_resume_8,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn numeric_int_1_entry(
                inputs: (i128, i128, i128, bool,),
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (mut b0_i0, mut b0_i1, mut b0_i2, mut b0_v0,) = inputs;
                'repeat: loop {
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b0_v0]);
                        return data::compiled::CompiledProgress::Yield(0);
                    }
                    *budget -= 1;
                    if b0_i0 <= 0_i128 {
                        let (b1_i0, b1_i1, b1_v0,) = (b0_i1, b0_i2, b0_v0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b1_v0]);
                            return data::compiled::CompiledProgress::Yield(1);
                        }
                        *budget -= 1;
                        if b1_v0 {
                            let (b2_i0, b2_i1,) = (b1_i0, b1_i1,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(2);
                            }
                            *budget -= 1;
                            let b2_i2 = b2_i0 - b2_i1;
                            if b2_i2 < i128::from(i64::MIN) || b2_i2 > i128::from(i64::MAX) {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Interpreted(3);
                            }
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(3);
                            }
                            *budget -= 1;

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0));
                        } else {
                            let (b3_i0, b3_i1,) = (b1_i0, b1_i1,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(4);
                            }
                            *budget -= 1;
                            let b3_i2 = b3_i1 - b3_i0;
                            if b3_i2 < i128::from(i64::MIN) || b3_i2 > i128::from(i64::MAX) {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Interpreted(5);
                            }
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(5);
                            }
                            *budget -= 1;

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1));
                        }
                    } else {
                        let (b4_i0, b4_i1, b4_i2, b4_v0,) = (b0_i0, b0_i1, b0_i2, b0_v0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b4_v0]);
                            return data::compiled::CompiledProgress::Yield(6);
                        }
                        *budget -= 1;
                        let b4_i3 = b4_i0 - 1_i128;
                        if b4_i3 < i128::from(i64::MIN) || b4_i3 > i128::from(i64::MAX) {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b4_v0]);
                            return data::compiled::CompiledProgress::Interpreted(7);
                        }
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b4_v0]);
                            return data::compiled::CompiledProgress::Yield(7);
                        }
                        *budget -= 1;
                        let b4_v1 = !b4_v0;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b4_v0, b4_v1]);
                            return data::compiled::CompiledProgress::Yield(8);
                        }
                        *budget -= 1;
                        {
                            (b0_i0, b0_i1, b0_i2, b0_v0,) = (b4_i3, b4_i2, b4_i1, b4_v1,);
                            continue 'repeat;
                        }
                    }
                }
            }

            fn numeric_int_1_resume_1(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_i1, b1_v0,) = (values.ints[0], values.ints[1], values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                if b1_v0 {
                    let (b2_i0, b2_i1,) = (b1_i0, b1_i1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(2)
                } else {
                    let (b3_i0, b3_i1,) = (b1_i0, b1_i1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(4)
                }
            }

            fn numeric_int_1_resume_2(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                let b2_i2 = b2_i0 - b2_i1;
                if b2_i2 < i128::from(i64::MIN) || b2_i2 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(3));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Next(3)
            }

            fn numeric_int_1_resume_3(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_i1, b2_i2,) = (values.ints[0], values.ints[1], values.ints[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn numeric_int_1_resume_4(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0, b3_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;
                let b3_i2 = b3_i1 - b3_i0;
                if b3_i2 < i128::from(i64::MIN) || b3_i2 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(5));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Next(5)
            }

            fn numeric_int_1_resume_5(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0, b3_i1, b3_i2,) = (values.ints[0], values.ints[1], values.ints[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn numeric_int_1_resume_6(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_i0, b4_i1, b4_i2, b4_v0,) = (values.ints[0], values.ints[1], values.ints[2], values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b4_v0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;
                let b4_i3 = b4_i0 - 1_i128;
                if b4_i3 < i128::from(i64::MIN) || b4_i3 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b4_v0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(7));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b4_v0]);
                CompiledResume::Next(7)
            }

            fn numeric_int_1_resume_7(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_i0, b4_i1, b4_i2, b4_i3, b4_v0,) = (values.ints[0], values.ints[1], values.ints[2], values.ints[3], values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b4_v0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                }
                *budget -= 1;
                let b4_v1 = !b4_v0;

                values.ints.clear();
                values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b4_v0, b4_v1]);
                CompiledResume::Next(8)
            }

            fn numeric_int_1_resume_8(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_i0, b4_i1, b4_i2, b4_i3, b4_v0, b4_v1,) = (values.ints[0], values.ints[1], values.ints[2], values.ints[3], values.bools[0], values.bools[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b4_v0, b4_v1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                }
                *budget -= 1;
                {
                    let (b0_i0, b0_i1, b0_i2, b0_v0,) = (b4_i3, b4_i2, b4_i1, b4_v1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);
                    CompiledResume::Next(0)
                }
            }

            fn numeric_int_2(
                point: usize,
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::numeric::NumericValues, &mut usize) -> CompiledResume;
                    11
                ] = [
                    |values, budget| CompiledResume::Exit(numeric_int_2_entry((values.ints[0],), values, budget)),
                    numeric_int_2_resume_1,
                    numeric_int_2_resume_2,
                    numeric_int_2_resume_3,
                    numeric_int_2_resume_4,
                    numeric_int_2_resume_5,
                    numeric_int_2_resume_6,
                    numeric_int_2_resume_7,
                    numeric_int_2_resume_8,
                    numeric_int_2_resume_9,
                    numeric_int_2_resume_10,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn numeric_int_2_entry(
                inputs: (i128,),
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (b0_i0,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return data::compiled::CompiledProgress::Yield(0);
                }
                *budget -= 1;
                let (b2_i0,) = if b0_i0 == 0_i128 {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(1);
                    }
                    *budget -= 1;
                    let b1_i0 = -3_i128;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;
                    let (b2_i0,) = {
                        (b1_i0,)
                    };
                    (b2_i0,)
                } else if b0_i0 == 1_i128 {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(8);
                    }
                    *budget -= 1;
                    let b5_i0 = 2_i128;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b5_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(9);
                    }
                    *budget -= 1;
                    let (b2_i0,) = {
                        (b5_i0,)
                    };
                    (b2_i0,)
                } else {
                    let (b6_i0,) = (b0_i0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b6_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(10);
                    }
                    *budget -= 1;
                    let (b2_i0,) = {
                        (b6_i0,)
                    };
                    (b2_i0,)
                };
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return data::compiled::CompiledProgress::Yield(3);
                }
                *budget -= 1;
                if b2_i0 > 0_i128 {
                    let (b3_i0,) = (b2_i0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(4);
                    }
                    *budget -= 1;
                    let b3_i1 = b3_i0 + 42_i128;
                    if b3_i1 < i128::from(i64::MIN) || b3_i1 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Interpreted(5);
                    }
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(5);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                } else {
                    let (b4_i0,) = (b2_i0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(6);
                    }
                    *budget -= 1;
                    let b4_i1 = -b4_i0;
                    if b4_i1 < i128::from(i64::MIN) || b4_i1 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Interpreted(7);
                    }
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(7);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                }
            }

            fn numeric_int_2_resume_1(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                let b1_i0 = -3_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Next(2)
            }

            fn numeric_int_2_resume_2(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                {
                    let (b2_i0,) = (b1_i0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(3)
                }
            }

            fn numeric_int_2_resume_3(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                if b2_i0 > 0_i128 {
                    let (b3_i0,) = (b2_i0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(4)
                } else {
                    let (b4_i0,) = (b2_i0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(6)
                }
            }

            fn numeric_int_2_resume_4(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;
                let b3_i1 = b3_i0 + 42_i128;
                if b3_i1 < i128::from(i64::MIN) || b3_i1 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(5));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Next(5)
            }

            fn numeric_int_2_resume_5(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0, b3_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn numeric_int_2_resume_6(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;
                let b4_i1 = -b4_i0;
                if b4_i1 < i128::from(i64::MIN) || b4_i1 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(7));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Next(7)
            }

            fn numeric_int_2_resume_7(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_i0, b4_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn numeric_int_2_resume_8(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                }
                *budget -= 1;
                let b5_i0 = 2_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b5_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Next(9)
            }

            fn numeric_int_2_resume_9(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b5_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(9));
                }
                *budget -= 1;
                {
                    let (b2_i0,) = (b5_i0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(3)
                }
            }

            fn numeric_int_2_resume_10(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b6_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b6_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(10));
                }
                *budget -= 1;
                {
                    let (b2_i0,) = (b6_i0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(3)
                }
            }

            fn numeric_int_3(
                point: usize,
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::numeric::NumericValues, &mut usize) -> CompiledResume;
                    5
                ] = [
                    |values, budget| CompiledResume::Exit(numeric_int_3_entry((values.ints[0], values.ints[1], values.bools[0],), values, budget)),
                    numeric_int_3_resume_1,
                    numeric_int_3_resume_2,
                    numeric_int_3_resume_3,
                    numeric_int_3_resume_4,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn numeric_int_3_entry(
                inputs: (i128, i128, bool,),
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (b0_i0, b0_i1, b0_v0,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);
                    return data::compiled::CompiledProgress::Yield(0);
                }
                *budget -= 1;
                let b0_i2 = if b0_i1 == 0 { 0_i128 } else { b0_i0 / b0_i1 };
                if b0_i2 < i128::from(i64::MIN) || b0_i2 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);
                    return data::compiled::CompiledProgress::Interpreted(1);
                }
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);
                    return data::compiled::CompiledProgress::Yield(1);
                }
                *budget -= 1;
                if b0_v0 {
                    let (b1_i0,) = (b0_i2,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;
                    let b1_i1 = -b1_i0;
                    if b1_i1 < i128::from(i64::MIN) || b1_i1 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Interpreted(3);
                    }
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(3);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                } else {
                    let (b2_i0,) = (b0_i2,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(4);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                }
            }

            fn numeric_int_3_resume_1(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b0_i0, b0_i1, b0_i2, b0_v0,) = (values.ints[0], values.ints[1], values.ints[2], values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                if b0_v0 {
                    let (b1_i0,) = (b0_i2,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(2)
                } else {
                    let (b2_i0,) = (b0_i2,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(4)
                }
            }

            fn numeric_int_3_resume_2(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                let b1_i1 = -b1_i0;
                if b1_i1 < i128::from(i64::MIN) || b1_i1 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(3));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Next(3)
            }

            fn numeric_int_3_resume_3(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn numeric_int_3_resume_4(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn numeric_int_4(
                point: usize,
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::numeric::NumericValues, &mut usize) -> CompiledResume;
                    6
                ] = [
                    |values, budget| CompiledResume::Exit(numeric_int_4_entry((values.ints[0], values.ints[1], values.bools[0],), values, budget)),
                    numeric_int_4_resume_1,
                    numeric_int_4_resume_2,
                    numeric_int_4_resume_3,
                    numeric_int_4_resume_4,
                    numeric_int_4_resume_5,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn numeric_int_4_entry(
                inputs: (i128, i128, bool,),
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (b0_i0, b0_i1, b0_v0,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);
                    return data::compiled::CompiledProgress::Yield(0);
                }
                *budget -= 1;
                let _r0_n0 = b0_i0 * b0_i1;
                let _r0_n1 = _r0_n0 + b0_i0;
                let _r0_n2 = _r0_n1 - b0_i1;
                let _r0_n3 = -_r0_n2;
                let b0_i2 = _r0_n3;
                if b0_i2 < i128::from(i64::MIN) || b0_i2 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);
                    return data::compiled::CompiledProgress::Interpreted(1);
                }
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);
                    return data::compiled::CompiledProgress::Yield(1);
                }
                *budget -= 1;
                if b0_v0 {
                    let (b1_i0, b1_i1,) = (b0_i1, b0_i2,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;
                    let b1_i2 = if b1_i0 == 0 { 0_i128 } else { b1_i1 / b1_i0 };
                    if b1_i2 < i128::from(i64::MIN) || b1_i2 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Interpreted(3);
                    }
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(3);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                } else {
                    let (b2_i0, b2_i1,) = (b0_i1, b0_i2,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(4);
                    }
                    *budget -= 1;
                    let b2_i2 = if b2_i0 == 0 { 0_i128 } else { b2_i1 % b2_i0 };
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(5);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                }
            }

            fn numeric_int_4_resume_1(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b0_i0, b0_i1, b0_i2, b0_v0,) = (values.ints[0], values.ints[1], values.ints[2], values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                if b0_v0 {
                    let (b1_i0, b1_i1,) = (b0_i1, b0_i2,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(2)
                } else {
                    let (b2_i0, b2_i1,) = (b0_i1, b0_i2,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(4)
                }
            }

            fn numeric_int_4_resume_2(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                let b1_i2 = if b1_i0 == 0 { 0_i128 } else { b1_i1 / b1_i0 };
                if b1_i2 < i128::from(i64::MIN) || b1_i2 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(3));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Next(3)
            }

            fn numeric_int_4_resume_3(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_i1, b1_i2,) = (values.ints[0], values.ints[1], values.ints[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn numeric_int_4_resume_4(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;
                let b2_i2 = if b2_i0 == 0 { 0_i128 } else { b2_i1 % b2_i0 };

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Next(5)
            }

            fn numeric_int_4_resume_5(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_i1, b2_i2,) = (values.ints[0], values.ints[1], values.ints[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn numeric_int_5(
                point: usize,
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::numeric::NumericValues, &mut usize) -> CompiledResume;
                    5
                ] = [
                    |values, budget| CompiledResume::Exit(numeric_int_5_entry((values.ints[0], values.ints[1], values.bools[0],), values, budget)),
                    numeric_int_5_resume_1,
                    numeric_int_5_resume_2,
                    numeric_int_5_resume_3,
                    numeric_int_5_resume_4,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn numeric_int_5_entry(
                inputs: (i128, i128, bool,),
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (b0_i0, b0_i1, b0_v0,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);
                    return data::compiled::CompiledProgress::Yield(0);
                }
                *budget -= 1;
                let _r0_n0 = if b0_i1 == 0 { 0_i128 } else { b0_i0 / b0_i1 };
                let _r0_n1 = if b0_i1 == 0 { 0_i128 } else { b0_i0 % b0_i1 };
                let _r0_n2 = _r0_n0 + _r0_n1;
                let b0_i2 = _r0_n2;
                if b0_i2 < i128::from(i64::MIN) || b0_i2 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);
                    return data::compiled::CompiledProgress::Interpreted(1);
                }
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);
                    return data::compiled::CompiledProgress::Yield(1);
                }
                *budget -= 1;
                if b0_v0 {
                    let (b1_i0,) = (b0_i2,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                } else {
                    let (b2_i0,) = (b0_i2,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(3);
                    }
                    *budget -= 1;
                    let b2_i1 = -b2_i0;
                    if b2_i1 < i128::from(i64::MIN) || b2_i1 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Interpreted(4);
                    }
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(4);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                }
            }

            fn numeric_int_5_resume_1(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b0_i0, b0_i1, b0_i2, b0_v0,) = (values.ints[0], values.ints[1], values.ints[2], values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                if b0_v0 {
                    let (b1_i0,) = (b0_i2,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(2)
                } else {
                    let (b2_i0,) = (b0_i2,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(3)
                }
            }

            fn numeric_int_5_resume_2(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn numeric_int_5_resume_3(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                let b2_i1 = -b2_i0;
                if b2_i1 < i128::from(i64::MIN) || b2_i1 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(4));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Next(4)
            }

            fn numeric_int_5_resume_4(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn numeric_int_6(
                point: usize,
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::numeric::NumericValues, &mut usize) -> CompiledResume;
                    9
                ] = [
                    |values, budget| CompiledResume::Exit(numeric_int_6_entry((values.ints[0], values.ints[1],), values, budget)),
                    numeric_int_6_resume_1,
                    numeric_int_6_resume_2,
                    numeric_int_6_resume_3,
                    numeric_int_6_resume_4,
                    numeric_int_6_resume_5,
                    numeric_int_6_resume_6,
                    numeric_int_6_resume_7,
                    numeric_int_6_resume_8,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn numeric_int_6_entry(
                inputs: (i128, i128,),
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (b0_i0, b0_i1,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return data::compiled::CompiledProgress::Yield(0);
                }
                *budget -= 1;
                let (b2_i0, b2_i1, b2_v0,) = if b0_i0 == b0_i1 {
                    let (b1_i0, b1_i1,) = (b0_i0, b0_i1,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(1);
                    }
                    *budget -= 1;
                    let b1_v0 = false;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0]);
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;
                    let (b2_i0, b2_i1, b2_v0,) = {
                        (b1_i0, b1_i1, b1_v0,)
                    };
                    (b2_i0, b2_i1, b2_v0,)
                } else {
                    let (b5_i0, b5_i1,) = (b0_i0, b0_i1,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(7);
                    }
                    *budget -= 1;
                    let b5_v0 = true;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b5_v0]);
                        return data::compiled::CompiledProgress::Yield(8);
                    }
                    *budget -= 1;
                    let (b2_i0, b2_i1, b2_v0,) = {
                        (b5_i0, b5_i1, b5_v0,)
                    };
                    (b2_i0, b2_i1, b2_v0,)
                };
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b2_v0]);
                    return data::compiled::CompiledProgress::Yield(3);
                }
                *budget -= 1;
                if b2_v0 {
                    let (b3_i0, b3_i1,) = (b2_i0, b2_i1,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(4);
                    }
                    *budget -= 1;
                    let b3_i2 = b3_i0 * b3_i1;
                    if b3_i2 < i128::from(i64::MIN) || b3_i2 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Interpreted(5);
                    }
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(5);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                } else {
                    let (b4_i0,) = (b2_i0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(6);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                }
            }

            fn numeric_int_6_resume_1(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                let b1_v0 = false;

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b1_v0]);
                CompiledResume::Next(2)
            }

            fn numeric_int_6_resume_2(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_i1, b1_v0,) = (values.ints[0], values.ints[1], values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                {
                    let (b2_i0, b2_i1, b2_v0,) = (b1_i0, b1_i1, b1_v0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b2_v0]);
                    CompiledResume::Next(3)
                }
            }

            fn numeric_int_6_resume_3(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_i1, b2_v0,) = (values.ints[0], values.ints[1], values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b2_v0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                if b2_v0 {
                    let (b3_i0, b3_i1,) = (b2_i0, b2_i1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(4)
                } else {
                    let (b4_i0,) = (b2_i0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(6)
                }
            }

            fn numeric_int_6_resume_4(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0, b3_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;
                let b3_i2 = b3_i0 * b3_i1;
                if b3_i2 < i128::from(i64::MIN) || b3_i2 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(5));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Next(5)
            }

            fn numeric_int_6_resume_5(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0, b3_i1, b3_i2,) = (values.ints[0], values.ints[1], values.ints[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn numeric_int_6_resume_6(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b4_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn numeric_int_6_resume_7(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b5_i0, b5_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                }
                *budget -= 1;
                let b5_v0 = true;

                values.ints.clear();
                values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b5_v0]);
                CompiledResume::Next(8)
            }

            fn numeric_int_6_resume_8(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b5_i0, b5_i1, b5_v0,) = (values.ints[0], values.ints[1], values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b5_v0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                }
                *budget -= 1;
                {
                    let (b2_i0, b2_i1, b2_v0,) = (b5_i0, b5_i1, b5_v0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b2_v0]);
                    CompiledResume::Next(3)
                }
            }

            fn numeric_int_9(
                point: usize,
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::numeric::NumericValues, &mut usize) -> CompiledResume;
                    5
                ] = [
                    |values, budget| CompiledResume::Exit(numeric_int_9_entry((values.ints[0], values.bools[0],), values, budget)),
                    numeric_int_9_resume_1,
                    numeric_int_9_resume_2,
                    numeric_int_9_resume_3,
                    numeric_int_9_resume_4,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn numeric_int_9_entry(
                inputs: (i128, bool,),
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (b0_i0, b0_v0,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);
                    return data::compiled::CompiledProgress::Yield(0);
                }
                *budget -= 1;
                let _r0_n0 = b0_i0 + 1_i128;
                let _r0_n1 = _r0_n0 + 2_i128;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);
                    return data::compiled::CompiledProgress::Yield(1);
                }
                *budget -= 1;
                if b0_v0 {
                    let (b1_i0,) = (b0_i0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                } else {
                    let (b2_i0,) = (b0_i0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(3);
                    }
                    *budget -= 1;
                    let b2_i1 = -b2_i0;
                    if b2_i1 < i128::from(i64::MIN) || b2_i1 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Interpreted(4);
                    }
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(4);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                }
            }

            fn numeric_int_9_resume_1(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b0_i0, b0_v0,) = (values.ints[0], values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                if b0_v0 {
                    let (b1_i0,) = (b0_i0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(2)
                } else {
                    let (b2_i0,) = (b0_i0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(3)
                }
            }

            fn numeric_int_9_resume_2(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn numeric_int_9_resume_3(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                let b2_i1 = -b2_i0;
                if b2_i1 < i128::from(i64::MIN) || b2_i1 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(4));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Next(4)
            }

            fn numeric_int_9_resume_4(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn numeric_int_10(
                point: usize,
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::numeric::NumericValues, &mut usize) -> CompiledResume;
                    5
                ] = [
                    |values, budget| CompiledResume::Exit(numeric_int_10_entry((values.ints[0], values.ints[1],), values, budget)),
                    numeric_int_10_resume_1,
                    numeric_int_10_resume_2,
                    numeric_int_10_resume_3,
                    numeric_int_10_resume_4,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn numeric_int_10_entry(
                inputs: (i128, i128,),
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (b0_i0, b0_i1,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return data::compiled::CompiledProgress::Yield(0);
                }
                *budget -= 1;
                if b0_i0 >= 0_i128 {
                    let (b1_i0, b1_i1,) = (b0_i0, b0_i1,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(1);
                    }
                    *budget -= 1;
                    let b1_i2 = b1_i0 + b1_i1;
                    if b1_i2 < i128::from(i64::MIN) || b1_i2 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Interpreted(2);
                    }
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                } else {
                    let (b2_i0, b2_i1,) = (b0_i0, b0_i1,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(3);
                    }
                    *budget -= 1;
                    let b2_i2 = b2_i1 - b2_i0;
                    if b2_i2 < i128::from(i64::MIN) || b2_i2 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Interpreted(4);
                    }
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(4);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                }
            }

            fn numeric_int_10_resume_1(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                let b1_i2 = b1_i0 + b1_i1;
                if b1_i2 < i128::from(i64::MIN) || b1_i2 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(2));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Next(2)
            }

            fn numeric_int_10_resume_2(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_i1, b1_i2,) = (values.ints[0], values.ints[1], values.ints[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn numeric_int_10_resume_3(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                let b2_i2 = b2_i1 - b2_i0;
                if b2_i2 < i128::from(i64::MIN) || b2_i2 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(4));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Next(4)
            }

            fn numeric_int_10_resume_4(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_i1, b2_i2,) = (values.ints[0], values.ints[1], values.ints[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn numeric_bool_0(
                point: usize,
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::numeric::NumericValues, &mut usize) -> CompiledResume;
                    7
                ] = [
                    |values, budget| CompiledResume::Exit(numeric_bool_0_entry((values.ints[0], values.bools[0],), values, budget)),
                    numeric_bool_0_resume_1,
                    numeric_bool_0_resume_2,
                    numeric_bool_0_resume_3,
                    numeric_bool_0_resume_4,
                    numeric_bool_0_resume_5,
                    numeric_bool_0_resume_6,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn numeric_bool_0_entry(
                inputs: (i128, bool,),
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (b0_i0, b0_v0,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);
                    return data::compiled::CompiledProgress::Yield(0);
                }
                *budget -= 1;
                if b0_i0 < 0_i128 {
                    let (b1_v0,) = (b0_v0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0]);
                        return data::compiled::CompiledProgress::Yield(1);
                    }
                    *budget -= 1;
                    let b1_v1 = !b1_v0;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0, b1_v1]);
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0, b1_v1]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                } else {
                    let (b2_i0, b2_v0,) = (b0_i0, b0_v0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b2_v0]);
                        return data::compiled::CompiledProgress::Yield(3);
                    }
                    *budget -= 1;
                    if b2_i0 >= 10_i128 {
                        let (b3_v0,) = (b2_v0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b3_v0]);
                            return data::compiled::CompiledProgress::Yield(4);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b3_v0]);
                        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                    } else {
                        let () = ();
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(5);
                        }
                        *budget -= 1;
                        let b4_v0 = false;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b4_v0]);
                            return data::compiled::CompiledProgress::Yield(6);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b4_v0]);
                        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2))
                    }
                }
            }

            fn numeric_bool_0_resume_1(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_v0,) = (values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                let b1_v1 = !b1_v0;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b1_v0, b1_v1]);
                CompiledResume::Next(2)
            }

            fn numeric_bool_0_resume_2(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_v0, b1_v1,) = (values.bools[0], values.bools[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0, b1_v1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b1_v0, b1_v1]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn numeric_bool_0_resume_3(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_v0,) = (values.ints[0], values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b2_v0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                if b2_i0 >= 10_i128 {
                    let (b3_v0,) = (b2_v0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b3_v0]);
                    CompiledResume::Next(4)
                } else {
                    let () = ();

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(5)
                }
            }

            fn numeric_bool_0_resume_4(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_v0,) = (values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b3_v0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b3_v0]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn numeric_bool_0_resume_5(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;
                let b4_v0 = false;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b4_v0]);
                CompiledResume::Next(6)
            }

            fn numeric_bool_0_resume_6(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_v0,) = (values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b4_v0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b4_v0]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2)))
            }
            data::compiled::CompiledFunctions {
                ints: data::Storage::Static(&[
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(0),
                        implementation: data::compiled::CompiledImplementation::Numeric(data::compiled::NumericImplementation {
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
                                    block: data::graph::BlockId(1),
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
                                    block: data::graph::BlockId(2),
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
                                    block: data::graph::BlockId(4),
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
                                    block: data::graph::BlockId(4),
                                    instruction: 1,
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
                            ]),
                            run: numeric_int_0,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(1),
                        implementation: data::compiled::CompiledImplementation::Numeric(data::compiled::NumericImplementation {
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 3,
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
                                    block: data::graph::BlockId(1),
                                    instruction: 0,
                                    ints: 2,
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
                                    block: data::graph::BlockId(2),
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
                                    block: data::graph::BlockId(2),
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
                                    ints: 3,
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
                                    instruction: 1,
                                    ints: 4,
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
                                    instruction: 2,
                                    ints: 4,
                                    bools: 2,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                            ]),
                            run: numeric_int_1,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(2),
                        implementation: data::compiled::CompiledImplementation::Numeric(data::compiled::NumericImplementation {
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
                            run: numeric_int_2,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(3),
                        implementation: data::compiled::CompiledImplementation::Numeric(data::compiled::NumericImplementation {
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 2,
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
                                    instruction: 1,
                                    ints: 3,
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
                                    block: data::graph::BlockId(1),
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
                                    block: data::graph::BlockId(1),
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
                            run: numeric_int_3,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(4),
                        implementation: data::compiled::CompiledImplementation::Numeric(data::compiled::NumericImplementation {
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 2,
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
                                    instruction: 1,
                                    ints: 3,
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
                                    block: data::graph::BlockId(2),
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
                            run: numeric_int_4,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(5),
                        implementation: data::compiled::CompiledImplementation::Numeric(data::compiled::NumericImplementation {
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 2,
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
                                    instruction: 1,
                                    ints: 3,
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
                                    block: data::graph::BlockId(1),
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
                            ]),
                            run: numeric_int_5,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(6),
                        implementation: data::compiled::CompiledImplementation::Numeric(data::compiled::NumericImplementation {
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
                                    ints: 2,
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
                                    block: data::graph::BlockId(2),
                                    instruction: 0,
                                    ints: 2,
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
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(5),
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
                                    block: data::graph::BlockId(5),
                                    instruction: 1,
                                    ints: 2,
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
                            run: numeric_int_6,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(9),
                        implementation: data::compiled::CompiledImplementation::Numeric(data::compiled::NumericImplementation {
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
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
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
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
                            ]),
                            run: numeric_int_9,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(10),
                        implementation: data::compiled::CompiledImplementation::Numeric(data::compiled::NumericImplementation {
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
                                    block: data::graph::BlockId(2),
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
                            run: numeric_int_10,
                        }),
                    },
                ]),
                bools: data::Storage::Static(&[
                    data::compiled::CompiledFunction {
                        function: data::function::BoolFunctionId(0),
                        implementation: data::compiled::CompiledImplementation::Numeric(data::compiled::NumericImplementation {
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
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
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
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
                                    block: data::graph::BlockId(1),
                                    instruction: 1,
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
                                    block: data::graph::BlockId(2),
                                    instruction: 0,
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
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(3),
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
                            run: numeric_bool_0,
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
                                    block: data::graph::BlockId(1),
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
                                    block: data::graph::BlockId(2),
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
                                    block: data::graph::BlockId(4),
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
                                    block: data::graph::BlockId(4),
                                    instruction: 1,
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
                            ]),
                            locals: data::Storage::Static(&[
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                ]),
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
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 1,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: calls_int_0_start,
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
                                    ints: 3,
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
                                    block: data::graph::BlockId(1),
                                    instruction: 0,
                                    ints: 2,
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
                                    block: data::graph::BlockId(2),
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
                                    block: data::graph::BlockId(2),
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
                                    ints: 3,
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
                                    instruction: 1,
                                    ints: 4,
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
                                    instruction: 2,
                                    ints: 4,
                                    bools: 2,
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
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 3,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                },
                                data::compiled::ReturnContract {
                                    point: 5,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: calls_int_1_start,
                        })),
                    },
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
                                data::Storage::Static(&[]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                ]),
                                data::Storage::Static(&[]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                },
                                data::compiled::ReturnContract {
                                    point: 7,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: calls_int_2_start,
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)),
                        implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                            root: false,
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 2,
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
                                    instruction: 1,
                                    ints: 3,
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
                                    block: data::graph::BlockId(1),
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
                                    block: data::graph::BlockId(1),
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
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                    point: 4,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                    ints: 2,
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
                                    instruction: 1,
                                    ints: 3,
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
                                    block: data::graph::BlockId(2),
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
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
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
                                    point: 3,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                },
                                data::compiled::ReturnContract {
                                    point: 5,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: calls_int_4_start,
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
                                    instruction: 1,
                                    ints: 3,
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
                                    block: data::graph::BlockId(1),
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
                            ]),
                            locals: data::Storage::Static(&[
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
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
                                    point: 2,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                },
                                data::compiled::ReturnContract {
                                    point: 4,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: calls_int_5_start,
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(6)),
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
                                    ints: 2,
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
                                    block: data::graph::BlockId(2),
                                    instruction: 0,
                                    ints: 2,
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
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(5),
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
                                    block: data::graph::BlockId(5),
                                    instruction: 1,
                                    ints: 2,
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
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
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
                            start: calls_int_6_start,
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(7)),
                        implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                            root: true,
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
                                    instruction: 2,
                                    ints: 3,
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
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                            ]),
                            calls: data::Storage::Static(&[
                                data::compiled::CallContract {
                                    point: 1,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    target: data::compiled::CallContractTarget::IntValue(data::graph::IntFunctionLocalId(0)),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "captured", data::source::SourceSpan::new(1895, 1911)),
                                },
                            ]),
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
                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(10)),
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
                                            source: data::graph::IntLocalId(1),
                                        },
                                    ]),
                                },
                            ]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 2,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: calls_int_7_start,
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(8)),
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
                                    ints: 5,
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
                                    instruction: 7,
                                    ints: 6,
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
                                    instruction: 8,
                                    ints: 7,
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
                                    instruction: 9,
                                    ints: 8,
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
                                    instruction: 10,
                                    ints: 9,
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
                                    instruction: 11,
                                    ints: 10,
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
                                    instruction: 12,
                                    ints: 11,
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
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
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
                                    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2174, 2191)),
                                },
                                data::compiled::CallContract {
                                    point: 7,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(1))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2194, 2217)),
                                },
                                data::compiled::CallContract {
                                    point: 10,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(2))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2220, 2229)),
                                },
                            ]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 12,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: calls_int_8_start,
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(9)),
                        implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                            root: false,
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
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
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
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
                            ]),
                            locals: data::Storage::Static(&[
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
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
                                    point: 2,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                },
                                data::compiled::ReturnContract {
                                    point: 4,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: calls_int_9_start,
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(10)),
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
                                    block: data::graph::BlockId(2),
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
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
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
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 2,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                },
                                data::compiled::ReturnContract {
                                    point: 4,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: calls_int_10_start,
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)),
                        implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                            root: false,
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
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
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
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
                                    block: data::graph::BlockId(1),
                                    instruction: 1,
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
                                    block: data::graph::BlockId(2),
                                    instruction: 0,
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
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(3),
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
                            locals: data::Storage::Static(&[
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 2,
                                    value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                },
                                data::compiled::ReturnContract {
                                    point: 4,
                                    value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                },
                                data::compiled::ReturnContract {
                                    point: 6,
                                    value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: calls_bool_0_start,
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
                0..11,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                11..12,
                0..0,
                12..13,
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
                    parameters: 2..6,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(1),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 6..7,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 7..10,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(1),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 10..13,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(1),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 13..16,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(1),
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
                    parameters: 18..20,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 20..20,
                    parameter_shapes: data::Storage::Static(&[]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 20..22,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(1),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 22..23,
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
                    parameters: 23..25,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(1),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 25..28,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(3),
                    ]),
                    return_: data::type_::ValueShapeId(5),
                    captures: data::Storage::Static(&[]),
                },
            ]),
            parameters: data::Storage::Static(&[
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
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
                data::type_::ValueShapeDescriptor::Bool,
                data::type_::ValueShapeDescriptor::Function {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                },
                data::type_::ValueShapeDescriptor::String,
                data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(0)),
                data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                    data::type_::ValueShapeId(3),
                    data::type_::ValueShapeId(0),
                    data::type_::ValueShapeId(4),
                    data::type_::ValueShapeId(0),
                ])),
            ]),
            shape_types: data::Storage::Static(&[
                data::type_::ValueType::Int,
                data::type_::ValueType::Bool,
                data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                }),
                data::type_::ValueType::String,
                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                data::type_::ValueType::Tuple(data::Storage::Static(&[
                    data::type_::ValueType::String,
                    data::type_::ValueType::Int,
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                    data::type_::ValueType::Int,
                ])),
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
            name: data::Text::Static("arithmetic"),
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
            name: data::Text::Static("shuffle"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Bool,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 1,
        },
        data::Export {
            name: data::Text::Static("choice"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Bool,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            slot: 0,
        },
        data::Export {
            name: data::Text::Static("switch"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 2,
        },
        data::Export {
            name: data::Text::Static("quotient"),
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
            name: data::Text::Static("operators"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Bool,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 4,
        },
        data::Export {
            name: data::Text::Static("divmod"),
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
            name: data::Text::Static("product"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 6,
        },
        data::Export {
            name: data::Text::Static("captured"),
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
            name: data::Text::Static("caller"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::String,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                    data::type_::TypeMetadata::String,
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                    data::type_::TypeMetadata::Int,
                ]))),
            },
            slot: 0,
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
            name: data::Text::Static("discarded"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Bool,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 9,
        },
    ]),
}
