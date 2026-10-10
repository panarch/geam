data::HostedModuleArtifact {
    module: data::ModuleArtifact {
        format: 30,
        program: data::ProgramTables {
            root: data::source::module_id(0),
            modules: data::Storage::Static(&[
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("example"),
                    source_context: Some(data::source::SourceContext::from_static_block("src/example.gleam", r#"
pub fn count(values: List(Int), total: Int) -> Int {
  case values {
    [] -> total
    [1, ..tail] -> count(tail, total + 1)
    [_, ..tail] -> count(tail, total)
  }
}

pub fn asserted(values: List(Int), total: Int) -> Int {
  case values {
    [] -> total
    _ -> {
      let assert [head, ..tail] = values
      let total = case head {
        1 -> total + 1
        _ -> total
      }
      asserted(tail, total)
    }
  }
}

pub fn equal_walk(values: List(Int), stop: List(Int), total: Int) -> Int {
  case values == stop {
    True -> total
    False ->
      case values {
        [] -> total
        [1, ..tail] -> equal_walk(tail, stop, total + 1)
        [_, ..tail] -> equal_walk(tail, stop, total)
      }
  }
}

pub fn prefix(values: List(Int)) -> Int {
  let assert [0, first as alias, _, second, ..tail] as original = values
  case tail == original {
    True -> alias
    False -> first + second
  }
}

pub fn same(left: List(Int), right: List(Int), negate: Bool) -> Bool {
  case negate {
    True -> left != right
    False -> left == right
  }
}

pub fn shuffle(left: List(Int), right: List(Int), steps: Int) -> Int {
  case steps <= 0 {
    True -> {
      let assert [first, ..] = left
      let assert [second, ..] = right
      first - second
    }
    False -> shuffle(right, left, steps - 1)
  }
}

pub fn duplicate(values: List(Int), other: List(Int), steps: Int) -> Int {
  case steps {
    0 -> {
      let assert [head, ..] = values
      case values == other {
        True -> head
        False -> -head
      }
    }
    _ -> duplicate(values, values, steps - 1)
  }
}

pub fn captured(values: List(Int), offset: Int) -> Int {
  let calculate = fn(input: Int) {
    let assert [head, ..] = values
    case input < 0 {
      True -> head - offset
      False -> head + offset + input
    }
  }
  calculate(3)
}

pub fn caller(
  values: List(Int),
  offset: Int,
  text: String,
) -> #(String, Int, List(Int), Int) {
  let first = count(values, offset)
  #(text, first, values, captured(values, offset))
}

pub fn stop(values: List(Int)) -> Int {
  case values {
    [] -> panic
    [head, ..] -> head
  }
}

pub fn late(values: List(Int)) -> Int {
  let assert [head, 2, ..tail] as original = values
  case tail == original {
    True -> head
    False -> head + 3
  }
}

fn forever(values: List(Int), total: Int) -> Int {
  case values {
    [] -> total
    _ -> forever(values, total + 1)
  }
}

pub fn running() -> Int {
  echo "entered-list"
  forever([1], 0)
}

pub fn main() -> Int {
  count([1, 0, 1], 5) + asserted([0, 1], 2) + equal_walk([1, 1], [], 0)
}
"#)),
                },
            ]),
            main: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Tuple {
                id: data::function::TupleFunctionId(0),
                return_type: data::Storage::Static(&[
                    data::type_::ValueType::String,
                    data::type_::ValueType::Int,
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                    data::type_::ValueType::Int,
                ]),
            }),
            functions: data::function::FunctionTables {
                value_returns: data::function::ValueFunctionTables {
                    never_functions: data::Storage::Static(&[]),
                    int_functions: data::Storage::Static(&[
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
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Echo(data::graph::Echo {
                                                subject: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                message: None,
                                                site: data::source::EchoSite::from_static("example", "running", data::source::SourceSpan::new(2461, 2480)),
                                                next: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
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
                                            instructions: 1..4,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("entered-list"))),
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
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::IntFunctionId(3),
                                            site: data::source::HostCallSite::from_static("example", "running", data::source::SourceSpan::new(2483, 2498)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
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
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
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
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntList,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
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
                                                    target: data::graph::BlockId(3),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                            local: data::graph::IntListLocalId(0),
                                                            type_id: data::type_::IntListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                            },
                                                        }),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(7),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                            local: data::graph::IntListLocalId(0),
                                                            type_id: data::type_::IntListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                            },
                                                        }),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 5..7,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::EqualInt {
                                                    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                                    right: data::graph::IntegerOperand::Immediate(1),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(4),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                            local: data::graph::IntListLocalId(0),
                                                            type_id: data::type_::IntListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                            },
                                                        }),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
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
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 7..9,
                                            instructions: 1..3,
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
                                            params: 9..11,
                                            instructions: 3..3,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(6),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                            local: data::graph::IntListLocalId(0),
                                                            type_id: data::type_::IntListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                            },
                                                        }),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 11..13,
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
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 13..15,
                                            instructions: 4..4,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(6),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                            local: data::graph::IntListLocalId(0),
                                                            type_id: data::type_::IntListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                            },
                                                        }),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
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
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
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
                                                shape: data::type_::ValueShapeId(1),
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
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
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
                                                shape: data::type_::ValueShapeId(4),
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
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(4)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::IntList {
                                                            target: data::graph::IntListLocalId(0),
                                                            source: data::graph::IntListLocalId(0),
                                                        },
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
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::FunctionCall {
                                                function: data::graph::IntFunctionLocalId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "captured", data::source::SourceSpan::new(1830, 1842)),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(2)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
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
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntList,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
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
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                            local: data::graph::IntListLocalId(0),
                                                            type_id: data::type_::IntListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                            },
                                                        }),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
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
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
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
                                            params: 0..3,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                                pattern: data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                    elements: data::Storage::Static(&[
                                                        data::graph::MatchPattern::Bind(data::graph::MatchPatternBinding {
                                                            index: 0,
                                                        }),
                                                    ]),
                                                    tail: Some(data::graph::MatchPatternListTail::Ignore),
                                                }),
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
                                                                family: data::graph::StorageFamily::IntList,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                failure: data::graph::Edge {
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
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::LtInt {
                                                    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                    right: data::graph::IntegerOperand::Immediate(0),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
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
                                                        ]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(3),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 6..8,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 8..11,
                                            instructions: 1..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 11..12,
                                            instructions: 2..2,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                                message: None,
                                                site: data::source::PanicSite::from_static("example", "<anonymous:0>", data::source::SourceSpan::new(1701, 1711)),
                                                pattern_span: data::source::SourceSpan::new(1712, 1722),
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
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
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
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
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::IntegerRegion(data::graph::ArithmeticRegion {
                                            inputs: data::Storage::Static(&[
                                                data::graph::IntLocalId(2),
                                                data::graph::IntLocalId(1),
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
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(2)),
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(3)),
                                ]),
                            },
                        })),
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
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
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
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(0),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "caller", data::source::SourceSpan::new(1965, 1986)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(0),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "caller", data::source::SourceSpan::new(2012, 2036)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                        data::type_::ValueType::Int,
                                                        data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                },
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            ]))),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::TupleLocalId(0)),
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
            compiled: {

                enum CompiledResume {
                    Next(usize),
                    Exit(data::compiled::CompiledProgress),
                }
                const CALL_GROUP_0: [data::compiled::calls::CallStart; 1] = {
                    use data::compiled::calls::{CallArguments, CallCapture, CallExecution, CallInputs, CallInteger, CallOps, CallOutput, CallProgress, CallStorage, CallValues, IntCallable};
                    use data::compiled::int_list::IntList;
                    enum FunctionState {
                        Int2Point0 { int_list0: IntList, int0: i128 },
                        Int2Point1 { int_list0: IntList, int0: i128, int_function0: IntCallable },
                        Int2Point2 { int_list0: IntList, int0: i128, int_function0: IntCallable, int1: i128 },
                        Int2Point3 { int_list0: IntList, int0: i128, int_function0: IntCallable, int1: i128, int2: i128 },
                        Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                    }
                    enum IntReturn {
                        Int2Call2 { int_list0: IntList, int0: i128, int_function0: IntCallable, int1: i128 },
                    }
                    impl IntReturn {
                        fn site(&self) -> data::source::HostCallSite {
                            match *self {
                                Self::Int2Call2 { .. } => data::source::HostCallSite::from_static("example", "captured", data::source::SourceSpan::new(1830, 1842)),
                            }
                        }
                        fn small(self, result: i128) -> FunctionState {
                            match self {
                                Self::Int2Call2 { int_list0, int0, int_function0, int1 } => {
                                    let int2 = result;
                                    FunctionState::Int2Point3 { int_list0, int0, int_function0, int1, int2 }
                                },
                            }
                        }
                        fn resume(self, result: CallInteger) -> FunctionState {
                            if let Some(result) = result.small() {
                                return self.small(result);
                            }
                            match self {
                                Self::Int2Call2 { int_list0, int0, int_function0, int1 } => {
                                    let int2 = result;
                                    FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 3,
                                        ints: 3,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 1,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into(), int2], int_lists: vec![int_list0], int_functions: vec![int_function0], ..CallValues::default() }) }
                                },
                            }
                        }
                    }
                    #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                    enum FunctionStep {
                        Yield(FunctionState),
                        Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                        Int { value: i128 },
                        IntBridge { function: data::function::IntFunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: IntReturn },
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
                                    FunctionStep::IntBridge { function, site, arguments, caller } => return CallProgress::Int {
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
                            FunctionState::Int2Point0 { int_list0, int0 } => calls_int_2_run(Int2State::Point0 { int_list0, int0 }, ops, budget),
                            FunctionState::Int2Point1 { int_list0, int0, int_function0 } => calls_int_2_run(Int2State::Point1 { int_list0, int0, int_function0 }, ops, budget),
                            FunctionState::Int2Point2 { int_list0, int0, int_function0, int1 } => calls_int_2_run(Int2State::Point2 { int_list0, int0, int_function0, int1 }, ops, budget),
                            FunctionState::Int2Point3 { int_list0, int0, int_function0, int1, int2 } => calls_int_2_run(Int2State::Point3 { int_list0, int0, int_function0, int1, int2 }, ops, budget),
                        }
                    }
                    enum Int2State {
                        Point0 { int_list0: IntList, int0: i128 },
                        Point1 { int_list0: IntList, int0: i128, int_function0: IntCallable },
                        Point2 { int_list0: IntList, int0: i128, int_function0: IntCallable, int1: i128 },
                        Point3 { int_list0: IntList, int0: i128, int_function0: IntCallable, int1: i128, int2: i128 },
                    }
                    fn calls_int_2_run(mut active: Int2State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        loop {
                            match active {
                                Int2State::Point0 { int_list0, int0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point0 { int_list0, int0 }); }
                                    *budget -= 1;
                                    let int_function0 = ops.int_closure(data::function::IntFunctionId(4), data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    }, vec![CallCapture::int_list(data::graph::IntListLocalId(0), int_list0.clone()), CallCapture::int(data::graph::IntLocalId(1), int0)]);
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point1 { int_list0, int0, int_function0 }); }
                                    *budget -= 1;
                                    let int1 = 3_i128;
                                    if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 1,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], int_lists: vec![int_list0], int_functions: vec![int_function0], ..CallValues::default() }) }; }
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point2 { int_list0, int0, int_function0, int1 }); }
                                    *budget -= 1;
                                    return {
                                        let callable = &int_function0;
                                        let captures = callable.captures();
                                        let target = callable.target();
                                        FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("example", "captured", data::source::SourceSpan::new(1830, 1842)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int1.into()], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: IntReturn::Int2Call2 { int_list0, int0, int_function0, int1 } }
                                    };
                                },
                                Int2State::Point1 { int_list0, int0, int_function0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point1 { int_list0, int0, int_function0 }); }
                                    *budget -= 1;
                                    let int1 = 3_i128;
                                    if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 1,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], int_lists: vec![int_list0], int_functions: vec![int_function0], ..CallValues::default() }) }; }
                                    active = Int2State::Point2 { int_list0, int0, int_function0, int1 };
                                    continue;
                                },
                                Int2State::Point2 { int_list0, int0, int_function0, int1 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point2 { int_list0, int0, int_function0, int1 }); }
                                    *budget -= 1;
                                    return {
                                        let callable = &int_function0;
                                        let captures = callable.captures();
                                        let target = callable.target();
                                        FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("example", "captured", data::source::SourceSpan::new(1830, 1842)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int1.into()], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: IntReturn::Int2Call2 { int_list0, int0, int_function0, int1 } }
                                    };
                                },
                                Int2State::Point3 { int_list0, int0, int_function0, int1, int2 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point3 { int_list0, int0, int_function0, int1, int2 }); }
                                    *budget -= 1;
                                    return {
                                        FunctionStep::Int { value: int2 }
                                    };
                                },
                            }
                        }
                    }
                    fn calls_int_2_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                        let active = match point {
                            0 => FunctionState::Int2Point0 { int_list0: values.int_list(0)?, int0: values.int(0)? },
                            1 => FunctionState::Int2Point1 { int_list0: values.int_list(0)?, int0: values.int(0)?, int_function0: values.int_function(0)? },
                            2 => {
                                return None
                            },
                            3 => FunctionState::Int2Point3 { int_list0: values.int_list(0)?, int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)?, int2: values.int(2)? },
                            _ => return None,
                        };
                        Some(active)
                    }
                    fn calls_int_2_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                        if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point, values) { return Some(execution); }
                        let active = calls_int_2_state(point, values)?;
                        Some(Box::new(FunctionExecution::new(active)))
                    }
                    [calls_int_2_start]
                };

                fn int_list_int_1(
                    point: usize,
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {

                    const RESUME: [
                        fn(&mut data::compiled::int_list::IntListValues, &data::compiled::int_list::IntListOps<'_>, &mut usize) -> CompiledResume;
                        12
                    ] = [
                        |values, _lists, budget| {
                            let _list0 = values.int_lists.remove(0);
                            CompiledResume::Exit(int_list_int_1_entry((values.ints[0], _list0,), values, _lists, budget))
                        },
                        int_list_int_1_resume_1,
                        int_list_int_1_resume_2,
                        int_list_int_1_resume_3,
                        int_list_int_1_resume_4,
                        int_list_int_1_resume_5,
                        int_list_int_1_resume_6,
                        int_list_int_1_resume_7,
                        int_list_int_1_resume_8,
                        int_list_int_1_resume_9,
                        int_list_int_1_resume_10,
                        int_list_int_1_resume_11,
                    ];

                    let mut point = point;
                    loop {
                        match RESUME[point](values, _lists, budget) {
                            CompiledResume::Next(next) => point = next,
                            CompiledResume::Exit(progress) => return progress,
                        }
                    }
                }

                fn int_list_int_1_entry(
                    inputs: (i128, data::compiled::int_list::IntList,),
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {
                    let (mut b0_i0, mut b0_l0,) = inputs;
                    'repeat: loop {
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b0_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.int_lists.clear();
                            values.int_lists.extend([b0_l0]);
                            return data::compiled::CompiledProgress::Yield(0);
                        }
                        *budget -= 1;
                        if b0_l0.is_empty() {
                            let _next = (b0_i0,);
                            drop(b0_l0);
                            let (b1_i0,) = _next;
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b1_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([]);
                                return data::compiled::CompiledProgress::Yield(1);
                            }
                            *budget -= 1;

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.int_lists.clear();
                            values.int_lists.extend([]);
                            return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0));
                        } else {
                            let (b2_i0, b2_l0,) = (b0_i0, b0_l0,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([b2_l0]);
                                return data::compiled::CompiledProgress::Yield(2);
                            }
                            *budget -= 1;
                            let (b6_i0, b6_l0,) = if !b2_l0.is_empty() {
                                let (b3_i0, b3_l0,) = (b2_i0, b2_l0,);
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.int_lists.clear();
                                    values.int_lists.extend([b3_l0]);
                                    return data::compiled::CompiledProgress::Yield(3);
                                }
                                let b3_i1 = match _lists.index(&b3_l0, 0) {
                                    Some(value) => value,
                                    None => {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b3_i0]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.int_lists.clear();
                                        values.int_lists.extend([b3_l0]);
                                        return data::compiled::CompiledProgress::Interpreted(3);
                                    }
                                };
                                *budget -= 1;
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.int_lists.clear();
                                    values.int_lists.extend([b3_l0]);
                                    return data::compiled::CompiledProgress::Yield(4);
                                }
                                *budget -= 1;
                                if b3_i1 == 1_i128 {
                                    let (b4_i0, b4_l0,) = (b3_i0, b3_l0,);
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b4_i0]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.int_lists.clear();
                                        values.int_lists.extend([b4_l0]);
                                        return data::compiled::CompiledProgress::Yield(5);
                                    }
                                    *budget -= 1;
                                    let b4_l1 = _lists.tail(&b4_l0, data::type_::IntListTypeId {
                                        list_type: data::type_::ListTypeId(0),
                                    }, 1);
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b4_i0]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.int_lists.clear();
                                        values.int_lists.extend([b4_l0, b4_l1]);
                                        return data::compiled::CompiledProgress::Yield(6);
                                    }
                                    *budget -= 1;
                                    let b4_i1 = b4_i0 + 1_i128;
                                    if b4_i1 < i128::from(i64::MIN) || b4_i1 > i128::from(i64::MAX) {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.int_lists.clear();
                                        values.int_lists.extend([b4_l0, b4_l1]);
                                        return data::compiled::CompiledProgress::Interpreted(7);
                                    }
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.int_lists.clear();
                                        values.int_lists.extend([b4_l0, b4_l1]);
                                        return data::compiled::CompiledProgress::Yield(7);
                                    }
                                    *budget -= 1;
                                    {
                                        let _next = (b4_i1, b4_l1,);
                                        drop(b4_l0);
                                        (b0_i0, b0_l0,) = _next;
                                        continue 'repeat;
                                    }
                                } else {
                                    let (b5_i0, b5_l0,) = (b3_i0, b3_l0,);
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b5_i0]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.int_lists.clear();
                                        values.int_lists.extend([b5_l0]);
                                        return data::compiled::CompiledProgress::Yield(8);
                                    }
                                    *budget -= 1;
                                    let (b6_i0, b6_l0,) = {
                                        (b5_i0, b5_l0,)
                                    };
                                    (b6_i0, b6_l0,)
                                }
                            } else {
                                let (b7_i0, b7_l0,) = (b2_i0, b2_l0,);
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b7_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.int_lists.clear();
                                    values.int_lists.extend([b7_l0]);
                                    return data::compiled::CompiledProgress::Yield(11);
                                }
                                *budget -= 1;
                                let (b6_i0, b6_l0,) = {
                                    (b7_i0, b7_l0,)
                                };
                                (b6_i0, b6_l0,)
                            };
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b6_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([b6_l0]);
                                return data::compiled::CompiledProgress::Yield(9);
                            }
                            *budget -= 1;
                            let b6_l1 = _lists.tail(&b6_l0, data::type_::IntListTypeId {
                                list_type: data::type_::ListTypeId(0),
                            }, 1);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b6_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([b6_l0, b6_l1]);
                                return data::compiled::CompiledProgress::Yield(10);
                            }
                            *budget -= 1;
                            {
                                let _next = (b6_i0, b6_l1,);
                                drop(b6_l0);
                                (b0_i0, b0_l0,) = _next;
                                continue 'repeat;
                            }
                        }
                    }
                }

                fn int_list_int_1_resume_1(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_i0,) = (values.ints[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
                }

                fn int_list_int_1_resume_2(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let _list0 = values.int_lists.remove(0);
                    let (b2_i0, b2_l0,) = (values.ints[0], _list0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b2_l0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                    }
                    *budget -= 1;
                    if !b2_l0.is_empty() {
                        let (b3_i0, b3_l0,) = (b2_i0, b2_l0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b3_l0]);
                        CompiledResume::Next(3)
                    } else {
                        let (b7_i0, b7_l0,) = (b2_i0, b2_l0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b7_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b7_l0]);
                        CompiledResume::Next(11)
                    }
                }

                fn int_list_int_1_resume_3(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let _list0 = values.int_lists.remove(0);
                    let (b3_i0, b3_l0,) = (values.ints[0], _list0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b3_l0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                    }
                    let b3_i1 = match _lists.index(&b3_l0, 0) {
                        Some(value) => value,
                        None => {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b3_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.int_lists.clear();
                            values.int_lists.extend([b3_l0]);
                            return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(3));
                        }
                    };
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b3_l0]);
                    CompiledResume::Next(4)
                }

                fn int_list_int_1_resume_4(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let _list0 = values.int_lists.remove(0);
                    let (b3_i0, b3_i1, b3_l0,) = (values.ints[0], values.ints[1], _list0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b3_l0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                    }
                    *budget -= 1;
                    if b3_i1 == 1_i128 {
                        let (b4_i0, b4_l0,) = (b3_i0, b3_l0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b4_l0]);
                        CompiledResume::Next(5)
                    } else {
                        let (b5_i0, b5_l0,) = (b3_i0, b3_l0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b5_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b5_l0]);
                        CompiledResume::Next(8)
                    }
                }

                fn int_list_int_1_resume_5(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let _list0 = values.int_lists.remove(0);
                    let (b4_i0, b4_l0,) = (values.ints[0], _list0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b4_l0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                    }
                    *budget -= 1;
                    let b4_l1 = _lists.tail(&b4_l0, data::type_::IntListTypeId {
                        list_type: data::type_::ListTypeId(0),
                    }, 1);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b4_l0, b4_l1]);
                    CompiledResume::Next(6)
                }

                fn int_list_int_1_resume_6(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let _list1 = values.int_lists.remove(1);
                    let _list0 = values.int_lists.remove(0);
                    let (b4_i0, b4_l0, b4_l1,) = (values.ints[0], _list0, _list1,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b4_l0, b4_l1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                    }
                    *budget -= 1;
                    let b4_i1 = b4_i0 + 1_i128;
                    if b4_i1 < i128::from(i64::MIN) || b4_i1 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b4_l0, b4_l1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(7));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b4_l0, b4_l1]);
                    CompiledResume::Next(7)
                }

                fn int_list_int_1_resume_7(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let _list1 = values.int_lists.remove(1);
                    let _list0 = values.int_lists.remove(0);
                    let (b4_i0, b4_i1, b4_l0, b4_l1,) = (values.ints[0], values.ints[1], _list0, _list1,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b4_l0, b4_l1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                    }
                    *budget -= 1;
                    {
                        let _next = (b4_i1, b4_l1,);
                        drop(b4_l0);
                        let (b0_i0, b0_l0,) = _next;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b0_l0]);
                        CompiledResume::Next(0)
                    }
                }

                fn int_list_int_1_resume_8(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let _list0 = values.int_lists.remove(0);
                    let (b5_i0, b5_l0,) = (values.ints[0], _list0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b5_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b5_l0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                    }
                    *budget -= 1;
                    {
                        let (b6_i0, b6_l0,) = (b5_i0, b5_l0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b6_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b6_l0]);
                        CompiledResume::Next(9)
                    }
                }

                fn int_list_int_1_resume_9(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let _list0 = values.int_lists.remove(0);
                    let (b6_i0, b6_l0,) = (values.ints[0], _list0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b6_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b6_l0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(9));
                    }
                    *budget -= 1;
                    let b6_l1 = _lists.tail(&b6_l0, data::type_::IntListTypeId {
                        list_type: data::type_::ListTypeId(0),
                    }, 1);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b6_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b6_l0, b6_l1]);
                    CompiledResume::Next(10)
                }

                fn int_list_int_1_resume_10(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let _list1 = values.int_lists.remove(1);
                    let _list0 = values.int_lists.remove(0);
                    let (b6_i0, b6_l0, b6_l1,) = (values.ints[0], _list0, _list1,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b6_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b6_l0, b6_l1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(10));
                    }
                    *budget -= 1;
                    {
                        let _next = (b6_i0, b6_l1,);
                        drop(b6_l0);
                        let (b0_i0, b0_l0,) = _next;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b0_l0]);
                        CompiledResume::Next(0)
                    }
                }

                fn int_list_int_1_resume_11(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let _list0 = values.int_lists.remove(0);
                    let (b7_i0, b7_l0,) = (values.ints[0], _list0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b7_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b7_l0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(11));
                    }
                    *budget -= 1;
                    {
                        let (b6_i0, b6_l0,) = (b7_i0, b7_l0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b6_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b6_l0]);
                        CompiledResume::Next(9)
                    }
                }

                fn int_list_int_3(
                    point: usize,
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {

                    const RESUME: [
                        fn(&mut data::compiled::int_list::IntListValues, &data::compiled::int_list::IntListOps<'_>, &mut usize) -> CompiledResume;
                        4
                    ] = [
                        |values, _lists, budget| {
                            let _list0 = values.int_lists.remove(0);
                            CompiledResume::Exit(int_list_int_3_entry((values.ints[0], _list0,), values, _lists, budget))
                        },
                        int_list_int_3_resume_1,
                        int_list_int_3_resume_2,
                        int_list_int_3_resume_3,
                    ];

                    let mut point = point;
                    loop {
                        match RESUME[point](values, _lists, budget) {
                            CompiledResume::Next(next) => point = next,
                            CompiledResume::Exit(progress) => return progress,
                        }
                    }
                }

                fn int_list_int_3_entry(
                    inputs: (i128, data::compiled::int_list::IntList,),
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {
                    let (mut b0_i0, mut b0_l0,) = inputs;
                    'repeat: loop {
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b0_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.int_lists.clear();
                            values.int_lists.extend([b0_l0]);
                            return data::compiled::CompiledProgress::Yield(0);
                        }
                        *budget -= 1;
                        if b0_l0.is_empty() {
                            let _next = (b0_i0,);
                            drop(b0_l0);
                            let (b1_i0,) = _next;
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b1_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([]);
                                return data::compiled::CompiledProgress::Yield(1);
                            }
                            *budget -= 1;

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.int_lists.clear();
                            values.int_lists.extend([]);
                            return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0));
                        } else {
                            let (b2_i0, b2_l0,) = (b0_i0, b0_l0,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([b2_l0]);
                                return data::compiled::CompiledProgress::Yield(2);
                            }
                            *budget -= 1;
                            let b2_i1 = b2_i0 + 1_i128;
                            if b2_i1 < i128::from(i64::MIN) || b2_i1 > i128::from(i64::MAX) {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([b2_l0]);
                                return data::compiled::CompiledProgress::Interpreted(3);
                            }
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([b2_l0]);
                                return data::compiled::CompiledProgress::Yield(3);
                            }
                            *budget -= 1;
                            {
                                (b0_i0, b0_l0,) = (b2_i1, b2_l0,);
                                continue 'repeat;
                            }
                        }
                    }
                }

                fn int_list_int_3_resume_1(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_i0,) = (values.ints[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
                }

                fn int_list_int_3_resume_2(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let _list0 = values.int_lists.remove(0);
                    let (b2_i0, b2_l0,) = (values.ints[0], _list0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b2_l0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                    }
                    *budget -= 1;
                    let b2_i1 = b2_i0 + 1_i128;
                    if b2_i1 < i128::from(i64::MIN) || b2_i1 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b2_l0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(3));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b2_l0]);
                    CompiledResume::Next(3)
                }

                fn int_list_int_3_resume_3(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let _list0 = values.int_lists.remove(0);
                    let (b2_i0, b2_i1, b2_l0,) = (values.ints[0], values.ints[1], _list0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b2_l0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                    }
                    *budget -= 1;
                    {
                        let (b0_i0, b0_l0,) = (b2_i1, b2_l0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b0_l0]);
                        CompiledResume::Next(0)
                    }
                }

                fn int_list_int_4(
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
                            let _list0 = values.int_lists.remove(0);
                            CompiledResume::Exit(int_list_int_4_entry((values.ints[0], values.ints[1], _list0,), values, _lists, budget))
                        },
                        int_list_int_4_resume_1,
                        int_list_int_4_resume_2,
                        int_list_int_4_resume_3,
                        int_list_int_4_resume_4,
                        int_list_int_4_resume_5,
                        int_list_int_4_resume_6,
                    ];

                    let mut point = point;
                    loop {
                        match RESUME[point](values, _lists, budget) {
                            CompiledResume::Next(next) => point = next,
                            CompiledResume::Exit(progress) => return progress,
                        }
                    }
                }

                fn int_list_int_4_entry(
                    inputs: (i128, i128, data::compiled::int_list::IntList,),
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {
                    let (b0_i0, b0_i1, b0_l0,) = inputs;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b0_l0]);
                        return data::compiled::CompiledProgress::Yield(0);
                    }
                    let _matched = if !b0_l0.is_empty() {
                        'pattern: {
                            let mut _reader = _lists.prefix(&b0_l0, 1);
                            let Some(_head0) = _reader.next() else {
                                break 'pattern Ok(None);
                            };
                            let Some(_small0) = _head0.small() else {
                                break 'pattern Err(());
                            };
                            let m0 = _small0;
                            break 'pattern Ok(Some((m0,)));
                        }
                    } else {
                        Ok(None)
                    };
                    match _matched {
                        Ok(Some((m0,))) => {
                            *budget -= 1;
                            let _next = (b0_i0, b0_i1, m0,);
                            drop(b0_l0);
                            let (b1_i0, b1_i1, b1_i2,) = _next;
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([]);
                                return data::compiled::CompiledProgress::Yield(1);
                            }
                            *budget -= 1;
                            if b1_i0 < 0_i128 {
                                let (b2_i0, b2_i1,) = (b1_i1, b1_i2,);
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.int_lists.clear();
                                    values.int_lists.extend([]);
                                    return data::compiled::CompiledProgress::Yield(2);
                                }
                                *budget -= 1;
                                let b2_i2 = b2_i1 - b2_i0;
                                if b2_i2 < i128::from(i64::MIN) || b2_i2 > i128::from(i64::MAX) {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.int_lists.clear();
                                    values.int_lists.extend([]);
                                    return data::compiled::CompiledProgress::Interpreted(3);
                                }
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.int_lists.clear();
                                    values.int_lists.extend([]);
                                    return data::compiled::CompiledProgress::Yield(3);
                                }
                                *budget -= 1;

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([]);
                                data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                            } else {
                                let (b3_i0, b3_i1, b3_i2,) = (b1_i0, b1_i1, b1_i2,);
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.int_lists.clear();
                                    values.int_lists.extend([]);
                                    return data::compiled::CompiledProgress::Yield(4);
                                }
                                *budget -= 1;
                                let _r0_n0 = b3_i2 + b3_i1;
                                let _r0_n1 = _r0_n0 + b3_i0;
                                let b3_i3 = _r0_n1;
                                if b3_i3 < i128::from(i64::MIN) || b3_i3 > i128::from(i64::MAX) {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.int_lists.clear();
                                    values.int_lists.extend([]);
                                    return data::compiled::CompiledProgress::Interpreted(5);
                                }
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.int_lists.clear();
                                    values.int_lists.extend([]);
                                    return data::compiled::CompiledProgress::Yield(5);
                                }
                                *budget -= 1;

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([]);
                                data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                            }
                        },
                        Ok(None) => {
                            *budget -= 1;
                            let (b4_l0,) = (b0_l0,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([b4_l0]);
                                return data::compiled::CompiledProgress::Yield(6);
                            }

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.int_lists.clear();
                            values.int_lists.extend([b4_l0]);
                            data::compiled::CompiledProgress::Interpreted(6)
                        },
                        Err(()) => {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.int_lists.clear();
                            values.int_lists.extend([b0_l0]);
                            data::compiled::CompiledProgress::Interpreted(0)
                        }
                    }
                }

                fn int_list_int_4_resume_1(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_i0, b1_i1, b1_i2,) = (values.ints[0], values.ints[1], values.ints[2],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                    }
                    *budget -= 1;
                    if b1_i0 < 0_i128 {
                        let (b2_i0, b2_i1,) = (b1_i1, b1_i2,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([]);
                        CompiledResume::Next(2)
                    } else {
                        let (b3_i0, b3_i1, b3_i2,) = (b1_i0, b1_i1, b1_i2,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([]);
                        CompiledResume::Next(4)
                    }
                }

                fn int_list_int_4_resume_2(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b2_i0, b2_i1,) = (values.ints[0], values.ints[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                    }
                    *budget -= 1;
                    let b2_i2 = b2_i1 - b2_i0;
                    if b2_i2 < i128::from(i64::MIN) || b2_i2 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(3));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([]);
                    CompiledResume::Next(3)
                }

                fn int_list_int_4_resume_3(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b2_i0, b2_i1, b2_i2,) = (values.ints[0], values.ints[1], values.ints[2],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
                }

                fn int_list_int_4_resume_4(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b3_i0, b3_i1, b3_i2,) = (values.ints[0], values.ints[1], values.ints[2],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                    }
                    *budget -= 1;
                    let _r0_n0 = b3_i2 + b3_i1;
                    let _r0_n1 = _r0_n0 + b3_i0;
                    let b3_i3 = _r0_n1;
                    if b3_i3 < i128::from(i64::MIN) || b3_i3 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(5));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([]);
                    CompiledResume::Next(5)
                }

                fn int_list_int_4_resume_5(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b3_i0, b3_i1, b3_i2, b3_i3,) = (values.ints[0], values.ints[1], values.ints[2], values.ints[3],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
                }

                fn int_list_int_4_resume_6(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let _list0 = values.int_lists.remove(0);
                    let (b4_l0,) = (_list0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b4_l0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b4_l0]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(6))
                }
                data::compiled::CompiledFunctions {
                    ints: data::Storage::Static(&[
                        data::compiled::CompiledFunction {
                            function: data::function::IntFunctionId(1),
                            implementation: data::compiled::CompiledImplementation::IntList(data::compiled::IntListImplementation {
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 1,
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
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 1,
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
                                        int_lists: 1,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(3),
                                        instruction: 1,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 1,
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
                                        int_lists: 1,
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
                                        int_lists: 2,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(4),
                                        instruction: 2,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 2,
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
                                        int_lists: 1,
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
                                        int_lists: 1,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(6),
                                        instruction: 1,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 2,
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
                                        int_lists: 1,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                run: int_list_int_1,
                            }),
                        },
                        data::compiled::CompiledFunction {
                            function: data::function::IntFunctionId(3),
                            implementation: data::compiled::CompiledImplementation::IntList(data::compiled::IntListImplementation {
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 1,
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
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 1,
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
                                        int_lists: 1,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                run: int_list_int_3,
                            }),
                        },
                        data::compiled::CompiledFunction {
                            function: data::function::IntFunctionId(4),
                            implementation: data::compiled::CompiledImplementation::IntList(data::compiled::IntListImplementation {
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 1,
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
                                        block: data::graph::BlockId(3),
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
                                        block: data::graph::BlockId(3),
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
                                        block: data::graph::BlockId(4),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 1,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                run: int_list_int_4,
                            }),
                        },
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
                                        int_lists: 1,
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
                                        int_lists: 1,
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
                                        int_lists: 1,
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
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 1,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
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
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
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
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
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
                                        point: 2,
                                        output: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        target: data::compiled::CallContractTarget::IntValue(data::graph::IntFunctionLocalId(0)),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("example", "captured", data::source::SourceSpan::new(1830, 1842)),
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
                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(4)),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                        reference: false,
                                        captures: data::Storage::Static(&[
                                            data::graph::FunctionCapture::IntList {
                                                target: data::graph::IntListLocalId(0),
                                                source: data::graph::IntListLocalId(0),
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
                                        point: 3,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: CALL_GROUP_0[0],
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
                    0..5,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    5..6,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
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
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 0..2,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 2..4,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 4..6,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
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
                        captures: data::Storage::Static(&[
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
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                shape: data::type_::ValueShapeId(0),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 7..10,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(2),
                        ]),
                        return_: data::type_::ValueShapeId(3),
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
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                        local: data::graph::IntListLocalId(0),
                        type_id: data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
                        },
                    }),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                        local: data::graph::IntListLocalId(0),
                        type_id: data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
                        },
                    }),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                        local: data::graph::IntListLocalId(0),
                        type_id: data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
                        },
                    }),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
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
                lifetimes: data::Storage::Static(&[
                    data::host::HostValueLifetime::LoadedOwner,
                ]),
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
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(0)),
                    data::type_::ValueShapeDescriptor::String,
                    data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                        data::type_::ValueShapeId(2),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(1),
                        data::type_::ValueShapeId(0),
                    ])),
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                    },
                ]),
                shape_types: data::Storage::Static(&[
                    data::type_::ValueType::Int,
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                    data::type_::ValueType::String,
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::String,
                        data::type_::ValueType::Int,
                        data::type_::ValueType::List(data::type_::ListTypeId(0)),
                        data::type_::ValueType::Int,
                    ])),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    }),
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
                            ints: data::Storage::Static(&[
                                data::type_::IntListTypeId {
                                    list_type: data::type_::ListTypeId(0),
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
                    },
                    callables: data::Storage::Static(&[]),
                },
            ]),
            lists: data::Storage::Static(&[]),
            functions: data::Storage::Static(&[]),
        },
        exports: data::Storage::Static(&[
            data::Export {
                name: data::Text::Static("caller"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::String,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                        data::type_::TypeMetadata::Int,
                    ]))),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("running"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 0,
            },
        ]),
    },
    value_functions: data::Storage::Static(&[]),
    never_functions: data::Storage::Static(&[]),
    callables: data::Storage::Static(&[]),
}
