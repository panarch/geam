data::ModuleArtifact {
    format: 12,
    program: data::ProgramTables {
        root: data::source::module_id(0),
        modules: data::Storage::Static(&[
            data::program::ExecutionModuleContext {
                module: data::Text::Static("example"),
                source_context: None,
            },
        ]),
        main: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::String(data::function::StringFunctionId(0))),
        functions: data::function::FunctionTables {
            value_returns: data::function::ValueFunctionTables {
                never_functions: data::Storage::Static(&[]),
                int_functions: data::Storage::Static(&[]),
                float_functions: data::Storage::Static(&[]),
                string_functions: data::Storage::Static(&[
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
                                        instructions: 0..5,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(1),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                elements: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            42,
                                                        ]),
                                                    }),
                                                ]),
                                                tail: None,
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[]),
                                                bindings: data::Storage::Static(&[]),
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
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(6),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(1),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
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
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::IntList,
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
                                        params: 0..0,
                                        instructions: 5..10,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(1),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                elements: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            1,
                                                        ]),
                                                    }),
                                                    data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            42,
                                                        ]),
                                                    }),
                                                ]),
                                                tail: None,
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[]),
                                                bindings: data::Storage::Static(&[]),
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
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(5),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(1),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
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
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::IntList,
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
                                        params: 0..0,
                                        instructions: 10..13,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                                                local: data::graph::ParameterListLocalId(1),
                                                type_id: data::type_::ParameterListTypeId {
                                                    list_type: data::type_::ListTypeId(1),
                                                    item: data::type_::parameter_id(0),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                elements: data::Storage::Static(&[]),
                                                tail: None,
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(3),
                                                args: data::Storage::Static(&[]),
                                                bindings: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Bool,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::ParameterList,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(4),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                                                        local: data::graph::ParameterListLocalId(1),
                                                        type_id: data::type_::ParameterListTypeId {
                                                            list_type: data::type_::ListTypeId(1),
                                                            item: data::type_::parameter_id(0),
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
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::ParameterList,
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
                                        params: 0..0,
                                        instructions: 13..50,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 0..1,
                                        instructions: 50..50,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                                                local: data::graph::ParameterListLocalId(0),
                                                type_id: data::type_::ParameterListTypeId {
                                                    list_type: data::type_::ListTypeId(1),
                                                    item: data::type_::parameter_id(0),
                                                },
                                            }),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(695, 705)),
                                            pattern_span: data::source::SourceSpan::new(706, 708),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..2,
                                        instructions: 50..50,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(643, 653)),
                                            pattern_span: data::source::SourceSpan::new(654, 661),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..3,
                                        instructions: 50..50,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(595, 605)),
                                            pattern_span: data::source::SourceSpan::new(606, 610),
                                        }),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                                            local: data::graph::ParameterListLocalId(0),
                                            type_id: data::type_::ParameterListTypeId {
                                                list_type: data::type_::ListTypeId(1),
                                                item: data::type_::parameter_id(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(8),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(5),
                                    },
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
                                            data::graph::IntLocalId(1),
                                        ])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
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
                                        }, data::graph::TypedListInstruction::Call {
                                            function: data::function::IntListFunctionId {
                                                index: 0,
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            },
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(613, 640)),
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
                                            data::graph::IntLocalId(1),
                                        ])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(false)),
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
                                        }, data::graph::TypedListInstruction::Call {
                                            function: data::function::IntListFunctionId {
                                                index: 0,
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            },
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(664, 692)),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                                                local: data::graph::ParameterListLocalId(0),
                                                type_id: data::type_::ParameterListTypeId {
                                                    list_type: data::type_::ListTypeId(1),
                                                    item: data::type_::parameter_id(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(8),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Parameter(data::type_::ParameterListTypeId {
                                            list_type: data::type_::ListTypeId(1),
                                            item: data::type_::parameter_id(0),
                                        }, data::graph::ParameterListInstruction::Empty)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                                                local: data::graph::ParameterListLocalId(1),
                                                type_id: data::type_::ParameterListTypeId {
                                                    list_type: data::type_::ListTypeId(1),
                                                    item: data::type_::parameter_id(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(8),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Parameter(data::type_::ParameterListTypeId {
                                            list_type: data::type_::ListTypeId(1),
                                            item: data::type_::parameter_id(0),
                                        }, data::graph::ParameterListInstruction::Call {
                                            function: data::function::ParameterListFunctionId {
                                                index: 0,
                                                type_id: data::type_::ParameterListTypeId {
                                                    list_type: data::type_::ListTypeId(1),
                                                    item: data::type_::parameter_id(0),
                                                },
                                            },
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                                                    local: data::graph::ParameterListLocalId(0),
                                                    type_id: data::type_::ParameterListTypeId {
                                                        list_type: data::type_::ListTypeId(1),
                                                        item: data::type_::parameter_id(0),
                                                    },
                                                }),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(711, 733)),
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
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(2),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(9),
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
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(3),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(10),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(1),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(2),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                            function: data::function::StringFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(3),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(736, 757)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(2),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(11),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(0),
                                                index: 1,
                                            },
                                            fields: data::Storage::Static(&[]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(3),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(5),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(12),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(1),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(2),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(4),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                            function: data::function::StringFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(3),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(5),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(763, 780)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Concatenate {
                                            left: data::graph::StringLocalId(0),
                                            right: data::graph::StringLocalId(1),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(3)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("failed:"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(4),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(6),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(13),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(1),
                                                index: 1,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(3)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(4)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                            function: data::function::StringFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(4),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(786, 811)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(5)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Concatenate {
                                            left: data::graph::StringLocalId(2),
                                            right: data::graph::StringLocalId(4),
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
                                                42,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(5),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(2),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(9),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(0),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(6),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(7),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(14),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(2),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(5),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(2),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
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
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                                                    data::type_::ValueType::Int,
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(15),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(6),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(7),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        ]))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(7),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    shape_id: data::type_::CustomValueShapeId(8),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(16),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(3),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                },
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(6)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                            function: data::function::StringFunctionId(2),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(7),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        shape_id: data::type_::CustomValueShapeId(8),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(817, 849)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(7)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Concatenate {
                                            left: data::graph::StringLocalId(5),
                                            right: data::graph::StringLocalId(6),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(8),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(11),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(0),
                                                index: 1,
                                            },
                                            fields: data::Storage::Static(&[]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(9),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(9),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(2),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(8),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(4),
                                                    },
                                                }),
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
                                                2,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(1),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                                                    data::type_::ValueType::Int,
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(18),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(9),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(9),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                        ]))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(10),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    shape_id: data::type_::CustomValueShapeId(10),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(19),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(3),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(1),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                },
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(8)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                            function: data::function::StringFunctionId(2),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(10),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        shape_id: data::type_::CustomValueShapeId(10),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(855, 883)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(9)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Concatenate {
                                            left: data::graph::StringLocalId(7),
                                            right: data::graph::StringLocalId(8),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(11),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(11),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(20),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(2),
                                                index: 1,
                                            },
                                            fields: data::Storage::Static(&[]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
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
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(2),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                                                    data::type_::ValueType::Int,
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(21),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(11),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(11),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                        ]))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(12),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    shape_id: data::type_::CustomValueShapeId(12),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(22),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(3),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(2),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                },
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(10)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                            function: data::function::StringFunctionId(2),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(12),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        shape_id: data::type_::CustomValueShapeId(12),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(889, 911)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(11)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Concatenate {
                                            left: data::graph::StringLocalId(9),
                                            right: data::graph::StringLocalId(10),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(12)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("done"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(13),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    shape_id: data::type_::CustomValueShapeId(13),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(23),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(3),
                                                index: 1,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(12)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(13)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                            function: data::function::StringFunctionId(2),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(13),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        shape_id: data::type_::CustomValueShapeId(13),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(917, 938)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(14)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Concatenate {
                                            left: data::graph::StringLocalId(11),
                                            right: data::graph::StringLocalId(13),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::StringLocalId(14)),
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
                                                    shape_id: data::type_::CustomValueShapeId(14),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Custom {
                                                        constructor: data::type_::CustomConstructorId {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            index: 0,
                                                        },
                                                        fields: data::Storage::Static(&[
                                                            data::graph::MatchPattern::Discard,
                                                        ]),
                                                    },
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
                                                            shape_id: data::type_::CustomValueShapeId(14),
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
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(14),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Custom {
                                                        constructor: data::type_::CustomConstructorId {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            index: 1,
                                                        },
                                                        fields: data::Storage::Static(&[]),
                                                    },
                                                ]),
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(3),
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
                                                target: data::graph::BlockId(4),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(14),
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
                                        params: 2..2,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..3,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(14),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(24),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(14),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(24),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(14),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(24),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("present:"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("missing:"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(14),
                                                },
                                            },
                                            index: 0,
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::StringLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::StringLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::StringLocalId(0)),
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
                                                    type_id: data::type_::CustomTypeId(3),
                                                    shape_id: data::type_::CustomValueShapeId(15),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Tuple(data::Storage::Static(&[
                                                        data::graph::MatchPattern::Custom {
                                                            constructor: data::type_::CustomConstructorId {
                                                                type_id: data::type_::CustomTypeId(2),
                                                                index: 0,
                                                            },
                                                            fields: data::Storage::Static(&[
                                                                data::graph::MatchPattern::Custom {
                                                                    constructor: data::type_::CustomConstructorId {
                                                                        type_id: data::type_::CustomTypeId(0),
                                                                        index: 0,
                                                                    },
                                                                    fields: data::Storage::Static(&[
                                                                        data::graph::MatchPattern::Discard,
                                                                    ]),
                                                                },
                                                            ]),
                                                        },
                                                        data::graph::MatchPattern::Discard,
                                                    ])),
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
                                                            type_id: data::type_::CustomTypeId(3),
                                                            shape_id: data::type_::CustomValueShapeId(15),
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
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    shape_id: data::type_::CustomValueShapeId(15),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Tuple(data::Storage::Static(&[
                                                        data::graph::MatchPattern::Custom {
                                                            constructor: data::type_::CustomConstructorId {
                                                                type_id: data::type_::CustomTypeId(2),
                                                                index: 0,
                                                            },
                                                            fields: data::Storage::Static(&[
                                                                data::graph::MatchPattern::Custom {
                                                                    constructor: data::type_::CustomConstructorId {
                                                                        type_id: data::type_::CustomTypeId(0),
                                                                        index: 1,
                                                                    },
                                                                    fields: data::Storage::Static(&[]),
                                                                },
                                                            ]),
                                                        },
                                                        data::graph::MatchPattern::Discard,
                                                    ])),
                                                ]),
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(3),
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
                                                target: data::graph::BlockId(4),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(3),
                                                            shape_id: data::type_::CustomValueShapeId(15),
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
                                        params: 2..2,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..3,
                                        instructions: 2..2,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    shape_id: data::type_::CustomValueShapeId(15),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Tuple(data::Storage::Static(&[
                                                        data::graph::MatchPattern::Custom {
                                                            constructor: data::type_::CustomConstructorId {
                                                                type_id: data::type_::CustomTypeId(2),
                                                                index: 1,
                                                            },
                                                            fields: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::MatchPattern::Discard,
                                                    ])),
                                                ]),
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(5),
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
                                                target: data::graph::BlockId(6),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(3),
                                                            shape_id: data::type_::CustomValueShapeId(15),
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
                                        params: 3..3,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 3..4,
                                        instructions: 3..4,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(3)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(3),
                                                shape_id: data::type_::CustomValueShapeId(15),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(25),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(3),
                                                shape_id: data::type_::CustomValueShapeId(15),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(25),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(3),
                                                shape_id: data::type_::CustomValueShapeId(15),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(25),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(3),
                                                shape_id: data::type_::CustomValueShapeId(15),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(25),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("nested:"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("empty:"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("none:"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    shape_id: data::type_::CustomValueShapeId(15),
                                                },
                                            },
                                            index: 0,
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::StringLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::StringLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::StringLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::StringLocalId(0)),
                            ]),
                        },
                    },
                ]),
                bit_array_functions: data::Storage::Static(&[]),
                utf_codepoint_functions: data::Storage::Static(&[]),
                custom_functions: data::Storage::Static(&[]),
                external_functions: data::Storage::Static(&[]),
                bool_functions: data::Storage::Static(&[]),
                nil_functions: data::Storage::Static(&[]),
                tuple_functions: data::Storage::Static(&[]),
            },
            list_returns: data::function::ListFunctionTables {
                parameter_list_functions: data::Storage::Static(&[
                    (data::function::ParameterListFunctionId {
                        index: 0,
                        type_id: data::type_::ParameterListTypeId {
                            list_type: data::type_::ListTypeId(1),
                            item: data::type_::parameter_id(0),
                        },
                    }, data::function::ExecutableFunction {
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
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                                            local: data::graph::ParameterListLocalId(0),
                                            type_id: data::type_::ParameterListTypeId {
                                                list_type: data::type_::ListTypeId(1),
                                                item: data::type_::parameter_id(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(8),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::ParameterListLocalId(0)),
                            ]),
                        },
                    }),
                ]),
                int_list_functions: data::Storage::Static(&[
                    (data::function::IntListFunctionId {
                        index: 0,
                        type_id: data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
                        },
                    }, data::function::ExecutableFunction {
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
                                            test: data::graph::BoolTest::ListLengthAtLeast {
                                                value: data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                },
                                                length: 1,
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(0),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(5),
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
                                        terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                            subject: data::graph::BoolLocalId(0),
                                            true_: data::graph::Edge {
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
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            false_: data::graph::Edge {
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
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..5,
                                        instructions: 0..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 5..6,
                                        instructions: 2..2,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
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
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 6..7,
                                        instructions: 2..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 7..8,
                                        instructions: 2..2,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
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
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
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
                                        shape: data::type_::ValueShapeId(5),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(5),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(5),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(5),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(5),
                                    },
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
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::ListIndex {
                                            list: data::graph::IntListLocalId(0),
                                            index: 0,
                                        }),
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
                                        }, data::graph::TypedListInstruction::DropFirst {
                                            list: data::graph::IntListLocalId(0),
                                            count: 1,
                                        })),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntListLocalId(1)),
                                data::function::FunctionExit::Return(data::graph::IntListLocalId(0)),
                            ]),
                        },
                    }),
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
        numeric: data::numeric::NumericFunctions::interpreted(),
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
                0..3,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                3..4,
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
            ],
            functions: data::Storage::Static(&[
                data::function::FunctionContract {
                    parameters: 0..0,
                    parameter_shapes: data::Storage::Static(&[]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 0..1,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(24),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 1..2,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(25),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 2..4,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(8),
                        data::type_::ValueShapeId(6),
                    ]),
                    return_: data::type_::ValueShapeId(8),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 4..6,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(5),
                        data::type_::ValueShapeId(6),
                    ]),
                    return_: data::type_::ValueShapeId(5),
                    captures: data::Storage::Static(&[]),
                },
            ]),
            parameters: data::Storage::Static(&[
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(1),
                        shape_id: data::type_::CustomValueShapeId(14),
                    },
                }),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(3),
                        shape_id: data::type_::CustomValueShapeId(15),
                    },
                }),
                data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                    local: data::graph::ParameterListLocalId(0),
                    type_id: data::type_::ParameterListTypeId {
                        list_type: data::type_::ListTypeId(1),
                        item: data::type_::parameter_id(0),
                    },
                }),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                    local: data::graph::IntListLocalId(0),
                    type_id: data::type_::IntListTypeId {
                        list_type: data::type_::ListTypeId(0),
                    },
                }),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
            ]),
        },
        list_types: data::type_::ListTypeTable {
            types: data::Storage::Static(&[
                data::type_::ListStorageTypeId::Int(data::type_::IntListTypeId {
                    list_type: data::type_::ListTypeId(0),
                }),
                data::type_::ListStorageTypeId::Parameter(data::type_::ParameterListTypeId {
                    list_type: data::type_::ListTypeId(1),
                    item: data::type_::parameter_id(0),
                }),
            ]),
            tuple_items: data::Storage::Static(&[]),
            function_items: data::Storage::Static(&[]),
        },
        custom_types: data::type_::CustomTypeTable {
            types: data::Storage::Static(&[
                data::type_::CustomTypeDescriptor {
                    type_: data::type_::NominalTypeMetadata {
                        package: data::Text::Static("geam"),
                        module: data::Text::Static("example"),
                        name: data::Text::Static("Option"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                    },
                    constructor_count: 2,
                    constructors: data::Storage::Static(&[
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(0),
                                index: 0,
                            },
                            name: data::Text::Static("Some"),
                            native_tag: data::Text::Static("some"),
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
                            name: data::Text::Static("None"),
                            native_tag: data::Text::Static("none"),
                            fields: data::Storage::Static(&[]),
                        },
                    ]),
                },
                data::type_::CustomTypeDescriptor {
                    type_: data::type_::NominalTypeMetadata {
                        package: data::Text::Static(""),
                        module: data::Text::Static("gleam"),
                        name: data::Text::Static("Result"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("geam"),
                                module: data::Text::Static("example"),
                                name: data::Text::Static("Option"),
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::Int,
                                ]),
                            }),
                            data::type_::TypeMetadata::String,
                        ]),
                    },
                    constructor_count: 2,
                    constructors: data::Storage::Static(&[
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(1),
                                index: 0,
                            },
                            name: data::Text::Static("Ok"),
                            native_tag: data::Text::Static("ok"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                    shape: data::type_::ValueShapeId(1),
                                    refinement: data::type_::FieldRefinement::Argument(0),
                                },
                            ]),
                        },
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(1),
                                index: 1,
                            },
                            name: data::Text::Static("Error"),
                            native_tag: data::Text::Static("error"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::String,
                                    shape: data::type_::ValueShapeId(2),
                                    refinement: data::type_::FieldRefinement::Argument(1),
                                },
                            ]),
                        },
                    ]),
                },
                data::type_::CustomTypeDescriptor {
                    type_: data::type_::NominalTypeMetadata {
                        package: data::Text::Static("geam"),
                        module: data::Text::Static("example"),
                        name: data::Text::Static("Option"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("geam"),
                                module: data::Text::Static("example"),
                                name: data::Text::Static("Option"),
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::Int,
                                ]),
                            }),
                        ]),
                    },
                    constructor_count: 2,
                    constructors: data::Storage::Static(&[
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(2),
                                index: 0,
                            },
                            name: data::Text::Static("Some"),
                            native_tag: data::Text::Static("some"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                    shape: data::type_::ValueShapeId(1),
                                    refinement: data::type_::FieldRefinement::Argument(0),
                                },
                            ]),
                        },
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(2),
                                index: 1,
                            },
                            name: data::Text::Static("None"),
                            native_tag: data::Text::Static("none"),
                            fields: data::Storage::Static(&[]),
                        },
                    ]),
                },
                data::type_::CustomTypeDescriptor {
                    type_: data::type_::NominalTypeMetadata {
                        package: data::Text::Static(""),
                        module: data::Text::Static("gleam"),
                        name: data::Text::Static("Result"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                                data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                    package: data::Text::Static("geam"),
                                    module: data::Text::Static("example"),
                                    name: data::Text::Static("Option"),
                                    arguments: data::Storage::Static(&[
                                        data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                            package: data::Text::Static("geam"),
                                            module: data::Text::Static("example"),
                                            name: data::Text::Static("Option"),
                                            arguments: data::Storage::Static(&[
                                                data::type_::TypeMetadata::Int,
                                            ]),
                                        }),
                                    ]),
                                }),
                                data::type_::TypeMetadata::Int,
                            ])),
                            data::type_::TypeMetadata::String,
                        ]),
                    },
                    constructor_count: 2,
                    constructors: data::Storage::Static(&[
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(3),
                                index: 0,
                            },
                            name: data::Text::Static("Ok"),
                            native_tag: data::Text::Static("ok"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::Tuple(data::Storage::Static(&[
                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                                        data::type_::ValueType::Int,
                                    ])),
                                    shape: data::type_::ValueShapeId(4),
                                    refinement: data::type_::FieldRefinement::Argument(0),
                                },
                            ]),
                        },
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(3),
                                index: 1,
                            },
                            name: data::Text::Static("Error"),
                            native_tag: data::Text::Static("error"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::String,
                                    shape: data::type_::ValueShapeId(2),
                                    refinement: data::type_::FieldRefinement::Argument(1),
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
                    name: data::Text::Static("Option"),
                    publicity: data::type_::CustomTypePublicity::Private,
                    opaque: false,
                    parameters: 1,
                    constructors: data::Storage::Static(&[
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Some"),
                            fields: data::Storage::Static(&[
                                data::type_::FieldDefinition {
                                    label: None,
                                    type_: data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                                },
                            ]),
                        },
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("None"),
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
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(0)),
                data::type_::ValueShapeDescriptor::String,
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(1)),
                data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                    data::type_::ValueShapeId(3),
                    data::type_::ValueShapeId(0),
                ])),
                data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(0)),
                data::type_::ValueShapeDescriptor::Bool,
                data::type_::ValueShapeDescriptor::Parameter(data::type_::parameter_id(0)),
                data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(7)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(2)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(3)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(4)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(5)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(6)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(7)),
                data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                    data::type_::ValueShapeId(14),
                    data::type_::ValueShapeId(0),
                ])),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(8)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(9)),
                data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                    data::type_::ValueShapeId(17),
                    data::type_::ValueShapeId(0),
                ])),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(10)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(11)),
                data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                    data::type_::ValueShapeId(20),
                    data::type_::ValueShapeId(0),
                ])),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(12)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(13)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(14)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(15)),
            ]),
            shape_types: data::Storage::Static(&[
                data::type_::ValueType::Int,
                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                data::type_::ValueType::String,
                data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                data::type_::ValueType::Tuple(data::Storage::Static(&[
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                    data::type_::ValueType::Int,
                ])),
                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                data::type_::ValueType::Bool,
                data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                data::type_::ValueType::List(data::type_::ListTypeId(1)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                data::type_::ValueType::Tuple(data::Storage::Static(&[
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                    data::type_::ValueType::Int,
                ])),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(3)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                data::type_::ValueType::Tuple(data::Storage::Static(&[
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                    data::type_::ValueType::Int,
                ])),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(3)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                data::type_::ValueType::Tuple(data::Storage::Static(&[
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                    data::type_::ValueType::Int,
                ])),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(3)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(3)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(3)),
            ]),
            custom_shapes: data::Storage::Static(&[
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(0),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Any,
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(2),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(1),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Any,
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(0),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(1),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(9),
                        data::type_::ValueShapeId(2),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(0),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(1),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(1),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(11),
                        data::type_::ValueShapeId(2),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(1),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(1),
                        data::type_::ValueShapeId(2),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(1),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(2),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(9),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(3),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(15),
                        data::type_::ValueShapeId(2),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(2),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(11),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(3),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(18),
                        data::type_::ValueShapeId(2),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(2),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(1),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(1),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(3),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(21),
                        data::type_::ValueShapeId(2),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(3),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(4),
                        data::type_::ValueShapeId(2),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(1),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(1),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(1),
                        data::type_::ValueShapeId(2),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Any,
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(3),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(4),
                        data::type_::ValueShapeId(2),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Any,
                },
            ]),
        },
    },
    entries: data::program::LibraryFunctionEntries {
        ints: data::Storage::Static(&[]),
        floats: data::Storage::Static(&[]),
        strings: data::Storage::Static(&[
            data::program::LibraryFunctionEntry {
                function: data::function::StringFunctionId(0),
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
    exports: data::Storage::Static(&[
        data::Export {
            name: data::Text::Static("main"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::String),
            },
            slot: 0,
        },
    ]),
}
