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
                                                    data::graph::BitArrayPatternSegment::Int {
                                                        pattern: data::graph::BitArrayPatternValue::Bind(data::graph::MatchIntPatternBinding {
                                                            binding: data::graph::MatchPatternBinding {
                                                                index: 2,
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
                                                                index: 3,
                                                            },
                                                            size: None,
                                                        }),
                                                        size: data::graph::BitArrayPatternSize::Fixed(8),
                                                        endianness: data::graph::Endianness::Big,
                                                        signedness: data::graph::Signedness::Unsigned,
                                                    },
                                                    data::graph::BitArrayPatternSegment::Bits {
                                                        pattern: data::graph::BitArrayBindingPattern::Bind(data::graph::MatchPatternBinding {
                                                            index: 4,
                                                        }),
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
                                                    data::graph::MatchEdgeArgument::Binding(3),
                                                    data::graph::MatchEdgeArgument::Binding(4),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    0,
                                                    1,
                                                    2,
                                                    3,
                                                    4,
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Int,
                                                            length: 5,
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
                                                                data::graph::TransferStep {
                                                                    source: 4,
                                                                    destination: 3,
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
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..8,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Int,
                                                            length: 1,
                                                            steps: data::Storage::Static(&[
                                                                data::graph::TransferStep {
                                                                    source: 5,
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
                                        params: 8..10,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            pattern: data::graph::MatchPattern::BitArray(data::graph::BitArrayPattern {
                                                segments: data::Storage::Static(&[]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                ]),
                                                bindings: data::Storage::Static(&[]),
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
                                        params: 10..11,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 11..11,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::SourceStop(data::graph::SourceStop {
                                            kind: data::graph::SourceStopKind::Panic,
                                            message: Some(data::graph::StringLocalId(0)),
                                            site: data::source::PanicSite::from_static("example", "checksum", data::source::SourceSpan::new(193, 221)),
                                        }),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
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
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
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
                                            data::graph::IntLocalId(4),
                                            data::graph::IntLocalId(0),
                                            data::graph::IntLocalId(1),
                                            data::graph::IntLocalId(2),
                                            data::graph::IntLocalId(3),
                                        ]),
                                        nodes: data::Storage::Static(&[
                                            data::graph::ArithmeticNode::Add(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Input(1)),
                                            data::graph::ArithmeticNode::Multiply(data::graph::ArithmeticOperand::Input(2), data::graph::ArithmeticOperand::Immediate(2)),
                                            data::graph::ArithmeticNode::Add(data::graph::ArithmeticOperand::Value(0), data::graph::ArithmeticOperand::Value(1)),
                                            data::graph::ArithmeticNode::Multiply(data::graph::ArithmeticOperand::Input(3), data::graph::ArithmeticOperand::Immediate(3)),
                                            data::graph::ArithmeticNode::Add(data::graph::ArithmeticOperand::Value(2), data::graph::ArithmeticOperand::Value(3)),
                                            data::graph::ArithmeticNode::Multiply(data::graph::ArithmeticOperand::Input(4), data::graph::ArithmeticOperand::Immediate(4)),
                                            data::graph::ArithmeticNode::Add(data::graph::ArithmeticOperand::Value(4), data::graph::ArithmeticOperand::Value(5)),
                                        ]),
                                        outputs: data::Storage::Static(&[
                                            data::graph::ArithmeticOutput {
                                                value: 6,
                                                slot: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                                    shape: data::type_::ValueShapeId(0),
                                                },
                                            },
                                        ]),
                                        native: true,
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("incomplete record"))),
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
                                                        size: data::graph::BitArrayPatternSize::Fixed(64),
                                                        endianness: data::graph::Endianness::Big,
                                                        signedness: data::graph::Signedness::Unsigned,
                                                    },
                                                    data::graph::BitArrayPatternSegment::Bits {
                                                        pattern: data::graph::BitArrayBindingPattern::Bind(data::graph::MatchPatternBinding {
                                                            index: 1,
                                                        }),
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
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    0,
                                                    1,
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
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..5,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
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
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 5..7,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            pattern: data::graph::MatchPattern::BitArray(data::graph::BitArrayPattern {
                                                segments: data::Storage::Static(&[]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                ]),
                                                bindings: data::Storage::Static(&[]),
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
                                        params: 7..8,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 8..8,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
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
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
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
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
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
                                            subject: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            pattern: data::graph::MatchPattern::BitArray(data::graph::BitArrayPattern {
                                                segments: data::Storage::Static(&[
                                                    data::graph::BitArrayPatternSegment::Int {
                                                        pattern: data::graph::BitArrayPatternValue::Alias {
                                                            pattern: data::Storage::Static(&data::graph::BitArrayPatternValue::Literal(data::graph::IntegerLiteral {
                                                                sign: data::Sign::Plus,
                                                                digits: data::Storage::Static(&[
                                                                    1,
                                                                ]),
                                                            })),
                                                            binding: data::graph::MatchIntPatternBinding {
                                                                binding: data::graph::MatchPatternBinding {
                                                                    index: 0,
                                                                },
                                                                size: None,
                                                            },
                                                        },
                                                        size: data::graph::BitArrayPatternSize::Fixed(8),
                                                        endianness: data::graph::Endianness::Big,
                                                        signedness: data::graph::Signedness::Unsigned,
                                                    },
                                                    data::graph::BitArrayPatternSegment::Int {
                                                        pattern: data::graph::BitArrayPatternValue::Literal(data::graph::IntegerLiteral {
                                                            sign: data::Sign::Plus,
                                                            digits: data::Storage::Static(&[
                                                                2,
                                                            ]),
                                                        }),
                                                        size: data::graph::BitArrayPatternSize::Fixed(8),
                                                        endianness: data::graph::Endianness::Big,
                                                        signedness: data::graph::Signedness::Unsigned,
                                                    },
                                                    data::graph::BitArrayPatternSegment::Bits {
                                                        pattern: data::graph::BitArrayBindingPattern::Bind(data::graph::MatchPatternBinding {
                                                            index: 1,
                                                        }),
                                                        size: Some(data::graph::BitArrayPatternSize::Fixed(8)),
                                                        unit: 1,
                                                    },
                                                    data::graph::BitArrayPatternSegment::Bits {
                                                        pattern: data::graph::BitArrayBindingPattern::Bind(data::graph::MatchPatternBinding {
                                                            index: 2,
                                                        }),
                                                        size: None,
                                                        unit: 8,
                                                    },
                                                ]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Binding(0),
                                                    data::graph::MatchEdgeArgument::Binding(1),
                                                    data::graph::MatchEdgeArgument::Binding(2),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0))),
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
                                                            family: data::graph::StorageFamily::Int,
                                                            length: 2,
                                                            steps: data::Storage::Static(&[
                                                                data::graph::TransferStep {
                                                                    source: 1,
                                                                    destination: 0,
                                                                },
                                                            ]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::BitArray,
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
                                                            ]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(6),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..7,
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
                                                ]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Binding(0),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(2))),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1))),
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    0,
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Int,
                                                            length: 3,
                                                            steps: data::Storage::Static(&[
                                                                data::graph::TransferStep {
                                                                    source: 2,
                                                                    destination: 0,
                                                                },
                                                            ]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::BitArray,
                                                            length: 2,
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
                                            failure: data::graph::Edge {
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
                                        params: 7..12,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            pattern: data::graph::MatchPattern::BitArray(data::graph::BitArrayPattern {
                                                segments: data::Storage::Static(&[
                                                    data::graph::BitArrayPatternSegment::Int {
                                                        pattern: data::graph::BitArrayPatternValue::Discard,
                                                        size: data::graph::BitArrayPatternSize::Fixed(24),
                                                        endianness: data::graph::Endianness::Big,
                                                        signedness: data::graph::Signedness::Unsigned,
                                                    },
                                                    data::graph::BitArrayPatternSegment::Bits {
                                                        pattern: data::graph::BitArrayBindingPattern::Discard,
                                                        size: None,
                                                        unit: 8,
                                                    },
                                                ]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(2))),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1))),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                ]),
                                                bindings: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Int,
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
                                        params: 12..16,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Int,
                                                            length: 1,
                                                            steps: data::Storage::Static(&[
                                                                data::graph::TransferStep {
                                                                    source: 3,
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
                                        params: 16..16,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 16..16,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 16..18,
                                        instructions: 3..3,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            pattern: data::graph::MatchPattern::BitArray(data::graph::BitArrayPattern {
                                                segments: data::Storage::Static(&[]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(7),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                ]),
                                                bindings: data::Storage::Static(&[]),
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
                                        params: 18..19,
                                        instructions: 3..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 19..19,
                                        instructions: 3..4,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(3)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
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
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(2)),
                                        shape: data::type_::ValueShapeId(2),
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
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                        shape: data::type_::ValueShapeId(2),
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
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
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
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Minus,
                                            digits: data::Storage::Static(&[
                                                2,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Minus,
                                            digits: data::Storage::Static(&[
                                                2,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
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
                                                        size: data::graph::BitArrayPatternSize::Fixed(9),
                                                        endianness: data::graph::Endianness::Little,
                                                        signedness: data::graph::Signedness::Signed,
                                                    },
                                                    data::graph::BitArrayPatternSegment::Bits {
                                                        pattern: data::graph::BitArrayBindingPattern::Bind(data::graph::MatchPatternBinding {
                                                            index: 1,
                                                        }),
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
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    0,
                                                    1,
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
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..5,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
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
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 5..7,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            pattern: data::graph::MatchPattern::BitArray(data::graph::BitArrayPattern {
                                                segments: data::Storage::Static(&[]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                ]),
                                                bindings: data::Storage::Static(&[]),
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
                                        params: 7..8,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 8..8,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
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
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
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
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
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
                                                        size: data::graph::BitArrayPatternSize::Fixed(64),
                                                        endianness: data::graph::Endianness::Big,
                                                        signedness: data::graph::Signedness::Unsigned,
                                                    },
                                                    data::graph::BitArrayPatternSegment::Int {
                                                        pattern: data::graph::BitArrayPatternValue::Literal(data::graph::IntegerLiteral {
                                                            sign: data::Sign::Plus,
                                                            digits: data::Storage::Static(&[
                                                                255,
                                                            ]),
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
                                        params: 1..2,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..2,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
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
                                                        size: data::graph::BitArrayPatternSize::Fixed(7),
                                                        endianness: data::graph::Endianness::Big,
                                                        signedness: data::graph::Signedness::Unsigned,
                                                    },
                                                    data::graph::BitArrayPatternSegment::Bits {
                                                        pattern: data::graph::BitArrayBindingPattern::Bind(data::graph::MatchPatternBinding {
                                                            index: 1,
                                                        }),
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
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1))),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    0,
                                                    1,
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
                                                            ]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::BitArray,
                                                            length: 2,
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
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(4),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 3..7,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                            pattern: data::graph::MatchPattern::BitArray(data::graph::BitArrayPattern {
                                                segments: data::Storage::Static(&[
                                                    data::graph::BitArrayPatternSegment::Int {
                                                        pattern: data::graph::BitArrayPatternValue::Bind(data::graph::MatchIntPatternBinding {
                                                            binding: data::graph::MatchPatternBinding {
                                                                index: 0,
                                                            },
                                                            size: None,
                                                        }),
                                                        size: data::graph::BitArrayPatternSize::Fixed(7),
                                                        endianness: data::graph::Endianness::Big,
                                                        signedness: data::graph::Signedness::Unsigned,
                                                    },
                                                    data::graph::BitArrayPatternSegment::Bits {
                                                        pattern: data::graph::BitArrayBindingPattern::Bind(data::graph::MatchPatternBinding {
                                                            index: 1,
                                                        }),
                                                        size: None,
                                                        unit: 1,
                                                    },
                                                ]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Binding(0),
                                                    data::graph::MatchEdgeArgument::Binding(1),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0))),
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    0,
                                                    1,
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Int,
                                                            length: 3,
                                                            steps: data::Storage::Static(&[
                                                                data::graph::TransferStep {
                                                                    source: 2,
                                                                    destination: 0,
                                                                },
                                                            ]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::BitArray,
                                                            length: 2,
                                                            steps: data::Storage::Static(&[
                                                                data::graph::TransferStep {
                                                                    source: 2,
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
                                            failure: data::graph::Edge {
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
                                        params: 7..12,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Int,
                                                            length: 1,
                                                            steps: data::Storage::Static(&[
                                                                data::graph::TransferStep {
                                                                    source: 3,
                                                                    destination: 0,
                                                                },
                                                            ]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::BitArray,
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
                                        params: 12..12,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 12..15,
                                        instructions: 2..2,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            pattern: data::graph::MatchPattern::BitArray(data::graph::BitArrayPattern {
                                                segments: data::Storage::Static(&[]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(5),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1))),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                ]),
                                                bindings: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
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
                                        params: 15..17,
                                        instructions: 2..2,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            pattern: data::graph::MatchPattern::BitArray(data::graph::BitArrayPattern {
                                                segments: data::Storage::Static(&[]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(6),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                ]),
                                                bindings: data::Storage::Static(&[]),
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
                                                target: data::graph::BlockId(7),
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
                                        params: 17..18,
                                        instructions: 2..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 18..18,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 18..18,
                                        instructions: 3..4,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(3)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                        shape: data::type_::ValueShapeId(2),
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
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                        shape: data::type_::ValueShapeId(2),
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
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
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
                                            data::graph::IntLocalId(1),
                                            data::graph::IntLocalId(2),
                                            data::graph::IntLocalId(0),
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
                                            shape: data::type_::ValueShapeId(0),
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
                                            shape: data::type_::ValueShapeId(0),
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
                ]),
                float_functions: data::Storage::Static(&[]),
                string_functions: data::Storage::Static(&[]),
                bit_array_functions: data::Storage::Static(&[]),
                utf_codepoint_functions: data::Storage::Static(&[]),
                custom_functions: data::Storage::Static(&[
                    data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 3,
                        },
                        body: data::function::ProfiledCustomFunctionBody {
                            _signature_shape: data::type_::CustomValueShape {
                                type_id: data::type_::CustomTypeId(0),
                                shape_id: data::type_::CustomValueShapeId(0),
                            },
                            _body_shape: data::type_::CustomValueShape {
                                type_id: data::type_::CustomTypeId(0),
                                shape_id: data::type_::CustomValueShapeId(0),
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
                                                            pattern: data::graph::BitArrayBindingPattern::Bind(data::graph::MatchPatternBinding {
                                                                index: 1,
                                                            }),
                                                            size: None,
                                                            unit: 1,
                                                        },
                                                    ]),
                                                }),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Binding(1),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                        data::graph::MatchEdgeArgument::Binding(0),
                                                    ]),
                                                    bindings: data::Storage::Static(&[
                                                        0,
                                                        1,
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(18),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
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
                                            params: 3..8,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::GtEqInt {
                                                    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
                                                    right: data::graph::IntegerOperand::Immediate(48),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(17),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
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
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BitArray,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 8..13,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::LtEqInt {
                                                    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
                                                    right: data::graph::IntegerOperand::Immediate(57),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(3),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
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
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(4),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
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
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BitArray,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 13..17,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 2,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 3,
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
                                            params: 17..20,
                                            instructions: 1..1,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(5),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
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
                                            params: 20..23,
                                            instructions: 1..1,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(6),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
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
                                            params: 23..26,
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
                                                        data::graph::BitArrayPatternSegment::Bits {
                                                            pattern: data::graph::BitArrayBindingPattern::Bind(data::graph::MatchPatternBinding {
                                                                index: 1,
                                                            }),
                                                            size: None,
                                                            unit: 1,
                                                        },
                                                    ]),
                                                }),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(7),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Binding(1),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                        data::graph::MatchEdgeArgument::Binding(0),
                                                    ]),
                                                    bindings: data::Storage::Static(&[
                                                        0,
                                                        1,
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(16),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
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
                                            params: 26..31,
                                            instructions: 1..1,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::EqualInt {
                                                    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
                                                    right: data::graph::IntegerOperand::Immediate(44),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(8),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 2,
                                                                steps: data::Storage::Static(&[]),
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
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(10),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 31..34,
                                            instructions: 1..1,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(9),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 34..37,
                                            instructions: 1..3,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
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
                                            params: 37..42,
                                            instructions: 3..3,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::EqualInt {
                                                    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
                                                    right: data::graph::IntegerOperand::Immediate(10),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(11),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 2,
                                                                steps: data::Storage::Static(&[]),
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
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(12),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
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
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BitArray,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 42..45,
                                            instructions: 3..3,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(9),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 45..48,
                                            instructions: 3..3,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(13),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
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
                                            params: 48..51,
                                            instructions: 3..3,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                pattern: data::graph::MatchPattern::BitArray(data::graph::BitArrayPattern {
                                                    segments: data::Storage::Static(&[]),
                                                }),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(14),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
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
                                                    target: data::graph::BlockId(15),
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
                                            params: 51..53,
                                            instructions: 3..5,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 53..53,
                                            instructions: 5..7,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 53..56,
                                            instructions: 7..7,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(13),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
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
                                            params: 56..59,
                                            instructions: 7..7,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(5),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
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
                                            params: 59..62,
                                            instructions: 7..7,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(6),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
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
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                                data::graph::IntLocalId(2),
                                            ]),
                                            nodes: data::Storage::Static(&[
                                                data::graph::ArithmeticNode::Multiply(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Immediate(10)),
                                                data::graph::ArithmeticNode::Add(data::graph::ArithmeticOperand::Value(0), data::graph::ArithmeticOperand::Input(1)),
                                                data::graph::ArithmeticNode::Subtract(data::graph::ArithmeticOperand::Value(1), data::graph::ArithmeticOperand::Immediate(48)),
                                            ]),
                                            outputs: data::Storage::Static(&[
                                                data::graph::ArithmeticOutput {
                                                    value: 2,
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
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
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
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                ]),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Nil(data::graph::NilLocalId(0)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Nil(data::graph::NilInstruction::Value),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(4),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Nil(data::graph::NilLocalId(0)),
                                                ]),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    }),
                                    data::function::FunctionExit::Return(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    }),
                                ]),
                            },
                        },
                    },
                ]),
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
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            pattern: data::graph::MatchPattern::BitArray(data::graph::BitArrayPattern {
                                                segments: data::Storage::Static(&[
                                                    data::graph::BitArrayPatternSegment::Int {
                                                        pattern: data::graph::BitArrayPatternValue::Literal(data::graph::IntegerLiteral {
                                                            sign: data::Sign::Plus,
                                                            digits: data::Storage::Static(&[
                                                                1,
                                                            ]),
                                                        }),
                                                        size: data::graph::BitArrayPatternSize::Fixed(8),
                                                        endianness: data::graph::Endianness::Big,
                                                        signedness: data::graph::Signedness::Unsigned,
                                                    },
                                                    data::graph::BitArrayPatternSegment::Bits {
                                                        pattern: data::graph::BitArrayBindingPattern::Bind(data::graph::MatchPatternBinding {
                                                            index: 0,
                                                        }),
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
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    0,
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
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
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
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
                                    data::graph::BlockHeader {
                                        params: 4..6,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            pattern: data::graph::MatchPattern::BitArray(data::graph::BitArrayPattern {
                                                segments: data::Storage::Static(&[]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0))),
                                                ]),
                                                bindings: data::Storage::Static(&[]),
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
                                                args: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::BitArray,
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
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 7..7,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(5),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(5),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(5),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(5),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Not(data::graph::BoolLocalId(0)))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(false)),
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

            enum CompiledResume {
                Next(usize),
                Exit(data::compiled::CompiledProgress),
            }

            fn bit_array_int_0(
                point: usize,
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::bit_array::BitArrayValues, &mut usize) -> CompiledResume;
                    6
                ] = [
                    |values, budget| CompiledResume::Exit(bit_array_int_0_entry((values.ints[0], values.bit_arrays[0],), values, budget)),
                    bit_array_int_0_resume_1,
                    bit_array_int_0_resume_2,
                    bit_array_int_0_resume_3,
                    bit_array_int_0_resume_4,
                    bit_array_int_0_resume_5,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn bit_array_int_0_entry(
                inputs: (i128, data::compiled::bit_array::BitArrayRange,),
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (mut b0_i0, mut b0_b0,) = inputs;
                'repeat: loop {
                    let (b1_i0, b1_i1, b1_i2, b1_i3, b1_i4, b1_b0,) = 'block_1: {
                        let (b3_i0,) = 'block_3: {
                            let () = 'block_4: {
                                let (b2_i0, b2_b0,) = 'block_2: {
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b0_i0]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.bit_arrays.clear();
                                        values.bit_arrays.extend_from_slice(&[b0_b0]);
                                        return data::compiled::CompiledProgress::Yield(0);
                                    }
                                    *budget -= 1;
                                    let _matched = (|| -> Option<(i128, i128, i128, i128, data::compiled::bit_array::BitArrayRange,)> {
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
                                            8_usize,
                                            data::graph::Endianness::Big,
                                            data::graph::Signedness::Unsigned,
                                        )?;
                                        let _matched_1 = _field_1;
                                        _offset = _offset.checked_add(8_usize)?;
                                        let _field_2 = values.integer(
                                            b0_b0,
                                            _offset,
                                            8_usize,
                                            data::graph::Endianness::Big,
                                            data::graph::Signedness::Unsigned,
                                        )?;
                                        let _matched_2 = _field_2;
                                        _offset = _offset.checked_add(8_usize)?;
                                        let _field_3 = values.integer(
                                            b0_b0,
                                            _offset,
                                            8_usize,
                                            data::graph::Endianness::Big,
                                            data::graph::Signedness::Unsigned,
                                        )?;
                                        let _matched_3 = _field_3;
                                        _offset = _offset.checked_add(8_usize)?;
                                        let _length_4 = b0_b0.bit_len().checked_sub(_offset)?;
                                        let _range_4 = b0_b0.slice(_offset, _length_4)?;
                                        let _matched_4 = _range_4;
                                        _offset = _offset.checked_add(_length_4)?;
                                        if _offset != b0_b0.bit_len() { return None; }
                                        Some((_matched_0, _matched_1, _matched_2, _matched_3, _matched_4,))
                                    })();
                                    match _matched {
                                        Some((_matched_0, _matched_1, _matched_2, _matched_3, _matched_4,)) => {
                                            break 'block_1 (_matched_0, _matched_1, _matched_2, _matched_3, b0_i0, _matched_4,);
                                        },
                                        None => {
                                            break 'block_2 (b0_i0, b0_b0,);
                                        },
                                    }
                                };
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b2_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.bit_arrays.clear();
                                    values.bit_arrays.extend_from_slice(&[b2_b0]);
                                    return data::compiled::CompiledProgress::Yield(3);
                                }
                                *budget -= 1;
                                let _matched = (|| -> Option<()> {
                                    let mut _offset = 0_usize;
                                    if _offset != b2_b0.bit_len() { return None; }
                                    Some(())
                                })();
                                match _matched {
                                    Some(()) => {
                                        break 'block_3 (b2_i0,);
                                    },
                                    None => {
                                        break 'block_4;
                                    },
                                }
                            };

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.bit_arrays.clear();
                            values.bit_arrays.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Interpreted(5);
                        };
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b3_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.bit_arrays.clear();
                            values.bit_arrays.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(4);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0));
                    };
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2, b1_i3, b1_i4]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b1_b0]);
                        return data::compiled::CompiledProgress::Yield(1);
                    }
                    *budget -= 1;
                    let _r0_n0 = b1_i4 + b1_i0;
                    let _r0_n1 = b1_i1 * 2_i128;
                    let _r0_n2 = _r0_n0 + _r0_n1;
                    let _r0_n3 = b1_i2 * 3_i128;
                    let _r0_n4 = _r0_n2 + _r0_n3;
                    let _r0_n5 = b1_i3 * 4_i128;
                    let _r0_n6 = _r0_n4 + _r0_n5;
                    let b1_i5 = _r0_n6;
                    if b1_i5 < i128::from(i64::MIN) || b1_i5 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2, b1_i3, b1_i4, b1_i5]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b1_b0]);
                        return data::compiled::CompiledProgress::Interpreted(2);
                    }
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2, b1_i3, b1_i4, b1_i5]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b1_b0]);
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;
                    {
                        (b0_i0, b0_b0,) = (b1_i5, b1_b0,);
                        continue 'repeat;
                    }
                }
            }

            fn bit_array_int_0_resume_1(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_i1, b1_i2, b1_i3, b1_i4, b1_b0,) = (values.ints[0], values.ints[1], values.ints[2], values.ints[3], values.ints[4], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2, b1_i3, b1_i4]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b1_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                let _r0_n0 = b1_i4 + b1_i0;
                let _r0_n1 = b1_i1 * 2_i128;
                let _r0_n2 = _r0_n0 + _r0_n1;
                let _r0_n3 = b1_i2 * 3_i128;
                let _r0_n4 = _r0_n2 + _r0_n3;
                let _r0_n5 = b1_i3 * 4_i128;
                let _r0_n6 = _r0_n4 + _r0_n5;
                let b1_i5 = _r0_n6;
                if b1_i5 < i128::from(i64::MIN) || b1_i5 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2, b1_i3, b1_i4, b1_i5]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b1_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(2));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2, b1_i3, b1_i4, b1_i5]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[b1_b0]);
                CompiledResume::Next(2)
            }

            fn bit_array_int_0_resume_2(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_i1, b1_i2, b1_i3, b1_i4, b1_i5, b1_b0,) = (values.ints[0], values.ints[1], values.ints[2], values.ints[3], values.ints[4], values.ints[5], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2, b1_i3, b1_i4, b1_i5]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b1_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                {
                    let (b0_i0, b0_b0,) = (b1_i5, b1_b0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b0_b0]);
                    CompiledResume::Next(0)
                }
            }

            fn bit_array_int_0_resume_3(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_b0,) = (values.ints[0], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b2_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                let _matched = (|| -> Option<()> {
                    let mut _offset = 0_usize;
                    if _offset != b2_b0.bit_len() { return None; }
                    Some(())
                })();
                match _matched {
                    Some(()) => {
                        let (b3_i0,) = (b2_i0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        CompiledResume::Next(4)
                    },
                    None => {
                        let () = ();

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        CompiledResume::Next(5)
                    },
                }
            }

            fn bit_array_int_0_resume_4(
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
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn bit_array_int_0_resume_5(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                let _ = budget;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(5))
            }

            fn bit_array_int_1(
                point: usize,
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::bit_array::BitArrayValues, &mut usize) -> CompiledResume;
                    7
                ] = [
                    |values, budget| CompiledResume::Exit(bit_array_int_1_entry((values.ints[0], values.bit_arrays[0],), values, budget)),
                    bit_array_int_1_resume_1,
                    bit_array_int_1_resume_2,
                    bit_array_int_1_resume_3,
                    bit_array_int_1_resume_4,
                    bit_array_int_1_resume_5,
                    bit_array_int_1_resume_6,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn bit_array_int_1_entry(
                inputs: (i128, data::compiled::bit_array::BitArrayRange,),
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (mut b0_i0, mut b0_b0,) = inputs;
                'repeat: loop {
                    let (b1_i0, b1_i1, b1_b0,) = 'block_1: {
                        let (b3_i0,) = 'block_3: {
                            let () = 'block_4: {
                                let (b2_i0, b2_b0,) = 'block_2: {
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b0_i0]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.bit_arrays.clear();
                                        values.bit_arrays.extend_from_slice(&[b0_b0]);
                                        return data::compiled::CompiledProgress::Yield(0);
                                    }
                                    *budget -= 1;
                                    let _matched = (|| -> Option<(i128, data::compiled::bit_array::BitArrayRange,)> {
                                        let mut _offset = 0_usize;
                                        let _field_0 = values.integer(
                                            b0_b0,
                                            _offset,
                                            64_usize,
                                            data::graph::Endianness::Big,
                                            data::graph::Signedness::Unsigned,
                                        )?;
                                        let _matched_0 = _field_0;
                                        _offset = _offset.checked_add(64_usize)?;
                                        let _length_1 = b0_b0.bit_len().checked_sub(_offset)?;
                                        let _range_1 = b0_b0.slice(_offset, _length_1)?;
                                        let _matched_1 = _range_1;
                                        _offset = _offset.checked_add(_length_1)?;
                                        if _offset != b0_b0.bit_len() { return None; }
                                        Some((_matched_0, _matched_1,))
                                    })();
                                    match _matched {
                                        Some((_matched_0, _matched_1,)) => {
                                            if _matched_0 > i128::from(i64::MAX) {
                                                let (b1_i0, b1_i1, b1_b0,) = (_matched_0, b0_i0, _matched_1,);

                                                values.ints.clear();
                                                values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                                                values.bools.clear();
                                                values.bools.extend_from_slice(&[]);
                                                values.bit_arrays.clear();
                                                values.bit_arrays.extend_from_slice(&[b1_b0]);
                                                return data::compiled::CompiledProgress::Interpreted(1);
                                            }
                                            break 'block_1 (_matched_0, b0_i0, _matched_1,);
                                        },
                                        None => {
                                            break 'block_2 (b0_i0, b0_b0,);
                                        },
                                    }
                                };
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b2_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.bit_arrays.clear();
                                    values.bit_arrays.extend_from_slice(&[b2_b0]);
                                    return data::compiled::CompiledProgress::Yield(3);
                                }
                                *budget -= 1;
                                let _matched = (|| -> Option<()> {
                                    let mut _offset = 0_usize;
                                    if _offset != b2_b0.bit_len() { return None; }
                                    Some(())
                                })();
                                match _matched {
                                    Some(()) => {
                                        break 'block_3 (b2_i0,);
                                    },
                                    None => {
                                        break 'block_4;
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
                            values.ints.extend_from_slice(&[b3_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.bit_arrays.clear();
                            values.bit_arrays.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(4);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0));
                    };
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b1_b0]);
                        return data::compiled::CompiledProgress::Yield(1);
                    }
                    *budget -= 1;
                    let b1_i2 = b1_i1 + b1_i0;
                    if b1_i2 < i128::from(i64::MIN) || b1_i2 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b1_b0]);
                        return data::compiled::CompiledProgress::Interpreted(2);
                    }
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b1_b0]);
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;
                    {
                        (b0_i0, b0_b0,) = (b1_i2, b1_b0,);
                        continue 'repeat;
                    }
                }
            }

            fn bit_array_int_1_resume_1(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_i1, b1_b0,) = (values.ints[0], values.ints[1], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b1_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                let b1_i2 = b1_i1 + b1_i0;
                if b1_i2 < i128::from(i64::MIN) || b1_i2 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b1_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(2));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[b1_b0]);
                CompiledResume::Next(2)
            }

            fn bit_array_int_1_resume_2(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_i1, b1_i2, b1_b0,) = (values.ints[0], values.ints[1], values.ints[2], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b1_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                {
                    let (b0_i0, b0_b0,) = (b1_i2, b1_b0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b0_b0]);
                    CompiledResume::Next(0)
                }
            }

            fn bit_array_int_1_resume_3(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_b0,) = (values.ints[0], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b2_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                let _matched = (|| -> Option<()> {
                    let mut _offset = 0_usize;
                    if _offset != b2_b0.bit_len() { return None; }
                    Some(())
                })();
                match _matched {
                    Some(()) => {
                        let (b3_i0,) = (b2_i0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        CompiledResume::Next(4)
                    },
                    None => {
                        let () = ();

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        CompiledResume::Next(5)
                    },
                }
            }

            fn bit_array_int_1_resume_4(
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
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn bit_array_int_1_resume_5(
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

            fn bit_array_int_1_resume_6(
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

            fn bit_array_int_2(
                point: usize,
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::bit_array::BitArrayValues, &mut usize) -> CompiledResume;
                    13
                ] = [
                    |values, budget| CompiledResume::Exit(bit_array_int_2_entry((values.ints[0], values.bit_arrays[0],), values, budget)),
                    bit_array_int_2_resume_1,
                    bit_array_int_2_resume_2,
                    bit_array_int_2_resume_3,
                    bit_array_int_2_resume_4,
                    bit_array_int_2_resume_5,
                    bit_array_int_2_resume_6,
                    bit_array_int_2_resume_7,
                    bit_array_int_2_resume_8,
                    bit_array_int_2_resume_9,
                    bit_array_int_2_resume_10,
                    bit_array_int_2_resume_11,
                    bit_array_int_2_resume_12,
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
                inputs: (i128, data::compiled::bit_array::BitArrayRange,),
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (mut b0_i0, mut b0_b0,) = inputs;
                'repeat: loop {
                    let (b3_i0, b3_i1, b3_i2, b3_b0,) = 'block_3: {
                        let () = 'block_4: {
                            let (b2_i0, b2_i1, b2_i2, b2_b0, b2_b1,) = 'block_2: {
                                let () = 'block_5: {
                                    let (b1_i0, b1_i1, b1_b0, b1_b1, b1_b2,) = 'block_1: {
                                        let (b7_i0,) = 'block_7: {
                                            let () = 'block_8: {
                                                let (b6_i0, b6_b0,) = 'block_6: {
                                                    if *budget == 0 {

                                                        values.ints.clear();
                                                        values.ints.extend_from_slice(&[b0_i0]);
                                                        values.bools.clear();
                                                        values.bools.extend_from_slice(&[]);
                                                        values.bit_arrays.clear();
                                                        values.bit_arrays.extend_from_slice(&[b0_b0]);
                                                        return data::compiled::CompiledProgress::Yield(0);
                                                    }
                                                    *budget -= 1;
                                                    let _matched = (|| -> Option<(i128, data::compiled::bit_array::BitArrayRange, data::compiled::bit_array::BitArrayRange,)> {
                                                        let mut _offset = 0_usize;
                                                        let _field_0 = values.integer(
                                                            b0_b0,
                                                            _offset,
                                                            8_usize,
                                                            data::graph::Endianness::Big,
                                                            data::graph::Signedness::Unsigned,
                                                        )?;
                                                        if _field_0 != 1_i128 { return None; }
                                                        let _matched_0 = _field_0;
                                                        _offset = _offset.checked_add(8_usize)?;
                                                        let _field_1 = values.integer(
                                                            b0_b0,
                                                            _offset,
                                                            8_usize,
                                                            data::graph::Endianness::Big,
                                                            data::graph::Signedness::Unsigned,
                                                        )?;
                                                        if _field_1 != 2_i128 { return None; }
                                                        _offset = _offset.checked_add(8_usize)?;
                                                        let _length_2 = usize::try_from(8_u64).ok()?;
                                                        let _range_2 = b0_b0.slice(_offset, _length_2)?;
                                                        let _matched_1 = _range_2;
                                                        _offset = _offset.checked_add(_length_2)?;
                                                        let _length_3 = b0_b0.bit_len().checked_sub(_offset)?;
                                                        if !_length_3.is_multiple_of(8_usize) { return None; }
                                                        let _range_3 = b0_b0.slice(_offset, _length_3)?;
                                                        let _matched_2 = _range_3;
                                                        _offset = _offset.checked_add(_length_3)?;
                                                        if _offset != b0_b0.bit_len() { return None; }
                                                        Some((_matched_0, _matched_1, _matched_2,))
                                                    })();
                                                    match _matched {
                                                        Some((_matched_0, _matched_1, _matched_2,)) => {
                                                            break 'block_1 (_matched_0, b0_i0, _matched_1, _matched_2, b0_b0,);
                                                        },
                                                        None => {
                                                            break 'block_6 (b0_i0, b0_b0,);
                                                        },
                                                    }
                                                };
                                                if *budget == 0 {

                                                    values.ints.clear();
                                                    values.ints.extend_from_slice(&[b6_i0]);
                                                    values.bools.clear();
                                                    values.bools.extend_from_slice(&[]);
                                                    values.bit_arrays.clear();
                                                    values.bit_arrays.extend_from_slice(&[b6_b0]);
                                                    return data::compiled::CompiledProgress::Yield(9);
                                                }
                                                *budget -= 1;
                                                let _matched = (|| -> Option<()> {
                                                    let mut _offset = 0_usize;
                                                    if _offset != b6_b0.bit_len() { return None; }
                                                    Some(())
                                                })();
                                                match _matched {
                                                    Some(()) => {
                                                        break 'block_7 (b6_i0,);
                                                    },
                                                    None => {
                                                        break 'block_8;
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
                                                return data::compiled::CompiledProgress::Yield(11);
                                            }
                                            *budget -= 1;
                                            let b8_i0 = -1_i128;
                                            if *budget == 0 {

                                                values.ints.clear();
                                                values.ints.extend_from_slice(&[b8_i0]);
                                                values.bools.clear();
                                                values.bools.extend_from_slice(&[]);
                                                values.bit_arrays.clear();
                                                values.bit_arrays.extend_from_slice(&[]);
                                                return data::compiled::CompiledProgress::Yield(12);
                                            }
                                            *budget -= 1;

                                            values.ints.clear();
                                            values.ints.extend_from_slice(&[b8_i0]);
                                            values.bools.clear();
                                            values.bools.extend_from_slice(&[]);
                                            values.bit_arrays.clear();
                                            values.bit_arrays.extend_from_slice(&[]);
                                            return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(3));
                                        };
                                        if *budget == 0 {

                                            values.ints.clear();
                                            values.ints.extend_from_slice(&[b7_i0]);
                                            values.bools.clear();
                                            values.bools.extend_from_slice(&[]);
                                            values.bit_arrays.clear();
                                            values.bit_arrays.extend_from_slice(&[]);
                                            return data::compiled::CompiledProgress::Yield(10);
                                        }
                                        *budget -= 1;

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b7_i0]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.bit_arrays.clear();
                                        values.bit_arrays.extend_from_slice(&[]);
                                        return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2));
                                    };
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.bit_arrays.clear();
                                        values.bit_arrays.extend_from_slice(&[b1_b0, b1_b1, b1_b2]);
                                        return data::compiled::CompiledProgress::Yield(1);
                                    }
                                    *budget -= 1;
                                    let _matched = (|| -> Option<(i128,)> {
                                        let mut _offset = 0_usize;
                                        let _field_0 = values.integer(
                                            b1_b0,
                                            _offset,
                                            8_usize,
                                            data::graph::Endianness::Big,
                                            data::graph::Signedness::Unsigned,
                                        )?;
                                        let _matched_0 = _field_0;
                                        _offset = _offset.checked_add(8_usize)?;
                                        if _offset != b1_b0.bit_len() { return None; }
                                        Some((_matched_0,))
                                    })();
                                    match _matched {
                                        Some((_matched_0,)) => {
                                            break 'block_2 (_matched_0, b1_i1, b1_i0, b1_b2, b1_b1,);
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
                                let b5_i0 = -2_i128;
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b5_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.bit_arrays.clear();
                                    values.bit_arrays.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Yield(8);
                                }
                                *budget -= 1;

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b5_i0]);
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
                                values.bit_arrays.extend_from_slice(&[b2_b0, b2_b1]);
                                return data::compiled::CompiledProgress::Yield(2);
                            }
                            *budget -= 1;
                            let _matched = (|| -> Option<()> {
                                let mut _offset = 0_usize;
                                let _field_0 = values.integer(
                                    b2_b0,
                                    _offset,
                                    24_usize,
                                    data::graph::Endianness::Big,
                                    data::graph::Signedness::Unsigned,
                                )?;
                                _offset = _offset.checked_add(24_usize)?;
                                let _length_1 = b2_b0.bit_len().checked_sub(_offset)?;
                                if !_length_1.is_multiple_of(8_usize) { return None; }
                                let _range_1 = b2_b0.slice(_offset, _length_1)?;
                                _offset = _offset.checked_add(_length_1)?;
                                if _offset != b2_b0.bit_len() { return None; }
                                Some(())
                            })();
                            match _matched {
                                Some(()) => {
                                    break 'block_3 (b2_i1, b2_i2, b2_i0, b2_b1,);
                                },
                                None => {
                                    break 'block_4;
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
                            return data::compiled::CompiledProgress::Yield(5);
                        }
                        *budget -= 1;
                        let b4_i0 = -2_i128;
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
                        return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0));
                    };
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b3_b0]);
                        return data::compiled::CompiledProgress::Yield(3);
                    }
                    *budget -= 1;
                    let _r0_n0 = b3_i0 + b3_i1;
                    let _r0_n1 = _r0_n0 + b3_i2;
                    let b3_i3 = _r0_n1;
                    if b3_i3 < i128::from(i64::MIN) || b3_i3 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b3_b0]);
                        return data::compiled::CompiledProgress::Interpreted(4);
                    }
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b3_b0]);
                        return data::compiled::CompiledProgress::Yield(4);
                    }
                    *budget -= 1;
                    {
                        (b0_i0, b0_b0,) = (b3_i3, b3_b0,);
                        continue 'repeat;
                    }
                }
            }

            fn bit_array_int_2_resume_1(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_i1, b1_b0, b1_b1, b1_b2,) = (values.ints[0], values.ints[1], values.bit_arrays[0], values.bit_arrays[1], values.bit_arrays[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b1_b0, b1_b1, b1_b2]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                let _matched = (|| -> Option<(i128,)> {
                    let mut _offset = 0_usize;
                    let _field_0 = values.integer(
                        b1_b0,
                        _offset,
                        8_usize,
                        data::graph::Endianness::Big,
                        data::graph::Signedness::Unsigned,
                    )?;
                    let _matched_0 = _field_0;
                    _offset = _offset.checked_add(8_usize)?;
                    if _offset != b1_b0.bit_len() { return None; }
                    Some((_matched_0,))
                })();
                match _matched {
                    Some((_matched_0,)) => {
                        let (b2_i0, b2_i1, b2_i2, b2_b0, b2_b1,) = (_matched_0, b1_i1, b1_i0, b1_b2, b1_b1,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b2_b0, b2_b1]);
                        CompiledResume::Next(2)
                    },
                    None => {
                        let () = ();

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        CompiledResume::Next(7)
                    },
                }
            }

            fn bit_array_int_2_resume_2(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_i1, b2_i2, b2_b0, b2_b1,) = (values.ints[0], values.ints[1], values.ints[2], values.bit_arrays[0], values.bit_arrays[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b2_b0, b2_b1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                let _matched = (|| -> Option<()> {
                    let mut _offset = 0_usize;
                    let _field_0 = values.integer(
                        b2_b0,
                        _offset,
                        24_usize,
                        data::graph::Endianness::Big,
                        data::graph::Signedness::Unsigned,
                    )?;
                    _offset = _offset.checked_add(24_usize)?;
                    let _length_1 = b2_b0.bit_len().checked_sub(_offset)?;
                    if !_length_1.is_multiple_of(8_usize) { return None; }
                    let _range_1 = b2_b0.slice(_offset, _length_1)?;
                    _offset = _offset.checked_add(_length_1)?;
                    if _offset != b2_b0.bit_len() { return None; }
                    Some(())
                })();
                match _matched {
                    Some(()) => {
                        let (b3_i0, b3_i1, b3_i2, b3_b0,) = (b2_i1, b2_i2, b2_i0, b2_b1,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b3_b0]);
                        CompiledResume::Next(3)
                    },
                    None => {
                        let () = ();

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        CompiledResume::Next(5)
                    },
                }
            }

            fn bit_array_int_2_resume_3(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0, b3_i1, b3_i2, b3_b0,) = (values.ints[0], values.ints[1], values.ints[2], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b3_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                let _r0_n0 = b3_i0 + b3_i1;
                let _r0_n1 = _r0_n0 + b3_i2;
                let b3_i3 = _r0_n1;
                if b3_i3 < i128::from(i64::MIN) || b3_i3 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b3_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(4));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[b3_b0]);
                CompiledResume::Next(4)
            }

            fn bit_array_int_2_resume_4(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0, b3_i1, b3_i2, b3_i3, b3_b0,) = (values.ints[0], values.ints[1], values.ints[2], values.ints[3], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b3_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;
                {
                    let (b0_i0, b0_b0,) = (b3_i3, b3_b0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b0_b0]);
                    CompiledResume::Next(0)
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
                let b4_i0 = -2_i128;

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
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
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
                let b5_i0 = -2_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b5_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Next(8)
            }

            fn bit_array_int_2_resume_8(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b5_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b5_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn bit_array_int_2_resume_9(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b6_i0, b6_b0,) = (values.ints[0], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b6_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b6_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(9));
                }
                *budget -= 1;
                let _matched = (|| -> Option<()> {
                    let mut _offset = 0_usize;
                    if _offset != b6_b0.bit_len() { return None; }
                    Some(())
                })();
                match _matched {
                    Some(()) => {
                        let (b7_i0,) = (b6_i0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b7_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        CompiledResume::Next(10)
                    },
                    None => {
                        let () = ();

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        CompiledResume::Next(11)
                    },
                }
            }

            fn bit_array_int_2_resume_10(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b7_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b7_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(10));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b7_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2)))
            }

            fn bit_array_int_2_resume_11(
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
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(11));
                }
                *budget -= 1;
                let b8_i0 = -1_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b8_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Next(12)
            }

            fn bit_array_int_2_resume_12(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b8_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b8_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(12));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b8_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(3)))
            }

            fn bit_array_int_3(
                point: usize,
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::bit_array::BitArrayValues, &mut usize) -> CompiledResume;
                    7
                ] = [
                    |values, budget| CompiledResume::Exit(bit_array_int_3_entry((values.ints[0], values.bit_arrays[0],), values, budget)),
                    bit_array_int_3_resume_1,
                    bit_array_int_3_resume_2,
                    bit_array_int_3_resume_3,
                    bit_array_int_3_resume_4,
                    bit_array_int_3_resume_5,
                    bit_array_int_3_resume_6,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn bit_array_int_3_entry(
                inputs: (i128, data::compiled::bit_array::BitArrayRange,),
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (mut b0_i0, mut b0_b0,) = inputs;
                'repeat: loop {
                    let (b1_i0, b1_i1, b1_b0,) = 'block_1: {
                        let (b3_i0,) = 'block_3: {
                            let () = 'block_4: {
                                let (b2_i0, b2_b0,) = 'block_2: {
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b0_i0]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.bit_arrays.clear();
                                        values.bit_arrays.extend_from_slice(&[b0_b0]);
                                        return data::compiled::CompiledProgress::Yield(0);
                                    }
                                    *budget -= 1;
                                    let _matched = (|| -> Option<(i128, data::compiled::bit_array::BitArrayRange,)> {
                                        let mut _offset = 0_usize;
                                        let _field_0 = values.integer(
                                            b0_b0,
                                            _offset,
                                            9_usize,
                                            data::graph::Endianness::Little,
                                            data::graph::Signedness::Signed,
                                        )?;
                                        let _matched_0 = _field_0;
                                        _offset = _offset.checked_add(9_usize)?;
                                        let _length_1 = b0_b0.bit_len().checked_sub(_offset)?;
                                        let _range_1 = b0_b0.slice(_offset, _length_1)?;
                                        let _matched_1 = _range_1;
                                        _offset = _offset.checked_add(_length_1)?;
                                        if _offset != b0_b0.bit_len() { return None; }
                                        Some((_matched_0, _matched_1,))
                                    })();
                                    match _matched {
                                        Some((_matched_0, _matched_1,)) => {
                                            break 'block_1 (_matched_0, b0_i0, _matched_1,);
                                        },
                                        None => {
                                            break 'block_2 (b0_i0, b0_b0,);
                                        },
                                    }
                                };
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b2_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.bit_arrays.clear();
                                    values.bit_arrays.extend_from_slice(&[b2_b0]);
                                    return data::compiled::CompiledProgress::Yield(3);
                                }
                                *budget -= 1;
                                let _matched = (|| -> Option<()> {
                                    let mut _offset = 0_usize;
                                    if _offset != b2_b0.bit_len() { return None; }
                                    Some(())
                                })();
                                match _matched {
                                    Some(()) => {
                                        break 'block_3 (b2_i0,);
                                    },
                                    None => {
                                        break 'block_4;
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
                            values.ints.extend_from_slice(&[b3_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.bit_arrays.clear();
                            values.bit_arrays.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(4);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0));
                    };
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b1_b0]);
                        return data::compiled::CompiledProgress::Yield(1);
                    }
                    *budget -= 1;
                    let b1_i2 = b1_i1 + b1_i0;
                    if b1_i2 < i128::from(i64::MIN) || b1_i2 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b1_b0]);
                        return data::compiled::CompiledProgress::Interpreted(2);
                    }
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b1_b0]);
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;
                    {
                        (b0_i0, b0_b0,) = (b1_i2, b1_b0,);
                        continue 'repeat;
                    }
                }
            }

            fn bit_array_int_3_resume_1(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_i1, b1_b0,) = (values.ints[0], values.ints[1], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b1_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                let b1_i2 = b1_i1 + b1_i0;
                if b1_i2 < i128::from(i64::MIN) || b1_i2 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b1_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(2));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[b1_b0]);
                CompiledResume::Next(2)
            }

            fn bit_array_int_3_resume_2(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_i1, b1_i2, b1_b0,) = (values.ints[0], values.ints[1], values.ints[2], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b1_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                {
                    let (b0_i0, b0_b0,) = (b1_i2, b1_b0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b0_b0]);
                    CompiledResume::Next(0)
                }
            }

            fn bit_array_int_3_resume_3(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_b0,) = (values.ints[0], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b2_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                let _matched = (|| -> Option<()> {
                    let mut _offset = 0_usize;
                    if _offset != b2_b0.bit_len() { return None; }
                    Some(())
                })();
                match _matched {
                    Some(()) => {
                        let (b3_i0,) = (b2_i0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        CompiledResume::Next(4)
                    },
                    None => {
                        let () = ();

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        CompiledResume::Next(5)
                    },
                }
            }

            fn bit_array_int_3_resume_4(
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
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn bit_array_int_3_resume_5(
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

            fn bit_array_int_3_resume_6(
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

            fn bit_array_int_4(
                point: usize,
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::bit_array::BitArrayValues, &mut usize) -> CompiledResume;
                    4
                ] = [
                    |values, budget| CompiledResume::Exit(bit_array_int_4_entry((values.bit_arrays[0],), values, budget)),
                    bit_array_int_4_resume_1,
                    bit_array_int_4_resume_2,
                    bit_array_int_4_resume_3,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn bit_array_int_4_entry(
                inputs: (data::compiled::bit_array::BitArrayRange,),
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (b0_b0,) = inputs;
                let (b1_i0,) = 'block_1: {
                    let () = 'block_2: {
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
                        let _matched = (|| -> Option<(i128,)> {
                            let mut _offset = 0_usize;
                            let _field_0 = values.integer(
                                b0_b0,
                                _offset,
                                64_usize,
                                data::graph::Endianness::Big,
                                data::graph::Signedness::Unsigned,
                            )?;
                            let _matched_0 = _field_0;
                            _offset = _offset.checked_add(64_usize)?;
                            let _field_1 = values.integer(
                                b0_b0,
                                _offset,
                                8_usize,
                                data::graph::Endianness::Big,
                                data::graph::Signedness::Unsigned,
                            )?;
                            if _field_1 != 255_i128 { return None; }
                            _offset = _offset.checked_add(8_usize)?;
                            if _offset != b0_b0.bit_len() { return None; }
                            Some((_matched_0,))
                        })();
                        match _matched {
                            Some((_matched_0,)) => {
                                if _matched_0 > i128::from(i64::MAX) {
                                    let (b1_i0,) = (_matched_0,);

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b1_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.bit_arrays.clear();
                                    values.bit_arrays.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Interpreted(1);
                                }
                                break 'block_1 (_matched_0,);
                            },
                            None => {
                                break 'block_2;
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
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;
                    let b2_i0 = -1_i128;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(3);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1));
                };
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return data::compiled::CompiledProgress::Yield(1);
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
            }

            fn bit_array_int_4_resume_1(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn bit_array_int_4_resume_2(
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
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                let b2_i0 = -1_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Next(3)
            }

            fn bit_array_int_4_resume_3(
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
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn bit_array_int_5(
                point: usize,
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::bit_array::BitArrayValues, &mut usize) -> CompiledResume;
                    13
                ] = [
                    |values, budget| CompiledResume::Exit(bit_array_int_5_entry((values.ints[0], values.bit_arrays[0], values.bit_arrays[1],), values, budget)),
                    bit_array_int_5_resume_1,
                    bit_array_int_5_resume_2,
                    bit_array_int_5_resume_3,
                    bit_array_int_5_resume_4,
                    bit_array_int_5_resume_5,
                    bit_array_int_5_resume_6,
                    bit_array_int_5_resume_7,
                    bit_array_int_5_resume_8,
                    bit_array_int_5_resume_9,
                    bit_array_int_5_resume_10,
                    bit_array_int_5_resume_11,
                    bit_array_int_5_resume_12,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn bit_array_int_5_entry(
                inputs: (i128, data::compiled::bit_array::BitArrayRange, data::compiled::bit_array::BitArrayRange,),
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (mut b0_i0, mut b0_b0, mut b0_b1,) = inputs;
                'repeat: loop {
                    let (b2_i0, b2_i1, b2_i2, b2_b0, b2_b1,) = 'block_2: {
                        let () = 'block_3: {
                            let (b1_i0, b1_i1, b1_b0, b1_b1,) = 'block_1: {
                                let (b6_i0,) = 'block_6: {
                                    let () = 'block_7: {
                                        let (b5_i0, b5_b0,) = 'block_5: {
                                            let () = 'block_8: {
                                                let (b4_i0, b4_b0, b4_b1,) = 'block_4: {
                                                    if *budget == 0 {

                                                        values.ints.clear();
                                                        values.ints.extend_from_slice(&[b0_i0]);
                                                        values.bools.clear();
                                                        values.bools.extend_from_slice(&[]);
                                                        values.bit_arrays.clear();
                                                        values.bit_arrays.extend_from_slice(&[b0_b0, b0_b1]);
                                                        return data::compiled::CompiledProgress::Yield(0);
                                                    }
                                                    *budget -= 1;
                                                    let _matched = (|| -> Option<(i128, data::compiled::bit_array::BitArrayRange,)> {
                                                        let mut _offset = 0_usize;
                                                        let _field_0 = values.integer(
                                                            b0_b0,
                                                            _offset,
                                                            7_usize,
                                                            data::graph::Endianness::Big,
                                                            data::graph::Signedness::Unsigned,
                                                        )?;
                                                        let _matched_0 = _field_0;
                                                        _offset = _offset.checked_add(7_usize)?;
                                                        let _length_1 = b0_b0.bit_len().checked_sub(_offset)?;
                                                        let _range_1 = b0_b0.slice(_offset, _length_1)?;
                                                        let _matched_1 = _range_1;
                                                        _offset = _offset.checked_add(_length_1)?;
                                                        if _offset != b0_b0.bit_len() { return None; }
                                                        Some((_matched_0, _matched_1,))
                                                    })();
                                                    match _matched {
                                                        Some((_matched_0, _matched_1,)) => {
                                                            break 'block_1 (_matched_0, b0_i0, _matched_1, b0_b1,);
                                                        },
                                                        None => {
                                                            break 'block_4 (b0_i0, b0_b0, b0_b1,);
                                                        },
                                                    }
                                                };
                                                if *budget == 0 {

                                                    values.ints.clear();
                                                    values.ints.extend_from_slice(&[b4_i0]);
                                                    values.bools.clear();
                                                    values.bools.extend_from_slice(&[]);
                                                    values.bit_arrays.clear();
                                                    values.bit_arrays.extend_from_slice(&[b4_b0, b4_b1]);
                                                    return data::compiled::CompiledProgress::Yield(6);
                                                }
                                                *budget -= 1;
                                                let _matched = (|| -> Option<()> {
                                                    let mut _offset = 0_usize;
                                                    if _offset != b4_b0.bit_len() { return None; }
                                                    Some(())
                                                })();
                                                match _matched {
                                                    Some(()) => {
                                                        break 'block_5 (b4_i0, b4_b1,);
                                                    },
                                                    None => {
                                                        break 'block_8;
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
                                                return data::compiled::CompiledProgress::Yield(11);
                                            }
                                            *budget -= 1;
                                            let b8_i0 = -1_i128;
                                            if *budget == 0 {

                                                values.ints.clear();
                                                values.ints.extend_from_slice(&[b8_i0]);
                                                values.bools.clear();
                                                values.bools.extend_from_slice(&[]);
                                                values.bit_arrays.clear();
                                                values.bit_arrays.extend_from_slice(&[]);
                                                return data::compiled::CompiledProgress::Yield(12);
                                            }
                                            *budget -= 1;

                                            values.ints.clear();
                                            values.ints.extend_from_slice(&[b8_i0]);
                                            values.bools.clear();
                                            values.bools.extend_from_slice(&[]);
                                            values.bit_arrays.clear();
                                            values.bit_arrays.extend_from_slice(&[]);
                                            return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(3));
                                        };
                                        if *budget == 0 {

                                            values.ints.clear();
                                            values.ints.extend_from_slice(&[b5_i0]);
                                            values.bools.clear();
                                            values.bools.extend_from_slice(&[]);
                                            values.bit_arrays.clear();
                                            values.bit_arrays.extend_from_slice(&[b5_b0]);
                                            return data::compiled::CompiledProgress::Yield(7);
                                        }
                                        *budget -= 1;
                                        let _matched = (|| -> Option<()> {
                                            let mut _offset = 0_usize;
                                            if _offset != b5_b0.bit_len() { return None; }
                                            Some(())
                                        })();
                                        match _matched {
                                            Some(()) => {
                                                break 'block_6 (b5_i0,);
                                            },
                                            None => {
                                                break 'block_7;
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
                                        return data::compiled::CompiledProgress::Yield(9);
                                    }
                                    *budget -= 1;
                                    let b7_i0 = -1_i128;
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b7_i0]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.bit_arrays.clear();
                                        values.bit_arrays.extend_from_slice(&[]);
                                        return data::compiled::CompiledProgress::Yield(10);
                                    }
                                    *budget -= 1;

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b7_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.bit_arrays.clear();
                                    values.bit_arrays.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2));
                                };
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b6_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.bit_arrays.clear();
                                    values.bit_arrays.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Yield(8);
                                }
                                *budget -= 1;

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b6_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.bit_arrays.clear();
                                values.bit_arrays.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1));
                            };
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.bit_arrays.clear();
                                values.bit_arrays.extend_from_slice(&[b1_b0, b1_b1]);
                                return data::compiled::CompiledProgress::Yield(1);
                            }
                            *budget -= 1;
                            let _matched = (|| -> Option<(i128, data::compiled::bit_array::BitArrayRange,)> {
                                let mut _offset = 0_usize;
                                let _field_0 = values.integer(
                                    b1_b1,
                                    _offset,
                                    7_usize,
                                    data::graph::Endianness::Big,
                                    data::graph::Signedness::Unsigned,
                                )?;
                                let _matched_0 = _field_0;
                                _offset = _offset.checked_add(7_usize)?;
                                let _length_1 = b1_b1.bit_len().checked_sub(_offset)?;
                                let _range_1 = b1_b1.slice(_offset, _length_1)?;
                                let _matched_1 = _range_1;
                                _offset = _offset.checked_add(_length_1)?;
                                if _offset != b1_b1.bit_len() { return None; }
                                Some((_matched_0, _matched_1,))
                            })();
                            match _matched {
                                Some((_matched_0, _matched_1,)) => {
                                    break 'block_2 (_matched_0, b1_i1, b1_i0, _matched_1, b1_b0,);
                                },
                                None => {
                                    break 'block_3;
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
                            return data::compiled::CompiledProgress::Yield(4);
                        }
                        *budget -= 1;
                        let b3_i0 = -1_i128;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b3_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.bit_arrays.clear();
                            values.bit_arrays.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(5);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0));
                    };
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b2_b0, b2_b1]);
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;
                    let _r0_n0 = b2_i1 + b2_i2;
                    let _r0_n1 = _r0_n0 + b2_i0;
                    let b2_i3 = _r0_n1;
                    if b2_i3 < i128::from(i64::MIN) || b2_i3 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2, b2_i3]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b2_b0, b2_b1]);
                        return data::compiled::CompiledProgress::Interpreted(3);
                    }
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2, b2_i3]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b2_b0, b2_b1]);
                        return data::compiled::CompiledProgress::Yield(3);
                    }
                    *budget -= 1;
                    {
                        (b0_i0, b0_b0, b0_b1,) = (b2_i3, b2_b1, b2_b0,);
                        continue 'repeat;
                    }
                }
            }

            fn bit_array_int_5_resume_1(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_i1, b1_b0, b1_b1,) = (values.ints[0], values.ints[1], values.bit_arrays[0], values.bit_arrays[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b1_b0, b1_b1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                let _matched = (|| -> Option<(i128, data::compiled::bit_array::BitArrayRange,)> {
                    let mut _offset = 0_usize;
                    let _field_0 = values.integer(
                        b1_b1,
                        _offset,
                        7_usize,
                        data::graph::Endianness::Big,
                        data::graph::Signedness::Unsigned,
                    )?;
                    let _matched_0 = _field_0;
                    _offset = _offset.checked_add(7_usize)?;
                    let _length_1 = b1_b1.bit_len().checked_sub(_offset)?;
                    let _range_1 = b1_b1.slice(_offset, _length_1)?;
                    let _matched_1 = _range_1;
                    _offset = _offset.checked_add(_length_1)?;
                    if _offset != b1_b1.bit_len() { return None; }
                    Some((_matched_0, _matched_1,))
                })();
                match _matched {
                    Some((_matched_0, _matched_1,)) => {
                        let (b2_i0, b2_i1, b2_i2, b2_b0, b2_b1,) = (_matched_0, b1_i1, b1_i0, _matched_1, b1_b0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b2_b0, b2_b1]);
                        CompiledResume::Next(2)
                    },
                    None => {
                        let () = ();

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        CompiledResume::Next(4)
                    },
                }
            }

            fn bit_array_int_5_resume_2(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_i1, b2_i2, b2_b0, b2_b1,) = (values.ints[0], values.ints[1], values.ints[2], values.bit_arrays[0], values.bit_arrays[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b2_b0, b2_b1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                let _r0_n0 = b2_i1 + b2_i2;
                let _r0_n1 = _r0_n0 + b2_i0;
                let b2_i3 = _r0_n1;
                if b2_i3 < i128::from(i64::MIN) || b2_i3 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2, b2_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b2_b0, b2_b1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(3));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2, b2_i3]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[b2_b0, b2_b1]);
                CompiledResume::Next(3)
            }

            fn bit_array_int_5_resume_3(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_i1, b2_i2, b2_i3, b2_b0, b2_b1,) = (values.ints[0], values.ints[1], values.ints[2], values.ints[3], values.bit_arrays[0], values.bit_arrays[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2, b2_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b2_b0, b2_b1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                {
                    let (b0_i0, b0_b0, b0_b1,) = (b2_i3, b2_b1, b2_b0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b0_b0, b0_b1]);
                    CompiledResume::Next(0)
                }
            }

            fn bit_array_int_5_resume_4(
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
                let b3_i0 = -1_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Next(5)
            }

            fn bit_array_int_5_resume_5(
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
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn bit_array_int_5_resume_6(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_i0, b4_b0, b4_b1,) = (values.ints[0], values.bit_arrays[0], values.bit_arrays[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b4_b0, b4_b1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;
                let _matched = (|| -> Option<()> {
                    let mut _offset = 0_usize;
                    if _offset != b4_b0.bit_len() { return None; }
                    Some(())
                })();
                match _matched {
                    Some(()) => {
                        let (b5_i0, b5_b0,) = (b4_i0, b4_b1,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b5_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b5_b0]);
                        CompiledResume::Next(7)
                    },
                    None => {
                        let () = ();

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        CompiledResume::Next(11)
                    },
                }
            }

            fn bit_array_int_5_resume_7(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b5_i0, b5_b0,) = (values.ints[0], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b5_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                }
                *budget -= 1;
                let _matched = (|| -> Option<()> {
                    let mut _offset = 0_usize;
                    if _offset != b5_b0.bit_len() { return None; }
                    Some(())
                })();
                match _matched {
                    Some(()) => {
                        let (b6_i0,) = (b5_i0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b6_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        CompiledResume::Next(8)
                    },
                    None => {
                        let () = ();

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        CompiledResume::Next(9)
                    },
                }
            }

            fn bit_array_int_5_resume_8(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b6_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b6_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b6_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn bit_array_int_5_resume_9(
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
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(9));
                }
                *budget -= 1;
                let b7_i0 = -1_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b7_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Next(10)
            }

            fn bit_array_int_5_resume_10(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b7_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b7_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(10));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b7_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2)))
            }

            fn bit_array_int_5_resume_11(
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
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(11));
                }
                *budget -= 1;
                let b8_i0 = -1_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b8_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Next(12)
            }

            fn bit_array_int_5_resume_12(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b8_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b8_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(12));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b8_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(3)))
            }

            fn bit_array_bool_0(
                point: usize,
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::bit_array::BitArrayValues, &mut usize) -> CompiledResume;
                    7
                ] = [
                    |values, budget| CompiledResume::Exit(bit_array_bool_0_entry((values.bools[0], values.bit_arrays[0],), values, budget)),
                    bit_array_bool_0_resume_1,
                    bit_array_bool_0_resume_2,
                    bit_array_bool_0_resume_3,
                    bit_array_bool_0_resume_4,
                    bit_array_bool_0_resume_5,
                    bit_array_bool_0_resume_6,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn bit_array_bool_0_entry(
                inputs: (bool, data::compiled::bit_array::BitArrayRange,),
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (mut b0_v0, mut b0_b0,) = inputs;
                'repeat: loop {
                    let (b1_v0, b1_b0,) = 'block_1: {
                        let (b3_v0,) = 'block_3: {
                            let () = 'block_4: {
                                let (b2_v0, b2_b0,) = 'block_2: {
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[b0_v0]);
                                        values.bit_arrays.clear();
                                        values.bit_arrays.extend_from_slice(&[b0_b0]);
                                        return data::compiled::CompiledProgress::Yield(0);
                                    }
                                    *budget -= 1;
                                    let _matched = (|| -> Option<(data::compiled::bit_array::BitArrayRange,)> {
                                        let mut _offset = 0_usize;
                                        let _field_0 = values.integer(
                                            b0_b0,
                                            _offset,
                                            8_usize,
                                            data::graph::Endianness::Big,
                                            data::graph::Signedness::Unsigned,
                                        )?;
                                        if _field_0 != 1_i128 { return None; }
                                        _offset = _offset.checked_add(8_usize)?;
                                        let _length_1 = b0_b0.bit_len().checked_sub(_offset)?;
                                        let _range_1 = b0_b0.slice(_offset, _length_1)?;
                                        let _matched_0 = _range_1;
                                        _offset = _offset.checked_add(_length_1)?;
                                        if _offset != b0_b0.bit_len() { return None; }
                                        Some((_matched_0,))
                                    })();
                                    match _matched {
                                        Some((_matched_0,)) => {
                                            break 'block_1 (b0_v0, _matched_0,);
                                        },
                                        None => {
                                            break 'block_2 (b0_v0, b0_b0,);
                                        },
                                    }
                                };
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[b2_v0]);
                                    values.bit_arrays.clear();
                                    values.bit_arrays.extend_from_slice(&[b2_b0]);
                                    return data::compiled::CompiledProgress::Yield(3);
                                }
                                *budget -= 1;
                                let _matched = (|| -> Option<()> {
                                    let mut _offset = 0_usize;
                                    if _offset != b2_b0.bit_len() { return None; }
                                    Some(())
                                })();
                                match _matched {
                                    Some(()) => {
                                        break 'block_3 (b2_v0,);
                                    },
                                    None => {
                                        break 'block_4;
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
                                return data::compiled::CompiledProgress::Yield(5);
                            }
                            *budget -= 1;
                            let b4_v0 = false;
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[b4_v0]);
                                values.bit_arrays.clear();
                                values.bit_arrays.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(6);
                            }
                            *budget -= 1;

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b4_v0]);
                            values.bit_arrays.clear();
                            values.bit_arrays.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1));
                        };
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b3_v0]);
                            values.bit_arrays.clear();
                            values.bit_arrays.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(4);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b3_v0]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0));
                    };
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b1_b0]);
                        return data::compiled::CompiledProgress::Yield(1);
                    }
                    *budget -= 1;
                    let b1_v1 = !b1_v0;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0, b1_v1]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b1_b0]);
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;
                    {
                        (b0_v0, b0_b0,) = (b1_v1, b1_b0,);
                        continue 'repeat;
                    }
                }
            }

            fn bit_array_bool_0_resume_1(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_v0, b1_b0,) = (values.bools[0], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b1_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                let b1_v1 = !b1_v0;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b1_v0, b1_v1]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[b1_b0]);
                CompiledResume::Next(2)
            }

            fn bit_array_bool_0_resume_2(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_v0, b1_v1, b1_b0,) = (values.bools[0], values.bools[1], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0, b1_v1]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b1_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                {
                    let (b0_v0, b0_b0,) = (b1_v1, b1_b0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b0_b0]);
                    CompiledResume::Next(0)
                }
            }

            fn bit_array_bool_0_resume_3(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_v0, b2_b0,) = (values.bools[0], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b2_v0]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b2_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                let _matched = (|| -> Option<()> {
                    let mut _offset = 0_usize;
                    if _offset != b2_b0.bit_len() { return None; }
                    Some(())
                })();
                match _matched {
                    Some(()) => {
                        let (b3_v0,) = (b2_v0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b3_v0]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        CompiledResume::Next(4)
                    },
                    None => {
                        let () = ();

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        CompiledResume::Next(5)
                    },
                }
            }

            fn bit_array_bool_0_resume_4(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_v0,) = (values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b3_v0]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b3_v0]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn bit_array_bool_0_resume_5(
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
                let b4_v0 = false;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b4_v0]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Next(6)
            }

            fn bit_array_bool_0_resume_6(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_v0,) = (values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b4_v0]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b4_v0]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn bit_array_custom_0(
                point: usize,
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::bit_array::BitArrayValues, &mut usize) -> CompiledResume;
                    23
                ] = [
                    |values, budget| CompiledResume::Exit(bit_array_custom_0_entry((values.ints[0], values.ints[1], values.bit_arrays[0],), values, budget)),
                    bit_array_custom_0_resume_1,
                    bit_array_custom_0_resume_2,
                    bit_array_custom_0_resume_3,
                    bit_array_custom_0_resume_4,
                    bit_array_custom_0_resume_5,
                    bit_array_custom_0_resume_6,
                    bit_array_custom_0_resume_7,
                    bit_array_custom_0_resume_8,
                    bit_array_custom_0_resume_9,
                    bit_array_custom_0_resume_10,
                    bit_array_custom_0_resume_11,
                    bit_array_custom_0_resume_12,
                    bit_array_custom_0_resume_13,
                    bit_array_custom_0_resume_14,
                    bit_array_custom_0_resume_15,
                    bit_array_custom_0_resume_16,
                    bit_array_custom_0_resume_17,
                    bit_array_custom_0_resume_18,
                    bit_array_custom_0_resume_19,
                    bit_array_custom_0_resume_20,
                    bit_array_custom_0_resume_21,
                    bit_array_custom_0_resume_22,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn bit_array_custom_0_entry(
                inputs: (i128, i128, data::compiled::bit_array::BitArrayRange,),
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (mut b0_i0, mut b0_i1, mut b0_b0,) = inputs;
                'repeat: loop {
                    let (b3_i0, b3_i1, b3_i2, b3_b0,) = 'block_3: {
                        let (b9_i0, b9_i1, b9_b0,) = 'block_9: {
                            let (b8_i0, b8_i1, b8_b0,) = 'block_8: {
                                let (b11_i0, b11_i1, b11_b0,) = 'block_11: {
                                    let (b14_i0, b14_i1,) = 'block_14: {
                                        let () = 'block_15: {
                                            let (b13_i0, b13_i1, b13_b0,) = 'block_13: {
                                                let (b12_i0, b12_i1, b12_b0,) = 'block_12: {
                                                    let (b10_i0, b10_i1, b10_i2, b10_b0, b10_b1,) = 'block_10: {
                                                        let (b7_i0, b7_i1, b7_i2, b7_b0, b7_b1,) = 'block_7: {
                                                            let (b16_i0, b16_i1, b16_b0,) = 'block_16: {
                                                                let (b6_i0, b6_i1, b6_b0,) = 'block_6: {
                                                                    let (b5_i0, b5_i1, b5_b0,) = 'block_5: {
                                                                        let (b4_i0, b4_i1, b4_b0,) = 'block_4: {
                                                                            let (b2_i0, b2_i1, b2_i2, b2_b0, b2_b1,) = 'block_2: {
                                                                                let (b17_i0, b17_i1, b17_b0,) = 'block_17: {
                                                                                    let (b1_i0, b1_i1, b1_i2, b1_b0, b1_b1,) = 'block_1: {
                                                                                        let (b18_i0, b18_i1, b18_b0,) = 'block_18: {
                                                                                            if *budget == 0 {

                                                                                                values.ints.clear();
                                                                                                values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                                                                                                values.bools.clear();
                                                                                                values.bools.extend_from_slice(&[]);
                                                                                                values.bit_arrays.clear();
                                                                                                values.bit_arrays.extend_from_slice(&[b0_b0]);
                                                                                                return data::compiled::CompiledProgress::Yield(0);
                                                                                            }
                                                                                            *budget -= 1;
                                                                                            let _matched = (|| -> Option<(i128, data::compiled::bit_array::BitArrayRange,)> {
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
                                                                                                let _matched_1 = _range_1;
                                                                                                _offset = _offset.checked_add(_length_1)?;
                                                                                                if _offset != b0_b0.bit_len() { return None; }
                                                                                                Some((_matched_0, _matched_1,))
                                                                                            })();
                                                                                            match _matched {
                                                                                                Some((_matched_0, _matched_1,)) => {
                                                                                                    break 'block_1 (b0_i0, b0_i1, _matched_0, b0_b0, _matched_1,);
                                                                                                },
                                                                                                None => {
                                                                                                    break 'block_18 (b0_i0, b0_i1, b0_b0,);
                                                                                                },
                                                                                            }
                                                                                        };
                                                                                        if *budget == 0 {

                                                                                            values.ints.clear();
                                                                                            values.ints.extend_from_slice(&[b18_i0, b18_i1]);
                                                                                            values.bools.clear();
                                                                                            values.bools.extend_from_slice(&[]);
                                                                                            values.bit_arrays.clear();
                                                                                            values.bit_arrays.extend_from_slice(&[b18_b0]);
                                                                                            return data::compiled::CompiledProgress::Yield(22);
                                                                                        }
                                                                                        *budget -= 1;
                                                                                        {
                                                                                            break 'block_6 (b18_i0, b18_i1, b18_b0,);
                                                                                        }
                                                                                    };
                                                                                    if *budget == 0 {

                                                                                        values.ints.clear();
                                                                                        values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                                                                                        values.bools.clear();
                                                                                        values.bools.extend_from_slice(&[]);
                                                                                        values.bit_arrays.clear();
                                                                                        values.bit_arrays.extend_from_slice(&[b1_b0, b1_b1]);
                                                                                        return data::compiled::CompiledProgress::Yield(1);
                                                                                    }
                                                                                    *budget -= 1;
                                                                                    if b1_i2 >= 48_i128 {
                                                                                        break 'block_2 (b1_i0, b1_i1, b1_i2, b1_b0, b1_b1,);
                                                                                    } else {
                                                                                        break 'block_17 (b1_i0, b1_i1, b1_b0,);
                                                                                    }
                                                                                };
                                                                                if *budget == 0 {

                                                                                    values.ints.clear();
                                                                                    values.ints.extend_from_slice(&[b17_i0, b17_i1]);
                                                                                    values.bools.clear();
                                                                                    values.bools.extend_from_slice(&[]);
                                                                                    values.bit_arrays.clear();
                                                                                    values.bit_arrays.extend_from_slice(&[b17_b0]);
                                                                                    return data::compiled::CompiledProgress::Yield(21);
                                                                                }
                                                                                *budget -= 1;
                                                                                {
                                                                                    break 'block_5 (b17_i0, b17_i1, b17_b0,);
                                                                                }
                                                                            };
                                                                            if *budget == 0 {

                                                                                values.ints.clear();
                                                                                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                                                                                values.bools.clear();
                                                                                values.bools.extend_from_slice(&[]);
                                                                                values.bit_arrays.clear();
                                                                                values.bit_arrays.extend_from_slice(&[b2_b0, b2_b1]);
                                                                                return data::compiled::CompiledProgress::Yield(2);
                                                                            }
                                                                            *budget -= 1;
                                                                            if b2_i2 <= 57_i128 {
                                                                                break 'block_3 (b2_i0, b2_i1, b2_i2, b2_b1,);
                                                                            } else {
                                                                                break 'block_4 (b2_i0, b2_i1, b2_b0,);
                                                                            }
                                                                        };
                                                                        if *budget == 0 {

                                                                            values.ints.clear();
                                                                            values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                                                                            values.bools.clear();
                                                                            values.bools.extend_from_slice(&[]);
                                                                            values.bit_arrays.clear();
                                                                            values.bit_arrays.extend_from_slice(&[b4_b0]);
                                                                            return data::compiled::CompiledProgress::Yield(5);
                                                                        }
                                                                        *budget -= 1;
                                                                        {
                                                                            break 'block_5 (b4_i0, b4_i1, b4_b0,);
                                                                        }
                                                                    };
                                                                    if *budget == 0 {

                                                                        values.ints.clear();
                                                                        values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                                                                        values.bools.clear();
                                                                        values.bools.extend_from_slice(&[]);
                                                                        values.bit_arrays.clear();
                                                                        values.bit_arrays.extend_from_slice(&[b5_b0]);
                                                                        return data::compiled::CompiledProgress::Yield(6);
                                                                    }
                                                                    *budget -= 1;
                                                                    {
                                                                        break 'block_6 (b5_i0, b5_i1, b5_b0,);
                                                                    }
                                                                };
                                                                if *budget == 0 {

                                                                    values.ints.clear();
                                                                    values.ints.extend_from_slice(&[b6_i0, b6_i1]);
                                                                    values.bools.clear();
                                                                    values.bools.extend_from_slice(&[]);
                                                                    values.bit_arrays.clear();
                                                                    values.bit_arrays.extend_from_slice(&[b6_b0]);
                                                                    return data::compiled::CompiledProgress::Yield(7);
                                                                }
                                                                *budget -= 1;
                                                                let _matched = (|| -> Option<(i128, data::compiled::bit_array::BitArrayRange,)> {
                                                                    let mut _offset = 0_usize;
                                                                    let _field_0 = values.integer(
                                                                        b6_b0,
                                                                        _offset,
                                                                        8_usize,
                                                                        data::graph::Endianness::Big,
                                                                        data::graph::Signedness::Unsigned,
                                                                    )?;
                                                                    let _matched_0 = _field_0;
                                                                    _offset = _offset.checked_add(8_usize)?;
                                                                    let _length_1 = b6_b0.bit_len().checked_sub(_offset)?;
                                                                    let _range_1 = b6_b0.slice(_offset, _length_1)?;
                                                                    let _matched_1 = _range_1;
                                                                    _offset = _offset.checked_add(_length_1)?;
                                                                    if _offset != b6_b0.bit_len() { return None; }
                                                                    Some((_matched_0, _matched_1,))
                                                                })();
                                                                match _matched {
                                                                    Some((_matched_0, _matched_1,)) => {
                                                                        break 'block_7 (b6_i0, b6_i1, _matched_0, b6_b0, _matched_1,);
                                                                    },
                                                                    None => {
                                                                        break 'block_16 (b6_i0, b6_i1, b6_b0,);
                                                                    },
                                                                }
                                                            };
                                                            if *budget == 0 {

                                                                values.ints.clear();
                                                                values.ints.extend_from_slice(&[b16_i0, b16_i1]);
                                                                values.bools.clear();
                                                                values.bools.extend_from_slice(&[]);
                                                                values.bit_arrays.clear();
                                                                values.bit_arrays.extend_from_slice(&[b16_b0]);
                                                                return data::compiled::CompiledProgress::Yield(20);
                                                            }
                                                            *budget -= 1;
                                                            {
                                                                break 'block_13 (b16_i0, b16_i1, b16_b0,);
                                                            }
                                                        };
                                                        if *budget == 0 {

                                                            values.ints.clear();
                                                            values.ints.extend_from_slice(&[b7_i0, b7_i1, b7_i2]);
                                                            values.bools.clear();
                                                            values.bools.extend_from_slice(&[]);
                                                            values.bit_arrays.clear();
                                                            values.bit_arrays.extend_from_slice(&[b7_b0, b7_b1]);
                                                            return data::compiled::CompiledProgress::Yield(8);
                                                        }
                                                        *budget -= 1;
                                                        if b7_i2 == 44_i128 {
                                                            break 'block_8 (b7_i0, b7_i1, b7_b1,);
                                                        } else {
                                                            break 'block_10 (b7_i0, b7_i1, b7_i2, b7_b0, b7_b1,);
                                                        }
                                                    };
                                                    if *budget == 0 {

                                                        values.ints.clear();
                                                        values.ints.extend_from_slice(&[b10_i0, b10_i1, b10_i2]);
                                                        values.bools.clear();
                                                        values.bools.extend_from_slice(&[]);
                                                        values.bit_arrays.clear();
                                                        values.bit_arrays.extend_from_slice(&[b10_b0, b10_b1]);
                                                        return data::compiled::CompiledProgress::Yield(13);
                                                    }
                                                    *budget -= 1;
                                                    if b10_i2 == 10_i128 {
                                                        break 'block_11 (b10_i0, b10_i1, b10_b1,);
                                                    } else {
                                                        break 'block_12 (b10_i0, b10_i1, b10_b0,);
                                                    }
                                                };
                                                if *budget == 0 {

                                                    values.ints.clear();
                                                    values.ints.extend_from_slice(&[b12_i0, b12_i1]);
                                                    values.bools.clear();
                                                    values.bools.extend_from_slice(&[]);
                                                    values.bit_arrays.clear();
                                                    values.bit_arrays.extend_from_slice(&[b12_b0]);
                                                    return data::compiled::CompiledProgress::Yield(15);
                                                }
                                                *budget -= 1;
                                                {
                                                    break 'block_13 (b12_i0, b12_i1, b12_b0,);
                                                }
                                            };
                                            if *budget == 0 {

                                                values.ints.clear();
                                                values.ints.extend_from_slice(&[b13_i0, b13_i1]);
                                                values.bools.clear();
                                                values.bools.extend_from_slice(&[]);
                                                values.bit_arrays.clear();
                                                values.bit_arrays.extend_from_slice(&[b13_b0]);
                                                return data::compiled::CompiledProgress::Yield(16);
                                            }
                                            *budget -= 1;
                                            let _matched = (|| -> Option<()> {
                                                let mut _offset = 0_usize;
                                                if _offset != b13_b0.bit_len() { return None; }
                                                Some(())
                                            })();
                                            match _matched {
                                                Some(()) => {
                                                    break 'block_14 (b13_i0, b13_i1,);
                                                },
                                                None => {
                                                    break 'block_15;
                                                },
                                            }
                                        };

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.bit_arrays.clear();
                                        values.bit_arrays.extend_from_slice(&[]);
                                        return data::compiled::CompiledProgress::Interpreted(19);
                                    };
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b14_i0, b14_i1]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.bit_arrays.clear();
                                        values.bit_arrays.extend_from_slice(&[]);
                                        return data::compiled::CompiledProgress::Yield(17);
                                    }
                                    *budget -= 1;
                                    let b14_i2 = b14_i1 + b14_i0;
                                    if b14_i2 < i128::from(i64::MIN) || b14_i2 > i128::from(i64::MAX) {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b14_i0, b14_i1, b14_i2]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.bit_arrays.clear();
                                        values.bit_arrays.extend_from_slice(&[]);
                                        return data::compiled::CompiledProgress::Interpreted(18);
                                    }

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b14_i0, b14_i1, b14_i2]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.bit_arrays.clear();
                                    values.bit_arrays.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Interpreted(18);
                                };
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b11_i0, b11_i1]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.bit_arrays.clear();
                                    values.bit_arrays.extend_from_slice(&[b11_b0]);
                                    return data::compiled::CompiledProgress::Yield(14);
                                }
                                *budget -= 1;
                                {
                                    break 'block_9 (b11_i0, b11_i1, b11_b0,);
                                }
                            };
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b8_i0, b8_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.bit_arrays.clear();
                                values.bit_arrays.extend_from_slice(&[b8_b0]);
                                return data::compiled::CompiledProgress::Yield(9);
                            }
                            *budget -= 1;
                            {
                                break 'block_9 (b8_i0, b8_i1, b8_b0,);
                            }
                        };
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b9_i0, b9_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.bit_arrays.clear();
                            values.bit_arrays.extend_from_slice(&[b9_b0]);
                            return data::compiled::CompiledProgress::Yield(10);
                        }
                        *budget -= 1;
                        let b9_i2 = 0_i128;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b9_i0, b9_i1, b9_i2]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.bit_arrays.clear();
                            values.bit_arrays.extend_from_slice(&[b9_b0]);
                            return data::compiled::CompiledProgress::Yield(11);
                        }
                        *budget -= 1;
                        let b9_i3 = b9_i1 + b9_i0;
                        if b9_i3 < i128::from(i64::MIN) || b9_i3 > i128::from(i64::MAX) {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b9_i0, b9_i1, b9_i2, b9_i3]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.bit_arrays.clear();
                            values.bit_arrays.extend_from_slice(&[b9_b0]);
                            return data::compiled::CompiledProgress::Interpreted(12);
                        }
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b9_i0, b9_i1, b9_i2, b9_i3]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.bit_arrays.clear();
                            values.bit_arrays.extend_from_slice(&[b9_b0]);
                            return data::compiled::CompiledProgress::Yield(12);
                        }
                        *budget -= 1;
                        {
                            (b0_i0, b0_i1, b0_b0,) = (b9_i2, b9_i3, b9_b0,);
                            continue 'repeat;
                        }
                    };
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b3_b0]);
                        return data::compiled::CompiledProgress::Yield(3);
                    }
                    *budget -= 1;
                    let _r0_n0 = b3_i0 * 10_i128;
                    let _r0_n1 = _r0_n0 + b3_i2;
                    let _r0_n2 = _r0_n1 - 48_i128;
                    let b3_i3 = _r0_n2;
                    if b3_i3 < i128::from(i64::MIN) || b3_i3 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b3_b0]);
                        return data::compiled::CompiledProgress::Interpreted(4);
                    }
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b3_b0]);
                        return data::compiled::CompiledProgress::Yield(4);
                    }
                    *budget -= 1;
                    {
                        (b0_i0, b0_i1, b0_b0,) = (b3_i3, b3_i1, b3_b0,);
                        continue 'repeat;
                    }
                }
            }

            fn bit_array_custom_0_resume_1(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_i0, b1_i1, b1_i2, b1_b0, b1_b1,) = (values.ints[0], values.ints[1], values.ints[2], values.bit_arrays[0], values.bit_arrays[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b1_b0, b1_b1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                if b1_i2 >= 48_i128 {
                    let (b2_i0, b2_i1, b2_i2, b2_b0, b2_b1,) = (b1_i0, b1_i1, b1_i2, b1_b0, b1_b1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b2_b0, b2_b1]);
                    CompiledResume::Next(2)
                } else {
                    let (b17_i0, b17_i1, b17_b0,) = (b1_i0, b1_i1, b1_b0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b17_i0, b17_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b17_b0]);
                    CompiledResume::Next(21)
                }
            }

            fn bit_array_custom_0_resume_2(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_i0, b2_i1, b2_i2, b2_b0, b2_b1,) = (values.ints[0], values.ints[1], values.ints[2], values.bit_arrays[0], values.bit_arrays[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b2_b0, b2_b1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                if b2_i2 <= 57_i128 {
                    let (b3_i0, b3_i1, b3_i2, b3_b0,) = (b2_i0, b2_i1, b2_i2, b2_b1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b3_b0]);
                    CompiledResume::Next(3)
                } else {
                    let (b4_i0, b4_i1, b4_b0,) = (b2_i0, b2_i1, b2_b0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b4_b0]);
                    CompiledResume::Next(5)
                }
            }

            fn bit_array_custom_0_resume_3(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0, b3_i1, b3_i2, b3_b0,) = (values.ints[0], values.ints[1], values.ints[2], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b3_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                let _r0_n0 = b3_i0 * 10_i128;
                let _r0_n1 = _r0_n0 + b3_i2;
                let _r0_n2 = _r0_n1 - 48_i128;
                let b3_i3 = _r0_n2;
                if b3_i3 < i128::from(i64::MIN) || b3_i3 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b3_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(4));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[b3_b0]);
                CompiledResume::Next(4)
            }

            fn bit_array_custom_0_resume_4(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0, b3_i1, b3_i2, b3_i3, b3_b0,) = (values.ints[0], values.ints[1], values.ints[2], values.ints[3], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b3_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;
                {
                    let (b0_i0, b0_i1, b0_b0,) = (b3_i3, b3_i1, b3_b0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b0_b0]);
                    CompiledResume::Next(0)
                }
            }

            fn bit_array_custom_0_resume_5(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b4_i0, b4_i1, b4_b0,) = (values.ints[0], values.ints[1], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b4_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;
                {
                    let (b5_i0, b5_i1, b5_b0,) = (b4_i0, b4_i1, b4_b0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b5_b0]);
                    CompiledResume::Next(6)
                }
            }

            fn bit_array_custom_0_resume_6(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b5_i0, b5_i1, b5_b0,) = (values.ints[0], values.ints[1], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b5_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;
                {
                    let (b6_i0, b6_i1, b6_b0,) = (b5_i0, b5_i1, b5_b0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b6_i0, b6_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b6_b0]);
                    CompiledResume::Next(7)
                }
            }

            fn bit_array_custom_0_resume_7(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b6_i0, b6_i1, b6_b0,) = (values.ints[0], values.ints[1], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b6_i0, b6_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b6_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                }
                *budget -= 1;
                let _matched = (|| -> Option<(i128, data::compiled::bit_array::BitArrayRange,)> {
                    let mut _offset = 0_usize;
                    let _field_0 = values.integer(
                        b6_b0,
                        _offset,
                        8_usize,
                        data::graph::Endianness::Big,
                        data::graph::Signedness::Unsigned,
                    )?;
                    let _matched_0 = _field_0;
                    _offset = _offset.checked_add(8_usize)?;
                    let _length_1 = b6_b0.bit_len().checked_sub(_offset)?;
                    let _range_1 = b6_b0.slice(_offset, _length_1)?;
                    let _matched_1 = _range_1;
                    _offset = _offset.checked_add(_length_1)?;
                    if _offset != b6_b0.bit_len() { return None; }
                    Some((_matched_0, _matched_1,))
                })();
                match _matched {
                    Some((_matched_0, _matched_1,)) => {
                        let (b7_i0, b7_i1, b7_i2, b7_b0, b7_b1,) = (b6_i0, b6_i1, _matched_0, b6_b0, _matched_1,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b7_i0, b7_i1, b7_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b7_b0, b7_b1]);
                        CompiledResume::Next(8)
                    },
                    None => {
                        let (b16_i0, b16_i1, b16_b0,) = (b6_i0, b6_i1, b6_b0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b16_i0, b16_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[b16_b0]);
                        CompiledResume::Next(20)
                    },
                }
            }

            fn bit_array_custom_0_resume_8(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b7_i0, b7_i1, b7_i2, b7_b0, b7_b1,) = (values.ints[0], values.ints[1], values.ints[2], values.bit_arrays[0], values.bit_arrays[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b7_i0, b7_i1, b7_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b7_b0, b7_b1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                }
                *budget -= 1;
                if b7_i2 == 44_i128 {
                    let (b8_i0, b8_i1, b8_b0,) = (b7_i0, b7_i1, b7_b1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b8_i0, b8_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b8_b0]);
                    CompiledResume::Next(9)
                } else {
                    let (b10_i0, b10_i1, b10_i2, b10_b0, b10_b1,) = (b7_i0, b7_i1, b7_i2, b7_b0, b7_b1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b10_i0, b10_i1, b10_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b10_b0, b10_b1]);
                    CompiledResume::Next(13)
                }
            }

            fn bit_array_custom_0_resume_9(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b8_i0, b8_i1, b8_b0,) = (values.ints[0], values.ints[1], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b8_i0, b8_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b8_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(9));
                }
                *budget -= 1;
                {
                    let (b9_i0, b9_i1, b9_b0,) = (b8_i0, b8_i1, b8_b0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b9_i0, b9_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b9_b0]);
                    CompiledResume::Next(10)
                }
            }

            fn bit_array_custom_0_resume_10(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b9_i0, b9_i1, b9_b0,) = (values.ints[0], values.ints[1], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b9_i0, b9_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b9_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(10));
                }
                *budget -= 1;
                let b9_i2 = 0_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b9_i0, b9_i1, b9_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[b9_b0]);
                CompiledResume::Next(11)
            }

            fn bit_array_custom_0_resume_11(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b9_i0, b9_i1, b9_i2, b9_b0,) = (values.ints[0], values.ints[1], values.ints[2], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b9_i0, b9_i1, b9_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b9_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(11));
                }
                *budget -= 1;
                let b9_i3 = b9_i1 + b9_i0;
                if b9_i3 < i128::from(i64::MIN) || b9_i3 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b9_i0, b9_i1, b9_i2, b9_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b9_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(12));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b9_i0, b9_i1, b9_i2, b9_i3]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[b9_b0]);
                CompiledResume::Next(12)
            }

            fn bit_array_custom_0_resume_12(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b9_i0, b9_i1, b9_i2, b9_i3, b9_b0,) = (values.ints[0], values.ints[1], values.ints[2], values.ints[3], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b9_i0, b9_i1, b9_i2, b9_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b9_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(12));
                }
                *budget -= 1;
                {
                    let (b0_i0, b0_i1, b0_b0,) = (b9_i2, b9_i3, b9_b0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b0_b0]);
                    CompiledResume::Next(0)
                }
            }

            fn bit_array_custom_0_resume_13(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b10_i0, b10_i1, b10_i2, b10_b0, b10_b1,) = (values.ints[0], values.ints[1], values.ints[2], values.bit_arrays[0], values.bit_arrays[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b10_i0, b10_i1, b10_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b10_b0, b10_b1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(13));
                }
                *budget -= 1;
                if b10_i2 == 10_i128 {
                    let (b11_i0, b11_i1, b11_b0,) = (b10_i0, b10_i1, b10_b1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b11_i0, b11_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b11_b0]);
                    CompiledResume::Next(14)
                } else {
                    let (b12_i0, b12_i1, b12_b0,) = (b10_i0, b10_i1, b10_b0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b12_i0, b12_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b12_b0]);
                    CompiledResume::Next(15)
                }
            }

            fn bit_array_custom_0_resume_14(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b11_i0, b11_i1, b11_b0,) = (values.ints[0], values.ints[1], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b11_i0, b11_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b11_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(14));
                }
                *budget -= 1;
                {
                    let (b9_i0, b9_i1, b9_b0,) = (b11_i0, b11_i1, b11_b0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b9_i0, b9_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b9_b0]);
                    CompiledResume::Next(10)
                }
            }

            fn bit_array_custom_0_resume_15(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b12_i0, b12_i1, b12_b0,) = (values.ints[0], values.ints[1], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b12_i0, b12_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b12_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(15));
                }
                *budget -= 1;
                {
                    let (b13_i0, b13_i1, b13_b0,) = (b12_i0, b12_i1, b12_b0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b13_i0, b13_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b13_b0]);
                    CompiledResume::Next(16)
                }
            }

            fn bit_array_custom_0_resume_16(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b13_i0, b13_i1, b13_b0,) = (values.ints[0], values.ints[1], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b13_i0, b13_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b13_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(16));
                }
                *budget -= 1;
                let _matched = (|| -> Option<()> {
                    let mut _offset = 0_usize;
                    if _offset != b13_b0.bit_len() { return None; }
                    Some(())
                })();
                match _matched {
                    Some(()) => {
                        let (b14_i0, b14_i1,) = (b13_i0, b13_i1,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b14_i0, b14_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        CompiledResume::Next(17)
                    },
                    None => {
                        let () = ();

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        CompiledResume::Next(19)
                    },
                }
            }

            fn bit_array_custom_0_resume_17(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b14_i0, b14_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b14_i0, b14_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(17));
                }
                *budget -= 1;
                let b14_i2 = b14_i1 + b14_i0;
                if b14_i2 < i128::from(i64::MIN) || b14_i2 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b14_i0, b14_i1, b14_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(18));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b14_i0, b14_i1, b14_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Next(18)
            }

            fn bit_array_custom_0_resume_18(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b14_i0, b14_i1, b14_i2,) = (values.ints[0], values.ints[1], values.ints[2],);
                let _ = budget;

                values.ints.clear();
                values.ints.extend_from_slice(&[b14_i0, b14_i1, b14_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(18))
            }

            fn bit_array_custom_0_resume_19(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                let _ = budget;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(19))
            }

            fn bit_array_custom_0_resume_20(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b16_i0, b16_i1, b16_b0,) = (values.ints[0], values.ints[1], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b16_i0, b16_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b16_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(20));
                }
                *budget -= 1;
                {
                    let (b13_i0, b13_i1, b13_b0,) = (b16_i0, b16_i1, b16_b0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b13_i0, b13_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b13_b0]);
                    CompiledResume::Next(16)
                }
            }

            fn bit_array_custom_0_resume_21(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b17_i0, b17_i1, b17_b0,) = (values.ints[0], values.ints[1], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b17_i0, b17_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b17_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(21));
                }
                *budget -= 1;
                {
                    let (b5_i0, b5_i1, b5_b0,) = (b17_i0, b17_i1, b17_b0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b5_b0]);
                    CompiledResume::Next(6)
                }
            }

            fn bit_array_custom_0_resume_22(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b18_i0, b18_i1, b18_b0,) = (values.ints[0], values.ints[1], values.bit_arrays[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b18_i0, b18_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b18_b0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(22));
                }
                *budget -= 1;
                {
                    let (b6_i0, b6_i1, b6_b0,) = (b18_i0, b18_i1, b18_b0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b6_i0, b6_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.bit_arrays.clear();
                    values.bit_arrays.extend_from_slice(&[b6_b0]);
                    CompiledResume::Next(7)
                }
            }
            data::compiled::CompiledFunctions {
                ints: data::Storage::Static(&[
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(0),
                        implementation: data::compiled::CompiledImplementation::BitArray(data::compiled::BitArrayImplementation {
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 1,
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
                                    ints: 5,
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
                                    instruction: 1,
                                    ints: 6,
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
                                    block: data::graph::BlockId(2),
                                    instruction: 0,
                                    ints: 1,
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
                            ]),
                            run: bit_array_int_0,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(1),
                        implementation: data::compiled::CompiledImplementation::BitArray(data::compiled::BitArrayImplementation {
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 1,
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
                                    ints: 2,
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
                                    instruction: 1,
                                    ints: 3,
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
                                    block: data::graph::BlockId(2),
                                    instruction: 0,
                                    ints: 1,
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
                            run: bit_array_int_1,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(2),
                        implementation: data::compiled::CompiledImplementation::BitArray(data::compiled::BitArrayImplementation {
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 1,
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
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 3,
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
                                    bit_arrays: 2,
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
                                    ints: 3,
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
                                    block: data::graph::BlockId(3),
                                    instruction: 1,
                                    ints: 4,
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
                                    ints: 1,
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
                                    block: data::graph::BlockId(7),
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
                                    block: data::graph::BlockId(8),
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
                            run: bit_array_int_2,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(3),
                        implementation: data::compiled::CompiledImplementation::BitArray(data::compiled::BitArrayImplementation {
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 1,
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
                                    ints: 2,
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
                                    instruction: 1,
                                    ints: 3,
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
                                    block: data::graph::BlockId(2),
                                    instruction: 0,
                                    ints: 1,
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
                            run: bit_array_int_3,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(4),
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
                            ]),
                            run: bit_array_int_4,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(5),
                        implementation: data::compiled::CompiledImplementation::BitArray(data::compiled::BitArrayImplementation {
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 2,
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
                                    bit_arrays: 2,
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
                                    bit_arrays: 2,
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
                                    bit_arrays: 2,
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
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 2,
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
                                    ints: 1,
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
                                    block: data::graph::BlockId(8),
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
                            run: bit_array_int_5,
                        }),
                    },
                ]),
                bools: data::Storage::Static(&[
                    data::compiled::CompiledFunction {
                        function: data::function::BoolFunctionId(0),
                        implementation: data::compiled::CompiledImplementation::BitArray(data::compiled::BitArrayImplementation {
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 0,
                                    bools: 1,
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
                                    ints: 0,
                                    bools: 1,
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
                                    instruction: 1,
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
                                    block: data::graph::BlockId(2),
                                    instruction: 0,
                                    ints: 0,
                                    bools: 1,
                                    bit_arrays: 1,
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
                            run: bit_array_bool_0,
                        }),
                    },
                ]),
                customs: data::Storage::Static(&[
                    data::compiled::CompiledFunction {
                        function: 0,
                        implementation: data::compiled::CompiledImplementation::BitArray(data::compiled::BitArrayImplementation {
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 2,
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
                                    bit_arrays: 2,
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
                                    bit_arrays: 2,
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
                                    ints: 3,
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
                                    block: data::graph::BlockId(3),
                                    instruction: 1,
                                    ints: 4,
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
                                    block: data::graph::BlockId(4),
                                    instruction: 0,
                                    ints: 2,
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
                                    block: data::graph::BlockId(5),
                                    instruction: 0,
                                    ints: 2,
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
                                    block: data::graph::BlockId(6),
                                    instruction: 0,
                                    ints: 2,
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
                                    block: data::graph::BlockId(7),
                                    instruction: 0,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 2,
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
                                    ints: 2,
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
                                    block: data::graph::BlockId(9),
                                    instruction: 0,
                                    ints: 2,
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
                                    block: data::graph::BlockId(9),
                                    instruction: 1,
                                    ints: 3,
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
                                    block: data::graph::BlockId(9),
                                    instruction: 2,
                                    ints: 4,
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
                                    block: data::graph::BlockId(10),
                                    instruction: 0,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 2,
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
                                    ints: 2,
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
                                    block: data::graph::BlockId(12),
                                    instruction: 0,
                                    ints: 2,
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
                                    block: data::graph::BlockId(13),
                                    instruction: 0,
                                    ints: 2,
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
                                    block: data::graph::BlockId(14),
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
                                    block: data::graph::BlockId(14),
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
                                    ints: 2,
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
                                    block: data::graph::BlockId(17),
                                    instruction: 0,
                                    ints: 2,
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
                                    block: data::graph::BlockId(18),
                                    instruction: 0,
                                    ints: 2,
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
                            run: bit_array_custom_0,
                        }),
                    },
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
                0..6,
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
                        data::type_::ValueShapeId(2),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 2..4,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(2),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 4..6,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(2),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 6..8,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(2),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 8..9,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(2),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 9..12,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(2),
                        data::type_::ValueShapeId(2),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 12..15,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(2),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(4),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 15..17,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(2),
                        data::type_::ValueShapeId(5),
                    ]),
                    return_: data::type_::ValueShapeId(5),
                    captures: data::Storage::Static(&[]),
                },
            ]),
            parameters: data::Storage::Static(&[
                data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
            ]),
        },
        list_types: data::type_::ListTypeTable {
            types: data::Storage::Static(&[]),
            tuple_items: data::Storage::Static(&[]),
            function_items: data::Storage::Static(&[]),
        },
        custom_types: data::type_::CustomTypeTable {
            types: data::Storage::Static(&[
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
                                type_id: data::type_::CustomTypeId(0),
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
                                type_id: data::type_::CustomTypeId(0),
                                index: 1,
                            },
                            name: data::Text::Static("Error"),
                            native_tag: data::Text::Static("error"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::Nil,
                                    shape: data::type_::ValueShapeId(1),
                                    refinement: data::type_::FieldRefinement::Argument(1),
                                },
                            ]),
                        },
                    ]),
                },
            ]),
            definitions: data::Storage::Static(&[]),
        },
        external_types: data::type_::ExternalTypeTable {
            types: data::Storage::Static(&[]),
        },
        value_shapes: data::type_::ValueShapeTable {
            shapes: data::Storage::Static(&[
                data::type_::ValueShapeDescriptor::Int,
                data::type_::ValueShapeDescriptor::Nil,
                data::type_::ValueShapeDescriptor::BitArray,
                data::type_::ValueShapeDescriptor::String,
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(0)),
                data::type_::ValueShapeDescriptor::Bool,
            ]),
            shape_types: data::Storage::Static(&[
                data::type_::ValueType::Int,
                data::type_::ValueType::Nil,
                data::type_::ValueType::BitArray,
                data::type_::ValueType::String,
                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                data::type_::ValueType::Bool,
            ]),
            custom_shapes: data::Storage::Static(&[
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(0),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(1),
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
        ]),
        floats: data::Storage::Static(&[]),
        strings: data::Storage::Static(&[]),
        bit_arrays: data::Storage::Static(&[]),
        utf_codepoints: data::Storage::Static(&[]),
        customs: data::Storage::Static(&[
            data::program::LibraryFunctionEntry {
                function: data::function::CustomFunctionId {
                    index: 0,
                    return_shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(0),
                        shape_id: data::type_::CustomValueShapeId(0),
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
                callables: data::Storage::Static(&[]),
            },
        ]),
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
            name: data::Text::Static("checksum"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::BitArray,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 0,
        },
        data::Export {
            name: data::Text::Static("parse"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::BitArray,
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                    package: data::Text::Static(""),
                    module: data::Text::Static("gleam"),
                    name: data::Text::Static("Result"),
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::Nil,
                    ]),
                })),
            },
            slot: 0,
        },
        data::Export {
            name: data::Text::Static("wide"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::BitArray,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 1,
        },
        data::Export {
            name: data::Text::Static("aliases"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::BitArray,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 2,
        },
        data::Export {
            name: data::Text::Static("little"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::BitArray,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 3,
        },
        data::Export {
            name: data::Text::Static("late_failure"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::BitArray,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 4,
        },
        data::Export {
            name: data::Text::Static("paired"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::BitArray,
                    data::type_::TypeMetadata::BitArray,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 5,
        },
        data::Export {
            name: data::Text::Static("toggle"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::BitArray,
                    data::type_::TypeMetadata::Bool,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            slot: 0,
        },
    ]),
}
