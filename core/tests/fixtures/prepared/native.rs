data::HostedModuleArtifact {
    module: data::ModuleArtifact {
        format: 2,
        program: data::ProgramTables {
            root: data::source::module_id(0),
            modules: data::Storage::Static(&[
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("main"),
                    source_context: Some(data::source::SourceContext::from_static("src/main.gleam", "pub type Tree(a) { Leaf(a) Branch(List(Tree(a))) }\n\n@external(erlang, \"native\", \"equal_native\")\nfn equal_native(value: a, target: b) -> Bool\n\n@external(erlang, \"native\", \"fold\")\nfn fold(callback: fn(Int) -> Int, initial: Int) -> Int\n\npub fn run() {\n  let source = Branch([Leaf(<<\"one\":utf8>>), Branch([Leaf(<<\"two\":utf8>>)])])\n  let expected = Branch([Leaf(\"one\"), Branch([Leaf(\"two\")])])\n  #(\n    equal_native(source, expected),\n    equal_native(#(<<\"one\":utf8>>, [<<\"two\":utf8>>]), #(\"one\", [\"two\"])),\n    fold(fn(value) { value + 1 }, 40),\n  )\n}\n\npub fn substring(value: String) {\n  let assert \"prefix:\" <> rest = value\n  let read = fn() { rest }\n  #(equal_native(#(rest, [rest]), #(read(), [read()])), read())\n}\n")),
                },
            ]),
            main: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Tuple {
                id: data::function::TupleFunctionId(0),
                return_type: data::Storage::Static(&[
                    data::type_::ValueType::Bool,
                    data::type_::ValueType::Bool,
                    data::type_::ValueType::Int,
                ]),
            }),
            functions: data::function::FunctionTables {
                value_returns: data::function::ValueFunctionTables {
                    never_functions: data::Storage::Static(&[]),
                    int_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
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
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    1,
                                                ]),
                                            })),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntLocalId(0),
                                                right: data::graph::IntLocalId(1),
                                            }),
                                        },
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(2)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 2,
                            return_: data::graph::IntLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    float_functions: data::Storage::Static(&[]),
                    string_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 0,
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
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::StringLocalId(0)),
                                ]),
                            },
                        })),
                    ]),
                    bit_array_functions: data::Storage::Static(&[]),
                    utf_codepoint_functions: data::Storage::Static(&[]),
                    custom_functions: data::Storage::Static(&[]),
                    external_functions: data::Storage::Static(&[]),
                    bool_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 0,
                            return_: data::graph::BoolLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 1,
                            return_: data::graph::BoolLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 3,
                            return_: data::graph::BoolLocalId(0),
                            body: ::core::marker::PhantomData,
                        })),
                    ]),
                    nil_functions: data::Storage::Static(&[]),
                    tuple_functions: data::Storage::Static(&[
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
                                            instructions: 0..34,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("one"))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::BitArray(data::graph::BitArrayInstruction::Value(data::Storage::Static(&[
                                                data::graph::BitArraySegment::String {
                                                    value: data::graph::StringLocalId(0),
                                                    encoding: data::graph::StringEncoding::Utf8,
                                                },
                                            ]))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(2),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                ]),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("two"))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::BitArray(data::graph::BitArrayInstruction::Value(data::Storage::Static(&[
                                                data::graph::BitArraySegment::String {
                                                    value: data::graph::StringLocalId(1),
                                                    encoding: data::graph::StringEncoding::Utf8,
                                                },
                                            ]))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(2),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 0,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                                ]),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                    local: data::graph::CustomListLocalId(0),
                                                    type_id: data::type_::CustomListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item_type: data::type_::CustomTypeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(7),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                                item_type: data::type_::CustomTypeId(0),
                                            }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                                data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(2),
                                                    },
                                                },
                                            ])))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(2),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(3),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(8),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                        local: data::graph::CustomListLocalId(0),
                                                        type_id: data::type_::CustomListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::CustomTypeId(0),
                                                        },
                                                    }),
                                                ]),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                    local: data::graph::CustomListLocalId(1),
                                                    type_id: data::type_::CustomListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item_type: data::type_::CustomTypeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(2),
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
                                                    id: data::graph::CustomLocalId(2),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(3),
                                                    },
                                                },
                                            ])))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(3),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(3),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(8),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                        local: data::graph::CustomListLocalId(1),
                                                        type_id: data::type_::CustomListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::CustomTypeId(0),
                                                        },
                                                    }),
                                                ]),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("one"))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(4),
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
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                ]),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(3)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("two"))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(5),
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
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(3)),
                                                ]),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                    local: data::graph::CustomListLocalId(2),
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
                                                    id: data::graph::CustomLocalId(5),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(4),
                                                    },
                                                },
                                            ])))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(6),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(5),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(11),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                        local: data::graph::CustomListLocalId(2),
                                                        type_id: data::type_::CustomListTypeId {
                                                            list_type: data::type_::ListTypeId(1),
                                                            item_type: data::type_::CustomTypeId(1),
                                                        },
                                                    }),
                                                ]),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                    local: data::graph::CustomListLocalId(3),
                                                    type_id: data::type_::CustomListTypeId {
                                                        list_type: data::type_::ListTypeId(1),
                                                        item_type: data::type_::CustomTypeId(1),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(5),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(1),
                                                item_type: data::type_::CustomTypeId(1),
                                            }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                                data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(4),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(4),
                                                    },
                                                },
                                                data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(6),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(5),
                                                    },
                                                },
                                            ])))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(7),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(5),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(11),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                constructor: data::type_::CustomConstructorId {
                                                    type_id: data::type_::CustomTypeId(1),
                                                    index: 1,
                                                },
                                                fields: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                        local: data::graph::CustomListLocalId(3),
                                                        type_id: data::type_::CustomListTypeId {
                                                            list_type: data::type_::ListTypeId(1),
                                                            item_type: data::type_::CustomTypeId(1),
                                                        },
                                                    }),
                                                ]),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(12),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                                function: data::function::BoolFunctionId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(3),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(3),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(7),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(1),
                                                            shape_id: data::type_::CustomValueShapeId(5),
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("main", "run", data::source::SourceSpan::new(398, 428)),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(4)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("one"))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::BitArray(data::graph::BitArrayInstruction::Value(data::Storage::Static(&[
                                                data::graph::BitArraySegment::String {
                                                    value: data::graph::StringLocalId(4),
                                                    encoding: data::graph::StringEncoding::Utf8,
                                                },
                                            ]))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(5)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("two"))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(3)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::BitArray(data::graph::BitArrayInstruction::Value(data::Storage::Static(&[
                                                data::graph::BitArraySegment::String {
                                                    value: data::graph::StringLocalId(5),
                                                    encoding: data::graph::StringEncoding::Utf8,
                                                },
                                            ]))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::BitArray {
                                                    local: data::graph::BitArrayListLocalId(0),
                                                    type_id: data::type_::BitArrayListTypeId {
                                                        list_type: data::type_::ListTypeId(2),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(13),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::BitArray(data::type_::BitArrayListTypeId {
                                                list_type: data::type_::ListTypeId(2),
                                            }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                                data::graph::BitArrayLocalId(3),
                                            ])))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::BitArray,
                                                        data::type_::ValueType::List(data::type_::ListTypeId(2)),
                                                    ]),
                                                },
                                                shape: data::type_::ValueShapeId(14),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                                data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(2)),
                                                data::graph::ParamLocal::List(data::graph::ListLocal::BitArray {
                                                    local: data::graph::BitArrayListLocalId(0),
                                                    type_id: data::type_::BitArrayListTypeId {
                                                        list_type: data::type_::ListTypeId(2),
                                                    },
                                                }),
                                            ]))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(6)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("one"))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(7)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("two"))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                    local: data::graph::StringListLocalId(0),
                                                    type_id: data::type_::StringListTypeId {
                                                        list_type: data::type_::ListTypeId(3),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(15),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::String(data::type_::StringListTypeId {
                                                list_type: data::type_::ListTypeId(3),
                                            }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                                data::graph::StringLocalId(7),
                                            ])))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(1),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                        data::type_::ValueType::List(data::type_::ListTypeId(3)),
                                                    ]),
                                                },
                                                shape: data::type_::ValueShapeId(16),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(6)),
                                                data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                    local: data::graph::StringListLocalId(0),
                                                    type_id: data::type_::StringListTypeId {
                                                        list_type: data::type_::ListTypeId(3),
                                                    },
                                                }),
                                            ]))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                shape: data::type_::ValueShapeId(12),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                                function: data::function::BoolFunctionId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Tuple {
                                                        local: data::graph::TupleLocalId(0),
                                                        type_: data::Storage::Static(&[
                                                            data::type_::ValueType::BitArray,
                                                            data::type_::ValueType::List(data::type_::ListTypeId(2)),
                                                        ]),
                                                    },
                                                    data::graph::ParamLocal::Tuple {
                                                        local: data::graph::TupleLocalId(1),
                                                        type_: data::Storage::Static(&[
                                                            data::type_::ValueType::String,
                                                            data::type_::ValueType::List(data::type_::ListTypeId(3)),
                                                        ]),
                                                    },
                                                ]),
                                                site: data::source::HostCallSite::from_static("main", "run", data::source::SourceSpan::new(434, 502)),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
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
                                                shape: data::type_::ValueShapeId(18),
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
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(0)),
                                                    captures: data::Storage::Static(&[]),
                                                },
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    40,
                                                ]),
                                            })),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(1),
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
                                                site: data::source::HostCallSite::from_static("main", "run", data::source::SourceSpan::new(508, 541)),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(2),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                },
                                                shape: data::type_::ValueShapeId(19),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            ]))),
                                        },
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::TupleLocalId(2)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
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
                                                subject: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                pattern: data::graph::MatchPattern::StringPrefix {
                                                    prefix: data::Text::Static("prefix:"),
                                                    left: None,
                                                    right: Some(data::graph::MatchPatternBinding {
                                                        index: 0,
                                                    }),
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
                                                                family: data::graph::StorageFamily::String,
                                                                positions: data::Storage::Static(&[
                                                                    1,
                                                                ]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                positions: data::Storage::Static(&[
                                                                    0,
                                                                ]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 1..2,
                                            instructions: 0..10,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..3,
                                            instructions: 10..10,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "substring", data::source::SourceSpan::new(586, 596)),
                                                pattern_span: data::source::SourceSpan::new(597, 614),
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::StringFunction {
                                                    local: data::graph::StringFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::String),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(20),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::String),
                                                },
                                                family: data::function::FunctionReturnFamily::String,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::String(data::function::StringFunctionId(0)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::String {
                                                            target: data::graph::StringLocalId(0),
                                                            source: data::graph::StringLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                    local: data::graph::StringListLocalId(0),
                                                    type_id: data::type_::StringListTypeId {
                                                        list_type: data::type_::ListTypeId(3),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(15),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::String(data::type_::StringListTypeId {
                                                list_type: data::type_::ListTypeId(3),
                                            }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                                data::graph::StringLocalId(0),
                                            ])))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                        data::type_::ValueType::List(data::type_::ListTypeId(3)),
                                                    ]),
                                                },
                                                shape: data::type_::ValueShapeId(16),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                    local: data::graph::StringListLocalId(0),
                                                    type_id: data::type_::StringListTypeId {
                                                        list_type: data::type_::ListTypeId(3),
                                                    },
                                                }),
                                            ]))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::FunctionCall {
                                                function: data::graph::StringFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("main", "substring", data::source::SourceSpan::new(686, 692)),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::FunctionCall {
                                                function: data::graph::StringFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("main", "substring", data::source::SourceSpan::new(695, 701)),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                    local: data::graph::StringListLocalId(1),
                                                    type_id: data::type_::StringListTypeId {
                                                        list_type: data::type_::ListTypeId(3),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(15),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::String(data::type_::StringListTypeId {
                                                list_type: data::type_::ListTypeId(3),
                                            }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                                data::graph::StringLocalId(2),
                                            ])))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(1),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                        data::type_::ValueType::List(data::type_::ListTypeId(3)),
                                                    ]),
                                                },
                                                shape: data::type_::ValueShapeId(16),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                    local: data::graph::StringListLocalId(1),
                                                    type_id: data::type_::StringListTypeId {
                                                        list_type: data::type_::ListTypeId(3),
                                                    },
                                                }),
                                            ]))),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(12),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                                function: data::function::BoolFunctionId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Tuple {
                                                        local: data::graph::TupleLocalId(0),
                                                        type_: data::Storage::Static(&[
                                                            data::type_::ValueType::String,
                                                            data::type_::ValueType::List(data::type_::ListTypeId(3)),
                                                        ]),
                                                    },
                                                    data::graph::ParamLocal::Tuple {
                                                        local: data::graph::TupleLocalId(1),
                                                        type_: data::Storage::Static(&[
                                                            data::type_::ValueType::String,
                                                            data::type_::ValueType::List(data::type_::ListTypeId(3)),
                                                        ]),
                                                    },
                                                ]),
                                                site: data::source::HostCallSite::from_static("main", "substring", data::source::SourceSpan::new(654, 704)),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(3)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::FunctionCall {
                                                function: data::graph::StringFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("main", "substring", data::source::SourceSpan::new(706, 712)),
                                            }),
                                        },
                                        data::graph::ProfiledInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(2),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::String,
                                                    ]),
                                                },
                                                shape: data::type_::ValueShapeId(21),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(3)),
                                            ]))),
                                        },
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::TupleLocalId(2)),
                                ]),
                            },
                        })),
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
                    0..2,
                    0..0,
                    2..3,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    3..6,
                    0..0,
                    6..8,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
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
                            data::type_::ValueShapeId(17),
                        ]),
                        return_: data::type_::ValueShapeId(17),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 1..3,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(18),
                            data::type_::ValueShapeId(17),
                        ]),
                        return_: data::type_::ValueShapeId(17),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 3..3,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(3),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                shape: data::type_::ValueShapeId(3),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 3..5,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(8),
                            data::type_::ValueShapeId(11),
                        ]),
                        return_: data::type_::ValueShapeId(12),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 5..7,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(14),
                            data::type_::ValueShapeId(16),
                        ]),
                        return_: data::type_::ValueShapeId(12),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 7..9,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(16),
                            data::type_::ValueShapeId(16),
                        ]),
                        return_: data::type_::ValueShapeId(12),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 9..9,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(19),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 9..10,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(3),
                        ]),
                        return_: data::type_::ValueShapeId(21),
                        captures: data::Storage::Static(&[]),
                    },
                ]),
                parameters: data::Storage::Static(&[
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
                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(3),
                        },
                    }),
                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(1),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(1),
                            shape_id: data::type_::CustomValueShapeId(5),
                        },
                    }),
                    data::graph::ParamLocal::Tuple {
                        local: data::graph::TupleLocalId(0),
                        type_: data::Storage::Static(&[
                            data::type_::ValueType::BitArray,
                            data::type_::ValueType::List(data::type_::ListTypeId(2)),
                        ]),
                    },
                    data::graph::ParamLocal::Tuple {
                        local: data::graph::TupleLocalId(1),
                        type_: data::Storage::Static(&[
                            data::type_::ValueType::String,
                            data::type_::ValueType::List(data::type_::ListTypeId(3)),
                        ]),
                    },
                    data::graph::ParamLocal::Tuple {
                        local: data::graph::TupleLocalId(0),
                        type_: data::Storage::Static(&[
                            data::type_::ValueType::String,
                            data::type_::ValueType::List(data::type_::ListTypeId(3)),
                        ]),
                    },
                    data::graph::ParamLocal::Tuple {
                        local: data::graph::TupleLocalId(1),
                        type_: data::Storage::Static(&[
                            data::type_::ValueType::String,
                            data::type_::ValueType::List(data::type_::ListTypeId(3)),
                        ]),
                    },
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
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
                    data::type_::ListStorageTypeId::BitArray(data::type_::BitArrayListTypeId {
                        list_type: data::type_::ListTypeId(2),
                    }),
                    data::type_::ListStorageTypeId::String(data::type_::StringListTypeId {
                        list_type: data::type_::ListTypeId(3),
                    }),
                ]),
                tuple_items: data::Storage::Static(&[]),
                function_items: data::Storage::Static(&[]),
            },
            custom_types: data::type_::CustomTypeTable {
                types: data::Storage::Static(&[
                    data::type_::CustomTypeDescriptor {
                        type_: data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("main"),
                            name: data::Text::Static("Tree"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::BitArray,
                            ]),
                        },
                        constructor_count: 2,
                        constructors: data::Storage::Static(&[
                            data::type_::CustomConstructorDescriptor {
                                id: data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                                name: data::Text::Static("Leaf"),
                                native_tag: data::Text::Static("leaf"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: None,
                                        type_: data::type_::ValueType::BitArray,
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
                                name: data::Text::Static("Branch"),
                                native_tag: data::Text::Static("branch"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: None,
                                        type_: data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                        refinement: data::type_::FieldRefinement::List(data::Storage::Static(&data::type_::FieldRefinement::Custom(data::Storage::Static(&[
                                            data::type_::FieldRefinement::Argument(0),
                                        ])))),
                                    },
                                ]),
                            },
                        ]),
                    },
                    data::type_::CustomTypeDescriptor {
                        type_: data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("main"),
                            name: data::Text::Static("Tree"),
                            arguments: data::Storage::Static(&[
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
                                name: data::Text::Static("Leaf"),
                                native_tag: data::Text::Static("leaf"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: None,
                                        type_: data::type_::ValueType::String,
                                        shape: data::type_::ValueShapeId(3),
                                        refinement: data::type_::FieldRefinement::Argument(0),
                                    },
                                ]),
                            },
                            data::type_::CustomConstructorDescriptor {
                                id: data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(1),
                                    index: 1,
                                },
                                name: data::Text::Static("Branch"),
                                native_tag: data::Text::Static("branch"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: None,
                                        type_: data::type_::ValueType::List(data::type_::ListTypeId(1)),
                                        shape: data::type_::ValueShapeId(5),
                                        refinement: data::type_::FieldRefinement::List(data::Storage::Static(&data::type_::FieldRefinement::Custom(data::Storage::Static(&[
                                            data::type_::FieldRefinement::Argument(0),
                                        ])))),
                                    },
                                ]),
                            },
                        ]),
                    },
                ]),
                definitions: data::Storage::Static(&[
                    data::type_::CustomDefinition {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("main"),
                        name: data::Text::Static("Tree"),
                        publicity: data::type_::CustomTypePublicity::Public,
                        opaque: false,
                        parameters: 1,
                        constructors: data::Storage::Static(&[
                            data::type_::ConstructorDefinition {
                                name: data::Text::Static("Leaf"),
                                fields: data::Storage::Static(&[
                                    data::type_::FieldDefinition {
                                        label: None,
                                        type_: data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                                    },
                                ]),
                            },
                            data::type_::ConstructorDefinition {
                                name: data::Text::Static("Branch"),
                                fields: data::Storage::Static(&[
                                    data::type_::FieldDefinition {
                                        label: None,
                                        type_: data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                            package: data::Text::Static("application"),
                                            module: data::Text::Static("main"),
                                            name: data::Text::Static("Tree"),
                                            arguments: data::Storage::Static(&[
                                                data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                                            ]),
                                        }))),
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
                    data::type_::ValueShapeDescriptor::BitArray,
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(0)),
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(1)),
                    data::type_::ValueShapeDescriptor::String,
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(1)),
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(4)),
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(2)),
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(6)),
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(3)),
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(4)),
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(9)),
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(5)),
                    data::type_::ValueShapeDescriptor::Bool,
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(0)),
                    data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(13),
                    ])),
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(3)),
                    data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                        data::type_::ValueShapeId(3),
                        data::type_::ValueShapeId(15),
                    ])),
                    data::type_::ValueShapeDescriptor::Int,
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(17),
                        ]),
                        return_: data::type_::ValueShapeId(17),
                    },
                    data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                        data::type_::ValueShapeId(12),
                        data::type_::ValueShapeId(12),
                        data::type_::ValueShapeId(17),
                    ])),
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(3),
                    },
                    data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                        data::type_::ValueShapeId(12),
                        data::type_::ValueShapeId(3),
                    ])),
                ]),
                shape_types: data::Storage::Static(&[
                    data::type_::ValueType::BitArray,
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                    data::type_::ValueType::String,
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                    data::type_::ValueType::List(data::type_::ListTypeId(1)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                    data::type_::ValueType::List(data::type_::ListTypeId(1)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                    data::type_::ValueType::Bool,
                    data::type_::ValueType::List(data::type_::ListTypeId(2)),
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::BitArray,
                        data::type_::ValueType::List(data::type_::ListTypeId(2)),
                    ])),
                    data::type_::ValueType::List(data::type_::ListTypeId(3)),
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::String,
                        data::type_::ValueType::List(data::type_::ListTypeId(3)),
                    ])),
                    data::type_::ValueType::Int,
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    }),
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::Bool,
                        data::type_::ValueType::Bool,
                        data::type_::ValueType::Int,
                    ])),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::ValueType::String),
                    }),
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::Bool,
                        data::type_::ValueType::String,
                    ])),
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
                        type_id: data::type_::CustomTypeId(1),
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(3),
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
                        type_id: data::type_::CustomTypeId(0),
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        constructor: data::type_::CustomConstructorRefinement::Exact(1),
                    },
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(1),
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(3),
                        ]),
                        constructor: data::type_::CustomConstructorRefinement::Exact(0),
                    },
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(1),
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(3),
                        ]),
                        constructor: data::type_::CustomConstructorRefinement::Exact(1),
                    },
                ]),
            },
        },
        entries: data::program::LibraryFunctionEntries {
            ints: data::Storage::Static(&[]),
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
                        },
                    },
                },
                data::program::LibraryFunctionEntry {
                    function: data::function::TupleFunctionId(1),
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
                        },
                    },
                },
            ]),
            lists: data::Storage::Static(&[]),
        },
        exports: data::Storage::Static(&[
            data::Export {
                name: data::Text::Static("run"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                        data::type_::TypeMetadata::Bool,
                        data::type_::TypeMetadata::Bool,
                        data::type_::TypeMetadata::Int,
                    ]))),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("substring"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                        data::type_::TypeMetadata::Bool,
                        data::type_::TypeMetadata::String,
                    ]))),
                },
                slot: 1,
            },
        ]),
    },
    value_functions: data::Storage::Static(&[
        data::host::HostedFunctionMetadata {
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("main", "equal_native", data::source::SourceSpan::new(96, 132)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("main"),
                        name: data::Text::Static("Tree"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::BitArray,
                        ]),
                    }),
                    data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("main"),
                        name: data::Text::Static("Tree"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::String,
                        ]),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("main"),
                        name: data::Text::Static("Tree"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::BitArray,
                        ]),
                    }),
                    shape: data::type_::ValueShapeId(8),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("main"),
                        name: data::Text::Static("Tree"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::String,
                        ]),
                    }),
                    shape: data::type_::ValueShapeId(11),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(3),
                        },
                    })),
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(1),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(1),
                            shape_id: data::type_::CustomValueShapeId(5),
                        },
                    })),
                ]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("main"),
                            name: data::Text::Static("Tree"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::String,
                            ]),
                        }))), data::type_::ListTypeId(1)),
                    ]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("main"),
                            name: data::Text::Static("Tree"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::String,
                            ]),
                        }), data::type_::CustomTypeId(1)),
                        (data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("main"),
                            name: data::Text::Static("Tree"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::BitArray,
                            ]),
                        }), data::type_::CustomTypeId(0)),
                    ]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[
                        data::host::NativeConversionId(0),
                        data::host::NativeConversionId(0),
                    ]),
                    nodes: data::Storage::Static(&[
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("main"),
                                name: data::Text::Static("Tree"),
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::String,
                                ]),
                            }),
                            kind: data::host::NativeConversionKind::Custom(data::Storage::Static(&[
                                data::host::NativeConstructor {
                                    constructor: data::type_::CustomConstructorId {
                                        type_id: data::type_::CustomTypeId(1),
                                        index: 0,
                                    },
                                    tag: data::Text::Static("leaf"),
                                    fields: data::Storage::Static(&[
                                        data::host::NativeConversionId(1),
                                    ]),
                                },
                                data::host::NativeConstructor {
                                    constructor: data::type_::CustomConstructorId {
                                        type_id: data::type_::CustomTypeId(1),
                                        index: 1,
                                    },
                                    tag: data::Text::Static("branch"),
                                    fields: data::Storage::Static(&[
                                        data::host::NativeConversionId(2),
                                    ]),
                                },
                            ])),
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::String,
                            kind: data::host::NativeConversionKind::String,
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("main"),
                                name: data::Text::Static("Tree"),
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::String,
                                ]),
                            }))),
                            kind: data::host::NativeConversionKind::List {
                                storage: data::type_::ListTypeId(1),
                                item: data::host::NativeConversionId(0),
                            },
                        },
                    ]),
                },
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Bool),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::Parameter(1),
                ]),
                return_: data::host::RegistrationType::Bool,
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                    data::host::RegistrationParameter::Value(1),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                    data::host::RegistrationType::Custom {
                        schema: data::host::CustomSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("main"),
                            name: data::Text::Static("Tree"),
                            parameter_count: 1,
                            constructors: data::Storage::Static(&[
                                data::host::ConstructorSchema {
                                    name: data::Text::Static("Leaf"),
                                    fields: data::Storage::Static(&[
                                        data::host::FieldSchema {
                                            label: None,
                                            type_: data::host::SchemaType::Parameter(0),
                                        },
                                    ]),
                                },
                                data::host::ConstructorSchema {
                                    name: data::Text::Static("Branch"),
                                    fields: data::Storage::Static(&[
                                        data::host::FieldSchema {
                                            label: None,
                                            type_: data::host::SchemaType::List(data::Storage::Static(&data::host::SchemaType::Custom {
                                                package: data::Text::Static("application"),
                                                module: data::Text::Static("main"),
                                                name: data::Text::Static("Tree"),
                                                arguments: data::Storage::Static(&[
                                                    data::host::SchemaType::Parameter(0),
                                                ]),
                                            })),
                                        },
                                    ]),
                                },
                            ]),
                        },
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::String,
                        ]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[
                    data::host::CustomSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("main"),
                        name: data::Text::Static("Tree"),
                        parameter_count: 1,
                        constructors: data::Storage::Static(&[
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Leaf"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(0),
                                    },
                                ]),
                            },
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Branch"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::List(data::Storage::Static(&data::host::SchemaType::Custom {
                                            package: data::Text::Static("application"),
                                            module: data::Text::Static("main"),
                                            name: data::Text::Static("Tree"),
                                            arguments: data::Storage::Static(&[
                                                data::host::SchemaType::Parameter(0),
                                            ]),
                                        })),
                                    },
                                ]),
                            },
                        ]),
                    },
                ]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: Some(data::Storage::Static(&[])),
            }),
        },
        data::host::HostedFunctionMetadata {
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("main", "equal_native", data::source::SourceSpan::new(96, 132)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                        data::type_::TypeMetadata::BitArray,
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::BitArray)),
                    ])),
                    data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::String)),
                    ])),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                        data::type_::TypeMetadata::BitArray,
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::BitArray)),
                    ])),
                    shape: data::type_::ValueShapeId(14),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::String)),
                    ])),
                    shape: data::type_::ValueShapeId(16),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::Tuple {
                        local: data::graph::TupleLocalId(0),
                        type_: data::Storage::Static(&[
                            data::type_::ValueType::BitArray,
                            data::type_::ValueType::List(data::type_::ListTypeId(2)),
                        ]),
                    }),
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::Tuple {
                        local: data::graph::TupleLocalId(1),
                        type_: data::Storage::Static(&[
                            data::type_::ValueType::String,
                            data::type_::ValueType::List(data::type_::ListTypeId(3)),
                        ]),
                    }),
                ]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("main"),
                            name: data::Text::Static("Tree"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::String,
                            ]),
                        }))), data::type_::ListTypeId(1)),
                    ]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("main"),
                            name: data::Text::Static("Tree"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::String,
                            ]),
                        }), data::type_::CustomTypeId(1)),
                    ]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[
                        data::host::NativeConversionId(0),
                        data::host::NativeConversionId(1),
                    ]),
                    nodes: data::Storage::Static(&[
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                                data::type_::TypeMetadata::String,
                                data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::String)),
                            ])),
                            kind: data::host::NativeConversionKind::Tuple(data::Storage::Static(&[
                                data::host::NativeConversionId(2),
                                data::host::NativeConversionId(3),
                            ])),
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("main"),
                                name: data::Text::Static("Tree"),
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::String,
                                ]),
                            }),
                            kind: data::host::NativeConversionKind::Custom(data::Storage::Static(&[
                                data::host::NativeConstructor {
                                    constructor: data::type_::CustomConstructorId {
                                        type_id: data::type_::CustomTypeId(1),
                                        index: 0,
                                    },
                                    tag: data::Text::Static("leaf"),
                                    fields: data::Storage::Static(&[
                                        data::host::NativeConversionId(2),
                                    ]),
                                },
                                data::host::NativeConstructor {
                                    constructor: data::type_::CustomConstructorId {
                                        type_id: data::type_::CustomTypeId(1),
                                        index: 1,
                                    },
                                    tag: data::Text::Static("branch"),
                                    fields: data::Storage::Static(&[
                                        data::host::NativeConversionId(4),
                                    ]),
                                },
                            ])),
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::String,
                            kind: data::host::NativeConversionKind::String,
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::String)),
                            kind: data::host::NativeConversionKind::List {
                                storage: data::type_::ListTypeId(3),
                                item: data::host::NativeConversionId(2),
                            },
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("main"),
                                name: data::Text::Static("Tree"),
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::String,
                                ]),
                            }))),
                            kind: data::host::NativeConversionKind::List {
                                storage: data::type_::ListTypeId(1),
                                item: data::host::NativeConversionId(1),
                            },
                        },
                    ]),
                },
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::BitArray,
                        data::type_::ValueType::List(data::type_::ListTypeId(2)),
                    ])),
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::String,
                        data::type_::ValueType::List(data::type_::ListTypeId(3)),
                    ])),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Bool),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::Parameter(1),
                ]),
                return_: data::host::RegistrationType::Bool,
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                    data::host::RegistrationParameter::Value(1),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                    data::host::RegistrationType::Custom {
                        schema: data::host::CustomSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("main"),
                            name: data::Text::Static("Tree"),
                            parameter_count: 1,
                            constructors: data::Storage::Static(&[
                                data::host::ConstructorSchema {
                                    name: data::Text::Static("Leaf"),
                                    fields: data::Storage::Static(&[
                                        data::host::FieldSchema {
                                            label: None,
                                            type_: data::host::SchemaType::Parameter(0),
                                        },
                                    ]),
                                },
                                data::host::ConstructorSchema {
                                    name: data::Text::Static("Branch"),
                                    fields: data::Storage::Static(&[
                                        data::host::FieldSchema {
                                            label: None,
                                            type_: data::host::SchemaType::List(data::Storage::Static(&data::host::SchemaType::Custom {
                                                package: data::Text::Static("application"),
                                                module: data::Text::Static("main"),
                                                name: data::Text::Static("Tree"),
                                                arguments: data::Storage::Static(&[
                                                    data::host::SchemaType::Parameter(0),
                                                ]),
                                            })),
                                        },
                                    ]),
                                },
                            ]),
                        },
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::String,
                        ]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[
                    data::host::CustomSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("main"),
                        name: data::Text::Static("Tree"),
                        parameter_count: 1,
                        constructors: data::Storage::Static(&[
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Leaf"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(0),
                                    },
                                ]),
                            },
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Branch"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::List(data::Storage::Static(&data::host::SchemaType::Custom {
                                            package: data::Text::Static("application"),
                                            module: data::Text::Static("main"),
                                            name: data::Text::Static("Tree"),
                                            arguments: data::Storage::Static(&[
                                                data::host::SchemaType::Parameter(0),
                                            ]),
                                        })),
                                    },
                                ]),
                            },
                        ]),
                    },
                ]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: Some(data::Storage::Static(&[])),
            }),
        },
        data::host::HostedFunctionMetadata {
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("main", "fold", data::source::SourceSpan::new(178, 225)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                    }),
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
            },
            type_arguments: data::Storage::Static(&[]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Function {
                        local: data::graph::ParamLocal::IntFunction {
                            local: data::graph::IntFunctionLocalId(0),
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            },
                        },
                        arity: 1,
                    },
                    data::host::HostCallParameter::Int(data::graph::IntLocalId(0)),
                ]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    }),
                    data::type_::ValueType::Int,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Int),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 0,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Function {
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Int,
                        ]),
                        return_: data::Storage::Static(&data::host::RegistrationType::Int),
                    },
                    data::host::RegistrationType::Int,
                ]),
                return_: data::host::RegistrationType::Int,
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Function {
                        slot: 0,
                        arity: 1,
                    },
                    data::host::RegistrationParameter::Int(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
        data::host::HostedFunctionMetadata {
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("main", "equal_native", data::source::SourceSpan::new(96, 132)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::String)),
                    ])),
                    data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::String)),
                    ])),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::String)),
                    ])),
                    shape: data::type_::ValueShapeId(16),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::String)),
                    ])),
                    shape: data::type_::ValueShapeId(16),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::Tuple {
                        local: data::graph::TupleLocalId(0),
                        type_: data::Storage::Static(&[
                            data::type_::ValueType::String,
                            data::type_::ValueType::List(data::type_::ListTypeId(3)),
                        ]),
                    }),
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::Tuple {
                        local: data::graph::TupleLocalId(1),
                        type_: data::Storage::Static(&[
                            data::type_::ValueType::String,
                            data::type_::ValueType::List(data::type_::ListTypeId(3)),
                        ]),
                    }),
                ]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("main"),
                            name: data::Text::Static("Tree"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::String,
                            ]),
                        }))), data::type_::ListTypeId(1)),
                    ]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("main"),
                            name: data::Text::Static("Tree"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::String,
                            ]),
                        }), data::type_::CustomTypeId(1)),
                    ]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[
                        data::host::NativeConversionId(0),
                        data::host::NativeConversionId(1),
                    ]),
                    nodes: data::Storage::Static(&[
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                                data::type_::TypeMetadata::String,
                                data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::String)),
                            ])),
                            kind: data::host::NativeConversionKind::Tuple(data::Storage::Static(&[
                                data::host::NativeConversionId(2),
                                data::host::NativeConversionId(3),
                            ])),
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("main"),
                                name: data::Text::Static("Tree"),
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::String,
                                ]),
                            }),
                            kind: data::host::NativeConversionKind::Custom(data::Storage::Static(&[
                                data::host::NativeConstructor {
                                    constructor: data::type_::CustomConstructorId {
                                        type_id: data::type_::CustomTypeId(1),
                                        index: 0,
                                    },
                                    tag: data::Text::Static("leaf"),
                                    fields: data::Storage::Static(&[
                                        data::host::NativeConversionId(2),
                                    ]),
                                },
                                data::host::NativeConstructor {
                                    constructor: data::type_::CustomConstructorId {
                                        type_id: data::type_::CustomTypeId(1),
                                        index: 1,
                                    },
                                    tag: data::Text::Static("branch"),
                                    fields: data::Storage::Static(&[
                                        data::host::NativeConversionId(4),
                                    ]),
                                },
                            ])),
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::String,
                            kind: data::host::NativeConversionKind::String,
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::String)),
                            kind: data::host::NativeConversionKind::List {
                                storage: data::type_::ListTypeId(3),
                                item: data::host::NativeConversionId(2),
                            },
                        },
                        data::host::NativeConversion {
                            type_: data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                                package: data::Text::Static("application"),
                                module: data::Text::Static("main"),
                                name: data::Text::Static("Tree"),
                                arguments: data::Storage::Static(&[
                                    data::type_::TypeMetadata::String,
                                ]),
                            }))),
                            kind: data::host::NativeConversionKind::List {
                                storage: data::type_::ListTypeId(1),
                                item: data::host::NativeConversionId(1),
                            },
                        },
                    ]),
                },
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::String,
                        data::type_::ValueType::List(data::type_::ListTypeId(3)),
                    ])),
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::String,
                        data::type_::ValueType::List(data::type_::ListTypeId(3)),
                    ])),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Bool),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                    data::host::RegistrationType::Parameter(1),
                ]),
                return_: data::host::RegistrationType::Bool,
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                    data::host::RegistrationParameter::Value(1),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(1),
                    data::host::RegistrationType::Custom {
                        schema: data::host::CustomSchema {
                            package: data::Text::Static("application"),
                            module: data::Text::Static("main"),
                            name: data::Text::Static("Tree"),
                            parameter_count: 1,
                            constructors: data::Storage::Static(&[
                                data::host::ConstructorSchema {
                                    name: data::Text::Static("Leaf"),
                                    fields: data::Storage::Static(&[
                                        data::host::FieldSchema {
                                            label: None,
                                            type_: data::host::SchemaType::Parameter(0),
                                        },
                                    ]),
                                },
                                data::host::ConstructorSchema {
                                    name: data::Text::Static("Branch"),
                                    fields: data::Storage::Static(&[
                                        data::host::FieldSchema {
                                            label: None,
                                            type_: data::host::SchemaType::List(data::Storage::Static(&data::host::SchemaType::Custom {
                                                package: data::Text::Static("application"),
                                                module: data::Text::Static("main"),
                                                name: data::Text::Static("Tree"),
                                                arguments: data::Storage::Static(&[
                                                    data::host::SchemaType::Parameter(0),
                                                ]),
                                            })),
                                        },
                                    ]),
                                },
                            ]),
                        },
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::String,
                        ]),
                    },
                ]),
                construction_customs: data::Storage::Static(&[
                    data::host::CustomSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("main"),
                        name: data::Text::Static("Tree"),
                        parameter_count: 1,
                        constructors: data::Storage::Static(&[
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Leaf"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(0),
                                    },
                                ]),
                            },
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Branch"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::List(data::Storage::Static(&data::host::SchemaType::Custom {
                                            package: data::Text::Static("application"),
                                            module: data::Text::Static("main"),
                                            name: data::Text::Static("Tree"),
                                            arguments: data::Storage::Static(&[
                                                data::host::SchemaType::Parameter(0),
                                            ]),
                                        })),
                                    },
                                ]),
                            },
                        ]),
                    },
                ]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: Some(data::Storage::Static(&[])),
            }),
        },
    ]),
    never_functions: data::Storage::Static(&[]),
}
