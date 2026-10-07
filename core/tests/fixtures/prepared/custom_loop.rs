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
                                        instructions: 0..7,
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
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
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
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                                7,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(2),
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
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(3),
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
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(0),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::CustomTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                            item_type: data::type_::CustomTypeId(0),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            },
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(1),
                                                },
                                            },
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(2),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            },
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(3),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(2),
                                                },
                                            },
                                        ])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                            family: data::function::FunctionReturnFamily::Int,
                                            kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(12))),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(13),
                                        site: data::source::HostCallSite::from_static("example", "run", data::source::SourceSpan::new(583, 646)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(0),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::CustomTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                    transfer: data::graph::Transfer {
                                        families: data::Storage::Static(&[
                                            data::graph::FamilyTransfer {
                                                family: data::graph::StorageFamily::Int,
                                                length: 1,
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
                                                target: data::graph::BlockId(3),
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
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::IntFunction {
                                                        local: data::graph::IntFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
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
                                        params: 3..5,
                                        instructions: 1..7,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 5..6,
                                        instructions: 7..8,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::IntFunction {
                                                        local: data::graph::IntFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
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
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(7),
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
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        shape: data::type_::ValueShapeId(6),
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
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                            family: data::function::FunctionReturnFamily::Int,
                                            kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(14))),
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
                                                7,
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
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                                2,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
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
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(2),
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
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(0),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::CustomTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                            item_type: data::type_::CustomTypeId(0),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            },
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(1),
                                                },
                                            },
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(2),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(2),
                                                },
                                            },
                                        ])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                            family: data::function::FunctionReturnFamily::Int,
                                            kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(12))),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(13),
                                        site: data::source::HostCallSite::from_static("example", "chosen", data::source::SourceSpan::new(773, 819)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(0),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::CustomTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                    transfer: data::graph::Transfer {
                                        families: data::Storage::Static(&[
                                            data::graph::FamilyTransfer {
                                                family: data::graph::StorageFamily::Int,
                                                length: 1,
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
                                        instructions: 0..7,
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
                                        shape: data::type_::ValueShapeId(7),
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
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
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
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
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
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(2),
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
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(0),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::CustomTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                            item_type: data::type_::CustomTypeId(0),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            },
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(1),
                                                },
                                            },
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(2),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(2),
                                                },
                                            },
                                        ])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                            family: data::function::FunctionReturnFamily::Int,
                                            kind: data::graph::FunctionInstructionKind::Closure {
                                                target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(15)),
                                                captures: data::Storage::Static(&[
                                                    data::graph::FunctionCapture::Bool {
                                                        target: data::graph::BoolLocalId(0),
                                                        source: data::graph::BoolLocalId(0),
                                                    },
                                                    data::graph::FunctionCapture::Int {
                                                        target: data::graph::IntLocalId(1),
                                                        source: data::graph::IntLocalId(1),
                                                    },
                                                ]),
                                            },
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(13),
                                        site: data::source::HostCallSite::from_static("example", "captured", data::source::SourceSpan::new(887, 1165)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(0),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::CustomTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                    transfer: data::graph::Transfer {
                                        families: data::Storage::Static(&[
                                            data::graph::FamilyTransfer {
                                                family: data::graph::StorageFamily::Int,
                                                length: 1,
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
                                        instructions: 0..6,
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
                                            sign: data::Sign::Plus,
                                            digits: data::Storage::Static(&[
                                                1,
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
                                                id: data::graph::CustomLocalId(1),
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
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(0),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::CustomTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                            item_type: data::type_::CustomTypeId(0),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            },
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(1),
                                                },
                                            },
                                        ])))),
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
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                            family: data::function::FunctionReturnFamily::Int,
                                            kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(16))),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(13),
                                        site: data::source::HostCallSite::from_static("example", "assertion", data::source::SourceSpan::new(1735, 1779)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(0),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::CustomTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
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
                                        instructions: 0..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                ]),
                                params: data::Storage::Static(&[]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(0),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::CustomTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                            item_type: data::type_::CustomTypeId(0),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
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
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
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
                                        function: data::function::IntFunctionId(13),
                                        site: data::source::HostCallSite::from_static("example", "empty", data::source::SourceSpan::new(1930, 1948)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(0),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::CustomTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
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
                                        instructions: 0..5,
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
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(0),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::CustomTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                            item_type: data::type_::CustomTypeId(0),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(2),
                                                },
                                            },
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            },
                                        ])))),
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
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
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
                                        function: data::function::IntFunctionId(13),
                                        site: data::source::HostCallSite::from_static("example", "panic_case", data::source::SourceSpan::new(1993, 2026)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(0),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::CustomTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
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
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(0),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::CustomTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                            item_type: data::type_::CustomTypeId(0),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            },
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(2),
                                                },
                                            },
                                        ])))),
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
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
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
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(13),
                                        site: data::source::HostCallSite::from_static("example", "unsupported", data::source::SourceSpan::new(2237, 2274)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(0),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::CustomTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
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
                                        instructions: 0..8,
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
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                                7,
                                            ]),
                                        })),
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
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
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
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(2),
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
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(3),
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
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(0),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::CustomTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                            item_type: data::type_::CustomTypeId(0),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            },
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(2),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(1),
                                                },
                                            },
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(3),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(2),
                                                },
                                            },
                                        ])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                            family: data::function::FunctionReturnFamily::Int,
                                            kind: data::graph::FunctionInstructionKind::Closure {
                                                target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(19)),
                                                captures: data::Storage::Static(&[
                                                    data::graph::FunctionCapture::Custom {
                                                        target: data::graph::CustomLocal {
                                                            id: data::graph::CustomLocalId(1),
                                                            shape: data::type_::CustomValueShape {
                                                                type_id: data::type_::CustomTypeId(0),
                                                                shape_id: data::type_::CustomValueShapeId(0),
                                                            },
                                                        },
                                                        source: data::graph::CustomLocal {
                                                            id: data::graph::CustomLocalId(0),
                                                            shape: data::type_::CustomValueShape {
                                                                type_id: data::type_::CustomTypeId(0),
                                                                shape_id: data::type_::CustomValueShapeId(0),
                                                            },
                                                        },
                                                    },
                                                ]),
                                            },
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(13),
                                        site: data::source::HostCallSite::from_static("example", "custom_capture", data::source::SourceSpan::new(2360, 2626)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(0),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::CustomTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                    transfer: data::graph::Transfer {
                                        families: data::Storage::Static(&[
                                            data::graph::FamilyTransfer {
                                                family: data::graph::StorageFamily::Int,
                                                length: 1,
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
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(0),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::CustomTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                            item_type: data::type_::CustomTypeId(0),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(1),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::CustomTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                            item_type: data::type_::CustomTypeId(0),
                                        }, data::graph::TypedListInstruction::Call {
                                            function: data::function::CustomListFunctionId {
                                                index: 0,
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::CustomTypeId(0),
                                                },
                                            },
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                    local: data::graph::CustomListLocalId(0),
                                                    type_id: data::type_::CustomListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item_type: data::type_::CustomTypeId(0),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "repeated", data::source::SourceSpan::new(2883, 2913)),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                            family: data::function::FunctionReturnFamily::Int,
                                            kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(12))),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(13),
                                        site: data::source::HostCallSite::from_static("example", "repeated", data::source::SourceSpan::new(2878, 2928)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(1),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::CustomTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
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
                                                family: data::graph::StorageFamily::CustomList,
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
                                        instructions: 0..6,
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
                                            sign: data::Sign::Plus,
                                            digits: data::Storage::Static(&[
                                                2,
                                            ]),
                                        })),
                                    }),
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
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                                3,
                                            ]),
                                        })),
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
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(0),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::CustomTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                            item_type: data::type_::CustomTypeId(0),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(1),
                                                },
                                            },
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            },
                                        ])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                            family: data::function::FunctionReturnFamily::Int,
                                            kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(12))),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(20),
                                        site: data::source::HostCallSite::from_static("example", "caller_overflow", data::source::SourceSpan::new(3183, 3233)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(0),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::CustomTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                    transfer: data::graph::Transfer {
                                        families: data::Storage::Static(&[
                                            data::graph::FamilyTransfer {
                                                family: data::graph::StorageFamily::Int,
                                                length: 1,
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
                                        instructions: 0..6,
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
                                        shape: data::type_::ValueShapeId(7),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(9),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(1),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                                3,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(7),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(9),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                            constructor: data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(1),
                                                index: 0,
                                            },
                                            fields: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(0),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(1),
                                                    item_type: data::type_::CustomTypeId(1),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(10),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                            list_type: data::type_::ListTypeId(1),
                                            item_type: data::type_::CustomTypeId(1),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            },
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(4),
                                                },
                                            },
                                        ])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(12),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                            family: data::function::FunctionReturnFamily::Int,
                                            kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(21))),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(22),
                                        site: data::source::HostCallSite::from_static("example", "guarded", data::source::SourceSpan::new(3773, 3847)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(0),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(1),
                                                item_type: data::type_::CustomTypeId(1),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                    transfer: data::graph::Transfer {
                                        families: data::Storage::Static(&[
                                            data::graph::FamilyTransfer {
                                                family: data::graph::StorageFamily::Int,
                                                length: 1,
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
                                        instructions: 0..17,
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
                                                2,
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
                                                9,
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
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3876, 3885)),
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
                                                1,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(7),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3888, 3903)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(4)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
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
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(7),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(2),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3906, 3926)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(5)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(8)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(4),
                                            args: data::Storage::Static(&[]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3929, 3936)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(9)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(10)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(6),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3939, 3953)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(11)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(13)),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(14)),
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
                                                    shape_id: data::type_::CustomValueShapeId(3),
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
                                                            shape_id: data::type_::CustomValueShapeId(3),
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
                                                    shape_id: data::type_::CustomValueShapeId(3),
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
                                                shape_id: data::type_::CustomValueShapeId(3),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(4),
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
                                                shape_id: data::type_::CustomValueShapeId(3),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(4),
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
                                            test: data::graph::BoolTest::ListLengthEquals {
                                                value: data::graph::ListLocal::Custom {
                                                    local: data::graph::CustomListLocalId(0),
                                                    type_id: data::type_::CustomListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item_type: data::type_::CustomTypeId(0),
                                                    },
                                                },
                                                length: 0,
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::CustomList,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::IntFunction,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                        local: data::graph::CustomListLocalId(0),
                                                        type_id: data::type_::CustomListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::CustomTypeId(0),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::IntFunction {
                                                        local: data::graph::IntFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        },
                                                    },
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
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..7,
                                        instructions: 0..3,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                        local: data::graph::CustomListLocalId(1),
                                                        type_id: data::type_::CustomListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::CustomTypeId(0),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    data::graph::ParamLocal::IntFunction {
                                                        local: data::graph::IntFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        },
                                                    },
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
                                                            family: data::graph::StorageFamily::CustomList,
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
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(0),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::CustomTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(5),
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
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(0),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::CustomTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(5),
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
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(3),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::ListIndex {
                                            list: data::graph::CustomListLocalId(0),
                                            index: 0,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(1),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::CustomTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                            item_type: data::type_::CustomTypeId(0),
                                        }, data::graph::TypedListInstruction::DropFirst {
                                            list: data::graph::CustomListLocalId(0),
                                            count: 1,
                                        })),
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
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(3),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "fold", data::source::SourceSpan::new(194, 212)),
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
                                                    shape_id: data::type_::CustomValueShapeId(3),
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
                                                            shape_id: data::type_::CustomValueShapeId(3),
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
                                                    shape_id: data::type_::CustomValueShapeId(3),
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
                                                shape_id: data::type_::CustomValueShapeId(3),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(4),
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
                                                shape_id: data::type_::CustomValueShapeId(3),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(4),
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
                                    data::graph::ProfiledInstruction::IntegerRegion(data::graph::ArithmeticRegion {
                                        inputs: data::Storage::Static(&[
                                            data::graph::IntLocalId(1),
                                            data::graph::IntLocalId(0),
                                        ]),
                                        nodes: data::Storage::Static(&[
                                            data::graph::ArithmeticNode::Multiply(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Immediate(2)),
                                            data::graph::ArithmeticNode::Add(data::graph::ArithmeticOperand::Input(1), data::graph::ArithmeticOperand::Value(0)),
                                        ]),
                                        outputs: data::Storage::Static(&[
                                            data::graph::ArithmeticOutput {
                                                value: 1,
                                                slot: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                    shape: data::type_::ValueShapeId(0),
                                                },
                                            },
                                        ]),
                                        native: true,
                                    }),
                                    data::graph::ProfiledInstruction::IntegerRegion(data::graph::ArithmeticRegion {
                                        inputs: data::Storage::Static(&[
                                            data::graph::IntLocalId(1),
                                            data::graph::IntLocalId(0),
                                        ]),
                                        nodes: data::Storage::Static(&[
                                            data::graph::ArithmeticNode::Multiply(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Immediate(2)),
                                            data::graph::ArithmeticNode::Subtract(data::graph::ArithmeticOperand::Input(1), data::graph::ArithmeticOperand::Value(0)),
                                        ]),
                                        outputs: data::Storage::Static(&[
                                            data::graph::ArithmeticOutput {
                                                value: 1,
                                                slot: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                    shape: data::type_::ValueShapeId(0),
                                                },
                                            },
                                        ]),
                                        native: true,
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
                            parameter_count: 2,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..4,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                            subject: data::graph::BoolLocalId(0),
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(3),
                                                        },
                                                    }),
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
                                                target: data::graph::BlockId(6),
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..7,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(3),
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
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
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
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(3),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 7..10,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 10..13,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(3),
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
                                                target: data::graph::BlockId(4),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
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
                                                target: data::graph::BlockId(5),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                        params: 13..16,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 16..18,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 18..19,
                                        instructions: 3..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(3)),
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
                                                shape_id: data::type_::CustomValueShapeId(3),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(4),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(7),
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
                                                shape_id: data::type_::CustomValueShapeId(3),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(4),
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
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
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
                                                shape_id: data::type_::CustomValueShapeId(3),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(4),
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
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::IntegerRegion(data::graph::ArithmeticRegion {
                                        inputs: data::Storage::Static(&[
                                            data::graph::IntLocalId(0),
                                            data::graph::IntLocalId(2),
                                            data::graph::IntLocalId(1),
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
                                    data::graph::ProfiledInstruction::IntegerRegion(data::graph::ArithmeticRegion {
                                        inputs: data::Storage::Static(&[
                                            data::graph::IntLocalId(0),
                                            data::graph::IntLocalId(2),
                                            data::graph::IntLocalId(1),
                                        ]),
                                        nodes: data::Storage::Static(&[
                                            data::graph::ArithmeticNode::Subtract(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Input(1)),
                                            data::graph::ArithmeticNode::Subtract(data::graph::ArithmeticOperand::Value(0), data::graph::ArithmeticOperand::Input(2)),
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
                                data::function::FunctionExit::Return(data::graph::IntLocalId(3)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(3)),
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
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(3),
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
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
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
                                                    ]),
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
                                        params: 4..5,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(3),
                                                },
                                            }),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "asserted", data::source::SourceSpan::new(1647, 1657)),
                                            pattern_span: data::source::SourceSpan::new(1658, 1668),
                                        }),
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
                                                shape_id: data::type_::CustomValueShapeId(3),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(4),
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
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(3),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(4),
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
                                                    shape_id: data::type_::CustomValueShapeId(3),
                                                },
                                            }),
                                            pattern: data::graph::MatchPattern::Custom {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 0,
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..2,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::SourceStop(data::graph::SourceStop {
                                            kind: data::graph::SourceStopKind::Panic,
                                            message: Some(data::graph::StringLocalId(0)),
                                            site: data::source::PanicSite::from_static("example", "stop", data::source::SourceSpan::new(1852, 1881)),
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 2..3,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
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
                                                shape_id: data::type_::CustomValueShapeId(3),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(4),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(13),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("loop callback stop"))),
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
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(3),
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
                                        params: 2..4,
                                        instructions: 0..2,
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
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(3),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(4),
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
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(23),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "non_leaf", data::source::SourceSpan::new(2160, 2172)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
                                        }),
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
                            parameter_count: 2,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..3,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(3),
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
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
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
                                                            shape_id: data::type_::CustomValueShapeId(3),
                                                        },
                                                    }),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Int,
                                                            length: 1,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 1,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 3..6,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 6..8,
                                        instructions: 2..2,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(3),
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
                                        params: 8..10,
                                        instructions: 2..3,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 10..11,
                                        instructions: 3..3,
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
                                                shape_id: data::type_::CustomValueShapeId(3),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(4),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
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
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(3),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(4),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            },
                                            index: 0,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::IntegerRegion(data::graph::ArithmeticRegion {
                                        inputs: data::Storage::Static(&[
                                            data::graph::IntLocalId(0),
                                            data::graph::IntLocalId(2),
                                            data::graph::IntLocalId(1),
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
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::ListLengthEquals {
                                                value: data::graph::ListLocal::Custom {
                                                    local: data::graph::CustomListLocalId(0),
                                                    type_id: data::type_::CustomListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item_type: data::type_::CustomTypeId(0),
                                                    },
                                                },
                                                length: 0,
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::CustomList,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::IntFunction,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                        local: data::graph::CustomListLocalId(0),
                                                        type_id: data::type_::CustomListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::CustomTypeId(0),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::IntFunction {
                                                        local: data::graph::IntFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        },
                                                    },
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
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..7,
                                        instructions: 0..4,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                        local: data::graph::CustomListLocalId(1),
                                                        type_id: data::type_::CustomListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::CustomTypeId(0),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                    data::graph::ParamLocal::IntFunction {
                                                        local: data::graph::IntFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        },
                                                    },
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
                                                            family: data::graph::StorageFamily::CustomList,
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
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(0),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::CustomTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(5),
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
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(0),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::CustomTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(5),
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
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        shape: data::type_::ValueShapeId(6),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(3),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::ListIndex {
                                            list: data::graph::CustomListLocalId(0),
                                            index: 0,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(1),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::CustomTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                            item_type: data::type_::CustomTypeId(0),
                                        }, data::graph::TypedListInstruction::DropFirst {
                                            list: data::graph::CustomListLocalId(0),
                                            count: 1,
                                        })),
                                    }),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::FunctionCall {
                                            function: data::graph::IntFunctionLocalId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(3),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "fold_adjusted", data::source::SourceSpan::new(3100, 3122)),
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
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(5),
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
                                                    data::graph::MatchPattern::Bool(true),
                                                ]),
                                            },
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                    data::graph::MatchEdgeArgument::Binding(0),
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(5),
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
                                                target: data::graph::BlockId(5),
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
                                        params: 2..5,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::GtInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                                right: data::graph::IntegerOperand::Immediate(0),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(5),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(3),
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
                                        params: 5..8,
                                        instructions: 0..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 8..9,
                                        instructions: 2..2,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
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
                                        params: 9..10,
                                        instructions: 2..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 10..11,
                                        instructions: 2..2,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
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
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(5),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(11),
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
                                                shape_id: data::type_::CustomValueShapeId(5),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(11),
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
                                                shape_id: data::type_::CustomValueShapeId(5),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(11),
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
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::CustomField {
                                            source: data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(5),
                                                },
                                            },
                                            index: 0,
                                        }),
                                    }),
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
                                            test: data::graph::BoolTest::ListLengthEquals {
                                                value: data::graph::ListLocal::Custom {
                                                    local: data::graph::CustomListLocalId(0),
                                                    type_id: data::type_::CustomListTypeId {
                                                        list_type: data::type_::ListTypeId(1),
                                                        item_type: data::type_::CustomTypeId(1),
                                                    },
                                                },
                                                length: 0,
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::CustomList,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::IntFunction,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                        local: data::graph::CustomListLocalId(0),
                                                        type_id: data::type_::CustomListTypeId {
                                                            list_type: data::type_::ListTypeId(1),
                                                            item_type: data::type_::CustomTypeId(1),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::IntFunction {
                                                        local: data::graph::IntFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                                data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        },
                                                    },
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
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..7,
                                        instructions: 0..3,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                        local: data::graph::CustomListLocalId(1),
                                                        type_id: data::type_::CustomListTypeId {
                                                            list_type: data::type_::ListTypeId(1),
                                                            item_type: data::type_::CustomTypeId(1),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    data::graph::ParamLocal::IntFunction {
                                                        local: data::graph::IntFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                                data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        },
                                                    },
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
                                                            family: data::graph::StorageFamily::CustomList,
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
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(0),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(1),
                                                item_type: data::type_::CustomTypeId(1),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(14),
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
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        shape: data::type_::ValueShapeId(12),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(0),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(1),
                                                item_type: data::type_::CustomTypeId(1),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(14),
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
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        shape: data::type_::ValueShapeId(12),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    shape_id: data::type_::CustomValueShapeId(5),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(11),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::ListIndex {
                                            list: data::graph::CustomListLocalId(0),
                                            index: 0,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(1),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(1),
                                                    item_type: data::type_::CustomTypeId(1),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(14),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                            list_type: data::type_::ListTypeId(1),
                                            item_type: data::type_::CustomTypeId(1),
                                        }, data::graph::TypedListInstruction::DropFirst {
                                            list: data::graph::CustomListLocalId(0),
                                            count: 1,
                                        })),
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
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(5),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "fold_decisions", data::source::SourceSpan::new(3460, 3478)),
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
                                        instructions: 0..6,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(7),
                                    },
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
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            ]),
                                        }),
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
                                                id: data::graph::CustomLocalId(2),
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
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(0),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::CustomTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                            item_type: data::type_::CustomTypeId(0),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(2),
                                                },
                                            },
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(1),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            },
                                            data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(2),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(1),
                                                },
                                            },
                                        ])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BoolFunction {
                                                local: data::graph::BoolFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(8),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Bool,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                            family: data::function::FunctionReturnFamily::Bool,
                                            kind: data::graph::FunctionInstructionKind::Closure {
                                                target: data::graph::FunctionTarget::Bool(data::function::BoolFunctionId(1)),
                                                captures: data::Storage::Static(&[]),
                                            },
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::BoolFunctionId(2),
                                        site: data::source::HostCallSite::from_static("example", "boolean", data::source::SourceSpan::new(1397, 1596)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(0),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::CustomTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Bool,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
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
                                            data::graph::FamilyTransfer {
                                                family: data::graph::StorageFamily::Custom,
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
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Match(data::graph::Match {
                                            subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(3),
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
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0))),
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
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                        params: 2..4,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::GtInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(0),
                                            },
                                            true_: data::graph::Edge {
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
                                            false_: data::graph::Edge {
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..4,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..5,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 5..6,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(7),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(3),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(4),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(7),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(7),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(7),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(7),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
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
                                            test: data::graph::BoolTest::ListLengthEquals {
                                                value: data::graph::ListLocal::Custom {
                                                    local: data::graph::CustomListLocalId(0),
                                                    type_id: data::type_::CustomListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item_type: data::type_::CustomTypeId(0),
                                                    },
                                                },
                                                length: 0,
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::CustomList,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::BoolFunction,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                        local: data::graph::CustomListLocalId(0),
                                                        type_id: data::type_::CustomListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::CustomTypeId(0),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                    data::graph::ParamLocal::BoolFunction {
                                                        local: data::graph::BoolFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Bool,
                                                                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                        },
                                                    },
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
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..7,
                                        instructions: 0..3,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                        local: data::graph::CustomListLocalId(1),
                                                        type_id: data::type_::CustomListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::CustomTypeId(0),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                    data::graph::ParamLocal::BoolFunction {
                                                        local: data::graph::BoolFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Bool,
                                                                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                        },
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
                                                            family: data::graph::StorageFamily::CustomList,
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
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(0),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::CustomTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(5),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(7),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Bool,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        shape: data::type_::ValueShapeId(8),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(7),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(0),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::CustomTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(5),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(7),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Bool,
                                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        shape: data::type_::ValueShapeId(8),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(3),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::ListIndex {
                                            list: data::graph::CustomListLocalId(0),
                                            index: 0,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(1),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::CustomTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                            item_type: data::type_::CustomTypeId(0),
                                        }, data::graph::TypedListInstruction::DropFirst {
                                            list: data::graph::CustomListLocalId(0),
                                            count: 1,
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(7),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::FunctionCall {
                                            function: data::graph::BoolFunctionLocalId(0),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(3),
                                                    },
                                                }),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "any", data::source::SourceSpan::new(1310, 1327)),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
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
                custom_list_functions: data::Storage::Static(&[
                    (data::function::CustomListFunctionId {
                        index: 0,
                        type_id: data::type_::CustomListTypeId {
                            list_type: data::type_::ListTypeId(0),
                            item_type: data::type_::CustomTypeId(0),
                        },
                    }, data::function::ExecutableFunction {
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
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                        local: data::graph::CustomListLocalId(0),
                                                        type_id: data::type_::CustomListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::CustomTypeId(0),
                                                        },
                                                    }),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                        local: data::graph::CustomListLocalId(0),
                                                        type_id: data::type_::CustomListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::CustomTypeId(0),
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
                                        params: 3..6,
                                        instructions: 0..3,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                        local: data::graph::CustomListLocalId(1),
                                                        type_id: data::type_::CustomListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::CustomTypeId(0),
                                                        },
                                                    }),
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
                                                            ]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::Custom,
                                                            length: 0,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                        data::graph::FamilyTransfer {
                                                            family: data::graph::StorageFamily::CustomList,
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
                                        params: 6..7,
                                        instructions: 3..3,
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
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(0),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::CustomTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(5),
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
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(0),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::CustomTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(5),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                            local: data::graph::CustomListLocalId(0),
                                            type_id: data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::CustomTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(5),
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
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            ]),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(1),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::CustomTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                            item_type: data::type_::CustomTypeId(0),
                                        }, data::graph::TypedListInstruction::Spread {
                                            elements: data::Storage::Static(&[
                                                data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                },
                                            ]),
                                            tail: data::graph::CustomListLocalId(0),
                                        })),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::CustomListLocalId(0)),
                            ]),
                        },
                    }),
                ]),
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

            enum CustomLoopResume {
                Next(usize),
                Exit(data::compiled::custom_loop::CustomLoopProgress),
            }

            fn callback_int_12_call(
                inputs: &data::compiled::custom_loop::CallbackInputs<'_>,
                values: &mut data::compiled::custom::CustomValues,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CallbackProgress<i128> {
                let Some(_integer0) = inputs.integer(0) else {
                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Entry);
                };
                callback_int_12_entry((_integer0, inputs.custom(0),), values, budget)
            }

            fn callback_int_12_entry(
                inputs: (i128, data::compiled::custom::CustomInput,),
                values: &mut data::compiled::custom::CustomValues,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CallbackProgress<i128> {
                let (b0_i0, b0_c0,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([b0_c0]);
                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(0));
                }
                let _matched = if b0_c0.matches_constructor(data::type_::CustomConstructorId {
                    type_id: data::type_::CustomTypeId(0),
                    index: 0,
                }) {
                    'pattern: {
                        let Some(_field0) = b0_c0.integer(0) else {
                            break 'pattern Err(());
                        };
                        let m0 = _field0;
                        break 'pattern Ok(Some((m0,)));
                    }
                } else {
                    Ok(None)
                };
                match _matched {
                    Ok(Some((m0,))) => {
                        *budget -= 1;
                        let _next = (b0_i0, m0,);
                        drop(b0_c0);
                        let (b1_i0, b1_i1,) = _next;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(1));
                        }
                        *budget -= 1;
                        let b1_i2 = b1_i0 + b1_i1;
                        if b1_i2 < i128::from(i64::MIN) || b1_i2 > i128::from(i64::MAX) {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(2));
                        }
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(2));
                        }
                        *budget -= 1;
                        data::compiled::custom_loop::CallbackProgress::Complete(b1_i2)
                    },
                    Ok(None) => {
                        *budget -= 1;
                        let (b2_i0, b2_c0,) = (b0_i0, b0_c0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([b2_c0]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(3));
                        }
                        let _matched = if b2_c0.matches_constructor(data::type_::CustomConstructorId {
                            type_id: data::type_::CustomTypeId(0),
                            index: 1,
                        }) {
                            'pattern: {
                                let Some(_field0) = b2_c0.integer(0) else {
                                    break 'pattern Err(());
                                };
                                let m0 = _field0;
                                break 'pattern Ok(Some((m0,)));
                            }
                        } else {
                            Ok(None)
                        };
                        match _matched {
                            Ok(Some((m0,))) => {
                                *budget -= 1;
                                let _next = (b2_i0, m0,);
                                drop(b2_c0);
                                let (b3_i0, b3_i1,) = _next;
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.customs.clear();
                                    values.customs.extend([]);
                                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(4));
                                }
                                *budget -= 1;
                                let b3_i2 = b3_i0 - b3_i1;
                                if b3_i2 < i128::from(i64::MIN) || b3_i2 > i128::from(i64::MAX) {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.customs.clear();
                                    values.customs.extend([]);
                                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(5));
                                }
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.customs.clear();
                                    values.customs.extend([]);
                                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(5));
                                }
                                *budget -= 1;
                                data::compiled::custom_loop::CallbackProgress::Complete(b3_i2)
                            },
                            Ok(None) => {
                                *budget -= 1;
                                let _next = (b2_i0,);
                                drop(b2_c0);
                                let (b4_i0,) = _next;
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b4_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.customs.clear();
                                    values.customs.extend([]);
                                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(6));
                                }
                                *budget -= 1;
                                data::compiled::custom_loop::CallbackProgress::Complete(b4_i0)
                            },
                            Err(()) => {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.customs.clear();
                                values.customs.extend([b2_c0]);
                                data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(3))
                            }
                        }
                    },
                    Err(()) => {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([b0_c0]);
                        data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(0))
                    }
                }
            }

            fn callback_int_14_call(
                inputs: &data::compiled::custom_loop::CallbackInputs<'_>,
                values: &mut data::compiled::custom::CustomValues,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CallbackProgress<i128> {
                let Some(_integer0) = inputs.integer(0) else {
                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Entry);
                };
                callback_int_14_entry((_integer0, inputs.custom(0),), values, budget)
            }

            fn callback_int_14_entry(
                inputs: (i128, data::compiled::custom::CustomInput,),
                values: &mut data::compiled::custom::CustomValues,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CallbackProgress<i128> {
                let (b0_i0, b0_c0,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([b0_c0]);
                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(0));
                }
                let _matched = if b0_c0.matches_constructor(data::type_::CustomConstructorId {
                    type_id: data::type_::CustomTypeId(0),
                    index: 0,
                }) {
                    'pattern: {
                        let Some(_field0) = b0_c0.integer(0) else {
                            break 'pattern Err(());
                        };
                        let m0 = _field0;
                        break 'pattern Ok(Some((m0,)));
                    }
                } else {
                    Ok(None)
                };
                match _matched {
                    Ok(Some((m0,))) => {
                        *budget -= 1;
                        let _next = (b0_i0, m0,);
                        drop(b0_c0);
                        let (b1_i0, b1_i1,) = _next;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(1));
                        }
                        *budget -= 1;
                        let _r0_n0 = b1_i1 * 2_i128;
                        let _r0_n1 = b1_i0 + _r0_n0;
                        let b1_i2 = _r0_n1;
                        if b1_i2 < i128::from(i64::MIN) || b1_i2 > i128::from(i64::MAX) {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(2));
                        }
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(2));
                        }
                        *budget -= 1;
                        data::compiled::custom_loop::CallbackProgress::Complete(b1_i2)
                    },
                    Ok(None) => {
                        *budget -= 1;
                        let (b2_i0, b2_c0,) = (b0_i0, b0_c0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([b2_c0]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(3));
                        }
                        let _matched = if b2_c0.matches_constructor(data::type_::CustomConstructorId {
                            type_id: data::type_::CustomTypeId(0),
                            index: 1,
                        }) {
                            'pattern: {
                                let Some(_field0) = b2_c0.integer(0) else {
                                    break 'pattern Err(());
                                };
                                let m0 = _field0;
                                break 'pattern Ok(Some((m0,)));
                            }
                        } else {
                            Ok(None)
                        };
                        match _matched {
                            Ok(Some((m0,))) => {
                                *budget -= 1;
                                let _next = (b2_i0, m0,);
                                drop(b2_c0);
                                let (b3_i0, b3_i1,) = _next;
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.customs.clear();
                                    values.customs.extend([]);
                                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(4));
                                }
                                *budget -= 1;
                                let _r0_n0 = b3_i1 * 2_i128;
                                let _r0_n1 = b3_i0 - _r0_n0;
                                let b3_i2 = _r0_n1;
                                if b3_i2 < i128::from(i64::MIN) || b3_i2 > i128::from(i64::MAX) {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.customs.clear();
                                    values.customs.extend([]);
                                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(5));
                                }
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.customs.clear();
                                    values.customs.extend([]);
                                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(5));
                                }
                                *budget -= 1;
                                data::compiled::custom_loop::CallbackProgress::Complete(b3_i2)
                            },
                            Ok(None) => {
                                *budget -= 1;
                                let _next = (b2_i0,);
                                drop(b2_c0);
                                let (b4_i0,) = _next;
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b4_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.customs.clear();
                                    values.customs.extend([]);
                                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(6));
                                }
                                *budget -= 1;
                                data::compiled::custom_loop::CallbackProgress::Complete(b4_i0)
                            },
                            Err(()) => {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.customs.clear();
                                values.customs.extend([b2_c0]);
                                data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(3))
                            }
                        }
                    },
                    Err(()) => {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([b0_c0]);
                        data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(0))
                    }
                }
            }

            fn callback_int_15_call(
                inputs: &data::compiled::custom_loop::CallbackInputs<'_>,
                values: &mut data::compiled::custom::CustomValues,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CallbackProgress<i128> {
                let Some(_integer0) = inputs.integer(0) else {
                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Entry);
                };
                let Some(_integer1) = inputs.integer(1) else {
                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Entry);
                };
                callback_int_15_entry((_integer0, _integer1, inputs.boolean(0), inputs.custom(0),), values, budget)
            }

            fn callback_int_15_entry(
                inputs: (i128, i128, bool, data::compiled::custom::CustomInput,),
                values: &mut data::compiled::custom::CustomValues,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CallbackProgress<i128> {
                let (b0_i0, b0_i1, b0_v0, b0_c0,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);
                    values.customs.clear();
                    values.customs.extend([b0_c0]);
                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(0));
                }
                *budget -= 1;
                if b0_v0 {
                    let (b1_i0, b1_i1, b1_c0,) = (b0_i0, b0_i1, b0_c0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([b1_c0]);
                        return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(1));
                    }
                    let _matched = if b1_c0.matches_constructor(data::type_::CustomConstructorId {
                        type_id: data::type_::CustomTypeId(0),
                        index: 0,
                    }) {
                        'pattern: {
                            let Some(_field0) = b1_c0.integer(0) else {
                                break 'pattern Err(());
                            };
                            let m0 = _field0;
                            break 'pattern Ok(Some((m0,)));
                        }
                    } else {
                        Ok(None)
                    };
                    match _matched {
                        Ok(Some((m0,))) => {
                            *budget -= 1;
                            let _next = (b1_i0, b1_i1, m0,);
                            drop(b1_c0);
                            let (b2_i0, b2_i1, b2_i2,) = _next;
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.customs.clear();
                                values.customs.extend([]);
                                return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(2));
                            }
                            *budget -= 1;
                            let _r0_n0 = b2_i0 + b2_i2;
                            let _r0_n1 = _r0_n0 + b2_i1;
                            let b2_i3 = _r0_n1;
                            if b2_i3 < i128::from(i64::MIN) || b2_i3 > i128::from(i64::MAX) {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2, b2_i3]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.customs.clear();
                                values.customs.extend([]);
                                return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(3));
                            }
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2, b2_i3]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.customs.clear();
                                values.customs.extend([]);
                                return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(3));
                            }
                            *budget -= 1;
                            data::compiled::custom_loop::CallbackProgress::Complete(b2_i3)
                        },
                        Ok(None) => {
                            *budget -= 1;
                            let (b3_i0, b3_i1, b3_c0,) = (b1_i0, b1_i1, b1_c0,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.customs.clear();
                                values.customs.extend([b3_c0]);
                                return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(4));
                            }
                            let _matched = if b3_c0.matches_constructor(data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(0),
                                index: 1,
                            }) {
                                'pattern: {
                                    let Some(_field0) = b3_c0.integer(0) else {
                                        break 'pattern Err(());
                                    };
                                    let m0 = _field0;
                                    break 'pattern Ok(Some((m0,)));
                                }
                            } else {
                                Ok(None)
                            };
                            match _matched {
                                Ok(Some((m0,))) => {
                                    *budget -= 1;
                                    let _next = (b3_i0, b3_i1, m0,);
                                    drop(b3_c0);
                                    let (b4_i0, b4_i1, b4_i2,) = _next;
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.customs.clear();
                                        values.customs.extend([]);
                                        return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(5));
                                    }
                                    *budget -= 1;
                                    let _r0_n0 = b4_i0 - b4_i2;
                                    let _r0_n1 = _r0_n0 - b4_i1;
                                    let b4_i3 = _r0_n1;
                                    if b4_i3 < i128::from(i64::MIN) || b4_i3 > i128::from(i64::MAX) {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.customs.clear();
                                        values.customs.extend([]);
                                        return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(6));
                                    }
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.customs.clear();
                                        values.customs.extend([]);
                                        return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(6));
                                    }
                                    *budget -= 1;
                                    data::compiled::custom_loop::CallbackProgress::Complete(b4_i3)
                                },
                                Ok(None) => {
                                    *budget -= 1;
                                    let _next = (b3_i0, b3_i1,);
                                    drop(b3_c0);
                                    let (b5_i0, b5_i1,) = _next;
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.customs.clear();
                                        values.customs.extend([]);
                                        return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(7));
                                    }
                                    *budget -= 1;
                                    let b5_i2 = b5_i0 + b5_i1;
                                    if b5_i2 < i128::from(i64::MIN) || b5_i2 > i128::from(i64::MAX) {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b5_i0, b5_i1, b5_i2]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.customs.clear();
                                        values.customs.extend([]);
                                        return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(8));
                                    }
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b5_i0, b5_i1, b5_i2]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.customs.clear();
                                        values.customs.extend([]);
                                        return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(8));
                                    }
                                    *budget -= 1;
                                    data::compiled::custom_loop::CallbackProgress::Complete(b5_i2)
                                },
                                Err(()) => {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.customs.clear();
                                    values.customs.extend([b3_c0]);
                                    data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(4))
                                }
                            }
                        },
                        Err(()) => {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([b1_c0]);
                            data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(1))
                        }
                    }
                } else {
                    let _next = (b0_i0,);
                    drop(b0_c0);
                    let (b6_i0,) = _next;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b6_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([]);
                        return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(9));
                    }
                    *budget -= 1;
                    data::compiled::custom_loop::CallbackProgress::Complete(b6_i0)
                }
            }

            fn callback_int_16_call(
                inputs: &data::compiled::custom_loop::CallbackInputs<'_>,
                values: &mut data::compiled::custom::CustomValues,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CallbackProgress<i128> {
                let Some(_integer0) = inputs.integer(0) else {
                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Entry);
                };
                callback_int_16_entry((_integer0, inputs.custom(0),), values, budget)
            }

            fn callback_int_16_entry(
                inputs: (i128, data::compiled::custom::CustomInput,),
                values: &mut data::compiled::custom::CustomValues,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CallbackProgress<i128> {
                let (b0_i0, b0_c0,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([b0_c0]);
                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(0));
                }
                let _matched = if b0_c0.matches_constructor(data::type_::CustomConstructorId {
                    type_id: data::type_::CustomTypeId(0),
                    index: 0,
                }) {
                    'pattern: {
                        let Some(_field0) = b0_c0.integer(0) else {
                            break 'pattern Err(());
                        };
                        let m0 = _field0;
                        break 'pattern Ok(Some((m0,)));
                    }
                } else {
                    Ok(None)
                };
                match _matched {
                    Ok(Some((m0,))) => {
                        *budget -= 1;
                        let _next = (b0_i0, m0,);
                        drop(b0_c0);
                        let (b1_i0, b1_i1,) = _next;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(1));
                        }
                        *budget -= 1;
                        let b1_i2 = b1_i0 + b1_i1;
                        if b1_i2 < i128::from(i64::MIN) || b1_i2 > i128::from(i64::MAX) {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(2));
                        }
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(2));
                        }
                        *budget -= 1;
                        data::compiled::custom_loop::CallbackProgress::Complete(b1_i2)
                    },
                    Ok(None) => {
                        *budget -= 1;
                        let (b2_c0,) = (b0_c0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([b2_c0]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(3));
                        }

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([b2_c0]);
                        data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(3))
                    },
                    Err(()) => {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([b0_c0]);
                        data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(0))
                    }
                }
            }

            fn callback_int_17_call(
                inputs: &data::compiled::custom_loop::CallbackInputs<'_>,
                values: &mut data::compiled::custom::CustomValues,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CallbackProgress<i128> {
                let Some(_integer0) = inputs.integer(0) else {
                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Entry);
                };
                callback_int_17_entry((_integer0, inputs.custom(0),), values, budget)
            }

            fn callback_int_17_entry(
                inputs: (i128, data::compiled::custom::CustomInput,),
                values: &mut data::compiled::custom::CustomValues,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CallbackProgress<i128> {
                let (b0_i0, b0_c0,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([b0_c0]);
                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(0));
                }
                let _matched = if b0_c0.matches_constructor(data::type_::CustomConstructorId {
                    type_id: data::type_::CustomTypeId(0),
                    index: 0,
                }) {
                    'pattern: {
                        let Some(_field0) = b0_c0.integer(0) else {
                            break 'pattern Err(());
                        };
                        break 'pattern Ok(Some(()));
                    }
                } else {
                    Ok(None)
                };
                match _matched {
                    Ok(Some(())) => {
                        *budget -= 1;
                        let _next = ();
                        drop(b0_c0);
                        let () = _next;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(1));
                        }

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([]);
                        data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(1))
                    },
                    Ok(None) => {
                        *budget -= 1;
                        let _next = (b0_i0,);
                        drop(b0_c0);
                        let (b2_i0,) = _next;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(2));
                        }
                        *budget -= 1;
                        data::compiled::custom_loop::CallbackProgress::Complete(b2_i0)
                    },
                    Err(()) => {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([b0_c0]);
                        data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(0))
                    }
                }
            }

            fn callback_int_19_call(
                inputs: &data::compiled::custom_loop::CallbackInputs<'_>,
                values: &mut data::compiled::custom::CustomValues,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CallbackProgress<i128> {
                let Some(_integer0) = inputs.integer(0) else {
                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Entry);
                };
                callback_int_19_entry((_integer0, inputs.custom(0), inputs.custom(1),), values, budget)
            }

            fn callback_int_19_entry(
                inputs: (i128, data::compiled::custom::CustomInput, data::compiled::custom::CustomInput,),
                values: &mut data::compiled::custom::CustomValues,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CallbackProgress<i128> {
                let (b0_i0, b0_c0, b0_c1,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([b0_c0, b0_c1]);
                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(0));
                }
                let b0_i1 = match b0_c1.integer(0) {
                    Some(value) => value,
                    None => {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([b0_c0, b0_c1]);
                        return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(0));
                    }
                };
                *budget -= 1;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([b0_c0, b0_c1]);
                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(1));
                }
                let _matched = if b0_c0.matches_constructor(data::type_::CustomConstructorId {
                    type_id: data::type_::CustomTypeId(0),
                    index: 0,
                }) {
                    'pattern: {
                        let Some(_field0) = b0_c0.integer(0) else {
                            break 'pattern Err(());
                        };
                        let m0 = _field0;
                        break 'pattern Ok(Some((m0,)));
                    }
                } else {
                    Ok(None)
                };
                match _matched {
                    Ok(Some((m0,))) => {
                        *budget -= 1;
                        let _next = (b0_i0, b0_i1, m0,);
                        drop(b0_c0);
                        drop(b0_c1);
                        let (b1_i0, b1_i1, b1_i2,) = _next;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(2));
                        }
                        *budget -= 1;
                        let _r0_n0 = b1_i0 + b1_i2;
                        let _r0_n1 = _r0_n0 + b1_i1;
                        let b1_i3 = _r0_n1;
                        if b1_i3 < i128::from(i64::MIN) || b1_i3 > i128::from(i64::MAX) {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2, b1_i3]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(3));
                        }
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2, b1_i3]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(3));
                        }
                        *budget -= 1;
                        data::compiled::custom_loop::CallbackProgress::Complete(b1_i3)
                    },
                    Ok(None) => {
                        *budget -= 1;
                        let _next = (b0_i0, b0_c0,);
                        drop(b0_c1);
                        let (b2_i0, b2_c0,) = _next;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([b2_c0]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(4));
                        }
                        let _matched = if b2_c0.matches_constructor(data::type_::CustomConstructorId {
                            type_id: data::type_::CustomTypeId(0),
                            index: 1,
                        }) {
                            'pattern: {
                                let Some(_field0) = b2_c0.integer(0) else {
                                    break 'pattern Err(());
                                };
                                let m0 = _field0;
                                break 'pattern Ok(Some((m0,)));
                            }
                        } else {
                            Ok(None)
                        };
                        match _matched {
                            Ok(Some((m0,))) => {
                                *budget -= 1;
                                let _next = (b2_i0, m0,);
                                drop(b2_c0);
                                let (b3_i0, b3_i1,) = _next;
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.customs.clear();
                                    values.customs.extend([]);
                                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(5));
                                }
                                *budget -= 1;
                                let b3_i2 = b3_i0 - b3_i1;
                                if b3_i2 < i128::from(i64::MIN) || b3_i2 > i128::from(i64::MAX) {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.customs.clear();
                                    values.customs.extend([]);
                                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(6));
                                }
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.customs.clear();
                                    values.customs.extend([]);
                                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(6));
                                }
                                *budget -= 1;
                                data::compiled::custom_loop::CallbackProgress::Complete(b3_i2)
                            },
                            Ok(None) => {
                                *budget -= 1;
                                let _next = (b2_i0,);
                                drop(b2_c0);
                                let (b4_i0,) = _next;
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b4_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.customs.clear();
                                    values.customs.extend([]);
                                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(7));
                                }
                                *budget -= 1;
                                data::compiled::custom_loop::CallbackProgress::Complete(b4_i0)
                            },
                            Err(()) => {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.customs.clear();
                                values.customs.extend([b2_c0]);
                                data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(4))
                            }
                        }
                    },
                    Err(()) => {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([b0_c0, b0_c1]);
                        data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(1))
                    }
                }
            }

            fn callback_int_21_call(
                inputs: &data::compiled::custom_loop::CallbackInputs<'_>,
                values: &mut data::compiled::custom::CustomValues,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CallbackProgress<i128> {
                let Some(_integer0) = inputs.integer(0) else {
                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Entry);
                };
                callback_int_21_entry((_integer0, inputs.custom(0),), values, budget)
            }

            fn callback_int_21_entry(
                inputs: (i128, data::compiled::custom::CustomInput,),
                values: &mut data::compiled::custom::CustomValues,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CallbackProgress<i128> {
                let (b0_i0, b0_c0,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([b0_c0]);
                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(0));
                }
                let _matched = if b0_c0.matches_constructor(data::type_::CustomConstructorId {
                    type_id: data::type_::CustomTypeId(1),
                    index: 0,
                }) {
                    'pattern: {
                        let Some(_field0) = b0_c0.integer(0) else {
                            break 'pattern Err(());
                        };
                        let m0 = _field0;
                        let Some(_field1) = b0_c0.boolean(1) else {
                            break 'pattern Err(());
                        };
                        if !_field1 {
                            break 'pattern Ok(None);
                        }
                        break 'pattern Ok(Some((m0,)));
                    }
                } else {
                    Ok(None)
                };
                let (b4_i0,) = match _matched {
                    Ok(Some((m0,))) => {
                        *budget -= 1;
                        let (b1_i0, b1_i1, b1_c0,) = (b0_i0, m0, b0_c0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([b1_c0]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(1));
                        }
                        *budget -= 1;
                        if b1_i1 > 0_i128 {
                            let (b2_i0, b2_i1, b2_c0,) = (b1_i0, b1_i1, b1_c0,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.customs.clear();
                                values.customs.extend([b2_c0]);
                                return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(2));
                            }
                            let b2_i2 = match b2_c0.integer(0) {
                                Some(value) => value,
                                None => {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.customs.clear();
                                    values.customs.extend([b2_c0]);
                                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(2));
                                }
                            };
                            *budget -= 1;
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.customs.clear();
                                values.customs.extend([b2_c0]);
                                return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(3));
                            }
                            *budget -= 1;
                            let _r1_n0 = b2_i0 + b2_i1;
                            let _r1_n1 = _r1_n0 + b2_i2;
                            let b2_i3 = _r1_n1;
                            if b2_i3 < i128::from(i64::MIN) || b2_i3 > i128::from(i64::MAX) {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2, b2_i3]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.customs.clear();
                                values.customs.extend([b2_c0]);
                                return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(4));
                            }
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2, b2_i3]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.customs.clear();
                                values.customs.extend([b2_c0]);
                                return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(4));
                            }
                            *budget -= 1;
                            return data::compiled::custom_loop::CallbackProgress::Complete(b2_i3);
                        } else {
                            let _next = (b1_i0,);
                            drop(b1_c0);
                            let (b3_i0,) = _next;
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.customs.clear();
                                values.customs.extend([]);
                                return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(5));
                            }
                            *budget -= 1;
                            let (b4_i0,) = {
                                (b3_i0,)
                            };
                            (b4_i0,)
                        }
                    },
                    Ok(None) => {
                        *budget -= 1;
                        let _next = (b0_i0,);
                        drop(b0_c0);
                        let (b5_i0,) = _next;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b5_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(7));
                        }
                        *budget -= 1;
                        let (b4_i0,) = {
                            (b5_i0,)
                        };
                        (b4_i0,)
                    },
                    Err(()) => {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([b0_c0]);
                        return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(0));
                    }
                };
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([]);
                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(6));
                }
                *budget -= 1;
                data::compiled::custom_loop::CallbackProgress::Complete(b4_i0)
            }

            fn callback_bool_1_call(
                inputs: &data::compiled::custom_loop::CallbackInputs<'_>,
                values: &mut data::compiled::custom::CustomValues,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CallbackProgress<bool> {
                callback_bool_1_entry((inputs.boolean(0), inputs.custom(0),), values, budget)
            }

            fn callback_bool_1_entry(
                inputs: (bool, data::compiled::custom::CustomInput,),
                values: &mut data::compiled::custom::CustomValues,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CallbackProgress<bool> {
                let (b0_v0, b0_c0,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);
                    values.customs.clear();
                    values.customs.extend([b0_c0]);
                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(0));
                }
                let _matched = if b0_c0.matches_constructor(data::type_::CustomConstructorId {
                    type_id: data::type_::CustomTypeId(0),
                    index: 0,
                }) {
                    'pattern: {
                        let Some(_field0) = b0_c0.integer(0) else {
                            break 'pattern Err(());
                        };
                        let m0 = _field0;
                        break 'pattern Ok(Some((m0,)));
                    }
                } else {
                    Ok(None)
                };
                match _matched {
                    Ok(Some((m0,))) => {
                        *budget -= 1;
                        let _next = (m0, b0_v0,);
                        drop(b0_c0);
                        let (b1_i0, b1_v0,) = _next;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b1_v0]);
                            values.customs.clear();
                            values.customs.extend([]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(1));
                        }
                        *budget -= 1;
                        if b1_i0 > 0_i128 {
                            let () = ();
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.customs.clear();
                                values.customs.extend([]);
                                return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(2));
                            }
                            *budget -= 1;
                            let b2_v0 = true;
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[b2_v0]);
                                values.customs.clear();
                                values.customs.extend([]);
                                return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(3));
                            }
                            *budget -= 1;
                            data::compiled::custom_loop::CallbackProgress::Complete(b2_v0)
                        } else {
                            let (b3_v0,) = (b1_v0,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[b3_v0]);
                                values.customs.clear();
                                values.customs.extend([]);
                                return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(4));
                            }
                            *budget -= 1;
                            data::compiled::custom_loop::CallbackProgress::Complete(b3_v0)
                        }
                    },
                    Ok(None) => {
                        *budget -= 1;
                        let _next = (b0_v0,);
                        drop(b0_c0);
                        let (b4_v0,) = _next;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b4_v0]);
                            values.customs.clear();
                            values.customs.extend([]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(5));
                        }
                        *budget -= 1;
                        data::compiled::custom_loop::CallbackProgress::Complete(b4_v0)
                    },
                    Err(()) => {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b0_v0]);
                        values.customs.clear();
                        values.customs.extend([b0_c0]);
                        data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(0))
                    }
                }
            }
            use data::compiled::calls::{BoolCallable, CallArguments, CallExecution, CallInputs, CallInteger, CallNativeFailure, CallNativeInput, CallNativeOps, CallNativeReturn, CallOps, CallOutput, CallProgress, CallStorage, CallValues, IntCallable};
            enum FunctionState {
                Int1Point0 { int0: i128, bool0: bool },
                Int1Point1 { int0: i128 },
                Int1Point2 { int0: i128, int_function0: IntCallable },
                Int1Point3 { int_function0: IntCallable, int0: i128 },
                Int1Point4 { int_function0: IntCallable, int0: i128, int1: i128 },
                Int1Point5 { int0: i128 },
                Int1Point6 { int0: i128, int_function0: IntCallable },
                Int2Point0 { int0: i128, int1: i128, bool0: bool },
                Int2Point1 { int0: i128, int1: i128, bool0: bool, int2: i128 },
                Int3Point0 { int0: i128 },
                Int3Point1 { int0: i128, int1: i128 },
                Int9Point0 { int0: i128 },
                Int9Point1 { int0: i128, int1: i128 },
                Int11Point0 {  },
                Int11Point1 { int0: i128 },
                Int11Point2 { int0: i128, int1: i128 },
                Int11Point3 { int0: i128, int1: i128, int2: i128 },
                Int11Point4 { int0: i128, int1: i128, int2: i128, int3: i128 },
                Int11Point5 { int0: i128, int1: i128, int2: i128, int3: i128, bool0: bool },
                Int11Point6 { int0: i128, int1: i128, int2: i128, int3: i128, bool0: bool, int4: i128 },
                Int11Point7 { int0: i128, int1: i128, int2: i128, int3: i128, bool0: bool, int4: i128, int5: i128 },
                Int11Point8 { int0: i128, int1: i128, int2: i128, int3: i128, bool0: bool, int4: i128, int5: i128, int6: i128 },
                Int11Point9 { int0: i128, int1: i128, int2: i128, int3: i128, bool0: bool, int4: i128, int5: i128, int6: i128, int7: i128 },
                Int11Point10 { int0: i128, int1: i128, int2: i128, int3: i128, bool0: bool, int4: i128, int5: i128, int6: i128, int7: i128, bool1: bool },
                Int11Point11 { int0: i128, int1: i128, int2: i128, int3: i128, bool0: bool, int4: i128, int5: i128, int6: i128, int7: i128, bool1: bool, int8: i128 },
                Int11Point12 { int0: i128, int1: i128, int2: i128, int3: i128, bool0: bool, int4: i128, int5: i128, int6: i128, int7: i128, bool1: bool, int8: i128, int9: i128 },
                Int11Point13 { int0: i128, int1: i128, int2: i128, int3: i128, bool0: bool, int4: i128, int5: i128, int6: i128, int7: i128, bool1: bool, int8: i128, int9: i128, int10: i128 },
                Int11Point14 { int0: i128, int1: i128, int2: i128, int3: i128, bool0: bool, int4: i128, int5: i128, int6: i128, int7: i128, bool1: bool, int8: i128, int9: i128, int10: i128, int11: i128 },
                Int11Point15 { int0: i128, int1: i128, int2: i128, int3: i128, bool0: bool, int4: i128, int5: i128, int6: i128, int7: i128, bool1: bool, int8: i128, int9: i128, int10: i128, int11: i128, int12: i128 },
                Int11Point16 { int0: i128, int1: i128, int2: i128, int3: i128, bool0: bool, int4: i128, int5: i128, int6: i128, int7: i128, bool1: bool, int8: i128, int9: i128, int10: i128, int11: i128, int12: i128, int13: i128 },
                Int11Point17 { int0: i128, int1: i128, int2: i128, int3: i128, bool0: bool, int4: i128, int5: i128, int6: i128, int7: i128, bool1: bool, int8: i128, int9: i128, int10: i128, int11: i128, int12: i128, int13: i128, int14: i128 },
                Int23Point0 { int0: i128 },
                Int23Point1 { int0: i128, int1: i128 },
                Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: CallValues },
            }
            enum IntReturn {
                Int11Call2 { int0: i128, int1: i128 },
                Int11Call5 { int0: i128, int1: i128, int2: i128, int3: i128, bool0: bool },
                Int11Call10 { int0: i128, int1: i128, int2: i128, int3: i128, bool0: bool, int4: i128, int5: i128, int6: i128, int7: i128, bool1: bool },
                Int11Call12 { int0: i128, int1: i128, int2: i128, int3: i128, bool0: bool, int4: i128, int5: i128, int6: i128, int7: i128, bool1: bool, int8: i128, int9: i128 },
                Int11Call15 { int0: i128, int1: i128, int2: i128, int3: i128, bool0: bool, int4: i128, int5: i128, int6: i128, int7: i128, bool1: bool, int8: i128, int9: i128, int10: i128, int11: i128, int12: i128 },
            }
            impl IntReturn {
                fn site(&self) -> data::source::HostCallSite {
                    match *self {
                        Self::Int11Call2 { .. } => data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3876, 3885)),
                        Self::Int11Call5 { .. } => data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3888, 3903)),
                        Self::Int11Call10 { .. } => data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3906, 3926)),
                        Self::Int11Call12 { .. } => data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3929, 3936)),
                        Self::Int11Call15 { .. } => data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3939, 3953)),
                    }
                }
                fn small(self, result: i128) -> FunctionState {
                    match self {
                        Self::Int11Call2 { int0, int1 } => {
                            let int2 = result;
                            FunctionState::Int11Point3 { int0, int1, int2 }
                        },
                        Self::Int11Call5 { int0, int1, int2, int3, bool0 } => {
                            let int4 = result;
                            FunctionState::Int11Point6 { int0, int1, int2, int3, bool0, int4 }
                        },
                        Self::Int11Call10 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1 } => {
                            let int8 = result;
                            FunctionState::Int11Point11 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8 }
                        },
                        Self::Int11Call12 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9 } => {
                            let int10 = result;
                            FunctionState::Int11Point13 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9, int10 }
                        },
                        Self::Int11Call15 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9, int10, int11, int12 } => {
                            let int13 = result;
                            FunctionState::Int11Point16 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9, int10, int11, int12, int13 }
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
                            }, values: CallValues { ints: vec![int0.into(), int1.into(), int2], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }
                        },
                        Self::Int11Call5 { int0, int1, int2, int3, bool0 } => {
                            let int4 = result;
                            FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }
                        },
                        Self::Int11Call10 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1 } => {
                            let int8 = result;
                            FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 11,
                                ints: 9,
                                bools: 2,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8], bools: vec![bool0, bool1], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }
                        },
                        Self::Int11Call12 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9 } => {
                            let int10 = result;
                            FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 13,
                                ints: 11,
                                bools: 2,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10], bools: vec![bool0, bool1], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }
                        },
                        Self::Int11Call15 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9, int10, int11, int12 } => {
                            let int13 = result;
                            FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 16,
                                ints: 14,
                                bools: 2,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13], bools: vec![bool0, bool1], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }
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
                IntScalarBridge { function: data::function::IntFunctionId, site: data::source::HostCallSite, input: CallNativeInput, caller: IntReturn },
            }
            struct FunctionExecution {
                active: Option<FunctionActive>,
                integer_returns: Vec<IntReturn>,
                boolean_returns: Vec<BoolReturn>,
                integer_function_returns: Vec<IntFunctionReturn>,
                boolean_function_returns: Vec<BoolFunctionReturn>,
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
                        data::compiled::CallTarget::Int(data::function::IntFunctionId(1)) => calls_int_1_state(point, values),
                        data::compiled::CallTarget::Int(data::function::IntFunctionId(2)) => calls_int_2_state(point, values),
                        data::compiled::CallTarget::Int(data::function::IntFunctionId(3)) => calls_int_3_state(point, values),
                        data::compiled::CallTarget::Int(data::function::IntFunctionId(9)) => calls_int_9_state(point, values),
                        data::compiled::CallTarget::Int(data::function::IntFunctionId(11)) => calls_int_11_state(point, values),
                        data::compiled::CallTarget::Int(data::function::IntFunctionId(23)) => calls_int_23_state(point, values),
                        _ => None,
                    };
                    let Some(active) = active else { return false; };
                    self.active = Some(FunctionActive::Running(active));
                    true
                }
                fn retained_bytes(&self) -> usize {
                    std::mem::size_of::<Self>() + self.integer_returns.capacity() * std::mem::size_of::<IntReturn>() + self.boolean_returns.capacity() * std::mem::size_of::<BoolReturn>() + self.integer_function_returns.capacity() * std::mem::size_of::<IntFunctionReturn>() + self.boolean_function_returns.capacity() * std::mem::size_of::<BoolFunctionReturn>()
                }
                fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                    let Some(active) = self.active.take() else { return CallProgress::Yield(self); };
                    let mut active = match active {
                        FunctionActive::Running(active) => active,
                        FunctionActive::IntCall { function, site, input, caller } => {
                            return CallProgress::Int {
                                function, site, arguments: input.arguments(),
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
                        match function_step(active, ops, budget) {
                            FunctionStep::Next(next) => active = next,
                            FunctionStep::Yield(active) => {
                                self.active = Some(FunctionActive::Running(active));
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
                                    self.active = Some(FunctionActive::Running(caller.resume(value)));
                                    self
                                }),
                            },
                            FunctionStep::IntScalarBridge { function, site, input, caller } => {
                                return CallProgress::Int {
                                    function, site, arguments: input.arguments(),
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
                                        return CallProgress::Interpreted { point, values };
                                    },
                                    data::compiled::CallTarget::Bool(function) => {
                                        if let Some(caller) = self.boolean_returns.pop() {
                                            let site = caller.site();
                                            return CallProgress::InterpretedBool {
                                                function, site, point, values,
                                                resume: Box::new(move |value| {
                                                    self.active = Some(FunctionActive::Running(caller.resume(value)));
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
                                                    self.active = Some(FunctionActive::Running(caller.resume(value)));
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
                                                    self.active = Some(FunctionActive::Running(caller.resume(value)));
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
                                return Ok(Some(CallProgress::Int {
                                    function, site, arguments: input.arguments(),
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
                        match function_step(active, ops, budget) {
                            FunctionStep::Next(next) => active = next,
                            FunctionStep::Yield(active) => {
                                self.active = Some(FunctionActive::Running(active));
                                return Ok(Some(CallProgress::Yield(self)));
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
                                    return Ok(Some(CallProgress::Complete { exit, output: CallOutput::Int(value.into()), execution: self }));
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
                                        return Ok(Some(CallProgress::Int {
                                            function, site, arguments: input.arguments(),
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
                                        return Ok(Some(CallProgress::Interpreted { point, values }));
                                    },
                                    data::compiled::CallTarget::Bool(function) => {
                                        if let Some(caller) = self.boolean_returns.pop() {
                                            let site = caller.site();
                                            return Ok(Some(CallProgress::InterpretedBool {
                                                function, site, point, values,
                                                resume: Box::new(move |value| {
                                                    self.active = Some(FunctionActive::Running(caller.resume(value)));
                                                    self
                                                }),
                                            }));
                                        }
                                        return Ok(Some(CallProgress::Interpreted { point, values }));
                                    },
                                    data::compiled::CallTarget::IntFunction(function) => {
                                        if let Some(caller) = self.integer_function_returns.pop() {
                                            let site = caller.site();
                                            return Ok(Some(CallProgress::InterpretedIntFunction {
                                                function, site, point, values,
                                                resume: Box::new(move |value| {
                                                    self.active = Some(FunctionActive::Running(caller.resume(value)));
                                                    self
                                                }),
                                            }));
                                        }
                                        return Ok(Some(CallProgress::Interpreted { point, values }));
                                    },
                                    data::compiled::CallTarget::BoolFunction(function) => {
                                        if let Some(caller) = self.boolean_function_returns.pop() {
                                            let site = caller.site();
                                            return Ok(Some(CallProgress::InterpretedBoolFunction {
                                                function, site, point, values,
                                                resume: Box::new(move |value| {
                                                    self.active = Some(FunctionActive::Running(caller.resume(value)));
                                                    self
                                                }),
                                            }));
                                        }
                                        return Ok(Some(CallProgress::Interpreted { point, values }));
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
                    FunctionState::Int1Point0 { int0, bool0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point0 { int0, bool0 }); }
                        *budget -= 1;
                        if bool0 { FunctionStep::Next(FunctionState::Int1Point1 { int0 }) } else { FunctionStep::Next(FunctionState::Int1Point5 { int0 }) }
                    },
                    FunctionState::Int1Point1 { int0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point1 { int0 }); }
                        *budget -= 1;
                        let int_function0 = ops.int_reference(data::function::IntFunctionId(14), data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        });
                        FunctionStep::Next(FunctionState::Int1Point2 { int0, int_function0 })
                    },
                    FunctionState::Int1Point2 { int0, int_function0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point2 { int0, int_function0 }); }
                        *budget -= 1;
                        FunctionStep::Next(FunctionState::Int1Point3 { int_function0: int_function0.clone(), int0 })
                    },
                    FunctionState::Int1Point3 { int_function0, int0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point3 { int_function0, int0 }); }
                        *budget -= 1;
                        let int1 = 7_i128;
                        if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(1)), point: data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(2),
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
                        }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int1Point4 { int_function0, int0, int1 })
                    },
                    FunctionState::Int1Point4 { int_function0, int0, int1 } => {
                        FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(1)), point: data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(2),
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
                        }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] } }
                    },
                    FunctionState::Int1Point5 { int0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point5 { int0 }); }
                        *budget -= 1;
                        let int_function0 = ops.int_reference(data::function::IntFunctionId(12), data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        });
                        FunctionStep::Next(FunctionState::Int1Point6 { int0, int_function0 })
                    },
                    FunctionState::Int1Point6 { int0, int_function0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point6 { int0, int_function0 }); }
                        *budget -= 1;
                        FunctionStep::Next(FunctionState::Int1Point3 { int_function0: int_function0.clone(), int0 })
                    },
                    FunctionState::Int2Point0 { int0, int1, bool0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point0 { int0, int1, bool0 }); }
                        *budget -= 1;
                        let int2 = 7_i128;
                        if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: data::compiled::CompiledCheckpoint {
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
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int2Point1 { int0, int1, bool0, int2 })
                    },
                    FunctionState::Int2Point1 { int0, int1, bool0, int2 } => {
                        FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: data::compiled::CompiledCheckpoint {
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
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }
                    },
                    FunctionState::Int3Point0 { int0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point0 { int0 }); }
                        *budget -= 1;
                        let int1 = 1_i128;
                        if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point: data::compiled::CompiledCheckpoint {
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
                        }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int3Point1 { int0, int1 })
                    },
                    FunctionState::Int3Point1 { int0, int1 } => {
                        FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point: data::compiled::CompiledCheckpoint {
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
                        }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }
                    },
                    FunctionState::Int9Point0 { int0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point0 { int0 }); }
                        *budget -= 1;
                        let int1 = 2_i128;
                        if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(9)), point: data::compiled::CompiledCheckpoint {
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
                        }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int9Point1 { int0, int1 })
                    },
                    FunctionState::Int9Point1 { int0, int1 } => {
                        FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(9)), point: data::compiled::CompiledCheckpoint {
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
                        }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }
                    },
                    FunctionState::Int11Point0 {  } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point0 {  }); }
                        *budget -= 1;
                        let int0 = 2_i128;
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
                        }, values: CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int11Point1 { int0 })
                    },
                    FunctionState::Int11Point1 { int0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point1 { int0 }); }
                        *budget -= 1;
                        let int1 = 9_i128;
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
                        }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int11Point2 { int0, int1 })
                    },
                    FunctionState::Int11Point2 { int0, int1 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point2 { int0, int1 }); }
                        *budget -= 1;
                        FunctionStep::IntBridge { function: data::function::IntFunctionId(0), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3876, 3885)), arguments: CallArguments { values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }, captures: None }, caller: IntReturn::Int11Call2 { int0, int1 } }
                    },
                    FunctionState::Int11Point3 { int0, int1, int2 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point3 { int0, int1, int2 }); }
                        *budget -= 1;
                        let int3 = 1_i128;
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
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int11Point4 { int0, int1, int2, int3 })
                    },
                    FunctionState::Int11Point4 { int0, int1, int2, int3 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point4 { int0, int1, int2, int3 }); }
                        *budget -= 1;
                        let bool0 = true;
                        FunctionStep::Next(FunctionState::Int11Point5 { int0, int1, int2, int3, bool0 })
                    },
                    FunctionState::Int11Point5 { int0, int1, int2, int3, bool0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point5 { int0, int1, int2, int3, bool0 }); }
                        *budget -= 1;
                        FunctionStep::IntCall { callee: FunctionState::Int1Point0 { int0: int3, bool0 }, caller: IntReturn::Int11Call5 { int0, int1, int2, int3, bool0 } }
                    },
                    FunctionState::Int11Point6 { int0, int1, int2, int3, bool0, int4 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point6 { int0, int1, int2, int3, bool0, int4 }); }
                        *budget -= 1;
                        let int5 = int2 + int4;
                        if int5 < i128::from(i64::MIN) || int5 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
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
                        FunctionStep::Next(FunctionState::Int11Point7 { int0, int1, int2, int3, bool0, int4, int5 })
                    },
                    FunctionState::Int11Point7 { int0, int1, int2, int3, bool0, int4, int5 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point7 { int0, int1, int2, int3, bool0, int4, int5 }); }
                        *budget -= 1;
                        let int6 = 0_i128;
                        if int6 < i128::from(i64::MIN) || int6 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
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
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into()], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int11Point8 { int0, int1, int2, int3, bool0, int4, int5, int6 })
                    },
                    FunctionState::Int11Point8 { int0, int1, int2, int3, bool0, int4, int5, int6 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point8 { int0, int1, int2, int3, bool0, int4, int5, int6 }); }
                        *budget -= 1;
                        let int7 = 3_i128;
                        if int7 < i128::from(i64::MIN) || int7 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
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
                        FunctionStep::Next(FunctionState::Int11Point9 { int0, int1, int2, int3, bool0, int4, int5, int6, int7 })
                    },
                    FunctionState::Int11Point9 { int0, int1, int2, int3, bool0, int4, int5, int6, int7 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point9 { int0, int1, int2, int3, bool0, int4, int5, int6, int7 }); }
                        *budget -= 1;
                        let bool1 = true;
                        FunctionStep::Next(FunctionState::Int11Point10 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1 })
                    },
                    FunctionState::Int11Point10 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point10 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1 }); }
                        *budget -= 1;
                        FunctionStep::IntCall { callee: FunctionState::Int2Point0 { int0: int6, int1: int7, bool0: bool1 }, caller: IntReturn::Int11Call10 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1 } }
                    },
                    FunctionState::Int11Point11 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point11 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8 }); }
                        *budget -= 1;
                        let int9 = int5 + int8;
                        if int9 < i128::from(i64::MIN) || int9 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 12,
                            ints: 10,
                            bools: 2,
                            bit_arrays: 0,
                            int_lists: 0,
                            strings: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into()], bools: vec![bool0, bool1], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int11Point12 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9 })
                    },
                    FunctionState::Int11Point12 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point12 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9 }); }
                        *budget -= 1;
                        FunctionStep::IntBridge { function: data::function::IntFunctionId(4), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3929, 3936)), arguments: CallArguments { values: CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }, captures: None }, caller: IntReturn::Int11Call12 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9 } }
                    },
                    FunctionState::Int11Point13 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9, int10 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point13 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9, int10 }); }
                        *budget -= 1;
                        let int11 = int9 + int10;
                        if int11 < i128::from(i64::MIN) || int11 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 14,
                            ints: 12,
                            bools: 2,
                            bit_arrays: 0,
                            int_lists: 0,
                            strings: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into()], bools: vec![bool0, bool1], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int11Point14 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9, int10, int11 })
                    },
                    FunctionState::Int11Point14 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9, int10, int11 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point14 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9, int10, int11 }); }
                        *budget -= 1;
                        let int12 = 2_i128;
                        if int12 < i128::from(i64::MIN) || int12 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 15,
                            ints: 13,
                            bools: 2,
                            bit_arrays: 0,
                            int_lists: 0,
                            strings: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into()], bools: vec![bool0, bool1], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int11Point15 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9, int10, int11, int12 })
                    },
                    FunctionState::Int11Point15 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9, int10, int11, int12 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point15 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9, int10, int11, int12 }); }
                        *budget -= 1;
                        FunctionStep::IntScalarBridge { function: data::function::IntFunctionId(6), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3939, 3953)), input: CallNativeInput::Int(int12.into()), caller: IntReturn::Int11Call15 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9, int10, int11, int12 } }
                    },
                    FunctionState::Int11Point16 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9, int10, int11, int12, int13 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point16 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9, int10, int11, int12, int13 }); }
                        *budget -= 1;
                        let int14 = int11 + int13;
                        if int14 < i128::from(i64::MIN) || int14 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 17,
                            ints: 15,
                            bools: 2,
                            bit_arrays: 0,
                            int_lists: 0,
                            strings: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into()], bools: vec![bool0, bool1], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point17 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9, int10, int11, int12, int13, int14 }); }
                        *budget -= 1;
                        FunctionStep::Int { value: int14, exit: data::graph::BlockGraphExitId(0) }
                    },
                    FunctionState::Int11Point17 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9, int10, int11, int12, int13, int14 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point17 { int0, int1, int2, int3, bool0, int4, int5, int6, int7, bool1, int8, int9, int10, int11, int12, int13, int14 }); }
                        *budget -= 1;
                        FunctionStep::Int { value: int14, exit: data::graph::BlockGraphExitId(0) }
                    },
                    FunctionState::Int23Point0 { int0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int23Point0 { int0 }); }
                        *budget -= 1;
                        let int1 = int0 + 1_i128;
                        if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(23)), point: data::compiled::CompiledCheckpoint {
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
                        }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int23Point1 { int0, int1 }); }
                        *budget -= 1;
                        FunctionStep::Int { value: int1, exit: data::graph::BlockGraphExitId(0) }
                    },
                    FunctionState::Int23Point1 { int0, int1 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int23Point1 { int0, int1 }); }
                        *budget -= 1;
                        FunctionStep::Int { value: int1, exit: data::graph::BlockGraphExitId(0) }
                    },
                }
            }
            fn calls_int_1_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                let active = match point {
                    0 => FunctionState::Int1Point0 { int0: values.int(0)?, bool0: values.bool(0)? },
                    1 => FunctionState::Int1Point1 { int0: values.int(0)? },
                    2 => FunctionState::Int1Point2 { int0: values.int(0)?, int_function0: values.int_function(0)? },
                    3 => FunctionState::Int1Point3 { int_function0: values.int_function(0)?, int0: values.int(0)? },
                    4 => FunctionState::Int1Point4 { int_function0: values.int_function(0)?, int0: values.int(0)?, int1: values.int(1)? },
                    5 => FunctionState::Int1Point5 { int0: values.int(0)? },
                    6 => FunctionState::Int1Point6 { int0: values.int(0)?, int_function0: values.int_function(0)? },
                    _ => return None,
                };
                Some(active)
            }
            fn calls_int_1_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(1)), point, values) { return Some(execution); }
                let active = calls_int_1_state(point, values)?;
                Some(Box::new(FunctionExecution::new(active)))
            }
            fn calls_int_2_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                let active = match point {
                    0 => FunctionState::Int2Point0 { int0: values.int(0)?, int1: values.int(1)?, bool0: values.bool(0)? },
                    1 => FunctionState::Int2Point1 { int0: values.int(0)?, int1: values.int(1)?, bool0: values.bool(0)?, int2: values.int(2)? },
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
                    0 => FunctionState::Int3Point0 { int0: values.int(0)? },
                    1 => FunctionState::Int3Point1 { int0: values.int(0)?, int1: values.int(1)? },
                    _ => return None,
                };
                Some(active)
            }
            fn calls_int_3_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point, values) { return Some(execution); }
                let active = calls_int_3_state(point, values)?;
                Some(Box::new(FunctionExecution::new(active)))
            }
            fn calls_int_9_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                let active = match point {
                    0 => FunctionState::Int9Point0 { int0: values.int(0)? },
                    1 => FunctionState::Int9Point1 { int0: values.int(0)?, int1: values.int(1)? },
                    _ => return None,
                };
                Some(active)
            }
            fn calls_int_9_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(9)), point, values) { return Some(execution); }
                let active = calls_int_9_state(point, values)?;
                Some(Box::new(FunctionExecution::new(active)))
            }
            fn calls_int_11_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                let active = match point {
                    0 => FunctionState::Int11Point0 {  },
                    1 => FunctionState::Int11Point1 { int0: values.int(0)? },
                    2 => FunctionState::Int11Point2 { int0: values.int(0)?, int1: values.int(1)? },
                    3 => FunctionState::Int11Point3 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                    4 => FunctionState::Int11Point4 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)? },
                    5 => FunctionState::Int11Point5 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, bool0: values.bool(0)? },
                    6 => FunctionState::Int11Point6 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, bool0: values.bool(0)?, int4: values.int(4)? },
                    7 => FunctionState::Int11Point7 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, bool0: values.bool(0)?, int4: values.int(4)?, int5: values.int(5)? },
                    8 => FunctionState::Int11Point8 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, bool0: values.bool(0)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)? },
                    9 => FunctionState::Int11Point9 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, bool0: values.bool(0)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)? },
                    10 => FunctionState::Int11Point10 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, bool0: values.bool(0)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, bool1: values.bool(1)? },
                    11 => FunctionState::Int11Point11 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, bool0: values.bool(0)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, bool1: values.bool(1)?, int8: values.int(8)? },
                    12 => FunctionState::Int11Point12 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, bool0: values.bool(0)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, bool1: values.bool(1)?, int8: values.int(8)?, int9: values.int(9)? },
                    13 => FunctionState::Int11Point13 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, bool0: values.bool(0)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, bool1: values.bool(1)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)? },
                    14 => FunctionState::Int11Point14 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, bool0: values.bool(0)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, bool1: values.bool(1)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)? },
                    15 => FunctionState::Int11Point15 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, bool0: values.bool(0)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, bool1: values.bool(1)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, int12: values.int(12)? },
                    16 => FunctionState::Int11Point16 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, bool0: values.bool(0)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, bool1: values.bool(1)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, int12: values.int(12)?, int13: values.int(13)? },
                    17 => FunctionState::Int11Point17 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)?, bool0: values.bool(0)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int7: values.int(7)?, bool1: values.bool(1)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)? },
                    _ => return None,
                };
                Some(active)
            }
            fn calls_int_11_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point, values) { return Some(execution); }
                let active = calls_int_11_state(point, values)?;
                Some(Box::new(FunctionExecution::new(active)))
            }
            fn calls_int_23_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                let active = match point {
                    0 => FunctionState::Int23Point0 { int0: values.int(0)? },
                    1 => FunctionState::Int23Point1 { int0: values.int(0)?, int1: values.int(1)? },
                    _ => return None,
                };
                Some(active)
            }
            fn calls_int_23_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(23)), point, values) { return Some(execution); }
                let active = calls_int_23_state(point, values)?;
                Some(Box::new(FunctionExecution::new(active)))
            }

            fn custom_loop_int_13(
                point: usize,
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CustomLoopProgress {

                const RESUME: [
                    fn(&mut data::compiled::custom_loop::CustomLoopValues, &data::compiled::custom_loop::CustomListOps<'_>, &mut usize) -> CustomLoopResume;
                    6
                ] = [
                    |values, _lists, budget| {
                        let _custom_list0 = values.custom_lists.remove(0);
                        let _int_function0 = values.int_functions.remove(0);
                        CustomLoopResume::Exit(custom_loop_int_13_entry((values.ints[0], _custom_list0, _int_function0,), values, _lists, budget))
                    },
                    custom_loop_int_13_resume_1,
                    custom_loop_int_13_resume_2,
                    custom_loop_int_13_resume_3,
                    custom_loop_int_13_resume_4,
                    custom_loop_int_13_resume_5,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, _lists, budget) {
                        CustomLoopResume::Next(next) => point = next,
                        CustomLoopResume::Exit(progress) => return progress,
                    }
                }
            }

            fn custom_loop_int_13_entry(
                inputs: (i128, data::compiled::custom_loop::CustomList, data::compiled::custom_loop::IntCallback,),
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CustomLoopProgress {
                let (mut b0_i0, mut b0_l0, mut b0_f0,) = inputs;
                'repeat: loop {
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([]);
                        values.custom_lists.clear();
                        values.custom_lists.extend([b0_l0]);
                        values.int_functions.clear();
                        values.int_functions.extend([b0_f0]);
                        values.bool_functions.clear();
                        values.bool_functions.extend([]);
                        return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(0));
                    }
                    *budget -= 1;
                    if b0_l0.is_empty() {
                        let _next = (b0_i0,);
                        drop(b0_l0);
                        drop(b0_f0);
                        let (b1_i0,) = _next;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([]);
                            values.int_functions.clear();
                            values.int_functions.extend([]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([]);
                            return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(1));
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([]);
                        values.custom_lists.clear();
                        values.custom_lists.extend([]);
                        values.int_functions.clear();
                        values.int_functions.extend([]);
                        values.bool_functions.clear();
                        values.bool_functions.extend([]);
                        return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)));
                    } else {
                        let (b2_i0, b2_l0, b2_f0,) = (b0_i0, b0_l0, b0_f0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([b2_l0]);
                            values.int_functions.clear();
                            values.int_functions.extend([b2_f0]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([]);
                            return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(2));
                        }
                        let b2_c0 = match _lists.index(&b2_l0, 0) {
                            Some(value) => value,
                            None => {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.customs.clear();
                                values.customs.extend([]);
                                values.custom_lists.clear();
                                values.custom_lists.extend([b2_l0]);
                                values.int_functions.clear();
                                values.int_functions.extend([b2_f0]);
                                values.bool_functions.clear();
                                values.bool_functions.extend([]);
                                return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Interpreted(2));
                            }
                        };
                        *budget -= 1;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([b2_c0]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([b2_l0]);
                            values.int_functions.clear();
                            values.int_functions.extend([b2_f0]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([]);
                            return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(3));
                        }
                        *budget -= 1;
                        let b2_l1 = _lists.tail(&b2_l0, data::type_::CustomListTypeId {
                            list_type: data::type_::ListTypeId(0),
                            item_type: data::type_::CustomTypeId(0),
                        }, 1);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([b2_c0]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([b2_l0, b2_l1]);
                            values.int_functions.clear();
                            values.int_functions.extend([b2_f0]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([]);
                            return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(4));
                        }
                        *budget -= 1;
                        let b2_i1 = match b2_f0.call(
                            &data::compiled::custom_loop::CallbackArguments {
                                ints: &[b2_i0],
                                bools: &[],
                                customs: &[&b2_c0],
                            },
                            &mut values.callee, budget,
                            ) {
                            data::compiled::custom_loop::CallbackProgress::Complete(value) => value,
                            data::compiled::custom_loop::CallbackProgress::Stopped(progress) => {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.customs.clear();
                                values.customs.extend([b2_c0]);
                                values.custom_lists.clear();
                                values.custom_lists.extend([b2_l0, b2_l1]);
                                values.int_functions.clear();
                                values.int_functions.extend([b2_f0]);
                                values.bool_functions.clear();
                                values.bool_functions.extend([]);
                                return data::compiled::custom_loop::CustomLoopProgress::Call { point: 4, progress };
                            }
                        };
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([b2_c0]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([b2_l0, b2_l1]);
                            values.int_functions.clear();
                            values.int_functions.extend([b2_f0]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([]);
                            return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(5));
                        }
                        *budget -= 1;
                        {
                            let _next = (b2_i1, b2_l1, b2_f0,);
                            drop(b2_c0);
                            drop(b2_l0);
                            (b0_i0, b0_l0, b0_f0,) = _next;
                            continue 'repeat;
                        }
                    }
                }
            }

            fn custom_loop_int_13_resume_1(
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> CustomLoopResume {
                let (b1_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([]);
                    values.int_functions.clear();
                    values.int_functions.extend([]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(1)));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.customs.clear();
                values.customs.extend([]);
                values.custom_lists.clear();
                values.custom_lists.extend([]);
                values.int_functions.clear();
                values.int_functions.extend([]);
                values.bool_functions.clear();
                values.bool_functions.extend([]);
                CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))))
            }

            fn custom_loop_int_13_resume_2(
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> CustomLoopResume {
                let _custom_list0 = values.custom_lists.remove(0);
                let _int_function0 = values.int_functions.remove(0);
                let (b2_i0, b2_l0, b2_f0,) = (values.ints[0], _custom_list0, _int_function0,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b2_l0]);
                    values.int_functions.clear();
                    values.int_functions.extend([b2_f0]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(2)));
                }
                let b2_c0 = match _lists.index(&b2_l0, 0) {
                    Some(value) => value,
                    None => {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([]);
                        values.custom_lists.clear();
                        values.custom_lists.extend([b2_l0]);
                        values.int_functions.clear();
                        values.int_functions.extend([b2_f0]);
                        values.bool_functions.clear();
                        values.bool_functions.extend([]);
                        return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Interpreted(2)));
                    }
                };
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.customs.clear();
                values.customs.extend([b2_c0]);
                values.custom_lists.clear();
                values.custom_lists.extend([b2_l0]);
                values.int_functions.clear();
                values.int_functions.extend([b2_f0]);
                values.bool_functions.clear();
                values.bool_functions.extend([]);
                CustomLoopResume::Next(3)
            }

            fn custom_loop_int_13_resume_3(
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> CustomLoopResume {
                let _custom0 = values.customs.remove(0);
                let _custom_list0 = values.custom_lists.remove(0);
                let _int_function0 = values.int_functions.remove(0);
                let (b2_i0, b2_c0, b2_l0, b2_f0,) = (values.ints[0], _custom0, _custom_list0, _int_function0,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([b2_c0]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b2_l0]);
                    values.int_functions.clear();
                    values.int_functions.extend([b2_f0]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(3)));
                }
                *budget -= 1;
                let b2_l1 = _lists.tail(&b2_l0, data::type_::CustomListTypeId {
                    list_type: data::type_::ListTypeId(0),
                    item_type: data::type_::CustomTypeId(0),
                }, 1);

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.customs.clear();
                values.customs.extend([b2_c0]);
                values.custom_lists.clear();
                values.custom_lists.extend([b2_l0, b2_l1]);
                values.int_functions.clear();
                values.int_functions.extend([b2_f0]);
                values.bool_functions.clear();
                values.bool_functions.extend([]);
                CustomLoopResume::Next(4)
            }

            fn custom_loop_int_13_resume_4(
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> CustomLoopResume {
                let _custom0 = values.customs.remove(0);
                let _custom_list1 = values.custom_lists.remove(1);
                let _custom_list0 = values.custom_lists.remove(0);
                let _int_function0 = values.int_functions.remove(0);
                let (b2_i0, b2_c0, b2_l0, b2_l1, b2_f0,) = (values.ints[0], _custom0, _custom_list0, _custom_list1, _int_function0,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([b2_c0]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b2_l0, b2_l1]);
                    values.int_functions.clear();
                    values.int_functions.extend([b2_f0]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(4)));
                }
                *budget -= 1;
                let b2_i1 = match b2_f0.call(
                    &data::compiled::custom_loop::CallbackArguments {
                        ints: &[b2_i0],
                        bools: &[],
                        customs: &[&b2_c0],
                    },
                    &mut values.callee, budget,
                    ) {
                    data::compiled::custom_loop::CallbackProgress::Complete(value) => value,
                    data::compiled::custom_loop::CallbackProgress::Stopped(progress) => {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([b2_c0]);
                        values.custom_lists.clear();
                        values.custom_lists.extend([b2_l0, b2_l1]);
                        values.int_functions.clear();
                        values.int_functions.extend([b2_f0]);
                        values.bool_functions.clear();
                        values.bool_functions.extend([]);
                        return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Call { point: 4, progress });
                    }
                };

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.customs.clear();
                values.customs.extend([b2_c0]);
                values.custom_lists.clear();
                values.custom_lists.extend([b2_l0, b2_l1]);
                values.int_functions.clear();
                values.int_functions.extend([b2_f0]);
                values.bool_functions.clear();
                values.bool_functions.extend([]);
                CustomLoopResume::Next(5)
            }

            fn custom_loop_int_13_resume_5(
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> CustomLoopResume {
                let _custom0 = values.customs.remove(0);
                let _custom_list1 = values.custom_lists.remove(1);
                let _custom_list0 = values.custom_lists.remove(0);
                let _int_function0 = values.int_functions.remove(0);
                let (b2_i0, b2_i1, b2_c0, b2_l0, b2_l1, b2_f0,) = (values.ints[0], values.ints[1], _custom0, _custom_list0, _custom_list1, _int_function0,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([b2_c0]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b2_l0, b2_l1]);
                    values.int_functions.clear();
                    values.int_functions.extend([b2_f0]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(5)));
                }
                *budget -= 1;
                {
                    let _next = (b2_i1, b2_l1, b2_f0,);
                    drop(b2_c0);
                    drop(b2_l0);
                    let (b0_i0, b0_l0, b0_f0,) = _next;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b0_l0]);
                    values.int_functions.clear();
                    values.int_functions.extend([b0_f0]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    CustomLoopResume::Next(0)
                }
            }

            fn custom_loop_int_20(
                point: usize,
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CustomLoopProgress {

                const RESUME: [
                    fn(&mut data::compiled::custom_loop::CustomLoopValues, &data::compiled::custom_loop::CustomListOps<'_>, &mut usize) -> CustomLoopResume;
                    7
                ] = [
                    |values, _lists, budget| {
                        let _custom_list0 = values.custom_lists.remove(0);
                        let _int_function0 = values.int_functions.remove(0);
                        CustomLoopResume::Exit(custom_loop_int_20_entry((values.ints[0], _custom_list0, _int_function0,), values, _lists, budget))
                    },
                    custom_loop_int_20_resume_1,
                    custom_loop_int_20_resume_2,
                    custom_loop_int_20_resume_3,
                    custom_loop_int_20_resume_4,
                    custom_loop_int_20_resume_5,
                    custom_loop_int_20_resume_6,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, _lists, budget) {
                        CustomLoopResume::Next(next) => point = next,
                        CustomLoopResume::Exit(progress) => return progress,
                    }
                }
            }

            fn custom_loop_int_20_entry(
                inputs: (i128, data::compiled::custom_loop::CustomList, data::compiled::custom_loop::IntCallback,),
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CustomLoopProgress {
                let (mut b0_i0, mut b0_l0, mut b0_f0,) = inputs;
                'repeat: loop {
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([]);
                        values.custom_lists.clear();
                        values.custom_lists.extend([b0_l0]);
                        values.int_functions.clear();
                        values.int_functions.extend([b0_f0]);
                        values.bool_functions.clear();
                        values.bool_functions.extend([]);
                        return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(0));
                    }
                    *budget -= 1;
                    if b0_l0.is_empty() {
                        let _next = (b0_i0,);
                        drop(b0_l0);
                        drop(b0_f0);
                        let (b1_i0,) = _next;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([]);
                            values.int_functions.clear();
                            values.int_functions.extend([]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([]);
                            return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(1));
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([]);
                        values.custom_lists.clear();
                        values.custom_lists.extend([]);
                        values.int_functions.clear();
                        values.int_functions.extend([]);
                        values.bool_functions.clear();
                        values.bool_functions.extend([]);
                        return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)));
                    } else {
                        let (b2_i0, b2_l0, b2_f0,) = (b0_i0, b0_l0, b0_f0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([b2_l0]);
                            values.int_functions.clear();
                            values.int_functions.extend([b2_f0]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([]);
                            return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(2));
                        }
                        let b2_c0 = match _lists.index(&b2_l0, 0) {
                            Some(value) => value,
                            None => {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.customs.clear();
                                values.customs.extend([]);
                                values.custom_lists.clear();
                                values.custom_lists.extend([b2_l0]);
                                values.int_functions.clear();
                                values.int_functions.extend([b2_f0]);
                                values.bool_functions.clear();
                                values.bool_functions.extend([]);
                                return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Interpreted(2));
                            }
                        };
                        *budget -= 1;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([b2_c0]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([b2_l0]);
                            values.int_functions.clear();
                            values.int_functions.extend([b2_f0]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([]);
                            return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(3));
                        }
                        *budget -= 1;
                        let b2_l1 = _lists.tail(&b2_l0, data::type_::CustomListTypeId {
                            list_type: data::type_::ListTypeId(0),
                            item_type: data::type_::CustomTypeId(0),
                        }, 1);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([b2_c0]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([b2_l0, b2_l1]);
                            values.int_functions.clear();
                            values.int_functions.extend([b2_f0]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([]);
                            return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(4));
                        }
                        *budget -= 1;
                        let b2_i1 = b2_i0 + 1_i128;
                        if b2_i1 < i128::from(i64::MIN) || b2_i1 > i128::from(i64::MAX) {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([b2_c0]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([b2_l0, b2_l1]);
                            values.int_functions.clear();
                            values.int_functions.extend([b2_f0]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([]);
                            return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Interpreted(5));
                        }
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([b2_c0]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([b2_l0, b2_l1]);
                            values.int_functions.clear();
                            values.int_functions.extend([b2_f0]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([]);
                            return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(5));
                        }
                        *budget -= 1;
                        let b2_i2 = match b2_f0.call(
                            &data::compiled::custom_loop::CallbackArguments {
                                ints: &[b2_i1],
                                bools: &[],
                                customs: &[&b2_c0],
                            },
                            &mut values.callee, budget,
                            ) {
                            data::compiled::custom_loop::CallbackProgress::Complete(value) => value,
                            data::compiled::custom_loop::CallbackProgress::Stopped(progress) => {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.customs.clear();
                                values.customs.extend([b2_c0]);
                                values.custom_lists.clear();
                                values.custom_lists.extend([b2_l0, b2_l1]);
                                values.int_functions.clear();
                                values.int_functions.extend([b2_f0]);
                                values.bool_functions.clear();
                                values.bool_functions.extend([]);
                                return data::compiled::custom_loop::CustomLoopProgress::Call { point: 5, progress };
                            }
                        };
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([b2_c0]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([b2_l0, b2_l1]);
                            values.int_functions.clear();
                            values.int_functions.extend([b2_f0]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([]);
                            return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(6));
                        }
                        *budget -= 1;
                        {
                            let _next = (b2_i2, b2_l1, b2_f0,);
                            drop(b2_c0);
                            drop(b2_l0);
                            (b0_i0, b0_l0, b0_f0,) = _next;
                            continue 'repeat;
                        }
                    }
                }
            }

            fn custom_loop_int_20_resume_1(
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> CustomLoopResume {
                let (b1_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([]);
                    values.int_functions.clear();
                    values.int_functions.extend([]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(1)));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.customs.clear();
                values.customs.extend([]);
                values.custom_lists.clear();
                values.custom_lists.extend([]);
                values.int_functions.clear();
                values.int_functions.extend([]);
                values.bool_functions.clear();
                values.bool_functions.extend([]);
                CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))))
            }

            fn custom_loop_int_20_resume_2(
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> CustomLoopResume {
                let _custom_list0 = values.custom_lists.remove(0);
                let _int_function0 = values.int_functions.remove(0);
                let (b2_i0, b2_l0, b2_f0,) = (values.ints[0], _custom_list0, _int_function0,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b2_l0]);
                    values.int_functions.clear();
                    values.int_functions.extend([b2_f0]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(2)));
                }
                let b2_c0 = match _lists.index(&b2_l0, 0) {
                    Some(value) => value,
                    None => {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([]);
                        values.custom_lists.clear();
                        values.custom_lists.extend([b2_l0]);
                        values.int_functions.clear();
                        values.int_functions.extend([b2_f0]);
                        values.bool_functions.clear();
                        values.bool_functions.extend([]);
                        return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Interpreted(2)));
                    }
                };
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.customs.clear();
                values.customs.extend([b2_c0]);
                values.custom_lists.clear();
                values.custom_lists.extend([b2_l0]);
                values.int_functions.clear();
                values.int_functions.extend([b2_f0]);
                values.bool_functions.clear();
                values.bool_functions.extend([]);
                CustomLoopResume::Next(3)
            }

            fn custom_loop_int_20_resume_3(
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> CustomLoopResume {
                let _custom0 = values.customs.remove(0);
                let _custom_list0 = values.custom_lists.remove(0);
                let _int_function0 = values.int_functions.remove(0);
                let (b2_i0, b2_c0, b2_l0, b2_f0,) = (values.ints[0], _custom0, _custom_list0, _int_function0,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([b2_c0]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b2_l0]);
                    values.int_functions.clear();
                    values.int_functions.extend([b2_f0]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(3)));
                }
                *budget -= 1;
                let b2_l1 = _lists.tail(&b2_l0, data::type_::CustomListTypeId {
                    list_type: data::type_::ListTypeId(0),
                    item_type: data::type_::CustomTypeId(0),
                }, 1);

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.customs.clear();
                values.customs.extend([b2_c0]);
                values.custom_lists.clear();
                values.custom_lists.extend([b2_l0, b2_l1]);
                values.int_functions.clear();
                values.int_functions.extend([b2_f0]);
                values.bool_functions.clear();
                values.bool_functions.extend([]);
                CustomLoopResume::Next(4)
            }

            fn custom_loop_int_20_resume_4(
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> CustomLoopResume {
                let _custom0 = values.customs.remove(0);
                let _custom_list1 = values.custom_lists.remove(1);
                let _custom_list0 = values.custom_lists.remove(0);
                let _int_function0 = values.int_functions.remove(0);
                let (b2_i0, b2_c0, b2_l0, b2_l1, b2_f0,) = (values.ints[0], _custom0, _custom_list0, _custom_list1, _int_function0,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([b2_c0]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b2_l0, b2_l1]);
                    values.int_functions.clear();
                    values.int_functions.extend([b2_f0]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(4)));
                }
                *budget -= 1;
                let b2_i1 = b2_i0 + 1_i128;
                if b2_i1 < i128::from(i64::MIN) || b2_i1 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([b2_c0]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b2_l0, b2_l1]);
                    values.int_functions.clear();
                    values.int_functions.extend([b2_f0]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Interpreted(5)));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.customs.clear();
                values.customs.extend([b2_c0]);
                values.custom_lists.clear();
                values.custom_lists.extend([b2_l0, b2_l1]);
                values.int_functions.clear();
                values.int_functions.extend([b2_f0]);
                values.bool_functions.clear();
                values.bool_functions.extend([]);
                CustomLoopResume::Next(5)
            }

            fn custom_loop_int_20_resume_5(
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> CustomLoopResume {
                let _custom0 = values.customs.remove(0);
                let _custom_list1 = values.custom_lists.remove(1);
                let _custom_list0 = values.custom_lists.remove(0);
                let _int_function0 = values.int_functions.remove(0);
                let (b2_i0, b2_i1, b2_c0, b2_l0, b2_l1, b2_f0,) = (values.ints[0], values.ints[1], _custom0, _custom_list0, _custom_list1, _int_function0,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([b2_c0]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b2_l0, b2_l1]);
                    values.int_functions.clear();
                    values.int_functions.extend([b2_f0]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(5)));
                }
                *budget -= 1;
                let b2_i2 = match b2_f0.call(
                    &data::compiled::custom_loop::CallbackArguments {
                        ints: &[b2_i1],
                        bools: &[],
                        customs: &[&b2_c0],
                    },
                    &mut values.callee, budget,
                    ) {
                    data::compiled::custom_loop::CallbackProgress::Complete(value) => value,
                    data::compiled::custom_loop::CallbackProgress::Stopped(progress) => {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([b2_c0]);
                        values.custom_lists.clear();
                        values.custom_lists.extend([b2_l0, b2_l1]);
                        values.int_functions.clear();
                        values.int_functions.extend([b2_f0]);
                        values.bool_functions.clear();
                        values.bool_functions.extend([]);
                        return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Call { point: 5, progress });
                    }
                };

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.customs.clear();
                values.customs.extend([b2_c0]);
                values.custom_lists.clear();
                values.custom_lists.extend([b2_l0, b2_l1]);
                values.int_functions.clear();
                values.int_functions.extend([b2_f0]);
                values.bool_functions.clear();
                values.bool_functions.extend([]);
                CustomLoopResume::Next(6)
            }

            fn custom_loop_int_20_resume_6(
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> CustomLoopResume {
                let _custom0 = values.customs.remove(0);
                let _custom_list1 = values.custom_lists.remove(1);
                let _custom_list0 = values.custom_lists.remove(0);
                let _int_function0 = values.int_functions.remove(0);
                let (b2_i0, b2_i1, b2_i2, b2_c0, b2_l0, b2_l1, b2_f0,) = (values.ints[0], values.ints[1], values.ints[2], _custom0, _custom_list0, _custom_list1, _int_function0,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([b2_c0]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b2_l0, b2_l1]);
                    values.int_functions.clear();
                    values.int_functions.extend([b2_f0]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(6)));
                }
                *budget -= 1;
                {
                    let _next = (b2_i2, b2_l1, b2_f0,);
                    drop(b2_c0);
                    drop(b2_l0);
                    let (b0_i0, b0_l0, b0_f0,) = _next;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b0_l0]);
                    values.int_functions.clear();
                    values.int_functions.extend([b0_f0]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    CustomLoopResume::Next(0)
                }
            }

            fn custom_loop_int_22(
                point: usize,
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CustomLoopProgress {

                const RESUME: [
                    fn(&mut data::compiled::custom_loop::CustomLoopValues, &data::compiled::custom_loop::CustomListOps<'_>, &mut usize) -> CustomLoopResume;
                    6
                ] = [
                    |values, _lists, budget| {
                        let _custom_list0 = values.custom_lists.remove(0);
                        let _int_function0 = values.int_functions.remove(0);
                        CustomLoopResume::Exit(custom_loop_int_22_entry((values.ints[0], _custom_list0, _int_function0,), values, _lists, budget))
                    },
                    custom_loop_int_22_resume_1,
                    custom_loop_int_22_resume_2,
                    custom_loop_int_22_resume_3,
                    custom_loop_int_22_resume_4,
                    custom_loop_int_22_resume_5,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, _lists, budget) {
                        CustomLoopResume::Next(next) => point = next,
                        CustomLoopResume::Exit(progress) => return progress,
                    }
                }
            }

            fn custom_loop_int_22_entry(
                inputs: (i128, data::compiled::custom_loop::CustomList, data::compiled::custom_loop::IntCallback,),
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CustomLoopProgress {
                let (mut b0_i0, mut b0_l0, mut b0_f0,) = inputs;
                'repeat: loop {
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([]);
                        values.custom_lists.clear();
                        values.custom_lists.extend([b0_l0]);
                        values.int_functions.clear();
                        values.int_functions.extend([b0_f0]);
                        values.bool_functions.clear();
                        values.bool_functions.extend([]);
                        return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(0));
                    }
                    *budget -= 1;
                    if b0_l0.is_empty() {
                        let _next = (b0_i0,);
                        drop(b0_l0);
                        drop(b0_f0);
                        let (b1_i0,) = _next;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([]);
                            values.int_functions.clear();
                            values.int_functions.extend([]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([]);
                            return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(1));
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([]);
                        values.custom_lists.clear();
                        values.custom_lists.extend([]);
                        values.int_functions.clear();
                        values.int_functions.extend([]);
                        values.bool_functions.clear();
                        values.bool_functions.extend([]);
                        return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)));
                    } else {
                        let (b2_i0, b2_l0, b2_f0,) = (b0_i0, b0_l0, b0_f0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([b2_l0]);
                            values.int_functions.clear();
                            values.int_functions.extend([b2_f0]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([]);
                            return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(2));
                        }
                        let b2_c0 = match _lists.index(&b2_l0, 0) {
                            Some(value) => value,
                            None => {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.customs.clear();
                                values.customs.extend([]);
                                values.custom_lists.clear();
                                values.custom_lists.extend([b2_l0]);
                                values.int_functions.clear();
                                values.int_functions.extend([b2_f0]);
                                values.bool_functions.clear();
                                values.bool_functions.extend([]);
                                return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Interpreted(2));
                            }
                        };
                        *budget -= 1;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([b2_c0]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([b2_l0]);
                            values.int_functions.clear();
                            values.int_functions.extend([b2_f0]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([]);
                            return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(3));
                        }
                        *budget -= 1;
                        let b2_l1 = _lists.tail(&b2_l0, data::type_::CustomListTypeId {
                            list_type: data::type_::ListTypeId(1),
                            item_type: data::type_::CustomTypeId(1),
                        }, 1);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([b2_c0]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([b2_l0, b2_l1]);
                            values.int_functions.clear();
                            values.int_functions.extend([b2_f0]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([]);
                            return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(4));
                        }
                        *budget -= 1;
                        let b2_i1 = match b2_f0.call(
                            &data::compiled::custom_loop::CallbackArguments {
                                ints: &[b2_i0],
                                bools: &[],
                                customs: &[&b2_c0],
                            },
                            &mut values.callee, budget,
                            ) {
                            data::compiled::custom_loop::CallbackProgress::Complete(value) => value,
                            data::compiled::custom_loop::CallbackProgress::Stopped(progress) => {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.customs.clear();
                                values.customs.extend([b2_c0]);
                                values.custom_lists.clear();
                                values.custom_lists.extend([b2_l0, b2_l1]);
                                values.int_functions.clear();
                                values.int_functions.extend([b2_f0]);
                                values.bool_functions.clear();
                                values.bool_functions.extend([]);
                                return data::compiled::custom_loop::CustomLoopProgress::Call { point: 4, progress };
                            }
                        };
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([b2_c0]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([b2_l0, b2_l1]);
                            values.int_functions.clear();
                            values.int_functions.extend([b2_f0]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([]);
                            return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(5));
                        }
                        *budget -= 1;
                        {
                            let _next = (b2_i1, b2_l1, b2_f0,);
                            drop(b2_c0);
                            drop(b2_l0);
                            (b0_i0, b0_l0, b0_f0,) = _next;
                            continue 'repeat;
                        }
                    }
                }
            }

            fn custom_loop_int_22_resume_1(
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> CustomLoopResume {
                let (b1_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([]);
                    values.int_functions.clear();
                    values.int_functions.extend([]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(1)));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b1_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.customs.clear();
                values.customs.extend([]);
                values.custom_lists.clear();
                values.custom_lists.extend([]);
                values.int_functions.clear();
                values.int_functions.extend([]);
                values.bool_functions.clear();
                values.bool_functions.extend([]);
                CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))))
            }

            fn custom_loop_int_22_resume_2(
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> CustomLoopResume {
                let _custom_list0 = values.custom_lists.remove(0);
                let _int_function0 = values.int_functions.remove(0);
                let (b2_i0, b2_l0, b2_f0,) = (values.ints[0], _custom_list0, _int_function0,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b2_l0]);
                    values.int_functions.clear();
                    values.int_functions.extend([b2_f0]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(2)));
                }
                let b2_c0 = match _lists.index(&b2_l0, 0) {
                    Some(value) => value,
                    None => {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([]);
                        values.custom_lists.clear();
                        values.custom_lists.extend([b2_l0]);
                        values.int_functions.clear();
                        values.int_functions.extend([b2_f0]);
                        values.bool_functions.clear();
                        values.bool_functions.extend([]);
                        return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Interpreted(2)));
                    }
                };
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.customs.clear();
                values.customs.extend([b2_c0]);
                values.custom_lists.clear();
                values.custom_lists.extend([b2_l0]);
                values.int_functions.clear();
                values.int_functions.extend([b2_f0]);
                values.bool_functions.clear();
                values.bool_functions.extend([]);
                CustomLoopResume::Next(3)
            }

            fn custom_loop_int_22_resume_3(
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> CustomLoopResume {
                let _custom0 = values.customs.remove(0);
                let _custom_list0 = values.custom_lists.remove(0);
                let _int_function0 = values.int_functions.remove(0);
                let (b2_i0, b2_c0, b2_l0, b2_f0,) = (values.ints[0], _custom0, _custom_list0, _int_function0,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([b2_c0]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b2_l0]);
                    values.int_functions.clear();
                    values.int_functions.extend([b2_f0]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(3)));
                }
                *budget -= 1;
                let b2_l1 = _lists.tail(&b2_l0, data::type_::CustomListTypeId {
                    list_type: data::type_::ListTypeId(1),
                    item_type: data::type_::CustomTypeId(1),
                }, 1);

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.customs.clear();
                values.customs.extend([b2_c0]);
                values.custom_lists.clear();
                values.custom_lists.extend([b2_l0, b2_l1]);
                values.int_functions.clear();
                values.int_functions.extend([b2_f0]);
                values.bool_functions.clear();
                values.bool_functions.extend([]);
                CustomLoopResume::Next(4)
            }

            fn custom_loop_int_22_resume_4(
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> CustomLoopResume {
                let _custom0 = values.customs.remove(0);
                let _custom_list1 = values.custom_lists.remove(1);
                let _custom_list0 = values.custom_lists.remove(0);
                let _int_function0 = values.int_functions.remove(0);
                let (b2_i0, b2_c0, b2_l0, b2_l1, b2_f0,) = (values.ints[0], _custom0, _custom_list0, _custom_list1, _int_function0,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([b2_c0]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b2_l0, b2_l1]);
                    values.int_functions.clear();
                    values.int_functions.extend([b2_f0]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(4)));
                }
                *budget -= 1;
                let b2_i1 = match b2_f0.call(
                    &data::compiled::custom_loop::CallbackArguments {
                        ints: &[b2_i0],
                        bools: &[],
                        customs: &[&b2_c0],
                    },
                    &mut values.callee, budget,
                    ) {
                    data::compiled::custom_loop::CallbackProgress::Complete(value) => value,
                    data::compiled::custom_loop::CallbackProgress::Stopped(progress) => {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([b2_c0]);
                        values.custom_lists.clear();
                        values.custom_lists.extend([b2_l0, b2_l1]);
                        values.int_functions.clear();
                        values.int_functions.extend([b2_f0]);
                        values.bool_functions.clear();
                        values.bool_functions.extend([]);
                        return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Call { point: 4, progress });
                    }
                };

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.customs.clear();
                values.customs.extend([b2_c0]);
                values.custom_lists.clear();
                values.custom_lists.extend([b2_l0, b2_l1]);
                values.int_functions.clear();
                values.int_functions.extend([b2_f0]);
                values.bool_functions.clear();
                values.bool_functions.extend([]);
                CustomLoopResume::Next(5)
            }

            fn custom_loop_int_22_resume_5(
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> CustomLoopResume {
                let _custom0 = values.customs.remove(0);
                let _custom_list1 = values.custom_lists.remove(1);
                let _custom_list0 = values.custom_lists.remove(0);
                let _int_function0 = values.int_functions.remove(0);
                let (b2_i0, b2_i1, b2_c0, b2_l0, b2_l1, b2_f0,) = (values.ints[0], values.ints[1], _custom0, _custom_list0, _custom_list1, _int_function0,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([b2_c0]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b2_l0, b2_l1]);
                    values.int_functions.clear();
                    values.int_functions.extend([b2_f0]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(5)));
                }
                *budget -= 1;
                {
                    let _next = (b2_i1, b2_l1, b2_f0,);
                    drop(b2_c0);
                    drop(b2_l0);
                    let (b0_i0, b0_l0, b0_f0,) = _next;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.customs.clear();
                    values.customs.extend([]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b0_l0]);
                    values.int_functions.clear();
                    values.int_functions.extend([b0_f0]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    CustomLoopResume::Next(0)
                }
            }

            fn custom_loop_bool_2(
                point: usize,
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CustomLoopProgress {

                const RESUME: [
                    fn(&mut data::compiled::custom_loop::CustomLoopValues, &data::compiled::custom_loop::CustomListOps<'_>, &mut usize) -> CustomLoopResume;
                    6
                ] = [
                    |values, _lists, budget| {
                        let _custom_list0 = values.custom_lists.remove(0);
                        let _bool_function0 = values.bool_functions.remove(0);
                        CustomLoopResume::Exit(custom_loop_bool_2_entry((values.bools[0], _custom_list0, _bool_function0,), values, _lists, budget))
                    },
                    custom_loop_bool_2_resume_1,
                    custom_loop_bool_2_resume_2,
                    custom_loop_bool_2_resume_3,
                    custom_loop_bool_2_resume_4,
                    custom_loop_bool_2_resume_5,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, _lists, budget) {
                        CustomLoopResume::Next(next) => point = next,
                        CustomLoopResume::Exit(progress) => return progress,
                    }
                }
            }

            fn custom_loop_bool_2_entry(
                inputs: (bool, data::compiled::custom_loop::CustomList, data::compiled::custom_loop::BoolCallback,),
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CustomLoopProgress {
                let (mut b0_v0, mut b0_l0, mut b0_g0,) = inputs;
                'repeat: loop {
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b0_v0]);
                        values.customs.clear();
                        values.customs.extend([]);
                        values.custom_lists.clear();
                        values.custom_lists.extend([b0_l0]);
                        values.int_functions.clear();
                        values.int_functions.extend([]);
                        values.bool_functions.clear();
                        values.bool_functions.extend([b0_g0]);
                        return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(0));
                    }
                    *budget -= 1;
                    if b0_l0.is_empty() {
                        let _next = (b0_v0,);
                        drop(b0_l0);
                        drop(b0_g0);
                        let (b1_v0,) = _next;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b1_v0]);
                            values.customs.clear();
                            values.customs.extend([]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([]);
                            values.int_functions.clear();
                            values.int_functions.extend([]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([]);
                            return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(1));
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0]);
                        values.customs.clear();
                        values.customs.extend([]);
                        values.custom_lists.clear();
                        values.custom_lists.extend([]);
                        values.int_functions.clear();
                        values.int_functions.extend([]);
                        values.bool_functions.clear();
                        values.bool_functions.extend([]);
                        return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)));
                    } else {
                        let (b2_v0, b2_l0, b2_g0,) = (b0_v0, b0_l0, b0_g0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b2_v0]);
                            values.customs.clear();
                            values.customs.extend([]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([b2_l0]);
                            values.int_functions.clear();
                            values.int_functions.extend([]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([b2_g0]);
                            return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(2));
                        }
                        let b2_c0 = match _lists.index(&b2_l0, 0) {
                            Some(value) => value,
                            None => {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[b2_v0]);
                                values.customs.clear();
                                values.customs.extend([]);
                                values.custom_lists.clear();
                                values.custom_lists.extend([b2_l0]);
                                values.int_functions.clear();
                                values.int_functions.extend([]);
                                values.bool_functions.clear();
                                values.bool_functions.extend([b2_g0]);
                                return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Interpreted(2));
                            }
                        };
                        *budget -= 1;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b2_v0]);
                            values.customs.clear();
                            values.customs.extend([b2_c0]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([b2_l0]);
                            values.int_functions.clear();
                            values.int_functions.extend([]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([b2_g0]);
                            return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(3));
                        }
                        *budget -= 1;
                        let b2_l1 = _lists.tail(&b2_l0, data::type_::CustomListTypeId {
                            list_type: data::type_::ListTypeId(0),
                            item_type: data::type_::CustomTypeId(0),
                        }, 1);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b2_v0]);
                            values.customs.clear();
                            values.customs.extend([b2_c0]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([b2_l0, b2_l1]);
                            values.int_functions.clear();
                            values.int_functions.extend([]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([b2_g0]);
                            return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(4));
                        }
                        *budget -= 1;
                        let b2_v1 = match b2_g0.call(
                            &data::compiled::custom_loop::CallbackArguments {
                                ints: &[],
                                bools: &[b2_v0],
                                customs: &[&b2_c0],
                            },
                            &mut values.callee, budget,
                            ) {
                            data::compiled::custom_loop::CallbackProgress::Complete(value) => value,
                            data::compiled::custom_loop::CallbackProgress::Stopped(progress) => {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[b2_v0]);
                                values.customs.clear();
                                values.customs.extend([b2_c0]);
                                values.custom_lists.clear();
                                values.custom_lists.extend([b2_l0, b2_l1]);
                                values.int_functions.clear();
                                values.int_functions.extend([]);
                                values.bool_functions.clear();
                                values.bool_functions.extend([b2_g0]);
                                return data::compiled::custom_loop::CustomLoopProgress::Call { point: 4, progress };
                            }
                        };
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b2_v0, b2_v1]);
                            values.customs.clear();
                            values.customs.extend([b2_c0]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([b2_l0, b2_l1]);
                            values.int_functions.clear();
                            values.int_functions.extend([]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([b2_g0]);
                            return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(5));
                        }
                        *budget -= 1;
                        {
                            let _next = (b2_v1, b2_l1, b2_g0,);
                            drop(b2_c0);
                            drop(b2_l0);
                            (b0_v0, b0_l0, b0_g0,) = _next;
                            continue 'repeat;
                        }
                    }
                }
            }

            fn custom_loop_bool_2_resume_1(
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> CustomLoopResume {
                let (b1_v0,) = (values.bools[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0]);
                    values.customs.clear();
                    values.customs.extend([]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([]);
                    values.int_functions.clear();
                    values.int_functions.extend([]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(1)));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b1_v0]);
                values.customs.clear();
                values.customs.extend([]);
                values.custom_lists.clear();
                values.custom_lists.extend([]);
                values.int_functions.clear();
                values.int_functions.extend([]);
                values.bool_functions.clear();
                values.bool_functions.extend([]);
                CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))))
            }

            fn custom_loop_bool_2_resume_2(
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> CustomLoopResume {
                let _custom_list0 = values.custom_lists.remove(0);
                let _bool_function0 = values.bool_functions.remove(0);
                let (b2_v0, b2_l0, b2_g0,) = (values.bools[0], _custom_list0, _bool_function0,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b2_v0]);
                    values.customs.clear();
                    values.customs.extend([]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b2_l0]);
                    values.int_functions.clear();
                    values.int_functions.extend([]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([b2_g0]);
                    return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(2)));
                }
                let b2_c0 = match _lists.index(&b2_l0, 0) {
                    Some(value) => value,
                    None => {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b2_v0]);
                        values.customs.clear();
                        values.customs.extend([]);
                        values.custom_lists.clear();
                        values.custom_lists.extend([b2_l0]);
                        values.int_functions.clear();
                        values.int_functions.extend([]);
                        values.bool_functions.clear();
                        values.bool_functions.extend([b2_g0]);
                        return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Interpreted(2)));
                    }
                };
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b2_v0]);
                values.customs.clear();
                values.customs.extend([b2_c0]);
                values.custom_lists.clear();
                values.custom_lists.extend([b2_l0]);
                values.int_functions.clear();
                values.int_functions.extend([]);
                values.bool_functions.clear();
                values.bool_functions.extend([b2_g0]);
                CustomLoopResume::Next(3)
            }

            fn custom_loop_bool_2_resume_3(
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> CustomLoopResume {
                let _custom0 = values.customs.remove(0);
                let _custom_list0 = values.custom_lists.remove(0);
                let _bool_function0 = values.bool_functions.remove(0);
                let (b2_v0, b2_c0, b2_l0, b2_g0,) = (values.bools[0], _custom0, _custom_list0, _bool_function0,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b2_v0]);
                    values.customs.clear();
                    values.customs.extend([b2_c0]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b2_l0]);
                    values.int_functions.clear();
                    values.int_functions.extend([]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([b2_g0]);
                    return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(3)));
                }
                *budget -= 1;
                let b2_l1 = _lists.tail(&b2_l0, data::type_::CustomListTypeId {
                    list_type: data::type_::ListTypeId(0),
                    item_type: data::type_::CustomTypeId(0),
                }, 1);

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b2_v0]);
                values.customs.clear();
                values.customs.extend([b2_c0]);
                values.custom_lists.clear();
                values.custom_lists.extend([b2_l0, b2_l1]);
                values.int_functions.clear();
                values.int_functions.extend([]);
                values.bool_functions.clear();
                values.bool_functions.extend([b2_g0]);
                CustomLoopResume::Next(4)
            }

            fn custom_loop_bool_2_resume_4(
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> CustomLoopResume {
                let _custom0 = values.customs.remove(0);
                let _custom_list1 = values.custom_lists.remove(1);
                let _custom_list0 = values.custom_lists.remove(0);
                let _bool_function0 = values.bool_functions.remove(0);
                let (b2_v0, b2_c0, b2_l0, b2_l1, b2_g0,) = (values.bools[0], _custom0, _custom_list0, _custom_list1, _bool_function0,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b2_v0]);
                    values.customs.clear();
                    values.customs.extend([b2_c0]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b2_l0, b2_l1]);
                    values.int_functions.clear();
                    values.int_functions.extend([]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([b2_g0]);
                    return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(4)));
                }
                *budget -= 1;
                let b2_v1 = match b2_g0.call(
                    &data::compiled::custom_loop::CallbackArguments {
                        ints: &[],
                        bools: &[b2_v0],
                        customs: &[&b2_c0],
                    },
                    &mut values.callee, budget,
                    ) {
                    data::compiled::custom_loop::CallbackProgress::Complete(value) => value,
                    data::compiled::custom_loop::CallbackProgress::Stopped(progress) => {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b2_v0]);
                        values.customs.clear();
                        values.customs.extend([b2_c0]);
                        values.custom_lists.clear();
                        values.custom_lists.extend([b2_l0, b2_l1]);
                        values.int_functions.clear();
                        values.int_functions.extend([]);
                        values.bool_functions.clear();
                        values.bool_functions.extend([b2_g0]);
                        return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Call { point: 4, progress });
                    }
                };

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b2_v0, b2_v1]);
                values.customs.clear();
                values.customs.extend([b2_c0]);
                values.custom_lists.clear();
                values.custom_lists.extend([b2_l0, b2_l1]);
                values.int_functions.clear();
                values.int_functions.extend([]);
                values.bool_functions.clear();
                values.bool_functions.extend([b2_g0]);
                CustomLoopResume::Next(5)
            }

            fn custom_loop_bool_2_resume_5(
                values: &mut data::compiled::custom_loop::CustomLoopValues,
                _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                budget: &mut usize,
            ) -> CustomLoopResume {
                let _custom0 = values.customs.remove(0);
                let _custom_list1 = values.custom_lists.remove(1);
                let _custom_list0 = values.custom_lists.remove(0);
                let _bool_function0 = values.bool_functions.remove(0);
                let (b2_v0, b2_v1, b2_c0, b2_l0, b2_l1, b2_g0,) = (values.bools[0], values.bools[1], _custom0, _custom_list0, _custom_list1, _bool_function0,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b2_v0, b2_v1]);
                    values.customs.clear();
                    values.customs.extend([b2_c0]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b2_l0, b2_l1]);
                    values.int_functions.clear();
                    values.int_functions.extend([]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([b2_g0]);
                    return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(5)));
                }
                *budget -= 1;
                {
                    let _next = (b2_v1, b2_l1, b2_g0,);
                    drop(b2_c0);
                    drop(b2_l0);
                    let (b0_v0, b0_l0, b0_g0,) = _next;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);
                    values.customs.clear();
                    values.customs.extend([]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b0_l0]);
                    values.int_functions.clear();
                    values.int_functions.extend([]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([b0_g0]);
                    CustomLoopResume::Next(0)
                }
            }
            data::compiled::CompiledFunctions {
                ints: data::Storage::Static(&[
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(13),
                        implementation: data::compiled::CompiledImplementation::CustomLoop(data::Storage::Static(&data::compiled::CustomLoopImplementation {
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
                                    custom_lists: 1,
                                    int_functions: 1,
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
                                    custom_lists: 1,
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
                                    customs: 1,
                                    custom_lists: 1,
                                    int_functions: 1,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 2,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 2,
                                    int_functions: 1,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 3,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 2,
                                    int_functions: 1,
                                    bool_functions: 0,
                                },
                            ]),
                            calls: data::Storage::Static(&[
                                data::compiled::CompiledLoopCall {
                                    point: 4,
                                    function: data::compiled::CompiledLoopFunction::Int {
                                        local: data::graph::IntFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(3),
                                            },
                                        }),
                                    ]),
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                },
                            ]),
                            run: custom_loop_int_13,
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(20),
                        implementation: data::compiled::CompiledImplementation::CustomLoop(data::Storage::Static(&data::compiled::CustomLoopImplementation {
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
                                    custom_lists: 1,
                                    int_functions: 1,
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
                                    custom_lists: 1,
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
                                    customs: 1,
                                    custom_lists: 1,
                                    int_functions: 1,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 2,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 2,
                                    int_functions: 1,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 3,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 2,
                                    int_functions: 1,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 4,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 2,
                                    int_functions: 1,
                                    bool_functions: 0,
                                },
                            ]),
                            calls: data::Storage::Static(&[
                                data::compiled::CompiledLoopCall {
                                    point: 5,
                                    function: data::compiled::CompiledLoopFunction::Int {
                                        local: data::graph::IntFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(3),
                                            },
                                        }),
                                    ]),
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                },
                            ]),
                            run: custom_loop_int_20,
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(22),
                        implementation: data::compiled::CompiledImplementation::CustomLoop(data::Storage::Static(&data::compiled::CustomLoopImplementation {
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
                                    custom_lists: 1,
                                    int_functions: 1,
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
                                    custom_lists: 1,
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
                                    customs: 1,
                                    custom_lists: 1,
                                    int_functions: 1,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 2,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 2,
                                    int_functions: 1,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 3,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 2,
                                    int_functions: 1,
                                    bool_functions: 0,
                                },
                            ]),
                            calls: data::Storage::Static(&[
                                data::compiled::CompiledLoopCall {
                                    point: 4,
                                    function: data::compiled::CompiledLoopFunction::Int {
                                        local: data::graph::IntFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                                data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(1),
                                                shape_id: data::type_::CustomValueShapeId(5),
                                            },
                                        }),
                                    ]),
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                },
                            ]),
                            run: custom_loop_int_22,
                        })),
                    },
                ]),
                bools: data::Storage::Static(&[
                    data::compiled::CompiledFunction {
                        function: data::function::BoolFunctionId(2),
                        implementation: data::compiled::CompiledImplementation::CustomLoop(data::Storage::Static(&data::compiled::CustomLoopImplementation {
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
                                    custom_lists: 1,
                                    int_functions: 0,
                                    bool_functions: 1,
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
                                    block: data::graph::BlockId(2),
                                    instruction: 0,
                                    ints: 0,
                                    bools: 1,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 1,
                                    int_functions: 0,
                                    bool_functions: 1,
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
                                    custom_lists: 1,
                                    int_functions: 0,
                                    bool_functions: 1,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 2,
                                    ints: 0,
                                    bools: 1,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 2,
                                    int_functions: 0,
                                    bool_functions: 1,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 3,
                                    ints: 0,
                                    bools: 2,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 1,
                                    custom_lists: 2,
                                    int_functions: 0,
                                    bool_functions: 1,
                                },
                            ]),
                            calls: data::Storage::Static(&[
                                data::compiled::CompiledLoopCall {
                                    point: 4,
                                    function: data::compiled::CompiledLoopFunction::Bool {
                                        local: data::graph::BoolFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Bool,
                                                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                        },
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(3),
                                            },
                                        }),
                                    ]),
                                    output: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                },
                            ]),
                            run: custom_loop_bool_2,
                        })),
                    },
                ]),
                customs: data::Storage::Static(&[
                ]),
                int_lists: data::Storage::Static(&[
                ]),
                callbacks: data::compiled::CompiledCallbacks {
                    ints: data::Storage::Static(&[
                        data::compiled::CompiledCallback {
                            function: data::function::IntFunctionId(12),
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
                            returns: data::Storage::Static(&[
                                data::graph::IntLocalId(2),
                                data::graph::IntLocalId(2),
                                data::graph::IntLocalId(0),
                            ]),
                            run: callback_int_12_call,
                        },
                        data::compiled::CompiledCallback {
                            function: data::function::IntFunctionId(14),
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
                            returns: data::Storage::Static(&[
                                data::graph::IntLocalId(2),
                                data::graph::IntLocalId(2),
                                data::graph::IntLocalId(0),
                            ]),
                            run: callback_int_14_call,
                        },
                        data::compiled::CompiledCallback {
                            function: data::function::IntFunctionId(15),
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
                                    block: data::graph::BlockId(4),
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
                            returns: data::Storage::Static(&[
                                data::graph::IntLocalId(3),
                                data::graph::IntLocalId(3),
                                data::graph::IntLocalId(2),
                                data::graph::IntLocalId(0),
                            ]),
                            run: callback_int_15_call,
                        },
                        data::compiled::CompiledCallback {
                            function: data::function::IntFunctionId(16),
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
                            returns: data::Storage::Static(&[
                                data::graph::IntLocalId(2),
                            ]),
                            run: callback_int_16_call,
                        },
                        data::compiled::CompiledCallback {
                            function: data::function::IntFunctionId(17),
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
                            returns: data::Storage::Static(&[
                                data::graph::IntLocalId(0),
                            ]),
                            run: callback_int_17_call,
                        },
                        data::compiled::CompiledCallback {
                            function: data::function::IntFunctionId(19),
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
                                    customs: 2,
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
                                    customs: 2,
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
                                    block: data::graph::BlockId(1),
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
                            returns: data::Storage::Static(&[
                                data::graph::IntLocalId(3),
                                data::graph::IntLocalId(2),
                                data::graph::IntLocalId(0),
                            ]),
                            run: callback_int_19_call,
                        },
                        data::compiled::CompiledCallback {
                            function: data::function::IntFunctionId(21),
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
                                    customs: 1,
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
                                    customs: 1,
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
                                    customs: 1,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
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
                            returns: data::Storage::Static(&[
                                data::graph::IntLocalId(3),
                                data::graph::IntLocalId(0),
                            ]),
                            run: callback_int_21_call,
                        },
                    ]),
                    bools: data::Storage::Static(&[
                        data::compiled::CompiledCallback {
                            function: data::function::BoolFunctionId(1),
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
                                    customs: 1,
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
                            returns: data::Storage::Static(&[
                                data::graph::BoolLocalId(0),
                                data::graph::BoolLocalId(0),
                                data::graph::BoolLocalId(0),
                            ]),
                            run: callback_bool_1_call,
                        },
                    ]),
                },
                native_loops: data::Storage::Static(&[
                ]),
                function_calls: data::Storage::Static(&[
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
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
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
                                                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
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
                                                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                    },
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
                                                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
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
                                                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                    },
                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(14)),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                            data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    },
                                    reference: true,
                                    captures: data::Storage::Static(&[]),
                                },
                                data::compiled::CreationContract {
                                    point: 5,
                                    output: data::graph::ParamLocal::IntFunction {
                                        local: data::graph::IntFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                    },
                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(12)),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                            data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    },
                                    reference: true,
                                    captures: data::Storage::Static(&[]),
                                },
                            ]),
                            returns: data::Storage::Static(&[]),
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
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[]),
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
                            returns: data::Storage::Static(&[]),
                            tails: data::Storage::Static(&[]),
                            start: calls_int_3_start,
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
                                    bools: 0,
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
                            returns: data::Storage::Static(&[]),
                            tails: data::Storage::Static(&[]),
                            start: calls_int_9_start,
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
                                    ints: 8,
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
                                    instruction: 11,
                                    ints: 9,
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
                                    instruction: 12,
                                    ints: 10,
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
                                    instruction: 13,
                                    ints: 11,
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
                                    instruction: 14,
                                    ints: 12,
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
                                    instruction: 15,
                                    ints: 13,
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
                                    instruction: 16,
                                    ints: 14,
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
                                    instruction: 17,
                                    ints: 15,
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
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
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
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
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
                                    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3876, 3885)),
                                },
                                data::compiled::CallContract {
                                    point: 5,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(1))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3888, 3903)),
                                },
                                data::compiled::CallContract {
                                    point: 10,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(2))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3906, 3926)),
                                },
                                data::compiled::CallContract {
                                    point: 12,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(4))),
                                    args: data::Storage::Static(&[]),
                                    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3929, 3936)),
                                },
                                data::compiled::CallContract {
                                    point: 15,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(6))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3939, 3953)),
                                },
                            ]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 17,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: calls_int_11_start,
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(23)),
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
                            start: calls_int_23_start,
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
                0..24,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                24..27,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                27..28,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
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
                        data::type_::ValueShapeId(7),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 4..7,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(7),
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
                    parameters: 8..8,
                    parameter_shapes: data::Storage::Static(&[]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 8..9,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 9..10,
                    parameter_shapes: data::Storage::Static(&[
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
                    parameters: 16..19,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(7),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 19..19,
                    parameter_shapes: data::Storage::Static(&[]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 19..21,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(4),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 21..24,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(5),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(6),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 24..26,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(4),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 26..28,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(4),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[
                        data::graph::ParamSlot {
                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                            shape: data::type_::ValueShapeId(7),
                        },
                        data::graph::ParamSlot {
                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                            shape: data::type_::ValueShapeId(0),
                        },
                    ]),
                },
                data::function::FunctionContract {
                    parameters: 28..30,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(4),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 30..32,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(4),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 32..34,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(4),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 34..36,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(4),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[
                        data::graph::ParamSlot {
                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                id: data::graph::CustomLocalId(1),
                                shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(0),
                                    shape_id: data::type_::CustomValueShapeId(0),
                                },
                            }),
                            shape: data::type_::ValueShapeId(1),
                        },
                    ]),
                },
                data::function::FunctionContract {
                    parameters: 36..39,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(5),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(6),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 39..41,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(11),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 41..44,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(14),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(12),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 44..45,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 45..47,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(7),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(7),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 47..49,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(7),
                        data::type_::ValueShapeId(4),
                    ]),
                    return_: data::type_::ValueShapeId(7),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 49..52,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(5),
                        data::type_::ValueShapeId(7),
                        data::type_::ValueShapeId(8),
                    ]),
                    return_: data::type_::ValueShapeId(7),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 52..55,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(5),
                    ]),
                    return_: data::type_::ValueShapeId(5),
                    captures: data::Storage::Static(&[]),
                },
            ]),
            parameters: data::Storage::Static(&[
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(0),
                        shape_id: data::type_::CustomValueShapeId(3),
                    },
                }),
                data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                    local: data::graph::CustomListLocalId(0),
                    type_id: data::type_::CustomListTypeId {
                        list_type: data::type_::ListTypeId(0),
                        item_type: data::type_::CustomTypeId(0),
                    },
                }),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::IntFunction {
                    local: data::graph::IntFunctionLocalId(0),
                    type_: data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                            data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    },
                },
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(0),
                        shape_id: data::type_::CustomValueShapeId(3),
                    },
                }),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(0),
                        shape_id: data::type_::CustomValueShapeId(3),
                    },
                }),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(0),
                        shape_id: data::type_::CustomValueShapeId(3),
                    },
                }),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(0),
                        shape_id: data::type_::CustomValueShapeId(3),
                    },
                }),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(0),
                        shape_id: data::type_::CustomValueShapeId(3),
                    },
                }),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(0),
                        shape_id: data::type_::CustomValueShapeId(3),
                    },
                }),
                data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                    local: data::graph::CustomListLocalId(0),
                    type_id: data::type_::CustomListTypeId {
                        list_type: data::type_::ListTypeId(0),
                        item_type: data::type_::CustomTypeId(0),
                    },
                }),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::IntFunction {
                    local: data::graph::IntFunctionLocalId(0),
                    type_: data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                            data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    },
                },
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(1),
                        shape_id: data::type_::CustomValueShapeId(5),
                    },
                }),
                data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                    local: data::graph::CustomListLocalId(0),
                    type_id: data::type_::CustomListTypeId {
                        list_type: data::type_::ListTypeId(1),
                        item_type: data::type_::CustomTypeId(1),
                    },
                }),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::IntFunction {
                    local: data::graph::IntFunctionLocalId(0),
                    type_: data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                            data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    },
                },
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                    id: data::graph::CustomLocalId(0),
                    shape: data::type_::CustomValueShape {
                        type_id: data::type_::CustomTypeId(0),
                        shape_id: data::type_::CustomValueShapeId(3),
                    },
                }),
                data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                    local: data::graph::CustomListLocalId(0),
                    type_id: data::type_::CustomListTypeId {
                        list_type: data::type_::ListTypeId(0),
                        item_type: data::type_::CustomTypeId(0),
                    },
                }),
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                data::graph::ParamLocal::BoolFunction {
                    local: data::graph::BoolFunctionLocalId(0),
                    type_: data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Bool,
                            data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                    },
                },
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                    local: data::graph::CustomListLocalId(0),
                    type_id: data::type_::CustomListTypeId {
                        list_type: data::type_::ListTypeId(0),
                        item_type: data::type_::CustomTypeId(0),
                    },
                }),
            ]),
        },
        list_types: data::type_::ListTypeTable {
            types: data::Storage::Static(&[
                data::type_::ListStorageTypeId::Custom(data::type_::CustomListTypeId {
                    list_type: data::type_::ListTypeId(0),
                    item_type: data::type_::CustomTypeId(0),
                }),
                data::type_::ListStorageTypeId::Custom(data::type_::CustomListTypeId {
                    list_type: data::type_::ListTypeId(1),
                    item_type: data::type_::CustomTypeId(1),
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
                        name: data::Text::Static("Item"),
                        arguments: data::Storage::Static(&[]),
                    },
                    constructor_count: 3,
                    constructors: data::Storage::Static(&[
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(0),
                                index: 0,
                            },
                            name: data::Text::Static("Add"),
                            native_tag: data::Text::Static("add"),
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
                            name: data::Text::Static("Subtract"),
                            native_tag: data::Text::Static("subtract"),
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
                            name: data::Text::Static("Skip"),
                            native_tag: data::Text::Static("skip"),
                            fields: data::Storage::Static(&[]),
                        },
                    ]),
                },
                data::type_::CustomTypeDescriptor {
                    type_: data::type_::NominalTypeMetadata {
                        package: data::Text::Static("geam"),
                        module: data::Text::Static("example"),
                        name: data::Text::Static("Decision"),
                        arguments: data::Storage::Static(&[]),
                    },
                    constructor_count: 1,
                    constructors: data::Storage::Static(&[
                        data::type_::CustomConstructorDescriptor {
                            id: data::type_::CustomConstructorId {
                                type_id: data::type_::CustomTypeId(1),
                                index: 0,
                            },
                            name: data::Text::Static("Decision"),
                            native_tag: data::Text::Static("decision"),
                            fields: data::Storage::Static(&[
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::Int,
                                    shape: data::type_::ValueShapeId(0),
                                    refinement: data::type_::FieldRefinement::Value,
                                },
                                data::type_::CustomFieldDescriptor {
                                    label: None,
                                    type_: data::type_::ValueType::Bool,
                                    shape: data::type_::ValueShapeId(7),
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
                    name: data::Text::Static("Decision"),
                    publicity: data::type_::CustomTypePublicity::Public,
                    opaque: false,
                    parameters: 0,
                    constructors: data::Storage::Static(&[
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Decision"),
                            fields: data::Storage::Static(&[
                                data::type_::FieldDefinition {
                                    label: None,
                                    type_: data::type_::TypeMetadata::Int,
                                },
                                data::type_::FieldDefinition {
                                    label: None,
                                    type_: data::type_::TypeMetadata::Bool,
                                },
                            ]),
                        },
                    ]),
                },
                data::type_::CustomDefinition {
                    package: data::Text::Static("geam"),
                    module: data::Text::Static("example"),
                    name: data::Text::Static("Item"),
                    publicity: data::type_::CustomTypePublicity::Public,
                    opaque: false,
                    parameters: 0,
                    constructors: data::Storage::Static(&[
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Add"),
                            fields: data::Storage::Static(&[
                                data::type_::FieldDefinition {
                                    label: None,
                                    type_: data::type_::TypeMetadata::Int,
                                },
                            ]),
                        },
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Subtract"),
                            fields: data::Storage::Static(&[
                                data::type_::FieldDefinition {
                                    label: None,
                                    type_: data::type_::TypeMetadata::Int,
                                },
                            ]),
                        },
                        data::type_::ConstructorDefinition {
                            name: data::Text::Static("Skip"),
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
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(1)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(2)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(3)),
                data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(4)),
                data::type_::ValueShapeDescriptor::Function {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(4),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                },
                data::type_::ValueShapeDescriptor::Bool,
                data::type_::ValueShapeDescriptor::Function {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(7),
                        data::type_::ValueShapeId(4),
                    ]),
                    return_: data::type_::ValueShapeId(7),
                },
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(4)),
                data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(9)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(5)),
                data::type_::ValueShapeDescriptor::Function {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(11),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                },
                data::type_::ValueShapeDescriptor::String,
                data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(11)),
            ]),
            shape_types: data::Storage::Static(&[
                data::type_::ValueType::Int,
                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::Int,
                        data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                }),
                data::type_::ValueType::Bool,
                data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::Bool,
                        data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Bool),
                }),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                data::type_::ValueType::List(data::type_::ListTypeId(1)),
                data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::Int,
                        data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                }),
                data::type_::ValueType::String,
                data::type_::ValueType::List(data::type_::ListTypeId(1)),
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
                    type_id: data::type_::CustomTypeId(0),
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
            name: data::Text::Static("run"),
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
            name: data::Text::Static("chosen"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Bool,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 1,
        },
        data::Export {
            name: data::Text::Static("captured"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Int,
                    data::type_::TypeMetadata::Bool,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 2,
        },
        data::Export {
            name: data::Text::Static("boolean"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Bool,
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            slot: 0,
        },
        data::Export {
            name: data::Text::Static("assertion"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 3,
        },
        data::Export {
            name: data::Text::Static("empty"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 4,
        },
        data::Export {
            name: data::Text::Static("panic_case"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 5,
        },
        data::Export {
            name: data::Text::Static("unsupported"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 6,
        },
        data::Export {
            name: data::Text::Static("custom_capture"),
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
            name: data::Text::Static("repeated"),
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
            name: data::Text::Static("caller_overflow"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            slot: 9,
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
