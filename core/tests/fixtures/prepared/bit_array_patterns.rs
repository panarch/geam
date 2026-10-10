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
        main: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Tuple {
            id: data::function::TupleFunctionId(0),
            return_type: data::Storage::Static(&[
                data::type_::ValueType::Int,
                data::type_::ValueType::Float,
                data::type_::ValueType::BitArray,
                data::type_::ValueType::Int,
            ]),
        }),
        functions: data::function::FunctionTables {
            value_returns: data::function::ValueFunctionTables {
                never_functions: data::Storage::Static(&[]),
                int_functions: data::Storage::Static(&[
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
                                                        pattern: data::graph::BitArrayPatternValue::Discard,
                                                        size: data::graph::BitArrayPatternSize::Dynamic {
                                                            value: data::graph::BitArrayPatternSizeExpr::Local(data::graph::IntLocalId(1)),
                                                            unit: 1,
                                                        },
                                                        endianness: data::graph::Endianness::Big,
                                                        signedness: data::graph::Signedness::Unsigned,
                                                    },
                                                    data::graph::BitArrayPatternSegment::Int {
                                                        pattern: data::graph::BitArrayPatternValue::Bind(data::graph::MatchIntPatternBinding {
                                                            binding: data::graph::MatchPatternBinding {
                                                                index: 0,
                                                            },
                                                            size: None,
                                                        }),
                                                        size: data::graph::BitArrayPatternSize::Dynamic {
                                                            value: data::graph::BitArrayPatternSizeExpr::Local(data::graph::IntLocalId(0)),
                                                            unit: 1,
                                                        },
                                                        endianness: data::graph::Endianness::Little,
                                                        signedness: data::graph::Signedness::Signed,
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
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    0,
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
                                                            family: data::graph::StorageFamily::BitArray,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[]),
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
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 3..4,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..4,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                                    data::graph::BitArrayPatternSegment::Int {
                                                        pattern: data::graph::BitArrayPatternValue::Alias {
                                                            pattern: data::Storage::Static(&data::graph::BitArrayPatternValue::Discard),
                                                            binding: data::graph::MatchIntPatternBinding {
                                                                binding: data::graph::MatchPatternBinding {
                                                                    index: 1,
                                                                },
                                                                size: Some(data::graph::MatchIntBindingId(0)),
                                                            },
                                                        },
                                                        size: data::graph::BitArrayPatternSize::Fixed(8),
                                                        endianness: data::graph::Endianness::Big,
                                                        signedness: data::graph::Signedness::Unsigned,
                                                    },
                                                    data::graph::BitArrayPatternSegment::Int {
                                                        pattern: data::graph::BitArrayPatternValue::Bind(data::graph::MatchIntPatternBinding {
                                                            binding: data::graph::MatchPatternBinding {
                                                                index: 2,
                                                            },
                                                            size: None,
                                                        }),
                                                        size: data::graph::BitArrayPatternSize::Dynamic {
                                                            value: data::graph::BitArrayPatternSizeExpr::Binding(data::graph::MatchIntBindingId(0)),
                                                            unit: 1,
                                                        },
                                                        endianness: data::graph::Endianness::Big,
                                                        signedness: data::graph::Signedness::Unsigned,
                                                    },
                                                    data::graph::BitArrayPatternSegment::Int {
                                                        pattern: data::graph::BitArrayPatternValue::Bind(data::graph::MatchIntPatternBinding {
                                                            binding: data::graph::MatchPatternBinding {
                                                                index: 3,
                                                            },
                                                            size: None,
                                                        }),
                                                        size: data::graph::BitArrayPatternSize::Dynamic {
                                                            value: data::graph::BitArrayPatternSizeExpr::Binding(data::graph::MatchIntBindingId(0)),
                                                            unit: 1,
                                                        },
                                                        endianness: data::graph::Endianness::Big,
                                                        signedness: data::graph::Signedness::Unsigned,
                                                    },
                                                ]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Binding(2),
                                                    data::graph::MatchEdgeArgument::Binding(3),
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    2,
                                                    3,
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
                                                target: data::graph::BlockId(5),
                                                args: data::Storage::Static(&[]),
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..3,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::GtEqInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
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
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 3..5,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 5..5,
                                        instructions: 1..1,
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
                                        params: 5..5,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 5..5,
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
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
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
                                                    data::graph::BitArrayPatternSegment::Int {
                                                        pattern: data::graph::BitArrayPatternValue::Bind(data::graph::MatchIntPatternBinding {
                                                            binding: data::graph::MatchPatternBinding {
                                                                index: 1,
                                                            },
                                                            size: None,
                                                        }),
                                                        size: data::graph::BitArrayPatternSize::Fixed(0),
                                                        endianness: data::graph::Endianness::Big,
                                                        signedness: data::graph::Signedness::Unsigned,
                                                    },
                                                    data::graph::BitArrayPatternSegment::Int {
                                                        pattern: data::graph::BitArrayPatternValue::Bind(data::graph::MatchIntPatternBinding {
                                                            binding: data::graph::MatchPatternBinding {
                                                                index: 2,
                                                            },
                                                            size: None,
                                                        }),
                                                        size: data::graph::BitArrayPatternSize::Fixed(16),
                                                        endianness: data::graph::Endianness::Little,
                                                        signedness: data::graph::Signedness::Signed,
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
                                                    data::graph::MatchEdgeArgument::Binding(1),
                                                    data::graph::MatchEdgeArgument::Binding(2),
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    0,
                                                    1,
                                                    2,
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
                                                target: data::graph::BlockId(5),
                                                args: data::Storage::Static(&[]),
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..4,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::GtInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(10),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
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
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..7,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 7..7,
                                        instructions: 1..1,
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
                                        params: 7..7,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 7..7,
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
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::IntegerRegion(data::graph::ArithmeticRegion {
                                        inputs: data::Storage::Static(&[
                                            data::graph::IntLocalId(0),
                                            data::graph::IntLocalId(1),
                                            data::graph::IntLocalId(2),
                                        ]),
                                        nodes: data::Storage::Static(&[
                                            data::graph::ArithmeticNode::Add(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Input(1)),
                                            data::graph::ArithmeticNode::Add(data::graph::ArithmeticOperand::Value(0), data::graph::ArithmeticOperand::Input(2)),
                                        ]),
                                        outputs: data::Storage::Static(&[
                                            data::graph::ArithmeticOutput {
                                                value: 1,
                                                slot: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                    shape: data::type_::ValueShapeId(1),
                                                },
                                            },
                                        ]),
                                        native: true,
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
                                data::function::FunctionExit::Return(data::graph::IntLocalId(3)),
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
                                                    data::graph::BitArrayPatternSegment::Int {
                                                        pattern: data::graph::BitArrayPatternValue::Discard,
                                                        size: data::graph::BitArrayPatternSize::Fixed(18446744073709551615),
                                                        endianness: data::graph::Endianness::Big,
                                                        signedness: data::graph::Signedness::Unsigned,
                                                    },
                                                ]),
                                            }),
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
                                                            family: data::graph::StorageFamily::BitArray,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..2,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..3,
                                        instructions: 1..1,
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
                                                    data::graph::BitArrayPatternSegment::Int {
                                                        pattern: data::graph::BitArrayPatternValue::Discard,
                                                        size: data::graph::BitArrayPatternSize::Dynamic {
                                                            value: data::graph::BitArrayPatternSizeExpr::Value(data::graph::IntegerLiteral {
                                                                sign: data::Sign::Plus,
                                                                digits: data::Storage::Static(&[
                                                                    0,
                                                                    2147483648,
                                                                ]),
                                                            }),
                                                            unit: 2,
                                                        },
                                                        endianness: data::graph::Endianness::Big,
                                                        signedness: data::graph::Signedness::Unsigned,
                                                    },
                                                ]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Binding(0),
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
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 3..4,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..5,
                                        instructions: 2..2,
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
                                                    data::graph::BitArrayPatternSegment::Int {
                                                        pattern: data::graph::BitArrayPatternValue::Bind(data::graph::MatchIntPatternBinding {
                                                            binding: data::graph::MatchPatternBinding {
                                                                index: 1,
                                                            },
                                                            size: None,
                                                        }),
                                                        size: data::graph::BitArrayPatternSize::Fixed(8),
                                                        endianness: data::graph::Endianness::Big,
                                                        signedness: data::graph::Signedness::Unsigned,
                                                    },
                                                ]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(5),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Binding(0),
                                                    data::graph::MatchEdgeArgument::Binding(1),
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    0,
                                                    1,
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
                                                target: data::graph::BlockId(6),
                                                args: data::Storage::Static(&[]),
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 5..7,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 7..7,
                                        instructions: 3..4,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(3)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Immediate(100),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Immediate(200),
                                        }),
                                    }),
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
                                data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(2)),
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
                bool_functions: data::Storage::Static(&[]),
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
                                                        size: data::graph::BitArrayPatternSize::Dynamic {
                                                            value: data::graph::BitArrayPatternSizeExpr::Local(data::graph::IntLocalId(0)),
                                                            unit: 1,
                                                        },
                                                        endianness: data::graph::Endianness::Little,
                                                        signedness: data::graph::Signedness::Signed,
                                                    },
                                                    data::graph::BitArrayPatternSegment::Float {
                                                        pattern: data::graph::BitArrayPatternValue::Bind(data::graph::MatchPatternBinding {
                                                            index: 1,
                                                        }),
                                                        size: data::graph::BitArrayPatternSize::Dynamic {
                                                            value: data::graph::BitArrayPatternSizeExpr::Local(data::graph::IntLocalId(0)),
                                                            unit: 1,
                                                        },
                                                        endianness: data::graph::Endianness::Little,
                                                    },
                                                    data::graph::BitArrayPatternSegment::Bits {
                                                        pattern: data::graph::BitArrayBindingPattern::Bind(data::graph::MatchPatternBinding {
                                                            index: 2,
                                                        }),
                                                        size: Some(data::graph::BitArrayPatternSize::Dynamic {
                                                            value: data::graph::BitArrayPatternSizeExpr::Local(data::graph::IntLocalId(0)),
                                                            unit: 8,
                                                        }),
                                                        unit: 8,
                                                    },
                                                    data::graph::BitArrayPatternSegment::Int {
                                                        pattern: data::graph::BitArrayPatternValue::Bind(data::graph::MatchIntPatternBinding {
                                                            binding: data::graph::MatchPatternBinding {
                                                                index: 3,
                                                            },
                                                            size: None,
                                                        }),
                                                        size: data::graph::BitArrayPatternSize::Fixed(8),
                                                        endianness: data::graph::Endianness::Big,
                                                        signedness: data::graph::Signedness::Unsigned,
                                                    },
                                                ]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Binding(0),
                                                    data::graph::MatchEdgeArgument::Binding(1),
                                                    data::graph::MatchEdgeArgument::Binding(2),
                                                    data::graph::MatchEdgeArgument::Binding(3),
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    0,
                                                    1,
                                                    2,
                                                    3,
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
                                                            family: data::graph::StorageFamily::BitArray,
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
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[]),
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
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..6,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 6..6,
                                        instructions: 1..7,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
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
                                        local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Float,
                                                    data::type_::ValueType::BitArray,
                                                    data::type_::ValueType::Int,
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                            data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        ]))),
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
                                            local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Float(data::graph::FloatInstruction::Value(f64::from_bits(13830554455654793216))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::BitArray(data::graph::BitArrayInstruction::Value(data::Storage::Static(&[
                                            data::graph::BitArraySegment::Int {
                                                value: data::graph::IntLocalId(1),
                                                bit_size: 8,
                                                endianness: data::graph::Endianness::Big,
                                            },
                                        ]))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
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
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Float,
                                                    data::type_::ValueType::BitArray,
                                                    data::type_::ValueType::Int,
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                            data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        ]))),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::TupleLocalId(0)),
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

            fn bit_array_int_2(
                point: usize,
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::bit_array::BitArrayValues, &mut usize) -> CompiledResume;
                    8
                ] = [
                    |values, budget| CompiledResume::Exit(bit_array_int_2_entry((values.bit_arrays[0],), values, budget)),
                    bit_array_int_2_resume_1,
                    bit_array_int_2_resume_2,
                    bit_array_int_2_resume_3,
                    bit_array_int_2_resume_4,
                    bit_array_int_2_resume_5,
                    bit_array_int_2_resume_6,
                    bit_array_int_2_resume_7,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn bit_array_int_2_entry(
                inputs: (data::compiled::bit_array::BitArrayRange,),
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (b0_b0,) = inputs;
                let (b2_i0, b2_i1, b2_i2,) = 'block_2: {
                    let () = 'block_4: {
                        let () = 'block_3: {
                            let (b1_i0, b1_i1, b1_i2,) = 'block_1: {
                                let () = 'block_5: {
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.bit_arrays.clear();
                                        values.bit_arrays.extend_from_slice(&[b0_b0]);
                                        return data::compiled::CompiledProgress::Yield(0);
                                    }
                                    *budget -= 1;
                                    let _matched = (|| -> Option<(i128, i128, i128,)> {
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
                                        let _field_1 = values.integer(
                                            b0_b0,
                                            _offset,
                                            0_usize,
                                            data::graph::Endianness::Big,
                                            data::graph::Signedness::Unsigned,
                                        )?;
                                        let _matched_1 = _field_1;
                                        _offset = _offset.checked_add(0_usize)?;
                                        let _field_2 = values.integer(
                                            b0_b0,
                                            _offset,
                                            16_usize,
                                            data::graph::Endianness::Little,
                                            data::graph::Signedness::Signed,
                                        )?;
                                        let _matched_2 = _field_2;
                                        _offset = _offset.checked_add(16_usize)?;
                                        let _length_3 = b0_b0.bit_len().checked_sub(_offset)?;
                                        let _range_3 = b0_b0.slice(_offset, _length_3)?;
                                        _offset = _offset.checked_add(_length_3)?;
                                        if _offset != b0_b0.bit_len() { return None; }
                                        Some((_matched_0, _matched_1, _matched_2,))
                                    })();
                                    match _matched {
                                        Some((_matched_0, _matched_1, _matched_2,)) => {
                                            break 'block_1 (_matched_0, _matched_1, _matched_2,);
                                        },
                                        None => {
                                            break 'block_5;
                                        },
                                    }
                                };
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.bit_arrays.clear();
                                    values.bit_arrays.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Yield(7);
                                }
                                *budget -= 1;
                                {
                                    break 'block_4;
                                }
                            };
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.bit_arrays.clear();
                                values.bit_arrays.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(1);
                            }
                            *budget -= 1;
                            if b1_i0 > 10_i128 {
                                break 'block_2 (b1_i0, b1_i1, b1_i2,);
                            } else {
                                break 'block_3;
                            }
                        };
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.bit_arrays.clear();
                            values.bit_arrays.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(4);
                        }
                        *budget -= 1;
                        {
                            break 'block_4;
                        }
                    };
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(5);
                    }
                    *budget -= 1;
                    let b4_i0 = -1_i128;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(6);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1));
                };
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return data::compiled::CompiledProgress::Yield(2);
                }
                *budget -= 1;
                let _r0_n0 = b2_i0 + b2_i1;
                let _r0_n1 = _r0_n0 + b2_i2;
                let b2_i3 = _r0_n1;
                if b2_i3 < i128::from(i64::MIN) || b2_i3 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2, b2_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return data::compiled::CompiledProgress::Interpreted(3);
                }
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2, b2_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return data::compiled::CompiledProgress::Yield(3);
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2, b2_i3]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
            }

            fn bit_array_int_2_resume_1(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_i1, b1_i2,) = (values.ints[0], values.ints[1], values.ints[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                if b1_i0 > 10_i128 {
                    let (b2_i0, b2_i1, b2_i2,) = (b1_i0, b1_i1, b1_i2,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    CompiledResume::Next(2)
                } else {
                    let () = ();

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    CompiledResume::Next(4)
                }
            }

            fn bit_array_int_2_resume_2(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_i1, b2_i2,) = (values.ints[0], values.ints[1], values.ints[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                let _r0_n0 = b2_i0 + b2_i1;
                let _r0_n1 = _r0_n0 + b2_i2;
                let b2_i3 = _r0_n1;
                if b2_i3 < i128::from(i64::MIN) || b2_i3 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2, b2_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(3));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2, b2_i3]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Next(3)
            }

            fn bit_array_int_2_resume_3(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_i1, b2_i2, b2_i3,) = (values.ints[0], values.ints[1], values.ints[2], values.ints[3],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2, b2_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2, b2_i3]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn bit_array_int_2_resume_4(
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
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;
                {
                    let () = ();

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    CompiledResume::Next(5)
                }
            }

            fn bit_array_int_2_resume_5(
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
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;
                let b4_i0 = -1_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b4_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Next(6)
            }

            fn bit_array_int_2_resume_6(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b4_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn bit_array_int_2_resume_7(
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
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                }
                *budget -= 1;
                {
                    let () = ();

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    CompiledResume::Next(5)
                }
            }
            data::compiled::CompiledFunctions {
                ints: data::Storage::Static(&[
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(2),
                        implementation: data::compiled::CompiledImplementation::BitArray(data::compiled::BitArrayImplementation {
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
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
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 0,
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
                            run: bit_array_int_2,
                        }),
                    },
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
                0..4,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                4..5,
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
                    parameters: 0..3,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(1),
                        data::type_::ValueShapeId(1),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 3..4,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
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
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 6..8,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(1),
                    ]),
                    return_: data::type_::ValueShapeId(3),
                    captures: data::Storage::Static(&[]),
                },
            ]),
            parameters: data::Storage::Static(&[
                data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
            ]),
        },
        list_types: data::type_::ListTypeTable {
            types: data::Storage::Static(&[]),
            tuple_items: data::Storage::Static(&[]),
            function_items: data::Storage::Static(&[]),
            lifetimes: data::Storage::Static(&[]),
        },
        custom_types: data::type_::CustomTypeTable {
            types: data::Storage::Static(&[]),
            definitions: data::Storage::Static(&[]),
        },
        external_types: data::type_::ExternalTypeTable {
            types: data::Storage::Static(&[]),
            lifetimes: data::Storage::Static(&[]),
            definitions: data::Storage::Static(&[]),
        },
        value_shapes: data::type_::ValueShapeTable {
            shapes: data::Storage::Static(&[
                data::type_::ValueShapeDescriptor::BitArray,
                data::type_::ValueShapeDescriptor::Int,
                data::type_::ValueShapeDescriptor::Float,
                data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                    data::type_::ValueShapeId(1),
                    data::type_::ValueShapeId(2),
                    data::type_::ValueShapeId(0),
                    data::type_::ValueShapeId(1),
                ])),
            ]),
            shape_types: data::Storage::Static(&[
                data::type_::ValueType::BitArray,
                data::type_::ValueType::Int,
                data::type_::ValueType::Float,
                data::type_::ValueType::Tuple(data::Storage::Static(&[
                    data::type_::ValueType::Int,
                    data::type_::ValueType::Float,
                    data::type_::ValueType::BitArray,
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
                callables: data::Storage::Static(&[]),
            },
        ]),
        lists: data::Storage::Static(&[]),
        functions: data::Storage::Static(&[]),
    },
    exports: data::Storage::Static(&[
        data::Export {
            name: data::Text::Static("zero_fields"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::BitArray,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Float,
                    data::type_::TypeMetadata::BitArray,
                    data::type_::TypeMetadata::Int,
                ]))),
            },
            slot: 0,
        },
        data::Export {
            name: data::Text::Static("signed_little"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::BitArray,
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 0,
        },
        data::Export {
            name: data::Text::Static("dependent_fields"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::BitArray,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 1,
        },
        data::Export {
            name: data::Text::Static("fixed_fields"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::BitArray,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 2,
        },
        data::Export {
            name: data::Text::Static("fixed_failure"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::BitArray,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 3,
        },
    ]),
}
