data::ModuleArtifact {
    format: 29,
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
                                                site: data::source::HostCallSite::from_static("example", "capture_chain", data::source::SourceSpan::new(367, 381)),
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
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "capture_chain", data::source::SourceSpan::new(384, 400)),
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
                            parameter_count: 2,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..2,
                                        instructions: 0..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                                function: data::function::ProfiledFunctionFunctionId::Int(data::function::IntFunctionFunctionId(1)),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "dynamic_target", data::source::SourceSpan::new(629, 643)),
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
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "dynamic_target", data::source::SourceSpan::new(646, 662)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            right: data::graph::IntegerOperand::Immediate(3),
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
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(9),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "nested", data::source::SourceSpan::new(818, 833)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            right: data::graph::IntegerOperand::Immediate(2),
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
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(12),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "canonical", data::source::SourceSpan::new(1696, 1713)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            right: data::graph::IntegerOperand::Immediate(2),
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
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(13),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "big_return", data::source::SourceSpan::new(1814, 1829)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            right: data::graph::IntegerOperand::Immediate(3),
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
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(14),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "failure", data::source::SourceSpan::new(1974, 1988)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            right: data::graph::IntegerOperand::Immediate(3),
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
                            parameter_count: 2,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..2,
                                        instructions: 0..3,
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
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "producer_suffix_int", data::source::SourceSpan::new(2190, 2208)),
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
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "producer_suffix_int", data::source::SourceSpan::new(2211, 2227)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
                                            right: data::graph::IntegerOperand::Immediate(2),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(3)),
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
                                        instructions: 0..3,
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
                                                target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(15)),
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
                                            site: data::source::HostCallSite::from_static("example", "reuse_callback", data::source::SourceSpan::new(2719, 2729)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
                                            right: data::graph::IntegerOperand::Immediate(1),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(3)),
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
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("canonical caller"))),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(16),
                                        site: data::source::HostCallSite::from_static("example", "repeated_roots", data::source::SourceSpan::new(3098, 3158)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
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
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::LtEqInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(0),
                                            },
                                            true_: data::graph::Edge {
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
                                            },
                                            false_: data::graph::Edge {
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
                                        params: 1..1,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..2,
                                        instructions: 1..4,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
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
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
                                        })),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(9),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "non_tail", data::source::SourceSpan::new(750, 769)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
                                            right: data::graph::IntegerOperand::Immediate(1),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(3)),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
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
                            parameter_count: 1,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..3,
                                        instructions: 0..3,
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
                                    data::graph::ParamSlot {
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
                                            site: data::source::HostCallSite::from_static("example", "<anonymous:3>", data::source::SourceSpan::new(1518, 1530)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::FunctionCall {
                                            function: data::graph::IntFunctionLocalId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "<anonymous:3>", data::source::SourceSpan::new(1533, 1543)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(3)),
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
                                        terminator: data::graph::Terminator::Echo(data::graph::Echo {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            message: None,
                                            site: data::source::EchoSite::from_static("example", "echo_value", data::source::SourceSpan::new(1630, 1640)),
                                            next: data::graph::Edge {
                                                target: data::graph::BlockId(1),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Mult {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
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
                                        params: 0..1,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Echo(data::graph::Echo {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            message: None,
                                            site: data::source::EchoSite::from_static("example", "failing", data::source::SourceSpan::new(1896, 1907)),
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
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..1,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::SourceStop(data::graph::SourceStop {
                                            kind: data::graph::SourceStopKind::Panic,
                                            message: Some(data::graph::StringLocalId(0)),
                                            site: data::source::PanicSite::from_static("example", "failing", data::source::SourceSpan::new(1910, 1932)),
                                        }),
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
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("call suffix"))),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[]),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
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
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..5,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 5..9,
                                        instructions: 0..3,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
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
                                                                    source: 4,
                                                                    destination: 1,
                                                                },
                                                                data::graph::TransferStep {
                                                                    source: 5,
                                                                    destination: 2,
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
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(5),
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
                                        local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        shape: data::type_::ValueShapeId(5),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(7),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "repeat_canonical", data::source::SourceSpan::new(2944, 2973)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
                                            right: data::graph::IntegerOperand::Immediate(1),
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
                                            right: data::graph::IntegerOperand::Immediate(7),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
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
                            parameter_count: 1,
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
                                            site: data::source::HostCallSite::from_static("example", "<anonymous:0>", data::source::SourceSpan::new(183, 198)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            right: data::graph::IntegerOperand::Immediate(1),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
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
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                            function: data::function::BoolFunctionId(6),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "mutual", data::source::SourceSpan::new(1085, 1096)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Not(data::graph::BoolLocalId(0)))),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::BoolLocalId(1)),
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
                                        instructions: 0..3,
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
                                            local: data::graph::ParamLocal::BoolFunction {
                                                local: data::graph::BoolFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Bool,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                            family: data::function::FunctionReturnFamily::Bool,
                                            kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Bool(data::function::BoolFunctionId(7))),
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
                                            shape: data::type_::ValueShapeId(4),
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
                                                function: data::function::ProfiledFunctionFunctionId::Bool(data::function::BoolFunctionFunctionId(0)),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::BoolFunction {
                                                        local: data::graph::BoolFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Bool,
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                        },
                                                    },
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "callable_captures", data::source::SourceSpan::new(1336, 1361)),
                                            },
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::FunctionCall {
                                            function: data::graph::BoolFunctionLocalId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "callable_captures", data::source::SourceSpan::new(1364, 1376)),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
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
                                                target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(10)),
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
                                            kind: data::graph::FunctionInstructionKind::Closure {
                                                target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(11)),
                                                captures: data::Storage::Static(&[
                                                    data::graph::FunctionCapture::IntFunction {
                                                        target: data::graph::IntFunctionLocalId(0),
                                                        source: data::graph::IntFunctionLocalId(0),
                                                    },
                                                    data::graph::FunctionCapture::IntFunction {
                                                        target: data::graph::IntFunctionLocalId(1),
                                                        source: data::graph::IntFunctionLocalId(0),
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
                                            function: data::graph::IntFunctionLocalId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "aliases", data::source::SourceSpan::new(1548, 1563)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::IntegerRegion(data::graph::ArithmeticRegion {
                                        inputs: data::Storage::Static(&[
                                            data::graph::IntLocalId(1),
                                            data::graph::IntLocalId(0),
                                        ]),
                                        nodes: data::Storage::Static(&[
                                            data::graph::ArithmeticNode::Add(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Input(1)),
                                            data::graph::ArithmeticNode::Multiply(data::graph::ArithmeticOperand::Immediate(2), data::graph::ArithmeticOperand::Value(0)),
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
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::EqualInt {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(3)),
                                        })),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
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
                                        instructions: 0..3,
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
                                            local: data::graph::ParamLocal::BoolFunction {
                                                local: data::graph::BoolFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(4),
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
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "producer_suffix_bool", data::source::SourceSpan::new(2429, 2448)),
                                            },
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::FunctionCall {
                                            function: data::graph::BoolFunctionLocalId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "producer_suffix_bool", data::source::SourceSpan::new(2452, 2464)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Not(data::graph::BoolLocalId(0)))),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::BoolLocalId(1)),
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
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                            function: data::function::BoolFunctionId(8),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "canonical_bool", data::source::SourceSpan::new(3298, 3319)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Not(data::graph::BoolLocalId(0)))),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::BoolLocalId(1)),
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
                                        instructions: 0..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                            shape: data::type_::ValueShapeId(4),
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
                                                function: data::function::ProfiledFunctionFunctionId::Bool(data::function::BoolFunctionFunctionId(2)),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "bool_captures", data::source::SourceSpan::new(3684, 3718)),
                                            },
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::FunctionCall {
                                            function: data::graph::BoolFunctionLocalId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "bool_captures", data::source::SourceSpan::new(3721, 3733)),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::BoolLocalId(1)),
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
                                            test: data::graph::BoolTest::LtEqInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(0),
                                            },
                                            true_: data::graph::Edge {
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
                                            },
                                            false_: data::graph::Edge {
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
                                        params: 1..1,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..2,
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
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
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
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::BoolFunctionId(9),
                                        site: data::source::HostCallSite::from_static("example", "even", data::source::SourceSpan::new(921, 935)),
                                    },
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
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Not(data::graph::BoolLocalId(0)))),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::BoolLocalId(1)),
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
                                        terminator: data::graph::Terminator::Echo(data::graph::Echo {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            message: None,
                                            site: data::source::EchoSite::from_static("example", "boolean_suffix", data::source::SourceSpan::new(3227, 3236)),
                                            next: data::graph::Edge {
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
                                            },
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
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::GtInt {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Immediate(0),
                                        })),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
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
                                            test: data::graph::BoolTest::LtEqInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(0),
                                            },
                                            true_: data::graph::Edge {
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
                                            },
                                            false_: data::graph::Edge {
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
                                        params: 1..1,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..2,
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
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(false)),
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
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::BoolFunctionId(6),
                                        site: data::source::HostCallSite::from_static("example", "odd", data::source::SourceSpan::new(1023, 1038)),
                                    },
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
                                        params: 0..3,
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
                                        local: data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Bool,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        shape: data::type_::ValueShapeId(3),
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
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::GtInt {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::FunctionCall {
                                            function: data::graph::BoolFunctionLocalId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "<anonymous:1>", data::source::SourceSpan::new(1232, 1256)),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::BoolLocalId(1)),
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
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                        params: 0..3,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                            subject: data::graph::BoolLocalId(0),
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
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
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 5..5,
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
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::GtInt {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
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
                int_function_functions: data::Storage::Static(&[
                    data::function::ExecutableFunction {
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
                                                kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(17))),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::IntFunctionFunctionId(3),
                                            site: data::source::HostCallSite::from_static("example", "compose", data::source::SourceSpan::new(269, 291)),
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
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[]),
                                        },
                                    },
                                ]),
                            },
                        },
                    },
                    data::function::ExecutableFunction {
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
                                            terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                                subject: data::graph::BoolLocalId(0),
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[]),
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
                                                    args: data::Storage::Static(&[]),
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
                                            params: 1..1,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 1..1,
                                            instructions: 1..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                                kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(17))),
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
                                                kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(18))),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntFunctionLocalId(0)),
                                    data::function::FunctionExit::Return(data::graph::IntFunctionLocalId(0)),
                                ]),
                            },
                        },
                    },
                    data::function::ExecutableFunction {
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
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Echo(data::graph::Echo {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                message: None,
                                                site: data::source::EchoSite::from_static("example", "int_suffix", data::source::SourceSpan::new(2069, 2078)),
                                                next: data::graph::Edge {
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
                                                },
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
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(19)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::Int {
                                                            target: data::graph::IntLocalId(1),
                                                            source: data::graph::IntLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntFunctionLocalId(0)),
                                ]),
                            },
                        },
                    },
                    data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 2,
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
                                            params: 0..2,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::LtEqInt {
                                                    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                    right: data::graph::IntegerOperand::Immediate(0),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
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
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
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
                                                                family: data::graph::StorageFamily::IntFunction,
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
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(20)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::IntFunction {
                                                            target: data::graph::IntFunctionLocalId(0),
                                                            source: data::graph::IntFunctionLocalId(0),
                                                        },
                                                    ]),
                                                },
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
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntFunctionLocalId(0)),
                                ]),
                            },
                        },
                    },
                ]),
                float_function_functions: data::Storage::Static(&[]),
                string_function_functions: data::Storage::Static(&[]),
                bit_array_function_functions: data::Storage::Static(&[]),
                utf_codepoint_function_functions: data::Storage::Static(&[]),
                custom_function_functions: data::Storage::Static(&[]),
                external_function_functions: data::Storage::Static(&[]),
                bool_function_functions: data::Storage::Static(&[
                    data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 2,
                        },
                        body: data::function::TypedFunctionBody {
                            _shape: data::type_::FunctionShape {
                                shape_id: data::type_::ValueShapeId(4),
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                },
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
                                            local: data::graph::ParamLocal::BoolFunction {
                                                local: data::graph::BoolFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
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
                                                shape: data::type_::ValueShapeId(4),
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
                                                    target: data::graph::FunctionTarget::Bool(data::function::BoolFunctionId(10)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::BoolFunction {
                                                            target: data::graph::BoolFunctionLocalId(0),
                                                            source: data::graph::BoolFunctionLocalId(0),
                                                        },
                                                        data::graph::FunctionCapture::Int {
                                                            target: data::graph::IntLocalId(1),
                                                            source: data::graph::IntLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::BoolFunctionLocalId(1)),
                                ]),
                            },
                        },
                    },
                    data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 1,
                        },
                        body: data::function::TypedFunctionBody {
                            _shape: data::type_::FunctionShape {
                                shape_id: data::type_::ValueShapeId(4),
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                },
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Echo(data::graph::Echo {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                message: None,
                                                site: data::source::EchoSite::from_static("example", "bool_suffix", data::source::SourceSpan::new(2310, 2319)),
                                                next: data::graph::Edge {
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
                                                },
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
                                                local: data::graph::ParamLocal::BoolFunction {
                                                    local: data::graph::BoolFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(4),
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
                                                    target: data::graph::FunctionTarget::Bool(data::function::BoolFunctionId(11)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::Int {
                                                            target: data::graph::IntLocalId(1),
                                                            source: data::graph::IntLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::BoolFunctionLocalId(0)),
                                ]),
                            },
                        },
                    },
                    data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 2,
                        },
                        body: data::function::TypedFunctionBody {
                            _shape: data::type_::FunctionShape {
                                shape_id: data::type_::ValueShapeId(4),
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                },
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
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
                                            function: data::function::BoolFunctionFunctionId(3),
                                            site: data::source::HostCallSite::from_static("example", "forward_predicate", data::source::SourceSpan::new(3561, 3595)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[]),
                                        },
                                    },
                                ]),
                            },
                        },
                    },
                    data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 2,
                        },
                        body: data::function::TypedFunctionBody {
                            _shape: data::type_::FunctionShape {
                                shape_id: data::type_::ValueShapeId(4),
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                },
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
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                                local: data::graph::ParamLocal::BoolFunction {
                                                    local: data::graph::BoolFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(4),
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
                                                    target: data::graph::FunctionTarget::Bool(data::function::BoolFunctionId(12)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::Bool {
                                                            target: data::graph::BoolLocalId(0),
                                                            source: data::graph::BoolLocalId(0),
                                                        },
                                                        data::graph::FunctionCapture::Int {
                                                            target: data::graph::IntLocalId(1),
                                                            source: data::graph::IntLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::BoolFunctionLocalId(0)),
                                ]),
                            },
                        },
                    },
                ]),
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
            const CALL_GROUP_0: [data::compiled::calls::CallStart; 3] = {
                use data::compiled::calls::{CallArguments, CallCapture, CallCaptureInputs, CallExecution, CallInputs, CallInteger, CallOps, CallOutput, CallProgress, CallStorage, CallValues, IntCallable};
                enum FunctionState {
                    Int0Point0 { int0: i128, int1: i128 },
                    Int0Point1 { int0: i128, int1: i128, int_function0: IntCallable },
                    Int0Point2 { int0: i128, int1: i128, int_function0: IntCallable, int2: i128 },
                    Int1Point0 { bool0: bool, int0: i128 },
                    Int1Point1 { bool0: bool, int0: i128, int_function0: IntCallable },
                    Int1Point2 { bool0: bool, int0: i128, int_function0: IntCallable, int1: i128 },
                    Int1Point3 { bool0: bool, int0: i128, int_function0: IntCallable, int1: i128, int2: i128 },
                    Int2Point0 { int0: i128 },
                    Int2Point1 { int0: i128, int1: i128 },
                    Int2Point2 { int0: i128, int1: i128, int2: i128 },
                    Int4Point0 { int0: i128 },
                    Int4Point1 { int0: i128, int1: i128 },
                    Int4Point2 { int0: i128, int1: i128, int2: i128 },
                    Int6Point0 { int0: i128, int1: i128 },
                    Int6Point1 { int0: i128, int1: i128, int_function0: IntCallable },
                    Int6Point2 { int0: i128, int1: i128, int_function0: IntCallable, int2: i128 },
                    Int6Point3 { int0: i128, int1: i128, int_function0: IntCallable, int2: i128, int3: i128 },
                    Int9Point0 { int0: i128 },
                    Int9Point1 {  },
                    Int9Point2 { int0: i128 },
                    Int9Point3 { int0: i128 },
                    Int9Point4 { int0: i128, int1: i128 },
                    Int9Point5 { int0: i128, int1: i128, int2: i128 },
                    Int9Point6 { int0: i128, int1: i128, int2: i128, int3: i128 },
                    Int10Point0 { int0: i128, int1: i128 },
                    Int10Point1 { int0: i128, int1: i128, int2: i128 },
                    Int11Point0 { int0: i128, int_function0: IntCallable, int_function1: IntCallable },
                    Int11Point1 { int0: i128, int_function0: IntCallable, int_function1: IntCallable, int1: i128 },
                    Int11Point2 { int0: i128, int_function0: IntCallable, int_function1: IntCallable, int1: i128, int2: i128 },
                    Int11Point3 { int0: i128, int_function0: IntCallable, int_function1: IntCallable, int1: i128, int2: i128, int3: i128 },
                    Int13Point0 { int0: i128 },
                    Int13Point1 { int0: i128, int1: i128 },
                    Int15Point0 { int0: i128, int1: i128 },
                    Int15Point1 { int0: i128, int1: i128, int2: i128 },
                    Int17Point0 { int0: i128 },
                    Int18Point0 { int0: i128 },
                    Int18Point1 { int0: i128, int1: i128 },
                    Int19Point0 { int0: i128, int1: i128 },
                    Int19Point1 { int0: i128, int1: i128, int2: i128 },
                    Int20Point0 { int0: i128, int_function0: IntCallable },
                    Int20Point1 { int0: i128, int_function0: IntCallable, int1: i128 },
                    Int20Point2 { int0: i128, int_function0: IntCallable, int1: i128, int2: i128 },
                    IntFunction0Point0 { int0: i128 },
                    IntFunction0Point1 { int0: i128, int_function0: IntCallable },
                    IntFunction1Point0 { bool0: bool },
                    IntFunction1Point1 {  },
                    IntFunction1Point2 { int_function0: IntCallable },
                    IntFunction1Point3 {  },
                    IntFunction1Point4 { int_function0: IntCallable },
                    IntFunction2Point0 { int0: i128 },
                    IntFunction2Point2 { int0: i128 },
                    IntFunction2Point3 { int0: i128, int_function0: IntCallable },
                    IntFunction3Point0 { int_function0: IntCallable, int0: i128 },
                    IntFunction3Point1 { int_function0: IntCallable },
                    IntFunction3Point2 { int_function0: IntCallable, int0: i128 },
                    IntFunction3Point3 { int_function0: IntCallable, int0: i128, int_function1: IntCallable },
                    IntFunction3Point4 { int_function0: IntCallable, int0: i128, int_function1: IntCallable, int1: i128 },
                    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                }
                enum IntReturn {
                    Int0Call1 { int0: i128, int1: i128, int_function0: IntCallable },
                    Int1Call1 { bool0: bool, int0: i128, int_function0: IntCallable },
                    Int2Call0 { int0: i128 },
                    Int4Call0 { int0: i128 },
                    Int6Call1 { int0: i128, int1: i128, int_function0: IntCallable },
                    Int9Call4 { int0: i128, int1: i128 },
                    Int11Call0 { int0: i128, int_function0: IntCallable, int_function1: IntCallable },
                    Int11Call1 { int0: i128, int_function0: IntCallable, int_function1: IntCallable, int1: i128 },
                    Int20Call0 { int0: i128, int_function0: IntCallable },
                }
                impl IntReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                            Self::Int0Call1 { .. } => data::source::HostCallSite::from_static("example", "capture_chain", data::source::SourceSpan::new(384, 400)),
                            Self::Int1Call1 { .. } => data::source::HostCallSite::from_static("example", "dynamic_target", data::source::SourceSpan::new(646, 662)),
                            Self::Int2Call0 { .. } => data::source::HostCallSite::from_static("example", "nested", data::source::SourceSpan::new(818, 833)),
                            Self::Int4Call0 { .. } => data::source::HostCallSite::from_static("example", "big_return", data::source::SourceSpan::new(1814, 1829)),
                            Self::Int6Call1 { .. } => data::source::HostCallSite::from_static("example", "producer_suffix_int", data::source::SourceSpan::new(2211, 2227)),
                            Self::Int9Call4 { .. } => data::source::HostCallSite::from_static("example", "non_tail", data::source::SourceSpan::new(750, 769)),
                            Self::Int11Call0 { .. } => data::source::HostCallSite::from_static("example", "<anonymous:3>", data::source::SourceSpan::new(1518, 1530)),
                            Self::Int11Call1 { .. } => data::source::HostCallSite::from_static("example", "<anonymous:3>", data::source::SourceSpan::new(1533, 1543)),
                            Self::Int20Call0 { .. } => data::source::HostCallSite::from_static("example", "<anonymous:0>", data::source::SourceSpan::new(183, 198)),
                        }
                    }
                    fn small(self, result: i128) -> FunctionState {
                        match self {
                            Self::Int0Call1 { int0, int1, int_function0 } => {
                                let int2 = result;
                                FunctionState::Int0Point2 { int0, int1, int_function0, int2 }
                            },
                            Self::Int1Call1 { bool0, int0, int_function0 } => {
                                let int1 = result;
                                FunctionState::Int1Point2 { bool0, int0, int_function0, int1 }
                            },
                            Self::Int2Call0 { int0 } => {
                                let int1 = result;
                                FunctionState::Int2Point1 { int0, int1 }
                            },
                            Self::Int4Call0 { int0 } => {
                                let int1 = result;
                                FunctionState::Int4Point1 { int0, int1 }
                            },
                            Self::Int6Call1 { int0, int1, int_function0 } => {
                                let int2 = result;
                                FunctionState::Int6Point2 { int0, int1, int_function0, int2 }
                            },
                            Self::Int9Call4 { int0, int1 } => {
                                let int2 = result;
                                FunctionState::Int9Point5 { int0, int1, int2 }
                            },
                            Self::Int11Call0 { int0, int_function0, int_function1 } => {
                                let int1 = result;
                                FunctionState::Int11Point1 { int0, int_function0, int_function1, int1 }
                            },
                            Self::Int11Call1 { int0, int_function0, int_function1, int1 } => {
                                let int2 = result;
                                FunctionState::Int11Point2 { int0, int_function0, int_function1, int1, int2 }
                            },
                            Self::Int20Call0 { int0, int_function0 } => {
                                let int1 = result;
                                FunctionState::Int20Point1 { int0, int_function0, int1 }
                            },
                        }
                    }
                    fn resume(self, result: CallInteger) -> FunctionState {
                        if let Some(result) = result.small() {
                            return self.small(result);
                        }
                        match self {
                            Self::Int0Call1 { int0, int1, int_function0 } => {
                                let int2 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
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
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2], int_functions: vec![int_function0], ..CallValues::default() }) }
                            },
                            Self::Int1Call1 { bool0, int0, int_function0 } => {
                                let int1 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(1)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 2,
                                    ints: 2,
                                    bools: 1,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 1,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1], int_functions: vec![int_function0], bools: vec![bool0], ..CallValues::default() }) }
                            },
                            Self::Int2Call0 { int0 } => {
                                let int1 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: data::compiled::CompiledCheckpoint {
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
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1], ..CallValues::default() }) }
                            },
                            Self::Int4Call0 { int0 } => {
                                let int1 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(4)), point: data::compiled::CompiledCheckpoint {
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
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1], ..CallValues::default() }) }
                            },
                            Self::Int6Call1 { int0, int1, int_function0 } => {
                                let int2 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(6)), point: data::compiled::CompiledCheckpoint {
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
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2], int_functions: vec![int_function0], ..CallValues::default() }) }
                            },
                            Self::Int9Call4 { int0, int1 } => {
                                let int2 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(9)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 2,
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
                            Self::Int11Call0 { int0, int_function0, int_function1 } => {
                                let int1 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 1,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 2,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1], int_functions: vec![int_function0, int_function1], ..CallValues::default() }) }
                            },
                            Self::Int11Call1 { int0, int_function0, int_function1, int1 } => {
                                let int2 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 2,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 2,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2], int_functions: vec![int_function0, int_function1], ..CallValues::default() }) }
                            },
                            Self::Int20Call0 { int0, int_function0 } => {
                                let int1 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(20)), point: data::compiled::CompiledCheckpoint {
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
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1], int_functions: vec![int_function0], ..CallValues::default() }) }
                            },
                        }
                    }
                }
                enum IntFunctionReturn {
                    Int0Call0 { int0: i128, int1: i128 },
                    Int1Call0 { bool0: bool, int0: i128 },
                    Int6Call0 { int0: i128, int1: i128 },
                }
                impl IntFunctionReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                            Self::Int0Call0 { .. } => data::source::HostCallSite::from_static("example", "capture_chain", data::source::SourceSpan::new(367, 381)),
                            Self::Int1Call0 { .. } => data::source::HostCallSite::from_static("example", "dynamic_target", data::source::SourceSpan::new(629, 643)),
                            Self::Int6Call0 { .. } => data::source::HostCallSite::from_static("example", "producer_suffix_int", data::source::SourceSpan::new(2190, 2208)),
                        }
                    }
                    fn small(self, result: IntCallable) -> FunctionState {
                        match self {
                            Self::Int0Call0 { int0, int1 } => {
                                let int_function0 = result.with_type(data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                });
                                FunctionState::Int0Point1 { int0, int1, int_function0 }
                            },
                            Self::Int1Call0 { bool0, int0 } => {
                                let int_function0 = result.with_type(data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                });
                                FunctionState::Int1Point1 { bool0, int0, int_function0 }
                            },
                            Self::Int6Call0 { int0, int1 } => {
                                let int_function0 = result.with_type(data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                });
                                FunctionState::Int6Point1 { int0, int1, int_function0 }
                            },
                        }
                    }
                    fn resume(self, result: IntCallable) -> FunctionState { self.small(result) }
                }
                #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                enum FunctionStep {
                    Yield(FunctionState),
                    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                    IntCall { callee: FunctionState, caller: IntReturn },
                    Int { value: i128 },
                    IntBridge { function: data::function::IntFunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: IntReturn },
                    IntFunctionCall { callee: FunctionState, caller: IntFunctionReturn },
                    IntFunctionTail { callee: FunctionState },
                    IntFunction { value: IntCallable },
                }
                struct FunctionExecution {
                    active: Option<FunctionState>,
                    pending_entry: bool,
                    integer_returns: Vec<IntReturn>,
                    integer_function_returns: Vec<IntFunctionReturn>,
                }
                impl FunctionExecution {
                    fn new(active: FunctionState) -> Self {
                        Self {
                            active: Some(active),
                            pending_entry: false,
                            integer_returns: Vec::new(),
                            integer_function_returns: Vec::new(),
                        }
                    }
                }
                impl CallExecution for FunctionExecution {
                    fn restart(&mut self, target: data::compiled::CallTarget, point: usize, values: CallInputs<'_>) -> bool {
                        if self.active.is_some() { return false; }
                        let active = match target {
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(0)) => calls_int_0_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(1)) => calls_int_1_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(6)) => calls_int_6_state(point, values),
                            _ => None,
                        };
                        let Some(active) = active else { return false; };
                        self.active = Some(active);
                        true
                    }
                    fn retained_bytes(&self) -> usize {
                        std::mem::size_of::<Self>() + self.integer_returns.capacity() * std::mem::size_of::<IntReturn>() + self.integer_function_returns.capacity() * std::mem::size_of::<IntFunctionReturn>()
                    }
                    fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                        let Some(mut active) = self.active.take() else { return CallProgress::Yield(self); };
                        loop {
                            if self.pending_entry {
                                if *budget == 0 { self.active = Some(active); return CallProgress::Yield(self); }
                                *budget -= 1;
                                self.pending_entry = false;
                            }
                            match function_step(active, ops, budget) {
                                FunctionStep::Yield(active) => {
                                    self.active = Some(active);
                                    return CallProgress::Yield(self);
                                },
                                FunctionStep::IntCall { callee, caller } => {
                                    self.integer_returns.push(caller);
                                    active = callee;
                                },
                                FunctionStep::Int { value } => {
                                    if let Some(caller) = self.integer_returns.pop() {
                                        active = caller.small(value);
                                    } else {
                                        self.integer_returns.clear();
                                        self.integer_function_returns.clear();
                                        return CallProgress::Complete { output: CallOutput::Int(value.into()), execution: self };
                                    }
                                },
                                FunctionStep::IntBridge { function, site, arguments, caller } => return CallProgress::Int {
                                    function, site, arguments,
                                    resume: Box::new(move |value| {
                                        self.active = Some(caller.resume(value));
                                        self
                                    }),
                                },
                                FunctionStep::IntFunctionCall { callee, caller } => {
                                    self.integer_function_returns.push(caller);
                                    active = callee;
                                },
                                FunctionStep::IntFunctionTail { callee } => {
                                    *budget -= 1;
                                    self.pending_entry = self.integer_function_returns.is_empty() && ops.root_tail_entry();
                                    active = callee;
                                },
                                FunctionStep::IntFunction { value } => {
                                    if let Some(caller) = self.integer_function_returns.pop() {
                                        active = caller.small(value);
                                    } else {
                                        self.integer_returns.clear();
                                        self.integer_function_returns.clear();
                                        return CallProgress::Complete { output: CallOutput::IntFunction(value), execution: self };
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
                                            return CallProgress::Interpreted { target, point, values };
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
                        FunctionState::Canonical { target, point, values } => FunctionStep::Canonical { target, point, values },
                        FunctionState::Int0Point0 { int0, int1 } => calls_int_0_run(Int0State::Point0 { int0, int1 }, ops, budget),
                        FunctionState::Int0Point1 { int0, int1, int_function0 } => calls_int_0_run(Int0State::Point1 { int0, int1, int_function0 }, ops, budget),
                        FunctionState::Int0Point2 { int0, int1, int_function0, int2 } => calls_int_0_run(Int0State::Point2 { int0, int1, int_function0, int2 }, ops, budget),
                        FunctionState::Int1Point0 { bool0, int0 } => calls_int_1_run(Int1State::Point0 { bool0, int0 }, ops, budget),
                        FunctionState::Int1Point1 { bool0, int0, int_function0 } => calls_int_1_run(Int1State::Point1 { bool0, int0, int_function0 }, ops, budget),
                        FunctionState::Int1Point2 { bool0, int0, int_function0, int1 } => calls_int_1_run(Int1State::Point2 { bool0, int0, int_function0, int1 }, ops, budget),
                        FunctionState::Int1Point3 { bool0, int0, int_function0, int1, int2 } => calls_int_1_run(Int1State::Point3 { bool0, int0, int_function0, int1, int2 }, ops, budget),
                        FunctionState::Int2Point0 { int0 } => calls_int_2_run(Int2State::Point0 { int0 }, ops, budget),
                        FunctionState::Int2Point1 { int0, int1 } => calls_int_2_run(Int2State::Point1 { int0, int1 }, ops, budget),
                        FunctionState::Int2Point2 { int0, int1, int2 } => calls_int_2_run(Int2State::Point2 { int0, int1, int2 }, ops, budget),
                        FunctionState::Int4Point0 { int0 } => calls_int_4_run(Int4State::Point0 { int0 }, ops, budget),
                        FunctionState::Int4Point1 { int0, int1 } => calls_int_4_run(Int4State::Point1 { int0, int1 }, ops, budget),
                        FunctionState::Int4Point2 { int0, int1, int2 } => calls_int_4_run(Int4State::Point2 { int0, int1, int2 }, ops, budget),
                        FunctionState::Int6Point0 { int0, int1 } => calls_int_6_run(Int6State::Point0 { int0, int1 }, ops, budget),
                        FunctionState::Int6Point1 { int0, int1, int_function0 } => calls_int_6_run(Int6State::Point1 { int0, int1, int_function0 }, ops, budget),
                        FunctionState::Int6Point2 { int0, int1, int_function0, int2 } => calls_int_6_run(Int6State::Point2 { int0, int1, int_function0, int2 }, ops, budget),
                        FunctionState::Int6Point3 { int0, int1, int_function0, int2, int3 } => calls_int_6_run(Int6State::Point3 { int0, int1, int_function0, int2, int3 }, ops, budget),
                        FunctionState::Int9Point0 { int0 } => calls_int_9_run(Int9State::Point0 { int0 }, ops, budget),
                        FunctionState::Int9Point1 {  } => calls_int_9_run(Int9State::Point1 {  }, ops, budget),
                        FunctionState::Int9Point2 { int0 } => calls_int_9_run(Int9State::Point2 { int0 }, ops, budget),
                        FunctionState::Int9Point3 { int0 } => calls_int_9_run(Int9State::Point3 { int0 }, ops, budget),
                        FunctionState::Int9Point4 { int0, int1 } => calls_int_9_run(Int9State::Point4 { int0, int1 }, ops, budget),
                        FunctionState::Int9Point5 { int0, int1, int2 } => calls_int_9_run(Int9State::Point5 { int0, int1, int2 }, ops, budget),
                        FunctionState::Int9Point6 { int0, int1, int2, int3 } => calls_int_9_run(Int9State::Point6 { int0, int1, int2, int3 }, ops, budget),
                        FunctionState::Int10Point0 { int0, int1 } => calls_int_10_run(Int10State::Point0 { int0, int1 }, ops, budget),
                        FunctionState::Int10Point1 { int0, int1, int2 } => calls_int_10_run(Int10State::Point1 { int0, int1, int2 }, ops, budget),
                        FunctionState::Int11Point0 { int0, int_function0, int_function1 } => calls_int_11_run(Int11State::Point0 { int0, int_function0, int_function1 }, ops, budget),
                        FunctionState::Int11Point1 { int0, int_function0, int_function1, int1 } => calls_int_11_run(Int11State::Point1 { int0, int_function0, int_function1, int1 }, ops, budget),
                        FunctionState::Int11Point2 { int0, int_function0, int_function1, int1, int2 } => calls_int_11_run(Int11State::Point2 { int0, int_function0, int_function1, int1, int2 }, ops, budget),
                        FunctionState::Int11Point3 { int0, int_function0, int_function1, int1, int2, int3 } => calls_int_11_run(Int11State::Point3 { int0, int_function0, int_function1, int1, int2, int3 }, ops, budget),
                        FunctionState::Int13Point0 { int0 } => calls_int_13_run(Int13State::Point0 { int0 }, ops, budget),
                        FunctionState::Int13Point1 { int0, int1 } => calls_int_13_run(Int13State::Point1 { int0, int1 }, ops, budget),
                        FunctionState::Int15Point0 { int0, int1 } => calls_int_15_run(Int15State::Point0 { int0, int1 }, ops, budget),
                        FunctionState::Int15Point1 { int0, int1, int2 } => calls_int_15_run(Int15State::Point1 { int0, int1, int2 }, ops, budget),
                        FunctionState::Int17Point0 { int0 } => calls_int_17_run(Int17State::Point0 { int0 }, ops, budget),
                        FunctionState::Int18Point0 { int0 } => calls_int_18_run(Int18State::Point0 { int0 }, ops, budget),
                        FunctionState::Int18Point1 { int0, int1 } => calls_int_18_run(Int18State::Point1 { int0, int1 }, ops, budget),
                        FunctionState::Int19Point0 { int0, int1 } => calls_int_19_run(Int19State::Point0 { int0, int1 }, ops, budget),
                        FunctionState::Int19Point1 { int0, int1, int2 } => calls_int_19_run(Int19State::Point1 { int0, int1, int2 }, ops, budget),
                        FunctionState::Int20Point0 { int0, int_function0 } => calls_int_20_run(Int20State::Point0 { int0, int_function0 }, ops, budget),
                        FunctionState::Int20Point1 { int0, int_function0, int1 } => calls_int_20_run(Int20State::Point1 { int0, int_function0, int1 }, ops, budget),
                        FunctionState::Int20Point2 { int0, int_function0, int1, int2 } => calls_int_20_run(Int20State::Point2 { int0, int_function0, int1, int2 }, ops, budget),
                        FunctionState::IntFunction0Point0 { int0 } => calls_intfunction_0_run(IntFunction0State::Point0 { int0 }, ops, budget),
                        FunctionState::IntFunction0Point1 { int0, int_function0 } => calls_intfunction_0_run(IntFunction0State::Point1 { int0, int_function0 }, ops, budget),
                        FunctionState::IntFunction1Point0 { bool0 } => calls_intfunction_1_run(IntFunction1State::Point0 { bool0 }, ops, budget),
                        FunctionState::IntFunction1Point1 {  } => calls_intfunction_1_run(IntFunction1State::Point1 {  }, ops, budget),
                        FunctionState::IntFunction1Point2 { int_function0 } => calls_intfunction_1_run(IntFunction1State::Point2 { int_function0 }, ops, budget),
                        FunctionState::IntFunction1Point3 {  } => calls_intfunction_1_run(IntFunction1State::Point3 {  }, ops, budget),
                        FunctionState::IntFunction1Point4 { int_function0 } => calls_intfunction_1_run(IntFunction1State::Point4 { int_function0 }, ops, budget),
                        FunctionState::IntFunction2Point0 { int0 } => calls_intfunction_2_run(IntFunction2State::Point0 { int0 }, ops, budget),
                        FunctionState::IntFunction2Point2 { int0 } => calls_intfunction_2_run(IntFunction2State::Point2 { int0 }, ops, budget),
                        FunctionState::IntFunction2Point3 { int0, int_function0 } => calls_intfunction_2_run(IntFunction2State::Point3 { int0, int_function0 }, ops, budget),
                        FunctionState::IntFunction3Point0 { int_function0, int0 } => calls_intfunction_3_run(IntFunction3State::Point0 { int_function0, int0 }, ops, budget),
                        FunctionState::IntFunction3Point1 { int_function0 } => calls_intfunction_3_run(IntFunction3State::Point1 { int_function0 }, ops, budget),
                        FunctionState::IntFunction3Point2 { int_function0, int0 } => calls_intfunction_3_run(IntFunction3State::Point2 { int_function0, int0 }, ops, budget),
                        FunctionState::IntFunction3Point3 { int_function0, int0, int_function1 } => calls_intfunction_3_run(IntFunction3State::Point3 { int_function0, int0, int_function1 }, ops, budget),
                        FunctionState::IntFunction3Point4 { int_function0, int0, int_function1, int1 } => calls_intfunction_3_run(IntFunction3State::Point4 { int_function0, int0, int_function1, int1 }, ops, budget),
                    }
                }
                enum Int0State {
                    Point0 { int0: i128, int1: i128 },
                    Point1 { int0: i128, int1: i128, int_function0: IntCallable },
                    Point2 { int0: i128, int1: i128, int_function0: IntCallable, int2: i128 },
                }
                fn calls_int_0_run(active: Int0State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int0State::Point0 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point0 { int0, int1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::IntFunctionCall { callee: FunctionState::IntFunction0Point0 { int0 }, caller: IntFunctionReturn::Int0Call0 { int0, int1 } }
                            }
                        },
                        Int0State::Point1 { int0, int1, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point1 { int0, int1, int_function0 }); }
                            *budget -= 1;
                            {
                                let callable = &int_function0;
                                let captures = callable.captures();
                                let target = callable.target();
                                if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (int1,)) {
                                    return FunctionStep::IntCall { callee, caller: IntReturn::Int0Call1 { int0, int1, int_function0 } };
                                }
                                FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("example", "capture_chain", data::source::SourceSpan::new(384, 400)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int1.into()], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: IntReturn::Int0Call1 { int0, int1, int_function0 } }
                            }
                        },
                        Int0State::Point2 { int0, int1, int_function0, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point2 { int0, int1, int_function0, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                    }
                }
                enum Int1State {
                    Point0 { bool0: bool, int0: i128 },
                    Point1 { bool0: bool, int0: i128, int_function0: IntCallable },
                    Point2 { bool0: bool, int0: i128, int_function0: IntCallable, int1: i128 },
                    Point3 { bool0: bool, int0: i128, int_function0: IntCallable, int1: i128, int2: i128 },
                }
                fn calls_int_1_run(active: Int1State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int1State::Point0 { bool0, int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point0 { bool0, int0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::IntFunctionCall { callee: FunctionState::IntFunction1Point0 { bool0 }, caller: IntFunctionReturn::Int1Call0 { bool0, int0 } }
                            }
                        },
                        Int1State::Point1 { bool0, int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point1 { bool0, int0, int_function0 }); }
                            *budget -= 1;
                            {
                                let callable = &int_function0;
                                let captures = callable.captures();
                                let target = callable.target();
                                if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (int0,)) {
                                    return FunctionStep::IntCall { callee, caller: IntReturn::Int1Call1 { bool0, int0, int_function0 } };
                                }
                                FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("example", "dynamic_target", data::source::SourceSpan::new(646, 662)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int0.into()], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: IntReturn::Int1Call1 { bool0, int0, int_function0 } }
                            }
                        },
                        Int1State::Point2 { bool0, int0, int_function0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point2 { bool0, int0, int_function0, int1 }); }
                            *budget -= 1;
                            let int2 = int1 + 3_i128;
                            if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(1)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 3,
                                ints: 3,
                                bools: 1,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 1,
                                bool_functions: 0,
                            }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into()], int_functions: vec![int_function0], bools: vec![bool0], ..CallValues::default() }) }; }
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point3 { bool0, int0, int_function0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                        Int1State::Point3 { bool0, int0, int_function0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point3 { bool0, int0, int_function0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                    }
                }
                enum Int2State {
                    Point0 { int0: i128 },
                    Point1 { int0: i128, int1: i128 },
                    Point2 { int0: i128, int1: i128, int2: i128 },
                }
                fn calls_int_2_run(active: Int2State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int2State::Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point0 { int0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::IntCall { callee: FunctionState::Int9Point0 { int0 }, caller: IntReturn::Int2Call0 { int0 } }
                            }
                        },
                        Int2State::Point1 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point1 { int0, int1 }); }
                            *budget -= 1;
                            let int2 = int1 + 2_i128;
                            if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 2,
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
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point2 { int0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                        Int2State::Point2 { int0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point2 { int0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                    }
                }
                enum Int4State {
                    Point0 { int0: i128 },
                    Point1 { int0: i128, int1: i128 },
                    Point2 { int0: i128, int1: i128, int2: i128 },
                }
                fn calls_int_4_run(active: Int4State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int4State::Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int4Point0 { int0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::IntCall { callee: FunctionState::Int13Point0 { int0 }, caller: IntReturn::Int4Call0 { int0 } }
                            }
                        },
                        Int4State::Point1 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int4Point1 { int0, int1 }); }
                            *budget -= 1;
                            let int2 = int1 + 3_i128;
                            if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(4)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 2,
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
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int4Point2 { int0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                        Int4State::Point2 { int0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int4Point2 { int0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                    }
                }
                enum Int6State {
                    Point0 { int0: i128, int1: i128 },
                    Point1 { int0: i128, int1: i128, int_function0: IntCallable },
                    Point2 { int0: i128, int1: i128, int_function0: IntCallable, int2: i128 },
                    Point3 { int0: i128, int1: i128, int_function0: IntCallable, int2: i128, int3: i128 },
                }
                fn calls_int_6_run(active: Int6State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int6State::Point0 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int6Point0 { int0, int1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::IntFunctionCall { callee: FunctionState::IntFunction2Point0 { int0 }, caller: IntFunctionReturn::Int6Call0 { int0, int1 } }
                            }
                        },
                        Int6State::Point1 { int0, int1, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int6Point1 { int0, int1, int_function0 }); }
                            *budget -= 1;
                            {
                                let callable = &int_function0;
                                let captures = callable.captures();
                                let target = callable.target();
                                if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (int1,)) {
                                    return FunctionStep::IntCall { callee, caller: IntReturn::Int6Call1 { int0, int1, int_function0 } };
                                }
                                FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("example", "producer_suffix_int", data::source::SourceSpan::new(2211, 2227)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int1.into()], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: IntReturn::Int6Call1 { int0, int1, int_function0 } }
                            }
                        },
                        Int6State::Point2 { int0, int1, int_function0, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int6Point2 { int0, int1, int_function0, int2 }); }
                            *budget -= 1;
                            let int3 = int2 + 2_i128;
                            if int3 < i128::from(i64::MIN) || int3 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(6)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 3,
                                ints: 4,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 1,
                                bool_functions: 0,
                            }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into()], int_functions: vec![int_function0], ..CallValues::default() }) }; }
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int6Point3 { int0, int1, int_function0, int2, int3 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int3 }
                            }
                        },
                        Int6State::Point3 { int0, int1, int_function0, int2, int3 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int6Point3 { int0, int1, int_function0, int2, int3 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int3 }
                            }
                        },
                    }
                }
                enum Int9State {
                    Point0 { int0: i128 },
                    Point1 {  },
                    Point2 { int0: i128 },
                    Point3 { int0: i128 },
                    Point4 { int0: i128, int1: i128 },
                    Point5 { int0: i128, int1: i128, int2: i128 },
                    Point6 { int0: i128, int1: i128, int2: i128, int3: i128 },
                }
                fn calls_int_9_run(mut active: Int9State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            Int9State::Point0 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point0 { int0 }); }
                                *budget -= 1;
                                active = {
                                    if int0 <= 0_i128 { Int9State::Point1 {  } } else { Int9State::Point3 { int0 } }
                                };
                                continue;
                            },
                            Int9State::Point1 {  } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point1 {  }); }
                                *budget -= 1;
                                let int0 = 0_i128;
                                if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(9)), point: data::compiled::CompiledCheckpoint {
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
                                }, values: Box::new(CallValues { ints: vec![int0.into()], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point2 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int9State::Point2 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point2 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int9State::Point3 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point3 { int0 }); }
                                *budget -= 1;
                                let int1 = int0 - 1_i128;
                                if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(9)), point: data::compiled::CompiledCheckpoint {
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
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point4 { int0, int1 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntCall { callee: FunctionState::Int9Point0 { int0: int1 }, caller: IntReturn::Int9Call4 { int0, int1 } }
                                };
                            },
                            Int9State::Point4 { int0, int1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point4 { int0, int1 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntCall { callee: FunctionState::Int9Point0 { int0: int1 }, caller: IntReturn::Int9Call4 { int0, int1 } }
                                };
                            },
                            Int9State::Point5 { int0, int1, int2 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point5 { int0, int1, int2 }); }
                                *budget -= 1;
                                let int3 = int2 + 1_i128;
                                if int3 < i128::from(i64::MIN) || int3 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(9)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 3,
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
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point6 { int0, int1, int2, int3 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int3 }
                                };
                            },
                            Int9State::Point6 { int0, int1, int2, int3 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point6 { int0, int1, int2, int3 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int3 }
                                };
                            },
                        }
                    }
                }
                enum Int10State {
                    Point0 { int0: i128, int1: i128 },
                    Point1 { int0: i128, int1: i128, int2: i128 },
                }
                fn calls_int_10_run(active: Int10State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int10State::Point0 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int10Point0 { int0, int1 }); }
                            *budget -= 1;
                            let int2 = int0 + int1;
                            if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(10)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
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
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int10Point1 { int0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                        Int10State::Point1 { int0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int10Point1 { int0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                    }
                }
                enum Int11State {
                    Point0 { int0: i128, int_function0: IntCallable, int_function1: IntCallable },
                    Point1 { int0: i128, int_function0: IntCallable, int_function1: IntCallable, int1: i128 },
                    Point2 { int0: i128, int_function0: IntCallable, int_function1: IntCallable, int1: i128, int2: i128 },
                    Point3 { int0: i128, int_function0: IntCallable, int_function1: IntCallable, int1: i128, int2: i128, int3: i128 },
                }
                fn calls_int_11_run(active: Int11State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int11State::Point0 { int0, int_function0, int_function1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point0 { int0, int_function0, int_function1 }); }
                            *budget -= 1;
                            {
                                let callable = &int_function0;
                                let captures = callable.captures();
                                let target = callable.target();
                                if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (int0,)) {
                                    return FunctionStep::IntCall { callee, caller: IntReturn::Int11Call0 { int0, int_function0, int_function1 } };
                                }
                                FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("example", "<anonymous:3>", data::source::SourceSpan::new(1518, 1530)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int0.into()], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: IntReturn::Int11Call0 { int0, int_function0, int_function1 } }
                            }
                        },
                        Int11State::Point1 { int0, int_function0, int_function1, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point1 { int0, int_function0, int_function1, int1 }); }
                            *budget -= 1;
                            {
                                let callable = &int_function1;
                                let captures = callable.captures();
                                let target = callable.target();
                                if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (int0,)) {
                                    return FunctionStep::IntCall { callee, caller: IntReturn::Int11Call1 { int0, int_function0, int_function1, int1 } };
                                }
                                FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("example", "<anonymous:3>", data::source::SourceSpan::new(1533, 1543)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int0.into()], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: IntReturn::Int11Call1 { int0, int_function0, int_function1, int1 } }
                            }
                        },
                        Int11State::Point2 { int0, int_function0, int_function1, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point2 { int0, int_function0, int_function1, int1, int2 }); }
                            *budget -= 1;
                            let int3 = int1 + int2;
                            if int3 < i128::from(i64::MIN) || int3 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 3,
                                ints: 4,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 2,
                                bool_functions: 0,
                            }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into()], int_functions: vec![int_function0, int_function1], ..CallValues::default() }) }; }
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point3 { int0, int_function0, int_function1, int1, int2, int3 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int3 }
                            }
                        },
                        Int11State::Point3 { int0, int_function0, int_function1, int1, int2, int3 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point3 { int0, int_function0, int_function1, int1, int2, int3 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int3 }
                            }
                        },
                    }
                }
                enum Int13State {
                    Point0 { int0: i128 },
                    Point1 { int0: i128, int1: i128 },
                }
                fn calls_int_13_run(active: Int13State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int13State::Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point0 { int0 }); }
                            *budget -= 1;
                            let int1 = int0 * int0;
                            if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(13)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], ..CallValues::default() }) }; }
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point1 { int0, int1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int1 }
                            }
                        },
                        Int13State::Point1 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point1 { int0, int1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int1 }
                            }
                        },
                    }
                }
                enum Int15State {
                    Point0 { int0: i128, int1: i128 },
                    Point1 { int0: i128, int1: i128, int2: i128 },
                }
                fn calls_int_15_run(active: Int15State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int15State::Point0 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int15Point0 { int0, int1 }); }
                            *budget -= 1;
                            let int2 = int0 + int1;
                            if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(15)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
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
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int15Point1 { int0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                        Int15State::Point1 { int0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int15Point1 { int0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                    }
                }
                enum Int17State {
                    Point0 { int0: i128 },
                }
                fn calls_int_17_run(active: Int17State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int17State::Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int17Point0 { int0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int0 }
                            }
                        },
                    }
                }
                enum Int18State {
                    Point0 { int0: i128 },
                    Point1 { int0: i128, int1: i128 },
                }
                fn calls_int_18_run(active: Int18State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int18State::Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int18Point0 { int0 }); }
                            *budget -= 1;
                            let int1 = int0 + 7_i128;
                            if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(18)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], ..CallValues::default() }) }; }
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int18Point1 { int0, int1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int1 }
                            }
                        },
                        Int18State::Point1 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int18Point1 { int0, int1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int1 }
                            }
                        },
                    }
                }
                enum Int19State {
                    Point0 { int0: i128, int1: i128 },
                    Point1 { int0: i128, int1: i128, int2: i128 },
                }
                fn calls_int_19_run(active: Int19State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int19State::Point0 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int19Point0 { int0, int1 }); }
                            *budget -= 1;
                            let int2 = int0 + int1;
                            if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(19)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
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
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int19Point1 { int0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                        Int19State::Point1 { int0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int19Point1 { int0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                    }
                }
                enum Int20State {
                    Point0 { int0: i128, int_function0: IntCallable },
                    Point1 { int0: i128, int_function0: IntCallable, int1: i128 },
                    Point2 { int0: i128, int_function0: IntCallable, int1: i128, int2: i128 },
                }
                fn calls_int_20_run(active: Int20State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int20State::Point0 { int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int20Point0 { int0, int_function0 }); }
                            *budget -= 1;
                            {
                                let callable = &int_function0;
                                let captures = callable.captures();
                                let target = callable.target();
                                if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (int0,)) {
                                    return FunctionStep::IntCall { callee, caller: IntReturn::Int20Call0 { int0, int_function0 } };
                                }
                                FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("example", "<anonymous:0>", data::source::SourceSpan::new(183, 198)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int0.into()], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: IntReturn::Int20Call0 { int0, int_function0 } }
                            }
                        },
                        Int20State::Point1 { int0, int_function0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int20Point1 { int0, int_function0, int1 }); }
                            *budget -= 1;
                            let int2 = int1 + 1_i128;
                            if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(20)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into()], int_functions: vec![int_function0], ..CallValues::default() }) }; }
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int20Point2 { int0, int_function0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                        Int20State::Point2 { int0, int_function0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int20Point2 { int0, int_function0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                    }
                }
                enum IntFunction0State {
                    Point0 { int0: i128 },
                    Point1 { int0: i128, int_function0: IntCallable },
                }
                fn calls_intfunction_0_run(active: IntFunction0State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        IntFunction0State::Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction0Point0 { int0 }); }
                            *budget -= 1;
                            let int_function0 = ops.int_reference(data::function::IntFunctionId(17), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            });
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction0Point1 { int0, int_function0 }); }
                            {
                                FunctionStep::IntFunctionTail { callee: FunctionState::IntFunction3Point0 { int_function0: int_function0.clone(), int0 } }
                            }
                        },
                        IntFunction0State::Point1 { int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction0Point1 { int0, int_function0 }); }
                            {
                                FunctionStep::IntFunctionTail { callee: FunctionState::IntFunction3Point0 { int_function0: int_function0.clone(), int0 } }
                            }
                        },
                    }
                }
                enum IntFunction1State {
                    Point0 { bool0: bool },
                    Point1 {  },
                    Point2 { int_function0: IntCallable },
                    Point3 {  },
                    Point4 { int_function0: IntCallable },
                }
                fn calls_intfunction_1_run(mut active: IntFunction1State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            IntFunction1State::Point0 { bool0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction1Point0 { bool0 }); }
                                *budget -= 1;
                                active = {
                                    if bool0 { IntFunction1State::Point1 {  } } else { IntFunction1State::Point3 {  } }
                                };
                                continue;
                            },
                            IntFunction1State::Point1 {  } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction1Point1 {  }); }
                                *budget -= 1;
                                let int_function0 = ops.int_reference(data::function::IntFunctionId(17), data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                });
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction1Point2 { int_function0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntFunction { value: int_function0 }
                                };
                            },
                            IntFunction1State::Point2 { int_function0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction1Point2 { int_function0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntFunction { value: int_function0 }
                                };
                            },
                            IntFunction1State::Point3 {  } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction1Point3 {  }); }
                                *budget -= 1;
                                let int_function0 = ops.int_reference(data::function::IntFunctionId(18), data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                });
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction1Point4 { int_function0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntFunction { value: int_function0 }
                                };
                            },
                            IntFunction1State::Point4 { int_function0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction1Point4 { int_function0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntFunction { value: int_function0 }
                                };
                            },
                        }
                    }
                }
                enum IntFunction2State {
                    Point0 { int0: i128 },
                    Point2 { int0: i128 },
                    Point3 { int0: i128, int_function0: IntCallable },
                }
                fn calls_intfunction_2_run(active: IntFunction2State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        IntFunction2State::Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction2Point0 { int0 }); }
                            *budget -= 1;
                            let int1 = int0 + 1_i128;
                            if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(2)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], ..CallValues::default() }) }; }
                            FunctionStep::Canonical { target: data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(2)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], ..CallValues::default() }) }
                        },
                        IntFunction2State::Point2 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction2Point2 { int0 }); }
                            *budget -= 1;
                            let int_function0 = ops.int_closure(data::function::IntFunctionId(19), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            }, vec![CallCapture::int(data::graph::IntLocalId(1), int0)]);
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction2Point3 { int0, int_function0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::IntFunction { value: int_function0 }
                            }
                        },
                        IntFunction2State::Point3 { int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction2Point3 { int0, int_function0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::IntFunction { value: int_function0 }
                            }
                        },
                    }
                }
                enum IntFunction3State {
                    Point0 { int_function0: IntCallable, int0: i128 },
                    Point1 { int_function0: IntCallable },
                    Point2 { int_function0: IntCallable, int0: i128 },
                    Point3 { int_function0: IntCallable, int0: i128, int_function1: IntCallable },
                    Point4 { int_function0: IntCallable, int0: i128, int_function1: IntCallable, int1: i128 },
                }
                fn calls_intfunction_3_run(mut active: IntFunction3State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            IntFunction3State::Point0 { int_function0, int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction3Point0 { int_function0, int0 }); }
                                *budget -= 1;
                                active = {
                                    if int0 <= 0_i128 { IntFunction3State::Point1 { int_function0: int_function0.clone() } } else { IntFunction3State::Point2 { int_function0: int_function0.clone(), int0 } }
                                };
                                continue;
                            },
                            IntFunction3State::Point1 { int_function0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction3Point1 { int_function0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntFunction { value: int_function0 }
                                };
                            },
                            IntFunction3State::Point2 { int_function0, int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction3Point2 { int_function0, int0 }); }
                                *budget -= 1;
                                let int_function1 = ops.int_closure(data::function::IntFunctionId(20), data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                }, vec![CallCapture::int_function(data::graph::IntFunctionLocalId(0), int_function0.clone())]);
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction3Point3 { int_function0, int0, int_function1 }); }
                                *budget -= 1;
                                let int1 = int0 - 1_i128;
                                if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(3)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 2,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 2,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], int_functions: vec![int_function0, int_function1], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction3Point4 { int_function0, int0, int_function1, int1 }); }
                                *budget -= 1;
                                active = {
                                    IntFunction3State::Point0 { int_function0: int_function1.clone(), int0: int1 }
                                };
                                continue;
                            },
                            IntFunction3State::Point3 { int_function0, int0, int_function1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction3Point3 { int_function0, int0, int_function1 }); }
                                *budget -= 1;
                                let int1 = int0 - 1_i128;
                                if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(3)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 2,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 2,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], int_functions: vec![int_function0, int_function1], ..CallValues::default() }) }; }
                                active = IntFunction3State::Point4 { int_function0, int0, int_function1, int1 };
                                continue;
                            },
                            IntFunction3State::Point4 { int_function0, int0, int_function1, int1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction3Point4 { int_function0, int0, int_function1, int1 }); }
                                *budget -= 1;
                                active = {
                                    IntFunction3State::Point0 { int_function0: int_function1.clone(), int0: int1 }
                                };
                                continue;
                            },
                        }
                    }
                }
                fn calls_entry_0(target: data::function::IntFunctionId, captures: &CallCaptureInputs<'_>, inputs: (i128,)) -> Option<FunctionState> {
                    let (argument0,) = inputs;
                    match target.0 {
                        2 => Some(FunctionState::Int2Point0 { int0: argument0 }),
                        4 => Some(FunctionState::Int4Point0 { int0: argument0 }),
                        9 => Some(FunctionState::Int9Point0 { int0: argument0 }),
                        10 => Some(FunctionState::Int10Point0 { int0: argument0, int1: captures.int(data::graph::IntLocalId(1))? }),
                        11 => Some(FunctionState::Int11Point0 { int0: argument0, int_function0: captures.int_function(data::graph::IntFunctionLocalId(0))?, int_function1: captures.int_function(data::graph::IntFunctionLocalId(1))? }),
                        13 => Some(FunctionState::Int13Point0 { int0: argument0 }),
                        15 => Some(FunctionState::Int15Point0 { int0: argument0, int1: captures.int(data::graph::IntLocalId(1))? }),
                        17 => Some(FunctionState::Int17Point0 { int0: argument0 }),
                        18 => Some(FunctionState::Int18Point0 { int0: argument0 }),
                        19 => Some(FunctionState::Int19Point0 { int0: argument0, int1: captures.int(data::graph::IntLocalId(1))? }),
                        20 => Some(FunctionState::Int20Point0 { int0: argument0, int_function0: captures.int_function(data::graph::IntFunctionLocalId(0))? }),
                        _ => None,
                    }
                }
                fn calls_int_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int0Point0 { int0: values.int(0)?, int1: values.int(1)? },
                        1 => {
                            if !matches!(values.int_function_target(0)?, data::function::IntFunctionId(2) | data::function::IntFunctionId(4) | data::function::IntFunctionId(9) | data::function::IntFunctionId(10) | data::function::IntFunctionId(11) | data::function::IntFunctionId(13) | data::function::IntFunctionId(15) | data::function::IntFunctionId(17) | data::function::IntFunctionId(18) | data::function::IntFunctionId(19) | data::function::IntFunctionId(20)) { return None; }
                            FunctionState::Int0Point1 { int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)? }
                        },
                        2 => FunctionState::Int0Point2 { int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)?, int2: values.int(2)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point, values) { return Some(execution); }
                    let active = calls_int_0_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_1_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int1Point0 { bool0: values.bool(0)?, int0: values.int(0)? },
                        1 => {
                            if !matches!(values.int_function_target(0)?, data::function::IntFunctionId(2) | data::function::IntFunctionId(4) | data::function::IntFunctionId(9) | data::function::IntFunctionId(10) | data::function::IntFunctionId(11) | data::function::IntFunctionId(13) | data::function::IntFunctionId(15) | data::function::IntFunctionId(17) | data::function::IntFunctionId(18) | data::function::IntFunctionId(19) | data::function::IntFunctionId(20)) { return None; }
                            FunctionState::Int1Point1 { bool0: values.bool(0)?, int0: values.int(0)?, int_function0: values.int_function(0)? }
                        },
                        2 => FunctionState::Int1Point2 { bool0: values.bool(0)?, int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)? },
                        3 => FunctionState::Int1Point3 { bool0: values.bool(0)?, int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_1_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(1)), point, values) { return Some(execution); }
                    let active = calls_int_1_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_6_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int6Point0 { int0: values.int(0)?, int1: values.int(1)? },
                        1 => {
                            if !matches!(values.int_function_target(0)?, data::function::IntFunctionId(2) | data::function::IntFunctionId(4) | data::function::IntFunctionId(9) | data::function::IntFunctionId(10) | data::function::IntFunctionId(11) | data::function::IntFunctionId(13) | data::function::IntFunctionId(15) | data::function::IntFunctionId(17) | data::function::IntFunctionId(18) | data::function::IntFunctionId(19) | data::function::IntFunctionId(20)) { return None; }
                            FunctionState::Int6Point1 { int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)? }
                        },
                        2 => FunctionState::Int6Point2 { int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)?, int2: values.int(2)? },
                        3 => FunctionState::Int6Point3 { int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)?, int2: values.int(2)?, int3: values.int(3)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_6_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(6)), point, values) { return Some(execution); }
                    let active = calls_int_6_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                [calls_int_0_start, calls_int_1_start, calls_int_6_start]
            };
            const CALL_GROUP_1: [data::compiled::calls::CallStart; 15] = {
                use data::compiled::calls::{CallArguments, CallCapture, CallCaptureInputs, CallExecution, CallInputs, CallInteger, CallOps, CallOutput, CallProgress, CallStorage, CallValues, IntCallable, StringValue};
                enum FunctionState {
                    Int2Point0 { int0: i128 },
                    Int2Point1 { int0: i128, int1: i128 },
                    Int2Point2 { int0: i128, int1: i128, int2: i128 },
                    Int4Point0 { int0: i128 },
                    Int4Point1 { int0: i128, int1: i128 },
                    Int4Point2 { int0: i128, int1: i128, int2: i128 },
                    Int7Point0 { int0: i128, int1: i128 },
                    Int7Point1 { int0: i128, int1: i128, int_function0: IntCallable },
                    Int7Point2 { int0: i128, int1: i128, int_function0: IntCallable, int2: i128 },
                    Int7Point3 { int0: i128, int1: i128, int_function0: IntCallable, int2: i128, int3: i128 },
                    Int8Point0 { int0: i128, int1: i128, int2: i128 },
                    Int8Point1 { int0: i128, int1: i128, int2: i128, string0: StringValue },
                    Int9Point0 { int0: i128 },
                    Int9Point1 {  },
                    Int9Point2 { int0: i128 },
                    Int9Point3 { int0: i128 },
                    Int9Point4 { int0: i128, int1: i128 },
                    Int9Point5 { int0: i128, int1: i128, int2: i128 },
                    Int9Point6 { int0: i128, int1: i128, int2: i128, int3: i128 },
                    Int10Point0 { int0: i128, int1: i128 },
                    Int10Point1 { int0: i128, int1: i128, int2: i128 },
                    Int11Point0 { int0: i128, int_function0: IntCallable, int_function1: IntCallable },
                    Int11Point1 { int0: i128, int_function0: IntCallable, int_function1: IntCallable, int1: i128 },
                    Int11Point2 { int0: i128, int_function0: IntCallable, int_function1: IntCallable, int1: i128, int2: i128 },
                    Int11Point3 { int0: i128, int_function0: IntCallable, int_function1: IntCallable, int1: i128, int2: i128, int3: i128 },
                    Int13Point0 { int0: i128 },
                    Int13Point1 { int0: i128, int1: i128 },
                    Int15Point0 { int0: i128, int1: i128 },
                    Int15Point1 { int0: i128, int1: i128, int2: i128 },
                    Int16Point0 { int0: i128, int1: i128, int2: i128, string0: StringValue },
                    Int16Point1 { int0: i128 },
                    Int16Point2 { int0: i128, int1: i128, int2: i128, string0: StringValue },
                    Int16Point3 { int0: i128, int1: i128, int2: i128, string0: StringValue, int3: i128 },
                    Int16Point4 { int0: i128, int1: i128, int2: i128, string0: StringValue, int3: i128, int4: i128 },
                    Int16Point5 { int0: i128, int1: i128, int2: i128, string0: StringValue, int3: i128, int4: i128, int5: i128 },
                    Int17Point0 { int0: i128 },
                    Int18Point0 { int0: i128 },
                    Int18Point1 { int0: i128, int1: i128 },
                    Int19Point0 { int0: i128, int1: i128 },
                    Int19Point1 { int0: i128, int1: i128, int2: i128 },
                    Int20Point0 { int0: i128, int_function0: IntCallable },
                    Int20Point1 { int0: i128, int_function0: IntCallable, int1: i128 },
                    Int20Point2 { int0: i128, int_function0: IntCallable, int1: i128, int2: i128 },
                    Bool2Point0 { int0: i128, int1: i128 },
                    Bool2Point1 { int0: i128, int1: i128, int_function0: IntCallable },
                    Bool2Point2 { int0: i128, int1: i128, int_function0: IntCallable, int_function1: IntCallable },
                    Bool2Point3 { int0: i128, int1: i128, int_function0: IntCallable, int_function1: IntCallable, int2: i128 },
                    Bool2Point4 { int0: i128, int1: i128, int_function0: IntCallable, int_function1: IntCallable, int2: i128, int3: i128 },
                    Bool2Point5 { int0: i128, int1: i128, int_function0: IntCallable, int_function1: IntCallable, int2: i128, int3: i128, bool0: bool },
                    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                }
                enum IntReturn {
                    Int2Call0 { int0: i128 },
                    Int4Call0 { int0: i128 },
                    Int7Call1 { int0: i128, int1: i128, int_function0: IntCallable },
                    Int9Call4 { int0: i128, int1: i128 },
                    Int11Call0 { int0: i128, int_function0: IntCallable, int_function1: IntCallable },
                    Int11Call1 { int0: i128, int_function0: IntCallable, int_function1: IntCallable, int1: i128 },
                    Int16Call3 { int0: i128, int1: i128, int2: i128, string0: StringValue, int3: i128 },
                    Int20Call0 { int0: i128, int_function0: IntCallable },
                    Bool2Call2 { int0: i128, int1: i128, int_function0: IntCallable, int_function1: IntCallable },
                }
                impl IntReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                            Self::Int2Call0 { .. } => data::source::HostCallSite::from_static("example", "nested", data::source::SourceSpan::new(818, 833)),
                            Self::Int4Call0 { .. } => data::source::HostCallSite::from_static("example", "big_return", data::source::SourceSpan::new(1814, 1829)),
                            Self::Int7Call1 { .. } => data::source::HostCallSite::from_static("example", "reuse_callback", data::source::SourceSpan::new(2719, 2729)),
                            Self::Int9Call4 { .. } => data::source::HostCallSite::from_static("example", "non_tail", data::source::SourceSpan::new(750, 769)),
                            Self::Int11Call0 { .. } => data::source::HostCallSite::from_static("example", "<anonymous:3>", data::source::SourceSpan::new(1518, 1530)),
                            Self::Int11Call1 { .. } => data::source::HostCallSite::from_static("example", "<anonymous:3>", data::source::SourceSpan::new(1533, 1543)),
                            Self::Int16Call3 { .. } => data::source::HostCallSite::from_static("example", "repeat_canonical", data::source::SourceSpan::new(2944, 2973)),
                            Self::Int20Call0 { .. } => data::source::HostCallSite::from_static("example", "<anonymous:0>", data::source::SourceSpan::new(183, 198)),
                            Self::Bool2Call2 { .. } => data::source::HostCallSite::from_static("example", "aliases", data::source::SourceSpan::new(1548, 1563)),
                        }
                    }
                    fn small(self, result: i128) -> FunctionState {
                        match self {
                            Self::Int2Call0 { int0 } => {
                                let int1 = result;
                                FunctionState::Int2Point1 { int0, int1 }
                            },
                            Self::Int4Call0 { int0 } => {
                                let int1 = result;
                                FunctionState::Int4Point1 { int0, int1 }
                            },
                            Self::Int7Call1 { int0, int1, int_function0 } => {
                                let int2 = result;
                                FunctionState::Int7Point2 { int0, int1, int_function0, int2 }
                            },
                            Self::Int9Call4 { int0, int1 } => {
                                let int2 = result;
                                FunctionState::Int9Point5 { int0, int1, int2 }
                            },
                            Self::Int11Call0 { int0, int_function0, int_function1 } => {
                                let int1 = result;
                                FunctionState::Int11Point1 { int0, int_function0, int_function1, int1 }
                            },
                            Self::Int11Call1 { int0, int_function0, int_function1, int1 } => {
                                let int2 = result;
                                FunctionState::Int11Point2 { int0, int_function0, int_function1, int1, int2 }
                            },
                            Self::Int16Call3 { int0, int1, int2, string0, int3 } => {
                                let int4 = result;
                                FunctionState::Int16Point4 { int0, int1, int2, string0, int3, int4 }
                            },
                            Self::Int20Call0 { int0, int_function0 } => {
                                let int1 = result;
                                FunctionState::Int20Point1 { int0, int_function0, int1 }
                            },
                            Self::Bool2Call2 { int0, int1, int_function0, int_function1 } => {
                                let int2 = result;
                                FunctionState::Bool2Point3 { int0, int1, int_function0, int_function1, int2 }
                            },
                        }
                    }
                    fn resume(self, result: CallInteger) -> FunctionState {
                        if let Some(result) = result.small() {
                            return self.small(result);
                        }
                        match self {
                            Self::Int2Call0 { int0 } => {
                                let int1 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: data::compiled::CompiledCheckpoint {
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
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1], ..CallValues::default() }) }
                            },
                            Self::Int4Call0 { int0 } => {
                                let int1 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(4)), point: data::compiled::CompiledCheckpoint {
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
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1], ..CallValues::default() }) }
                            },
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
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2], int_functions: vec![int_function0], ..CallValues::default() }) }
                            },
                            Self::Int9Call4 { int0, int1 } => {
                                let int2 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(9)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 2,
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
                            Self::Int11Call0 { int0, int_function0, int_function1 } => {
                                let int1 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 1,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 2,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1], int_functions: vec![int_function0, int_function1], ..CallValues::default() }) }
                            },
                            Self::Int11Call1 { int0, int_function0, int_function1, int1 } => {
                                let int2 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 2,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 2,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2], int_functions: vec![int_function0, int_function1], ..CallValues::default() }) }
                            },
                            Self::Int16Call3 { int0, int1, int2, string0, int3 } => {
                                let int4 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(16)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 2,
                                    ints: 5,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 1,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4], strings: vec![string0], ..CallValues::default() }) }
                            },
                            Self::Int20Call0 { int0, int_function0 } => {
                                let int1 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(20)), point: data::compiled::CompiledCheckpoint {
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
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1], int_functions: vec![int_function0], ..CallValues::default() }) }
                            },
                            Self::Bool2Call2 { int0, int1, int_function0, int_function1 } => {
                                let int2 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(2)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 3,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 2,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2], int_functions: vec![int_function0, int_function1], ..CallValues::default() }) }
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
                #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                enum FunctionStep {
                    Yield(FunctionState),
                    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                    IntCall { callee: FunctionState, caller: IntReturn },
                    IntTail { callee: FunctionState },
                    Int { value: i128 },
                    IntBridge { function: data::function::IntFunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: IntReturn },
                    Bool { value: bool },
                }
                struct FunctionExecution {
                    active: Option<FunctionState>,
                    pending_entry: bool,
                    integer_returns: Vec<IntReturn>,
                    boolean_returns: Vec<BoolReturn>,
                }
                impl FunctionExecution {
                    fn new(active: FunctionState) -> Self {
                        Self {
                            active: Some(active),
                            pending_entry: false,
                            integer_returns: Vec::new(),
                            boolean_returns: Vec::new(),
                        }
                    }
                }
                impl CallExecution for FunctionExecution {
                    fn restart(&mut self, target: data::compiled::CallTarget, point: usize, values: CallInputs<'_>) -> bool {
                        if self.active.is_some() { return false; }
                        let active = match target {
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(2)) => calls_int_2_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(4)) => calls_int_4_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(7)) => calls_int_7_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(8)) => calls_int_8_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(9)) => calls_int_9_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(10)) => calls_int_10_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(11)) => calls_int_11_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(13)) => calls_int_13_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(15)) => calls_int_15_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(16)) => calls_int_16_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(17)) => calls_int_17_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(18)) => calls_int_18_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(19)) => calls_int_19_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(20)) => calls_int_20_state(point, values),
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(2)) => calls_bool_2_state(point, values),
                            _ => None,
                        };
                        let Some(active) = active else { return false; };
                        self.active = Some(active);
                        true
                    }
                    fn retained_bytes(&self) -> usize {
                        std::mem::size_of::<Self>() + self.integer_returns.capacity() * std::mem::size_of::<IntReturn>() + self.boolean_returns.capacity() * std::mem::size_of::<BoolReturn>()
                    }
                    fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                        let Some(mut active) = self.active.take() else { return CallProgress::Yield(self); };
                        loop {
                            if self.pending_entry {
                                if *budget == 0 { self.active = Some(active); return CallProgress::Yield(self); }
                                *budget -= 1;
                                self.pending_entry = false;
                            }
                            match function_step(active, ops, budget) {
                                FunctionStep::Yield(active) => {
                                    self.active = Some(active);
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
                                        self.boolean_returns.clear();
                                        return CallProgress::Complete { output: CallOutput::Int(value.into()), execution: self };
                                    }
                                },
                                FunctionStep::IntBridge { function, site, arguments, caller } => return CallProgress::Int {
                                    function, site, arguments,
                                    resume: Box::new(move |value| {
                                        self.active = Some(caller.resume(value));
                                        self
                                    }),
                                },
                                FunctionStep::Bool { value } => {
                                    if let Some(caller) = self.boolean_returns.pop() {
                                        active = caller.small(value);
                                    } else {
                                        self.integer_returns.clear();
                                        self.boolean_returns.clear();
                                        return CallProgress::Complete { output: CallOutput::Bool(value), execution: self };
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
                                            return CallProgress::Interpreted { target, point, values };
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
                        FunctionState::Canonical { target, point, values } => FunctionStep::Canonical { target, point, values },
                        FunctionState::Int2Point0 { int0 } => calls_int_2_run(Int2State::Point0 { int0 }, ops, budget),
                        FunctionState::Int2Point1 { int0, int1 } => calls_int_2_run(Int2State::Point1 { int0, int1 }, ops, budget),
                        FunctionState::Int2Point2 { int0, int1, int2 } => calls_int_2_run(Int2State::Point2 { int0, int1, int2 }, ops, budget),
                        FunctionState::Int4Point0 { int0 } => calls_int_4_run(Int4State::Point0 { int0 }, ops, budget),
                        FunctionState::Int4Point1 { int0, int1 } => calls_int_4_run(Int4State::Point1 { int0, int1 }, ops, budget),
                        FunctionState::Int4Point2 { int0, int1, int2 } => calls_int_4_run(Int4State::Point2 { int0, int1, int2 }, ops, budget),
                        FunctionState::Int7Point0 { int0, int1 } => calls_int_7_run(Int7State::Point0 { int0, int1 }, ops, budget),
                        FunctionState::Int7Point1 { int0, int1, int_function0 } => calls_int_7_run(Int7State::Point1 { int0, int1, int_function0 }, ops, budget),
                        FunctionState::Int7Point2 { int0, int1, int_function0, int2 } => calls_int_7_run(Int7State::Point2 { int0, int1, int_function0, int2 }, ops, budget),
                        FunctionState::Int7Point3 { int0, int1, int_function0, int2, int3 } => calls_int_7_run(Int7State::Point3 { int0, int1, int_function0, int2, int3 }, ops, budget),
                        FunctionState::Int8Point0 { int0, int1, int2 } => calls_int_8_run(Int8State::Point0 { int0, int1, int2 }, ops, budget),
                        FunctionState::Int8Point1 { int0, int1, int2, string0 } => calls_int_8_run(Int8State::Point1 { int0, int1, int2, string0 }, ops, budget),
                        FunctionState::Int9Point0 { int0 } => calls_int_9_run(Int9State::Point0 { int0 }, ops, budget),
                        FunctionState::Int9Point1 {  } => calls_int_9_run(Int9State::Point1 {  }, ops, budget),
                        FunctionState::Int9Point2 { int0 } => calls_int_9_run(Int9State::Point2 { int0 }, ops, budget),
                        FunctionState::Int9Point3 { int0 } => calls_int_9_run(Int9State::Point3 { int0 }, ops, budget),
                        FunctionState::Int9Point4 { int0, int1 } => calls_int_9_run(Int9State::Point4 { int0, int1 }, ops, budget),
                        FunctionState::Int9Point5 { int0, int1, int2 } => calls_int_9_run(Int9State::Point5 { int0, int1, int2 }, ops, budget),
                        FunctionState::Int9Point6 { int0, int1, int2, int3 } => calls_int_9_run(Int9State::Point6 { int0, int1, int2, int3 }, ops, budget),
                        FunctionState::Int10Point0 { int0, int1 } => calls_int_10_run(Int10State::Point0 { int0, int1 }, ops, budget),
                        FunctionState::Int10Point1 { int0, int1, int2 } => calls_int_10_run(Int10State::Point1 { int0, int1, int2 }, ops, budget),
                        FunctionState::Int11Point0 { int0, int_function0, int_function1 } => calls_int_11_run(Int11State::Point0 { int0, int_function0, int_function1 }, ops, budget),
                        FunctionState::Int11Point1 { int0, int_function0, int_function1, int1 } => calls_int_11_run(Int11State::Point1 { int0, int_function0, int_function1, int1 }, ops, budget),
                        FunctionState::Int11Point2 { int0, int_function0, int_function1, int1, int2 } => calls_int_11_run(Int11State::Point2 { int0, int_function0, int_function1, int1, int2 }, ops, budget),
                        FunctionState::Int11Point3 { int0, int_function0, int_function1, int1, int2, int3 } => calls_int_11_run(Int11State::Point3 { int0, int_function0, int_function1, int1, int2, int3 }, ops, budget),
                        FunctionState::Int13Point0 { int0 } => calls_int_13_run(Int13State::Point0 { int0 }, ops, budget),
                        FunctionState::Int13Point1 { int0, int1 } => calls_int_13_run(Int13State::Point1 { int0, int1 }, ops, budget),
                        FunctionState::Int15Point0 { int0, int1 } => calls_int_15_run(Int15State::Point0 { int0, int1 }, ops, budget),
                        FunctionState::Int15Point1 { int0, int1, int2 } => calls_int_15_run(Int15State::Point1 { int0, int1, int2 }, ops, budget),
                        FunctionState::Int16Point0 { int0, int1, int2, string0 } => calls_int_16_run(Int16State::Point0 { int0, int1, int2, string0 }, ops, budget),
                        FunctionState::Int16Point1 { int0 } => calls_int_16_run(Int16State::Point1 { int0 }, ops, budget),
                        FunctionState::Int16Point2 { int0, int1, int2, string0 } => calls_int_16_run(Int16State::Point2 { int0, int1, int2, string0 }, ops, budget),
                        FunctionState::Int16Point3 { int0, int1, int2, string0, int3 } => calls_int_16_run(Int16State::Point3 { int0, int1, int2, string0, int3 }, ops, budget),
                        FunctionState::Int16Point4 { int0, int1, int2, string0, int3, int4 } => calls_int_16_run(Int16State::Point4 { int0, int1, int2, string0, int3, int4 }, ops, budget),
                        FunctionState::Int16Point5 { int0, int1, int2, string0, int3, int4, int5 } => calls_int_16_run(Int16State::Point5 { int0, int1, int2, string0, int3, int4, int5 }, ops, budget),
                        FunctionState::Int17Point0 { int0 } => calls_int_17_run(Int17State::Point0 { int0 }, ops, budget),
                        FunctionState::Int18Point0 { int0 } => calls_int_18_run(Int18State::Point0 { int0 }, ops, budget),
                        FunctionState::Int18Point1 { int0, int1 } => calls_int_18_run(Int18State::Point1 { int0, int1 }, ops, budget),
                        FunctionState::Int19Point0 { int0, int1 } => calls_int_19_run(Int19State::Point0 { int0, int1 }, ops, budget),
                        FunctionState::Int19Point1 { int0, int1, int2 } => calls_int_19_run(Int19State::Point1 { int0, int1, int2 }, ops, budget),
                        FunctionState::Int20Point0 { int0, int_function0 } => calls_int_20_run(Int20State::Point0 { int0, int_function0 }, ops, budget),
                        FunctionState::Int20Point1 { int0, int_function0, int1 } => calls_int_20_run(Int20State::Point1 { int0, int_function0, int1 }, ops, budget),
                        FunctionState::Int20Point2 { int0, int_function0, int1, int2 } => calls_int_20_run(Int20State::Point2 { int0, int_function0, int1, int2 }, ops, budget),
                        FunctionState::Bool2Point0 { int0, int1 } => calls_bool_2_run(Bool2State::Point0 { int0, int1 }, ops, budget),
                        FunctionState::Bool2Point1 { int0, int1, int_function0 } => calls_bool_2_run(Bool2State::Point1 { int0, int1, int_function0 }, ops, budget),
                        FunctionState::Bool2Point2 { int0, int1, int_function0, int_function1 } => calls_bool_2_run(Bool2State::Point2 { int0, int1, int_function0, int_function1 }, ops, budget),
                        FunctionState::Bool2Point3 { int0, int1, int_function0, int_function1, int2 } => calls_bool_2_run(Bool2State::Point3 { int0, int1, int_function0, int_function1, int2 }, ops, budget),
                        FunctionState::Bool2Point4 { int0, int1, int_function0, int_function1, int2, int3 } => calls_bool_2_run(Bool2State::Point4 { int0, int1, int_function0, int_function1, int2, int3 }, ops, budget),
                        FunctionState::Bool2Point5 { int0, int1, int_function0, int_function1, int2, int3, bool0 } => calls_bool_2_run(Bool2State::Point5 { int0, int1, int_function0, int_function1, int2, int3, bool0 }, ops, budget),
                    }
                }
                enum Int2State {
                    Point0 { int0: i128 },
                    Point1 { int0: i128, int1: i128 },
                    Point2 { int0: i128, int1: i128, int2: i128 },
                }
                fn calls_int_2_run(active: Int2State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int2State::Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point0 { int0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::IntCall { callee: FunctionState::Int9Point0 { int0 }, caller: IntReturn::Int2Call0 { int0 } }
                            }
                        },
                        Int2State::Point1 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point1 { int0, int1 }); }
                            *budget -= 1;
                            let int2 = int1 + 2_i128;
                            if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 2,
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
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point2 { int0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                        Int2State::Point2 { int0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point2 { int0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                    }
                }
                enum Int4State {
                    Point0 { int0: i128 },
                    Point1 { int0: i128, int1: i128 },
                    Point2 { int0: i128, int1: i128, int2: i128 },
                }
                fn calls_int_4_run(active: Int4State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int4State::Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int4Point0 { int0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::IntCall { callee: FunctionState::Int13Point0 { int0 }, caller: IntReturn::Int4Call0 { int0 } }
                            }
                        },
                        Int4State::Point1 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int4Point1 { int0, int1 }); }
                            *budget -= 1;
                            let int2 = int1 + 3_i128;
                            if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(4)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 2,
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
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int4Point2 { int0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                        Int4State::Point2 { int0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int4Point2 { int0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                    }
                }
                enum Int7State {
                    Point0 { int0: i128, int1: i128 },
                    Point1 { int0: i128, int1: i128, int_function0: IntCallable },
                    Point2 { int0: i128, int1: i128, int_function0: IntCallable, int2: i128 },
                    Point3 { int0: i128, int1: i128, int_function0: IntCallable, int2: i128, int3: i128 },
                }
                fn calls_int_7_run(active: Int7State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int7State::Point0 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int7Point0 { int0, int1 }); }
                            *budget -= 1;
                            let int_function0 = ops.int_closure(data::function::IntFunctionId(15), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            }, vec![CallCapture::int(data::graph::IntLocalId(1), int1)]);
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int7Point1 { int0, int1, int_function0 }); }
                            *budget -= 1;
                            {
                                let callable = &int_function0;
                                let captures = callable.captures();
                                let target = callable.target();
                                if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (int0,)) {
                                    return FunctionStep::IntCall { callee, caller: IntReturn::Int7Call1 { int0, int1, int_function0 } };
                                }
                                FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("example", "reuse_callback", data::source::SourceSpan::new(2719, 2729)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int0.into()], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: IntReturn::Int7Call1 { int0, int1, int_function0 } }
                            }
                        },
                        Int7State::Point1 { int0, int1, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int7Point1 { int0, int1, int_function0 }); }
                            *budget -= 1;
                            {
                                let callable = &int_function0;
                                let captures = callable.captures();
                                let target = callable.target();
                                if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (int0,)) {
                                    return FunctionStep::IntCall { callee, caller: IntReturn::Int7Call1 { int0, int1, int_function0 } };
                                }
                                FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("example", "reuse_callback", data::source::SourceSpan::new(2719, 2729)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int0.into()], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: IntReturn::Int7Call1 { int0, int1, int_function0 } }
                            }
                        },
                        Int7State::Point2 { int0, int1, int_function0, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int7Point2 { int0, int1, int_function0, int2 }); }
                            *budget -= 1;
                            let int3 = int2 + 1_i128;
                            if int3 < i128::from(i64::MIN) || int3 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(7)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 3,
                                ints: 4,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 1,
                                bool_functions: 0,
                            }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into()], int_functions: vec![int_function0], ..CallValues::default() }) }; }
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int7Point3 { int0, int1, int_function0, int2, int3 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int3 }
                            }
                        },
                        Int7State::Point3 { int0, int1, int_function0, int2, int3 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int7Point3 { int0, int1, int_function0, int2, int3 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int3 }
                            }
                        },
                    }
                }
                enum Int8State {
                    Point0 { int0: i128, int1: i128, int2: i128 },
                    Point1 { int0: i128, int1: i128, int2: i128, string0: StringValue },
                }
                fn calls_int_8_run(active: Int8State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int8State::Point0 { int0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point0 { int0, int1, int2 }); }
                            *budget -= 1;
                            let string0 = StringValue::from("canonical caller");
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point1 { int0, int1, int2, string0 }); }
                            {
                                FunctionStep::IntTail { callee: FunctionState::Int16Point0 { int0, int1, int2, string0: string0.clone() } }
                            }
                        },
                        Int8State::Point1 { int0, int1, int2, string0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point1 { int0, int1, int2, string0 }); }
                            {
                                FunctionStep::IntTail { callee: FunctionState::Int16Point0 { int0, int1, int2, string0: string0.clone() } }
                            }
                        },
                    }
                }
                enum Int9State {
                    Point0 { int0: i128 },
                    Point1 {  },
                    Point2 { int0: i128 },
                    Point3 { int0: i128 },
                    Point4 { int0: i128, int1: i128 },
                    Point5 { int0: i128, int1: i128, int2: i128 },
                    Point6 { int0: i128, int1: i128, int2: i128, int3: i128 },
                }
                fn calls_int_9_run(mut active: Int9State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            Int9State::Point0 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point0 { int0 }); }
                                *budget -= 1;
                                active = {
                                    if int0 <= 0_i128 { Int9State::Point1 {  } } else { Int9State::Point3 { int0 } }
                                };
                                continue;
                            },
                            Int9State::Point1 {  } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point1 {  }); }
                                *budget -= 1;
                                let int0 = 0_i128;
                                if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(9)), point: data::compiled::CompiledCheckpoint {
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
                                }, values: Box::new(CallValues { ints: vec![int0.into()], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point2 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int9State::Point2 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point2 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int9State::Point3 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point3 { int0 }); }
                                *budget -= 1;
                                let int1 = int0 - 1_i128;
                                if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(9)), point: data::compiled::CompiledCheckpoint {
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
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point4 { int0, int1 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntCall { callee: FunctionState::Int9Point0 { int0: int1 }, caller: IntReturn::Int9Call4 { int0, int1 } }
                                };
                            },
                            Int9State::Point4 { int0, int1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point4 { int0, int1 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntCall { callee: FunctionState::Int9Point0 { int0: int1 }, caller: IntReturn::Int9Call4 { int0, int1 } }
                                };
                            },
                            Int9State::Point5 { int0, int1, int2 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point5 { int0, int1, int2 }); }
                                *budget -= 1;
                                let int3 = int2 + 1_i128;
                                if int3 < i128::from(i64::MIN) || int3 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(9)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 3,
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
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point6 { int0, int1, int2, int3 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int3 }
                                };
                            },
                            Int9State::Point6 { int0, int1, int2, int3 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point6 { int0, int1, int2, int3 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int3 }
                                };
                            },
                        }
                    }
                }
                enum Int10State {
                    Point0 { int0: i128, int1: i128 },
                    Point1 { int0: i128, int1: i128, int2: i128 },
                }
                fn calls_int_10_run(active: Int10State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int10State::Point0 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int10Point0 { int0, int1 }); }
                            *budget -= 1;
                            let int2 = int0 + int1;
                            if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(10)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
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
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int10Point1 { int0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                        Int10State::Point1 { int0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int10Point1 { int0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                    }
                }
                enum Int11State {
                    Point0 { int0: i128, int_function0: IntCallable, int_function1: IntCallable },
                    Point1 { int0: i128, int_function0: IntCallable, int_function1: IntCallable, int1: i128 },
                    Point2 { int0: i128, int_function0: IntCallable, int_function1: IntCallable, int1: i128, int2: i128 },
                    Point3 { int0: i128, int_function0: IntCallable, int_function1: IntCallable, int1: i128, int2: i128, int3: i128 },
                }
                fn calls_int_11_run(active: Int11State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int11State::Point0 { int0, int_function0, int_function1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point0 { int0, int_function0, int_function1 }); }
                            *budget -= 1;
                            {
                                let callable = &int_function0;
                                let captures = callable.captures();
                                let target = callable.target();
                                if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (int0,)) {
                                    return FunctionStep::IntCall { callee, caller: IntReturn::Int11Call0 { int0, int_function0, int_function1 } };
                                }
                                FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("example", "<anonymous:3>", data::source::SourceSpan::new(1518, 1530)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int0.into()], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: IntReturn::Int11Call0 { int0, int_function0, int_function1 } }
                            }
                        },
                        Int11State::Point1 { int0, int_function0, int_function1, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point1 { int0, int_function0, int_function1, int1 }); }
                            *budget -= 1;
                            {
                                let callable = &int_function1;
                                let captures = callable.captures();
                                let target = callable.target();
                                if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (int0,)) {
                                    return FunctionStep::IntCall { callee, caller: IntReturn::Int11Call1 { int0, int_function0, int_function1, int1 } };
                                }
                                FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("example", "<anonymous:3>", data::source::SourceSpan::new(1533, 1543)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int0.into()], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: IntReturn::Int11Call1 { int0, int_function0, int_function1, int1 } }
                            }
                        },
                        Int11State::Point2 { int0, int_function0, int_function1, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point2 { int0, int_function0, int_function1, int1, int2 }); }
                            *budget -= 1;
                            let int3 = int1 + int2;
                            if int3 < i128::from(i64::MIN) || int3 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 3,
                                ints: 4,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 2,
                                bool_functions: 0,
                            }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into()], int_functions: vec![int_function0, int_function1], ..CallValues::default() }) }; }
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point3 { int0, int_function0, int_function1, int1, int2, int3 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int3 }
                            }
                        },
                        Int11State::Point3 { int0, int_function0, int_function1, int1, int2, int3 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point3 { int0, int_function0, int_function1, int1, int2, int3 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int3 }
                            }
                        },
                    }
                }
                enum Int13State {
                    Point0 { int0: i128 },
                    Point1 { int0: i128, int1: i128 },
                }
                fn calls_int_13_run(active: Int13State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int13State::Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point0 { int0 }); }
                            *budget -= 1;
                            let int1 = int0 * int0;
                            if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(13)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], ..CallValues::default() }) }; }
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point1 { int0, int1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int1 }
                            }
                        },
                        Int13State::Point1 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point1 { int0, int1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int1 }
                            }
                        },
                    }
                }
                enum Int15State {
                    Point0 { int0: i128, int1: i128 },
                    Point1 { int0: i128, int1: i128, int2: i128 },
                }
                fn calls_int_15_run(active: Int15State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int15State::Point0 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int15Point0 { int0, int1 }); }
                            *budget -= 1;
                            let int2 = int0 + int1;
                            if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(15)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
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
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int15Point1 { int0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                        Int15State::Point1 { int0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int15Point1 { int0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                    }
                }
                enum Int16State {
                    Point0 { int0: i128, int1: i128, int2: i128, string0: StringValue },
                    Point1 { int0: i128 },
                    Point2 { int0: i128, int1: i128, int2: i128, string0: StringValue },
                    Point3 { int0: i128, int1: i128, int2: i128, string0: StringValue, int3: i128 },
                    Point4 { int0: i128, int1: i128, int2: i128, string0: StringValue, int3: i128, int4: i128 },
                    Point5 { int0: i128, int1: i128, int2: i128, string0: StringValue, int3: i128, int4: i128, int5: i128 },
                }
                fn calls_int_16_run(mut active: Int16State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            Int16State::Point0 { int0, int1, int2, string0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point0 { int0, int1, int2, string0 }); }
                                *budget -= 1;
                                active = {
                                    if int0 <= 0_i128 { Int16State::Point1 { int0: int1 } } else { Int16State::Point2 { int0, int1, int2, string0: string0.clone() } }
                                };
                                continue;
                            },
                            Int16State::Point1 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point1 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int16State::Point2 { int0, int1, int2, string0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point2 { int0, int1, int2, string0 }); }
                                *budget -= 1;
                                let int3 = int0 - 1_i128;
                                if int3 < i128::from(i64::MIN) || int3 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(16)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 1,
                                    ints: 4,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 1,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into()], strings: vec![string0], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point3 { int0, int1, int2, string0, int3 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntCall { callee: FunctionState::Int7Point0 { int0: int1, int1: int2 }, caller: IntReturn::Int16Call3 { int0, int1, int2, string0, int3 } }
                                };
                            },
                            Int16State::Point3 { int0, int1, int2, string0, int3 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point3 { int0, int1, int2, string0, int3 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntCall { callee: FunctionState::Int7Point0 { int0: int1, int1: int2 }, caller: IntReturn::Int16Call3 { int0, int1, int2, string0, int3 } }
                                };
                            },
                            Int16State::Point4 { int0, int1, int2, string0, int3, int4 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point4 { int0, int1, int2, string0, int3, int4 }); }
                                *budget -= 1;
                                let int5 = int2 + 1_i128;
                                if int5 < i128::from(i64::MIN) || int5 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(16)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 3,
                                    ints: 6,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 1,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into()], strings: vec![string0], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point5 { int0, int1, int2, string0, int3, int4, int5 }); }
                                *budget -= 1;
                                active = {
                                    Int16State::Point0 { int0: int3, int1: int4, int2: int5, string0: string0.clone() }
                                };
                                continue;
                            },
                            Int16State::Point5 { int0, int1, int2, string0, int3, int4, int5 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int16Point5 { int0, int1, int2, string0, int3, int4, int5 }); }
                                *budget -= 1;
                                active = {
                                    Int16State::Point0 { int0: int3, int1: int4, int2: int5, string0: string0.clone() }
                                };
                                continue;
                            },
                        }
                    }
                }
                enum Int17State {
                    Point0 { int0: i128 },
                }
                fn calls_int_17_run(active: Int17State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int17State::Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int17Point0 { int0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int0 }
                            }
                        },
                    }
                }
                enum Int18State {
                    Point0 { int0: i128 },
                    Point1 { int0: i128, int1: i128 },
                }
                fn calls_int_18_run(active: Int18State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int18State::Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int18Point0 { int0 }); }
                            *budget -= 1;
                            let int1 = int0 + 7_i128;
                            if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(18)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], ..CallValues::default() }) }; }
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int18Point1 { int0, int1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int1 }
                            }
                        },
                        Int18State::Point1 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int18Point1 { int0, int1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int1 }
                            }
                        },
                    }
                }
                enum Int19State {
                    Point0 { int0: i128, int1: i128 },
                    Point1 { int0: i128, int1: i128, int2: i128 },
                }
                fn calls_int_19_run(active: Int19State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int19State::Point0 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int19Point0 { int0, int1 }); }
                            *budget -= 1;
                            let int2 = int0 + int1;
                            if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(19)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
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
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int19Point1 { int0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                        Int19State::Point1 { int0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int19Point1 { int0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                    }
                }
                enum Int20State {
                    Point0 { int0: i128, int_function0: IntCallable },
                    Point1 { int0: i128, int_function0: IntCallable, int1: i128 },
                    Point2 { int0: i128, int_function0: IntCallable, int1: i128, int2: i128 },
                }
                fn calls_int_20_run(active: Int20State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int20State::Point0 { int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int20Point0 { int0, int_function0 }); }
                            *budget -= 1;
                            {
                                let callable = &int_function0;
                                let captures = callable.captures();
                                let target = callable.target();
                                if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (int0,)) {
                                    return FunctionStep::IntCall { callee, caller: IntReturn::Int20Call0 { int0, int_function0 } };
                                }
                                FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("example", "<anonymous:0>", data::source::SourceSpan::new(183, 198)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int0.into()], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: IntReturn::Int20Call0 { int0, int_function0 } }
                            }
                        },
                        Int20State::Point1 { int0, int_function0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int20Point1 { int0, int_function0, int1 }); }
                            *budget -= 1;
                            let int2 = int1 + 1_i128;
                            if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(20)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into()], int_functions: vec![int_function0], ..CallValues::default() }) }; }
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int20Point2 { int0, int_function0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                        Int20State::Point2 { int0, int_function0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int20Point2 { int0, int_function0, int1, int2 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Int { value: int2 }
                            }
                        },
                    }
                }
                enum Bool2State {
                    Point0 { int0: i128, int1: i128 },
                    Point1 { int0: i128, int1: i128, int_function0: IntCallable },
                    Point2 { int0: i128, int1: i128, int_function0: IntCallable, int_function1: IntCallable },
                    Point3 { int0: i128, int1: i128, int_function0: IntCallable, int_function1: IntCallable, int2: i128 },
                    Point4 { int0: i128, int1: i128, int_function0: IntCallable, int_function1: IntCallable, int2: i128, int3: i128 },
                    Point5 { int0: i128, int1: i128, int_function0: IntCallable, int_function1: IntCallable, int2: i128, int3: i128, bool0: bool },
                }
                fn calls_bool_2_run(mut active: Bool2State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            Bool2State::Point0 { int0, int1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool2Point0 { int0, int1 }); }
                                *budget -= 1;
                                let int_function0 = ops.int_closure(data::function::IntFunctionId(10), data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                }, vec![CallCapture::int(data::graph::IntLocalId(1), int0)]);
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool2Point1 { int0, int1, int_function0 }); }
                                *budget -= 1;
                                let int_function1 = ops.int_closure(data::function::IntFunctionId(11), data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                }, vec![CallCapture::int_function(data::graph::IntFunctionLocalId(0), int_function0.clone()), CallCapture::int_function(data::graph::IntFunctionLocalId(1), int_function0.clone())]);
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool2Point2 { int0, int1, int_function0, int_function1 }); }
                                *budget -= 1;
                                return {
                                    let callable = &int_function1;
                                    let captures = callable.captures();
                                    let target = callable.target();
                                    if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (int1,)) {
                                        return FunctionStep::IntCall { callee, caller: IntReturn::Bool2Call2 { int0, int1, int_function0, int_function1 } };
                                    }
                                    FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("example", "aliases", data::source::SourceSpan::new(1548, 1563)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int1.into()], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: IntReturn::Bool2Call2 { int0, int1, int_function0, int_function1 } }
                                };
                            },
                            Bool2State::Point1 { int0, int1, int_function0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool2Point1 { int0, int1, int_function0 }); }
                                *budget -= 1;
                                let int_function1 = ops.int_closure(data::function::IntFunctionId(11), data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                }, vec![CallCapture::int_function(data::graph::IntFunctionLocalId(0), int_function0.clone()), CallCapture::int_function(data::graph::IntFunctionLocalId(1), int_function0.clone())]);
                                active = Bool2State::Point2 { int0, int1, int_function0, int_function1 };
                                continue;
                            },
                            Bool2State::Point2 { int0, int1, int_function0, int_function1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool2Point2 { int0, int1, int_function0, int_function1 }); }
                                *budget -= 1;
                                return {
                                    let callable = &int_function1;
                                    let captures = callable.captures();
                                    let target = callable.target();
                                    if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (int1,)) {
                                        return FunctionStep::IntCall { callee, caller: IntReturn::Bool2Call2 { int0, int1, int_function0, int_function1 } };
                                    }
                                    FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("example", "aliases", data::source::SourceSpan::new(1548, 1563)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int1.into()], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: IntReturn::Bool2Call2 { int0, int1, int_function0, int_function1 } }
                                };
                            },
                            Bool2State::Point3 { int0, int1, int_function0, int_function1, int2 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool2Point3 { int0, int1, int_function0, int_function1, int2 }); }
                                *budget -= 1;
                                let region0 = int1 + int0;
                                let region1 = 2_i128 * region0;
                                let int3 = region1;
                                if int3 < i128::from(i64::MIN) || int3 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(2)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 4,
                                    ints: 4,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 2,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into()], int_functions: vec![int_function0, int_function1], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool2Point4 { int0, int1, int_function0, int_function1, int2, int3 }); }
                                *budget -= 1;
                                let bool0 = int2 == int3;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool2Point5 { int0, int1, int_function0, int_function1, int2, int3, bool0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Bool { value: bool0 }
                                };
                            },
                            Bool2State::Point4 { int0, int1, int_function0, int_function1, int2, int3 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool2Point4 { int0, int1, int_function0, int_function1, int2, int3 }); }
                                *budget -= 1;
                                let bool0 = int2 == int3;
                                active = Bool2State::Point5 { int0, int1, int_function0, int_function1, int2, int3, bool0 };
                                continue;
                            },
                            Bool2State::Point5 { int0, int1, int_function0, int_function1, int2, int3, bool0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool2Point5 { int0, int1, int_function0, int_function1, int2, int3, bool0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Bool { value: bool0 }
                                };
                            },
                        }
                    }
                }
                fn calls_entry_0(target: data::function::IntFunctionId, captures: &CallCaptureInputs<'_>, inputs: (i128,)) -> Option<FunctionState> {
                    let (argument0,) = inputs;
                    match target.0 {
                        2 => Some(FunctionState::Int2Point0 { int0: argument0 }),
                        4 => Some(FunctionState::Int4Point0 { int0: argument0 }),
                        9 => Some(FunctionState::Int9Point0 { int0: argument0 }),
                        10 => Some(FunctionState::Int10Point0 { int0: argument0, int1: captures.int(data::graph::IntLocalId(1))? }),
                        11 => Some(FunctionState::Int11Point0 { int0: argument0, int_function0: captures.int_function(data::graph::IntFunctionLocalId(0))?, int_function1: captures.int_function(data::graph::IntFunctionLocalId(1))? }),
                        13 => Some(FunctionState::Int13Point0 { int0: argument0 }),
                        15 => Some(FunctionState::Int15Point0 { int0: argument0, int1: captures.int(data::graph::IntLocalId(1))? }),
                        17 => Some(FunctionState::Int17Point0 { int0: argument0 }),
                        18 => Some(FunctionState::Int18Point0 { int0: argument0 }),
                        19 => Some(FunctionState::Int19Point0 { int0: argument0, int1: captures.int(data::graph::IntLocalId(1))? }),
                        20 => Some(FunctionState::Int20Point0 { int0: argument0, int_function0: captures.int_function(data::graph::IntFunctionLocalId(0))? }),
                        _ => None,
                    }
                }
                fn calls_int_2_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int2Point0 { int0: values.int(0)? },
                        1 => FunctionState::Int2Point1 { int0: values.int(0)?, int1: values.int(1)? },
                        2 => FunctionState::Int2Point2 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_2_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point, values) { return Some(execution); }
                    let active = calls_int_2_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_4_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int4Point0 { int0: values.int(0)? },
                        1 => FunctionState::Int4Point1 { int0: values.int(0)?, int1: values.int(1)? },
                        2 => FunctionState::Int4Point2 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_4_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(4)), point, values) { return Some(execution); }
                    let active = calls_int_4_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_7_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int7Point0 { int0: values.int(0)?, int1: values.int(1)? },
                        1 => {
                            if !matches!(values.int_function_target(0)?, data::function::IntFunctionId(2) | data::function::IntFunctionId(4) | data::function::IntFunctionId(9) | data::function::IntFunctionId(10) | data::function::IntFunctionId(11) | data::function::IntFunctionId(13) | data::function::IntFunctionId(15) | data::function::IntFunctionId(17) | data::function::IntFunctionId(18) | data::function::IntFunctionId(19) | data::function::IntFunctionId(20)) { return None; }
                            FunctionState::Int7Point1 { int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)? }
                        },
                        2 => FunctionState::Int7Point2 { int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)?, int2: values.int(2)? },
                        3 => FunctionState::Int7Point3 { int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)?, int2: values.int(2)?, int3: values.int(3)? },
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
                        0 => FunctionState::Int8Point0 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        1 => FunctionState::Int8Point1 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, string0: values.string(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_8_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(8)), point, values) { return Some(execution); }
                    let active = calls_int_8_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_9_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int9Point0 { int0: values.int(0)? },
                        1 => FunctionState::Int9Point1 {  },
                        2 => FunctionState::Int9Point2 { int0: values.int(0)? },
                        3 => FunctionState::Int9Point3 { int0: values.int(0)? },
                        4 => FunctionState::Int9Point4 { int0: values.int(0)?, int1: values.int(1)? },
                        5 => FunctionState::Int9Point5 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        6 => FunctionState::Int9Point6 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_9_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(9)), point, values) { return Some(execution); }
                    let active = calls_int_9_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_10_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int10Point0 { int0: values.int(0)?, int1: values.int(1)? },
                        1 => FunctionState::Int10Point1 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_10_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(10)), point, values) { return Some(execution); }
                    let active = calls_int_10_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_11_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => {
                            if !matches!(values.int_function_target(0)?, data::function::IntFunctionId(2) | data::function::IntFunctionId(4) | data::function::IntFunctionId(9) | data::function::IntFunctionId(10) | data::function::IntFunctionId(11) | data::function::IntFunctionId(13) | data::function::IntFunctionId(15) | data::function::IntFunctionId(17) | data::function::IntFunctionId(18) | data::function::IntFunctionId(19) | data::function::IntFunctionId(20)) { return None; }
                            FunctionState::Int11Point0 { int0: values.int(0)?, int_function0: values.int_function(0)?, int_function1: values.int_function(1)? }
                        },
                        1 => {
                            if !matches!(values.int_function_target(1)?, data::function::IntFunctionId(2) | data::function::IntFunctionId(4) | data::function::IntFunctionId(9) | data::function::IntFunctionId(10) | data::function::IntFunctionId(11) | data::function::IntFunctionId(13) | data::function::IntFunctionId(15) | data::function::IntFunctionId(17) | data::function::IntFunctionId(18) | data::function::IntFunctionId(19) | data::function::IntFunctionId(20)) { return None; }
                            FunctionState::Int11Point1 { int0: values.int(0)?, int_function0: values.int_function(0)?, int_function1: values.int_function(1)?, int1: values.int(1)? }
                        },
                        2 => FunctionState::Int11Point2 { int0: values.int(0)?, int_function0: values.int_function(0)?, int_function1: values.int_function(1)?, int1: values.int(1)?, int2: values.int(2)? },
                        3 => FunctionState::Int11Point3 { int0: values.int(0)?, int_function0: values.int_function(0)?, int_function1: values.int_function(1)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_11_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point, values) { return Some(execution); }
                    let active = calls_int_11_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_13_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int13Point0 { int0: values.int(0)? },
                        1 => FunctionState::Int13Point1 { int0: values.int(0)?, int1: values.int(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_13_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(13)), point, values) { return Some(execution); }
                    let active = calls_int_13_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_15_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int15Point0 { int0: values.int(0)?, int1: values.int(1)? },
                        1 => FunctionState::Int15Point1 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_15_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(15)), point, values) { return Some(execution); }
                    let active = calls_int_15_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_16_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int16Point0 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, string0: values.string(0)? },
                        1 => FunctionState::Int16Point1 { int0: values.int(0)? },
                        2 => FunctionState::Int16Point2 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, string0: values.string(0)? },
                        3 => FunctionState::Int16Point3 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, string0: values.string(0)?, int3: values.int(3)? },
                        4 => FunctionState::Int16Point4 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, string0: values.string(0)?, int3: values.int(3)?, int4: values.int(4)? },
                        5 => FunctionState::Int16Point5 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, string0: values.string(0)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)? },
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
                        0 => FunctionState::Int17Point0 { int0: values.int(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_17_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(17)), point, values) { return Some(execution); }
                    let active = calls_int_17_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_18_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int18Point0 { int0: values.int(0)? },
                        1 => FunctionState::Int18Point1 { int0: values.int(0)?, int1: values.int(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_18_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(18)), point, values) { return Some(execution); }
                    let active = calls_int_18_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_19_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int19Point0 { int0: values.int(0)?, int1: values.int(1)? },
                        1 => FunctionState::Int19Point1 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_19_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(19)), point, values) { return Some(execution); }
                    let active = calls_int_19_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_20_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => {
                            if !matches!(values.int_function_target(0)?, data::function::IntFunctionId(2) | data::function::IntFunctionId(4) | data::function::IntFunctionId(9) | data::function::IntFunctionId(10) | data::function::IntFunctionId(11) | data::function::IntFunctionId(13) | data::function::IntFunctionId(15) | data::function::IntFunctionId(17) | data::function::IntFunctionId(18) | data::function::IntFunctionId(19) | data::function::IntFunctionId(20)) { return None; }
                            FunctionState::Int20Point0 { int0: values.int(0)?, int_function0: values.int_function(0)? }
                        },
                        1 => FunctionState::Int20Point1 { int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)? },
                        2 => FunctionState::Int20Point2 { int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_20_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(20)), point, values) { return Some(execution); }
                    let active = calls_int_20_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_bool_2_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool2Point0 { int0: values.int(0)?, int1: values.int(1)? },
                        1 => FunctionState::Bool2Point1 { int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)? },
                        2 => {
                            if !matches!(values.int_function_target(1)?, data::function::IntFunctionId(2) | data::function::IntFunctionId(4) | data::function::IntFunctionId(9) | data::function::IntFunctionId(10) | data::function::IntFunctionId(11) | data::function::IntFunctionId(13) | data::function::IntFunctionId(15) | data::function::IntFunctionId(17) | data::function::IntFunctionId(18) | data::function::IntFunctionId(19) | data::function::IntFunctionId(20)) { return None; }
                            FunctionState::Bool2Point2 { int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)?, int_function1: values.int_function(1)? }
                        },
                        3 => FunctionState::Bool2Point3 { int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)?, int_function1: values.int_function(1)?, int2: values.int(2)? },
                        4 => FunctionState::Bool2Point4 { int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)?, int_function1: values.int_function(1)?, int2: values.int(2)?, int3: values.int(3)? },
                        5 => FunctionState::Bool2Point5 { int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)?, int_function1: values.int_function(1)?, int2: values.int(2)?, int3: values.int(3)?, bool0: values.bool(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_2_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(2)), point, values) { return Some(execution); }
                    let active = calls_bool_2_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                [calls_int_2_start, calls_int_4_start, calls_int_7_start, calls_int_8_start, calls_int_9_start, calls_int_10_start, calls_int_11_start, calls_int_13_start, calls_int_15_start, calls_int_16_start, calls_int_17_start, calls_int_18_start, calls_int_19_start, calls_int_20_start, calls_bool_2_start]
            };
            const CALL_GROUP_2: [data::compiled::calls::CallStart; 7] = {
                use data::compiled::calls::{BoolCallable, CallArguments, CallCaptureInputs, CallExecution, CallInputs, CallOps, CallOutput, CallProgress, CallStorage, CallValues};
                enum FunctionState {
                    Bool0Point0 { int0: i128 },
                    Bool0Point1 { int0: i128, bool0: bool },
                    Bool0Point2 { int0: i128, bool0: bool, bool1: bool },
                    Bool6Point0 { int0: i128 },
                    Bool6Point1 {  },
                    Bool6Point2 { bool0: bool },
                    Bool6Point3 { int0: i128 },
                    Bool6Point4 { int0: i128, int1: i128 },
                    Bool7Point0 { bool0: bool },
                    Bool7Point1 { bool0: bool, bool1: bool },
                    Bool9Point0 { int0: i128 },
                    Bool9Point1 {  },
                    Bool9Point2 { bool0: bool },
                    Bool9Point3 { int0: i128 },
                    Bool9Point4 { int0: i128, int1: i128 },
                    Bool10Point0 { int0: i128, bool_function0: BoolCallable, int1: i128 },
                    Bool10Point1 { int0: i128, bool_function0: BoolCallable, int1: i128, bool0: bool },
                    Bool10Point2 { int0: i128, bool_function0: BoolCallable, int1: i128, bool0: bool, bool1: bool },
                    Bool11Point0 { int0: i128, int1: i128 },
                    Bool11Point1 { int0: i128, int1: i128, bool0: bool },
                    Bool12Point0 { int0: i128, bool0: bool, int1: i128 },
                    Bool12Point1 { int0: i128, int1: i128 },
                    Bool12Point2 { int0: i128, int1: i128, bool0: bool },
                    Bool12Point3 {  },
                    Bool12Point4 { bool0: bool },
                }
                enum BoolReturn {
                    Bool0Call0 { int0: i128 },
                    Bool10Call1 { int0: i128, bool_function0: BoolCallable, int1: i128, bool0: bool },
                }
                impl BoolReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                            Self::Bool0Call0 { .. } => data::source::HostCallSite::from_static("example", "mutual", data::source::SourceSpan::new(1085, 1096)),
                            Self::Bool10Call1 { .. } => data::source::HostCallSite::from_static("example", "<anonymous:1>", data::source::SourceSpan::new(1232, 1256)),
                        }
                    }
                    fn small(self, result: bool) -> FunctionState {
                        match self {
                            Self::Bool0Call0 { int0 } => {
                                let bool0 = result;
                                FunctionState::Bool0Point1 { int0, bool0 }
                            },
                            Self::Bool10Call1 { int0, bool_function0, int1, bool0 } => {
                                let bool1 = result;
                                FunctionState::Bool10Point2 { int0, bool_function0, int1, bool0, bool1 }
                            },
                        }
                    }
                    fn resume(self, result: bool) -> FunctionState { self.small(result) }
                }
                #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                enum FunctionStep {
                    Yield(FunctionState),
                    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                    BoolCall { callee: FunctionState, caller: BoolReturn },
                    BoolTail { callee: FunctionState },
                    Bool { value: bool },
                    BoolBridge { function: data::function::BoolFunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: BoolReturn },
                }
                struct FunctionExecution {
                    active: Option<FunctionState>,
                    pending_entry: bool,
                    boolean_returns: Vec<BoolReturn>,
                }
                impl FunctionExecution {
                    fn new(active: FunctionState) -> Self {
                        Self {
                            active: Some(active),
                            pending_entry: false,
                            boolean_returns: Vec::new(),
                        }
                    }
                }
                impl CallExecution for FunctionExecution {
                    fn restart(&mut self, target: data::compiled::CallTarget, point: usize, values: CallInputs<'_>) -> bool {
                        if self.active.is_some() { return false; }
                        let active = match target {
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)) => calls_bool_0_state(point, values),
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(6)) => calls_bool_6_state(point, values),
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(7)) => calls_bool_7_state(point, values),
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(9)) => calls_bool_9_state(point, values),
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(10)) => calls_bool_10_state(point, values),
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(11)) => calls_bool_11_state(point, values),
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(12)) => calls_bool_12_state(point, values),
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
                            if self.pending_entry {
                                if *budget == 0 { self.active = Some(active); return CallProgress::Yield(self); }
                                *budget -= 1;
                                self.pending_entry = false;
                            }
                            match function_step(active, ops, budget) {
                                FunctionStep::Yield(active) => {
                                    self.active = Some(active);
                                    return CallProgress::Yield(self);
                                },
                                FunctionStep::BoolCall { callee, caller } => {
                                    self.boolean_returns.push(caller);
                                    active = callee;
                                },
                                FunctionStep::BoolTail { callee } => {
                                    *budget -= 1;
                                    self.pending_entry = self.boolean_returns.is_empty() && ops.root_tail_entry();
                                    active = callee;
                                },
                                FunctionStep::Bool { value } => {
                                    if let Some(caller) = self.boolean_returns.pop() {
                                        active = caller.small(value);
                                    } else {
                                        self.boolean_returns.clear();
                                        return CallProgress::Complete { output: CallOutput::Bool(value), execution: self };
                                    }
                                },
                                FunctionStep::BoolBridge { function, site, arguments, caller } => return CallProgress::Bool {
                                    function, site, arguments,
                                    resume: Box::new(move |value| {
                                        self.active = Some(caller.resume(value));
                                        self
                                    }),
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
                        FunctionState::Bool0Point0 { int0 } => calls_bool_0_run(Bool0State::Point0 { int0 }, ops, budget),
                        FunctionState::Bool0Point1 { int0, bool0 } => calls_bool_0_run(Bool0State::Point1 { int0, bool0 }, ops, budget),
                        FunctionState::Bool0Point2 { int0, bool0, bool1 } => calls_bool_0_run(Bool0State::Point2 { int0, bool0, bool1 }, ops, budget),
                        FunctionState::Bool6Point0 { int0 } => calls_bool_6_run(Bool6State::Point0 { int0 }, ops, budget),
                        FunctionState::Bool6Point1 {  } => calls_bool_6_run(Bool6State::Point1 {  }, ops, budget),
                        FunctionState::Bool6Point2 { bool0 } => calls_bool_6_run(Bool6State::Point2 { bool0 }, ops, budget),
                        FunctionState::Bool6Point3 { int0 } => calls_bool_6_run(Bool6State::Point3 { int0 }, ops, budget),
                        FunctionState::Bool6Point4 { int0, int1 } => calls_bool_6_run(Bool6State::Point4 { int0, int1 }, ops, budget),
                        FunctionState::Bool7Point0 { bool0 } => calls_bool_7_run(Bool7State::Point0 { bool0 }, ops, budget),
                        FunctionState::Bool7Point1 { bool0, bool1 } => calls_bool_7_run(Bool7State::Point1 { bool0, bool1 }, ops, budget),
                        FunctionState::Bool9Point0 { int0 } => calls_bool_9_run(Bool9State::Point0 { int0 }, ops, budget),
                        FunctionState::Bool9Point1 {  } => calls_bool_9_run(Bool9State::Point1 {  }, ops, budget),
                        FunctionState::Bool9Point2 { bool0 } => calls_bool_9_run(Bool9State::Point2 { bool0 }, ops, budget),
                        FunctionState::Bool9Point3 { int0 } => calls_bool_9_run(Bool9State::Point3 { int0 }, ops, budget),
                        FunctionState::Bool9Point4 { int0, int1 } => calls_bool_9_run(Bool9State::Point4 { int0, int1 }, ops, budget),
                        FunctionState::Bool10Point0 { int0, bool_function0, int1 } => calls_bool_10_run(Bool10State::Point0 { int0, bool_function0, int1 }, ops, budget),
                        FunctionState::Bool10Point1 { int0, bool_function0, int1, bool0 } => calls_bool_10_run(Bool10State::Point1 { int0, bool_function0, int1, bool0 }, ops, budget),
                        FunctionState::Bool10Point2 { int0, bool_function0, int1, bool0, bool1 } => calls_bool_10_run(Bool10State::Point2 { int0, bool_function0, int1, bool0, bool1 }, ops, budget),
                        FunctionState::Bool11Point0 { int0, int1 } => calls_bool_11_run(Bool11State::Point0 { int0, int1 }, ops, budget),
                        FunctionState::Bool11Point1 { int0, int1, bool0 } => calls_bool_11_run(Bool11State::Point1 { int0, int1, bool0 }, ops, budget),
                        FunctionState::Bool12Point0 { int0, bool0, int1 } => calls_bool_12_run(Bool12State::Point0 { int0, bool0, int1 }, ops, budget),
                        FunctionState::Bool12Point1 { int0, int1 } => calls_bool_12_run(Bool12State::Point1 { int0, int1 }, ops, budget),
                        FunctionState::Bool12Point2 { int0, int1, bool0 } => calls_bool_12_run(Bool12State::Point2 { int0, int1, bool0 }, ops, budget),
                        FunctionState::Bool12Point3 {  } => calls_bool_12_run(Bool12State::Point3 {  }, ops, budget),
                        FunctionState::Bool12Point4 { bool0 } => calls_bool_12_run(Bool12State::Point4 { bool0 }, ops, budget),
                    }
                }
                enum Bool0State {
                    Point0 { int0: i128 },
                    Point1 { int0: i128, bool0: bool },
                    Point2 { int0: i128, bool0: bool, bool1: bool },
                }
                fn calls_bool_0_run(active: Bool0State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Bool0State::Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point0 { int0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::BoolCall { callee: FunctionState::Bool6Point0 { int0 }, caller: BoolReturn::Bool0Call0 { int0 } }
                            }
                        },
                        Bool0State::Point1 { int0, bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point1 { int0, bool0 }); }
                            *budget -= 1;
                            let bool1 = !bool0;
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point2 { int0, bool0, bool1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Bool { value: bool1 }
                            }
                        },
                        Bool0State::Point2 { int0, bool0, bool1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point2 { int0, bool0, bool1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Bool { value: bool1 }
                            }
                        },
                    }
                }
                enum Bool6State {
                    Point0 { int0: i128 },
                    Point1 {  },
                    Point2 { bool0: bool },
                    Point3 { int0: i128 },
                    Point4 { int0: i128, int1: i128 },
                }
                fn calls_bool_6_run(mut active: Bool6State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            Bool6State::Point0 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool6Point0 { int0 }); }
                                *budget -= 1;
                                active = {
                                    if int0 <= 0_i128 { Bool6State::Point1 {  } } else { Bool6State::Point3 { int0 } }
                                };
                                continue;
                            },
                            Bool6State::Point1 {  } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool6Point1 {  }); }
                                *budget -= 1;
                                let bool0 = true;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool6Point2 { bool0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Bool { value: bool0 }
                                };
                            },
                            Bool6State::Point2 { bool0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool6Point2 { bool0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Bool { value: bool0 }
                                };
                            },
                            Bool6State::Point3 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool6Point3 { int0 }); }
                                *budget -= 1;
                                let int1 = int0 - 1_i128;
                                if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(6)), point: data::compiled::CompiledCheckpoint {
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
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool6Point4 { int0, int1 }); }
                                return {
                                    FunctionStep::BoolTail { callee: FunctionState::Bool9Point0 { int0: int1 } }
                                };
                            },
                            Bool6State::Point4 { int0, int1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool6Point4 { int0, int1 }); }
                                return {
                                    FunctionStep::BoolTail { callee: FunctionState::Bool9Point0 { int0: int1 } }
                                };
                            },
                        }
                    }
                }
                enum Bool7State {
                    Point0 { bool0: bool },
                    Point1 { bool0: bool, bool1: bool },
                }
                fn calls_bool_7_run(active: Bool7State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Bool7State::Point0 { bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool7Point0 { bool0 }); }
                            *budget -= 1;
                            let bool1 = !bool0;
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool7Point1 { bool0, bool1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Bool { value: bool1 }
                            }
                        },
                        Bool7State::Point1 { bool0, bool1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool7Point1 { bool0, bool1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Bool { value: bool1 }
                            }
                        },
                    }
                }
                enum Bool9State {
                    Point0 { int0: i128 },
                    Point1 {  },
                    Point2 { bool0: bool },
                    Point3 { int0: i128 },
                    Point4 { int0: i128, int1: i128 },
                }
                fn calls_bool_9_run(mut active: Bool9State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            Bool9State::Point0 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool9Point0 { int0 }); }
                                *budget -= 1;
                                active = {
                                    if int0 <= 0_i128 { Bool9State::Point1 {  } } else { Bool9State::Point3 { int0 } }
                                };
                                continue;
                            },
                            Bool9State::Point1 {  } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool9Point1 {  }); }
                                *budget -= 1;
                                let bool0 = false;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool9Point2 { bool0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Bool { value: bool0 }
                                };
                            },
                            Bool9State::Point2 { bool0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool9Point2 { bool0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Bool { value: bool0 }
                                };
                            },
                            Bool9State::Point3 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool9Point3 { int0 }); }
                                *budget -= 1;
                                let int1 = int0 - 1_i128;
                                if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(9)), point: data::compiled::CompiledCheckpoint {
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
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool9Point4 { int0, int1 }); }
                                return {
                                    FunctionStep::BoolTail { callee: FunctionState::Bool6Point0 { int0: int1 } }
                                };
                            },
                            Bool9State::Point4 { int0, int1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool9Point4 { int0, int1 }); }
                                return {
                                    FunctionStep::BoolTail { callee: FunctionState::Bool6Point0 { int0: int1 } }
                                };
                            },
                        }
                    }
                }
                enum Bool10State {
                    Point0 { int0: i128, bool_function0: BoolCallable, int1: i128 },
                    Point1 { int0: i128, bool_function0: BoolCallable, int1: i128, bool0: bool },
                    Point2 { int0: i128, bool_function0: BoolCallable, int1: i128, bool0: bool, bool1: bool },
                }
                fn calls_bool_10_run(active: Bool10State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Bool10State::Point0 { int0, bool_function0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool10Point0 { int0, bool_function0, int1 }); }
                            *budget -= 1;
                            let bool0 = int0 > int1;
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool10Point1 { int0, bool_function0, int1, bool0 }); }
                            *budget -= 1;
                            {
                                let callable = &bool_function0;
                                let captures = callable.captures();
                                let target = callable.target();
                                if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (bool0,)) {
                                    return FunctionStep::BoolCall { callee, caller: BoolReturn::Bool10Call1 { int0, bool_function0, int1, bool0 } };
                                }
                                FunctionStep::BoolBridge { function: target, site: data::source::HostCallSite::from_static("example", "<anonymous:1>", data::source::SourceSpan::new(1232, 1256)), arguments: CallArguments { values: Box::new(CallValues { bools: vec![bool0], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: BoolReturn::Bool10Call1 { int0, bool_function0, int1, bool0 } }
                            }
                        },
                        Bool10State::Point1 { int0, bool_function0, int1, bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool10Point1 { int0, bool_function0, int1, bool0 }); }
                            *budget -= 1;
                            {
                                let callable = &bool_function0;
                                let captures = callable.captures();
                                let target = callable.target();
                                if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (bool0,)) {
                                    return FunctionStep::BoolCall { callee, caller: BoolReturn::Bool10Call1 { int0, bool_function0, int1, bool0 } };
                                }
                                FunctionStep::BoolBridge { function: target, site: data::source::HostCallSite::from_static("example", "<anonymous:1>", data::source::SourceSpan::new(1232, 1256)), arguments: CallArguments { values: Box::new(CallValues { bools: vec![bool0], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: BoolReturn::Bool10Call1 { int0, bool_function0, int1, bool0 } }
                            }
                        },
                        Bool10State::Point2 { int0, bool_function0, int1, bool0, bool1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool10Point2 { int0, bool_function0, int1, bool0, bool1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Bool { value: bool1 }
                            }
                        },
                    }
                }
                enum Bool11State {
                    Point0 { int0: i128, int1: i128 },
                    Point1 { int0: i128, int1: i128, bool0: bool },
                }
                fn calls_bool_11_run(active: Bool11State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Bool11State::Point0 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool11Point0 { int0, int1 }); }
                            *budget -= 1;
                            let bool0 = int0 > int1;
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool11Point1 { int0, int1, bool0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Bool { value: bool0 }
                            }
                        },
                        Bool11State::Point1 { int0, int1, bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool11Point1 { int0, int1, bool0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Bool { value: bool0 }
                            }
                        },
                    }
                }
                enum Bool12State {
                    Point0 { int0: i128, bool0: bool, int1: i128 },
                    Point1 { int0: i128, int1: i128 },
                    Point2 { int0: i128, int1: i128, bool0: bool },
                    Point3 {  },
                    Point4 { bool0: bool },
                }
                fn calls_bool_12_run(active: Bool12State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Bool12State::Point0 { int0, bool0, int1 } => {
                            {
                                let values = ops.numeric();
                                let progress = numeric_bool_12_entry((int0, int1, bool0,), values, budget);
                                calls_bool_12_numeric(progress, values)
                            }
                        },
                        Bool12State::Point1 { int0, int1 } => {
                            {
                                let values = ops.numeric();
                                values.ints.clear();
                                values.ints.extend_from_slice(&[int0, int1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                let progress = numeric_bool_12(1, values, budget);
                                calls_bool_12_numeric(progress, values)
                            }
                        },
                        Bool12State::Point2 { int0, int1, bool0 } => {
                            {
                                let values = ops.numeric();
                                values.ints.clear();
                                values.ints.extend_from_slice(&[int0, int1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[bool0]);
                                let progress = numeric_bool_12(2, values, budget);
                                calls_bool_12_numeric(progress, values)
                            }
                        },
                        Bool12State::Point3 {  } => {
                            {
                                let values = ops.numeric();
                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                let progress = numeric_bool_12(3, values, budget);
                                calls_bool_12_numeric(progress, values)
                            }
                        },
                        Bool12State::Point4 { bool0 } => {
                            {
                                let values = ops.numeric();
                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[bool0]);
                                let progress = numeric_bool_12(4, values, budget);
                                calls_bool_12_numeric(progress, values)
                            }
                        },
                    }
                }
                fn calls_entry_0(target: data::function::BoolFunctionId, _captures: &CallCaptureInputs<'_>, inputs: (bool,)) -> Option<FunctionState> {
                    let (argument0,) = inputs;
                    match target.0 {
                        7 => Some(FunctionState::Bool7Point0 { bool0: argument0 }),
                        _ => None,
                    }
                }
                fn calls_bool_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool0Point0 { int0: values.int(0)? },
                        1 => FunctionState::Bool0Point1 { int0: values.int(0)?, bool0: values.bool(0)? },
                        2 => FunctionState::Bool0Point2 { int0: values.int(0)?, bool0: values.bool(0)?, bool1: values.bool(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point, values) { return Some(execution); }
                    let active = calls_bool_0_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_bool_6_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool6Point0 { int0: values.int(0)? },
                        1 => FunctionState::Bool6Point1 {  },
                        2 => FunctionState::Bool6Point2 { bool0: values.bool(0)? },
                        3 => FunctionState::Bool6Point3 { int0: values.int(0)? },
                        4 => FunctionState::Bool6Point4 { int0: values.int(0)?, int1: values.int(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_6_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(6)), point, values) { return Some(execution); }
                    let active = calls_bool_6_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_bool_7_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool7Point0 { bool0: values.bool(0)? },
                        1 => FunctionState::Bool7Point1 { bool0: values.bool(0)?, bool1: values.bool(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_7_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(7)), point, values) { return Some(execution); }
                    let active = calls_bool_7_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_bool_9_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool9Point0 { int0: values.int(0)? },
                        1 => FunctionState::Bool9Point1 {  },
                        2 => FunctionState::Bool9Point2 { bool0: values.bool(0)? },
                        3 => FunctionState::Bool9Point3 { int0: values.int(0)? },
                        4 => FunctionState::Bool9Point4 { int0: values.int(0)?, int1: values.int(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_9_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(9)), point, values) { return Some(execution); }
                    let active = calls_bool_9_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_bool_10_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool10Point0 { int0: values.int(0)?, bool_function0: values.bool_function(0)?, int1: values.int(1)? },
                        1 => {
                            if !matches!(values.bool_function_target(0)?, data::function::BoolFunctionId(7)) { return None; }
                            FunctionState::Bool10Point1 { int0: values.int(0)?, bool_function0: values.bool_function(0)?, int1: values.int(1)?, bool0: values.bool(0)? }
                        },
                        2 => FunctionState::Bool10Point2 { int0: values.int(0)?, bool_function0: values.bool_function(0)?, int1: values.int(1)?, bool0: values.bool(0)?, bool1: values.bool(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_10_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(10)), point, values) { return Some(execution); }
                    let active = calls_bool_10_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_bool_11_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool11Point0 { int0: values.int(0)?, int1: values.int(1)? },
                        1 => FunctionState::Bool11Point1 { int0: values.int(0)?, int1: values.int(1)?, bool0: values.bool(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_11_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(11)), point, values) { return Some(execution); }
                    let active = calls_bool_11_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_bool_12_numeric(progress: data::compiled::CompiledProgress, values: &data::compiled::numeric::NumericValues) -> FunctionStep {
                    const STATES: [fn(&data::compiled::numeric::NumericValues) -> FunctionState; 5] = [
                        |values| FunctionState::Bool12Point0 { int0: values.ints[0], int1: values.ints[1], bool0: values.bools[0] },
                        |values| FunctionState::Bool12Point1 { int0: values.ints[0], int1: values.ints[1] },
                        |values| FunctionState::Bool12Point2 { int0: values.ints[0], int1: values.ints[1], bool0: values.bools[0] },
                        |_values| FunctionState::Bool12Point3 {  },
                        |values| FunctionState::Bool12Point4 { bool0: values.bools[0] },
                    ];
                    const RETURNS: [fn(&data::compiled::numeric::NumericValues) -> FunctionStep; 2] = [
                        |values| FunctionStep::Bool { value: values.bools[0] },
                        |values| FunctionStep::Bool { value: values.bools[0] },
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
                            ];
                            FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(12)), point: POINTS[point], values: Box::new(CallValues { ints: values.ints.iter().copied().map(Into::into).collect(), bools: values.bools.clone(), ..CallValues::default() }) }
                        },
                        data::compiled::CompiledProgress::Complete(exit) => RETURNS[exit.0](values),
                    }
                }
                fn calls_bool_12_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool12Point0 { int0: values.int(0)?, bool0: values.bool(0)?, int1: values.int(1)? },
                        1 => FunctionState::Bool12Point1 { int0: values.int(0)?, int1: values.int(1)? },
                        2 => FunctionState::Bool12Point2 { int0: values.int(0)?, int1: values.int(1)?, bool0: values.bool(0)? },
                        3 => FunctionState::Bool12Point3 {  },
                        4 => FunctionState::Bool12Point4 { bool0: values.bool(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_12_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(12)), point, values) { return Some(execution); }
                    let active = calls_bool_12_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                [calls_bool_0_start, calls_bool_6_start, calls_bool_7_start, calls_bool_9_start, calls_bool_10_start, calls_bool_11_start, calls_bool_12_start]
            };
            const CALL_GROUP_3: [data::compiled::calls::CallStart; 3] = {
                use data::compiled::calls::{BoolCallable, CallArguments, CallCapture, CallCaptureInputs, CallExecution, CallInputs, CallOps, CallOutput, CallProgress, CallStorage, CallValues};
                enum FunctionState {
                    Bool0Point0 { int0: i128 },
                    Bool0Point1 { int0: i128, bool0: bool },
                    Bool0Point2 { int0: i128, bool0: bool, bool1: bool },
                    Bool1Point0 { int0: i128, int1: i128 },
                    Bool1Point1 { int0: i128, int1: i128, bool_function0: BoolCallable },
                    Bool1Point2 { int0: i128, int1: i128, bool_function0: BoolCallable, bool_function1: BoolCallable },
                    Bool1Point3 { int0: i128, int1: i128, bool_function0: BoolCallable, bool_function1: BoolCallable, bool0: bool },
                    Bool3Point0 { int0: i128, int1: i128 },
                    Bool3Point1 { int0: i128, int1: i128, bool_function0: BoolCallable },
                    Bool3Point2 { int0: i128, int1: i128, bool_function0: BoolCallable, bool0: bool },
                    Bool3Point3 { int0: i128, int1: i128, bool_function0: BoolCallable, bool0: bool, bool1: bool },
                    Bool5Point0 { bool0: bool, int0: i128, int1: i128 },
                    Bool5Point1 { bool0: bool, int0: i128, int1: i128, bool_function0: BoolCallable },
                    Bool5Point2 { bool0: bool, int0: i128, int1: i128, bool_function0: BoolCallable, bool1: bool },
                    Bool6Point0 { int0: i128 },
                    Bool6Point1 {  },
                    Bool6Point2 { bool0: bool },
                    Bool6Point3 { int0: i128 },
                    Bool6Point4 { int0: i128, int1: i128 },
                    Bool7Point0 { bool0: bool },
                    Bool7Point1 { bool0: bool, bool1: bool },
                    Bool9Point0 { int0: i128 },
                    Bool9Point1 {  },
                    Bool9Point2 { bool0: bool },
                    Bool9Point3 { int0: i128 },
                    Bool9Point4 { int0: i128, int1: i128 },
                    Bool10Point0 { int0: i128, bool_function0: BoolCallable, int1: i128 },
                    Bool10Point1 { int0: i128, bool_function0: BoolCallable, int1: i128, bool0: bool },
                    Bool10Point2 { int0: i128, bool_function0: BoolCallable, int1: i128, bool0: bool, bool1: bool },
                    Bool11Point0 { int0: i128, int1: i128 },
                    Bool11Point1 { int0: i128, int1: i128, bool0: bool },
                    Bool12Point0 { int0: i128, bool0: bool, int1: i128 },
                    Bool12Point1 { int0: i128, int1: i128 },
                    Bool12Point2 { int0: i128, int1: i128, bool0: bool },
                    Bool12Point3 {  },
                    Bool12Point4 { bool0: bool },
                    BoolFunction0Point0 { int0: i128, bool_function0: BoolCallable },
                    BoolFunction0Point1 { int0: i128, bool_function0: BoolCallable, bool_function1: BoolCallable },
                    BoolFunction1Point0 { int0: i128 },
                    BoolFunction1Point2 { int0: i128 },
                    BoolFunction1Point3 { int0: i128, bool_function0: BoolCallable },
                    BoolFunction2Point0 { bool0: bool, int0: i128 },
                    BoolFunction3Point0 { bool0: bool, int0: i128 },
                    BoolFunction3Point1 { bool0: bool, int0: i128, bool_function0: BoolCallable },
                }
                enum BoolReturn {
                    Bool0Call0 { int0: i128 },
                    Bool1Call2 { int0: i128, int1: i128, bool_function0: BoolCallable, bool_function1: BoolCallable },
                    Bool3Call1 { int0: i128, int1: i128, bool_function0: BoolCallable },
                    Bool5Call1 { bool0: bool, int0: i128, int1: i128, bool_function0: BoolCallable },
                    Bool10Call1 { int0: i128, bool_function0: BoolCallable, int1: i128, bool0: bool },
                }
                impl BoolReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                            Self::Bool0Call0 { .. } => data::source::HostCallSite::from_static("example", "mutual", data::source::SourceSpan::new(1085, 1096)),
                            Self::Bool1Call2 { .. } => data::source::HostCallSite::from_static("example", "callable_captures", data::source::SourceSpan::new(1364, 1376)),
                            Self::Bool3Call1 { .. } => data::source::HostCallSite::from_static("example", "producer_suffix_bool", data::source::SourceSpan::new(2452, 2464)),
                            Self::Bool5Call1 { .. } => data::source::HostCallSite::from_static("example", "bool_captures", data::source::SourceSpan::new(3721, 3733)),
                            Self::Bool10Call1 { .. } => data::source::HostCallSite::from_static("example", "<anonymous:1>", data::source::SourceSpan::new(1232, 1256)),
                        }
                    }
                    fn small(self, result: bool) -> FunctionState {
                        match self {
                            Self::Bool0Call0 { int0 } => {
                                let bool0 = result;
                                FunctionState::Bool0Point1 { int0, bool0 }
                            },
                            Self::Bool1Call2 { int0, int1, bool_function0, bool_function1 } => {
                                let bool0 = result;
                                FunctionState::Bool1Point3 { int0, int1, bool_function0, bool_function1, bool0 }
                            },
                            Self::Bool3Call1 { int0, int1, bool_function0 } => {
                                let bool0 = result;
                                FunctionState::Bool3Point2 { int0, int1, bool_function0, bool0 }
                            },
                            Self::Bool5Call1 { bool0, int0, int1, bool_function0 } => {
                                let bool1 = result;
                                FunctionState::Bool5Point2 { bool0, int0, int1, bool_function0, bool1 }
                            },
                            Self::Bool10Call1 { int0, bool_function0, int1, bool0 } => {
                                let bool1 = result;
                                FunctionState::Bool10Point2 { int0, bool_function0, int1, bool0, bool1 }
                            },
                        }
                    }
                    fn resume(self, result: bool) -> FunctionState { self.small(result) }
                }
                enum BoolFunctionReturn {
                    Bool1Call1 { int0: i128, int1: i128, bool_function0: BoolCallable },
                    Bool3Call0 { int0: i128, int1: i128 },
                    Bool5Call0 { bool0: bool, int0: i128, int1: i128 },
                }
                impl BoolFunctionReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                            Self::Bool1Call1 { .. } => data::source::HostCallSite::from_static("example", "callable_captures", data::source::SourceSpan::new(1336, 1361)),
                            Self::Bool3Call0 { .. } => data::source::HostCallSite::from_static("example", "producer_suffix_bool", data::source::SourceSpan::new(2429, 2448)),
                            Self::Bool5Call0 { .. } => data::source::HostCallSite::from_static("example", "bool_captures", data::source::SourceSpan::new(3684, 3718)),
                        }
                    }
                    fn small(self, result: BoolCallable) -> FunctionState {
                        match self {
                            Self::Bool1Call1 { int0, int1, bool_function0 } => {
                                let bool_function1 = result.with_type(data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                });
                                FunctionState::Bool1Point2 { int0, int1, bool_function0, bool_function1 }
                            },
                            Self::Bool3Call0 { int0, int1 } => {
                                let bool_function0 = result.with_type(data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                });
                                FunctionState::Bool3Point1 { int0, int1, bool_function0 }
                            },
                            Self::Bool5Call0 { bool0, int0, int1 } => {
                                let bool_function0 = result.with_type(data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                });
                                FunctionState::Bool5Point1 { bool0, int0, int1, bool_function0 }
                            },
                        }
                    }
                    fn resume(self, result: BoolCallable) -> FunctionState { self.small(result) }
                }
                #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                enum FunctionStep {
                    Yield(FunctionState),
                    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                    BoolCall { callee: FunctionState, caller: BoolReturn },
                    BoolTail { callee: FunctionState },
                    Bool { value: bool },
                    BoolBridge { function: data::function::BoolFunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: BoolReturn },
                    BoolFunctionCall { callee: FunctionState, caller: BoolFunctionReturn },
                    BoolFunctionTail { callee: FunctionState },
                    BoolFunction { value: BoolCallable },
                }
                struct FunctionExecution {
                    active: Option<FunctionState>,
                    pending_entry: bool,
                    boolean_returns: Vec<BoolReturn>,
                    boolean_function_returns: Vec<BoolFunctionReturn>,
                }
                impl FunctionExecution {
                    fn new(active: FunctionState) -> Self {
                        Self {
                            active: Some(active),
                            pending_entry: false,
                            boolean_returns: Vec::new(),
                            boolean_function_returns: Vec::new(),
                        }
                    }
                }
                impl CallExecution for FunctionExecution {
                    fn restart(&mut self, target: data::compiled::CallTarget, point: usize, values: CallInputs<'_>) -> bool {
                        if self.active.is_some() { return false; }
                        let active = match target {
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)) => calls_bool_1_state(point, values),
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(3)) => calls_bool_3_state(point, values),
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(5)) => calls_bool_5_state(point, values),
                            _ => None,
                        };
                        let Some(active) = active else { return false; };
                        self.active = Some(active);
                        true
                    }
                    fn retained_bytes(&self) -> usize {
                        std::mem::size_of::<Self>() + self.boolean_returns.capacity() * std::mem::size_of::<BoolReturn>() + self.boolean_function_returns.capacity() * std::mem::size_of::<BoolFunctionReturn>()
                    }
                    fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                        let Some(mut active) = self.active.take() else { return CallProgress::Yield(self); };
                        loop {
                            if self.pending_entry {
                                if *budget == 0 { self.active = Some(active); return CallProgress::Yield(self); }
                                *budget -= 1;
                                self.pending_entry = false;
                            }
                            match function_step(active, ops, budget) {
                                FunctionStep::Yield(active) => {
                                    self.active = Some(active);
                                    return CallProgress::Yield(self);
                                },
                                FunctionStep::BoolCall { callee, caller } => {
                                    self.boolean_returns.push(caller);
                                    active = callee;
                                },
                                FunctionStep::BoolTail { callee } => {
                                    *budget -= 1;
                                    self.pending_entry = self.boolean_returns.is_empty() && ops.root_tail_entry();
                                    active = callee;
                                },
                                FunctionStep::Bool { value } => {
                                    if let Some(caller) = self.boolean_returns.pop() {
                                        active = caller.small(value);
                                    } else {
                                        self.boolean_returns.clear();
                                        self.boolean_function_returns.clear();
                                        return CallProgress::Complete { output: CallOutput::Bool(value), execution: self };
                                    }
                                },
                                FunctionStep::BoolBridge { function, site, arguments, caller } => return CallProgress::Bool {
                                    function, site, arguments,
                                    resume: Box::new(move |value| {
                                        self.active = Some(caller.resume(value));
                                        self
                                    }),
                                },
                                FunctionStep::BoolFunctionCall { callee, caller } => {
                                    self.boolean_function_returns.push(caller);
                                    active = callee;
                                },
                                FunctionStep::BoolFunctionTail { callee } => {
                                    *budget -= 1;
                                    self.pending_entry = self.boolean_function_returns.is_empty() && ops.root_tail_entry();
                                    active = callee;
                                },
                                FunctionStep::BoolFunction { value } => {
                                    if let Some(caller) = self.boolean_function_returns.pop() {
                                        active = caller.small(value);
                                    } else {
                                        self.boolean_returns.clear();
                                        self.boolean_function_returns.clear();
                                        return CallProgress::Complete { output: CallOutput::BoolFunction(value), execution: self };
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
                        FunctionState::Bool0Point0 { int0 } => calls_bool_0_run(Bool0State::Point0 { int0 }, ops, budget),
                        FunctionState::Bool0Point1 { int0, bool0 } => calls_bool_0_run(Bool0State::Point1 { int0, bool0 }, ops, budget),
                        FunctionState::Bool0Point2 { int0, bool0, bool1 } => calls_bool_0_run(Bool0State::Point2 { int0, bool0, bool1 }, ops, budget),
                        FunctionState::Bool1Point0 { int0, int1 } => calls_bool_1_run(Bool1State::Point0 { int0, int1 }, ops, budget),
                        FunctionState::Bool1Point1 { int0, int1, bool_function0 } => calls_bool_1_run(Bool1State::Point1 { int0, int1, bool_function0 }, ops, budget),
                        FunctionState::Bool1Point2 { int0, int1, bool_function0, bool_function1 } => calls_bool_1_run(Bool1State::Point2 { int0, int1, bool_function0, bool_function1 }, ops, budget),
                        FunctionState::Bool1Point3 { int0, int1, bool_function0, bool_function1, bool0 } => calls_bool_1_run(Bool1State::Point3 { int0, int1, bool_function0, bool_function1, bool0 }, ops, budget),
                        FunctionState::Bool3Point0 { int0, int1 } => calls_bool_3_run(Bool3State::Point0 { int0, int1 }, ops, budget),
                        FunctionState::Bool3Point1 { int0, int1, bool_function0 } => calls_bool_3_run(Bool3State::Point1 { int0, int1, bool_function0 }, ops, budget),
                        FunctionState::Bool3Point2 { int0, int1, bool_function0, bool0 } => calls_bool_3_run(Bool3State::Point2 { int0, int1, bool_function0, bool0 }, ops, budget),
                        FunctionState::Bool3Point3 { int0, int1, bool_function0, bool0, bool1 } => calls_bool_3_run(Bool3State::Point3 { int0, int1, bool_function0, bool0, bool1 }, ops, budget),
                        FunctionState::Bool5Point0 { bool0, int0, int1 } => calls_bool_5_run(Bool5State::Point0 { bool0, int0, int1 }, ops, budget),
                        FunctionState::Bool5Point1 { bool0, int0, int1, bool_function0 } => calls_bool_5_run(Bool5State::Point1 { bool0, int0, int1, bool_function0 }, ops, budget),
                        FunctionState::Bool5Point2 { bool0, int0, int1, bool_function0, bool1 } => calls_bool_5_run(Bool5State::Point2 { bool0, int0, int1, bool_function0, bool1 }, ops, budget),
                        FunctionState::Bool6Point0 { int0 } => calls_bool_6_run(Bool6State::Point0 { int0 }, ops, budget),
                        FunctionState::Bool6Point1 {  } => calls_bool_6_run(Bool6State::Point1 {  }, ops, budget),
                        FunctionState::Bool6Point2 { bool0 } => calls_bool_6_run(Bool6State::Point2 { bool0 }, ops, budget),
                        FunctionState::Bool6Point3 { int0 } => calls_bool_6_run(Bool6State::Point3 { int0 }, ops, budget),
                        FunctionState::Bool6Point4 { int0, int1 } => calls_bool_6_run(Bool6State::Point4 { int0, int1 }, ops, budget),
                        FunctionState::Bool7Point0 { bool0 } => calls_bool_7_run(Bool7State::Point0 { bool0 }, ops, budget),
                        FunctionState::Bool7Point1 { bool0, bool1 } => calls_bool_7_run(Bool7State::Point1 { bool0, bool1 }, ops, budget),
                        FunctionState::Bool9Point0 { int0 } => calls_bool_9_run(Bool9State::Point0 { int0 }, ops, budget),
                        FunctionState::Bool9Point1 {  } => calls_bool_9_run(Bool9State::Point1 {  }, ops, budget),
                        FunctionState::Bool9Point2 { bool0 } => calls_bool_9_run(Bool9State::Point2 { bool0 }, ops, budget),
                        FunctionState::Bool9Point3 { int0 } => calls_bool_9_run(Bool9State::Point3 { int0 }, ops, budget),
                        FunctionState::Bool9Point4 { int0, int1 } => calls_bool_9_run(Bool9State::Point4 { int0, int1 }, ops, budget),
                        FunctionState::Bool10Point0 { int0, bool_function0, int1 } => calls_bool_10_run(Bool10State::Point0 { int0, bool_function0, int1 }, ops, budget),
                        FunctionState::Bool10Point1 { int0, bool_function0, int1, bool0 } => calls_bool_10_run(Bool10State::Point1 { int0, bool_function0, int1, bool0 }, ops, budget),
                        FunctionState::Bool10Point2 { int0, bool_function0, int1, bool0, bool1 } => calls_bool_10_run(Bool10State::Point2 { int0, bool_function0, int1, bool0, bool1 }, ops, budget),
                        FunctionState::Bool11Point0 { int0, int1 } => calls_bool_11_run(Bool11State::Point0 { int0, int1 }, ops, budget),
                        FunctionState::Bool11Point1 { int0, int1, bool0 } => calls_bool_11_run(Bool11State::Point1 { int0, int1, bool0 }, ops, budget),
                        FunctionState::Bool12Point0 { int0, bool0, int1 } => calls_bool_12_run(Bool12State::Point0 { int0, bool0, int1 }, ops, budget),
                        FunctionState::Bool12Point1 { int0, int1 } => calls_bool_12_run(Bool12State::Point1 { int0, int1 }, ops, budget),
                        FunctionState::Bool12Point2 { int0, int1, bool0 } => calls_bool_12_run(Bool12State::Point2 { int0, int1, bool0 }, ops, budget),
                        FunctionState::Bool12Point3 {  } => calls_bool_12_run(Bool12State::Point3 {  }, ops, budget),
                        FunctionState::Bool12Point4 { bool0 } => calls_bool_12_run(Bool12State::Point4 { bool0 }, ops, budget),
                        FunctionState::BoolFunction0Point0 { int0, bool_function0 } => calls_boolfunction_0_run(BoolFunction0State::Point0 { int0, bool_function0 }, ops, budget),
                        FunctionState::BoolFunction0Point1 { int0, bool_function0, bool_function1 } => calls_boolfunction_0_run(BoolFunction0State::Point1 { int0, bool_function0, bool_function1 }, ops, budget),
                        FunctionState::BoolFunction1Point0 { int0 } => calls_boolfunction_1_run(BoolFunction1State::Point0 { int0 }, ops, budget),
                        FunctionState::BoolFunction1Point2 { int0 } => calls_boolfunction_1_run(BoolFunction1State::Point2 { int0 }, ops, budget),
                        FunctionState::BoolFunction1Point3 { int0, bool_function0 } => calls_boolfunction_1_run(BoolFunction1State::Point3 { int0, bool_function0 }, ops, budget),
                        FunctionState::BoolFunction2Point0 { bool0, int0 } => calls_boolfunction_2_run(BoolFunction2State::Point0 { bool0, int0 }, ops, budget),
                        FunctionState::BoolFunction3Point0 { bool0, int0 } => calls_boolfunction_3_run(BoolFunction3State::Point0 { bool0, int0 }, ops, budget),
                        FunctionState::BoolFunction3Point1 { bool0, int0, bool_function0 } => calls_boolfunction_3_run(BoolFunction3State::Point1 { bool0, int0, bool_function0 }, ops, budget),
                    }
                }
                enum Bool0State {
                    Point0 { int0: i128 },
                    Point1 { int0: i128, bool0: bool },
                    Point2 { int0: i128, bool0: bool, bool1: bool },
                }
                fn calls_bool_0_run(active: Bool0State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Bool0State::Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point0 { int0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::BoolCall { callee: FunctionState::Bool6Point0 { int0 }, caller: BoolReturn::Bool0Call0 { int0 } }
                            }
                        },
                        Bool0State::Point1 { int0, bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point1 { int0, bool0 }); }
                            *budget -= 1;
                            let bool1 = !bool0;
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point2 { int0, bool0, bool1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Bool { value: bool1 }
                            }
                        },
                        Bool0State::Point2 { int0, bool0, bool1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point2 { int0, bool0, bool1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Bool { value: bool1 }
                            }
                        },
                    }
                }
                enum Bool1State {
                    Point0 { int0: i128, int1: i128 },
                    Point1 { int0: i128, int1: i128, bool_function0: BoolCallable },
                    Point2 { int0: i128, int1: i128, bool_function0: BoolCallable, bool_function1: BoolCallable },
                    Point3 { int0: i128, int1: i128, bool_function0: BoolCallable, bool_function1: BoolCallable, bool0: bool },
                }
                fn calls_bool_1_run(active: Bool1State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Bool1State::Point0 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point0 { int0, int1 }); }
                            *budget -= 1;
                            let bool_function0 = ops.bool_reference(data::function::BoolFunctionId(7), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Bool,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                            });
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point1 { int0, int1, bool_function0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::BoolFunctionCall { callee: FunctionState::BoolFunction0Point0 { int0, bool_function0: bool_function0.clone() }, caller: BoolFunctionReturn::Bool1Call1 { int0, int1, bool_function0 } }
                            }
                        },
                        Bool1State::Point1 { int0, int1, bool_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point1 { int0, int1, bool_function0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::BoolFunctionCall { callee: FunctionState::BoolFunction0Point0 { int0, bool_function0: bool_function0.clone() }, caller: BoolFunctionReturn::Bool1Call1 { int0, int1, bool_function0 } }
                            }
                        },
                        Bool1State::Point2 { int0, int1, bool_function0, bool_function1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point2 { int0, int1, bool_function0, bool_function1 }); }
                            *budget -= 1;
                            {
                                let callable = &bool_function1;
                                let captures = callable.captures();
                                let target = callable.target();
                                if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (int1,)) {
                                    return FunctionStep::BoolCall { callee, caller: BoolReturn::Bool1Call2 { int0, int1, bool_function0, bool_function1 } };
                                }
                                FunctionStep::BoolBridge { function: target, site: data::source::HostCallSite::from_static("example", "callable_captures", data::source::SourceSpan::new(1364, 1376)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int1.into()], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: BoolReturn::Bool1Call2 { int0, int1, bool_function0, bool_function1 } }
                            }
                        },
                        Bool1State::Point3 { int0, int1, bool_function0, bool_function1, bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point3 { int0, int1, bool_function0, bool_function1, bool0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Bool { value: bool0 }
                            }
                        },
                    }
                }
                enum Bool3State {
                    Point0 { int0: i128, int1: i128 },
                    Point1 { int0: i128, int1: i128, bool_function0: BoolCallable },
                    Point2 { int0: i128, int1: i128, bool_function0: BoolCallable, bool0: bool },
                    Point3 { int0: i128, int1: i128, bool_function0: BoolCallable, bool0: bool, bool1: bool },
                }
                fn calls_bool_3_run(active: Bool3State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Bool3State::Point0 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool3Point0 { int0, int1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::BoolFunctionCall { callee: FunctionState::BoolFunction1Point0 { int0 }, caller: BoolFunctionReturn::Bool3Call0 { int0, int1 } }
                            }
                        },
                        Bool3State::Point1 { int0, int1, bool_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool3Point1 { int0, int1, bool_function0 }); }
                            *budget -= 1;
                            {
                                let callable = &bool_function0;
                                let captures = callable.captures();
                                let target = callable.target();
                                if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (int1,)) {
                                    return FunctionStep::BoolCall { callee, caller: BoolReturn::Bool3Call1 { int0, int1, bool_function0 } };
                                }
                                FunctionStep::BoolBridge { function: target, site: data::source::HostCallSite::from_static("example", "producer_suffix_bool", data::source::SourceSpan::new(2452, 2464)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int1.into()], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: BoolReturn::Bool3Call1 { int0, int1, bool_function0 } }
                            }
                        },
                        Bool3State::Point2 { int0, int1, bool_function0, bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool3Point2 { int0, int1, bool_function0, bool0 }); }
                            *budget -= 1;
                            let bool1 = !bool0;
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool3Point3 { int0, int1, bool_function0, bool0, bool1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Bool { value: bool1 }
                            }
                        },
                        Bool3State::Point3 { int0, int1, bool_function0, bool0, bool1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool3Point3 { int0, int1, bool_function0, bool0, bool1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Bool { value: bool1 }
                            }
                        },
                    }
                }
                enum Bool5State {
                    Point0 { bool0: bool, int0: i128, int1: i128 },
                    Point1 { bool0: bool, int0: i128, int1: i128, bool_function0: BoolCallable },
                    Point2 { bool0: bool, int0: i128, int1: i128, bool_function0: BoolCallable, bool1: bool },
                }
                fn calls_bool_5_run(active: Bool5State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Bool5State::Point0 { bool0, int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool5Point0 { bool0, int0, int1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::BoolFunctionCall { callee: FunctionState::BoolFunction2Point0 { bool0, int0 }, caller: BoolFunctionReturn::Bool5Call0 { bool0, int0, int1 } }
                            }
                        },
                        Bool5State::Point1 { bool0, int0, int1, bool_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool5Point1 { bool0, int0, int1, bool_function0 }); }
                            *budget -= 1;
                            {
                                let callable = &bool_function0;
                                let captures = callable.captures();
                                let target = callable.target();
                                if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (int1,)) {
                                    return FunctionStep::BoolCall { callee, caller: BoolReturn::Bool5Call1 { bool0, int0, int1, bool_function0 } };
                                }
                                FunctionStep::BoolBridge { function: target, site: data::source::HostCallSite::from_static("example", "bool_captures", data::source::SourceSpan::new(3721, 3733)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int1.into()], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: BoolReturn::Bool5Call1 { bool0, int0, int1, bool_function0 } }
                            }
                        },
                        Bool5State::Point2 { bool0, int0, int1, bool_function0, bool1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool5Point2 { bool0, int0, int1, bool_function0, bool1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Bool { value: bool1 }
                            }
                        },
                    }
                }
                enum Bool6State {
                    Point0 { int0: i128 },
                    Point1 {  },
                    Point2 { bool0: bool },
                    Point3 { int0: i128 },
                    Point4 { int0: i128, int1: i128 },
                }
                fn calls_bool_6_run(mut active: Bool6State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            Bool6State::Point0 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool6Point0 { int0 }); }
                                *budget -= 1;
                                active = {
                                    if int0 <= 0_i128 { Bool6State::Point1 {  } } else { Bool6State::Point3 { int0 } }
                                };
                                continue;
                            },
                            Bool6State::Point1 {  } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool6Point1 {  }); }
                                *budget -= 1;
                                let bool0 = true;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool6Point2 { bool0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Bool { value: bool0 }
                                };
                            },
                            Bool6State::Point2 { bool0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool6Point2 { bool0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Bool { value: bool0 }
                                };
                            },
                            Bool6State::Point3 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool6Point3 { int0 }); }
                                *budget -= 1;
                                let int1 = int0 - 1_i128;
                                if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(6)), point: data::compiled::CompiledCheckpoint {
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
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool6Point4 { int0, int1 }); }
                                return {
                                    FunctionStep::BoolTail { callee: FunctionState::Bool9Point0 { int0: int1 } }
                                };
                            },
                            Bool6State::Point4 { int0, int1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool6Point4 { int0, int1 }); }
                                return {
                                    FunctionStep::BoolTail { callee: FunctionState::Bool9Point0 { int0: int1 } }
                                };
                            },
                        }
                    }
                }
                enum Bool7State {
                    Point0 { bool0: bool },
                    Point1 { bool0: bool, bool1: bool },
                }
                fn calls_bool_7_run(active: Bool7State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Bool7State::Point0 { bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool7Point0 { bool0 }); }
                            *budget -= 1;
                            let bool1 = !bool0;
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool7Point1 { bool0, bool1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Bool { value: bool1 }
                            }
                        },
                        Bool7State::Point1 { bool0, bool1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool7Point1 { bool0, bool1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Bool { value: bool1 }
                            }
                        },
                    }
                }
                enum Bool9State {
                    Point0 { int0: i128 },
                    Point1 {  },
                    Point2 { bool0: bool },
                    Point3 { int0: i128 },
                    Point4 { int0: i128, int1: i128 },
                }
                fn calls_bool_9_run(mut active: Bool9State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            Bool9State::Point0 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool9Point0 { int0 }); }
                                *budget -= 1;
                                active = {
                                    if int0 <= 0_i128 { Bool9State::Point1 {  } } else { Bool9State::Point3 { int0 } }
                                };
                                continue;
                            },
                            Bool9State::Point1 {  } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool9Point1 {  }); }
                                *budget -= 1;
                                let bool0 = false;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool9Point2 { bool0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Bool { value: bool0 }
                                };
                            },
                            Bool9State::Point2 { bool0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool9Point2 { bool0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Bool { value: bool0 }
                                };
                            },
                            Bool9State::Point3 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool9Point3 { int0 }); }
                                *budget -= 1;
                                let int1 = int0 - 1_i128;
                                if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(9)), point: data::compiled::CompiledCheckpoint {
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
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool9Point4 { int0, int1 }); }
                                return {
                                    FunctionStep::BoolTail { callee: FunctionState::Bool6Point0 { int0: int1 } }
                                };
                            },
                            Bool9State::Point4 { int0, int1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool9Point4 { int0, int1 }); }
                                return {
                                    FunctionStep::BoolTail { callee: FunctionState::Bool6Point0 { int0: int1 } }
                                };
                            },
                        }
                    }
                }
                enum Bool10State {
                    Point0 { int0: i128, bool_function0: BoolCallable, int1: i128 },
                    Point1 { int0: i128, bool_function0: BoolCallable, int1: i128, bool0: bool },
                    Point2 { int0: i128, bool_function0: BoolCallable, int1: i128, bool0: bool, bool1: bool },
                }
                fn calls_bool_10_run(active: Bool10State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Bool10State::Point0 { int0, bool_function0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool10Point0 { int0, bool_function0, int1 }); }
                            *budget -= 1;
                            let bool0 = int0 > int1;
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool10Point1 { int0, bool_function0, int1, bool0 }); }
                            *budget -= 1;
                            {
                                let callable = &bool_function0;
                                let captures = callable.captures();
                                let target = callable.target();
                                if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_1(target, &captures, (bool0,)) {
                                    return FunctionStep::BoolCall { callee, caller: BoolReturn::Bool10Call1 { int0, bool_function0, int1, bool0 } };
                                }
                                FunctionStep::BoolBridge { function: target, site: data::source::HostCallSite::from_static("example", "<anonymous:1>", data::source::SourceSpan::new(1232, 1256)), arguments: CallArguments { values: Box::new(CallValues { bools: vec![bool0], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: BoolReturn::Bool10Call1 { int0, bool_function0, int1, bool0 } }
                            }
                        },
                        Bool10State::Point1 { int0, bool_function0, int1, bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool10Point1 { int0, bool_function0, int1, bool0 }); }
                            *budget -= 1;
                            {
                                let callable = &bool_function0;
                                let captures = callable.captures();
                                let target = callable.target();
                                if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_1(target, &captures, (bool0,)) {
                                    return FunctionStep::BoolCall { callee, caller: BoolReturn::Bool10Call1 { int0, bool_function0, int1, bool0 } };
                                }
                                FunctionStep::BoolBridge { function: target, site: data::source::HostCallSite::from_static("example", "<anonymous:1>", data::source::SourceSpan::new(1232, 1256)), arguments: CallArguments { values: Box::new(CallValues { bools: vec![bool0], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: BoolReturn::Bool10Call1 { int0, bool_function0, int1, bool0 } }
                            }
                        },
                        Bool10State::Point2 { int0, bool_function0, int1, bool0, bool1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool10Point2 { int0, bool_function0, int1, bool0, bool1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Bool { value: bool1 }
                            }
                        },
                    }
                }
                enum Bool11State {
                    Point0 { int0: i128, int1: i128 },
                    Point1 { int0: i128, int1: i128, bool0: bool },
                }
                fn calls_bool_11_run(active: Bool11State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Bool11State::Point0 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool11Point0 { int0, int1 }); }
                            *budget -= 1;
                            let bool0 = int0 > int1;
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool11Point1 { int0, int1, bool0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Bool { value: bool0 }
                            }
                        },
                        Bool11State::Point1 { int0, int1, bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool11Point1 { int0, int1, bool0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::Bool { value: bool0 }
                            }
                        },
                    }
                }
                enum Bool12State {
                    Point0 { int0: i128, bool0: bool, int1: i128 },
                    Point1 { int0: i128, int1: i128 },
                    Point2 { int0: i128, int1: i128, bool0: bool },
                    Point3 {  },
                    Point4 { bool0: bool },
                }
                fn calls_bool_12_run(active: Bool12State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Bool12State::Point0 { int0, bool0, int1 } => {
                            {
                                let values = ops.numeric();
                                let progress = numeric_bool_12_entry((int0, int1, bool0,), values, budget);
                                calls_bool_12_numeric(progress, values)
                            }
                        },
                        Bool12State::Point1 { int0, int1 } => {
                            {
                                let values = ops.numeric();
                                values.ints.clear();
                                values.ints.extend_from_slice(&[int0, int1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                let progress = numeric_bool_12(1, values, budget);
                                calls_bool_12_numeric(progress, values)
                            }
                        },
                        Bool12State::Point2 { int0, int1, bool0 } => {
                            {
                                let values = ops.numeric();
                                values.ints.clear();
                                values.ints.extend_from_slice(&[int0, int1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[bool0]);
                                let progress = numeric_bool_12(2, values, budget);
                                calls_bool_12_numeric(progress, values)
                            }
                        },
                        Bool12State::Point3 {  } => {
                            {
                                let values = ops.numeric();
                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                let progress = numeric_bool_12(3, values, budget);
                                calls_bool_12_numeric(progress, values)
                            }
                        },
                        Bool12State::Point4 { bool0 } => {
                            {
                                let values = ops.numeric();
                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[bool0]);
                                let progress = numeric_bool_12(4, values, budget);
                                calls_bool_12_numeric(progress, values)
                            }
                        },
                    }
                }
                enum BoolFunction0State {
                    Point0 { int0: i128, bool_function0: BoolCallable },
                    Point1 { int0: i128, bool_function0: BoolCallable, bool_function1: BoolCallable },
                }
                fn calls_boolfunction_0_run(active: BoolFunction0State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        BoolFunction0State::Point0 { int0, bool_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction0Point0 { int0, bool_function0 }); }
                            *budget -= 1;
                            let bool_function1 = ops.bool_closure(data::function::BoolFunctionId(10), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                            }, vec![CallCapture::bool_function(data::graph::BoolFunctionLocalId(0), bool_function0.clone()), CallCapture::int(data::graph::IntLocalId(1), int0)]);
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction0Point1 { int0, bool_function0, bool_function1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::BoolFunction { value: bool_function1 }
                            }
                        },
                        BoolFunction0State::Point1 { int0, bool_function0, bool_function1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction0Point1 { int0, bool_function0, bool_function1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::BoolFunction { value: bool_function1 }
                            }
                        },
                    }
                }
                enum BoolFunction1State {
                    Point0 { int0: i128 },
                    Point2 { int0: i128 },
                    Point3 { int0: i128, bool_function0: BoolCallable },
                }
                fn calls_boolfunction_1_run(active: BoolFunction1State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        BoolFunction1State::Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction1Point0 { int0 }); }
                            *budget -= 1;
                            let int1 = int0 + 1_i128;
                            if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(1)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], ..CallValues::default() }) }; }
                            FunctionStep::Canonical { target: data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(1)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], ..CallValues::default() }) }
                        },
                        BoolFunction1State::Point2 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction1Point2 { int0 }); }
                            *budget -= 1;
                            let bool_function0 = ops.bool_closure(data::function::BoolFunctionId(11), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                            }, vec![CallCapture::int(data::graph::IntLocalId(1), int0)]);
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction1Point3 { int0, bool_function0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::BoolFunction { value: bool_function0 }
                            }
                        },
                        BoolFunction1State::Point3 { int0, bool_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction1Point3 { int0, bool_function0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::BoolFunction { value: bool_function0 }
                            }
                        },
                    }
                }
                enum BoolFunction2State {
                    Point0 { bool0: bool, int0: i128 },
                }
                fn calls_boolfunction_2_run(active: BoolFunction2State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        BoolFunction2State::Point0 { bool0, int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction2Point0 { bool0, int0 }); }
                            {
                                FunctionStep::BoolFunctionTail { callee: FunctionState::BoolFunction3Point0 { bool0, int0 } }
                            }
                        },
                    }
                }
                enum BoolFunction3State {
                    Point0 { bool0: bool, int0: i128 },
                    Point1 { bool0: bool, int0: i128, bool_function0: BoolCallable },
                }
                fn calls_boolfunction_3_run(active: BoolFunction3State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        BoolFunction3State::Point0 { bool0, int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction3Point0 { bool0, int0 }); }
                            *budget -= 1;
                            let bool_function0 = ops.bool_closure(data::function::BoolFunctionId(12), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                            }, vec![CallCapture::bool(data::graph::BoolLocalId(0), bool0), CallCapture::int(data::graph::IntLocalId(1), int0)]);
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction3Point1 { bool0, int0, bool_function0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::BoolFunction { value: bool_function0 }
                            }
                        },
                        BoolFunction3State::Point1 { bool0, int0, bool_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction3Point1 { bool0, int0, bool_function0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::BoolFunction { value: bool_function0 }
                            }
                        },
                    }
                }
                fn calls_entry_0(target: data::function::BoolFunctionId, captures: &CallCaptureInputs<'_>, inputs: (i128,)) -> Option<FunctionState> {
                    let (argument0,) = inputs;
                    match target.0 {
                        0 => Some(FunctionState::Bool0Point0 { int0: argument0 }),
                        6 => Some(FunctionState::Bool6Point0 { int0: argument0 }),
                        9 => Some(FunctionState::Bool9Point0 { int0: argument0 }),
                        10 => Some(FunctionState::Bool10Point0 { int0: argument0, bool_function0: captures.bool_function(data::graph::BoolFunctionLocalId(0))?, int1: captures.int(data::graph::IntLocalId(1))? }),
                        11 => Some(FunctionState::Bool11Point0 { int0: argument0, int1: captures.int(data::graph::IntLocalId(1))? }),
                        12 => Some(FunctionState::Bool12Point0 { int0: argument0, bool0: captures.bool(data::graph::BoolLocalId(0))?, int1: captures.int(data::graph::IntLocalId(1))? }),
                        _ => None,
                    }
                }
                fn calls_entry_1(target: data::function::BoolFunctionId, _captures: &CallCaptureInputs<'_>, inputs: (bool,)) -> Option<FunctionState> {
                    let (argument0,) = inputs;
                    match target.0 {
                        7 => Some(FunctionState::Bool7Point0 { bool0: argument0 }),
                        _ => None,
                    }
                }
                fn calls_bool_1_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool1Point0 { int0: values.int(0)?, int1: values.int(1)? },
                        1 => FunctionState::Bool1Point1 { int0: values.int(0)?, int1: values.int(1)?, bool_function0: values.bool_function(0)? },
                        2 => {
                            if !matches!(values.bool_function_target(1)?, data::function::BoolFunctionId(0) | data::function::BoolFunctionId(6) | data::function::BoolFunctionId(9) | data::function::BoolFunctionId(10) | data::function::BoolFunctionId(11) | data::function::BoolFunctionId(12)) { return None; }
                            FunctionState::Bool1Point2 { int0: values.int(0)?, int1: values.int(1)?, bool_function0: values.bool_function(0)?, bool_function1: values.bool_function(1)? }
                        },
                        3 => FunctionState::Bool1Point3 { int0: values.int(0)?, int1: values.int(1)?, bool_function0: values.bool_function(0)?, bool_function1: values.bool_function(1)?, bool0: values.bool(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_1_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)), point, values) { return Some(execution); }
                    let active = calls_bool_1_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_bool_3_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool3Point0 { int0: values.int(0)?, int1: values.int(1)? },
                        1 => {
                            if !matches!(values.bool_function_target(0)?, data::function::BoolFunctionId(0) | data::function::BoolFunctionId(6) | data::function::BoolFunctionId(9) | data::function::BoolFunctionId(10) | data::function::BoolFunctionId(11) | data::function::BoolFunctionId(12)) { return None; }
                            FunctionState::Bool3Point1 { int0: values.int(0)?, int1: values.int(1)?, bool_function0: values.bool_function(0)? }
                        },
                        2 => FunctionState::Bool3Point2 { int0: values.int(0)?, int1: values.int(1)?, bool_function0: values.bool_function(0)?, bool0: values.bool(0)? },
                        3 => FunctionState::Bool3Point3 { int0: values.int(0)?, int1: values.int(1)?, bool_function0: values.bool_function(0)?, bool0: values.bool(0)?, bool1: values.bool(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_3_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(3)), point, values) { return Some(execution); }
                    let active = calls_bool_3_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_bool_5_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool5Point0 { bool0: values.bool(0)?, int0: values.int(0)?, int1: values.int(1)? },
                        1 => {
                            if !matches!(values.bool_function_target(0)?, data::function::BoolFunctionId(0) | data::function::BoolFunctionId(6) | data::function::BoolFunctionId(9) | data::function::BoolFunctionId(10) | data::function::BoolFunctionId(11) | data::function::BoolFunctionId(12)) { return None; }
                            FunctionState::Bool5Point1 { bool0: values.bool(0)?, int0: values.int(0)?, int1: values.int(1)?, bool_function0: values.bool_function(0)? }
                        },
                        2 => FunctionState::Bool5Point2 { bool0: values.bool(0)?, int0: values.int(0)?, int1: values.int(1)?, bool_function0: values.bool_function(0)?, bool1: values.bool(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_5_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(5)), point, values) { return Some(execution); }
                    let active = calls_bool_5_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_bool_12_numeric(progress: data::compiled::CompiledProgress, values: &data::compiled::numeric::NumericValues) -> FunctionStep {
                    const STATES: [fn(&data::compiled::numeric::NumericValues) -> FunctionState; 5] = [
                        |values| FunctionState::Bool12Point0 { int0: values.ints[0], int1: values.ints[1], bool0: values.bools[0] },
                        |values| FunctionState::Bool12Point1 { int0: values.ints[0], int1: values.ints[1] },
                        |values| FunctionState::Bool12Point2 { int0: values.ints[0], int1: values.ints[1], bool0: values.bools[0] },
                        |_values| FunctionState::Bool12Point3 {  },
                        |values| FunctionState::Bool12Point4 { bool0: values.bools[0] },
                    ];
                    const RETURNS: [fn(&data::compiled::numeric::NumericValues) -> FunctionStep; 2] = [
                        |values| FunctionStep::Bool { value: values.bools[0] },
                        |values| FunctionStep::Bool { value: values.bools[0] },
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
                            ];
                            FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(12)), point: POINTS[point], values: Box::new(CallValues { ints: values.ints.iter().copied().map(Into::into).collect(), bools: values.bools.clone(), ..CallValues::default() }) }
                        },
                        data::compiled::CompiledProgress::Complete(exit) => RETURNS[exit.0](values),
                    }
                }
                [calls_bool_1_start, calls_bool_3_start, calls_bool_5_start]
            };
            const CALL_GROUP_4: [data::compiled::calls::CallStart; 4] = {
                use data::compiled::calls::{CallCapture, CallExecution, CallInputs, CallOps, CallOutput, CallProgress, CallStorage, CallValues, IntCallable};
                enum FunctionState {
                    IntFunction0Point0 { int0: i128 },
                    IntFunction0Point1 { int0: i128, int_function0: IntCallable },
                    IntFunction1Point0 { bool0: bool },
                    IntFunction1Point1 {  },
                    IntFunction1Point2 { int_function0: IntCallable },
                    IntFunction1Point3 {  },
                    IntFunction1Point4 { int_function0: IntCallable },
                    IntFunction2Point0 { int0: i128 },
                    IntFunction2Point1 { int0: i128, int1: i128 },
                    IntFunction2Point2 { int0: i128 },
                    IntFunction2Point3 { int0: i128, int_function0: IntCallable },
                    IntFunction3Point0 { int_function0: IntCallable, int0: i128 },
                    IntFunction3Point1 { int_function0: IntCallable },
                    IntFunction3Point2 { int_function0: IntCallable, int0: i128 },
                    IntFunction3Point3 { int_function0: IntCallable, int0: i128, int_function1: IntCallable },
                    IntFunction3Point4 { int_function0: IntCallable, int0: i128, int_function1: IntCallable, int1: i128 },
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
                #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                enum FunctionStep {
                    Yield(FunctionState),
                    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                    IntFunctionTail { callee: FunctionState },
                    IntFunction { value: IntCallable },
                }
                struct FunctionExecution {
                    active: Option<FunctionState>,
                    pending_entry: bool,
                    integer_function_returns: Vec<IntFunctionReturn>,
                }
                impl FunctionExecution {
                    fn new(active: FunctionState) -> Self {
                        Self {
                            active: Some(active),
                            pending_entry: false,
                            integer_function_returns: Vec::new(),
                        }
                    }
                }
                impl CallExecution for FunctionExecution {
                    fn restart(&mut self, target: data::compiled::CallTarget, point: usize, values: CallInputs<'_>) -> bool {
                        if self.active.is_some() { return false; }
                        let active = match target {
                            data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(0)) => calls_intfunction_0_state(point, values),
                            data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(1)) => calls_intfunction_1_state(point, values),
                            data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(2)) => calls_intfunction_2_state(point, values),
                            data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(3)) => calls_intfunction_3_state(point, values),
                            _ => None,
                        };
                        let Some(active) = active else { return false; };
                        self.active = Some(active);
                        true
                    }
                    fn retained_bytes(&self) -> usize {
                        std::mem::size_of::<Self>() + self.integer_function_returns.capacity() * std::mem::size_of::<IntFunctionReturn>()
                    }
                    fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                        let Some(mut active) = self.active.take() else { return CallProgress::Yield(self); };
                        loop {
                            if self.pending_entry {
                                if *budget == 0 { self.active = Some(active); return CallProgress::Yield(self); }
                                *budget -= 1;
                                self.pending_entry = false;
                            }
                            match function_step(active, ops, budget) {
                                FunctionStep::Yield(active) => {
                                    self.active = Some(active);
                                    return CallProgress::Yield(self);
                                },
                                FunctionStep::IntFunctionTail { callee } => {
                                    *budget -= 1;
                                    self.pending_entry = self.integer_function_returns.is_empty() && ops.root_tail_entry();
                                    active = callee;
                                },
                                FunctionStep::IntFunction { value } => {
                                    if let Some(caller) = self.integer_function_returns.pop() {
                                        active = caller.small(value);
                                    } else {
                                        self.integer_function_returns.clear();
                                        return CallProgress::Complete { output: CallOutput::IntFunction(value), execution: self };
                                    }
                                },
                                FunctionStep::Canonical { target, point, values } => {
                                    match target {
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
                        FunctionState::IntFunction0Point0 { int0 } => calls_intfunction_0_run(IntFunction0State::Point0 { int0 }, ops, budget),
                        FunctionState::IntFunction0Point1 { int0, int_function0 } => calls_intfunction_0_run(IntFunction0State::Point1 { int0, int_function0 }, ops, budget),
                        FunctionState::IntFunction1Point0 { bool0 } => calls_intfunction_1_run(IntFunction1State::Point0 { bool0 }, ops, budget),
                        FunctionState::IntFunction1Point1 {  } => calls_intfunction_1_run(IntFunction1State::Point1 {  }, ops, budget),
                        FunctionState::IntFunction1Point2 { int_function0 } => calls_intfunction_1_run(IntFunction1State::Point2 { int_function0 }, ops, budget),
                        FunctionState::IntFunction1Point3 {  } => calls_intfunction_1_run(IntFunction1State::Point3 {  }, ops, budget),
                        FunctionState::IntFunction1Point4 { int_function0 } => calls_intfunction_1_run(IntFunction1State::Point4 { int_function0 }, ops, budget),
                        FunctionState::IntFunction2Point0 { int0 } => calls_intfunction_2_run(IntFunction2State::Point0 { int0 }, ops, budget),
                        FunctionState::IntFunction2Point1 { int0, int1 } => calls_intfunction_2_run(IntFunction2State::Point1 { int0, int1 }, ops, budget),
                        FunctionState::IntFunction2Point2 { int0 } => calls_intfunction_2_run(IntFunction2State::Point2 { int0 }, ops, budget),
                        FunctionState::IntFunction2Point3 { int0, int_function0 } => calls_intfunction_2_run(IntFunction2State::Point3 { int0, int_function0 }, ops, budget),
                        FunctionState::IntFunction3Point0 { int_function0, int0 } => calls_intfunction_3_run(IntFunction3State::Point0 { int_function0, int0 }, ops, budget),
                        FunctionState::IntFunction3Point1 { int_function0 } => calls_intfunction_3_run(IntFunction3State::Point1 { int_function0 }, ops, budget),
                        FunctionState::IntFunction3Point2 { int_function0, int0 } => calls_intfunction_3_run(IntFunction3State::Point2 { int_function0, int0 }, ops, budget),
                        FunctionState::IntFunction3Point3 { int_function0, int0, int_function1 } => calls_intfunction_3_run(IntFunction3State::Point3 { int_function0, int0, int_function1 }, ops, budget),
                        FunctionState::IntFunction3Point4 { int_function0, int0, int_function1, int1 } => calls_intfunction_3_run(IntFunction3State::Point4 { int_function0, int0, int_function1, int1 }, ops, budget),
                    }
                }
                enum IntFunction0State {
                    Point0 { int0: i128 },
                    Point1 { int0: i128, int_function0: IntCallable },
                }
                fn calls_intfunction_0_run(active: IntFunction0State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        IntFunction0State::Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction0Point0 { int0 }); }
                            *budget -= 1;
                            let int_function0 = ops.int_reference(data::function::IntFunctionId(17), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            });
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction0Point1 { int0, int_function0 }); }
                            {
                                FunctionStep::IntFunctionTail { callee: FunctionState::IntFunction3Point0 { int_function0: int_function0.clone(), int0 } }
                            }
                        },
                        IntFunction0State::Point1 { int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction0Point1 { int0, int_function0 }); }
                            {
                                FunctionStep::IntFunctionTail { callee: FunctionState::IntFunction3Point0 { int_function0: int_function0.clone(), int0 } }
                            }
                        },
                    }
                }
                enum IntFunction1State {
                    Point0 { bool0: bool },
                    Point1 {  },
                    Point2 { int_function0: IntCallable },
                    Point3 {  },
                    Point4 { int_function0: IntCallable },
                }
                fn calls_intfunction_1_run(mut active: IntFunction1State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            IntFunction1State::Point0 { bool0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction1Point0 { bool0 }); }
                                *budget -= 1;
                                active = {
                                    if bool0 { IntFunction1State::Point1 {  } } else { IntFunction1State::Point3 {  } }
                                };
                                continue;
                            },
                            IntFunction1State::Point1 {  } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction1Point1 {  }); }
                                *budget -= 1;
                                let int_function0 = ops.int_reference(data::function::IntFunctionId(17), data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                });
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction1Point2 { int_function0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntFunction { value: int_function0 }
                                };
                            },
                            IntFunction1State::Point2 { int_function0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction1Point2 { int_function0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntFunction { value: int_function0 }
                                };
                            },
                            IntFunction1State::Point3 {  } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction1Point3 {  }); }
                                *budget -= 1;
                                let int_function0 = ops.int_reference(data::function::IntFunctionId(18), data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                });
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction1Point4 { int_function0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntFunction { value: int_function0 }
                                };
                            },
                            IntFunction1State::Point4 { int_function0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction1Point4 { int_function0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntFunction { value: int_function0 }
                                };
                            },
                        }
                    }
                }
                enum IntFunction2State {
                    Point0 { int0: i128 },
                    Point1 { int0: i128, int1: i128 },
                    Point2 { int0: i128 },
                    Point3 { int0: i128, int_function0: IntCallable },
                }
                fn calls_intfunction_2_run(active: IntFunction2State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        IntFunction2State::Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction2Point0 { int0 }); }
                            *budget -= 1;
                            let int1 = int0 + 1_i128;
                            if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(2)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], ..CallValues::default() }) }; }
                            FunctionStep::Canonical { target: data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(2)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], ..CallValues::default() }) }
                        },
                        IntFunction2State::Point1 { int0, int1 } => {
                            FunctionStep::Canonical { target: data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(2)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], ..CallValues::default() }) }
                        },
                        IntFunction2State::Point2 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction2Point2 { int0 }); }
                            *budget -= 1;
                            let int_function0 = ops.int_closure(data::function::IntFunctionId(19), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            }, vec![CallCapture::int(data::graph::IntLocalId(1), int0)]);
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction2Point3 { int0, int_function0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::IntFunction { value: int_function0 }
                            }
                        },
                        IntFunction2State::Point3 { int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction2Point3 { int0, int_function0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::IntFunction { value: int_function0 }
                            }
                        },
                    }
                }
                enum IntFunction3State {
                    Point0 { int_function0: IntCallable, int0: i128 },
                    Point1 { int_function0: IntCallable },
                    Point2 { int_function0: IntCallable, int0: i128 },
                    Point3 { int_function0: IntCallable, int0: i128, int_function1: IntCallable },
                    Point4 { int_function0: IntCallable, int0: i128, int_function1: IntCallable, int1: i128 },
                }
                fn calls_intfunction_3_run(mut active: IntFunction3State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            IntFunction3State::Point0 { int_function0, int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction3Point0 { int_function0, int0 }); }
                                *budget -= 1;
                                active = {
                                    if int0 <= 0_i128 { IntFunction3State::Point1 { int_function0: int_function0.clone() } } else { IntFunction3State::Point2 { int_function0: int_function0.clone(), int0 } }
                                };
                                continue;
                            },
                            IntFunction3State::Point1 { int_function0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction3Point1 { int_function0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::IntFunction { value: int_function0 }
                                };
                            },
                            IntFunction3State::Point2 { int_function0, int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction3Point2 { int_function0, int0 }); }
                                *budget -= 1;
                                let int_function1 = ops.int_closure(data::function::IntFunctionId(20), data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                }, vec![CallCapture::int_function(data::graph::IntFunctionLocalId(0), int_function0.clone())]);
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction3Point3 { int_function0, int0, int_function1 }); }
                                *budget -= 1;
                                let int1 = int0 - 1_i128;
                                if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(3)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 2,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 2,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], int_functions: vec![int_function0, int_function1], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction3Point4 { int_function0, int0, int_function1, int1 }); }
                                *budget -= 1;
                                active = {
                                    IntFunction3State::Point0 { int_function0: int_function1.clone(), int0: int1 }
                                };
                                continue;
                            },
                            IntFunction3State::Point3 { int_function0, int0, int_function1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction3Point3 { int_function0, int0, int_function1 }); }
                                *budget -= 1;
                                let int1 = int0 - 1_i128;
                                if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(3)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 2,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 2,
                                    bool_functions: 0,
                                }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], int_functions: vec![int_function0, int_function1], ..CallValues::default() }) }; }
                                active = IntFunction3State::Point4 { int_function0, int0, int_function1, int1 };
                                continue;
                            },
                            IntFunction3State::Point4 { int_function0, int0, int_function1, int1 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction3Point4 { int_function0, int0, int_function1, int1 }); }
                                *budget -= 1;
                                active = {
                                    IntFunction3State::Point0 { int_function0: int_function1.clone(), int0: int1 }
                                };
                                continue;
                            },
                        }
                    }
                }
                fn calls_intfunction_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::IntFunction0Point0 { int0: values.int(0)? },
                        1 => FunctionState::IntFunction0Point1 { int0: values.int(0)?, int_function0: values.int_function(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_intfunction_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(0)), point, values) { return Some(execution); }
                    let active = calls_intfunction_0_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_intfunction_1_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::IntFunction1Point0 { bool0: values.bool(0)? },
                        1 => FunctionState::IntFunction1Point1 {  },
                        2 => FunctionState::IntFunction1Point2 { int_function0: values.int_function(0)? },
                        3 => FunctionState::IntFunction1Point3 {  },
                        4 => FunctionState::IntFunction1Point4 { int_function0: values.int_function(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_intfunction_1_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(1)), point, values) { return Some(execution); }
                    let active = calls_intfunction_1_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_intfunction_2_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::IntFunction2Point0 { int0: values.int(0)? },
                        1 => FunctionState::IntFunction2Point1 { int0: values.int(0)?, int1: values.int(1)? },
                        2 => FunctionState::IntFunction2Point2 { int0: values.int(0)? },
                        3 => FunctionState::IntFunction2Point3 { int0: values.int(0)?, int_function0: values.int_function(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_intfunction_2_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(2)), point, values) { return Some(execution); }
                    let active = calls_intfunction_2_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_intfunction_3_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::IntFunction3Point0 { int_function0: values.int_function(0)?, int0: values.int(0)? },
                        1 => FunctionState::IntFunction3Point1 { int_function0: values.int_function(0)? },
                        2 => FunctionState::IntFunction3Point2 { int_function0: values.int_function(0)?, int0: values.int(0)? },
                        3 => FunctionState::IntFunction3Point3 { int_function0: values.int_function(0)?, int0: values.int(0)?, int_function1: values.int_function(1)? },
                        4 => FunctionState::IntFunction3Point4 { int_function0: values.int_function(0)?, int0: values.int(0)?, int_function1: values.int_function(1)?, int1: values.int(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_intfunction_3_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(3)), point, values) { return Some(execution); }
                    let active = calls_intfunction_3_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                [calls_intfunction_0_start, calls_intfunction_1_start, calls_intfunction_2_start, calls_intfunction_3_start]
            };
            const CALL_GROUP_5: [data::compiled::calls::CallStart; 4] = {
                use data::compiled::calls::{BoolCallable, CallCapture, CallExecution, CallInputs, CallOps, CallOutput, CallProgress, CallStorage, CallValues};
                enum FunctionState {
                    BoolFunction0Point0 { int0: i128, bool_function0: BoolCallable },
                    BoolFunction0Point1 { int0: i128, bool_function0: BoolCallable, bool_function1: BoolCallable },
                    BoolFunction1Point0 { int0: i128 },
                    BoolFunction1Point1 { int0: i128, int1: i128 },
                    BoolFunction1Point2 { int0: i128 },
                    BoolFunction1Point3 { int0: i128, bool_function0: BoolCallable },
                    BoolFunction2Point0 { bool0: bool, int0: i128 },
                    BoolFunction3Point0 { bool0: bool, int0: i128 },
                    BoolFunction3Point1 { bool0: bool, int0: i128, bool_function0: BoolCallable },
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
                    Yield(FunctionState),
                    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                    BoolFunctionTail { callee: FunctionState },
                    BoolFunction { value: BoolCallable },
                }
                struct FunctionExecution {
                    active: Option<FunctionState>,
                    pending_entry: bool,
                    boolean_function_returns: Vec<BoolFunctionReturn>,
                }
                impl FunctionExecution {
                    fn new(active: FunctionState) -> Self {
                        Self {
                            active: Some(active),
                            pending_entry: false,
                            boolean_function_returns: Vec::new(),
                        }
                    }
                }
                impl CallExecution for FunctionExecution {
                    fn restart(&mut self, target: data::compiled::CallTarget, point: usize, values: CallInputs<'_>) -> bool {
                        if self.active.is_some() { return false; }
                        let active = match target {
                            data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(0)) => calls_boolfunction_0_state(point, values),
                            data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(1)) => calls_boolfunction_1_state(point, values),
                            data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(2)) => calls_boolfunction_2_state(point, values),
                            data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(3)) => calls_boolfunction_3_state(point, values),
                            _ => None,
                        };
                        let Some(active) = active else { return false; };
                        self.active = Some(active);
                        true
                    }
                    fn retained_bytes(&self) -> usize {
                        std::mem::size_of::<Self>() + self.boolean_function_returns.capacity() * std::mem::size_of::<BoolFunctionReturn>()
                    }
                    fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                        let Some(mut active) = self.active.take() else { return CallProgress::Yield(self); };
                        loop {
                            if self.pending_entry {
                                if *budget == 0 { self.active = Some(active); return CallProgress::Yield(self); }
                                *budget -= 1;
                                self.pending_entry = false;
                            }
                            match function_step(active, ops, budget) {
                                FunctionStep::Yield(active) => {
                                    self.active = Some(active);
                                    return CallProgress::Yield(self);
                                },
                                FunctionStep::BoolFunctionTail { callee } => {
                                    *budget -= 1;
                                    self.pending_entry = self.boolean_function_returns.is_empty() && ops.root_tail_entry();
                                    active = callee;
                                },
                                FunctionStep::BoolFunction { value } => {
                                    if let Some(caller) = self.boolean_function_returns.pop() {
                                        active = caller.small(value);
                                    } else {
                                        self.boolean_function_returns.clear();
                                        return CallProgress::Complete { output: CallOutput::BoolFunction(value), execution: self };
                                    }
                                },
                                FunctionStep::Canonical { target, point, values } => {
                                    match target {
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
                        FunctionState::BoolFunction0Point0 { int0, bool_function0 } => calls_boolfunction_0_run(BoolFunction0State::Point0 { int0, bool_function0 }, ops, budget),
                        FunctionState::BoolFunction0Point1 { int0, bool_function0, bool_function1 } => calls_boolfunction_0_run(BoolFunction0State::Point1 { int0, bool_function0, bool_function1 }, ops, budget),
                        FunctionState::BoolFunction1Point0 { int0 } => calls_boolfunction_1_run(BoolFunction1State::Point0 { int0 }, ops, budget),
                        FunctionState::BoolFunction1Point1 { int0, int1 } => calls_boolfunction_1_run(BoolFunction1State::Point1 { int0, int1 }, ops, budget),
                        FunctionState::BoolFunction1Point2 { int0 } => calls_boolfunction_1_run(BoolFunction1State::Point2 { int0 }, ops, budget),
                        FunctionState::BoolFunction1Point3 { int0, bool_function0 } => calls_boolfunction_1_run(BoolFunction1State::Point3 { int0, bool_function0 }, ops, budget),
                        FunctionState::BoolFunction2Point0 { bool0, int0 } => calls_boolfunction_2_run(BoolFunction2State::Point0 { bool0, int0 }, ops, budget),
                        FunctionState::BoolFunction3Point0 { bool0, int0 } => calls_boolfunction_3_run(BoolFunction3State::Point0 { bool0, int0 }, ops, budget),
                        FunctionState::BoolFunction3Point1 { bool0, int0, bool_function0 } => calls_boolfunction_3_run(BoolFunction3State::Point1 { bool0, int0, bool_function0 }, ops, budget),
                    }
                }
                enum BoolFunction0State {
                    Point0 { int0: i128, bool_function0: BoolCallable },
                    Point1 { int0: i128, bool_function0: BoolCallable, bool_function1: BoolCallable },
                }
                fn calls_boolfunction_0_run(active: BoolFunction0State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        BoolFunction0State::Point0 { int0, bool_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction0Point0 { int0, bool_function0 }); }
                            *budget -= 1;
                            let bool_function1 = ops.bool_closure(data::function::BoolFunctionId(10), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                            }, vec![CallCapture::bool_function(data::graph::BoolFunctionLocalId(0), bool_function0.clone()), CallCapture::int(data::graph::IntLocalId(1), int0)]);
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction0Point1 { int0, bool_function0, bool_function1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::BoolFunction { value: bool_function1 }
                            }
                        },
                        BoolFunction0State::Point1 { int0, bool_function0, bool_function1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction0Point1 { int0, bool_function0, bool_function1 }); }
                            *budget -= 1;
                            {
                                FunctionStep::BoolFunction { value: bool_function1 }
                            }
                        },
                    }
                }
                enum BoolFunction1State {
                    Point0 { int0: i128 },
                    Point1 { int0: i128, int1: i128 },
                    Point2 { int0: i128 },
                    Point3 { int0: i128, bool_function0: BoolCallable },
                }
                fn calls_boolfunction_1_run(active: BoolFunction1State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        BoolFunction1State::Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction1Point0 { int0 }); }
                            *budget -= 1;
                            let int1 = int0 + 1_i128;
                            if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(1)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], ..CallValues::default() }) }; }
                            FunctionStep::Canonical { target: data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(1)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], ..CallValues::default() }) }
                        },
                        BoolFunction1State::Point1 { int0, int1 } => {
                            FunctionStep::Canonical { target: data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(1)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], ..CallValues::default() }) }
                        },
                        BoolFunction1State::Point2 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction1Point2 { int0 }); }
                            *budget -= 1;
                            let bool_function0 = ops.bool_closure(data::function::BoolFunctionId(11), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                            }, vec![CallCapture::int(data::graph::IntLocalId(1), int0)]);
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction1Point3 { int0, bool_function0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::BoolFunction { value: bool_function0 }
                            }
                        },
                        BoolFunction1State::Point3 { int0, bool_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction1Point3 { int0, bool_function0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::BoolFunction { value: bool_function0 }
                            }
                        },
                    }
                }
                enum BoolFunction2State {
                    Point0 { bool0: bool, int0: i128 },
                }
                fn calls_boolfunction_2_run(active: BoolFunction2State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        BoolFunction2State::Point0 { bool0, int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction2Point0 { bool0, int0 }); }
                            {
                                FunctionStep::BoolFunctionTail { callee: FunctionState::BoolFunction3Point0 { bool0, int0 } }
                            }
                        },
                    }
                }
                enum BoolFunction3State {
                    Point0 { bool0: bool, int0: i128 },
                    Point1 { bool0: bool, int0: i128, bool_function0: BoolCallable },
                }
                fn calls_boolfunction_3_run(active: BoolFunction3State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        BoolFunction3State::Point0 { bool0, int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction3Point0 { bool0, int0 }); }
                            *budget -= 1;
                            let bool_function0 = ops.bool_closure(data::function::BoolFunctionId(12), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                            }, vec![CallCapture::bool(data::graph::BoolLocalId(0), bool0), CallCapture::int(data::graph::IntLocalId(1), int0)]);
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction3Point1 { bool0, int0, bool_function0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::BoolFunction { value: bool_function0 }
                            }
                        },
                        BoolFunction3State::Point1 { bool0, int0, bool_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction3Point1 { bool0, int0, bool_function0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::BoolFunction { value: bool_function0 }
                            }
                        },
                    }
                }
                fn calls_boolfunction_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::BoolFunction0Point0 { int0: values.int(0)?, bool_function0: values.bool_function(0)? },
                        1 => FunctionState::BoolFunction0Point1 { int0: values.int(0)?, bool_function0: values.bool_function(0)?, bool_function1: values.bool_function(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_boolfunction_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(0)), point, values) { return Some(execution); }
                    let active = calls_boolfunction_0_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_boolfunction_1_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::BoolFunction1Point0 { int0: values.int(0)? },
                        1 => FunctionState::BoolFunction1Point1 { int0: values.int(0)?, int1: values.int(1)? },
                        2 => FunctionState::BoolFunction1Point2 { int0: values.int(0)? },
                        3 => FunctionState::BoolFunction1Point3 { int0: values.int(0)?, bool_function0: values.bool_function(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_boolfunction_1_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(1)), point, values) { return Some(execution); }
                    let active = calls_boolfunction_1_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_boolfunction_2_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::BoolFunction2Point0 { bool0: values.bool(0)?, int0: values.int(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_boolfunction_2_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(2)), point, values) { return Some(execution); }
                    let active = calls_boolfunction_2_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_boolfunction_3_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::BoolFunction3Point0 { bool0: values.bool(0)?, int0: values.int(0)? },
                        1 => FunctionState::BoolFunction3Point1 { bool0: values.bool(0)?, int0: values.int(0)?, bool_function0: values.bool_function(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_boolfunction_3_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(3)), point, values) { return Some(execution); }
                    let active = calls_boolfunction_3_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                [calls_boolfunction_0_start, calls_boolfunction_1_start, calls_boolfunction_2_start, calls_boolfunction_3_start]
            };

            fn string_int_8(
                point: usize,
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                    2
                ] = [
                    |values, budget| CompiledResume::Exit(string_int_8_entry((values.ints[0], values.ints[1], values.ints[2],), values, budget)),
                    string_int_8_resume_1,
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
                inputs: (i128, i128, i128,),
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (b0_i0, b0_i1, b0_i2,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return data::compiled::CompiledProgress::Yield(0);
                }
                *budget -= 1;
                let b0_s0 = data::compiled::string::StringRange::literal("canonical caller");
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0]);
                    return data::compiled::CompiledProgress::Yield(1);
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
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
                let (b0_i0, b0_i1, b0_i2, b0_s0,) = (values.ints[0], values.ints[1], values.ints[2], values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b0_s0]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn numeric_bool_6(
                point: usize,
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::numeric::NumericValues, &mut usize) -> CompiledResume;
                    5
                ] = [
                    |values, budget| CompiledResume::Exit(numeric_bool_6_entry((values.ints[0],), values, budget)),
                    numeric_bool_6_resume_1,
                    numeric_bool_6_resume_2,
                    numeric_bool_6_resume_3,
                    numeric_bool_6_resume_4,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn numeric_bool_6_entry(
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
                if b0_i0 <= 0_i128 {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(1);
                    }
                    *budget -= 1;
                    let b1_v0 = true;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0]);
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0]);
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
                    let b2_i1 = b2_i0 - 1_i128;
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

            fn numeric_bool_6_resume_1(
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
                let b1_v0 = true;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b1_v0]);
                CompiledResume::Next(2)
            }

            fn numeric_bool_6_resume_2(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_v0,) = (values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b1_v0]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn numeric_bool_6_resume_3(
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
                let b2_i1 = b2_i0 - 1_i128;
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

            fn numeric_bool_6_resume_4(
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

            fn numeric_bool_9(
                point: usize,
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::numeric::NumericValues, &mut usize) -> CompiledResume;
                    5
                ] = [
                    |values, budget| CompiledResume::Exit(numeric_bool_9_entry((values.ints[0],), values, budget)),
                    numeric_bool_9_resume_1,
                    numeric_bool_9_resume_2,
                    numeric_bool_9_resume_3,
                    numeric_bool_9_resume_4,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn numeric_bool_9_entry(
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
                if b0_i0 <= 0_i128 {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(1);
                    }
                    *budget -= 1;
                    let b1_v0 = false;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0]);
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0]);
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
                    let b2_i1 = b2_i0 - 1_i128;
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

            fn numeric_bool_9_resume_1(
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
                let b1_v0 = false;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b1_v0]);
                CompiledResume::Next(2)
            }

            fn numeric_bool_9_resume_2(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b1_v0,) = (values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b1_v0]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn numeric_bool_9_resume_3(
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
                let b2_i1 = b2_i0 - 1_i128;
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

            fn numeric_bool_9_resume_4(
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

            fn numeric_bool_12(
                point: usize,
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::numeric::NumericValues, &mut usize) -> CompiledResume;
                    5
                ] = [
                    |values, budget| CompiledResume::Exit(numeric_bool_12_entry((values.ints[0], values.ints[1], values.bools[0],), values, budget)),
                    numeric_bool_12_resume_1,
                    numeric_bool_12_resume_2,
                    numeric_bool_12_resume_3,
                    numeric_bool_12_resume_4,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn numeric_bool_12_entry(
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
                if b0_v0 {
                    let (b1_i0, b1_i1,) = (b0_i0, b0_i1,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(1);
                    }
                    *budget -= 1;
                    let b1_v0 = b1_i0 > b1_i1;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0]);
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                } else {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(3);
                    }
                    *budget -= 1;
                    let b2_v0 = false;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b2_v0]);
                        return data::compiled::CompiledProgress::Yield(4);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b2_v0]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                }
            }

            fn numeric_bool_12_resume_1(
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
                let b1_v0 = b1_i0 > b1_i1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b1_v0]);
                CompiledResume::Next(2)
            }

            fn numeric_bool_12_resume_2(
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

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b1_v0]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn numeric_bool_12_resume_3(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let () = ();
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                let b2_v0 = false;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b2_v0]);
                CompiledResume::Next(4)
            }

            fn numeric_bool_12_resume_4(
                values: &mut data::compiled::numeric::NumericValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b2_v0,) = (values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b2_v0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b2_v0]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }
            data::compiled::CompiledFunctions {
                ints: data::Storage::Static(&[
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(8),
                        implementation: data::compiled::CompiledImplementation::String(data::compiled::StringImplementation {
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
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
                                    block: data::graph::BlockId(0),
                                    instruction: 1,
                                    ints: 3,
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
                ]),
                bools: data::Storage::Static(&[
                    data::compiled::CompiledFunction {
                        function: data::function::BoolFunctionId(6),
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
                            run: numeric_bool_6,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::BoolFunctionId(9),
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
                            run: numeric_bool_9,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::BoolFunctionId(12),
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
                            run: numeric_bool_12,
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
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(0))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "capture_chain", data::source::SourceSpan::new(367, 381)),
                                },
                                data::compiled::CallContract {
                                    point: 1,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    target: data::compiled::CallContractTarget::IntValue(data::graph::IntFunctionLocalId(0)),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "capture_chain", data::source::SourceSpan::new(384, 400)),
                                },
                            ]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 2,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_0[0],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(1)),
                        implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                            root: true,
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
                                    int_functions: 1,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 2,
                                    ints: 2,
                                    bools: 1,
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
                                    ints: 3,
                                    bools: 1,
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
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[
                                data::compiled::CallContract {
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
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(1))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "dynamic_target", data::source::SourceSpan::new(629, 643)),
                                },
                                data::compiled::CallContract {
                                    point: 1,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    target: data::compiled::CallContractTarget::IntValue(data::graph::IntFunctionLocalId(0)),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "dynamic_target", data::source::SourceSpan::new(646, 662)),
                                },
                            ]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 3,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_0[1],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)),
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
                                    instruction: 2,
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
                            calls: data::Storage::Static(&[
                                data::compiled::CallContract {
                                    point: 0,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(9))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "nested", data::source::SourceSpan::new(818, 833)),
                                },
                            ]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 2,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_1[0],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(4)),
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
                                    instruction: 2,
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
                            calls: data::Storage::Static(&[
                                data::compiled::CallContract {
                                    point: 0,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(13))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "big_return", data::source::SourceSpan::new(1814, 1829)),
                                },
                            ]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 2,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_1[1],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(6)),
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
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 3,
                                    ints: 4,
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
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[
                                data::compiled::CallContract {
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
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(2))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "producer_suffix_int", data::source::SourceSpan::new(2190, 2208)),
                                },
                                data::compiled::CallContract {
                                    point: 1,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    target: data::compiled::CallContractTarget::IntValue(data::graph::IntFunctionLocalId(0)),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "producer_suffix_int", data::source::SourceSpan::new(2211, 2227)),
                                },
                            ]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 3,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_0[2],
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
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 3,
                                    ints: 4,
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
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
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
                                    site: data::source::HostCallSite::from_static("example", "reuse_callback", data::source::SourceSpan::new(2719, 2729)),
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
                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(15)),
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
                                    point: 3,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_1[2],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(8)),
                        implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                            root: false,
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
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
                                    block: data::graph::BlockId(0),
                                    instruction: 1,
                                    ints: 3,
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
                            locals: data::Storage::Static(&[
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[]),
                            tails: data::Storage::Static(&[
                                data::compiled::TailContract {
                                    point: 1,
                                    target: data::compiled::CallTarget::Int(data::function::IntFunctionId(16)),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "repeated_roots", data::source::SourceSpan::new(3098, 3158)),
                                },
                            ]),
                            start: CALL_GROUP_1[3],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(9)),
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
                                    block: data::graph::BlockId(2),
                                    instruction: 2,
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
                                    instruction: 3,
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
                            ]),
                            calls: data::Storage::Static(&[
                                data::compiled::CallContract {
                                    point: 4,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(9))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "non_tail", data::source::SourceSpan::new(750, 769)),
                                },
                            ]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 2,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                },
                                data::compiled::ReturnContract {
                                    point: 6,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_1[4],
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
                                    block: data::graph::BlockId(0),
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
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 1,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_1[5],
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
                                    ints: 1,
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
                                    instruction: 1,
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
                                    instruction: 2,
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
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 3,
                                    ints: 4,
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
                                    data::graph::ParamLocal::IntFunction {
                                        local: data::graph::IntFunctionLocalId(1),
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
                                    data::graph::ParamLocal::IntFunction {
                                        local: data::graph::IntFunctionLocalId(1),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                    },
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
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
                                    data::graph::ParamLocal::IntFunction {
                                        local: data::graph::IntFunctionLocalId(1),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                    },
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
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
                                    site: data::source::HostCallSite::from_static("example", "<anonymous:3>", data::source::SourceSpan::new(1518, 1530)),
                                },
                                data::compiled::CallContract {
                                    point: 1,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    target: data::compiled::CallContractTarget::IntValue(data::graph::IntFunctionLocalId(1)),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "<anonymous:3>", data::source::SourceSpan::new(1533, 1543)),
                                },
                            ]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 3,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_1[6],
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
                            start: CALL_GROUP_1[7],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(15)),
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
                                    block: data::graph::BlockId(0),
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
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 1,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_1[8],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(16)),
                        implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                            root: true,
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 3,
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
                                    strings: 1,
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
                                    strings: 1,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 2,
                                    ints: 5,
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
                                    instruction: 3,
                                    ints: 6,
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
                            locals: data::Storage::Static(&[
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[
                                data::compiled::CallContract {
                                    point: 3,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(7))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "repeat_canonical", data::source::SourceSpan::new(2944, 2973)),
                                },
                            ]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 1,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_1[9],
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
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 0,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_1[10],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(18)),
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
                            start: CALL_GROUP_1[11],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(19)),
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
                                    block: data::graph::BlockId(0),
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
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 1,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_1[12],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(20)),
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
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
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
                                    site: data::source::HostCallSite::from_static("example", "<anonymous:0>", data::source::SourceSpan::new(183, 198)),
                                },
                            ]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 2,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_1[13],
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
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 2,
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
                            ]),
                            locals: data::Storage::Static(&[
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[
                                data::compiled::CallContract {
                                    point: 0,
                                    output: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(6))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "mutual", data::source::SourceSpan::new(1085, 1096)),
                                },
                            ]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 2,
                                    value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_2[0],
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
                                    int_functions: 0,
                                    bool_functions: 1,
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
                                    bool_functions: 2,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 3,
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
                            ]),
                            locals: data::Storage::Static(&[
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::BoolFunction {
                                        local: data::graph::BoolFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Bool,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                        },
                                    },
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::BoolFunction {
                                        local: data::graph::BoolFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Bool,
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
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::BoolFunction {
                                        local: data::graph::BoolFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Bool,
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
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(0))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Bool,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "callable_captures", data::source::SourceSpan::new(1336, 1361)),
                                },
                                data::compiled::CallContract {
                                    point: 2,
                                    output: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    target: data::compiled::CallContractTarget::BoolValue(data::graph::BoolFunctionLocalId(1)),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "callable_captures", data::source::SourceSpan::new(1364, 1376)),
                                },
                            ]),
                            creations: data::Storage::Static(&[
                                data::compiled::CreationContract {
                                    point: 0,
                                    output: data::graph::ParamLocal::BoolFunction {
                                        local: data::graph::BoolFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Bool,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                        },
                                    },
                                    target: data::graph::FunctionTarget::Bool(data::function::BoolFunctionId(7)),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Bool,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                    },
                                    reference: true,
                                    captures: data::Storage::Static(&[]),
                                },
                            ]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 3,
                                    value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_3[0],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(2)),
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
                                    instruction: 3,
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
                                    int_functions: 2,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 5,
                                    ints: 4,
                                    bools: 1,
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
                                    data::graph::ParamLocal::IntFunction {
                                        local: data::graph::IntFunctionLocalId(1),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                    },
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
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
                                    data::graph::ParamLocal::IntFunction {
                                        local: data::graph::IntFunctionLocalId(1),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                    },
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
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
                                    data::graph::ParamLocal::IntFunction {
                                        local: data::graph::IntFunctionLocalId(1),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                    },
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[
                                data::compiled::CallContract {
                                    point: 2,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    target: data::compiled::CallContractTarget::IntValue(data::graph::IntFunctionLocalId(1)),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "aliases", data::source::SourceSpan::new(1548, 1563)),
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
                                            source: data::graph::IntLocalId(0),
                                        },
                                    ]),
                                },
                                data::compiled::CreationContract {
                                    point: 1,
                                    output: data::graph::ParamLocal::IntFunction {
                                        local: data::graph::IntFunctionLocalId(1),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                    },
                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(11)),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    },
                                    reference: false,
                                    captures: data::Storage::Static(&[
                                        data::graph::FunctionCapture::IntFunction {
                                            target: data::graph::IntFunctionLocalId(0),
                                            source: data::graph::IntFunctionLocalId(0),
                                        },
                                        data::graph::FunctionCapture::IntFunction {
                                            target: data::graph::IntFunctionLocalId(1),
                                            source: data::graph::IntFunctionLocalId(0),
                                        },
                                    ]),
                                },
                            ]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 5,
                                    value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_1[14],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(3)),
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
                                    int_functions: 0,
                                    bool_functions: 1,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 2,
                                    ints: 2,
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
                                    ints: 2,
                                    bools: 2,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 1,
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
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::BoolFunction {
                                        local: data::graph::BoolFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                        },
                                    },
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::BoolFunction {
                                        local: data::graph::BoolFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                        },
                                    },
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[
                                data::compiled::CallContract {
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
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(1))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "producer_suffix_bool", data::source::SourceSpan::new(2429, 2448)),
                                },
                                data::compiled::CallContract {
                                    point: 1,
                                    output: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    target: data::compiled::CallContractTarget::BoolValue(data::graph::BoolFunctionLocalId(0)),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "producer_suffix_bool", data::source::SourceSpan::new(2452, 2464)),
                                },
                            ]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 3,
                                    value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_3[1],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(5)),
                        implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                            root: true,
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
                                    ints: 2,
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
                                    instruction: 2,
                                    ints: 2,
                                    bools: 2,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 1,
                                },
                            ]),
                            locals: data::Storage::Static(&[
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::BoolFunction {
                                        local: data::graph::BoolFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                        },
                                    },
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[
                                data::compiled::CallContract {
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
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(2))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "bool_captures", data::source::SourceSpan::new(3684, 3718)),
                                },
                                data::compiled::CallContract {
                                    point: 1,
                                    output: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    target: data::compiled::CallContractTarget::BoolValue(data::graph::BoolFunctionLocalId(0)),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "bool_captures", data::source::SourceSpan::new(3721, 3733)),
                                },
                            ]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 2,
                                    value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_3[2],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(6)),
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
                                ]),
                                data::Storage::Static(&[]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                    value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                },
                            ]),
                            tails: data::Storage::Static(&[
                                data::compiled::TailContract {
                                    point: 4,
                                    target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(9)),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "even", data::source::SourceSpan::new(921, 935)),
                                },
                            ]),
                            start: CALL_GROUP_2[1],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(7)),
                        implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                            root: false,
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
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
                                    block: data::graph::BlockId(0),
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
                            ]),
                            locals: data::Storage::Static(&[
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 1,
                                    value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_2[2],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(9)),
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
                                ]),
                                data::Storage::Static(&[]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                    value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                },
                            ]),
                            tails: data::Storage::Static(&[
                                data::compiled::TailContract {
                                    point: 4,
                                    target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(6)),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "odd", data::source::SourceSpan::new(1023, 1038)),
                                },
                            ]),
                            start: CALL_GROUP_2[3],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(10)),
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
                                    bool_functions: 1,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 1,
                                    ints: 2,
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
                                    instruction: 2,
                                    ints: 2,
                                    bools: 2,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 1,
                                },
                            ]),
                            locals: data::Storage::Static(&[
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::BoolFunction {
                                        local: data::graph::BoolFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Bool,
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
                                                data::type_::ValueType::Bool,
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
                                                data::type_::ValueType::Bool,
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
                                    output: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    target: data::compiled::CallContractTarget::BoolValue(data::graph::BoolFunctionLocalId(0)),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "<anonymous:1>", data::source::SourceSpan::new(1232, 1256)),
                                },
                            ]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 2,
                                    value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_2[4],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(11)),
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
                                    block: data::graph::BlockId(0),
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
                            start: CALL_GROUP_2[5],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(12)),
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
                            locals: data::Storage::Static(&[
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
                                    value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                },
                                data::compiled::ReturnContract {
                                    point: 4,
                                    value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_2[6],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(0)),
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
                                    int_functions: 1,
                                    bool_functions: 0,
                                },
                            ]),
                            locals: data::Storage::Static(&[
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
                            ]),
                            calls: data::Storage::Static(&[]),
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
                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(17)),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    },
                                    reference: true,
                                    captures: data::Storage::Static(&[]),
                                },
                            ]),
                            returns: data::Storage::Static(&[]),
                            tails: data::Storage::Static(&[
                                data::compiled::TailContract {
                                    point: 1,
                                    target: data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(3)),
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
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "compose", data::source::SourceSpan::new(269, 291)),
                                },
                            ]),
                            start: CALL_GROUP_4[0],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(1)),
                        implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                            root: true,
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
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
                                    ints: 0,
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
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[]),
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
                                ]),
                                data::Storage::Static(&[]),
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
                                ]),
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[
                                data::compiled::CreationContract {
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
                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(17)),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    },
                                    reference: true,
                                    captures: data::Storage::Static(&[]),
                                },
                                data::compiled::CreationContract {
                                    point: 3,
                                    output: data::graph::ParamLocal::IntFunction {
                                        local: data::graph::IntFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                    },
                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(18)),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    },
                                    reference: true,
                                    captures: data::Storage::Static(&[]),
                                },
                            ]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 2,
                                    value: data::graph::ParamLocal::IntFunction {
                                        local: data::graph::IntFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                    },
                                },
                                data::compiled::ReturnContract {
                                    point: 4,
                                    value: data::graph::ParamLocal::IntFunction {
                                        local: data::graph::IntFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                    },
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_4[1],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(2)),
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
                                    block: data::graph::BlockId(1),
                                    instruction: 1,
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
                            ]),
                            locals: data::Storage::Static(&[
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
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[
                                data::compiled::CreationContract {
                                    point: 2,
                                    output: data::graph::ParamLocal::IntFunction {
                                        local: data::graph::IntFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                    },
                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(19)),
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
                                            source: data::graph::IntLocalId(0),
                                        },
                                    ]),
                                },
                            ]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 3,
                                    value: data::graph::ParamLocal::IntFunction {
                                        local: data::graph::IntFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                    },
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_4[2],
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
                                    int_functions: 1,
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
                                    int_functions: 1,
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
                                    int_functions: 1,
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
                                    int_functions: 2,
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
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 2,
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
                                    data::graph::ParamLocal::IntFunction {
                                        local: data::graph::IntFunctionLocalId(1),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                    },
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[
                                data::compiled::CreationContract {
                                    point: 2,
                                    output: data::graph::ParamLocal::IntFunction {
                                        local: data::graph::IntFunctionLocalId(1),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                    },
                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(20)),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    },
                                    reference: false,
                                    captures: data::Storage::Static(&[
                                        data::graph::FunctionCapture::IntFunction {
                                            target: data::graph::IntFunctionLocalId(0),
                                            source: data::graph::IntFunctionLocalId(0),
                                        },
                                    ]),
                                },
                            ]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 1,
                                    value: data::graph::ParamLocal::IntFunction {
                                        local: data::graph::IntFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                    },
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_4[3],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(0)),
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
                                    bool_functions: 1,
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
                                    bool_functions: 2,
                                },
                            ]),
                            locals: data::Storage::Static(&[
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::BoolFunction {
                                        local: data::graph::BoolFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Bool,
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
                                                data::type_::ValueType::Bool,
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
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[
                                data::compiled::CreationContract {
                                    point: 0,
                                    output: data::graph::ParamLocal::BoolFunction {
                                        local: data::graph::BoolFunctionLocalId(1),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                        },
                                    },
                                    target: data::graph::FunctionTarget::Bool(data::function::BoolFunctionId(10)),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                    },
                                    reference: false,
                                    captures: data::Storage::Static(&[
                                        data::graph::FunctionCapture::BoolFunction {
                                            target: data::graph::BoolFunctionLocalId(0),
                                            source: data::graph::BoolFunctionLocalId(0),
                                        },
                                        data::graph::FunctionCapture::Int {
                                            target: data::graph::IntLocalId(1),
                                            source: data::graph::IntLocalId(0),
                                        },
                                    ]),
                                },
                            ]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 1,
                                    value: data::graph::ParamLocal::BoolFunction {
                                        local: data::graph::BoolFunctionLocalId(1),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                        },
                                    },
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_5[0],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(1)),
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
                                    bool_functions: 1,
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
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[
                                data::compiled::CreationContract {
                                    point: 2,
                                    output: data::graph::ParamLocal::BoolFunction {
                                        local: data::graph::BoolFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                        },
                                    },
                                    target: data::graph::FunctionTarget::Bool(data::function::BoolFunctionId(11)),
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
                                    point: 3,
                                    value: data::graph::ParamLocal::BoolFunction {
                                        local: data::graph::BoolFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                        },
                                    },
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_5[1],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(2)),
                        implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                            root: true,
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
                            ]),
                            locals: data::Storage::Static(&[
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[]),
                            tails: data::Storage::Static(&[
                                data::compiled::TailContract {
                                    point: 0,
                                    target: data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(3)),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "forward_predicate", data::source::SourceSpan::new(3561, 3595)),
                                },
                            ]),
                            start: CALL_GROUP_5[2],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(3)),
                        implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                            root: true,
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
                                    bool_functions: 1,
                                },
                            ]),
                            locals: data::Storage::Static(&[
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                            ]),
                            calls: data::Storage::Static(&[]),
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
                                    target: data::graph::FunctionTarget::Bool(data::function::BoolFunctionId(12)),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                    },
                                    reference: false,
                                    captures: data::Storage::Static(&[
                                        data::graph::FunctionCapture::Bool {
                                            target: data::graph::BoolLocalId(0),
                                            source: data::graph::BoolLocalId(0),
                                        },
                                        data::graph::FunctionCapture::Int {
                                            target: data::graph::IntLocalId(1),
                                            source: data::graph::IntLocalId(0),
                                        },
                                    ]),
                                },
                            ]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 1,
                                    value: data::graph::ParamLocal::BoolFunction {
                                        local: data::graph::BoolFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                        },
                                    },
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_5[3],
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
                21..34,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                34..38,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                38..42,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
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
                        data::type_::ValueShapeId(2),
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
                    parameters: 5..6,
                    parameter_shapes: data::Storage::Static(&[
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
                    parameters: 7..8,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
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
                    parameters: 10..12,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 12..15,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 15..16,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 16..17,
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
                    parameters: 17..18,
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
                        data::graph::ParamSlot {
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
                    ]),
                },
                data::function::FunctionContract {
                    parameters: 18..19,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 19..20,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 20..21,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 21..22,
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
                    parameters: 22..26,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(5),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 26..27,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 27..28,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 28..29,
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
                    parameters: 29..30,
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
                    parameters: 30..31,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 31..33,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 33..35,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 35..37,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 37..38,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 38..41,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(2),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 41..42,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 42..43,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(2),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 43..44,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 44..45,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 45..46,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[
                        data::graph::ParamSlot {
                            local: data::graph::ParamLocal::BoolFunction {
                                local: data::graph::BoolFunctionLocalId(0),
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Bool,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                },
                            },
                            shape: data::type_::ValueShapeId(3),
                        },
                        data::graph::ParamSlot {
                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                            shape: data::type_::ValueShapeId(0),
                        },
                    ]),
                },
                data::function::FunctionContract {
                    parameters: 46..47,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[
                        data::graph::ParamSlot {
                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                            shape: data::type_::ValueShapeId(0),
                        },
                    ]),
                },
                data::function::FunctionContract {
                    parameters: 47..48,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[
                        data::graph::ParamSlot {
                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                            shape: data::type_::ValueShapeId(2),
                        },
                        data::graph::ParamSlot {
                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                            shape: data::type_::ValueShapeId(0),
                        },
                    ]),
                },
                data::function::FunctionContract {
                    parameters: 48..49,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 49..50,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(2),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 50..51,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 51..53,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(1),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 53..55,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(3),
                    ]),
                    return_: data::type_::ValueShapeId(4),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 55..56,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(4),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 56..58,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(2),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(4),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 58..60,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(2),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(4),
                    captures: data::Storage::Static(&[]),
                },
            ]),
            parameters: data::Storage::Static(&[
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                data::graph::ParamLocal::BoolFunction {
                    local: data::graph::BoolFunctionLocalId(0),
                    type_: data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Bool,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                    },
                },
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                data::type_::ValueShapeDescriptor::Int,
                data::type_::ValueShapeDescriptor::Function {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                },
                data::type_::ValueShapeDescriptor::Bool,
                data::type_::ValueShapeDescriptor::Function {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(2),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                },
                data::type_::ValueShapeDescriptor::Function {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(2),
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
                data::type_::ValueType::Bool,
                data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::Bool,
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                }),
                data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
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
            data::program::LibraryFunctionEntry {
                function: data::function::BoolFunctionId(2),
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
                function: data::function::BoolFunctionId(3),
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
                function: data::function::BoolFunctionId(4),
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
                function: data::function::BoolFunctionId(5),
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
            name: data::Text::Static("capture_chain"),
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
            name: data::Text::Static("dynamic_target"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Bool,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 1,
        },
        data::Export {
            name: data::Text::Static("nested"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 2,
        },
        data::Export {
            name: data::Text::Static("mutual"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            slot: 0,
        },
        data::Export {
            name: data::Text::Static("callable_captures"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            slot: 1,
        },
        data::Export {
            name: data::Text::Static("aliases"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            slot: 2,
        },
        data::Export {
            name: data::Text::Static("canonical"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 3,
        },
        data::Export {
            name: data::Text::Static("big_return"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 4,
        },
        data::Export {
            name: data::Text::Static("failure"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 5,
        },
        data::Export {
            name: data::Text::Static("producer_suffix_int"),
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
            name: data::Text::Static("producer_suffix_bool"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            slot: 3,
        },
        data::Export {
            name: data::Text::Static("reuse_callback"),
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
            name: data::Text::Static("repeated_roots"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 8,
        },
        data::Export {
            name: data::Text::Static("canonical_bool"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            slot: 4,
        },
        data::Export {
            name: data::Text::Static("bool_captures"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Bool,
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            slot: 5,
        },
    ]),
}
