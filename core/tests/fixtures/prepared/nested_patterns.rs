data::ModuleArtifact {
    format: 13,
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
                int_functions: data::Storage::Static(&[
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
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(30),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[
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
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(30),
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
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(30),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Tuple(data::Storage::Static(&[
                                                        data::graph::MatchPattern::Bool(false),
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
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(30),
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
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(30),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Tuple(data::Storage::Static(&[
                                                        data::graph::MatchPattern::Bool(true),
                                                        data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                            elements: data::Storage::Static(&[]),
                                                            tail: None,
                                                        }),
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
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(30),
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
                                        instructions: 3..3,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(30),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Tuple(data::Storage::Static(&[
                                                        data::graph::MatchPattern::Bool(true),
                                                        data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                            elements: data::Storage::Static(&[
                                                                data::graph::MatchPattern::Discard,
                                                            ]),
                                                            tail: None,
                                                        }),
                                                    ])),
                                                ]),
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(7),
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
                                                target: data::graph::BlockId(8),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(30),
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
                                        params: 4..4,
                                        instructions: 3..4,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(3)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..5,
                                        instructions: 4..7,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(4)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(30),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(47),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(30),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(47),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(30),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(47),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(30),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(47),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(30),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(47),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Bool,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(30),
                                                },
                                            },
                                            index: 0,
                                        }),
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
                                        }, data::graph::TypedListInstruction::TupleIndex {
                                            tuple: data::graph::TupleLocalId(0),
                                            index: 1,
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::ListIndex {
                                            list: data::graph::IntListLocalId(0),
                                            index: 1,
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
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
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(31),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[
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
                                                            shape_id: data::type_::CustomValueShapeId(31),
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
                                                    shape_id: data::type_::CustomValueShapeId(31),
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
                                                            type_id: data::type_::CustomTypeId(2),
                                                            index: 0,
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
                                                            shape_id: data::type_::CustomValueShapeId(31),
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
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(31),
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
                                                            type_id: data::type_::CustomTypeId(2),
                                                            index: 1,
                                                        },
                                                        fields: data::Storage::Static(&[
                                                            data::graph::MatchPattern::Bool(false),
                                                            data::graph::MatchPattern::Discard,
                                                        ]),
                                                    },
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
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(31),
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
                                        instructions: 3..3,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(31),
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
                                                            type_id: data::type_::CustomTypeId(2),
                                                            index: 1,
                                                        },
                                                        fields: data::Storage::Static(&[
                                                            data::graph::MatchPattern::Bool(true),
                                                            data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                                elements: data::Storage::Static(&[]),
                                                                tail: None,
                                                            }),
                                                        ]),
                                                    },
                                                ]),
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(7),
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
                                                target: data::graph::BlockId(8),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(31),
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
                                        params: 4..4,
                                        instructions: 3..4,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(3)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..5,
                                        instructions: 4..4,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(31),
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
                                                            type_id: data::type_::CustomTypeId(2),
                                                            index: 1,
                                                        },
                                                        fields: data::Storage::Static(&[
                                                            data::graph::MatchPattern::Bool(true),
                                                            data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                                elements: data::Storage::Static(&[
                                                                    data::graph::MatchPattern::Discard,
                                                                ]),
                                                                tail: None,
                                                            }),
                                                        ]),
                                                    },
                                                ]),
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(9),
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
                                                target: data::graph::BlockId(10),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(31),
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
                                        params: 5..5,
                                        instructions: 4..5,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(4)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 5..6,
                                        instructions: 5..8,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(5)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(31),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(48),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(31),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(48),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(31),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(48),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(31),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(48),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(31),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(48),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(31),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(48),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(31),
                                                },
                                            },
                                            index: 0,
                                        }),
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
                                        }, data::graph::TypedListInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            },
                                            index: 1,
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::ListIndex {
                                            list: data::graph::IntListLocalId(0),
                                            index: 1,
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                            ]),
                        },
                    },
                ]),
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
                                        instructions: 0..3,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            pattern: data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                sign: data::Sign::NoSign,
                                                digits: data::Storage::Static(&[]),
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
                                                            family: data::graph::StorageFamily::String,
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
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(62),
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
                                        instructions: 3..10,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            pattern: data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                sign: data::Sign::NoSign,
                                                digits: data::Storage::Static(&[]),
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
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Bool,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
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
                                                target: data::graph::BlockId(61),
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
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Bool,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 0..0,
                                        instructions: 10..15,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            pattern: data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                sign: data::Sign::NoSign,
                                                digits: data::Storage::Static(&[]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(3),
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
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Bool,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
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
                                                target: data::graph::BlockId(60),
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
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Bool,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 0..0,
                                        instructions: 15..21,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            pattern: data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                sign: data::Sign::NoSign,
                                                digits: data::Storage::Static(&[]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(4),
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
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Bool,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
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
                                                target: data::graph::BlockId(59),
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
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Bool,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 0..0,
                                        instructions: 21..28,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            pattern: data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    7,
                                                ]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(5),
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
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Bool,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
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
                                                target: data::graph::BlockId(58),
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
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Bool,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 0..0,
                                        instructions: 28..36,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            pattern: data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    7,
                                                ]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(6),
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
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Bool,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
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
                                                target: data::graph::BlockId(57),
                                                args: data::Storage::Static(&[
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
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Bool,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 0..0,
                                        instructions: 36..39,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            pattern: data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                sign: data::Sign::NoSign,
                                                digits: data::Storage::Static(&[]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(7),
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
                                                            family: data::graph::StorageFamily::String,
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
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(56),
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
                                        instructions: 39..42,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            pattern: data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                sign: data::Sign::NoSign,
                                                digits: data::Storage::Static(&[]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(8),
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
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(55),
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
                                        params: 0..0,
                                        instructions: 42..49,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            pattern: data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                sign: data::Sign::NoSign,
                                                digits: data::Storage::Static(&[]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(9),
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
                                                            family: data::graph::StorageFamily::Custom,
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
                                                target: data::graph::BlockId(54),
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
                                                            family: data::graph::StorageFamily::Custom,
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 0..0,
                                        instructions: 49..54,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            pattern: data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                sign: data::Sign::NoSign,
                                                digits: data::Storage::Static(&[]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(10),
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
                                                            family: data::graph::StorageFamily::Custom,
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
                                                target: data::graph::BlockId(53),
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 0..0,
                                        instructions: 54..60,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            pattern: data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                sign: data::Sign::NoSign,
                                                digits: data::Storage::Static(&[]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(11),
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
                                                            family: data::graph::StorageFamily::Custom,
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
                                                target: data::graph::BlockId(52),
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 0..0,
                                        instructions: 60..67,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            pattern: data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    7,
                                                ]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(12),
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
                                                            family: data::graph::StorageFamily::Custom,
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
                                                target: data::graph::BlockId(51),
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
                                                            family: data::graph::StorageFamily::Custom,
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 0..0,
                                        instructions: 67..75,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            pattern: data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    7,
                                                ]),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(13),
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
                                                            family: data::graph::StorageFamily::Custom,
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
                                                target: data::graph::BlockId(50),
                                                args: data::Storage::Static(&[
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
                                                            family: data::graph::StorageFamily::Custom,
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 0..0,
                                        instructions: 75..81,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            pattern: data::graph::MatchPattern::Tuple(data::Storage::Static(&[
                                                data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                    sign: data::Sign::Plus,
                                                    digits: data::Storage::Static(&[
                                                        3,
                                                    ]),
                                                }),
                                                data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                    sign: data::Sign::Plus,
                                                    digits: data::Storage::Static(&[
                                                        5,
                                                    ]),
                                                }),
                                                data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                    elements: data::Storage::Static(&[
                                                        data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                            sign: data::Sign::Plus,
                                                            digits: data::Storage::Static(&[
                                                                7,
                                                            ]),
                                                        }),
                                                    ]),
                                                    tail: None,
                                                }),
                                            ])),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(14),
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
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
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
                                                target: data::graph::BlockId(49),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Tuple {
                                                        local: data::graph::TupleLocalId(0),
                                                        type_: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                            data::type_::ValueType::Int,
                                                            data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                        ]),
                                                    },
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 0..0,
                                        instructions: 81..84,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            pattern: data::graph::MatchPattern::Tuple(data::Storage::Static(&[
                                                data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                    sign: data::Sign::NoSign,
                                                    digits: data::Storage::Static(&[]),
                                                }),
                                                data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                    sign: data::Sign::NoSign,
                                                    digits: data::Storage::Static(&[]),
                                                }),
                                                data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                    elements: data::Storage::Static(&[]),
                                                    tail: None,
                                                }),
                                            ])),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(15),
                                                args: data::Storage::Static(&[]),
                                                bindings: data::Storage::Static(&[]),
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
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::IntList,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(48),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Tuple {
                                                        local: data::graph::TupleLocalId(0),
                                                        type_: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                            data::type_::ValueType::Int,
                                                            data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                        ]),
                                                    },
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 0..0,
                                        instructions: 84..88,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            pattern: data::graph::MatchPattern::Tuple(data::Storage::Static(&[
                                                data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                    sign: data::Sign::NoSign,
                                                    digits: data::Storage::Static(&[]),
                                                }),
                                                data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                    sign: data::Sign::NoSign,
                                                    digits: data::Storage::Static(&[]),
                                                }),
                                                data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                    elements: data::Storage::Static(&[]),
                                                    tail: None,
                                                }),
                                            ])),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(16),
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
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
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
                                                target: data::graph::BlockId(47),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Tuple {
                                                        local: data::graph::TupleLocalId(0),
                                                        type_: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                            data::type_::ValueType::Int,
                                                            data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                        ]),
                                                    },
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 0..0,
                                        instructions: 88..91,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            pattern: data::graph::MatchPattern::Tuple(data::Storage::Static(&[
                                                data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                    sign: data::Sign::NoSign,
                                                    digits: data::Storage::Static(&[]),
                                                }),
                                                data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                    sign: data::Sign::NoSign,
                                                    digits: data::Storage::Static(&[]),
                                                }),
                                                data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                    elements: data::Storage::Static(&[]),
                                                    tail: None,
                                                }),
                                            ])),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(17),
                                                args: data::Storage::Static(&[]),
                                                bindings: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
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
                                                target: data::graph::BlockId(46),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Tuple {
                                                        local: data::graph::TupleLocalId(0),
                                                        type_: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                            data::type_::ValueType::Int,
                                                            data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                        ]),
                                                    },
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
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
                                        instructions: 91..96,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            pattern: data::graph::MatchPattern::Bool(true),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(18),
                                                args: data::Storage::Static(&[]),
                                                bindings: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Bool,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::StringList,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(45),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
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
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::StringList,
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
                                        instructions: 96..101,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            pattern: data::graph::MatchPattern::Bool(false),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(19),
                                                args: data::Storage::Static(&[]),
                                                bindings: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Bool,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::StringList,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(44),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
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
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::StringList,
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
                                        instructions: 101..107,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            pattern: data::graph::MatchPattern::Bool(false),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(20),
                                                args: data::Storage::Static(&[]),
                                                bindings: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Bool,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::StringList,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(43),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
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
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::StringList,
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
                                        instructions: 107..110,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            pattern: data::graph::MatchPattern::Bool(false),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(21),
                                                args: data::Storage::Static(&[]),
                                                bindings: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
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
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(42),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
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
                                        instructions: 110..116,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            pattern: data::graph::MatchPattern::Bool(true),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(22),
                                                args: data::Storage::Static(&[]),
                                                bindings: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Bool,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::StringList,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(41),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
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
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::StringList,
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
                                        instructions: 116..122,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            pattern: data::graph::MatchPattern::Bool(false),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(23),
                                                args: data::Storage::Static(&[]),
                                                bindings: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Bool,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::StringList,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(40),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
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
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::StringList,
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
                                        instructions: 122..129,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            pattern: data::graph::MatchPattern::Bool(false),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(24),
                                                args: data::Storage::Static(&[]),
                                                bindings: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Bool,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::StringList,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(39),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
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
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::StringList,
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
                                        instructions: 129..132,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            pattern: data::graph::MatchPattern::Bool(false),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(25),
                                                args: data::Storage::Static(&[]),
                                                bindings: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
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
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(38),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
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
                                        instructions: 132..137,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            pattern: data::graph::MatchPattern::Tuple(data::Storage::Static(&[
                                                data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                    sign: data::Sign::Plus,
                                                    digits: data::Storage::Static(&[
                                                        7,
                                                    ]),
                                                }),
                                                data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                    elements: data::Storage::Static(&[
                                                        data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                            sign: data::Sign::Plus,
                                                            digits: data::Storage::Static(&[
                                                                9,
                                                            ]),
                                                        }),
                                                    ]),
                                                    tail: None,
                                                }),
                                            ])),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(26),
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
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Tuple,
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
                                                target: data::graph::BlockId(37),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Tuple {
                                                        local: data::graph::TupleLocalId(0),
                                                        type_: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                            data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                        ]),
                                                    },
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 0..0,
                                        instructions: 137..140,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            pattern: data::graph::MatchPattern::Tuple(data::Storage::Static(&[
                                                data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                    sign: data::Sign::NoSign,
                                                    digits: data::Storage::Static(&[]),
                                                }),
                                                data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                    elements: data::Storage::Static(&[]),
                                                    tail: None,
                                                }),
                                            ])),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(27),
                                                args: data::Storage::Static(&[]),
                                                bindings: data::Storage::Static(&[]),
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
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::IntList,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(36),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Tuple {
                                                        local: data::graph::TupleLocalId(0),
                                                        type_: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                            data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                        ]),
                                                    },
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 0..0,
                                        instructions: 140..143,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            pattern: data::graph::MatchPattern::Tuple(data::Storage::Static(&[
                                                data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                    sign: data::Sign::NoSign,
                                                    digits: data::Storage::Static(&[]),
                                                }),
                                                data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                    elements: data::Storage::Static(&[]),
                                                    tail: None,
                                                }),
                                            ])),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(28),
                                                args: data::Storage::Static(&[]),
                                                bindings: data::Storage::Static(&[]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
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
                                                target: data::graph::BlockId(35),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Tuple {
                                                        local: data::graph::TupleLocalId(0),
                                                        type_: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                            data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                        ]),
                                                    },
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::String,
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
                                        instructions: 143..148,
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
                                                target: data::graph::BlockId(29),
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
                                                target: data::graph::BlockId(34),
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
                                        instructions: 148..153,
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
                                                target: data::graph::BlockId(30),
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
                                                target: data::graph::BlockId(33),
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
                                        instructions: 153..156,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                                                local: data::graph::ParameterListLocalId(1),
                                                type_id: data::type_::ParameterListTypeId {
                                                    list_type: data::type_::ListTypeId(2),
                                                    item: data::type_::parameter_id(0),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                elements: data::Storage::Static(&[]),
                                                tail: None,
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(31),
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
                                                target: data::graph::BlockId(32),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                                                        local: data::graph::ParameterListLocalId(1),
                                                        type_id: data::type_::ParameterListTypeId {
                                                            list_type: data::type_::ListTypeId(2),
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
                                        instructions: 156..193,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 0..1,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                                                local: data::graph::ParameterListLocalId(0),
                                                type_id: data::type_::ParameterListTypeId {
                                                    list_type: data::type_::ListTypeId(2),
                                                    item: data::type_::parameter_id(0),
                                                },
                                            }),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(3484, 3494)),
                                            pattern_span: data::source::SourceSpan::new(3495, 3497),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..2,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(3432, 3442)),
                                            pattern_span: data::source::SourceSpan::new(3443, 3450),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..3,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(3384, 3394)),
                                            pattern_span: data::source::SourceSpan::new(3395, 3399),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 3..4,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(3333, 3343)),
                                            pattern_span: data::source::SourceSpan::new(3344, 3352),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..5,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(3292, 3302)),
                                            pattern_span: data::source::SourceSpan::new(3303, 3311),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 5..6,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(3246, 3256)),
                                            pattern_span: data::source::SourceSpan::new(3257, 3266),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 6..7,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(3191, 3201)),
                                            pattern_span: data::source::SourceSpan::new(3202, 3207),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 7..8,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(3122, 3132)),
                                            pattern_span: data::source::SourceSpan::new(3133, 3138),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 8..9,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(3058, 3068)),
                                            pattern_span: data::source::SourceSpan::new(3069, 3074),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 9..10,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(2996, 3006)),
                                            pattern_span: data::source::SourceSpan::new(3007, 3011),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 10..11,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(2949, 2959)),
                                            pattern_span: data::source::SourceSpan::new(2960, 2965),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 11..12,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(2897, 2907)),
                                            pattern_span: data::source::SourceSpan::new(2908, 2913),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 12..13,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(2850, 2860)),
                                            pattern_span: data::source::SourceSpan::new(2861, 2866),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 13..14,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(2805, 2815)),
                                            pattern_span: data::source::SourceSpan::new(2816, 2820),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 14..15,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(2750, 2760)),
                                            pattern_span: data::source::SourceSpan::new(2761, 2772),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 15..16,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(2704, 2714)),
                                            pattern_span: data::source::SourceSpan::new(2715, 2726),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 16..17,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(2659, 2669)),
                                            pattern_span: data::source::SourceSpan::new(2670, 2681),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 17..18,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(2606, 2616)),
                                            pattern_span: data::source::SourceSpan::new(2617, 2629),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 18..19,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(2545, 2555)),
                                            pattern_span: data::source::SourceSpan::new(2556, 2557),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 19..20,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(2487, 2497)),
                                            pattern_span: data::source::SourceSpan::new(2498, 2499),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 20..21,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(2432, 2442)),
                                            pattern_span: data::source::SourceSpan::new(2443, 2444),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 21..22,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(2378, 2388)),
                                            pattern_span: data::source::SourceSpan::new(2389, 2390),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 22..23,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(2319, 2329)),
                                            pattern_span: data::source::SourceSpan::new(2330, 2331),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 23..24,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(2275, 2285)),
                                            pattern_span: data::source::SourceSpan::new(2286, 2287),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 24..25,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(2226, 2236)),
                                            pattern_span: data::source::SourceSpan::new(2237, 2238),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 25..26,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(2178, 2188)),
                                            pattern_span: data::source::SourceSpan::new(2189, 2190),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 26..27,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(2133, 2143)),
                                            pattern_span: data::source::SourceSpan::new(2144, 2145),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 27..28,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(2091, 2101)),
                                            pattern_span: data::source::SourceSpan::new(2102, 2103),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 28..29,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(2050, 2060)),
                                            pattern_span: data::source::SourceSpan::new(2061, 2062),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 29..30,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(2004, 2014)),
                                            pattern_span: data::source::SourceSpan::new(2015, 2016),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 30..31,
                                        instructions: 193..193,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "main", data::source::SourceSpan::new(1962, 1972)),
                                            pattern_span: data::source::SourceSpan::new(1973, 1974),
                                        }),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                                            local: data::graph::ParameterListLocalId(0),
                                            type_id: data::type_::ParameterListTypeId {
                                                list_type: data::type_::ListTypeId(2),
                                                item: data::type_::parameter_id(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(31),
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
                                        local: data::graph::ParamLocal::Tuple {
                                            local: data::graph::TupleLocalId(0),
                                            type_: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                            ]),
                                        },
                                        shape: data::type_::ValueShapeId(29),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Tuple {
                                            local: data::graph::TupleLocalId(0),
                                            type_: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                            ]),
                                        },
                                        shape: data::type_::ValueShapeId(29),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Tuple {
                                            local: data::graph::TupleLocalId(0),
                                            type_: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                            ]),
                                        },
                                        shape: data::type_::ValueShapeId(29),
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
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
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
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
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
                                        local: data::graph::ParamLocal::Tuple {
                                            local: data::graph::TupleLocalId(0),
                                            type_: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                                data::type_::ValueType::Int,
                                                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                            ]),
                                        },
                                        shape: data::type_::ValueShapeId(21),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Tuple {
                                            local: data::graph::TupleLocalId(0),
                                            type_: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                                data::type_::ValueType::Int,
                                                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                            ]),
                                        },
                                        shape: data::type_::ValueShapeId(21),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Tuple {
                                            local: data::graph::TupleLocalId(0),
                                            type_: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                                data::type_::ValueType::Int,
                                                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                            ]),
                                        },
                                        shape: data::type_::ValueShapeId(21),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Tuple {
                                            local: data::graph::TupleLocalId(0),
                                            type_: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                                data::type_::ValueType::Int,
                                                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                            ]),
                                        },
                                        shape: data::type_::ValueShapeId(21),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("invalid"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(13),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(0),
                                                index: 1,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(4),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(1977, 2001)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(false)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            shape: data::type_::ValueShapeId(2),
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
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Bool,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                        ]))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(5),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(14),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(0),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                    ]),
                                                },
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(5),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2019, 2047)),
                                        }),
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
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Bool,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                        ]))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(5),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(14),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(0),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                    ]),
                                                },
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(5),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2065, 2088)),
                                        }),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                        ])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Bool,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                        ]))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(5),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(14),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(0),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                    ]),
                                                },
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(5),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2106, 2130)),
                                        }),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            shape: data::type_::ValueShapeId(2),
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
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Bool,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                        ]))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(5),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(14),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(0),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                    ]),
                                                },
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(5),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2148, 2175)),
                                        }),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            shape: data::type_::ValueShapeId(2),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            data::graph::IntLocalId(2),
                                        ])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Bool,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                        ]))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(5),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(14),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(0),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                    ]),
                                                },
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(5),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2193, 2223)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("invalid"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(6),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(15),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(1),
                                                index: 1,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2241, 2272)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(7),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(16),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(2),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(8),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(17),
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
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(7),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(8),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2290, 2316)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(false)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            shape: data::type_::ValueShapeId(2),
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
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(9),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(18),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(2),
                                                index: 1,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(10),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(19),
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
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(9),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(10),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2334, 2375)),
                                        }),
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
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(9),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(18),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(2),
                                                index: 1,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(10),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(19),
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
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(9),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(10),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2393, 2429)),
                                        }),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                        ])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(9),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(18),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(2),
                                                index: 1,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(10),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(19),
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
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(9),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(10),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2447, 2484)),
                                        }),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            shape: data::type_::ValueShapeId(2),
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
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(9),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(18),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(2),
                                                index: 1,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(10),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(19),
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
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(9),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(10),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2502, 2542)),
                                        }),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            shape: data::type_::ValueShapeId(2),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            data::graph::IntLocalId(2),
                                        ])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(9),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(18),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(2),
                                                index: 1,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(10),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(19),
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
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(9),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(10),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2560, 2603)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            data::graph::IntLocalId(2),
                                        ])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    shape_id: data::type_::CustomValueShapeId(11),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(20),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(3),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(21),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Call {
                                            function: data::function::TupleFunctionId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        shape_id: data::type_::CustomValueShapeId(11),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2632, 2656)),
                                        }),
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
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    shape_id: data::type_::CustomValueShapeId(11),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(20),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(3),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(21),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Call {
                                            function: data::function::TupleFunctionId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        shape_id: data::type_::CustomValueShapeId(11),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2684, 2701)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                        ])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    shape_id: data::type_::CustomValueShapeId(11),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(20),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(3),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(21),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Call {
                                            function: data::function::TupleFunctionId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        shape_id: data::type_::CustomValueShapeId(11),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2729, 2747)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("invalid"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
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
                                                index: 1,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(21),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Call {
                                            function: data::function::TupleFunctionId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        shape_id: data::type_::CustomValueShapeId(12),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2775, 2802)),
                                        }),
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
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                local: data::graph::StringListLocalId(0),
                                                type_id: data::type_::StringListTypeId {
                                                    list_type: data::type_::ListTypeId(1),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::String(data::type_::StringListTypeId {
                                            list_type: data::type_::ListTypeId(1),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Bool,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(1)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(7),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                local: data::graph::StringListLocalId(0),
                                                type_id: data::type_::StringListTypeId {
                                                    list_type: data::type_::ListTypeId(1),
                                                },
                                            }),
                                        ]))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(4),
                                                    shape_id: data::type_::CustomValueShapeId(13),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(23),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(4),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::List(data::type_::ListTypeId(1)),
                                                    ]),
                                                },
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                            function: data::function::BoolFunctionId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(4),
                                                        shape_id: data::type_::CustomValueShapeId(13),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2823, 2847)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(false)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                local: data::graph::StringListLocalId(0),
                                                type_id: data::type_::StringListTypeId {
                                                    list_type: data::type_::ListTypeId(1),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::String(data::type_::StringListTypeId {
                                            list_type: data::type_::ListTypeId(1),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Bool,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(1)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(7),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                local: data::graph::StringListLocalId(0),
                                                type_id: data::type_::StringListTypeId {
                                                    list_type: data::type_::ListTypeId(1),
                                                },
                                            }),
                                        ]))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(4),
                                                    shape_id: data::type_::CustomValueShapeId(13),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(23),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(4),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::List(data::type_::ListTypeId(1)),
                                                    ]),
                                                },
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                            function: data::function::BoolFunctionId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(4),
                                                        shape_id: data::type_::CustomValueShapeId(13),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2869, 2894)),
                                        }),
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
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("tail"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                local: data::graph::StringListLocalId(0),
                                                type_id: data::type_::StringListTypeId {
                                                    list_type: data::type_::ListTypeId(1),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::String(data::type_::StringListTypeId {
                                            list_type: data::type_::ListTypeId(1),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                            data::graph::StringLocalId(0),
                                        ])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Bool,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(1)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(7),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                local: data::graph::StringListLocalId(0),
                                                type_id: data::type_::StringListTypeId {
                                                    list_type: data::type_::ListTypeId(1),
                                                },
                                            }),
                                        ]))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(4),
                                                    shape_id: data::type_::CustomValueShapeId(13),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(23),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(4),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::List(data::type_::ListTypeId(1)),
                                                    ]),
                                                },
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                            function: data::function::BoolFunctionId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(4),
                                                        shape_id: data::type_::CustomValueShapeId(13),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2916, 2946)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("invalid"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(4),
                                                    shape_id: data::type_::CustomValueShapeId(14),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(24),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(4),
                                                index: 1,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                            function: data::function::BoolFunctionId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(4),
                                                        shape_id: data::type_::CustomValueShapeId(14),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2968, 2993)),
                                        }),
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
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(5),
                                                    shape_id: data::type_::CustomValueShapeId(15),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(25),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(5),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                local: data::graph::StringListLocalId(0),
                                                type_id: data::type_::StringListTypeId {
                                                    list_type: data::type_::ListTypeId(1),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::String(data::type_::StringListTypeId {
                                            list_type: data::type_::ListTypeId(1),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(5)),
                                                    data::type_::ValueType::List(data::type_::ListTypeId(1)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(26),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(5),
                                                    shape_id: data::type_::CustomValueShapeId(15),
                                                },
                                            }),
                                            data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                local: data::graph::StringListLocalId(0),
                                                type_id: data::type_::StringListTypeId {
                                                    list_type: data::type_::ListTypeId(1),
                                                },
                                            }),
                                        ]))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(6),
                                                    shape_id: data::type_::CustomValueShapeId(16),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(27),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(6),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(5)),
                                                        data::type_::ValueType::List(data::type_::ListTypeId(1)),
                                                    ]),
                                                },
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                            function: data::function::BoolFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(6),
                                                        shape_id: data::type_::CustomValueShapeId(16),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3014, 3055)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(false)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(5),
                                                    shape_id: data::type_::CustomValueShapeId(15),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(25),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(5),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                local: data::graph::StringListLocalId(0),
                                                type_id: data::type_::StringListTypeId {
                                                    list_type: data::type_::ListTypeId(1),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::String(data::type_::StringListTypeId {
                                            list_type: data::type_::ListTypeId(1),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(5)),
                                                    data::type_::ValueType::List(data::type_::ListTypeId(1)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(26),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(5),
                                                    shape_id: data::type_::CustomValueShapeId(15),
                                                },
                                            }),
                                            data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                local: data::graph::StringListLocalId(0),
                                                type_id: data::type_::StringListTypeId {
                                                    list_type: data::type_::ListTypeId(1),
                                                },
                                            }),
                                        ]))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(6),
                                                    shape_id: data::type_::CustomValueShapeId(16),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(27),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(6),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(5)),
                                                        data::type_::ValueType::List(data::type_::ListTypeId(1)),
                                                    ]),
                                                },
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                            function: data::function::BoolFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(6),
                                                        shape_id: data::type_::CustomValueShapeId(16),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3077, 3119)),
                                        }),
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
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(5),
                                                    shape_id: data::type_::CustomValueShapeId(15),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(25),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(5),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("tail"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                local: data::graph::StringListLocalId(0),
                                                type_id: data::type_::StringListTypeId {
                                                    list_type: data::type_::ListTypeId(1),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::String(data::type_::StringListTypeId {
                                            list_type: data::type_::ListTypeId(1),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                            data::graph::StringLocalId(0),
                                        ])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(5)),
                                                    data::type_::ValueType::List(data::type_::ListTypeId(1)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(26),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(5),
                                                    shape_id: data::type_::CustomValueShapeId(15),
                                                },
                                            }),
                                            data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                local: data::graph::StringListLocalId(0),
                                                type_id: data::type_::StringListTypeId {
                                                    list_type: data::type_::ListTypeId(1),
                                                },
                                            }),
                                        ]))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(6),
                                                    shape_id: data::type_::CustomValueShapeId(16),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(27),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(6),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(5)),
                                                        data::type_::ValueType::List(data::type_::ListTypeId(1)),
                                                    ]),
                                                },
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                            function: data::function::BoolFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(6),
                                                        shape_id: data::type_::CustomValueShapeId(16),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3141, 3188)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("invalid"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(6),
                                                    shape_id: data::type_::CustomValueShapeId(17),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(28),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(6),
                                                index: 1,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                            function: data::function::BoolFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(6),
                                                        shape_id: data::type_::CustomValueShapeId(17),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3210, 3243)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(2),
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
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    shape_id: data::type_::CustomValueShapeId(11),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(20),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(3),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(29),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Call {
                                            function: data::function::TupleFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        shape_id: data::type_::CustomValueShapeId(11),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3269, 3289)),
                                        }),
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
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    shape_id: data::type_::CustomValueShapeId(11),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(20),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(3),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(29),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Call {
                                            function: data::function::TupleFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        shape_id: data::type_::CustomValueShapeId(11),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3314, 3330)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("invalid"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
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
                                                index: 1,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(29),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Call {
                                            function: data::function::TupleFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        shape_id: data::type_::CustomValueShapeId(12),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3355, 3381)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            shape: data::type_::ValueShapeId(2),
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
                                            shape: data::type_::ValueShapeId(3),
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
                                            shape: data::type_::ValueShapeId(1),
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
                                            shape: data::type_::ValueShapeId(3),
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
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3402, 3429)),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            shape: data::type_::ValueShapeId(2),
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
                                            shape: data::type_::ValueShapeId(3),
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
                                            shape: data::type_::ValueShapeId(1),
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
                                            shape: data::type_::ValueShapeId(3),
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
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3453, 3481)),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                                                local: data::graph::ParameterListLocalId(0),
                                                type_id: data::type_::ParameterListTypeId {
                                                    list_type: data::type_::ListTypeId(2),
                                                    item: data::type_::parameter_id(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(31),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Parameter(data::type_::ParameterListTypeId {
                                            list_type: data::type_::ListTypeId(2),
                                            item: data::type_::parameter_id(0),
                                        }, data::graph::ParameterListInstruction::Empty)),
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
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                                                local: data::graph::ParameterListLocalId(1),
                                                type_id: data::type_::ParameterListTypeId {
                                                    list_type: data::type_::ListTypeId(2),
                                                    item: data::type_::parameter_id(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(31),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Parameter(data::type_::ParameterListTypeId {
                                            list_type: data::type_::ListTypeId(2),
                                            item: data::type_::parameter_id(0),
                                        }, data::graph::ParameterListInstruction::Call {
                                            function: data::function::ParameterListFunctionId {
                                                index: 0,
                                                type_id: data::type_::ParameterListTypeId {
                                                    list_type: data::type_::ListTypeId(2),
                                                    item: data::type_::parameter_id(0),
                                                },
                                            },
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                                                    local: data::graph::ParameterListLocalId(0),
                                                    type_id: data::type_::ParameterListTypeId {
                                                        list_type: data::type_::ListTypeId(2),
                                                        item: data::type_::parameter_id(0),
                                                    },
                                                }),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3500, 3522)),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                                    type_id: data::type_::CustomTypeId(7),
                                                    shape_id: data::type_::CustomValueShapeId(18),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(32),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(7),
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
                                                    type_id: data::type_::CustomTypeId(8),
                                                    shape_id: data::type_::CustomValueShapeId(19),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(33),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(8),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(7),
                                                        shape_id: data::type_::CustomValueShapeId(18),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                            function: data::function::StringFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(8),
                                                        shape_id: data::type_::CustomValueShapeId(19),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3525, 3546)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(2),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(7),
                                                    shape_id: data::type_::CustomValueShapeId(20),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(34),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(7),
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
                                                    type_id: data::type_::CustomTypeId(8),
                                                    shape_id: data::type_::CustomValueShapeId(21),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(35),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(8),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(2),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(7),
                                                        shape_id: data::type_::CustomValueShapeId(20),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                            function: data::function::StringFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(3),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(8),
                                                        shape_id: data::type_::CustomValueShapeId(21),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3552, 3569)),
                                        }),
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
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(3)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("failed:"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(4),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(8),
                                                    shape_id: data::type_::CustomValueShapeId(22),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(36),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(8),
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
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                            function: data::function::StringFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(4),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(8),
                                                        shape_id: data::type_::CustomValueShapeId(22),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3575, 3600)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(5)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Concatenate {
                                            left: data::graph::StringLocalId(2),
                                            right: data::graph::StringLocalId(4),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(5),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(7),
                                                    shape_id: data::type_::CustomValueShapeId(18),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(32),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(7),
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
                                                    type_id: data::type_::CustomTypeId(9),
                                                    shape_id: data::type_::CustomValueShapeId(23),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(37),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(9),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(5),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(7),
                                                        shape_id: data::type_::CustomValueShapeId(18),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(2),
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
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(9)),
                                                    data::type_::ValueType::Int,
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(38),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(6),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(9),
                                                    shape_id: data::type_::CustomValueShapeId(23),
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
                                                    type_id: data::type_::CustomTypeId(10),
                                                    shape_id: data::type_::CustomValueShapeId(24),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(39),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(10),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(9)),
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                },
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(6)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                            function: data::function::StringFunctionId(2),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(7),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(10),
                                                        shape_id: data::type_::CustomValueShapeId(24),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3606, 3638)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(7)),
                                            shape: data::type_::ValueShapeId(0),
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
                                                    type_id: data::type_::CustomTypeId(7),
                                                    shape_id: data::type_::CustomValueShapeId(20),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(34),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(7),
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
                                                    type_id: data::type_::CustomTypeId(9),
                                                    shape_id: data::type_::CustomValueShapeId(25),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(40),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(9),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(8),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(7),
                                                        shape_id: data::type_::CustomValueShapeId(20),
                                                    },
                                                }),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            shape: data::type_::ValueShapeId(2),
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
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(9)),
                                                    data::type_::ValueType::Int,
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(41),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(9),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(9),
                                                    shape_id: data::type_::CustomValueShapeId(25),
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
                                                    type_id: data::type_::CustomTypeId(10),
                                                    shape_id: data::type_::CustomValueShapeId(26),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(42),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(10),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(1),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(9)),
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                },
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(8)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                            function: data::function::StringFunctionId(2),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(10),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(10),
                                                        shape_id: data::type_::CustomValueShapeId(26),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3644, 3672)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(9)),
                                            shape: data::type_::ValueShapeId(0),
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
                                                    type_id: data::type_::CustomTypeId(9),
                                                    shape_id: data::type_::CustomValueShapeId(27),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(43),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(9),
                                                index: 1,
                                            },
                                            fields: data::Storage::Static(&[]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                            shape: data::type_::ValueShapeId(2),
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
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(9)),
                                                    data::type_::ValueType::Int,
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(44),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(11),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(9),
                                                    shape_id: data::type_::CustomValueShapeId(27),
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
                                                    type_id: data::type_::CustomTypeId(10),
                                                    shape_id: data::type_::CustomValueShapeId(28),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(45),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(10),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(2),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(9)),
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                },
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(10)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                            function: data::function::StringFunctionId(2),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(12),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(10),
                                                        shape_id: data::type_::CustomValueShapeId(28),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3678, 3700)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(11)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Concatenate {
                                            left: data::graph::StringLocalId(9),
                                            right: data::graph::StringLocalId(10),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(12)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("done"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(13),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(10),
                                                    shape_id: data::type_::CustomValueShapeId(29),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(46),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(10),
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
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                            function: data::function::StringFunctionId(2),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(13),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(10),
                                                        shape_id: data::type_::CustomValueShapeId(29),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3706, 3727)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(14)),
                                            shape: data::type_::ValueShapeId(0),
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
                                                    type_id: data::type_::CustomTypeId(8),
                                                    shape_id: data::type_::CustomValueShapeId(35),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(8),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Custom {
                                                        constructor: data::type_::CustomConstructorId {
                                                            type_id: data::type_::CustomTypeId(7),
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
                                                            type_id: data::type_::CustomTypeId(8),
                                                            shape_id: data::type_::CustomValueShapeId(35),
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
                                                    type_id: data::type_::CustomTypeId(8),
                                                    shape_id: data::type_::CustomValueShapeId(35),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(8),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Custom {
                                                        constructor: data::type_::CustomConstructorId {
                                                            type_id: data::type_::CustomTypeId(7),
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
                                                            type_id: data::type_::CustomTypeId(8),
                                                            shape_id: data::type_::CustomValueShapeId(35),
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
                                                type_id: data::type_::CustomTypeId(8),
                                                shape_id: data::type_::CustomValueShapeId(35),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(52),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(8),
                                                shape_id: data::type_::CustomValueShapeId(35),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(52),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(8),
                                                shape_id: data::type_::CustomValueShapeId(35),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(52),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("present:"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("missing:"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(8),
                                                    shape_id: data::type_::CustomValueShapeId(35),
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
                                                    type_id: data::type_::CustomTypeId(10),
                                                    shape_id: data::type_::CustomValueShapeId(36),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(10),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Tuple(data::Storage::Static(&[
                                                        data::graph::MatchPattern::Custom {
                                                            constructor: data::type_::CustomConstructorId {
                                                                type_id: data::type_::CustomTypeId(9),
                                                                index: 0,
                                                            },
                                                            fields: data::Storage::Static(&[
                                                                data::graph::MatchPattern::Custom {
                                                                    constructor: data::type_::CustomConstructorId {
                                                                        type_id: data::type_::CustomTypeId(7),
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
                                                            type_id: data::type_::CustomTypeId(10),
                                                            shape_id: data::type_::CustomValueShapeId(36),
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
                                                    type_id: data::type_::CustomTypeId(10),
                                                    shape_id: data::type_::CustomValueShapeId(36),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(10),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Tuple(data::Storage::Static(&[
                                                        data::graph::MatchPattern::Custom {
                                                            constructor: data::type_::CustomConstructorId {
                                                                type_id: data::type_::CustomTypeId(9),
                                                                index: 0,
                                                            },
                                                            fields: data::Storage::Static(&[
                                                                data::graph::MatchPattern::Custom {
                                                                    constructor: data::type_::CustomConstructorId {
                                                                        type_id: data::type_::CustomTypeId(7),
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
                                                            type_id: data::type_::CustomTypeId(10),
                                                            shape_id: data::type_::CustomValueShapeId(36),
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
                                                    type_id: data::type_::CustomTypeId(10),
                                                    shape_id: data::type_::CustomValueShapeId(36),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(10),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Tuple(data::Storage::Static(&[
                                                        data::graph::MatchPattern::Custom {
                                                            constructor: data::type_::CustomConstructorId {
                                                                type_id: data::type_::CustomTypeId(9),
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
                                                            type_id: data::type_::CustomTypeId(10),
                                                            shape_id: data::type_::CustomValueShapeId(36),
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
                                                type_id: data::type_::CustomTypeId(10),
                                                shape_id: data::type_::CustomValueShapeId(36),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(53),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(10),
                                                shape_id: data::type_::CustomValueShapeId(36),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(53),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(10),
                                                shape_id: data::type_::CustomValueShapeId(36),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(53),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(10),
                                                shape_id: data::type_::CustomValueShapeId(36),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(53),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("nested:"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("empty:"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("none:"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(10),
                                                    shape_id: data::type_::CustomValueShapeId(36),
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
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(4),
                                                    shape_id: data::type_::CustomValueShapeId(33),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(4),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[
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
                                                            type_id: data::type_::CustomTypeId(4),
                                                            shape_id: data::type_::CustomValueShapeId(33),
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
                                                    type_id: data::type_::CustomTypeId(4),
                                                    shape_id: data::type_::CustomValueShapeId(33),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(4),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Tuple(data::Storage::Static(&[
                                                        data::graph::MatchPattern::Discard,
                                                        data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                            elements: data::Storage::Static(&[
                                                                data::graph::MatchPattern::Discard,
                                                            ]),
                                                            tail: Some(data::graph::MatchPatternListTail::Ignore),
                                                        }),
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
                                                            type_id: data::type_::CustomTypeId(4),
                                                            shape_id: data::type_::CustomValueShapeId(33),
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
                                        instructions: 2..4,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(4),
                                                shape_id: data::type_::CustomValueShapeId(33),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(50),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(4),
                                                shape_id: data::type_::CustomValueShapeId(33),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(50),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(4),
                                                shape_id: data::type_::CustomValueShapeId(33),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(50),
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
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(false)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Bool,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(1)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(7),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(4),
                                                    shape_id: data::type_::CustomValueShapeId(33),
                                                },
                                            },
                                            index: 0,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::TupleIndex {
                                            tuple: data::graph::TupleLocalId(0),
                                            index: 0,
                                        }),
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
                                                    type_id: data::type_::CustomTypeId(6),
                                                    shape_id: data::type_::CustomValueShapeId(34),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(6),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[
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
                                                            type_id: data::type_::CustomTypeId(6),
                                                            shape_id: data::type_::CustomValueShapeId(34),
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
                                                    type_id: data::type_::CustomTypeId(6),
                                                    shape_id: data::type_::CustomValueShapeId(34),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(6),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Tuple(data::Storage::Static(&[
                                                        data::graph::MatchPattern::Discard,
                                                        data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                            elements: data::Storage::Static(&[
                                                                data::graph::MatchPattern::Discard,
                                                            ]),
                                                            tail: Some(data::graph::MatchPatternListTail::Ignore),
                                                        }),
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
                                                            type_id: data::type_::CustomTypeId(6),
                                                            shape_id: data::type_::CustomValueShapeId(34),
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
                                        instructions: 2..5,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(6),
                                                shape_id: data::type_::CustomValueShapeId(34),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(51),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(6),
                                                shape_id: data::type_::CustomValueShapeId(34),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(51),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(6),
                                                shape_id: data::type_::CustomValueShapeId(34),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(51),
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
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(false)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(5)),
                                                    data::type_::ValueType::List(data::type_::ListTypeId(1)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(9),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(6),
                                                    shape_id: data::type_::CustomValueShapeId(34),
                                                },
                                            },
                                            index: 0,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(5),
                                                    shape_id: data::type_::CustomValueShapeId(1),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(8),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::TupleIndex {
                                            tuple: data::graph::TupleLocalId(0),
                                            index: 0,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(5),
                                                    shape_id: data::type_::CustomValueShapeId(1),
                                                },
                                            },
                                            index: 0,
                                        }),
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
                                                    shape_id: data::type_::CustomValueShapeId(32),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[
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
                                                            type_id: data::type_::CustomTypeId(3),
                                                            shape_id: data::type_::CustomValueShapeId(32),
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
                                        instructions: 0..4,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..2,
                                        instructions: 4..4,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    shape_id: data::type_::CustomValueShapeId(32),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                        elements: data::Storage::Static(&[]),
                                                        tail: None,
                                                    }),
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
                                                            shape_id: data::type_::CustomValueShapeId(32),
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
                                        instructions: 4..8,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..3,
                                        instructions: 8..8,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    shape_id: data::type_::CustomValueShapeId(32),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                        elements: data::Storage::Static(&[
                                                            data::graph::MatchPattern::Discard,
                                                        ]),
                                                        tail: None,
                                                    }),
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
                                                            shape_id: data::type_::CustomValueShapeId(32),
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
                                        instructions: 8..12,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 3..4,
                                        instructions: 12..17,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(3)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(3),
                                                shape_id: data::type_::CustomValueShapeId(32),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(49),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(3),
                                                shape_id: data::type_::CustomValueShapeId(32),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(49),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(3),
                                                shape_id: data::type_::CustomValueShapeId(32),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(49),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(3),
                                                shape_id: data::type_::CustomValueShapeId(32),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(49),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
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
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(21),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                        ]))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
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
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(21),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                        ]))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
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
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(21),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                        ]))),
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
                                        }, data::graph::TypedListInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    shape_id: data::type_::CustomValueShapeId(32),
                                                },
                                            },
                                            index: 0,
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::ListIndex {
                                            list: data::graph::IntListLocalId(0),
                                            index: 0,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::ListIndex {
                                            list: data::graph::IntListLocalId(0),
                                            index: 1,
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
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Int(data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        }, data::graph::TypedListInstruction::DropFirst {
                                            list: data::graph::IntListLocalId(0),
                                            count: 2,
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(21),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(1),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                        ]))),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::TupleLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::TupleLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::TupleLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::TupleLocalId(0)),
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
                                                    shape_id: data::type_::CustomValueShapeId(32),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[
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
                                                            type_id: data::type_::CustomTypeId(3),
                                                            shape_id: data::type_::CustomValueShapeId(32),
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
                                        instructions: 0..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..2,
                                        instructions: 3..3,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    shape_id: data::type_::CustomValueShapeId(32),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                        elements: data::Storage::Static(&[]),
                                                        tail: None,
                                                    }),
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
                                                            shape_id: data::type_::CustomValueShapeId(32),
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
                                        instructions: 3..6,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..3,
                                        instructions: 6..10,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(3),
                                                shape_id: data::type_::CustomValueShapeId(32),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(49),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(3),
                                                shape_id: data::type_::CustomValueShapeId(32),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(49),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(3),
                                                shape_id: data::type_::CustomValueShapeId(32),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(49),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
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
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(29),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                        ]))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
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
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(29),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                        ]))),
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
                                        }, data::graph::TypedListInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    shape_id: data::type_::CustomValueShapeId(32),
                                                },
                                            },
                                            index: 0,
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Int(data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        }, data::graph::TypedListInstruction::DropFirst {
                                            list: data::graph::IntListLocalId(0),
                                            count: 1,
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(29),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(1),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                        ]))),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::TupleLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::TupleLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::TupleLocalId(0)),
                            ]),
                        },
                    },
                ]),
            },
            list_returns: data::function::ListFunctionTables {
                parameter_list_functions: data::Storage::Static(&[
                    (data::function::ParameterListFunctionId {
                        index: 0,
                        type_id: data::type_::ParameterListTypeId {
                            list_type: data::type_::ListTypeId(2),
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
                                                list_type: data::type_::ListTypeId(2),
                                                item: data::type_::parameter_id(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(31),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
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
                                        shape: data::type_::ValueShapeId(3),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
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
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(1),
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
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
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
                                            shape: data::type_::ValueShapeId(3),
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
        compiled: data::compiled::CompiledFunctions::interpreted(),
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
                0..2,
                0..0,
                2..5,
                0..0,
                0..0,
                0..0,
                0..0,
                5..7,
                0..0,
                7..9,
                9..10,
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
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
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
                        data::type_::ValueShapeId(47),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 1..2,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(48),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 2..2,
                    parameter_shapes: data::Storage::Static(&[]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 2..3,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(52),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 3..4,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(53),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 4..5,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(50),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 5..6,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(51),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 6..7,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(49),
                    ]),
                    return_: data::type_::ValueShapeId(21),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 7..8,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(49),
                    ]),
                    return_: data::type_::ValueShapeId(29),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 8..10,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(31),
                        data::type_::ValueShapeId(1),
                    ]),
                    return_: data::type_::ValueShapeId(31),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 10..12,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(3),
                        data::type_::ValueShapeId(1),
                    ]),
                    return_: data::type_::ValueShapeId(3),
                    captures: data::Storage::Static(&[]),
                },
            ]),
            parameters: data::Storage::Static(&[
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(0),
                        shape_id: data::type_::CustomValueShapeId(30),
                    },
                }),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(1),
                        shape_id: data::type_::CustomValueShapeId(31),
                    },
                }),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(8),
                        shape_id: data::type_::CustomValueShapeId(35),
                    },
                }),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(10),
                        shape_id: data::type_::CustomValueShapeId(36),
                    },
                }),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(4),
                        shape_id: data::type_::CustomValueShapeId(33),
                    },
                }),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(6),
                        shape_id: data::type_::CustomValueShapeId(34),
                    },
                }),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(3),
                        shape_id: data::type_::CustomValueShapeId(32),
                    },
                }),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(3),
                        shape_id: data::type_::CustomValueShapeId(32),
                    },
                }),
                data::graph::ParamLocal::List(data::graph::ListLocal::Parameter {
                    local: data::graph::ParameterListLocalId(0),
                    type_id: data::type_::ParameterListTypeId {
                        list_type: data::type_::ListTypeId(2),
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
                data::type_::ListStorageTypeId::String(data::type_::StringListTypeId {
                    list_type: data::type_::ListTypeId(1),
                }),
                data::type_::ListStorageTypeId::Parameter(data::type_::ParameterListTypeId {
                    list_type: data::type_::ListTypeId(2),
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
                        package: data::Text::Static(""),
                        module: data::Text::Static("gleam"),
                        name: data::Text::Static("Result"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                                data::type_::TypeMetadata::Bool,
                                data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                            ])),
                            data::type_::TypeMetadata::String,
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
                                    type_: data::type_::ValueType::Tuple(data::Storage::Static(&[
                                        data::type_::ValueType::Bool,
                                        data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                    ])),
                                    shape: data::type_::ValueShapeId(4),
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
                                    type_: data::type_::ValueType::String,
                                    shape: data::type_::ValueShapeId(0),
                                    refinement: data::type_::FieldRefinement::Argument(1),
                                },
                            ]),
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
                                name: data::Text::Static("Payload"),
                                arguments: data::Storage::Static(&[]),
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
                                    type_: data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                                    shape: data::type_::ValueShapeId(5),
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
                                    shape: data::type_::ValueShapeId(0),
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
                        name: data::Text::Static("Payload"),
                        arguments: data::Storage::Static(&[]),
                    },
                    constructor_count: 2,
                    constructors: data::Storage::Static(&[
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(2),
                                index: 0,
                            },
                            name: data::Text::Static("Missing"),
                            native_tag: data::Text::Static("missing"),
                            fields: data::Storage::Static(&[]),
                        },
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(2),
                                index: 1,
                            },
                            name: data::Text::Static("Present"),
                            native_tag: data::Text::Static("present"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::Bool,
                                    shape: data::type_::ValueShapeId(1),
                                    refinement: data::type_::FieldRefinement::Value,
                                },
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                    shape: data::type_::ValueShapeId(3),
                                    refinement: data::type_::FieldRefinement::List(data::Storage::Static(&data::type_::FieldRefinement::Value)),
                                },
                            ]),
                        },
                    ]),
                },
                data::type_::CustomTypeDescriptor {
                    type_: data::type_::NominalTypeMetadata {
                        package: data::Text::Static(""),
                        module: data::Text::Static("gleam"),
                        name: data::Text::Static("Result"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
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
                                    type_: data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                    shape: data::type_::ValueShapeId(3),
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
                                    shape: data::type_::ValueShapeId(0),
                                    refinement: data::type_::FieldRefinement::Argument(1),
                                },
                            ]),
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
                                data::type_::TypeMetadata::Bool,
                                data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::String)),
                            ])),
                            data::type_::TypeMetadata::String,
                        ]),
                    },
                    constructor_count: 2,
                    constructors: data::Storage::Static(&[
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(4),
                                index: 0,
                            },
                            name: data::Text::Static("Ok"),
                            native_tag: data::Text::Static("ok"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::Tuple(data::Storage::Static(&[
                                        data::type_::ValueType::Bool,
                                        data::type_::ValueType::List(data::type_::ListTypeId(1)),
                                    ])),
                                    shape: data::type_::ValueShapeId(7),
                                    refinement: data::type_::FieldRefinement::Argument(0),
                                },
                            ]),
                        },
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(4),
                                index: 1,
                            },
                            name: data::Text::Static("Error"),
                            native_tag: data::Text::Static("error"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::String,
                                    shape: data::type_::ValueShapeId(0),
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
                        name: data::Text::Static("Request"),
                        arguments: data::Storage::Static(&[]),
                    },
                    constructor_count: 1,
                    constructors: data::Storage::Static(&[
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(5),
                                index: 0,
                            },
                            name: data::Text::Static("Request"),
                            native_tag: data::Text::Static("request"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: Some(data::Text::Static("enabled")),
                                    type_: data::type_::ValueType::Bool,
                                    shape: data::type_::ValueShapeId(1),
                                    refinement: data::type_::FieldRefinement::Value,
                                },
                            ]),
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
                                    name: data::Text::Static("Request"),
                                    arguments: data::Storage::Static(&[]),
                                }),
                                data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::String)),
                            ])),
                            data::type_::TypeMetadata::String,
                        ]),
                    },
                    constructor_count: 2,
                    constructors: data::Storage::Static(&[
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(6),
                                index: 0,
                            },
                            name: data::Text::Static("Ok"),
                            native_tag: data::Text::Static("ok"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::Tuple(data::Storage::Static(&[
                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(5)),
                                        data::type_::ValueType::List(data::type_::ListTypeId(1)),
                                    ])),
                                    shape: data::type_::ValueShapeId(9),
                                    refinement: data::type_::FieldRefinement::Argument(0),
                                },
                            ]),
                        },
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(6),
                                index: 1,
                            },
                            name: data::Text::Static("Error"),
                            native_tag: data::Text::Static("error"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::String,
                                    shape: data::type_::ValueShapeId(0),
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
                            data::type_::TypeMetadata::Int,
                        ]),
                    },
                    constructor_count: 2,
                    constructors: data::Storage::Static(&[
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(7),
                                index: 0,
                            },
                            name: data::Text::Static("Some"),
                            native_tag: data::Text::Static("some"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::Int,
                                    shape: data::type_::ValueShapeId(2),
                                    refinement: data::type_::FieldRefinement::Argument(0),
                                },
                            ]),
                        },
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(7),
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
                                type_id: data::type_::CustomTypeId(8),
                                index: 0,
                            },
                            name: data::Text::Static("Ok"),
                            native_tag: data::Text::Static("ok"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::Custom(data::type_::CustomTypeId(7)),
                                    shape: data::type_::ValueShapeId(10),
                                    refinement: data::type_::FieldRefinement::Argument(0),
                                },
                            ]),
                        },
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(8),
                                index: 1,
                            },
                            name: data::Text::Static("Error"),
                            native_tag: data::Text::Static("error"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::String,
                                    shape: data::type_::ValueShapeId(0),
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
                                type_id: data::type_::CustomTypeId(9),
                                index: 0,
                            },
                            name: data::Text::Static("Some"),
                            native_tag: data::Text::Static("some"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::Custom(data::type_::CustomTypeId(7)),
                                    shape: data::type_::ValueShapeId(10),
                                    refinement: data::type_::FieldRefinement::Argument(0),
                                },
                            ]),
                        },
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(9),
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
                                type_id: data::type_::CustomTypeId(10),
                                index: 0,
                            },
                            name: data::Text::Static("Ok"),
                            native_tag: data::Text::Static("ok"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::Tuple(data::Storage::Static(&[
                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(9)),
                                        data::type_::ValueType::Int,
                                    ])),
                                    shape: data::type_::ValueShapeId(12),
                                    refinement: data::type_::FieldRefinement::Argument(0),
                                },
                            ]),
                        },
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(10),
                                index: 1,
                            },
                            name: data::Text::Static("Error"),
                            native_tag: data::Text::Static("error"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::String,
                                    shape: data::type_::ValueShapeId(0),
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
                data::type_::CustomDefinition {
                    package: data::Text::Static("geam"),
                    module: data::Text::Static("example"),
                    name: data::Text::Static("Payload"),
                    publicity: data::type_::CustomTypePublicity::Public,
                    opaque: false,
                    parameters: 0,
                    constructors: data::Storage::Static(&[
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Missing"),
                            fields: data::Storage::Static(&[]),
                        },
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Present"),
                            fields: data::Storage::Static(&[
                                data::type_::FieldDefinition {
                                    label: None,
                                    type_: data::type_::TypeMetadata::Bool,
                                },
                                data::type_::FieldDefinition {
                                    label: None,
                                    type_: data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                                },
                            ]),
                        },
                    ]),
                },
                data::type_::CustomDefinition {
                    package: data::Text::Static("geam"),
                    module: data::Text::Static("example"),
                    name: data::Text::Static("Request"),
                    publicity: data::type_::CustomTypePublicity::Public,
                    opaque: false,
                    parameters: 0,
                    constructors: data::Storage::Static(&[
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Request"),
                            fields: data::Storage::Static(&[
                                data::type_::FieldDefinition {
                                    label: Some(data::Text::Static("enabled")),
                                    type_: data::type_::TypeMetadata::Bool,
                                },
                            ]),
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
                data::type_::ValueShapeDescriptor::String,
                data::type_::ValueShapeDescriptor::Bool,
                data::type_::ValueShapeDescriptor::Int,
                data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(2)),
                data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                    data::type_::ValueShapeId(1),
                    data::type_::ValueShapeId(3),
                ])),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(0)),
                data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(0)),
                data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                    data::type_::ValueShapeId(1),
                    data::type_::ValueShapeId(6),
                ])),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(1)),
                data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                    data::type_::ValueShapeId(8),
                    data::type_::ValueShapeId(6),
                ])),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(2)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(3)),
                data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                    data::type_::ValueShapeId(11),
                    data::type_::ValueShapeId(2),
                ])),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(4)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(5)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(6)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(7)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(8)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(9)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(10)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(11)),
                data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                    data::type_::ValueShapeId(2),
                    data::type_::ValueShapeId(2),
                    data::type_::ValueShapeId(3),
                ])),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(12)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(13)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(14)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(15)),
                data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                    data::type_::ValueShapeId(25),
                    data::type_::ValueShapeId(6),
                ])),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(16)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(17)),
                data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                    data::type_::ValueShapeId(2),
                    data::type_::ValueShapeId(3),
                ])),
                data::type_::ValueShapeDescriptor::Parameter(data::type_::parameter_id(0)),
                data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(30)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(18)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(19)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(20)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(21)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(22)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(23)),
                data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                    data::type_::ValueShapeId(37),
                    data::type_::ValueShapeId(2),
                ])),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(24)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(25)),
                data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                    data::type_::ValueShapeId(40),
                    data::type_::ValueShapeId(2),
                ])),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(26)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(27)),
                data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                    data::type_::ValueShapeId(43),
                    data::type_::ValueShapeId(2),
                ])),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(28)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(29)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(30)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(31)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(32)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(33)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(34)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(35)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(36)),
            ]),
            shape_types: data::Storage::Static(&[
                data::type_::ValueType::String,
                data::type_::ValueType::Bool,
                data::type_::ValueType::Int,
                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                data::type_::ValueType::Tuple(data::Storage::Static(&[
                    data::type_::ValueType::Bool,
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                ])),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                data::type_::ValueType::List(data::type_::ListTypeId(1)),
                data::type_::ValueType::Tuple(data::Storage::Static(&[
                    data::type_::ValueType::Bool,
                    data::type_::ValueType::List(data::type_::ListTypeId(1)),
                ])),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(5)),
                data::type_::ValueType::Tuple(data::Storage::Static(&[
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(5)),
                    data::type_::ValueType::List(data::type_::ListTypeId(1)),
                ])),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(7)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(9)),
                data::type_::ValueType::Tuple(data::Storage::Static(&[
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(9)),
                    data::type_::ValueType::Int,
                ])),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(3)),
                data::type_::ValueType::Tuple(data::Storage::Static(&[
                    data::type_::ValueType::Int,
                    data::type_::ValueType::Int,
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                ])),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(3)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(4)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(4)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(5)),
                data::type_::ValueType::Tuple(data::Storage::Static(&[
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(5)),
                    data::type_::ValueType::List(data::type_::ListTypeId(1)),
                ])),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(6)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(6)),
                data::type_::ValueType::Tuple(data::Storage::Static(&[
                    data::type_::ValueType::Int,
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                ])),
                data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                data::type_::ValueType::List(data::type_::ListTypeId(2)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(7)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(8)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(7)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(8)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(8)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(9)),
                data::type_::ValueType::Tuple(data::Storage::Static(&[
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(9)),
                    data::type_::ValueType::Int,
                ])),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(10)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(9)),
                data::type_::ValueType::Tuple(data::Storage::Static(&[
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(9)),
                    data::type_::ValueType::Int,
                ])),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(10)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(9)),
                data::type_::ValueType::Tuple(data::Storage::Static(&[
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(9)),
                    data::type_::ValueType::Int,
                ])),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(10)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(10)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(3)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(4)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(6)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(8)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(10)),
            ]),
            custom_shapes: data::Storage::Static(&[
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(2),
                    arguments: data::Storage::Static(&[]),
                    constructor: data::type_::CustomConstructorRefinement::Any,
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(5),
                    arguments: data::Storage::Static(&[]),
                    constructor: data::type_::CustomConstructorRefinement::Any,
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(7),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(2),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Any,
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(9),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(10),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Any,
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(0),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(4),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(1),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(0),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(4),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(1),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(5),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(1),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(2),
                    arguments: data::Storage::Static(&[]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(1),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(16),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(2),
                    arguments: data::Storage::Static(&[]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(1),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(1),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(18),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(3),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(3),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(3),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(3),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(1),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(4),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(7),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(4),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(7),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(1),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(5),
                    arguments: data::Storage::Static(&[]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(6),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(26),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(6),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(9),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(1),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(7),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(2),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(8),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(32),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(7),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(2),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(1),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(8),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(34),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(8),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(10),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(1),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(9),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(32),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(10),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(38),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(9),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(34),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(10),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(41),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(9),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(10),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(1),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(10),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(44),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(10),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(12),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(1),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(0),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(4),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Any,
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(1),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(5),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Any,
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(3),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(3),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Any,
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(4),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(7),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Any,
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(6),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(9),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Any,
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(8),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(10),
                        data::type_::ValueShapeId(0),
                    ]),
                    constructor: data::type_::CustomConstructorRefinement::Any,
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(10),
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(12),
                        data::type_::ValueShapeId(0),
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
