data::HostedEntryArtifact {
    format: 22,
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
        main: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Int(data::function::IntFunctionId(0))),
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
                                        instructions: 0..19,
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
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
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
                                            data::graph::IntLocalId(1),
                                            data::graph::IntLocalId(2),
                                        ])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
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
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2527, 2546)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
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
                                                local: data::graph::IntListLocalId(1),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Int(data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                            data::graph::IntLocalId(5),
                                            data::graph::IntLocalId(6),
                                        ])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(2),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(1),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2549, 2568)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(4)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(8)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
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
                                                local: data::graph::IntListLocalId(2),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Int(data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                            data::graph::IntLocalId(10),
                                            data::graph::IntLocalId(11),
                                        ])))),
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
                                        }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::NoSign,
                                            digits: data::Storage::Static(&[]),
                                        })),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(3),
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
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2571, 2596)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(9)),
                                            right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(13)),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(14)),
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
                                                tail: Some(data::graph::MatchPatternListTail::Bind(data::graph::MatchPatternBinding {
                                                    index: 1,
                                                })),
                                            }),
                                            success: data::graph::MatchEdge {
                                                target: data::graph::BlockId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                    data::graph::MatchEdgeArgument::Binding(1),
                                                    data::graph::MatchEdgeArgument::Binding(0),
                                                ]),
                                                bindings: data::Storage::Static(&[
                                                    0,
                                                    1,
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
                                            failure: data::graph::Edge {
                                                target: data::graph::BlockId(7),
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
                                        params: 5..8,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::IntSwitch(data::graph::IntSwitch {
                                            subject: data::graph::IntLocalId(1),
                                            clauses: data::Storage::Static(&[
                                                (data::graph::IntegerLiteral {
                                                    sign: data::Sign::Plus,
                                                    digits: data::Storage::Static(&[
                                                        1,
                                                    ]),
                                                }, data::graph::Edge {
                                                    target: data::graph::BlockId(4),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                                                length: 1,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                }),
                                            ]),
                                            fallback: data::graph::Edge {
                                                target: data::graph::BlockId(6),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                                            length: 1,
                                                            steps: data::Storage::Static(&[]),
                                                        },
                                                    ]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 8..10,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(5),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                        params: 10..12,
                                        instructions: 1..1,
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
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 12..14,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(5),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                        params: 14..15,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "asserted", data::source::SourceSpan::new(277, 287)),
                                            pattern_span: data::source::SourceSpan::new(288, 302),
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
                                            test: data::graph::BoolTest::Equal {
                                                left: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                                right: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(1),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
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
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(1),
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
                                        params: 3..4,
                                        instructions: 0..0,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 4..7,
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
                                                target: data::graph::BlockId(3),
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
                                                target: data::graph::BlockId(4),
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
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 8..11,
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
                                                target: data::graph::BlockId(5),
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
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                            false_: data::graph::Edge {
                                                target: data::graph::BlockId(9),
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
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 11..14,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                            test: data::graph::BoolTest::EqualInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            },
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(6),
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
                                                target: data::graph::BlockId(7),
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
                                        params: 14..17,
                                        instructions: 1..3,
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
                                                            length: 2,
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
                                        params: 17..20,
                                        instructions: 3..3,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(8),
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
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                transfer: data::graph::Transfer {
                                                    families: data::Storage::Static(&[]),
                                                },
                                            },
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 20..23,
                                        instructions: 3..4,
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
                                                            length: 2,
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
                                        params: 23..26,
                                        instructions: 4..4,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(8),
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
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
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
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
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
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
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
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
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
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
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
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
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
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
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
                                        local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
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
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                            left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            right: data::graph::IntegerOperand::Immediate(1),
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
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
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
            use data::compiled::calls::{BoolCallable, CallArguments, CallExecution, CallInputs, CallInteger, CallOps, CallOutput, CallProgress, CallStorage, CallValues, IntCallable};
            use data::compiled::int_list::IntList;
            enum FunctionState {
                Int0Point0 {  },
                Int0Point1 { int0: i128 },
                Int0Point2 { int0: i128, int1: i128 },
                Int0Point3 { int0: i128, int1: i128, int2: i128 },
                Int0Point4 { int0: i128, int1: i128, int2: i128, int_list0: IntList },
                Int0Point5 { int0: i128, int1: i128, int2: i128, int_list0: IntList, int3: i128 },
                Int0Point6 { int0: i128, int1: i128, int2: i128, int_list0: IntList, int3: i128, int4: i128 },
                Int0Point7 { int0: i128, int1: i128, int2: i128, int_list0: IntList, int3: i128, int4: i128, int5: i128 },
                Int0Point8 { int0: i128, int1: i128, int2: i128, int_list0: IntList, int3: i128, int4: i128, int5: i128, int6: i128 },
                Int0Point9 { int0: i128, int1: i128, int2: i128, int_list0: IntList, int3: i128, int4: i128, int5: i128, int6: i128, int_list1: IntList },
                Int0Point10 { int0: i128, int1: i128, int2: i128, int_list0: IntList, int3: i128, int4: i128, int5: i128, int6: i128, int_list1: IntList, int7: i128 },
                Int0Point11 { int0: i128, int1: i128, int2: i128, int_list0: IntList, int3: i128, int4: i128, int5: i128, int6: i128, int_list1: IntList, int7: i128, int8: i128 },
                Int0Point12 { int0: i128, int1: i128, int2: i128, int_list0: IntList, int3: i128, int4: i128, int5: i128, int6: i128, int_list1: IntList, int7: i128, int8: i128, int9: i128 },
                Int0Point13 { int0: i128, int1: i128, int2: i128, int_list0: IntList, int3: i128, int4: i128, int5: i128, int6: i128, int_list1: IntList, int7: i128, int8: i128, int9: i128, int10: i128 },
                Int0Point14 { int0: i128, int1: i128, int2: i128, int_list0: IntList, int3: i128, int4: i128, int5: i128, int6: i128, int_list1: IntList, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128 },
                Int0Point15 { int0: i128, int1: i128, int2: i128, int_list0: IntList, int3: i128, int4: i128, int5: i128, int6: i128, int_list1: IntList, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, int_list2: IntList },
                Int0Point16 { int0: i128, int1: i128, int2: i128, int_list0: IntList, int3: i128, int4: i128, int5: i128, int6: i128, int_list1: IntList, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, int_list2: IntList, int_list3: IntList },
                Int0Point17 { int0: i128, int1: i128, int2: i128, int_list0: IntList, int3: i128, int4: i128, int5: i128, int6: i128, int_list1: IntList, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, int_list2: IntList, int_list3: IntList, int12: i128 },
                Int0Point18 { int0: i128, int1: i128, int2: i128, int_list0: IntList, int3: i128, int4: i128, int5: i128, int6: i128, int_list1: IntList, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, int_list2: IntList, int_list3: IntList, int12: i128, int13: i128 },
                Int0Point19 { int0: i128, int1: i128, int2: i128, int_list0: IntList, int3: i128, int4: i128, int5: i128, int6: i128, int_list1: IntList, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, int_list2: IntList, int_list3: IntList, int12: i128, int13: i128, int14: i128 },
                Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: CallValues },
            }
            enum IntReturn {
                Int0Call5 { int0: i128, int1: i128, int2: i128, int_list0: IntList, int3: i128 },
                Int0Call10 { int0: i128, int1: i128, int2: i128, int_list0: IntList, int3: i128, int4: i128, int5: i128, int6: i128, int_list1: IntList, int7: i128 },
                Int0Call17 { int0: i128, int1: i128, int2: i128, int_list0: IntList, int3: i128, int4: i128, int5: i128, int6: i128, int_list1: IntList, int7: i128, int8: i128, int9: i128, int10: i128, int11: i128, int_list2: IntList, int_list3: IntList, int12: i128 },
            }
            impl IntReturn {
                fn site(&self) -> data::source::HostCallSite {
                    match *self {
                        Self::Int0Call5 { .. } => data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2527, 2546)),
                        Self::Int0Call10 { .. } => data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2549, 2568)),
                        Self::Int0Call17 { .. } => data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2571, 2596)),
                    }
                }
                fn small(self, result: i128) -> FunctionState {
                    match self {
                        Self::Int0Call5 { int0, int1, int2, int_list0, int3 } => {
                            let int4 = result;
                            FunctionState::Int0Point6 { int0, int1, int2, int_list0, int3, int4 }
                        },
                        Self::Int0Call10 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7 } => {
                            let int8 = result;
                            FunctionState::Int0Point11 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8 }
                        },
                        Self::Int0Call17 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10, int11, int_list2, int_list3, int12 } => {
                            let int13 = result;
                            FunctionState::Int0Point18 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10, int11, int_list2, int_list3, int12, int13 }
                        },
                    }
                }
                fn resume(self, result: CallInteger) -> FunctionState {
                    if let Some(result) = result.small() {
                        return self.small(result);
                    }
                    match self {
                        Self::Int0Call5 { int0, int1, int2, int_list0, int3 } => {
                            let int4 = result;
                            FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 6,
                                ints: 5,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 1,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] } }
                        },
                        Self::Int0Call10 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7 } => {
                            let int8 = result;
                            FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 11,
                                ints: 9,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 2,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8], bools: vec![], int_lists: vec![int_list0, int_list1], int_functions: vec![], bool_functions: vec![] } }
                        },
                        Self::Int0Call17 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10, int11, int_list2, int_list3, int12 } => {
                            let int13 = result;
                            FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 18,
                                ints: 14,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 4,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13], bools: vec![], int_lists: vec![int_list0, int_list1, int_list2, int_list3], int_functions: vec![], bool_functions: vec![] } }
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
                Int { value: i128, exit: data::graph::BlockGraphExitId },
                IntBridge { function: data::function::IntFunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: IntReturn },
            }
            struct FunctionExecution {
                active: Option<FunctionState>,
                integer_returns: Vec<IntReturn>,
                boolean_returns: Vec<BoolReturn>,
                integer_function_returns: Vec<IntFunctionReturn>,
                boolean_function_returns: Vec<BoolFunctionReturn>,
            }
            impl FunctionExecution {
                fn new(active: FunctionState) -> Self {
                    Self {
                        active: Some(active),
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
                        data::compiled::CallTarget::Int(data::function::IntFunctionId(0)) => calls_int_0_state(point, values),
                        _ => None,
                    };
                    let Some(active) = active else { return false; };
                    self.active = Some(active);
                    true
                }
                fn retained_bytes(&self) -> usize {
                    std::mem::size_of::<Self>() + self.integer_returns.capacity() * std::mem::size_of::<IntReturn>() + self.boolean_returns.capacity() * std::mem::size_of::<BoolReturn>() + self.integer_function_returns.capacity() * std::mem::size_of::<IntFunctionReturn>() + self.boolean_function_returns.capacity() * std::mem::size_of::<BoolFunctionReturn>()
                }
                fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                    let Some(mut active) = self.active.take() else { return CallProgress::Yield(self); };
                    loop {
                        match function_step(active, ops, budget) {
                            FunctionStep::Next(next) => active = next,
                            FunctionStep::Yield(active) => {
                                self.active = Some(active);
                                return CallProgress::Yield(self);
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
                                        return CallProgress::Interpreted { point, values };
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
                                        return CallProgress::Interpreted { point, values };
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
                                        return CallProgress::Interpreted { point, values };
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
                                        return CallProgress::Interpreted { point, values };
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
                    FunctionState::Int0Point0 {  } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point0 {  }); }
                        *budget -= 1;
                        let int0 = 1_i128;
                        if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
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
                        FunctionStep::Next(FunctionState::Int0Point1 { int0 })
                    },
                    FunctionState::Int0Point1 { int0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point1 { int0 }); }
                        *budget -= 1;
                        let int1 = 0_i128;
                        if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
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
                        FunctionStep::Next(FunctionState::Int0Point2 { int0, int1 })
                    },
                    FunctionState::Int0Point2 { int0, int1 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point2 { int0, int1 }); }
                        *budget -= 1;
                        let int2 = 1_i128;
                        if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
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
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int0Point3 { int0, int1, int2 })
                    },
                    FunctionState::Int0Point3 { int0, int1, int2 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point3 { int0, int1, int2 }); }
                        *budget -= 1;
                        let int_list0 = ops.lists().value(data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
                        }, &[int0 as i64, int1 as i64, int2 as i64]);
                        FunctionStep::Next(FunctionState::Int0Point4 { int0, int1, int2, int_list0 })
                    },
                    FunctionState::Int0Point4 { int0, int1, int2, int_list0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point4 { int0, int1, int2, int_list0 }); }
                        *budget -= 1;
                        let int3 = 5_i128;
                        if int3 < i128::from(i64::MIN) || int3 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 5,
                            ints: 4,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 1,
                            strings: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int0Point5 { int0, int1, int2, int_list0, int3 })
                    },
                    FunctionState::Int0Point5 { int0, int1, int2, int_list0, int3 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point5 { int0, int1, int2, int_list0, int3 }); }
                        *budget -= 1;
                        FunctionStep::IntBridge { function: data::function::IntFunctionId(1), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2527, 2546)), arguments: CallArguments { values: CallValues { ints: vec![int3.into()], bools: vec![], int_lists: vec![int_list0.clone()], int_functions: vec![], bool_functions: vec![] }, captures: None }, caller: IntReturn::Int0Call5 { int0, int1, int2, int_list0, int3 } }
                    },
                    FunctionState::Int0Point6 { int0, int1, int2, int_list0, int3, int4 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point6 { int0, int1, int2, int_list0, int3, int4 }); }
                        *budget -= 1;
                        let int5 = 0_i128;
                        if int5 < i128::from(i64::MIN) || int5 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 7,
                            ints: 6,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 1,
                            strings: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int0Point7 { int0, int1, int2, int_list0, int3, int4, int5 })
                    },
                    FunctionState::Int0Point7 { int0, int1, int2, int_list0, int3, int4, int5 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point7 { int0, int1, int2, int_list0, int3, int4, int5 }); }
                        *budget -= 1;
                        let int6 = 1_i128;
                        if int6 < i128::from(i64::MIN) || int6 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 8,
                            ints: 7,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 1,
                            strings: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int0Point8 { int0, int1, int2, int_list0, int3, int4, int5, int6 })
                    },
                    FunctionState::Int0Point8 { int0, int1, int2, int_list0, int3, int4, int5, int6 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point8 { int0, int1, int2, int_list0, int3, int4, int5, int6 }); }
                        *budget -= 1;
                        let int_list1 = ops.lists().value(data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
                        }, &[int5 as i64, int6 as i64]);
                        FunctionStep::Next(FunctionState::Int0Point9 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1 })
                    },
                    FunctionState::Int0Point9 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point9 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1 }); }
                        *budget -= 1;
                        let int7 = 2_i128;
                        if int7 < i128::from(i64::MIN) || int7 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 10,
                            ints: 8,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 2,
                            strings: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into()], bools: vec![], int_lists: vec![int_list0, int_list1], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int0Point10 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7 })
                    },
                    FunctionState::Int0Point10 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point10 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7 }); }
                        *budget -= 1;
                        FunctionStep::IntBridge { function: data::function::IntFunctionId(2), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2549, 2568)), arguments: CallArguments { values: CallValues { ints: vec![int7.into()], bools: vec![], int_lists: vec![int_list1.clone()], int_functions: vec![], bool_functions: vec![] }, captures: None }, caller: IntReturn::Int0Call10 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7 } }
                    },
                    FunctionState::Int0Point11 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point11 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8 }); }
                        *budget -= 1;
                        let int9 = int4 + int8;
                        if int9 < i128::from(i64::MIN) || int9 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 12,
                            ints: 10,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 2,
                            strings: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into()], bools: vec![], int_lists: vec![int_list0, int_list1], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int0Point12 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9 })
                    },
                    FunctionState::Int0Point12 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point12 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9 }); }
                        *budget -= 1;
                        let int10 = 1_i128;
                        if int10 < i128::from(i64::MIN) || int10 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 13,
                            ints: 11,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 2,
                            strings: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into()], bools: vec![], int_lists: vec![int_list0, int_list1], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int0Point13 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10 })
                    },
                    FunctionState::Int0Point13 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point13 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10 }); }
                        *budget -= 1;
                        let int11 = 1_i128;
                        if int11 < i128::from(i64::MIN) || int11 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 14,
                            ints: 12,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 2,
                            strings: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into()], bools: vec![], int_lists: vec![int_list0, int_list1], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int0Point14 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10, int11 })
                    },
                    FunctionState::Int0Point14 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10, int11 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point14 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10, int11 }); }
                        *budget -= 1;
                        let int_list2 = ops.lists().value(data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
                        }, &[int10 as i64, int11 as i64]);
                        FunctionStep::Next(FunctionState::Int0Point15 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10, int11, int_list2 })
                    },
                    FunctionState::Int0Point15 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10, int11, int_list2 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point15 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10, int11, int_list2 }); }
                        *budget -= 1;
                        let int_list3 = ops.lists().value(data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
                        }, &[]);
                        FunctionStep::Next(FunctionState::Int0Point16 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10, int11, int_list2, int_list3 })
                    },
                    FunctionState::Int0Point16 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10, int11, int_list2, int_list3 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point16 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10, int11, int_list2, int_list3 }); }
                        *budget -= 1;
                        let int12 = 0_i128;
                        if int12 < i128::from(i64::MIN) || int12 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 17,
                            ints: 13,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 4,
                            strings: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into()], bools: vec![], int_lists: vec![int_list0, int_list1, int_list2, int_list3], int_functions: vec![], bool_functions: vec![] } }; }
                        FunctionStep::Next(FunctionState::Int0Point17 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10, int11, int_list2, int_list3, int12 })
                    },
                    FunctionState::Int0Point17 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10, int11, int_list2, int_list3, int12 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point17 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10, int11, int_list2, int_list3, int12 }); }
                        *budget -= 1;
                        FunctionStep::IntBridge { function: data::function::IntFunctionId(3), site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2571, 2596)), arguments: CallArguments { values: CallValues { ints: vec![int12.into()], bools: vec![], int_lists: vec![int_list2.clone(), int_list3.clone()], int_functions: vec![], bool_functions: vec![] }, captures: None }, caller: IntReturn::Int0Call17 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10, int11, int_list2, int_list3, int12 } }
                    },
                    FunctionState::Int0Point18 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10, int11, int_list2, int_list3, int12, int13 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point18 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10, int11, int_list2, int_list3, int12, int13 }); }
                        *budget -= 1;
                        let int14 = int9 + int13;
                        if int14 < i128::from(i64::MIN) || int14 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                            block: data::graph::BlockId(0),
                            instruction: 19,
                            ints: 15,
                            bools: 0,
                            bit_arrays: 0,
                            int_lists: 4,
                            strings: 0,
                            customs: 0,
                            custom_lists: 0,
                            int_functions: 0,
                            bool_functions: 0,
                        }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into(), int4.into(), int5.into(), int6.into(), int7.into(), int8.into(), int9.into(), int10.into(), int11.into(), int12.into(), int13.into(), int14.into()], bools: vec![], int_lists: vec![int_list0, int_list1, int_list2, int_list3], int_functions: vec![], bool_functions: vec![] } }; }
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point19 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10, int11, int_list2, int_list3, int12, int13, int14 }); }
                        *budget -= 1;
                        FunctionStep::Int { value: int14, exit: data::graph::BlockGraphExitId(0) }
                    },
                    FunctionState::Int0Point19 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10, int11, int_list2, int_list3, int12, int13, int14 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point19 { int0, int1, int2, int_list0, int3, int4, int5, int6, int_list1, int7, int8, int9, int10, int11, int_list2, int_list3, int12, int13, int14 }); }
                        *budget -= 1;
                        FunctionStep::Int { value: int14, exit: data::graph::BlockGraphExitId(0) }
                    },
                }
            }
            fn calls_int_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                let active = match point {
                    0 => FunctionState::Int0Point0 {  },
                    1 => FunctionState::Int0Point1 { int0: values.int(0)? },
                    2 => FunctionState::Int0Point2 { int0: values.int(0)?, int1: values.int(1)? },
                    3 => FunctionState::Int0Point3 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                    4 => FunctionState::Int0Point4 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int_list0: values.int_list(0)? },
                    5 => FunctionState::Int0Point5 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int_list0: values.int_list(0)?, int3: values.int(3)? },
                    6 => FunctionState::Int0Point6 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int_list0: values.int_list(0)?, int3: values.int(3)?, int4: values.int(4)? },
                    7 => FunctionState::Int0Point7 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int_list0: values.int_list(0)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)? },
                    8 => FunctionState::Int0Point8 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int_list0: values.int_list(0)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)? },
                    9 => FunctionState::Int0Point9 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int_list0: values.int_list(0)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int_list1: values.int_list(1)? },
                    10 => FunctionState::Int0Point10 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int_list0: values.int_list(0)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int_list1: values.int_list(1)?, int7: values.int(7)? },
                    11 => FunctionState::Int0Point11 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int_list0: values.int_list(0)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int_list1: values.int_list(1)?, int7: values.int(7)?, int8: values.int(8)? },
                    12 => FunctionState::Int0Point12 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int_list0: values.int_list(0)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int_list1: values.int_list(1)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)? },
                    13 => FunctionState::Int0Point13 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int_list0: values.int_list(0)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int_list1: values.int_list(1)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)? },
                    14 => FunctionState::Int0Point14 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int_list0: values.int_list(0)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int_list1: values.int_list(1)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)? },
                    15 => FunctionState::Int0Point15 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int_list0: values.int_list(0)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int_list1: values.int_list(1)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, int_list2: values.int_list(2)? },
                    16 => FunctionState::Int0Point16 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int_list0: values.int_list(0)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int_list1: values.int_list(1)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, int_list2: values.int_list(2)?, int_list3: values.int_list(3)? },
                    17 => FunctionState::Int0Point17 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int_list0: values.int_list(0)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int_list1: values.int_list(1)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, int_list2: values.int_list(2)?, int_list3: values.int_list(3)?, int12: values.int(12)? },
                    18 => FunctionState::Int0Point18 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int_list0: values.int_list(0)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int_list1: values.int_list(1)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, int_list2: values.int_list(2)?, int_list3: values.int_list(3)?, int12: values.int(12)?, int13: values.int(13)? },
                    19 => FunctionState::Int0Point19 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int_list0: values.int_list(0)?, int3: values.int(3)?, int4: values.int(4)?, int5: values.int(5)?, int6: values.int(6)?, int_list1: values.int_list(1)?, int7: values.int(7)?, int8: values.int(8)?, int9: values.int(9)?, int10: values.int(10)?, int11: values.int(11)?, int_list2: values.int_list(2)?, int_list3: values.int_list(3)?, int12: values.int(12)?, int13: values.int(13)?, int14: values.int(14)? },
                    _ => return None,
                };
                Some(active)
            }
            fn calls_int_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point, values) { return Some(execution); }
                let active = calls_int_0_state(point, values)?;
                Some(Box::new(FunctionExecution::new(active)))
            }

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

            fn int_list_int_2(
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
                        let _list0 = values.int_lists.remove(0);
                        CompiledResume::Exit(int_list_int_2_entry((values.ints[0], _list0,), values, _lists, budget))
                    },
                    int_list_int_2_resume_1,
                    int_list_int_2_resume_2,
                    int_list_int_2_resume_3,
                    int_list_int_2_resume_4,
                    int_list_int_2_resume_5,
                    int_list_int_2_resume_6,
                    int_list_int_2_resume_7,
                    int_list_int_2_resume_8,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, _lists, budget) {
                        CompiledResume::Next(next) => point = next,
                        CompiledResume::Exit(progress) => return progress,
                    }
                }
            }

            fn int_list_int_2_entry(
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
                        let _matched = if !b2_l0.is_empty() {
                            'pattern: {
                                let mut _reader = _lists.prefix(&b2_l0, 1);
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
                                let m1 = _lists.tail(&b2_l0, data::type_::IntListTypeId {
                                    list_type: data::type_::ListTypeId(0),
                                }, 1);
                                let _next = (b2_i0, m0, m1,);
                                drop(b2_l0);
                                let (b3_i0, b3_i1, b3_l0,) = _next;
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.int_lists.clear();
                                    values.int_lists.extend([b3_l0]);
                                    return data::compiled::CompiledProgress::Yield(3);
                                }
                                *budget -= 1;
                                let (b5_i0, b5_l0,) = if b3_i1 == 1_i128 {
                                    let (b4_i0, b4_l0,) = (b3_i0, b3_l0,);
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b4_i0]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.int_lists.clear();
                                        values.int_lists.extend([b4_l0]);
                                        return data::compiled::CompiledProgress::Yield(4);
                                    }
                                    *budget -= 1;
                                    let b4_i1 = b4_i0 + 1_i128;
                                    if b4_i1 < i128::from(i64::MIN) || b4_i1 > i128::from(i64::MAX) {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.int_lists.clear();
                                        values.int_lists.extend([b4_l0]);
                                        return data::compiled::CompiledProgress::Interpreted(5);
                                    }
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.int_lists.clear();
                                        values.int_lists.extend([b4_l0]);
                                        return data::compiled::CompiledProgress::Yield(5);
                                    }
                                    *budget -= 1;
                                    let (b5_i0, b5_l0,) = {
                                        (b4_i1, b4_l0,)
                                    };
                                    (b5_i0, b5_l0,)
                                } else {
                                    let (b6_i0, b6_l0,) = (b3_i0, b3_l0,);
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b6_i0]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.int_lists.clear();
                                        values.int_lists.extend([b6_l0]);
                                        return data::compiled::CompiledProgress::Yield(7);
                                    }
                                    *budget -= 1;
                                    let (b5_i0, b5_l0,) = {
                                        (b6_i0, b6_l0,)
                                    };
                                    (b5_i0, b5_l0,)
                                };
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b5_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.int_lists.clear();
                                    values.int_lists.extend([b5_l0]);
                                    return data::compiled::CompiledProgress::Yield(6);
                                }
                                *budget -= 1;
                                {
                                    (b0_i0, b0_l0,) = (b5_i0, b5_l0,);
                                    continue 'repeat;
                                }
                            },
                            Ok(None) => {
                                *budget -= 1;
                                let (b7_l0,) = (b2_l0,);
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.int_lists.clear();
                                    values.int_lists.extend([b7_l0]);
                                    return data::compiled::CompiledProgress::Yield(8);
                                }

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([b7_l0]);
                                return data::compiled::CompiledProgress::Interpreted(8);
                            },
                            Err(()) => {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([b2_l0]);
                                return data::compiled::CompiledProgress::Interpreted(2);
                            }
                        }
                    }
                }
            }

            fn int_list_int_2_resume_1(
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

            fn int_list_int_2_resume_2(
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
                let _matched = if !b2_l0.is_empty() {
                    'pattern: {
                        let mut _reader = _lists.prefix(&b2_l0, 1);
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
                        let m1 = _lists.tail(&b2_l0, data::type_::IntListTypeId {
                            list_type: data::type_::ListTypeId(0),
                        }, 1);
                        let _next = (b2_i0, m0, m1,);
                        drop(b2_l0);
                        let (b3_i0, b3_i1, b3_l0,) = _next;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b3_l0]);
                        CompiledResume::Next(3)
                    },
                    Ok(None) => {
                        *budget -= 1;
                        let (b7_l0,) = (b2_l0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b7_l0]);
                        CompiledResume::Next(8)
                    },
                    Err(()) => {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b2_l0]);
                        CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(2))
                    }
                }
            }

            fn int_list_int_2_resume_3(
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
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
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
                    CompiledResume::Next(4)
                } else {
                    let (b6_i0, b6_l0,) = (b3_i0, b3_l0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b6_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b6_l0]);
                    CompiledResume::Next(7)
                }
            }

            fn int_list_int_2_resume_4(
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
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;
                let b4_i1 = b4_i0 + 1_i128;
                if b4_i1 < i128::from(i64::MIN) || b4_i1 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b4_l0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(5));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([b4_l0]);
                CompiledResume::Next(5)
            }

            fn int_list_int_2_resume_5(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list0 = values.int_lists.remove(0);
                let (b4_i0, b4_i1, b4_l0,) = (values.ints[0], values.ints[1], _list0,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b4_l0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                *budget -= 1;
                {
                    let (b5_i0, b5_l0,) = (b4_i1, b4_l0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b5_l0]);
                    CompiledResume::Next(6)
                }
            }

            fn int_list_int_2_resume_6(
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
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;
                {
                    let (b0_i0, b0_l0,) = (b5_i0, b5_l0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b0_l0]);
                    CompiledResume::Next(0)
                }
            }

            fn int_list_int_2_resume_7(
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
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                }
                *budget -= 1;
                {
                    let (b5_i0, b5_l0,) = (b6_i0, b6_l0,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b5_l0]);
                    CompiledResume::Next(6)
                }
            }

            fn int_list_int_2_resume_8(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list0 = values.int_lists.remove(0);
                let (b7_l0,) = (_list0,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b7_l0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([b7_l0]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(8))
            }

            fn int_list_int_3(
                point: usize,
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::int_list::IntListValues, &data::compiled::int_list::IntListOps<'_>, &mut usize) -> CompiledResume;
                    14
                ] = [
                    |values, _lists, budget| {
                        let _list1 = values.int_lists.remove(1);
                        let _list0 = values.int_lists.remove(0);
                        CompiledResume::Exit(int_list_int_3_entry((values.ints[0], _list0, _list1,), values, _lists, budget))
                    },
                    int_list_int_3_resume_1,
                    int_list_int_3_resume_2,
                    int_list_int_3_resume_3,
                    int_list_int_3_resume_4,
                    int_list_int_3_resume_5,
                    int_list_int_3_resume_6,
                    int_list_int_3_resume_7,
                    int_list_int_3_resume_8,
                    int_list_int_3_resume_9,
                    int_list_int_3_resume_10,
                    int_list_int_3_resume_11,
                    int_list_int_3_resume_12,
                    int_list_int_3_resume_13,
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
                inputs: (i128, data::compiled::int_list::IntList, data::compiled::int_list::IntList,),
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let (mut b0_i0, mut b0_l0, mut b0_l1,) = inputs;
                'repeat: loop {
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b0_l0, b0_l1]);
                        return data::compiled::CompiledProgress::Yield(0);
                    }
                    *budget -= 1;
                    if _lists.equal(&b0_l0, &b0_l1) {
                        let _next = (b0_i0,);
                        drop(b0_l0);
                        drop(b0_l1);
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
                        let (b2_i0, b2_l0, b2_l1,) = (b0_i0, b0_l0, b0_l1,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.int_lists.clear();
                            values.int_lists.extend([b2_l0, b2_l1]);
                            return data::compiled::CompiledProgress::Yield(2);
                        }
                        *budget -= 1;
                        if b2_l0.is_empty() {
                            let _next = (b2_i0,);
                            drop(b2_l0);
                            drop(b2_l1);
                            let (b3_i0,) = _next;
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([]);
                                return data::compiled::CompiledProgress::Yield(3);
                            }
                            *budget -= 1;

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b3_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.int_lists.clear();
                            values.int_lists.extend([]);
                            return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1));
                        } else {
                            let (b4_i0, b4_l0, b4_l1,) = (b2_i0, b2_l0, b2_l1,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b4_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([b4_l0, b4_l1]);
                                return data::compiled::CompiledProgress::Yield(4);
                            }
                            *budget -= 1;
                            let (b8_i0, b8_l0, b8_l1,) = if !b4_l0.is_empty() {
                                let (b5_i0, b5_l0, b5_l1,) = (b4_i0, b4_l0, b4_l1,);
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b5_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.int_lists.clear();
                                    values.int_lists.extend([b5_l0, b5_l1]);
                                    return data::compiled::CompiledProgress::Yield(5);
                                }
                                let b5_i1 = match _lists.index(&b5_l0, 0) {
                                    Some(value) => value,
                                    None => {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b5_i0]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.int_lists.clear();
                                        values.int_lists.extend([b5_l0, b5_l1]);
                                        return data::compiled::CompiledProgress::Interpreted(5);
                                    }
                                };
                                *budget -= 1;
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.int_lists.clear();
                                    values.int_lists.extend([b5_l0, b5_l1]);
                                    return data::compiled::CompiledProgress::Yield(6);
                                }
                                *budget -= 1;
                                if b5_i1 == 1_i128 {
                                    let (b6_i0, b6_l0, b6_l1,) = (b5_i0, b5_l0, b5_l1,);
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b6_i0]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.int_lists.clear();
                                        values.int_lists.extend([b6_l0, b6_l1]);
                                        return data::compiled::CompiledProgress::Yield(7);
                                    }
                                    *budget -= 1;
                                    let b6_l2 = _lists.tail(&b6_l0, data::type_::IntListTypeId {
                                        list_type: data::type_::ListTypeId(0),
                                    }, 1);
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b6_i0]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.int_lists.clear();
                                        values.int_lists.extend([b6_l0, b6_l1, b6_l2]);
                                        return data::compiled::CompiledProgress::Yield(8);
                                    }
                                    *budget -= 1;
                                    let b6_i1 = b6_i0 + 1_i128;
                                    if b6_i1 < i128::from(i64::MIN) || b6_i1 > i128::from(i64::MAX) {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b6_i0, b6_i1]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.int_lists.clear();
                                        values.int_lists.extend([b6_l0, b6_l1, b6_l2]);
                                        return data::compiled::CompiledProgress::Interpreted(9);
                                    }
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b6_i0, b6_i1]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.int_lists.clear();
                                        values.int_lists.extend([b6_l0, b6_l1, b6_l2]);
                                        return data::compiled::CompiledProgress::Yield(9);
                                    }
                                    *budget -= 1;
                                    {
                                        let _next = (b6_i1, b6_l2, b6_l1,);
                                        drop(b6_l0);
                                        (b0_i0, b0_l0, b0_l1,) = _next;
                                        continue 'repeat;
                                    }
                                } else {
                                    let (b7_i0, b7_l0, b7_l1,) = (b5_i0, b5_l0, b5_l1,);
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b7_i0]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);
                                        values.int_lists.clear();
                                        values.int_lists.extend([b7_l0, b7_l1]);
                                        return data::compiled::CompiledProgress::Yield(10);
                                    }
                                    *budget -= 1;
                                    let (b8_i0, b8_l0, b8_l1,) = {
                                        (b7_i0, b7_l0, b7_l1,)
                                    };
                                    (b8_i0, b8_l0, b8_l1,)
                                }
                            } else {
                                let (b9_i0, b9_l0, b9_l1,) = (b4_i0, b4_l0, b4_l1,);
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b9_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.int_lists.clear();
                                    values.int_lists.extend([b9_l0, b9_l1]);
                                    return data::compiled::CompiledProgress::Yield(13);
                                }
                                *budget -= 1;
                                let (b8_i0, b8_l0, b8_l1,) = {
                                    (b9_i0, b9_l0, b9_l1,)
                                };
                                (b8_i0, b8_l0, b8_l1,)
                            };
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b8_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([b8_l0, b8_l1]);
                                return data::compiled::CompiledProgress::Yield(11);
                            }
                            *budget -= 1;
                            let b8_l2 = _lists.tail(&b8_l0, data::type_::IntListTypeId {
                                list_type: data::type_::ListTypeId(0),
                            }, 1);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b8_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([b8_l0, b8_l1, b8_l2]);
                                return data::compiled::CompiledProgress::Yield(12);
                            }
                            *budget -= 1;
                            {
                                let _next = (b8_i0, b8_l2, b8_l1,);
                                drop(b8_l0);
                                (b0_i0, b0_l0, b0_l1,) = _next;
                                continue 'repeat;
                            }
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
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                if b2_l0.is_empty() {
                    let _next = (b2_i0,);
                    drop(b2_l0);
                    drop(b2_l1);
                    let (b3_i0,) = _next;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([]);
                    CompiledResume::Next(3)
                } else {
                    let (b4_i0, b4_l0, b4_l1,) = (b2_i0, b2_l0, b2_l1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b4_l0, b4_l1]);
                    CompiledResume::Next(4)
                }
            }

            fn int_list_int_3_resume_3(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b3_i0,) = (values.ints[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b3_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
            }

            fn int_list_int_3_resume_4(
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
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                }
                *budget -= 1;
                if !b4_l0.is_empty() {
                    let (b5_i0, b5_l0, b5_l1,) = (b4_i0, b4_l0, b4_l1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b5_l0, b5_l1]);
                    CompiledResume::Next(5)
                } else {
                    let (b9_i0, b9_l0, b9_l1,) = (b4_i0, b4_l0, b4_l1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b9_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b9_l0, b9_l1]);
                    CompiledResume::Next(13)
                }
            }

            fn int_list_int_3_resume_5(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list1 = values.int_lists.remove(1);
                let _list0 = values.int_lists.remove(0);
                let (b5_i0, b5_l0, b5_l1,) = (values.ints[0], _list0, _list1,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b5_l0, b5_l1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                }
                let b5_i1 = match _lists.index(&b5_l0, 0) {
                    Some(value) => value,
                    None => {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b5_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b5_l0, b5_l1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(5));
                    }
                };
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([b5_l0, b5_l1]);
                CompiledResume::Next(6)
            }

            fn int_list_int_3_resume_6(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list1 = values.int_lists.remove(1);
                let _list0 = values.int_lists.remove(0);
                let (b5_i0, b5_i1, b5_l0, b5_l1,) = (values.ints[0], values.ints[1], _list0, _list1,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b5_l0, b5_l1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                }
                *budget -= 1;
                if b5_i1 == 1_i128 {
                    let (b6_i0, b6_l0, b6_l1,) = (b5_i0, b5_l0, b5_l1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b6_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b6_l0, b6_l1]);
                    CompiledResume::Next(7)
                } else {
                    let (b7_i0, b7_l0, b7_l1,) = (b5_i0, b5_l0, b5_l1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b7_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b7_l0, b7_l1]);
                    CompiledResume::Next(10)
                }
            }

            fn int_list_int_3_resume_7(
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
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                }
                *budget -= 1;
                let b6_l2 = _lists.tail(&b6_l0, data::type_::IntListTypeId {
                    list_type: data::type_::ListTypeId(0),
                }, 1);

                values.ints.clear();
                values.ints.extend_from_slice(&[b6_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([b6_l0, b6_l1, b6_l2]);
                CompiledResume::Next(8)
            }

            fn int_list_int_3_resume_8(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list2 = values.int_lists.remove(2);
                let _list1 = values.int_lists.remove(1);
                let _list0 = values.int_lists.remove(0);
                let (b6_i0, b6_l0, b6_l1, b6_l2,) = (values.ints[0], _list0, _list1, _list2,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b6_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b6_l0, b6_l1, b6_l2]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                }
                *budget -= 1;
                let b6_i1 = b6_i0 + 1_i128;
                if b6_i1 < i128::from(i64::MIN) || b6_i1 > i128::from(i64::MAX) {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b6_i0, b6_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b6_l0, b6_l1, b6_l2]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(9));
                }

                values.ints.clear();
                values.ints.extend_from_slice(&[b6_i0, b6_i1]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([b6_l0, b6_l1, b6_l2]);
                CompiledResume::Next(9)
            }

            fn int_list_int_3_resume_9(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list2 = values.int_lists.remove(2);
                let _list1 = values.int_lists.remove(1);
                let _list0 = values.int_lists.remove(0);
                let (b6_i0, b6_i1, b6_l0, b6_l1, b6_l2,) = (values.ints[0], values.ints[1], _list0, _list1, _list2,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b6_i0, b6_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b6_l0, b6_l1, b6_l2]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(9));
                }
                *budget -= 1;
                {
                    let _next = (b6_i1, b6_l2, b6_l1,);
                    drop(b6_l0);
                    let (b0_i0, b0_l0, b0_l1,) = _next;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b0_l0, b0_l1]);
                    CompiledResume::Next(0)
                }
            }

            fn int_list_int_3_resume_10(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list1 = values.int_lists.remove(1);
                let _list0 = values.int_lists.remove(0);
                let (b7_i0, b7_l0, b7_l1,) = (values.ints[0], _list0, _list1,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b7_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b7_l0, b7_l1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(10));
                }
                *budget -= 1;
                {
                    let (b8_i0, b8_l0, b8_l1,) = (b7_i0, b7_l0, b7_l1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b8_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b8_l0, b8_l1]);
                    CompiledResume::Next(11)
                }
            }

            fn int_list_int_3_resume_11(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list1 = values.int_lists.remove(1);
                let _list0 = values.int_lists.remove(0);
                let (b8_i0, b8_l0, b8_l1,) = (values.ints[0], _list0, _list1,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b8_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b8_l0, b8_l1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(11));
                }
                *budget -= 1;
                let b8_l2 = _lists.tail(&b8_l0, data::type_::IntListTypeId {
                    list_type: data::type_::ListTypeId(0),
                }, 1);

                values.ints.clear();
                values.ints.extend_from_slice(&[b8_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);
                values.int_lists.clear();
                values.int_lists.extend([b8_l0, b8_l1, b8_l2]);
                CompiledResume::Next(12)
            }

            fn int_list_int_3_resume_12(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list2 = values.int_lists.remove(2);
                let _list1 = values.int_lists.remove(1);
                let _list0 = values.int_lists.remove(0);
                let (b8_i0, b8_l0, b8_l1, b8_l2,) = (values.ints[0], _list0, _list1, _list2,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b8_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b8_l0, b8_l1, b8_l2]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(12));
                }
                *budget -= 1;
                {
                    let _next = (b8_i0, b8_l2, b8_l1,);
                    drop(b8_l0);
                    let (b0_i0, b0_l0, b0_l1,) = _next;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b0_l0, b0_l1]);
                    CompiledResume::Next(0)
                }
            }

            fn int_list_int_3_resume_13(
                values: &mut data::compiled::int_list::IntListValues,
                _lists: &data::compiled::int_list::IntListOps<'_>,
                budget: &mut usize,
            ) -> CompiledResume {
                let _list1 = values.int_lists.remove(1);
                let _list0 = values.int_lists.remove(0);
                let (b9_i0, b9_l0, b9_l1,) = (values.ints[0], _list0, _list1,);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b9_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b9_l0, b9_l1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(13));
                }
                *budget -= 1;
                {
                    let (b8_i0, b8_l0, b8_l1,) = (b9_i0, b9_l0, b9_l1,);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b8_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b8_l0, b8_l1]);
                    CompiledResume::Next(11)
                }
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
                        function: data::function::IntFunctionId(2),
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
                                    block: data::graph::BlockId(7),
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
                            run: int_list_int_2,
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
                                    int_lists: 2,
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
                                    int_lists: 2,
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
                                    block: data::graph::BlockId(5),
                                    instruction: 0,
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
                                    block: data::graph::BlockId(5),
                                    instruction: 1,
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
                                    block: data::graph::BlockId(6),
                                    instruction: 0,
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
                                    block: data::graph::BlockId(6),
                                    instruction: 1,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 3,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(6),
                                    instruction: 2,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 3,
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
                                    int_lists: 2,
                                    strings: 0,
                                    customs: 0,
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
                                    int_lists: 2,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(8),
                                    instruction: 1,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 3,
                                    strings: 0,
                                    customs: 0,
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
                                    int_lists: 2,
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
                                    ints: 3,
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
                                    instruction: 5,
                                    ints: 4,
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
                                    instruction: 6,
                                    ints: 5,
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
                                    instruction: 7,
                                    ints: 6,
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
                                    instruction: 8,
                                    ints: 7,
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
                                    instruction: 9,
                                    ints: 7,
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
                                    block: data::graph::BlockId(0),
                                    instruction: 10,
                                    ints: 8,
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
                                    block: data::graph::BlockId(0),
                                    instruction: 11,
                                    ints: 9,
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
                                    block: data::graph::BlockId(0),
                                    instruction: 12,
                                    ints: 10,
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
                                    block: data::graph::BlockId(0),
                                    instruction: 13,
                                    ints: 11,
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
                                    block: data::graph::BlockId(0),
                                    instruction: 14,
                                    ints: 12,
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
                                    block: data::graph::BlockId(0),
                                    instruction: 15,
                                    ints: 12,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 3,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 16,
                                    ints: 12,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 4,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 17,
                                    ints: 13,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 4,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 18,
                                    ints: 14,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 4,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                },
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 19,
                                    ints: 15,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 4,
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
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(0),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(0),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(0),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(0),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(0),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(0),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(1),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(0),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(1),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(0),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(1),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(0),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(1),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(0),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(1),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(0),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(1),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(0),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(1),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(2),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(0),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(1),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
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
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(0),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(1),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
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
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(0),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(1),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
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
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(0),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                        local: data::graph::IntListLocalId(1),
                                        type_id: data::type_::IntListTypeId {
                                            list_type: data::type_::ListTypeId(0),
                                        },
                                    }),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(11)),
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
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[
                                data::compiled::CallContract {
                                    point: 5,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(1))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2527, 2546)),
                                },
                                data::compiled::CallContract {
                                    point: 10,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(2))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2549, 2568)),
                                },
                                data::compiled::CallContract {
                                    point: 17,
                                    output: data::graph::ParamLocal::Int(data::graph::IntLocalId(13)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(3))),
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
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(12)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2571, 2596)),
                                },
                            ]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 19,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(14)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: calls_int_0_start,
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
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
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
                    parameters: 4..7,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(1),
                        data::type_::ValueShapeId(1),
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
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
                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                    local: data::graph::IntListLocalId(1),
                    type_id: data::type_::IntListTypeId {
                        list_type: data::type_::ListTypeId(0),
                    },
                }),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
