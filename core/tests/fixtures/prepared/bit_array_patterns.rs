data::ModuleArtifact {
    format: 6,
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
                                                        size: data::graph::BitArrayPatternSize {
                                                            value: data::graph::BitArrayPatternSizeExpr::Local(data::graph::IntLocalId(1)),
                                                            unit: 1,
                                                        },
                                                        endianness: data::graph::Endianness::Big,
                                                        signedness: data::graph::Signedness::Unsigned,
                                                    },
                                                    data::graph::BitArrayPatternSegment::Int {
                                                        pattern: data::graph::BitArrayPatternValue::Bind(data::graph::MatchPatternBinding {
                                                            index: 0,
                                                        }),
                                                        size: data::graph::BitArrayPatternSize {
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
                                    data::graph::ProfiledInstruction {
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
                                    },
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
                                                        pattern: data::graph::BitArrayPatternValue::Bind(data::graph::MatchPatternBinding {
                                                            index: 0,
                                                        }),
                                                        size: data::graph::BitArrayPatternSize {
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
                                                        size: data::graph::BitArrayPatternSize {
                                                            value: data::graph::BitArrayPatternSizeExpr::Local(data::graph::IntLocalId(0)),
                                                            unit: 1,
                                                        },
                                                        endianness: data::graph::Endianness::Little,
                                                    },
                                                    data::graph::BitArrayPatternSegment::Bits {
                                                        pattern: data::graph::BitArrayBindingPattern::Bind(data::graph::MatchPatternBinding {
                                                            index: 2,
                                                        }),
                                                        size: Some(data::graph::BitArrayPatternSize {
                                                            value: data::graph::BitArrayPatternSizeExpr::Local(data::graph::IntLocalId(0)),
                                                            unit: 8,
                                                        }),
                                                        unit: 8,
                                                    },
                                                    data::graph::BitArrayPatternSegment::Int {
                                                        pattern: data::graph::BitArrayPatternValue::Bind(data::graph::MatchPatternBinding {
                                                            index: 3,
                                                        }),
                                                        size: data::graph::BitArrayPatternSize {
                                                            value: data::graph::BitArrayPatternSizeExpr::Value(data::graph::IntegerLiteral {
                                                                sign: data::Sign::Plus,
                                                                digits: data::Storage::Static(&[
                                                                    8,
                                                                ]),
                                                            }),
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
                                    data::graph::ProfiledInstruction {
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
                                    },
                                    data::graph::ProfiledInstruction {
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
                                    },
                                    data::graph::ProfiledInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Float(data::graph::FloatLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Float(data::graph::FloatInstruction::Value(f64::from_bits(13830554455654793216))),
                                    },
                                    data::graph::ProfiledInstruction {
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
                                    },
                                    data::graph::ProfiledInstruction {
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
                                    },
                                    data::graph::ProfiledInstruction {
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
                                    },
                                    data::graph::ProfiledInstruction {
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
                                    },
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
                0..1,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
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
                    parameters: 3..5,
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
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
            ]),
        },
        list_types: data::type_::ListTypeTable {
            types: data::Storage::Static(&[]),
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
    ]),
}
