data::HostedEntryArtifact {
    format: 16,
    program: data::ProgramTables {
        root: data::source::module_id(0),
        modules: data::Storage::Static(&[
            data::program::ExecutionModuleContext {
                module: data::Text::Static("example"),
                source_context: Some(data::source::SourceContext::from_static_block("src/example.gleam", r#"
pub fn empty() -> List(Int) {
  []
}

pub fn numeric_tail(flag: Bool) -> List(Int) {
  case flag {
    True -> empty()
    False -> fixed()
  }
}

fn fixed() -> List(Int) {
  [7, -9]
}

pub fn prefix(first: Int, second: Int, tail: List(Int)) -> List(Int) {
  [first, second, ..tail]
}

pub fn choose(
  flag: Bool,
  first: Int,
  second: Int,
  left: List(Int),
  right: List(Int),
) -> List(Int) {
  let selected = case flag {
    True -> [first, second, ..left]
    False -> [second, first, ..right]
  }
  [first, ..selected]
}

pub fn reverse(values: List(Int)) -> List(Int) {
  reverse_loop(values, [])
}

fn reverse_loop(values: List(Int), result: List(Int)) -> List(Int) {
  case values {
    [] -> result
    [head, ..tail] -> reverse_loop(tail, [head, ..result])
  }
}

pub fn selected_reverse(values: List(Int), result: List(Int)) -> List(Int) {
  case values {
    [] -> reverse(result)
    [head, ..tail] ->
      case head % 2 {
        0 -> selected_reverse(tail, [head, ..result])
        _ -> selected_reverse(tail, result)
      }
  }
}

pub fn promoted(value: Int, tail: List(Int)) -> List(Int) {
  let next = value + 1
  [next, ..tail]
}

fn uncompiled(value: Int, tail: List(Int)) -> List(Int) {
  let text = "uncompiled"
  case text {
    "uncompiled" -> [value, ..tail]
    _ -> tail
  }
}

pub fn interpreted_tail(value: Int, tail: List(Int), flag: Bool) -> List(Int) {
  case flag {
    True -> uncompiled(value, tail)
    False -> uncompiled(0 - value, tail)
  }
}

pub fn main() -> List(Int) {
  selected_reverse([3, 4, 5, 6], [])
}

pub fn caller(
  values: List(Int),
  offset: Int,
  text: String,
) -> #(String, Int, List(Int), List(Int)) {
  let result = reverse([offset, ..values])
  #(text, offset, values, result)
}

fn forever(value: Int, values: List(Int)) -> List(Int) {
  case value >= 0 {
    True -> forever(value + 1, [value, ..values])
    False -> values
  }
}

pub fn running() -> List(Int) {
  echo "entered-construction"
  forever(0, [])
}
"#)),
            },
        ]),
        main: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::List(data::function::ProfiledListFunctionId::Core(data::function::ListFunctionId::Int(data::function::IntListFunctionId {
            index: 0,
            type_id: data::type_::IntListTypeId {
                list_type: data::type_::ListTypeId(0),
            },
        })))),
        functions: data::function::FunctionTables {
            value_returns: data::function::ValueFunctionTables {
                never_functions: data::Storage::Static(&[]),
                int_functions: data::Storage::Static(&[]),
                float_functions: data::Storage::Static(&[]),
                string_functions: data::Storage::Static(&[]),
                bit_array_functions: data::Storage::Static(&[]),
                utf_codepoint_functions: data::Storage::Static(&[]),
                custom_functions: data::Storage::Static(&[]),
                external_functions: data::Storage::Static(&[]),
                bool_functions: data::Storage::Static(&[]),
                nil_functions: data::Storage::Static(&[]),
                tuple_functions: data::Storage::Static(&[]),
            },
            list_returns: data::function::ListFunctionTables {
                parameter_list_functions: data::Storage::Static(&[]),
                int_list_functions: data::Storage::Static(&[
                    (data::function::IntListFunctionId {
                        index: 0,
                        type_id: data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
                        },
                    }, data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
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
                                                3,
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
                                                4,
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
                                                5,
                                            ]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Plus,
                                            digits: data::Storage::Static(&[
                                                6,
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
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Int(data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                            data::graph::IntLocalId(0),
                                            data::graph::IntLocalId(1),
                                            data::graph::IntLocalId(2),
                                            data::graph::IntLocalId(3),
                                        ])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(1),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Int(data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntListFunctionId {
                                            index: 1,
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        },
                                        site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(1522, 1556)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
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
                                        ]),
                                    },
                                },
                            ]),
                        },
                    }))),
                    (data::function::IntListFunctionId {
                        index: 1,
                        type_id: data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
                        },
                    }, data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
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
                                            test: data::graph::BoolTest::ListLengthEquals {
                                                value: data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                },
                                                length: 0,
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
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
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(0),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(1),
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
                                        params: 2..3,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 3..5,
                                        instructions: 0..3,
                                        terminator: data::graph::Terminator::IntSwitch(data::graph::IntSwitch {
                                            subject: data::graph::IntLocalId(1),
                                            clauses: data::Storage::Static(&[
                                                (data::graph::IntegerLiteral {
                                                    sign: data::Sign::NoSign,
                                                    digits: data::Storage::Static(&[]),
                                                }, data::graph::Edge {
                                                    target: data::graph::BlockId(3),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                            local: data::graph::IntListLocalId(1),
                                                            type_id: data::type_::IntListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                            },
                                                        }),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                            local: data::graph::IntListLocalId(2),
                                                            type_id: data::type_::IntListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
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
                                                                family: data::graph::StorageFamily::IntList,
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
                                                        ]),
                                                    },
                                                }),
                                            ]),
                                            fallback: data::graph::Edge {
                                                target: data::graph::BlockId(4),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(1),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(2),
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
                                                            family: data::graph::StorageFamily::IntList,
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
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 5..8,
                                        instructions: 3..4,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(1),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(2),
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
                                                            family: data::graph::StorageFamily::IntList,
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
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 8..10,
                                        instructions: 4..4,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(1),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                        },
                                                    }),
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
                                                            family: data::graph::StorageFamily::IntList,
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
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
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
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::ListIndex {
                                            list: data::graph::IntListLocalId(0),
                                            index: 0,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(2),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(1),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Remainder {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Immediate(2),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(2),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Int(data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        }, data::graph::TypedListInstruction::Spread {
                                            elements: data::Storage::Static(&[
                                                data::graph::IntLocalId(0),
                                            ]),
                                            tail: data::graph::IntListLocalId(0),
                                        })),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntListFunctionId {
                                            index: 2,
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        },
                                        site: data::source::HostCallSite::from_static("example", "selected_reverse", data::source::SourceSpan::new(882, 897)),
                                    },
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
                            ]),
                        },
                    }))),
                    (data::function::IntListFunctionId {
                        index: 2,
                        type_id: data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
                        },
                    }, data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
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
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(1),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Int(data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntListFunctionId {
                                            index: 3,
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        },
                                        site: data::source::HostCallSite::from_static("example", "reverse", data::source::SourceSpan::new(583, 607)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                    ]),
                                    transfer: data::graph::Transfer {
                                        families: data::Storage::Static(&[]),
                                    },
                                },
                            ]),
                        },
                    }))),
                    (data::function::IntListFunctionId {
                        index: 3,
                        type_id: data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
                        },
                    }, data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
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
                                            test: data::graph::BoolTest::ListLengthEquals {
                                                value: data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                },
                                                length: 0,
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(1),
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
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(0),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(1),
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
                                        params: 2..3,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 3..5,
                                        instructions: 0..3,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(2),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(3),
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
                                                            family: data::graph::StorageFamily::IntList,
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
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(1),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
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
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::ListIndex {
                                            list: data::graph::IntListLocalId(0),
                                            index: 0,
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(2),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(1),
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
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(3),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Int(data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        }, data::graph::TypedListInstruction::Spread {
                                            elements: data::Storage::Static(&[
                                                data::graph::IntLocalId(0),
                                            ]),
                                            tail: data::graph::IntListLocalId(1),
                                        })),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntListLocalId(0)),
                            ]),
                        },
                    }))),
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
        compiled: {

            enum CompiledResume {
                Next(usize),
                Exit(data::compiled::CompiledProgress),
            }

            fn int_list_int_list_0(
                point: usize,
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::int_list::IntListValues, &data::compiled::int_list::IntListOps<'_>, &mut usize) -> CompiledResume;
                    7
                ] = [
                    |values, _lists, budget| {
                        CompiledResume::Exit(int_list_int_list_0_entry((), values, _lists, budget))
                    },
                    int_list_int_list_0_resume_1,
                    int_list_int_list_0_resume_2,
                    int_list_int_list_0_resume_3,
                    int_list_int_list_0_resume_4,
                    int_list_int_list_0_resume_5,
                    int_list_int_list_0_resume_6,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, _lists, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn int_list_int_list_0_entry(
                inputs: (),
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let () = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([]);
                    return data::compiled::CompiledProgress::Yield(0);
                }
                *budget -= 1;
                let b0_i0 = 3_i128;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([]);
                    return data::compiled::CompiledProgress::Yield(1);
                }
                *budget -= 1;
                let b0_i1 = 4_i128;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([]);
                    return data::compiled::CompiledProgress::Yield(2);
                }
                *budget -= 1;
                let b0_i2 = 5_i128;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([]);
                    return data::compiled::CompiledProgress::Yield(3);
                }
                *budget -= 1;
                let b0_i3 = 6_i128;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2, b0_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([]);
                    return data::compiled::CompiledProgress::Yield(4);
                }
                *budget -= 1;
                let b0_l0 = _lists.value(data::type_::IntListTypeId {
                    list_type: data::type_::ListTypeId(0),
                }, &[b0_i0 as i64, b0_i1 as i64, b0_i2 as i64, b0_i3 as i64]);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2, b0_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b0_l0]);
                    return data::compiled::CompiledProgress::Yield(5);
                }
                *budget -= 1;
                let b0_l1 = _lists.value(data::type_::IntListTypeId {
                    list_type: data::type_::ListTypeId(0),
                }, &[]);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2, b0_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b0_l0, b0_l1]);
                    return data::compiled::CompiledProgress::Yield(6);
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2, b0_i3]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([b0_l0, b0_l1]);
                data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
            }

            fn int_list_int_list_0_resume_1(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b0_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                let b0_i1 = 4_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([]);
                CompiledResume::Next(2)
            }

            fn int_list_int_list_0_resume_2(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b0_i0, b0_i1,) = (values.ints[0], values.ints[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                let b0_i2 = 5_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([]);
                CompiledResume::Next(3)
            }

            fn int_list_int_list_0_resume_3(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b0_i0, b0_i1, b0_i2,) = (values.ints[0], values.ints[1], values.ints[2],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                let b0_i3 = 6_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2, b0_i3]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([]);
                CompiledResume::Next(4)
            }

            fn int_list_int_list_0_resume_4(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b0_i0, b0_i1, b0_i2, b0_i3,) = (values.ints[0], values.ints[1], values.ints[2], values.ints[3],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2, b0_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;
                let b0_l0 = _lists.value(data::type_::IntListTypeId {
                    list_type: data::type_::ListTypeId(0),
                }, &[b0_i0 as i64, b0_i1 as i64, b0_i2 as i64, b0_i3 as i64]);

                values.ints.clear();
                values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2, b0_i3]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([b0_l0]);
                CompiledResume::Next(5)
            }

            fn int_list_int_list_0_resume_5(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list0 = values.int_lists.remove(0);
                let (b0_i0, b0_i1, b0_i2, b0_i3, b0_l0,) = (values.ints[0], values.ints[1], values.ints[2], values.ints[3], _list0,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2, b0_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b0_l0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;
                let b0_l1 = _lists.value(data::type_::IntListTypeId {
                    list_type: data::type_::ListTypeId(0),
                }, &[]);

                values.ints.clear();
                values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2, b0_i3]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([b0_l0, b0_l1]);
                CompiledResume::Next(6)
            }

            fn int_list_int_list_0_resume_6(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list1 = values.int_lists.remove(1);
                let _list0 = values.int_lists.remove(0);
                let (b0_i0, b0_i1, b0_i2, b0_i3, b0_l0, b0_l1,) = (values.ints[0], values.ints[1], values.ints[2], values.ints[3], _list0, _list1,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2, b0_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b0_l0, b0_l1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2, b0_i3]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([b0_l0, b0_l1]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn int_list_int_list_1(
                point: usize,
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::int_list::IntListValues, &data::compiled::int_list::IntListOps<'_>, &mut usize) -> CompiledResume;
                    9
                ] = [
                    |values, _lists, budget| {
                        let _list1 = values.int_lists.remove(1);
                        let _list0 = values.int_lists.remove(0);
                        CompiledResume::Exit(int_list_int_list_1_entry((_list0, _list1,), values, _lists, budget))
                    },
                    int_list_int_list_1_resume_1,
                    int_list_int_list_1_resume_2,
                    int_list_int_list_1_resume_3,
                    int_list_int_list_1_resume_4,
                    int_list_int_list_1_resume_5,
                    int_list_int_list_1_resume_6,
                    int_list_int_list_1_resume_7,
                    int_list_int_list_1_resume_8,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, _lists, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn int_list_int_list_1_entry(
                inputs: (data::compiled::int_list::IntList, data::compiled::int_list::IntList,),
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (mut b0_l0, mut b0_l1,) = inputs;
                'repeat: loop {
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b0_l0, b0_l1]);
                        return data::compiled::CompiledProgress::Yield(0);
                    }
                    *budget -= 1;
                    if b0_l0.is_empty() {
                        let _next = (b0_l1,);
                        drop(b0_l0);
                        let (b1_l0,) = _next;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.int_lists.clear();
                            values.int_lists.extend([b1_l0]);
                            return data::compiled::CompiledProgress::Yield(1);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b1_l0]);
                        return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0));
                    } else {
                        let (b2_l0, b2_l1,) = (b0_l0, b0_l1,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.int_lists.clear();
                            values.int_lists.extend([b2_l0, b2_l1]);
                            return data::compiled::CompiledProgress::Yield(2);
                        }
                        let b2_i0 = match _lists.index(&b2_l0, 0) {
                            Some(value) => value,
                            None => {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([b2_l0, b2_l1]);
                                return data::compiled::CompiledProgress::Interpreted(2);
                            }
                        };
                        *budget -= 1;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.int_lists.clear();
                            values.int_lists.extend([b2_l0, b2_l1]);
                            return data::compiled::CompiledProgress::Yield(3);
                        }
                        *budget -= 1;
                        let b2_l2 = _lists.tail(&b2_l0, data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
                        }, 1);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.int_lists.clear();
                            values.int_lists.extend([b2_l0, b2_l1, b2_l2]);
                            return data::compiled::CompiledProgress::Yield(4);
                        }
                        *budget -= 1;
                        let b2_i1 = if 2_i128 == 0 { 0_i128 } else { b2_i0 % 2_i128 };
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.int_lists.clear();
                            values.int_lists.extend([b2_l0, b2_l1, b2_l2]);
                            return data::compiled::CompiledProgress::Yield(5);
                        }
                        *budget -= 1;
                        if b2_i1 == 0_i128 {
                            let _next = (b2_i0, b2_l1, b2_l2,);
                            drop(b2_l0);
                            let (b3_i0, b3_l0, b3_l1,) = _next;
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([b3_l0, b3_l1]);
                                return data::compiled::CompiledProgress::Yield(6);
                            }
                            *budget -= 1;
                            let b3_l2 = _lists.prepend(data::type_::IntListTypeId {
                                list_type: data::type_::ListTypeId(0),
                            }, &[b3_i0 as i64], &b3_l0);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([b3_l0, b3_l1, b3_l2]);
                                return data::compiled::CompiledProgress::Yield(7);
                            }
                            *budget -= 1;
                            {
                                let _next = (b3_l1, b3_l2,);
                                drop(b3_l0);
                                (b0_l0, b0_l1,) = _next;
                                continue 'repeat;
                            }
                        } else {
                            let _next = (b2_l1, b2_l2,);
                            drop(b2_l0);
                            let (b4_l0, b4_l1,) = _next;
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([b4_l0, b4_l1]);
                                return data::compiled::CompiledProgress::Yield(8);
                            }
                            *budget -= 1;
                            {
                                (b0_l0, b0_l1,) = (b4_l1, b4_l0,);
                                continue 'repeat;
                            }
                        }
                    }
                }
            }

            fn int_list_int_list_1_resume_1(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list0 = values.int_lists.remove(0);
                let (b1_l0,) = (_list0,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b1_l0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([b1_l0]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn int_list_int_list_1_resume_2(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list1 = values.int_lists.remove(1);
                let _list0 = values.int_lists.remove(0);
                let (b2_l0, b2_l1,) = (_list0, _list1,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b2_l0, b2_l1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                let b2_i0 = match _lists.index(&b2_l0, 0) {
                    Some(value) => value,
                    None => {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b2_l0, b2_l1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(2));
                    }
                };
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([b2_l0, b2_l1]);
                CompiledResume::Next(3)
            }

            fn int_list_int_list_1_resume_3(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list1 = values.int_lists.remove(1);
                let _list0 = values.int_lists.remove(0);
                let (b2_i0, b2_l0, b2_l1,) = (values.ints[0], _list0, _list1,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b2_l0, b2_l1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                let b2_l2 = _lists.tail(&b2_l0, data::type_::IntListTypeId {
                    list_type: data::type_::ListTypeId(0),
                }, 1);

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([b2_l0, b2_l1, b2_l2]);
                CompiledResume::Next(4)
            }

            fn int_list_int_list_1_resume_4(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list2 = values.int_lists.remove(2);
                let _list1 = values.int_lists.remove(1);
                let _list0 = values.int_lists.remove(0);
                let (b2_i0, b2_l0, b2_l1, b2_l2,) = (values.ints[0], _list0, _list1, _list2,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b2_l0, b2_l1, b2_l2]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;
                let b2_i1 = if 2_i128 == 0 { 0_i128 } else { b2_i0 % 2_i128 };

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([b2_l0, b2_l1, b2_l2]);
                CompiledResume::Next(5)
            }

            fn int_list_int_list_1_resume_5(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list2 = values.int_lists.remove(2);
                let _list1 = values.int_lists.remove(1);
                let _list0 = values.int_lists.remove(0);
                let (b2_i0, b2_i1, b2_l0, b2_l1, b2_l2,) = (values.ints[0], values.ints[1], _list0, _list1, _list2,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b2_l0, b2_l1, b2_l2]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;
                if b2_i1 == 0_i128 {
                    let _next = (b2_i0, b2_l1, b2_l2,);
                    drop(b2_l0);
                    let (b3_i0, b3_l0, b3_l1,) = _next;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b3_l0, b3_l1]);
                    CompiledResume::Next(6)
                } else {
                    let _next = (b2_l1, b2_l2,);
                    drop(b2_l0);
                    let (b4_l0, b4_l1,) = _next;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b4_l0, b4_l1]);
                    CompiledResume::Next(8)
                }
            }

            fn int_list_int_list_1_resume_6(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list1 = values.int_lists.remove(1);
                let _list0 = values.int_lists.remove(0);
                let (b3_i0, b3_l0, b3_l1,) = (values.ints[0], _list0, _list1,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b3_l0, b3_l1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;
                let b3_l2 = _lists.prepend(data::type_::IntListTypeId {
                    list_type: data::type_::ListTypeId(0),
                }, &[b3_i0 as i64], &b3_l0);

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([b3_l0, b3_l1, b3_l2]);
                CompiledResume::Next(7)
            }

            fn int_list_int_list_1_resume_7(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list2 = values.int_lists.remove(2);
                let _list1 = values.int_lists.remove(1);
                let _list0 = values.int_lists.remove(0);
                let (b3_i0, b3_l0, b3_l1, b3_l2,) = (values.ints[0], _list0, _list1, _list2,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b3_l0, b3_l1, b3_l2]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                }
                *budget -= 1;
                {
                    let _next = (b3_l1, b3_l2,);
                    drop(b3_l0);
                    let (b0_l0, b0_l1,) = _next;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b0_l0, b0_l1]);
                    CompiledResume::Next(0)
                }
            }

            fn int_list_int_list_1_resume_8(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list1 = values.int_lists.remove(1);
                let _list0 = values.int_lists.remove(0);
                let (b4_l0, b4_l1,) = (_list0, _list1,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b4_l0, b4_l1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                }
                *budget -= 1;
                {
                    let (b0_l0, b0_l1,) = (b4_l1, b4_l0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b0_l0, b0_l1]);
                    CompiledResume::Next(0)
                }
            }

            fn int_list_int_list_2(
                point: usize,
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::int_list::IntListValues, &data::compiled::int_list::IntListOps<'_>, &mut usize) -> CompiledResume;
                    2
                ] = [
                    |values, _lists, budget| {
                        let _list0 = values.int_lists.remove(0);
                        CompiledResume::Exit(int_list_int_list_2_entry((_list0,), values, _lists, budget))
                    },
                    int_list_int_list_2_resume_1,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, _lists, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn int_list_int_list_2_entry(
                inputs: (data::compiled::int_list::IntList,),
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (b0_l0,) = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b0_l0]);
                    return data::compiled::CompiledProgress::Yield(0);
                }
                *budget -= 1;
                let b0_l1 = _lists.value(data::type_::IntListTypeId {
                    list_type: data::type_::ListTypeId(0),
                }, &[]);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b0_l0, b0_l1]);
                    return data::compiled::CompiledProgress::Yield(1);
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([b0_l0, b0_l1]);
                data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
            }

            fn int_list_int_list_2_resume_1(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list1 = values.int_lists.remove(1);
                let _list0 = values.int_lists.remove(0);
                let (b0_l0, b0_l1,) = (_list0, _list1,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b0_l0, b0_l1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([b0_l0, b0_l1]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn int_list_int_list_3(
                point: usize,
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::int_list::IntListValues, &data::compiled::int_list::IntListOps<'_>, &mut usize) -> CompiledResume;
                    6
                ] = [
                    |values, _lists, budget| {
                        let _list1 = values.int_lists.remove(1);
                        let _list0 = values.int_lists.remove(0);
                        CompiledResume::Exit(int_list_int_list_3_entry((_list0, _list1,), values, _lists, budget))
                    },
                    int_list_int_list_3_resume_1,
                    int_list_int_list_3_resume_2,
                    int_list_int_list_3_resume_3,
                    int_list_int_list_3_resume_4,
                    int_list_int_list_3_resume_5,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, _lists, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn int_list_int_list_3_entry(
                inputs: (data::compiled::int_list::IntList, data::compiled::int_list::IntList,),
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (mut b0_l0, mut b0_l1,) = inputs;
                'repeat: loop {
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b0_l0, b0_l1]);
                        return data::compiled::CompiledProgress::Yield(0);
                    }
                    *budget -= 1;
                    if b0_l0.is_empty() {
                        let _next = (b0_l1,);
                        drop(b0_l0);
                        let (b1_l0,) = _next;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.int_lists.clear();
                            values.int_lists.extend([b1_l0]);
                            return data::compiled::CompiledProgress::Yield(1);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b1_l0]);
                        return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0));
                    } else {
                        let (b2_l0, b2_l1,) = (b0_l0, b0_l1,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.int_lists.clear();
                            values.int_lists.extend([b2_l0, b2_l1]);
                            return data::compiled::CompiledProgress::Yield(2);
                        }
                        let b2_i0 = match _lists.index(&b2_l0, 0) {
                            Some(value) => value,
                            None => {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([b2_l0, b2_l1]);
                                return data::compiled::CompiledProgress::Interpreted(2);
                            }
                        };
                        *budget -= 1;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.int_lists.clear();
                            values.int_lists.extend([b2_l0, b2_l1]);
                            return data::compiled::CompiledProgress::Yield(3);
                        }
                        *budget -= 1;
                        let b2_l2 = _lists.tail(&b2_l0, data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
                        }, 1);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.int_lists.clear();
                            values.int_lists.extend([b2_l0, b2_l1, b2_l2]);
                            return data::compiled::CompiledProgress::Yield(4);
                        }
                        *budget -= 1;
                        let b2_l3 = _lists.prepend(data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
                        }, &[b2_i0 as i64], &b2_l1);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.int_lists.clear();
                            values.int_lists.extend([b2_l0, b2_l1, b2_l2, b2_l3]);
                            return data::compiled::CompiledProgress::Yield(5);
                        }
                        *budget -= 1;
                        {
                            let _next = (b2_l2, b2_l3,);
                            drop(b2_l0);
                            drop(b2_l1);
                            (b0_l0, b0_l1,) = _next;
                            continue 'repeat;
                        }
                    }
                }
            }

            fn int_list_int_list_3_resume_1(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list0 = values.int_lists.remove(0);
                let (b1_l0,) = (_list0,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b1_l0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([b1_l0]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn int_list_int_list_3_resume_2(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list1 = values.int_lists.remove(1);
                let _list0 = values.int_lists.remove(0);
                let (b2_l0, b2_l1,) = (_list0, _list1,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b2_l0, b2_l1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                let b2_i0 = match _lists.index(&b2_l0, 0) {
                    Some(value) => value,
                    None => {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b2_l0, b2_l1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(2));
                    }
                };
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([b2_l0, b2_l1]);
                CompiledResume::Next(3)
            }

            fn int_list_int_list_3_resume_3(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list1 = values.int_lists.remove(1);
                let _list0 = values.int_lists.remove(0);
                let (b2_i0, b2_l0, b2_l1,) = (values.ints[0], _list0, _list1,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b2_l0, b2_l1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;
                let b2_l2 = _lists.tail(&b2_l0, data::type_::IntListTypeId {
                    list_type: data::type_::ListTypeId(0),
                }, 1);

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([b2_l0, b2_l1, b2_l2]);
                CompiledResume::Next(4)
            }

            fn int_list_int_list_3_resume_4(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list2 = values.int_lists.remove(2);
                let _list1 = values.int_lists.remove(1);
                let _list0 = values.int_lists.remove(0);
                let (b2_i0, b2_l0, b2_l1, b2_l2,) = (values.ints[0], _list0, _list1, _list2,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b2_l0, b2_l1, b2_l2]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;
                let b2_l3 = _lists.prepend(data::type_::IntListTypeId {
                    list_type: data::type_::ListTypeId(0),
                }, &[b2_i0 as i64], &b2_l1);

                values.ints.clear();
                values.ints.extend_from_slice(&[b2_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([b2_l0, b2_l1, b2_l2, b2_l3]);
                CompiledResume::Next(5)
            }

            fn int_list_int_list_3_resume_5(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list3 = values.int_lists.remove(3);
                let _list2 = values.int_lists.remove(2);
                let _list1 = values.int_lists.remove(1);
                let _list0 = values.int_lists.remove(0);
                let (b2_i0, b2_l0, b2_l1, b2_l2, b2_l3,) = (values.ints[0], _list0, _list1, _list2, _list3,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b2_l0, b2_l1, b2_l2, b2_l3]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;
                {
                    let _next = (b2_l2, b2_l3,);
                    drop(b2_l0);
                    drop(b2_l1);
                    let (b0_l0, b0_l1,) = _next;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b0_l0, b0_l1]);
                    CompiledResume::Next(0)
                }
            }
            data::compiled::CompiledFunctions {
                ints: data::Storage::Static(&[
                ]),
                bools: data::Storage::Static(&[
                ]),
                customs: data::Storage::Static(&[
                ]),
                int_lists: data::Storage::Static(&[
                    data::compiled::CompiledFunction {
                        function: data::function::IntListFunctionId {
                            index: 0,
                            type_id: data::type_::IntListTypeId {
                                list_type: data::type_::ListTypeId(0),
                            },
                        },
                        implementation: data::compiled::CompiledImplementation::IntList(data::compiled::IntListImplementation {
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 0,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 1,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 2,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 3,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 4,
                                    ints: 4,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 5,
                                    ints: 4,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 1,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 6,
                                    ints: 4,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 2,
                                },
                            ]),
                            run: int_list_int_list_0,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntListFunctionId {
                            index: 1,
                            type_id: data::type_::IntListTypeId {
                                list_type: data::type_::ListTypeId(0),
                            },
                        },
                        implementation: data::compiled::CompiledImplementation::IntList(data::compiled::IntListImplementation {
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 0,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 2,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 0,
                                    ints: 0,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 1,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 0,
                                    ints: 0,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 2,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 1,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 2,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 2,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 3,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 3,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 3,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(3),
                                    instruction: 0,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 2,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(3),
                                    instruction: 1,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 3,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(4),
                                    instruction: 0,
                                    ints: 0,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 2,
                                },
                            ]),
                            run: int_list_int_list_1,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntListFunctionId {
                            index: 2,
                            type_id: data::type_::IntListTypeId {
                                list_type: data::type_::ListTypeId(0),
                            },
                        },
                        implementation: data::compiled::CompiledImplementation::IntList(data::compiled::IntListImplementation {
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 0,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 1,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 1,
                                    ints: 0,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 2,
                                },
                            ]),
                            run: int_list_int_list_2,
                        }),
                    },
                    data::compiled::CompiledFunction {
                        function: data::function::IntListFunctionId {
                            index: 3,
                            type_id: data::type_::IntListTypeId {
                                list_type: data::type_::ListTypeId(0),
                            },
                        },
                        implementation: data::compiled::CompiledImplementation::IntList(data::compiled::IntListImplementation {
                            entry: 0,
                            checkpoints: data::Storage::Static(&[
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 0,
                                    ints: 0,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 2,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(1),
                                    instruction: 0,
                                    ints: 0,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 1,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 0,
                                    ints: 0,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 2,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 1,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 2,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 2,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 3,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 3,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 4,
                                },
                            ]),
                            run: int_list_int_list_3,
                        }),
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
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
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
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
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
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 0..2,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(1),
                        data::type_::ValueShapeId(1),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 2..3,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(1),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 3..5,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(1),
                        data::type_::ValueShapeId(1),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
            ]),
            parameters: data::Storage::Static(&[
                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                    local: data::graph::IntListLocalId(0),
                    type_id: data::type_::IntListTypeId {
                        list_type: data::type_::ListTypeId(0),
                    },
                }),
                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                    local: data::graph::IntListLocalId(1),
                    type_id: data::type_::IntListTypeId {
                        list_type: data::type_::ListTypeId(0),
                    },
                }),
                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                    local: data::graph::IntListLocalId(0),
                    type_id: data::type_::IntListTypeId {
                        list_type: data::type_::ListTypeId(0),
                    },
                }),
                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                    local: data::graph::IntListLocalId(0),
                    type_id: data::type_::IntListTypeId {
                        list_type: data::type_::ListTypeId(0),
                    },
                }),
                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                    local: data::graph::IntListLocalId(1),
                    type_id: data::type_::IntListTypeId {
                        list_type: data::type_::ListTypeId(0),
                    },
                }),
            ]),
        },
        list_types: data::type_::ListTypeTable {
            types: data::Storage::Static(&[
                data::type_::ListStorageTypeId::Int(data::type_::IntListTypeId {
                    list_type: data::type_::ListTypeId(0),
                }),
            ]),
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
                data::type_::ValueShapeDescriptor::Int,
                data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(0)),
            ]),
            shape_types: data::Storage::Static(&[
                data::type_::ValueType::Int,
                data::type_::ValueType::List(data::type_::ListTypeId(0)),
            ]),
            custom_shapes: data::Storage::Static(&[]),
        },
    },
    value_functions: data::Storage::Static(&[]),
    never_functions: data::Storage::Static(&[]),
}
