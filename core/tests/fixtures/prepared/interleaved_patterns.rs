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
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(1),
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
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "selected", data::source::SourceSpan::new(1038, 1048)),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(2),
                                        site: data::source::HostCallSite::from_static("example", "selected", data::source::SourceSpan::new(1031, 1049)),
                                    },
                                    args: data::Storage::Static(&[
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
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(1),
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
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "selected_grouped", data::source::SourceSpan::new(1113, 1123)),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(3),
                                        site: data::source::HostCallSite::from_static("example", "selected_grouped", data::source::SourceSpan::new(1098, 1124)),
                                    },
                                    args: data::Storage::Static(&[
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
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Custom {
                                                        constructor: data::type_::CustomConstructorId {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            index: 0,
                                                        },
                                                        fields: data::Storage::Static(&[
                                                            data::graph::MatchPattern::Bind(data::graph::MatchPatternBinding {
                                                                index: 0,
                                                            }),
                                                        ]),
                                                    },
                                                ]),
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
                                                            shape_id: data::type_::CustomValueShapeId(0),
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
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..3,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Custom {
                                                        constructor: data::type_::CustomConstructorId {
                                                            type_id: data::type_::CustomTypeId(2),
                                                            index: 0,
                                                        },
                                                        fields: data::Storage::Static(&[
                                                            data::graph::MatchPattern::Bind(data::graph::MatchPatternBinding {
                                                                index: 0,
                                                            }),
                                                        ]),
                                                    },
                                                ]),
                                            },
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
                                                            shape_id: data::type_::CustomValueShapeId(0),
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
                                        params: 3..4,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..5,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Custom {
                                                        constructor: data::type_::CustomConstructorId {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            index: 1,
                                                        },
                                                        fields: data::Storage::Static(&[
                                                            data::graph::MatchPattern::Bind(data::graph::MatchPatternBinding {
                                                                index: 0,
                                                            }),
                                                        ]),
                                                    },
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
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(0),
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
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 6..7,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Custom {
                                                        constructor: data::type_::CustomConstructorId {
                                                            type_id: data::type_::CustomTypeId(2),
                                                            index: 2,
                                                        },
                                                        fields: data::Storage::Static(&[
                                                            data::graph::MatchPattern::Bind(data::graph::MatchPatternBinding {
                                                                index: 0,
                                                            }),
                                                        ]),
                                                    },
                                                ]),
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(7),
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
                                                target: data::graph::BlockId(8),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(0),
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
                                        params: 7..8,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(3)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 8..9,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Custom {
                                                        constructor: data::type_::CustomConstructorId {
                                                            type_id: data::type_::CustomTypeId(2),
                                                            index: 1,
                                                        },
                                                        fields: data::Storage::Static(&[
                                                            data::graph::MatchPattern::Bind(data::graph::MatchPatternBinding {
                                                                index: 0,
                                                            }),
                                                        ]),
                                                    },
                                                ]),
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(9),
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
                                                target: data::graph::BlockId(10),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(0),
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
                                        params: 9..10,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(4)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 10..11,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(5)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(1),
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
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(1),
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
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(1),
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
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(1),
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
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(1),
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
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(1),
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
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            },
                                            index: 0,
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
                                                    shape_id: data::type_::CustomValueShapeId(0),
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
                                                    data::graph::MatchEdgeArgument::Binding(0),
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    0,
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
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
                                                            shape_id: data::type_::CustomValueShapeId(0),
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
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(1),
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
                                                ]),
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(2),
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
                                                target: data::graph::BlockId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(1),
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
                                        params: 2..3,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 3..4,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..5,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
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
                                                target: data::graph::BlockId(10),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(0),
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
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(2),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Bind(data::graph::MatchPatternBinding {
                                                        index: 0,
                                                    }),
                                                ]),
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(6),
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
                                                target: data::graph::BlockId(7),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(2),
                                                            shape_id: data::type_::CustomValueShapeId(2),
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
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 7..8,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(2),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    index: 2,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::MatchPattern::Bind(data::graph::MatchPatternBinding {
                                                        index: 0,
                                                    }),
                                                ]),
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(8),
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
                                                target: data::graph::BlockId(9),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(2),
                                                            shape_id: data::type_::CustomValueShapeId(2),
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
                                        params: 8..9,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(3)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 9..10,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(4)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 10..11,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(5)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(1),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(2),
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
                                                shape_id: data::type_::CustomValueShapeId(1),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(2),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(3),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(2),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(3),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(2),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(3),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(1),
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
                                                    shape_id: data::type_::CustomValueShapeId(1),
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
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(2),
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
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            },
                                            index: 0,
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
                string_functions: data::Storage::Static(&[]),
                bit_array_functions: data::Storage::Static(&[]),
                utf_codepoint_functions: data::Storage::Static(&[]),
                custom_functions: data::Storage::Static(&[
                    data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 1,
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
                                                        target: data::graph::BlockId(2),
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
                                                            2,
                                                        ]),
                                                    }, data::graph::Edge {
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
                                                    }),
                                                    (data::graph::IntegerLiteral {
                                                        sign: data::Sign::Plus,
                                                        digits: data::Storage::Static(&[
                                                            3,
                                                        ]),
                                                    }, data::graph::Edge {
                                                        target: data::graph::BlockId(4),
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
                                                            4,
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
                                            instructions: 0..3,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 1..1,
                                            instructions: 3..6,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 1..1,
                                            instructions: 6..9,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 1..1,
                                            instructions: 9..12,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(3)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 1..1,
                                            instructions: 12..15,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(4)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 1..1,
                                            instructions: 15..17,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(5)),
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
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    100,
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
                                                shape: data::type_::ValueShapeId(4),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(1),
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
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    101,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(4),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(5),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    index: 1,
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
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(4),
                                                        },
                                                    }),
                                                ]),
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
                                                    102,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(5),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(2),
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
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(2),
                                                            shape_id: data::type_::CustomValueShapeId(5),
                                                        },
                                                    }),
                                                ]),
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
                                                    103,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(7),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    index: 1,
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
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(2),
                                                            shape_id: data::type_::CustomValueShapeId(6),
                                                        },
                                                    }),
                                                ]),
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
                                                    104,
                                                ]),
                                            })),
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
                                                shape: data::type_::ValueShapeId(8),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    index: 2,
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
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 1,
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
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    105,
                                                ]),
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
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 2,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(1),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    }),
                                    data::function::FunctionExit::Return(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(1),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    }),
                                    data::function::FunctionExit::Return(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(1),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    }),
                                    data::function::FunctionExit::Return(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(1),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    }),
                                    data::function::FunctionExit::Return(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(1),
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
            const CALL_GROUP_0: [data::compiled::calls::CallStart; 2] = {
                use data::compiled::calls::{CallArguments, CallCustom, CallExecution, CallInputs, CallInteger, CallOps, CallOutput, CallProgress, CallStorage, CallValues};
                enum FunctionState {
                    Int0Point0 { int0: i128 },
                    Int0Point1 { int0: i128, custom0: CallCustom },
                    Int1Point0 { int0: i128 },
                    Int1Point1 { int0: i128, custom0: CallCustom },
                    Int2Point0 { custom0: CallCustom },
                    Int2Point1 { int0: i128 },
                    Int2Point2 { custom0: CallCustom },
                    Int2Point3 { int0: i128 },
                    Int2Point4 { custom0: CallCustom },
                    Int2Point5 { int0: i128 },
                    Int2Point6 { custom0: CallCustom },
                    Int2Point7 { int0: i128 },
                    Int2Point8 { custom0: CallCustom },
                    Int2Point9 { int0: i128 },
                    Int2Point10 { custom0: CallCustom },
                    Int2Point11 { custom0: CallCustom, int0: i128 },
                    Int3Point0 { custom0: CallCustom },
                    Int3Point1 { custom0: CallCustom },
                    Int3Point2 { int0: i128 },
                    Int3Point3 { custom0: CallCustom },
                    Int3Point4 { custom0: CallCustom, int0: i128 },
                    Int3Point5 { custom0: CallCustom },
                    Int3Point6 { custom0: CallCustom },
                    Int3Point7 { int0: i128 },
                    Int3Point8 { custom0: CallCustom },
                    Int3Point9 { int0: i128 },
                    Int3Point10 { custom0: CallCustom },
                    Int3Point11 { custom0: CallCustom, int0: i128 },
                    Int3Point12 { custom0: CallCustom },
                    Int3Point13 { custom0: CallCustom, int0: i128 },
                }
                enum IntReturn {
                }
                impl IntReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                        }
                    }
                    fn small(self, result: i128) -> FunctionState {
                        let _ = result;
                        match self {
                        }
                    }
                    fn resume(self, result: CallInteger) -> FunctionState {
                        if let Some(result) = result.small() {
                            return self.small(result);
                        }
                        match self {
                        }
                    }
                }
                enum CustomReturn {
                    Int0Call0 { int0: i128 },
                    Int1Call0 { int0: i128 },
                }
                impl CustomReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                            Self::Int0Call0 { .. } => data::source::HostCallSite::from_static("example", "selected", data::source::SourceSpan::new(1038, 1048)),
                            Self::Int1Call0 { .. } => data::source::HostCallSite::from_static("example", "selected_grouped", data::source::SourceSpan::new(1113, 1123)),
                        }
                    }
                    fn small(self, result: CallCustom) -> FunctionState {
                        match self {
                            Self::Int0Call0 { int0 } => {
                                let custom0 = result;
                                FunctionState::Int0Point1 { int0, custom0 }
                            },
                            Self::Int1Call0 { int0 } => {
                                let custom0 = result;
                                FunctionState::Int1Point1 { int0, custom0 }
                            },
                        }
                    }
                    fn resume(self, result: CallCustom) -> FunctionState { self.small(result) }
                }
                #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                enum FunctionStep {
                    Yield(FunctionState),
                    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                    IntTail { callee: FunctionState },
                    Int { value: i128 },
                    CustomBridge { function: data::function::CustomFunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: CustomReturn },
                }
                struct FunctionExecution {
                    active: Option<FunctionState>,
                    pending_entry: bool,
                    integer_returns: Vec<IntReturn>,
                    custom_returns: Vec<CustomReturn>,
                }
                impl FunctionExecution {
                    fn new(active: FunctionState) -> Self {
                        Self {
                            active: Some(active),
                            pending_entry: false,
                            integer_returns: Vec::new(),
                            custom_returns: Vec::new(),
                        }
                    }
                }
                impl CallExecution for FunctionExecution {
                    fn restart(&mut self, target: data::compiled::CallTarget, point: usize, values: CallInputs<'_>) -> bool {
                        if self.active.is_some() { return false; }
                        let active = match target {
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(0)) => calls_int_0_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(1)) => calls_int_1_state(point, values),
                            _ => None,
                        };
                        let Some(active) = active else { return false; };
                        self.active = Some(active);
                        true
                    }
                    fn retained_bytes(&self) -> usize {
                        std::mem::size_of::<Self>() + self.integer_returns.capacity() * std::mem::size_of::<IntReturn>() + self.custom_returns.capacity() * std::mem::size_of::<CustomReturn>()
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
                                        self.custom_returns.clear();
                                        return CallProgress::Complete { output: CallOutput::Int(value.into()), execution: self };
                                    }
                                },
                                FunctionStep::CustomBridge { function, site, arguments, caller } => return CallProgress::Custom {
                                    function, site, arguments,
                                    resume: Box::new(move |value| {
                                        self.active = Some(caller.resume(value));
                                        self
                                    }),
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
                                        data::compiled::CallTarget::Custom(function) => {
                                            if let Some(caller) = self.custom_returns.pop() {
                                                let site = caller.site();
                                                return CallProgress::InterpretedCustom {
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
                        FunctionState::Int0Point0 { int0 } => calls_int_0_run(Int0State::Point0 { int0 }, ops, budget),
                        FunctionState::Int0Point1 { int0, custom0 } => calls_int_0_run(Int0State::Point1 { int0, custom0 }, ops, budget),
                        FunctionState::Int1Point0 { int0 } => calls_int_1_run(Int1State::Point0 { int0 }, ops, budget),
                        FunctionState::Int1Point1 { int0, custom0 } => calls_int_1_run(Int1State::Point1 { int0, custom0 }, ops, budget),
                        FunctionState::Int2Point0 { custom0 } => calls_int_2_run(Int2State::Point0 { custom0 }, ops, budget),
                        FunctionState::Int2Point1 { int0 } => calls_int_2_run(Int2State::Point1 { int0 }, ops, budget),
                        FunctionState::Int2Point2 { custom0 } => calls_int_2_run(Int2State::Point2 { custom0 }, ops, budget),
                        FunctionState::Int2Point3 { int0 } => calls_int_2_run(Int2State::Point3 { int0 }, ops, budget),
                        FunctionState::Int2Point4 { custom0 } => calls_int_2_run(Int2State::Point4 { custom0 }, ops, budget),
                        FunctionState::Int2Point5 { int0 } => calls_int_2_run(Int2State::Point5 { int0 }, ops, budget),
                        FunctionState::Int2Point6 { custom0 } => calls_int_2_run(Int2State::Point6 { custom0 }, ops, budget),
                        FunctionState::Int2Point7 { int0 } => calls_int_2_run(Int2State::Point7 { int0 }, ops, budget),
                        FunctionState::Int2Point8 { custom0 } => calls_int_2_run(Int2State::Point8 { custom0 }, ops, budget),
                        FunctionState::Int2Point9 { int0 } => calls_int_2_run(Int2State::Point9 { int0 }, ops, budget),
                        FunctionState::Int2Point10 { custom0 } => calls_int_2_run(Int2State::Point10 { custom0 }, ops, budget),
                        FunctionState::Int2Point11 { custom0, int0 } => calls_int_2_run(Int2State::Point11 { custom0, int0 }, ops, budget),
                        FunctionState::Int3Point0 { custom0 } => calls_int_3_run(Int3State::Point0 { custom0 }, ops, budget),
                        FunctionState::Int3Point1 { custom0 } => calls_int_3_run(Int3State::Point1 { custom0 }, ops, budget),
                        FunctionState::Int3Point2 { int0 } => calls_int_3_run(Int3State::Point2 { int0 }, ops, budget),
                        FunctionState::Int3Point3 { custom0 } => calls_int_3_run(Int3State::Point3 { custom0 }, ops, budget),
                        FunctionState::Int3Point4 { custom0, int0 } => calls_int_3_run(Int3State::Point4 { custom0, int0 }, ops, budget),
                        FunctionState::Int3Point5 { custom0 } => calls_int_3_run(Int3State::Point5 { custom0 }, ops, budget),
                        FunctionState::Int3Point6 { custom0 } => calls_int_3_run(Int3State::Point6 { custom0 }, ops, budget),
                        FunctionState::Int3Point7 { int0 } => calls_int_3_run(Int3State::Point7 { int0 }, ops, budget),
                        FunctionState::Int3Point8 { custom0 } => calls_int_3_run(Int3State::Point8 { custom0 }, ops, budget),
                        FunctionState::Int3Point9 { int0 } => calls_int_3_run(Int3State::Point9 { int0 }, ops, budget),
                        FunctionState::Int3Point10 { custom0 } => calls_int_3_run(Int3State::Point10 { custom0 }, ops, budget),
                        FunctionState::Int3Point11 { custom0, int0 } => calls_int_3_run(Int3State::Point11 { custom0, int0 }, ops, budget),
                        FunctionState::Int3Point12 { custom0 } => calls_int_3_run(Int3State::Point12 { custom0 }, ops, budget),
                        FunctionState::Int3Point13 { custom0, int0 } => calls_int_3_run(Int3State::Point13 { custom0, int0 }, ops, budget),
                    }
                }
                enum Int0State {
                    Point0 { int0: i128 },
                    Point1 { int0: i128, custom0: CallCustom },
                }
                fn calls_int_0_run(active: Int0State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int0State::Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point0 { int0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::CustomBridge { function: data::function::CustomFunctionId {
                                    index: 0,
                                    return_shape: data::type_::CustomValueShape {
                                        type_id: data::type_::CustomTypeId(0),
                                        shape_id: data::type_::CustomValueShapeId(0),
                                    },
                                }, site: data::source::HostCallSite::from_static("example", "selected", data::source::SourceSpan::new(1038, 1048)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int0.into()], ..CallValues::default() }), captures: None }, caller: CustomReturn::Int0Call0 { int0 } }
                            }
                        },
                        Int0State::Point1 { int0, custom0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point1 { int0, custom0 }); }
                            {
                                FunctionStep::IntTail { callee: FunctionState::Int2Point0 { custom0: custom0.clone() } }
                            }
                        },
                    }
                }
                enum Int1State {
                    Point0 { int0: i128 },
                    Point1 { int0: i128, custom0: CallCustom },
                }
                fn calls_int_1_run(active: Int1State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Int1State::Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point0 { int0 }); }
                            *budget -= 1;
                            {
                                FunctionStep::CustomBridge { function: data::function::CustomFunctionId {
                                    index: 0,
                                    return_shape: data::type_::CustomValueShape {
                                        type_id: data::type_::CustomTypeId(0),
                                        shape_id: data::type_::CustomValueShapeId(0),
                                    },
                                }, site: data::source::HostCallSite::from_static("example", "selected_grouped", data::source::SourceSpan::new(1113, 1123)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int0.into()], ..CallValues::default() }), captures: None }, caller: CustomReturn::Int1Call0 { int0 } }
                            }
                        },
                        Int1State::Point1 { int0, custom0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point1 { int0, custom0 }); }
                            {
                                FunctionStep::IntTail { callee: FunctionState::Int3Point0 { custom0: custom0.clone() } }
                            }
                        },
                    }
                }
                enum Int2State {
                    Point0 { custom0: CallCustom },
                    Point1 { int0: i128 },
                    Point2 { custom0: CallCustom },
                    Point3 { int0: i128 },
                    Point4 { custom0: CallCustom },
                    Point5 { int0: i128 },
                    Point6 { custom0: CallCustom },
                    Point7 { int0: i128 },
                    Point8 { custom0: CallCustom },
                    Point9 { int0: i128 },
                    Point10 { custom0: CallCustom },
                    Point11 { custom0: CallCustom, int0: i128 },
                }
                fn calls_int_2_run(mut active: Int2State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            Int2State::Point0 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point0 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(i128,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(0),
                                            index: 0,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Custom(data::type_::CustomTypeId(1))) { return None; }
                                        if !field0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(1),
                                            index: 0,
                                        }) { return Some(None); }
                                        let field1 = field0.field(0)?;
                                        if !field1.matches_type(&data::type_::ValueType::Int) { return None; }
                                        let binding0 = field1.integer()?;
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: data::compiled::CompiledCheckpoint {
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
                                        Some((binding0,)) => Int2State::Point1 { int0: binding0 },
                                        None => Int2State::Point2 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int2State::Point1 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point1 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int2State::Point2 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point2 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(i128,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(0),
                                            index: 1,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Custom(data::type_::CustomTypeId(2))) { return None; }
                                        if !field0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(2),
                                            index: 0,
                                        }) { return Some(None); }
                                        let field1 = field0.field(0)?;
                                        if !field1.matches_type(&data::type_::ValueType::Int) { return None; }
                                        let binding0 = field1.integer()?;
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: data::compiled::CompiledCheckpoint {
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
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) }; };
                                    *budget -= 1;
                                    match matched {
                                        Some((binding0,)) => Int2State::Point3 { int0: binding0 },
                                        None => Int2State::Point4 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int2State::Point3 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point3 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int2State::Point4 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point4 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(i128,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(0),
                                            index: 0,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Custom(data::type_::CustomTypeId(1))) { return None; }
                                        if !field0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(1),
                                            index: 1,
                                        }) { return Some(None); }
                                        let field1 = field0.field(0)?;
                                        if !field1.matches_type(&data::type_::ValueType::Int) { return None; }
                                        let binding0 = field1.integer()?;
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: data::compiled::CompiledCheckpoint {
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
                                        Some((binding0,)) => Int2State::Point5 { int0: binding0 },
                                        None => Int2State::Point6 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int2State::Point5 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point5 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int2State::Point6 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point6 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(i128,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(0),
                                            index: 1,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Custom(data::type_::CustomTypeId(2))) { return None; }
                                        if !field0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(2),
                                            index: 2,
                                        }) { return Some(None); }
                                        let field1 = field0.field(0)?;
                                        if !field1.matches_type(&data::type_::ValueType::Int) { return None; }
                                        let binding0 = field1.integer()?;
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: data::compiled::CompiledCheckpoint {
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
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) }; };
                                    *budget -= 1;
                                    match matched {
                                        Some((binding0,)) => Int2State::Point7 { int0: binding0 },
                                        None => Int2State::Point8 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int2State::Point7 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point7 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int2State::Point8 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point8 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(i128,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(0),
                                            index: 1,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Custom(data::type_::CustomTypeId(2))) { return None; }
                                        if !field0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(2),
                                            index: 1,
                                        }) { return Some(None); }
                                        let field1 = field0.field(0)?;
                                        if !field1.matches_type(&data::type_::ValueType::Int) { return None; }
                                        let binding0 = field1.integer()?;
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(8),
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
                                        Some((binding0,)) => Int2State::Point9 { int0: binding0 },
                                        None => Int2State::Point10 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int2State::Point9 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point9 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int2State::Point10 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point10 { custom0 }); }
                                let int0 = match (|| {
                                let field = custom0.field(0)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(10),
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
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point11 { custom0, int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int2State::Point11 { custom0, int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point11 { custom0, int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                        }
                    }
                }
                enum Int3State {
                    Point0 { custom0: CallCustom },
                    Point1 { custom0: CallCustom },
                    Point2 { int0: i128 },
                    Point3 { custom0: CallCustom },
                    Point4 { custom0: CallCustom, int0: i128 },
                    Point5 { custom0: CallCustom },
                    Point6 { custom0: CallCustom },
                    Point7 { int0: i128 },
                    Point8 { custom0: CallCustom },
                    Point9 { int0: i128 },
                    Point10 { custom0: CallCustom },
                    Point11 { custom0: CallCustom, int0: i128 },
                    Point12 { custom0: CallCustom },
                    Point13 { custom0: CallCustom, int0: i128 },
                }
                fn calls_int_3_run(mut active: Int3State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            Int3State::Point0 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point0 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(CallCustom,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(0),
                                            index: 0,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Custom(data::type_::CustomTypeId(1))) { return None; }
                                        let binding0 = field0.custom()?;
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point: data::compiled::CompiledCheckpoint {
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
                                        Some((binding0,)) => Int3State::Point1 { custom0: binding0 },
                                        None => Int3State::Point5 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int3State::Point1 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point1 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(i128,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(1),
                                            index: 0,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Int) { return None; }
                                        let binding0 = field0.integer()?;
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point: data::compiled::CompiledCheckpoint {
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
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) }; };
                                    *budget -= 1;
                                    match matched {
                                        Some((binding0,)) => Int3State::Point2 { int0: binding0 },
                                        None => Int3State::Point3 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int3State::Point2 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point2 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int3State::Point3 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point3 { custom0 }); }
                                let int0 = match (|| {
                                let field = custom0.field(0)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point: data::compiled::CompiledCheckpoint {
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
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) },
                                };
                                *budget -= 1;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point4 { custom0, int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int3State::Point4 { custom0, int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point4 { custom0, int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int3State::Point5 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point5 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(CallCustom,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(0),
                                            index: 1,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Custom(data::type_::CustomTypeId(2))) { return None; }
                                        let binding0 = field0.custom()?;
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point: data::compiled::CompiledCheckpoint {
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
                                        Some((binding0,)) => Int3State::Point6 { custom0: binding0 },
                                        None => Int3State::Point12 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int3State::Point6 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point6 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(i128,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(2),
                                            index: 0,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Int) { return None; }
                                        let binding0 = field0.integer()?;
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(5),
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
                                        Some((binding0,)) => Int3State::Point7 { int0: binding0 },
                                        None => Int3State::Point8 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int3State::Point7 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point7 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int3State::Point8 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point8 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(i128,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(2),
                                            index: 2,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Int) { return None; }
                                        let binding0 = field0.integer()?;
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point: data::compiled::CompiledCheckpoint {
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
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) }; };
                                    *budget -= 1;
                                    match matched {
                                        Some((binding0,)) => Int3State::Point9 { int0: binding0 },
                                        None => Int3State::Point10 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int3State::Point9 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point9 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int3State::Point10 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point10 { custom0 }); }
                                let int0 = match (|| {
                                let field = custom0.field(0)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(9),
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
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point11 { custom0, int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int3State::Point11 { custom0, int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point11 { custom0, int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int3State::Point12 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point12 { custom0 }); }
                                let int0 = match (|| {
                                let field = custom0.field(0)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(10),
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
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point13 { custom0, int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int3State::Point13 { custom0, int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point13 { custom0, int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                        }
                    }
                }
                fn calls_int_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int0Point0 { int0: values.int(0)? },
                        1 => FunctionState::Int0Point1 { int0: values.int(0)?, custom0: values.custom(0)? },
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
                        0 => FunctionState::Int1Point0 { int0: values.int(0)? },
                        1 => FunctionState::Int1Point1 { int0: values.int(0)?, custom0: values.custom(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_1_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(1)), point, values) { return Some(execution); }
                    let active = calls_int_1_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                [calls_int_0_start, calls_int_1_start]
            };
            const CALL_GROUP_1: [data::compiled::calls::CallStart; 2] = {
                use data::compiled::calls::{CallCustom, CallExecution, CallInputs, CallInteger, CallOps, CallOutput, CallProgress, CallStorage, CallValues};
                enum FunctionState {
                    Int2Point0 { custom0: CallCustom },
                    Int2Point1 { int0: i128 },
                    Int2Point2 { custom0: CallCustom },
                    Int2Point3 { int0: i128 },
                    Int2Point4 { custom0: CallCustom },
                    Int2Point5 { int0: i128 },
                    Int2Point6 { custom0: CallCustom },
                    Int2Point7 { int0: i128 },
                    Int2Point8 { custom0: CallCustom },
                    Int2Point9 { int0: i128 },
                    Int2Point10 { custom0: CallCustom },
                    Int2Point11 { custom0: CallCustom, int0: i128 },
                    Int3Point0 { custom0: CallCustom },
                    Int3Point1 { custom0: CallCustom },
                    Int3Point2 { int0: i128 },
                    Int3Point3 { custom0: CallCustom },
                    Int3Point4 { custom0: CallCustom, int0: i128 },
                    Int3Point5 { custom0: CallCustom },
                    Int3Point6 { custom0: CallCustom },
                    Int3Point7 { int0: i128 },
                    Int3Point8 { custom0: CallCustom },
                    Int3Point9 { int0: i128 },
                    Int3Point10 { custom0: CallCustom },
                    Int3Point11 { custom0: CallCustom, int0: i128 },
                    Int3Point12 { custom0: CallCustom },
                    Int3Point13 { custom0: CallCustom, int0: i128 },
                }
                enum IntReturn {
                }
                impl IntReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                        }
                    }
                    fn small(self, result: i128) -> FunctionState {
                        let _ = result;
                        match self {
                        }
                    }
                    fn resume(self, result: CallInteger) -> FunctionState {
                        if let Some(result) = result.small() {
                            return self.small(result);
                        }
                        match self {
                        }
                    }
                }
                #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                enum FunctionStep {
                    Yield(FunctionState),
                    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                    Int { value: i128 },
                }
                struct FunctionExecution {
                    active: Option<FunctionState>,
                    integer_returns: Vec<IntReturn>,
                }
                impl FunctionExecution {
                    fn new(active: FunctionState) -> Self {
                        Self {
                            active: Some(active),
                            integer_returns: Vec::new(),
                        }
                    }
                }
                impl CallExecution for FunctionExecution {
                    fn restart(&mut self, target: data::compiled::CallTarget, point: usize, values: CallInputs<'_>) -> bool {
                        if self.active.is_some() { return false; }
                        let active = match target {
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(2)) => calls_int_2_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(3)) => calls_int_3_state(point, values),
                            _ => None,
                        };
                        let Some(active) = active else { return false; };
                        self.active = Some(active);
                        true
                    }
                    fn retained_bytes(&self) -> usize {
                        std::mem::size_of::<Self>() + self.integer_returns.capacity() * std::mem::size_of::<IntReturn>()
                    }
                    fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                        let Some(mut active) = self.active.take() else { return CallProgress::Yield(self); };
                        loop {
                            match function_step(active, ops, budget) {
                                FunctionStep::Yield(active) => {
                                    self.active = Some(active);
                                    return CallProgress::Yield(self);
                                },
                                FunctionStep::Int { value } => {
                                    if let Some(caller) = self.integer_returns.pop() {
                                        active = caller.small(value);
                                    } else {
                                        self.integer_returns.clear();
                                        return CallProgress::Complete { output: CallOutput::Int(value.into()), execution: self };
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
                                        _ => return CallProgress::Interpreted { target, point, values },
                                    }
                                },
                            }
                        }
                    }
                }
                fn function_step(active: FunctionState, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        FunctionState::Int2Point0 { custom0 } => calls_int_2_run(Int2State::Point0 { custom0 }, ops, budget),
                        FunctionState::Int2Point1 { int0 } => calls_int_2_run(Int2State::Point1 { int0 }, ops, budget),
                        FunctionState::Int2Point2 { custom0 } => calls_int_2_run(Int2State::Point2 { custom0 }, ops, budget),
                        FunctionState::Int2Point3 { int0 } => calls_int_2_run(Int2State::Point3 { int0 }, ops, budget),
                        FunctionState::Int2Point4 { custom0 } => calls_int_2_run(Int2State::Point4 { custom0 }, ops, budget),
                        FunctionState::Int2Point5 { int0 } => calls_int_2_run(Int2State::Point5 { int0 }, ops, budget),
                        FunctionState::Int2Point6 { custom0 } => calls_int_2_run(Int2State::Point6 { custom0 }, ops, budget),
                        FunctionState::Int2Point7 { int0 } => calls_int_2_run(Int2State::Point7 { int0 }, ops, budget),
                        FunctionState::Int2Point8 { custom0 } => calls_int_2_run(Int2State::Point8 { custom0 }, ops, budget),
                        FunctionState::Int2Point9 { int0 } => calls_int_2_run(Int2State::Point9 { int0 }, ops, budget),
                        FunctionState::Int2Point10 { custom0 } => calls_int_2_run(Int2State::Point10 { custom0 }, ops, budget),
                        FunctionState::Int2Point11 { custom0, int0 } => calls_int_2_run(Int2State::Point11 { custom0, int0 }, ops, budget),
                        FunctionState::Int3Point0 { custom0 } => calls_int_3_run(Int3State::Point0 { custom0 }, ops, budget),
                        FunctionState::Int3Point1 { custom0 } => calls_int_3_run(Int3State::Point1 { custom0 }, ops, budget),
                        FunctionState::Int3Point2 { int0 } => calls_int_3_run(Int3State::Point2 { int0 }, ops, budget),
                        FunctionState::Int3Point3 { custom0 } => calls_int_3_run(Int3State::Point3 { custom0 }, ops, budget),
                        FunctionState::Int3Point4 { custom0, int0 } => calls_int_3_run(Int3State::Point4 { custom0, int0 }, ops, budget),
                        FunctionState::Int3Point5 { custom0 } => calls_int_3_run(Int3State::Point5 { custom0 }, ops, budget),
                        FunctionState::Int3Point6 { custom0 } => calls_int_3_run(Int3State::Point6 { custom0 }, ops, budget),
                        FunctionState::Int3Point7 { int0 } => calls_int_3_run(Int3State::Point7 { int0 }, ops, budget),
                        FunctionState::Int3Point8 { custom0 } => calls_int_3_run(Int3State::Point8 { custom0 }, ops, budget),
                        FunctionState::Int3Point9 { int0 } => calls_int_3_run(Int3State::Point9 { int0 }, ops, budget),
                        FunctionState::Int3Point10 { custom0 } => calls_int_3_run(Int3State::Point10 { custom0 }, ops, budget),
                        FunctionState::Int3Point11 { custom0, int0 } => calls_int_3_run(Int3State::Point11 { custom0, int0 }, ops, budget),
                        FunctionState::Int3Point12 { custom0 } => calls_int_3_run(Int3State::Point12 { custom0 }, ops, budget),
                        FunctionState::Int3Point13 { custom0, int0 } => calls_int_3_run(Int3State::Point13 { custom0, int0 }, ops, budget),
                    }
                }
                enum Int2State {
                    Point0 { custom0: CallCustom },
                    Point1 { int0: i128 },
                    Point2 { custom0: CallCustom },
                    Point3 { int0: i128 },
                    Point4 { custom0: CallCustom },
                    Point5 { int0: i128 },
                    Point6 { custom0: CallCustom },
                    Point7 { int0: i128 },
                    Point8 { custom0: CallCustom },
                    Point9 { int0: i128 },
                    Point10 { custom0: CallCustom },
                    Point11 { custom0: CallCustom, int0: i128 },
                }
                fn calls_int_2_run(mut active: Int2State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            Int2State::Point0 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point0 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(i128,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(0),
                                            index: 0,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Custom(data::type_::CustomTypeId(1))) { return None; }
                                        if !field0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(1),
                                            index: 0,
                                        }) { return Some(None); }
                                        let field1 = field0.field(0)?;
                                        if !field1.matches_type(&data::type_::ValueType::Int) { return None; }
                                        let binding0 = field1.integer()?;
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: data::compiled::CompiledCheckpoint {
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
                                        Some((binding0,)) => Int2State::Point1 { int0: binding0 },
                                        None => Int2State::Point2 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int2State::Point1 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point1 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int2State::Point2 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point2 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(i128,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(0),
                                            index: 1,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Custom(data::type_::CustomTypeId(2))) { return None; }
                                        if !field0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(2),
                                            index: 0,
                                        }) { return Some(None); }
                                        let field1 = field0.field(0)?;
                                        if !field1.matches_type(&data::type_::ValueType::Int) { return None; }
                                        let binding0 = field1.integer()?;
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: data::compiled::CompiledCheckpoint {
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
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) }; };
                                    *budget -= 1;
                                    match matched {
                                        Some((binding0,)) => Int2State::Point3 { int0: binding0 },
                                        None => Int2State::Point4 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int2State::Point3 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point3 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int2State::Point4 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point4 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(i128,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(0),
                                            index: 0,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Custom(data::type_::CustomTypeId(1))) { return None; }
                                        if !field0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(1),
                                            index: 1,
                                        }) { return Some(None); }
                                        let field1 = field0.field(0)?;
                                        if !field1.matches_type(&data::type_::ValueType::Int) { return None; }
                                        let binding0 = field1.integer()?;
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: data::compiled::CompiledCheckpoint {
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
                                        Some((binding0,)) => Int2State::Point5 { int0: binding0 },
                                        None => Int2State::Point6 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int2State::Point5 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point5 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int2State::Point6 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point6 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(i128,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(0),
                                            index: 1,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Custom(data::type_::CustomTypeId(2))) { return None; }
                                        if !field0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(2),
                                            index: 2,
                                        }) { return Some(None); }
                                        let field1 = field0.field(0)?;
                                        if !field1.matches_type(&data::type_::ValueType::Int) { return None; }
                                        let binding0 = field1.integer()?;
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: data::compiled::CompiledCheckpoint {
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
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) }; };
                                    *budget -= 1;
                                    match matched {
                                        Some((binding0,)) => Int2State::Point7 { int0: binding0 },
                                        None => Int2State::Point8 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int2State::Point7 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point7 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int2State::Point8 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point8 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(i128,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(0),
                                            index: 1,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Custom(data::type_::CustomTypeId(2))) { return None; }
                                        if !field0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(2),
                                            index: 1,
                                        }) { return Some(None); }
                                        let field1 = field0.field(0)?;
                                        if !field1.matches_type(&data::type_::ValueType::Int) { return None; }
                                        let binding0 = field1.integer()?;
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(8),
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
                                        Some((binding0,)) => Int2State::Point9 { int0: binding0 },
                                        None => Int2State::Point10 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int2State::Point9 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point9 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int2State::Point10 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point10 { custom0 }); }
                                let int0 = match (|| {
                                let field = custom0.field(0)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(10),
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
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point11 { custom0, int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int2State::Point11 { custom0, int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point11 { custom0, int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                        }
                    }
                }
                enum Int3State {
                    Point0 { custom0: CallCustom },
                    Point1 { custom0: CallCustom },
                    Point2 { int0: i128 },
                    Point3 { custom0: CallCustom },
                    Point4 { custom0: CallCustom, int0: i128 },
                    Point5 { custom0: CallCustom },
                    Point6 { custom0: CallCustom },
                    Point7 { int0: i128 },
                    Point8 { custom0: CallCustom },
                    Point9 { int0: i128 },
                    Point10 { custom0: CallCustom },
                    Point11 { custom0: CallCustom, int0: i128 },
                    Point12 { custom0: CallCustom },
                    Point13 { custom0: CallCustom, int0: i128 },
                }
                fn calls_int_3_run(mut active: Int3State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    loop {
                        match active {
                            Int3State::Point0 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point0 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(CallCustom,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(0),
                                            index: 0,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Custom(data::type_::CustomTypeId(1))) { return None; }
                                        let binding0 = field0.custom()?;
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point: data::compiled::CompiledCheckpoint {
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
                                        Some((binding0,)) => Int3State::Point1 { custom0: binding0 },
                                        None => Int3State::Point5 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int3State::Point1 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point1 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(i128,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(1),
                                            index: 0,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Int) { return None; }
                                        let binding0 = field0.integer()?;
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point: data::compiled::CompiledCheckpoint {
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
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) }; };
                                    *budget -= 1;
                                    match matched {
                                        Some((binding0,)) => Int3State::Point2 { int0: binding0 },
                                        None => Int3State::Point3 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int3State::Point2 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point2 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int3State::Point3 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point3 { custom0 }); }
                                let int0 = match (|| {
                                let field = custom0.field(0)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point: data::compiled::CompiledCheckpoint {
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
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) },
                                };
                                *budget -= 1;
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point4 { custom0, int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int3State::Point4 { custom0, int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point4 { custom0, int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int3State::Point5 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point5 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(CallCustom,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(0),
                                            index: 1,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Custom(data::type_::CustomTypeId(2))) { return None; }
                                        let binding0 = field0.custom()?;
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point: data::compiled::CompiledCheckpoint {
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
                                        Some((binding0,)) => Int3State::Point6 { custom0: binding0 },
                                        None => Int3State::Point12 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int3State::Point6 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point6 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(i128,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(2),
                                            index: 0,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Int) { return None; }
                                        let binding0 = field0.integer()?;
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(5),
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
                                        Some((binding0,)) => Int3State::Point7 { int0: binding0 },
                                        None => Int3State::Point8 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int3State::Point7 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point7 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int3State::Point8 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point8 { custom0 }); }
                                active = {
                                    let matched = (|| -> Option<Option<(i128,)>> {
                                        if !custom0.matches_constructor(data::type_::CustomConstructorId {
                                            type_id: data::type_::CustomTypeId(2),
                                            index: 2,
                                        }) { return Some(None); }
                                        let field0 = custom0.field(0)?;
                                        if !field0.matches_type(&data::type_::ValueType::Int) { return None; }
                                        let binding0 = field0.integer()?;
                                        Some(Some((binding0,)))
                                    })();
                                    let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point: data::compiled::CompiledCheckpoint {
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
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) }; };
                                    *budget -= 1;
                                    match matched {
                                        Some((binding0,)) => Int3State::Point9 { int0: binding0 },
                                        None => Int3State::Point10 { custom0: custom0.clone() },
                                    }
                                };
                                continue;
                            },
                            Int3State::Point9 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point9 { int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int3State::Point10 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point10 { custom0 }); }
                                let int0 = match (|| {
                                let field = custom0.field(0)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(9),
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
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point11 { custom0, int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int3State::Point11 { custom0, int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point11 { custom0, int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int3State::Point12 { custom0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point12 { custom0 }); }
                                let int0 = match (|| {
                                let field = custom0.field(0)?;
                                    if !field.matches_type(&data::type_::ValueType::Int) { return None; }
                                    field.integer()
                                })() {
                                    Some(value) => value,
                                    None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(10),
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
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point13 { custom0, int0 }); }
                                *budget -= 1;
                                return {
                                    FunctionStep::Int { value: int0 }
                                };
                            },
                            Int3State::Point13 { custom0, int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point13 { custom0, int0 }); }
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
                        0 => FunctionState::Int2Point0 { custom0: values.custom(0)? },
                        1 => FunctionState::Int2Point1 { int0: values.int(0)? },
                        2 => FunctionState::Int2Point2 { custom0: values.custom(0)? },
                        3 => FunctionState::Int2Point3 { int0: values.int(0)? },
                        4 => FunctionState::Int2Point4 { custom0: values.custom(0)? },
                        5 => FunctionState::Int2Point5 { int0: values.int(0)? },
                        6 => FunctionState::Int2Point6 { custom0: values.custom(0)? },
                        7 => FunctionState::Int2Point7 { int0: values.int(0)? },
                        8 => FunctionState::Int2Point8 { custom0: values.custom(0)? },
                        9 => FunctionState::Int2Point9 { int0: values.int(0)? },
                        10 => FunctionState::Int2Point10 { custom0: values.custom(0)? },
                        11 => FunctionState::Int2Point11 { custom0: values.custom(0)?, int0: values.int(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_2_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point, values) { return Some(execution); }
                    let active = calls_int_2_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_3_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int3Point0 { custom0: values.custom(0)? },
                        1 => FunctionState::Int3Point1 { custom0: values.custom(0)? },
                        2 => FunctionState::Int3Point2 { int0: values.int(0)? },
                        3 => FunctionState::Int3Point3 { custom0: values.custom(0)? },
                        4 => FunctionState::Int3Point4 { custom0: values.custom(0)?, int0: values.int(0)? },
                        5 => FunctionState::Int3Point5 { custom0: values.custom(0)? },
                        6 => FunctionState::Int3Point6 { custom0: values.custom(0)? },
                        7 => FunctionState::Int3Point7 { int0: values.int(0)? },
                        8 => FunctionState::Int3Point8 { custom0: values.custom(0)? },
                        9 => FunctionState::Int3Point9 { int0: values.int(0)? },
                        10 => FunctionState::Int3Point10 { custom0: values.custom(0)? },
                        11 => FunctionState::Int3Point11 { custom0: values.custom(0)?, int0: values.int(0)? },
                        12 => FunctionState::Int3Point12 { custom0: values.custom(0)? },
                        13 => FunctionState::Int3Point13 { custom0: values.custom(0)?, int0: values.int(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_3_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point, values) { return Some(execution); }
                    let active = calls_int_3_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                [calls_int_2_start, calls_int_3_start]
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
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)),
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
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    }),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[
                                data::compiled::CallContract {
                                    point: 0,
                                    output: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    }),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Custom(data::function::CustomFunctionId {
                                        index: 0,
                                        return_shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    })),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "selected", data::source::SourceSpan::new(1038, 1048)),
                                },
                            ]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[]),
                            tails: data::Storage::Static(&[
                                data::compiled::TailContract {
                                    point: 1,
                                    target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "selected", data::source::SourceSpan::new(1031, 1049)),
                                },
                            ]),
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
                                    bools: 0,
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
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    }),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[
                                data::compiled::CallContract {
                                    point: 0,
                                    output: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    }),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Custom(data::function::CustomFunctionId {
                                        index: 0,
                                        return_shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    })),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "selected_grouped", data::source::SourceSpan::new(1113, 1123)),
                                },
                            ]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[]),
                            tails: data::Storage::Static(&[
                                data::compiled::TailContract {
                                    point: 1,
                                    target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(0),
                                            },
                                        }),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "selected_grouped", data::source::SourceSpan::new(1098, 1124)),
                                },
                            ]),
                            start: CALL_GROUP_0[1],
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
                                    customs: 1,
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
                                    customs: 1,
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
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(10),
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
                                    block: data::graph::BlockId(10),
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
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 1,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                },
                                data::compiled::ReturnContract {
                                    point: 3,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                },
                                data::compiled::ReturnContract {
                                    point: 5,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                },
                                data::compiled::ReturnContract {
                                    point: 7,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                },
                                data::compiled::ReturnContract {
                                    point: 9,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                },
                                data::compiled::ReturnContract {
                                    point: 11,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_1[0],
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
                                    customs: 1,
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
                                    customs: 1,
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
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(9),
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
                                    block: data::graph::BlockId(9),
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
                                    block: data::graph::BlockId(10),
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
                                    block: data::graph::BlockId(10),
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
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(1),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(1),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(1),
                                            shape_id: data::type_::CustomValueShapeId(1),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(2),
                                            shape_id: data::type_::CustomValueShapeId(2),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(2),
                                            shape_id: data::type_::CustomValueShapeId(2),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(2),
                                            shape_id: data::type_::CustomValueShapeId(2),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(2),
                                            shape_id: data::type_::CustomValueShapeId(2),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(0),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                        id: data::graph::CustomLocalId(0),
                                        shape: data::type_::CustomValueShape {
                                            type_id: data::type_::CustomTypeId(0),
                                            shape_id: data::type_::CustomValueShapeId(0),
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
                                data::compiled::ReturnContract {
                                    point: 4,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                },
                                data::compiled::ReturnContract {
                                    point: 7,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                },
                                data::compiled::ReturnContract {
                                    point: 9,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                },
                                data::compiled::ReturnContract {
                                    point: 11,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                },
                                data::compiled::ReturnContract {
                                    point: 13,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_1[1],
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
                0..4,
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
                0..0,
                0..0,
                0..0,
                0..0,
            ],
            functions: data::Storage::Static(&[
                data::function::FunctionContract {
                    parameters: 0..1,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 1..2,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 2..3,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(1),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 3..4,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(1),
                    ]),
                    return_: data::type_::ValueShapeId(0),
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
            ]),
            parameters: data::Storage::Static(&[
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(0),
                        shape_id: data::type_::CustomValueShapeId(0),
                    },
                }),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(0),
                        shape_id: data::type_::CustomValueShapeId(0),
                    },
                }),
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
            types: data::Storage::Static(&[
                data::type_::CustomTypeDescriptor {
                    type_: data::type_::NominalTypeMetadata {
                        package: data::Text::Static("geam"),
                        module: data::Text::Static("example"),
                        name: data::Text::Static("Frame"),
                        arguments: data::Storage::Static(&[]),
                    },
                    native_visible: false,
                    lifetime: data::host::HostValueLifetime::LoadedOwner,
                    constructor_count: 3,
                    constructors: data::Storage::Static(&[
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(0),
                                index: 0,
                            },
                            name: data::Text::Static("Data"),
                            native_tag: data::Text::Static("data"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                                    shape: data::type_::ValueShapeId(2),
                                    refinement: data::type_::FieldRefinement::Custom(data::Storage::Static(&[])),
                                },
                            ]),
                        },
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(0),
                                index: 1,
                            },
                            name: data::Text::Static("Control"),
                            native_tag: data::Text::Static("control"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                                    shape: data::type_::ValueShapeId(3),
                                    refinement: data::type_::FieldRefinement::Custom(data::Storage::Static(&[])),
                                },
                            ]),
                        },
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(0),
                                index: 2,
                            },
                            name: data::Text::Static("Continuation"),
                            native_tag: data::Text::Static("continuation"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::Int,
                                    shape: data::type_::ValueShapeId(0),
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
                        name: data::Text::Static("DataFrame"),
                        arguments: data::Storage::Static(&[]),
                    },
                    native_visible: false,
                    lifetime: data::host::HostValueLifetime::LoadedOwner,
                    constructor_count: 2,
                    constructors: data::Storage::Static(&[
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(1),
                                index: 0,
                            },
                            name: data::Text::Static("Text"),
                            native_tag: data::Text::Static("text"),
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
                                type_id: data::type_::CustomTypeId(1),
                                index: 1,
                            },
                            name: data::Text::Static("Binary"),
                            native_tag: data::Text::Static("binary"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::Int,
                                    shape: data::type_::ValueShapeId(0),
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
                        name: data::Text::Static("ControlFrame"),
                        arguments: data::Storage::Static(&[]),
                    },
                    native_visible: false,
                    lifetime: data::host::HostValueLifetime::LoadedOwner,
                    constructor_count: 3,
                    constructors: data::Storage::Static(&[
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(2),
                                index: 0,
                            },
                            name: data::Text::Static("Close"),
                            native_tag: data::Text::Static("close"),
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
                                type_id: data::type_::CustomTypeId(2),
                                index: 1,
                            },
                            name: data::Text::Static("Ping"),
                            native_tag: data::Text::Static("ping"),
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
                                type_id: data::type_::CustomTypeId(2),
                                index: 2,
                            },
                            name: data::Text::Static("Pong"),
                            native_tag: data::Text::Static("pong"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::Int,
                                    shape: data::type_::ValueShapeId(0),
                                    refinement: data::type_::FieldRefinement::Value,
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
                    name: data::Text::Static("ControlFrame"),
                    publicity: data::type_::CustomTypePublicity::Private,
                    opaque: false,
                    native_access: None,
                    retention_lifetime: data::host::HostValueLifetime::LoadedOwner,
                    parameters: 0,
                    constructors: data::Storage::Static(&[
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Close"),
                            fields: data::Storage::Static(&[
                                data::type_::FieldDefinition {
                                    label: None,
                                    type_: data::type_::TypeMetadata::Int,
                                },
                            ]),
                        },
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Ping"),
                            fields: data::Storage::Static(&[
                                data::type_::FieldDefinition {
                                    label: None,
                                    type_: data::type_::TypeMetadata::Int,
                                },
                            ]),
                        },
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Pong"),
                            fields: data::Storage::Static(&[
                                data::type_::FieldDefinition {
                                    label: None,
                                    type_: data::type_::TypeMetadata::Int,
                                },
                            ]),
                        },
                    ]),
                },
                data::type_::CustomDefinition {
                    package: data::Text::Static("geam"),
                    module: data::Text::Static("example"),
                    name: data::Text::Static("DataFrame"),
                    publicity: data::type_::CustomTypePublicity::Private,
                    opaque: false,
                    native_access: None,
                    retention_lifetime: data::host::HostValueLifetime::LoadedOwner,
                    parameters: 0,
                    constructors: data::Storage::Static(&[
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Text"),
                            fields: data::Storage::Static(&[
                                data::type_::FieldDefinition {
                                    label: None,
                                    type_: data::type_::TypeMetadata::Int,
                                },
                            ]),
                        },
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Binary"),
                            fields: data::Storage::Static(&[
                                data::type_::FieldDefinition {
                                    label: None,
                                    type_: data::type_::TypeMetadata::Int,
                                },
                            ]),
                        },
                    ]),
                },
                data::type_::CustomDefinition {
                    package: data::Text::Static("geam"),
                    module: data::Text::Static("example"),
                    name: data::Text::Static("Frame"),
                    publicity: data::type_::CustomTypePublicity::Private,
                    opaque: false,
                    native_access: None,
                    retention_lifetime: data::host::HostValueLifetime::LoadedOwner,
                    parameters: 0,
                    constructors: data::Storage::Static(&[
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Data"),
                            fields: data::Storage::Static(&[
                                data::type_::FieldDefinition {
                                    label: None,
                                    type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                        package: data::Text::Static("geam"),
                                        module: data::Text::Static("example"),
                                        name: data::Text::Static("DataFrame"),
                                        arguments: data::Storage::Static(&[]),
                                    }),
                                },
                            ]),
                        },
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Control"),
                            fields: data::Storage::Static(&[
                                data::type_::FieldDefinition {
                                    label: None,
                                    type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                        package: data::Text::Static("geam"),
                                        module: data::Text::Static("example"),
                                        name: data::Text::Static("ControlFrame"),
                                        arguments: data::Storage::Static(&[]),
                                    }),
                                },
                            ]),
                        },
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Continuation"),
                            fields: data::Storage::Static(&[
                                data::type_::FieldDefinition {
                                    label: None,
                                    type_: data::type_::TypeMetadata::Int,
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
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(3)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(4)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(5)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(6)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(7)),
            ]),
            shape_types: data::Storage::Static(&[
                data::type_::ValueType::Int,
                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
            ]),
            custom_shapes: data::Storage::Static(&[
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(0),
                    arguments: data::Storage::Static(&[]),
                    constructor: data::type_::CustomConstructorRefinement::Any,
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(1),
                    arguments: data::Storage::Static(&[]),
                    constructor: data::type_::CustomConstructorRefinement::Any,
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(2),
                    arguments: data::Storage::Static(&[]),
                    constructor: data::type_::CustomConstructorRefinement::Any,
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(1),
                    arguments: data::Storage::Static(&[]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(1),
                    arguments: data::Storage::Static(&[]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(1),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(2),
                    arguments: data::Storage::Static(&[]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(0),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(2),
                    arguments: data::Storage::Static(&[]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(1),
                },
                data::type_::CustomValueShapeDescriptor {
                    type_id: data::type_::CustomTypeId(2),
                    arguments: data::Storage::Static(&[]),
                    constructor: data::type_::CustomConstructorRefinement::Exact(2),
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
        ]),
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
    exports: data::Storage::Static(&[
        data::Export {
            name: data::Text::Static("selected"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 0,
        },
        data::Export {
            name: data::Text::Static("selected_grouped"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 1,
        },
    ]),
}
