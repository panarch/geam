data::HostedEntryArtifact {
    format: 23,
    program: data::ProgramTables {
        root: data::source::module_id(0),
        modules: data::Storage::Static(&[
            data::program::ExecutionModuleContext {
                module: data::Text::Static("example"),
                source_context: Some(data::source::SourceContext::from_static_block("src/bit_array_entry.gleam", r#"
pub type Outcome {
  Done(Int)
  Bad
}

fn scan(input: BitArray, total: Int) -> Outcome {
  case input {
    <<value:8, rest:bits>> -> scan(rest, total + value)
    <<>> -> Done(total)
    _ -> Bad
  }
}

pub fn main() {
  echo scan(<<1, 2, 3>>, 0)
  echo scan(<<1:7>>, 0)
  Nil
}
"#)),
            },
        ]),
        main: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Nil(data::function::NilFunctionId(0))),
        functions: data::function::FunctionTables {
            value_returns: data::function::ValueFunctionTables {
                never_functions: data::Storage::Static(&[]),
                int_functions: data::Storage::Static(&[]),
                float_functions: data::Storage::Static(&[]),
                string_functions: data::Storage::Static(&[]),
                bit_array_functions: data::Storage::Static(&[]),
                utf_codepoint_functions: data::Storage::Static(&[]),
                custom_functions: data::Storage::Static(&[
                    data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 2,
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
                                            instructions: 1..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 8..8,
                                            instructions: 2..3,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
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
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
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
                                                shape: data::type_::ValueShapeId(2),
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
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[]),
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
                    })),
                ]),
                external_functions: data::Storage::Static(&[]),
                bool_functions: data::Storage::Static(&[]),
                nil_functions: data::Storage::Static(&[
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
                                        instructions: 0..6,
                                        terminator: data::graph::Terminator::Echo(data::graph::Echo {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                            message: None,
                                            site: data::source::EchoSite::from_static("example", "main", data::source::SourceSpan::new(223, 248)),
                                            next: data::graph::Edge {
                                                target: data::graph::BlockId(1),
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
                                        params: 0..0,
                                        instructions: 6..10,
                                        terminator: data::graph::Terminator::Echo(data::graph::Echo {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                            message: None,
                                            site: data::source::EchoSite::from_static("example", "main", data::source::SourceSpan::new(251, 272)),
                                            next: data::graph::Edge {
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
                                        params: 0..0,
                                        instructions: 10..11,
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
                                                1,
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
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Plus,
                                            digits: data::Storage::Static(&[
                                                3,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::BitArray(data::graph::BitArrayInstruction::Value(data::Storage::Static(&[
                                            data::graph::BitArraySegment::Int {
                                                value: data::graph::IntLocalId(0),
                                                bit_size: 8,
                                                endianness: data::graph::Endianness::Big,
                                            },
                                            data::graph::BitArraySegment::Int {
                                                value: data::graph::IntLocalId(1),
                                                bit_size: 8,
                                                endianness: data::graph::Endianness::Big,
                                            },
                                            data::graph::BitArraySegment::Int {
                                                value: data::graph::IntLocalId(2),
                                                bit_size: 8,
                                                endianness: data::graph::Endianness::Big,
                                            },
                                        ]))),
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
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(2),
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
                                                data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(228, 248)),
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
                                                1,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::BitArray(data::graph::BitArrayInstruction::Value(data::Storage::Static(&[
                                            data::graph::BitArraySegment::Int {
                                                value: data::graph::IntLocalId(0),
                                                bit_size: 7,
                                                endianness: data::graph::Endianness::Big,
                                            },
                                        ]))),
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
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(2),
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
                                                data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(256, 272)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Nil(data::graph::NilLocalId(0)),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Nil(data::graph::NilInstruction::Value),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::NilLocalId(0)),
                            ]),
                        },
                    })),
                ]),
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

            fn bit_array_custom_0(
                point: usize,
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::bit_array::BitArrayValues, &mut usize) -> CompiledResume;
                    6
                ] = [
                    |values, budget| CompiledResume::Exit(bit_array_custom_0_entry((values.ints[0], values.bit_arrays[0],), values, budget)),
                    bit_array_custom_0_resume_1,
                    bit_array_custom_0_resume_2,
                    bit_array_custom_0_resume_3,
                    bit_array_custom_0_resume_4,
                    bit_array_custom_0_resume_5,
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

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.bit_arrays.clear();
                            values.bit_arrays.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Interpreted(5);
                        };

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.bit_arrays.clear();
                        values.bit_arrays.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Interpreted(4);
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

            fn bit_array_custom_0_resume_1(
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

            fn bit_array_custom_0_resume_2(
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

            fn bit_array_custom_0_resume_3(
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

            fn bit_array_custom_0_resume_4(
                values: &mut data::compiled::bit_array::BitArrayValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0,) = (values.ints[0],);
                let _ = budget;

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.bit_arrays.clear();
                values.bit_arrays.extend_from_slice(&[]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(4))
            }

            fn bit_array_custom_0_resume_5(
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
            data::compiled::CompiledFunctions {
                ints: data::Storage::Static(&[
                ]),
                bools: data::Storage::Static(&[
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
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..1,
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
                        data::type_::ValueShapeId(1),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 2..2,
                    parameter_shapes: data::Storage::Static(&[]),
                    return_: data::type_::ValueShapeId(3),
                    captures: data::Storage::Static(&[]),
                },
            ]),
            parameters: data::Storage::Static(&[
                data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                        package: data::Text::Static("example"),
                        module: data::Text::Static("example"),
                        name: data::Text::Static("Outcome"),
                        arguments: data::Storage::Static(&[]),
                    },
                    constructor_count: 2,
                    constructors: data::Storage::Static(&[
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(0),
                                index: 0,
                            },
                            name: data::Text::Static("Done"),
                            native_tag: data::Text::Static("done"),
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
                            name: data::Text::Static("Bad"),
                            native_tag: data::Text::Static("bad"),
                            fields: data::Storage::Static(&[]),
                        },
                    ]),
                },
            ]),
            definitions: data::Storage::Static(&[
                data::type_::CustomDefinition {
                    package: data::Text::Static("example"),
                    module: data::Text::Static("example"),
                    name: data::Text::Static("Outcome"),
                    publicity: data::type_::CustomTypePublicity::Public,
                    opaque: false,
                    parameters: 0,
                    constructors: data::Storage::Static(&[
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Done"),
                            fields: data::Storage::Static(&[
                                data::type_::FieldDefinition {
                                    label: None,
                                    type_: data::type_::TypeMetadata::Int,
                                },
                            ]),
                        },
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Bad"),
                            fields: data::Storage::Static(&[]),
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
                data::type_::ValueShapeDescriptor::Int,
                data::type_::ValueShapeDescriptor::BitArray,
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(0)),
                data::type_::ValueShapeDescriptor::Nil,
            ]),
            shape_types: data::Storage::Static(&[
                data::type_::ValueType::Int,
                data::type_::ValueType::BitArray,
                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                data::type_::ValueType::Nil,
            ]),
            custom_shapes: data::Storage::Static(&[
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(0),
                    arguments: data::Storage::Static(&[]),
                    constructor: data::type_::CustomConstructorRefinement::Any,
                },
            ]),
        },
    },
    value_functions: data::Storage::Static(&[]),
    never_functions: data::Storage::Static(&[]),
}
