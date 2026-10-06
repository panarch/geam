data::HostedModuleArtifact {
    module: data::ModuleArtifact {
        format: 23,
        program: data::ProgramTables {
            root: data::source::module_id(0),
            modules: data::Storage::Static(&[
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("example"),
                    source_context: Some(data::source::SourceContext::from_static_block("src/example.gleam", r#"
pub fn count(text: String, total: Int) -> Int {
  case text {
    "λ" <> rest -> count(rest, total + 1)
    "" -> total
    _ -> panic as "expected lambda prefix"
  }
}

pub fn select(text: String) -> Int {
  case text {
    "red" -> 7
    "blue" -> 9
    "\n\"\\λ" -> 11
    _ -> -1
  }
}

pub fn aliases(left: String, right: String, flag: Bool, total: Int) -> Int {
  case left {
    "λ" as prefix <> rest as whole if rest == right && flag -> {
      case whole == left && prefix == "λ" {
        True -> total + 2
        False -> -100
      }
    }
    "λ" <> rest if rest == right -> total + 4
    "λ" <> _ -> total + 8
    _ -> total + 16
  }
}

pub fn alternate(left: String, right: String, total: Int) -> Int {
  case left {
    "λ" <> rest | "m" <> rest -> alternate(right, rest, total + 1)
    "" ->
      case right {
        "" -> total
        _ -> alternate(right, left, total)
      }
    _ -> total
  }
}

pub fn same(left: String, right: String, expected: Bool) -> Bool {
  let equal = left == right
  let different = left != right
  case equal == expected {
    True -> different != expected
    False -> False
  }
}

pub fn empty_prefix(text: String) -> Bool {
  case text {
    "" as prefix <> rest as whole ->
      case prefix == "" {
        True -> rest == whole
        False -> False
      }
    _ -> False
  }
}

pub fn literal_only() -> Int {
  case "λtail" {
    "λ" <> rest if rest == "tail" -> 7
    _ -> -1
  }
}

pub fn asserted(text: String, total: Int) -> Int {
  let assert "λ" as prefix <> rest as whole = text as "lambda required"
  case rest != whole {
    True ->
      case prefix == "λ" {
        True -> total + 1
        False -> -1
      }
    False -> -1
  }
}

pub fn caller(text: String, total: Int) -> #(String, Int, List(Int), Bool) {
  let result = count(text, total)
  #(text, result, [3, 5], same(text, text, True))
}

pub fn spin(text: String) -> Int {
  case text {
    "λ" <> _ -> spin(text)
    _ -> 0
  }
}

pub fn running(text: String) -> Int {
  echo "entered-string"
  spin(text)
}

pub fn unsupported(text: String) -> Int {
  let joined = text <> "tail"
  case joined {
    "tail" -> 1
    _ -> 2
  }
}

pub fn main() -> Int {
  count("λλλ", 4)
}

pub fn assert_literal(text: String) -> Int {
  let assert "\n\"\\λ" as whole = text as "literal required"
  case whole {
    "\n\"\\λ" -> 17
    _ -> -1
  }
}

pub fn assert_prefix(text: String) -> Int {
  let assert "λ" <> _ = text as "prefix required"
  19
}

pub fn assert_suffix(text: String, total: Int) -> Int {
  let assert "λ" as unused <> rest = text as "suffix required"
  case total != -1 {
    True ->
      case rest {
        "tail" -> total + 23
        _ -> total + 29
      }
    False -> -1
  }
}

pub fn bits_with_boolean_guard(
  bytes: BitArray,
  left: Bool,
  right: Bool,
) -> Int {
  let assert <<head:8, _:bits>> = bytes
  case left == right {
    True -> head
    False -> 0
  }
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
                                                test: data::graph::BoolTest::StringStartsWith {
                                                    value: data::graph::StringLocalId(0),
                                                    prefix: data::Text::Static("λ"),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..4,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
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
                                            params: 4..6,
                                            instructions: 2..3,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::Equal {
                                                    left: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    right: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(3),
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
                                                        ]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(4),
                                                    args: data::Storage::Static(&[]),
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
                                        data::graph::BlockHeader {
                                            params: 7..7,
                                            instructions: 3..4,
                                            terminator: data::graph::Terminator::SourceStop(data::graph::SourceStop {
                                                kind: data::graph::SourceStopKind::Panic,
                                                message: Some(data::graph::StringLocalId(0)),
                                                site: data::source::PanicSite::from_static("example", "count", data::source::SourceSpan::new(130, 163)),
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
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
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::DropPrefix {
                                                value: data::graph::StringLocalId(0),
                                                prefix: data::Text::Static("λ"),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static(""))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("expected lambda prefix"))),
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
                                                test: data::graph::BoolTest::StringStartsWith {
                                                    value: data::graph::StringLocalId(0),
                                                    prefix: data::Text::Static("λ"),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(20),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
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
                                            params: 4..8,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::Equal {
                                                    left: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                    right: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 2,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(19),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 2,
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
                                            params: 8..12,
                                            instructions: 1..1,
                                            terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                                subject: data::graph::BoolLocalId(0),
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(3),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 1,
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
                                                    target: data::graph::BlockId(9),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
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
                                            params: 12..14,
                                            instructions: 1..3,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::Equal {
                                                    left: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    right: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(4),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
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
                                                    target: data::graph::BlockId(8),
                                                    args: data::Storage::Static(&[]),
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
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 14..16,
                                            instructions: 3..4,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::Equal {
                                                    left: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    right: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(5),
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
                                                        ]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(6),
                                                    args: data::Storage::Static(&[]),
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
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 16..17,
                                            instructions: 4..5,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 17..17,
                                            instructions: 5..5,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(7),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 17..17,
                                            instructions: 5..6,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 17..17,
                                            instructions: 6..6,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(7),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 17..20,
                                            instructions: 6..6,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(10),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
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
                                            instructions: 6..6,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(11),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 23..26,
                                            instructions: 6..6,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::StringStartsWith {
                                                    value: data::graph::StringLocalId(0),
                                                    prefix: data::Text::Static("λ"),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(12),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(18),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 26..29,
                                            instructions: 6..7,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::Equal {
                                                    left: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                    right: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(13),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(14),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 29..31,
                                            instructions: 7..9,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 31..33,
                                            instructions: 9..9,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(15),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 33..35,
                                            instructions: 9..9,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::StringStartsWith {
                                                    value: data::graph::StringLocalId(0),
                                                    prefix: data::Text::Static("λ"),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(16),
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
                                                        ]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(17),
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
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 35..36,
                                            instructions: 9..10,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(3)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 36..37,
                                            instructions: 10..11,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(4)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 37..39,
                                            instructions: 11..11,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(15),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 39..42,
                                            instructions: 11..11,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(10),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 42..45,
                                            instructions: 11..11,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(11),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
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
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
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
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::DropPrefix {
                                                value: data::graph::StringLocalId(0),
                                                prefix: data::Text::Static("λ"),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("λ"))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::DropPrefix {
                                                value: data::graph::StringLocalId(0),
                                                prefix: data::Text::Static("λ"),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("λ"))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(2),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Minus,
                                                digits: data::Storage::Static(&[
                                                    100,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::DropPrefix {
                                                value: data::graph::StringLocalId(0),
                                                prefix: data::Text::Static("λ"),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::DropPrefix {
                                                value: data::graph::StringLocalId(0),
                                                prefix: data::Text::Static("λ"),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(4),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(8),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(16),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
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
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                pattern: data::graph::MatchPattern::Alias {
                                                    pattern: data::Storage::Static(&data::graph::MatchPattern::StringPrefix {
                                                        prefix: data::Text::Static("λ"),
                                                        left: Some(data::graph::MatchPatternBinding {
                                                            index: 0,
                                                        }),
                                                        right: Some(data::graph::MatchPatternBinding {
                                                            index: 1,
                                                        }),
                                                    }),
                                                    binding: data::graph::MatchPatternBinding {
                                                        index: 2,
                                                    },
                                                },
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Binding(0),
                                                        data::graph::MatchEdgeArgument::Binding(1),
                                                        data::graph::MatchEdgeArgument::Binding(2),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[
                                                        0,
                                                        1,
                                                        2,
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 3,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 1,
                                                                        destination: 0,
                                                                    },
                                                                    data::graph::TransferStep {
                                                                        source: 2,
                                                                        destination: 1,
                                                                    },
                                                                    data::graph::TransferStep {
                                                                        source: 3,
                                                                        destination: 2,
                                                                    },
                                                                ]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(6),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
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
                                            params: 2..6,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::NotEqual {
                                                    left: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    right: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(5),
                                                    args: data::Storage::Static(&[]),
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
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 6..8,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::Equal {
                                                    left: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    right: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(3),
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
                                                        ]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(4),
                                                    args: data::Storage::Static(&[]),
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
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 8..9,
                                            instructions: 1..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 9..9,
                                            instructions: 2..3,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 9..9,
                                            instructions: 3..4,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 9..10,
                                            instructions: 4..5,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                message: Some(data::graph::StringLocalId(1)),
                                                site: data::source::PanicSite::from_static("example", "asserted", data::source::SourceSpan::new(1508, 1518)),
                                                pattern_span: data::source::SourceSpan::new(1519, 1550),
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
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
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("λ"))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("lambda required"))),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
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
                                            params: 0..1,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Echo(data::graph::Echo {
                                                subject: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                message: None,
                                                site: data::source::EchoSite::from_static("example", "running", data::source::SourceSpan::new(2018, 2039)),
                                                next: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 1..2,
                                            instructions: 1..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("entered-string"))),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::IntFunctionId(4),
                                            site: data::source::HostCallSite::from_static("example", "running", data::source::SourceSpan::new(2042, 2052)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[]),
                                        },
                                    },
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
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::StringStartsWith {
                                                    value: data::graph::StringLocalId(0),
                                                    prefix: data::Text::Static("λ"),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
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
                                            params: 1..2,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..2,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::NoSign,
                                                digits: data::Storage::Static(&[]),
                                            })),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
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
                    bool_functions: data::Storage::Static(&[
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
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::Equal {
                                                    left: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                    right: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
                                                                length: 2,
                                                                steps: data::Storage::Static(&[
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
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
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
                                            instructions: 2..3,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 5..5,
                                            instructions: 3..4,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Equal {
                                                left: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                right: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::NotEqual {
                                                left: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                right: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::NotEqual {
                                                left: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                right: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                    data::function::FunctionExit::Return(data::graph::BoolLocalId(2)),
                                    data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
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
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::StringStartsWith {
                                                    value: data::graph::StringLocalId(0),
                                                    prefix: data::Text::Static(""),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(4),
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
                                            params: 1..2,
                                            instructions: 0..3,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::Equal {
                                                    left: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                    right: data::graph::ParamLocal::String(data::graph::StringLocalId(3)),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 2,
                                                                steps: data::Storage::Static(&[
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
                                            params: 2..4,
                                            instructions: 3..4,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 4..4,
                                            instructions: 4..5,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 4..4,
                                            instructions: 5..6,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static(""))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::DropPrefix {
                                                value: data::graph::StringLocalId(0),
                                                prefix: data::Text::Static(""),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(3)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static(""))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Equal {
                                                left: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                right: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(false)),
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
                                    data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
                                ]),
                            },
                        })),
                    ]),
                    nil_functions: data::Storage::Static(&[]),
                    tuple_functions: data::Storage::Static(&[
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
                                            instructions: 0..7,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "caller", data::source::SourceSpan::new(1811, 1829)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(1),
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
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                shape: data::type_::ValueShapeId(1),
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
                                                data::graph::IntLocalId(2),
                                                data::graph::IntLocalId(3),
                                            ])))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                                function: data::function::BoolFunctionId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "caller", data::source::SourceSpan::new(1856, 1878)),
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
                                                        data::type_::ValueType::Bool,
                                                    ]),
                                                },
                                                shape: data::type_::ValueShapeId(4),
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
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
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

                fn string_int_0(
                    point: usize,
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {

                    const RESUME: [
                        fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                        9
                    ] = [
                        |values, budget| CompiledResume::Exit(string_int_0_entry((values.ints[0], values.strings[0],), values, budget)),
                        string_int_0_resume_1,
                        string_int_0_resume_2,
                        string_int_0_resume_3,
                        string_int_0_resume_4,
                        string_int_0_resume_5,
                        string_int_0_resume_6,
                        string_int_0_resume_7,
                        string_int_0_resume_8,
                    ];

                    let mut point = point;
                    loop {
                        match RESUME[point](values, budget) {
                            CompiledResume::Next(next) => point = next,
                            CompiledResume::Exit(progress) => return progress,
                        }
                    }
                }

                fn string_int_0_entry(
                    inputs: (i128, data::compiled::string::StringRange,),
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {
                    let (mut b0_i0, mut b0_s0,) = inputs;
                    'repeat: loop {
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b0_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b0_s0]);
                            return data::compiled::CompiledProgress::Yield(0);
                        }
                        *budget -= 1;
                        if values.bytes(b0_s0).starts_with("λ".as_bytes()) {
                            let (b1_i0, b1_s0,) = (b0_i0, b0_s0,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b1_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b1_s0]);
                                return data::compiled::CompiledProgress::Yield(1);
                            }
                            *budget -= 1;
                            let b1_s1 = b1_s0.drop_prefix(2);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b1_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                                return data::compiled::CompiledProgress::Yield(2);
                            }
                            *budget -= 1;
                            let b1_i1 = b1_i0 + 1_i128;
                            if b1_i1 < i128::from(i64::MIN) || b1_i1 > i128::from(i64::MAX) {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                                return data::compiled::CompiledProgress::Interpreted(3);
                            }
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                                return data::compiled::CompiledProgress::Yield(3);
                            }
                            *budget -= 1;
                            {
                                (b0_i0, b0_s0,) = (b1_i1, b1_s1,);
                                continue 'repeat;
                            }
                        } else {
                            let (b2_i0, b2_s0,) = (b0_i0, b0_s0,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b2_s0]);
                                return data::compiled::CompiledProgress::Yield(4);
                            }
                            *budget -= 1;
                            let b2_s1 = data::compiled::string::StringRange::literal("");
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                                return data::compiled::CompiledProgress::Yield(5);
                            }
                            *budget -= 1;
                            if values.bytes(b2_s0) == values.bytes(b2_s1) {
                                let (b3_i0,) = (b2_i0,);
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);

                                    values.strings.clear();
                                    values.strings.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Yield(6);
                                }
                                *budget -= 1;

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0));
                            } else {
                                let () = ();
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);

                                    values.strings.clear();
                                    values.strings.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Yield(7);
                                }
                                *budget -= 1;
                                let b4_s0 = data::compiled::string::StringRange::literal("expected lambda prefix");
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);

                                    values.strings.clear();
                                    values.strings.extend_from_slice(&[b4_s0]);
                                    return data::compiled::CompiledProgress::Yield(8);
                                }

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b4_s0]);
                                return data::compiled::CompiledProgress::Interpreted(8);
                            }
                        }
                    }
                }

                fn string_int_0_resume_1(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_i0, b1_s0,) = (values.ints[0], values.strings[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                    }
                    *budget -= 1;
                    let b1_s1 = b1_s0.drop_prefix(2);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                    CompiledResume::Next(2)
                }

                fn string_int_0_resume_2(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_i0, b1_s0, b1_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                    }
                    *budget -= 1;
                    let b1_i1 = b1_i0 + 1_i128;
                    if b1_i1 < i128::from(i64::MIN) || b1_i1 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(3));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                    CompiledResume::Next(3)
                }

                fn string_int_0_resume_3(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_i0, b1_i1, b1_s0, b1_s1,) = (values.ints[0], values.ints[1], values.strings[0], values.strings[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                    }
                    *budget -= 1;
                    {
                        let (b0_i0, b0_s0,) = (b1_i1, b1_s1,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b0_s0]);
                        CompiledResume::Next(0)
                    }
                }

                fn string_int_0_resume_4(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b2_i0, b2_s0,) = (values.ints[0], values.strings[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b2_s0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                    }
                    *budget -= 1;
                    let b2_s1 = data::compiled::string::StringRange::literal("");

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                    CompiledResume::Next(5)
                }

                fn string_int_0_resume_5(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b2_i0, b2_s0, b2_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                    }
                    *budget -= 1;
                    if values.bytes(b2_s0) == values.bytes(b2_s1) {
                        let (b3_i0,) = (b2_i0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        CompiledResume::Next(6)
                    } else {
                        let () = ();

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        CompiledResume::Next(7)
                    }
                }

                fn string_int_0_resume_6(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b3_i0,) = (values.ints[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
                }

                fn string_int_0_resume_7(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                    }
                    *budget -= 1;
                    let b4_s0 = data::compiled::string::StringRange::literal("expected lambda prefix");

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b4_s0]);
                    CompiledResume::Next(8)
                }

                fn string_int_0_resume_8(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b4_s0,) = (values.strings[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b4_s0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b4_s0]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(8))
                }

                fn string_int_1(
                    point: usize,
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {

                    const RESUME: [
                        fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                        32
                    ] = [
                        |values, budget| CompiledResume::Exit(string_int_1_entry((values.ints[0], values.bools[0], values.strings[0], values.strings[1],), values, budget)),
                        string_int_1_resume_1,
                        string_int_1_resume_2,
                        string_int_1_resume_3,
                        string_int_1_resume_4,
                        string_int_1_resume_5,
                        string_int_1_resume_6,
                        string_int_1_resume_7,
                        string_int_1_resume_8,
                        string_int_1_resume_9,
                        string_int_1_resume_10,
                        string_int_1_resume_11,
                        string_int_1_resume_12,
                        string_int_1_resume_13,
                        string_int_1_resume_14,
                        string_int_1_resume_15,
                        string_int_1_resume_16,
                        string_int_1_resume_17,
                        string_int_1_resume_18,
                        string_int_1_resume_19,
                        string_int_1_resume_20,
                        string_int_1_resume_21,
                        string_int_1_resume_22,
                        string_int_1_resume_23,
                        string_int_1_resume_24,
                        string_int_1_resume_25,
                        string_int_1_resume_26,
                        string_int_1_resume_27,
                        string_int_1_resume_28,
                        string_int_1_resume_29,
                        string_int_1_resume_30,
                        string_int_1_resume_31,
                    ];

                    let mut point = point;
                    loop {
                        match RESUME[point](values, budget) {
                            CompiledResume::Next(next) => point = next,
                            CompiledResume::Exit(progress) => return progress,
                        }
                    }
                }

                fn string_int_1_entry(
                    inputs: (i128, bool, data::compiled::string::StringRange, data::compiled::string::StringRange,),
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {
                    let (b0_i0, b0_v0, b0_s0, b0_s1,) = inputs;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b0_v0]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                        return data::compiled::CompiledProgress::Yield(0);
                    }
                    *budget -= 1;
                    let (b11_i0, b11_s0, b11_s1,) = if values.bytes(b0_s0).starts_with("λ".as_bytes()) {
                        let (b1_i0, b1_v0, b1_s0, b1_s1,) = (b0_i0, b0_v0, b0_s0, b0_s1,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b1_v0]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                            return data::compiled::CompiledProgress::Yield(1);
                        }
                        *budget -= 1;
                        let b1_s2 = b1_s0.drop_prefix(2);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b1_v0]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                            return data::compiled::CompiledProgress::Yield(2);
                        }
                        *budget -= 1;
                        let (b10_i0, b10_s0, b10_s1,) = if values.bytes(b1_s2) == values.bytes(b1_s1) {
                            let (b2_i0, b2_v0, b2_s0, b2_s1,) = (b1_i0, b1_v0, b1_s0, b1_s1,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[b2_v0]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                                return data::compiled::CompiledProgress::Yield(3);
                            }
                            *budget -= 1;
                            if b2_v0 {
                                let (b3_i0, b3_s0,) = (b2_i0, b2_s0,);
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);

                                    values.strings.clear();
                                    values.strings.extend_from_slice(&[b3_s0]);
                                    return data::compiled::CompiledProgress::Yield(4);
                                }
                                *budget -= 1;
                                let b3_s1 = data::compiled::string::StringRange::literal("λ");
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);

                                    values.strings.clear();
                                    values.strings.extend_from_slice(&[b3_s0, b3_s1]);
                                    return data::compiled::CompiledProgress::Yield(5);
                                }
                                *budget -= 1;
                                let b3_s2 = b3_s0.drop_prefix(2);
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);

                                    values.strings.clear();
                                    values.strings.extend_from_slice(&[b3_s0, b3_s1, b3_s2]);
                                    return data::compiled::CompiledProgress::Yield(6);
                                }
                                *budget -= 1;
                                if values.bytes(b3_s0) == values.bytes(b3_s0) {
                                    let (b4_i0, b4_s0,) = (b3_i0, b3_s1,);
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b4_i0]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);

                                        values.strings.clear();
                                        values.strings.extend_from_slice(&[b4_s0]);
                                        return data::compiled::CompiledProgress::Yield(7);
                                    }
                                    *budget -= 1;
                                    let b4_s1 = data::compiled::string::StringRange::literal("λ");
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b4_i0]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);

                                        values.strings.clear();
                                        values.strings.extend_from_slice(&[b4_s0, b4_s1]);
                                        return data::compiled::CompiledProgress::Yield(8);
                                    }
                                    *budget -= 1;
                                    if values.bytes(b4_s0) == values.bytes(b4_s1) {
                                        let (b5_i0,) = (b4_i0,);
                                        if *budget == 0 {

                                            values.ints.clear();
                                            values.ints.extend_from_slice(&[b5_i0]);
                                            values.bools.clear();
                                            values.bools.extend_from_slice(&[]);

                                            values.strings.clear();
                                            values.strings.extend_from_slice(&[]);
                                            return data::compiled::CompiledProgress::Yield(9);
                                        }
                                        *budget -= 1;
                                        let b5_i1 = b5_i0 + 2_i128;
                                        if b5_i1 < i128::from(i64::MIN) || b5_i1 > i128::from(i64::MAX) {

                                            values.ints.clear();
                                            values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                                            values.bools.clear();
                                            values.bools.extend_from_slice(&[]);

                                            values.strings.clear();
                                            values.strings.extend_from_slice(&[]);
                                            return data::compiled::CompiledProgress::Interpreted(10);
                                        }
                                        if *budget == 0 {

                                            values.ints.clear();
                                            values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                                            values.bools.clear();
                                            values.bools.extend_from_slice(&[]);

                                            values.strings.clear();
                                            values.strings.extend_from_slice(&[]);
                                            return data::compiled::CompiledProgress::Yield(10);
                                        }
                                        *budget -= 1;

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);

                                        values.strings.clear();
                                        values.strings.extend_from_slice(&[]);
                                        return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0));
                                    } else {
                                        let () = ();
                                        if *budget == 0 {

                                            values.ints.clear();
                                            values.ints.extend_from_slice(&[]);
                                            values.bools.clear();
                                            values.bools.extend_from_slice(&[]);

                                            values.strings.clear();
                                            values.strings.extend_from_slice(&[]);
                                            return data::compiled::CompiledProgress::Yield(11);
                                        }
                                        *budget -= 1;
                                        {
                                        };
                                    }
                                } else {
                                    let () = ();
                                    if *budget == 0 {

                                        values.ints.clear();
                                        values.ints.extend_from_slice(&[]);
                                        values.bools.clear();
                                        values.bools.extend_from_slice(&[]);

                                        values.strings.clear();
                                        values.strings.extend_from_slice(&[]);
                                        return data::compiled::CompiledProgress::Yield(14);
                                    }
                                    *budget -= 1;
                                    {
                                    };
                                };
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);

                                    values.strings.clear();
                                    values.strings.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Yield(12);
                                }
                                *budget -= 1;
                                let b7_i0 = -100_i128;
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b7_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);

                                    values.strings.clear();
                                    values.strings.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Yield(13);
                                }
                                *budget -= 1;

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b7_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1));
                            } else {
                                let (b9_i0, b9_s0, b9_s1,) = (b2_i0, b2_s0, b2_s1,);
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b9_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);

                                    values.strings.clear();
                                    values.strings.extend_from_slice(&[b9_s0, b9_s1]);
                                    return data::compiled::CompiledProgress::Yield(15);
                                }
                                *budget -= 1;
                                let (b10_i0, b10_s0, b10_s1,) = {
                                    (b9_i0, b9_s0, b9_s1,)
                                };
                                (b10_i0, b10_s0, b10_s1,)
                            }
                        } else {
                            let (b19_i0, b19_s0, b19_s1,) = (b1_i0, b1_s0, b1_s1,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b19_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b19_s0, b19_s1]);
                                return data::compiled::CompiledProgress::Yield(30);
                            }
                            *budget -= 1;
                            let (b10_i0, b10_s0, b10_s1,) = {
                                (b19_i0, b19_s0, b19_s1,)
                            };
                            (b10_i0, b10_s0, b10_s1,)
                        };
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b10_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b10_s0, b10_s1]);
                            return data::compiled::CompiledProgress::Yield(16);
                        }
                        *budget -= 1;
                        let (b11_i0, b11_s0, b11_s1,) = {
                            (b10_i0, b10_s0, b10_s1,)
                        };
                        (b11_i0, b11_s0, b11_s1,)
                    } else {
                        let (b20_i0, b20_s0, b20_s1,) = (b0_i0, b0_s0, b0_s1,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b20_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b20_s0, b20_s1]);
                            return data::compiled::CompiledProgress::Yield(31);
                        }
                        *budget -= 1;
                        let (b11_i0, b11_s0, b11_s1,) = {
                            (b20_i0, b20_s0, b20_s1,)
                        };
                        (b11_i0, b11_s0, b11_s1,)
                    };
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b11_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b11_s0, b11_s1]);
                        return data::compiled::CompiledProgress::Yield(17);
                    }
                    *budget -= 1;
                    let (b15_i0, b15_s0,) = if values.bytes(b11_s0).starts_with("λ".as_bytes()) {
                        let (b12_i0, b12_s0, b12_s1,) = (b11_i0, b11_s0, b11_s1,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b12_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b12_s0, b12_s1]);
                            return data::compiled::CompiledProgress::Yield(18);
                        }
                        *budget -= 1;
                        let b12_s2 = b12_s0.drop_prefix(2);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b12_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b12_s0, b12_s1, b12_s2]);
                            return data::compiled::CompiledProgress::Yield(19);
                        }
                        *budget -= 1;
                        if values.bytes(b12_s2) == values.bytes(b12_s1) {
                            let (b13_i0, b13_s0,) = (b12_i0, b12_s0,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b13_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b13_s0]);
                                return data::compiled::CompiledProgress::Yield(20);
                            }
                            *budget -= 1;
                            let b13_s1 = b13_s0.drop_prefix(2);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b13_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b13_s0, b13_s1]);
                                return data::compiled::CompiledProgress::Yield(21);
                            }
                            *budget -= 1;
                            let b13_i1 = b13_i0 + 4_i128;
                            if b13_i1 < i128::from(i64::MIN) || b13_i1 > i128::from(i64::MAX) {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b13_i0, b13_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b13_s0, b13_s1]);
                                return data::compiled::CompiledProgress::Interpreted(22);
                            }
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b13_i0, b13_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b13_s0, b13_s1]);
                                return data::compiled::CompiledProgress::Yield(22);
                            }
                            *budget -= 1;

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b13_i0, b13_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b13_s0, b13_s1]);
                            return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2));
                        } else {
                            let (b14_i0, b14_s0,) = (b12_i0, b12_s0,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b14_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b14_s0]);
                                return data::compiled::CompiledProgress::Yield(23);
                            }
                            *budget -= 1;
                            let (b15_i0, b15_s0,) = {
                                (b14_i0, b14_s0,)
                            };
                            (b15_i0, b15_s0,)
                        }
                    } else {
                        let (b18_i0, b18_s0,) = (b11_i0, b11_s0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b18_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b18_s0]);
                            return data::compiled::CompiledProgress::Yield(29);
                        }
                        *budget -= 1;
                        let (b15_i0, b15_s0,) = {
                            (b18_i0, b18_s0,)
                        };
                        (b15_i0, b15_s0,)
                    };
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b15_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b15_s0]);
                        return data::compiled::CompiledProgress::Yield(24);
                    }
                    *budget -= 1;
                    if values.bytes(b15_s0).starts_with("λ".as_bytes()) {
                        let (b16_i0,) = (b15_i0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b16_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(25);
                        }
                        *budget -= 1;
                        let b16_i1 = b16_i0 + 8_i128;
                        if b16_i1 < i128::from(i64::MIN) || b16_i1 > i128::from(i64::MAX) {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b16_i0, b16_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Interpreted(26);
                        }
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b16_i0, b16_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(26);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b16_i0, b16_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(3))
                    } else {
                        let (b17_i0,) = (b15_i0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b17_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(27);
                        }
                        *budget -= 1;
                        let b17_i1 = b17_i0 + 16_i128;
                        if b17_i1 < i128::from(i64::MIN) || b17_i1 > i128::from(i64::MAX) {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b17_i0, b17_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Interpreted(28);
                        }
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b17_i0, b17_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(28);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b17_i0, b17_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(4))
                    }
                }

                fn string_int_1_resume_1(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_i0, b1_v0, b1_s0, b1_s1,) = (values.ints[0], values.bools[0], values.strings[0], values.strings[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                    }
                    *budget -= 1;
                    let b1_s2 = b1_s0.drop_prefix(2);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                    CompiledResume::Next(2)
                }

                fn string_int_1_resume_2(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_i0, b1_v0, b1_s0, b1_s1, b1_s2,) = (values.ints[0], values.bools[0], values.strings[0], values.strings[1], values.strings[2],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                    }
                    *budget -= 1;
                    if values.bytes(b1_s2) == values.bytes(b1_s1) {
                        let (b2_i0, b2_v0, b2_s0, b2_s1,) = (b1_i0, b1_v0, b1_s0, b1_s1,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b2_v0]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                        CompiledResume::Next(3)
                    } else {
                        let (b19_i0, b19_s0, b19_s1,) = (b1_i0, b1_s0, b1_s1,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b19_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b19_s0, b19_s1]);
                        CompiledResume::Next(30)
                    }
                }

                fn string_int_1_resume_3(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b2_i0, b2_v0, b2_s0, b2_s1,) = (values.ints[0], values.bools[0], values.strings[0], values.strings[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b2_v0]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                    }
                    *budget -= 1;
                    if b2_v0 {
                        let (b3_i0, b3_s0,) = (b2_i0, b2_s0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b3_s0]);
                        CompiledResume::Next(4)
                    } else {
                        let (b9_i0, b9_s0, b9_s1,) = (b2_i0, b2_s0, b2_s1,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b9_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b9_s0, b9_s1]);
                        CompiledResume::Next(15)
                    }
                }

                fn string_int_1_resume_4(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b3_i0, b3_s0,) = (values.ints[0], values.strings[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b3_s0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                    }
                    *budget -= 1;
                    let b3_s1 = data::compiled::string::StringRange::literal("λ");

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b3_s0, b3_s1]);
                    CompiledResume::Next(5)
                }

                fn string_int_1_resume_5(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b3_i0, b3_s0, b3_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b3_s0, b3_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                    }
                    *budget -= 1;
                    let b3_s2 = b3_s0.drop_prefix(2);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b3_s0, b3_s1, b3_s2]);
                    CompiledResume::Next(6)
                }

                fn string_int_1_resume_6(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b3_i0, b3_s0, b3_s1, b3_s2,) = (values.ints[0], values.strings[0], values.strings[1], values.strings[2],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b3_s0, b3_s1, b3_s2]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                    }
                    *budget -= 1;
                    if values.bytes(b3_s0) == values.bytes(b3_s0) {
                        let (b4_i0, b4_s0,) = (b3_i0, b3_s1,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b4_s0]);
                        CompiledResume::Next(7)
                    } else {
                        let () = ();

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        CompiledResume::Next(14)
                    }
                }

                fn string_int_1_resume_7(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b4_i0, b4_s0,) = (values.ints[0], values.strings[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b4_s0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                    }
                    *budget -= 1;
                    let b4_s1 = data::compiled::string::StringRange::literal("λ");

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b4_s0, b4_s1]);
                    CompiledResume::Next(8)
                }

                fn string_int_1_resume_8(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b4_i0, b4_s0, b4_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b4_s0, b4_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                    }
                    *budget -= 1;
                    if values.bytes(b4_s0) == values.bytes(b4_s1) {
                        let (b5_i0,) = (b4_i0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b5_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        CompiledResume::Next(9)
                    } else {
                        let () = ();

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        CompiledResume::Next(11)
                    }
                }

                fn string_int_1_resume_9(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b5_i0,) = (values.ints[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b5_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(9));
                    }
                    *budget -= 1;
                    let b5_i1 = b5_i0 + 2_i128;
                    if b5_i1 < i128::from(i64::MIN) || b5_i1 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(10));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(10)
                }

                fn string_int_1_resume_10(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b5_i0, b5_i1,) = (values.ints[0], values.ints[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(10));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0, b5_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
                }

                fn string_int_1_resume_11(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(11));
                    }
                    *budget -= 1;
                    {
                        let () = ();

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        CompiledResume::Next(12)
                    }
                }

                fn string_int_1_resume_12(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(12));
                    }
                    *budget -= 1;
                    let b7_i0 = -100_i128;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b7_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(13)
                }

                fn string_int_1_resume_13(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b7_i0,) = (values.ints[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b7_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(13));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b7_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
                }

                fn string_int_1_resume_14(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(14));
                    }
                    *budget -= 1;
                    {
                        let () = ();

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        CompiledResume::Next(12)
                    }
                }

                fn string_int_1_resume_15(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b9_i0, b9_s0, b9_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b9_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b9_s0, b9_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(15));
                    }
                    *budget -= 1;
                    {
                        let (b10_i0, b10_s0, b10_s1,) = (b9_i0, b9_s0, b9_s1,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b10_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b10_s0, b10_s1]);
                        CompiledResume::Next(16)
                    }
                }

                fn string_int_1_resume_16(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b10_i0, b10_s0, b10_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b10_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b10_s0, b10_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(16));
                    }
                    *budget -= 1;
                    {
                        let (b11_i0, b11_s0, b11_s1,) = (b10_i0, b10_s0, b10_s1,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b11_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b11_s0, b11_s1]);
                        CompiledResume::Next(17)
                    }
                }

                fn string_int_1_resume_17(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b11_i0, b11_s0, b11_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b11_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b11_s0, b11_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(17));
                    }
                    *budget -= 1;
                    if values.bytes(b11_s0).starts_with("λ".as_bytes()) {
                        let (b12_i0, b12_s0, b12_s1,) = (b11_i0, b11_s0, b11_s1,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b12_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b12_s0, b12_s1]);
                        CompiledResume::Next(18)
                    } else {
                        let (b18_i0, b18_s0,) = (b11_i0, b11_s0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b18_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b18_s0]);
                        CompiledResume::Next(29)
                    }
                }

                fn string_int_1_resume_18(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b12_i0, b12_s0, b12_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b12_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b12_s0, b12_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(18));
                    }
                    *budget -= 1;
                    let b12_s2 = b12_s0.drop_prefix(2);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b12_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b12_s0, b12_s1, b12_s2]);
                    CompiledResume::Next(19)
                }

                fn string_int_1_resume_19(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b12_i0, b12_s0, b12_s1, b12_s2,) = (values.ints[0], values.strings[0], values.strings[1], values.strings[2],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b12_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b12_s0, b12_s1, b12_s2]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(19));
                    }
                    *budget -= 1;
                    if values.bytes(b12_s2) == values.bytes(b12_s1) {
                        let (b13_i0, b13_s0,) = (b12_i0, b12_s0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b13_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b13_s0]);
                        CompiledResume::Next(20)
                    } else {
                        let (b14_i0, b14_s0,) = (b12_i0, b12_s0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b14_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b14_s0]);
                        CompiledResume::Next(23)
                    }
                }

                fn string_int_1_resume_20(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b13_i0, b13_s0,) = (values.ints[0], values.strings[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b13_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b13_s0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(20));
                    }
                    *budget -= 1;
                    let b13_s1 = b13_s0.drop_prefix(2);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b13_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b13_s0, b13_s1]);
                    CompiledResume::Next(21)
                }

                fn string_int_1_resume_21(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b13_i0, b13_s0, b13_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b13_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b13_s0, b13_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(21));
                    }
                    *budget -= 1;
                    let b13_i1 = b13_i0 + 4_i128;
                    if b13_i1 < i128::from(i64::MIN) || b13_i1 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b13_i0, b13_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b13_s0, b13_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(22));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b13_i0, b13_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b13_s0, b13_s1]);
                    CompiledResume::Next(22)
                }

                fn string_int_1_resume_22(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b13_i0, b13_i1, b13_s0, b13_s1,) = (values.ints[0], values.ints[1], values.strings[0], values.strings[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b13_i0, b13_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b13_s0, b13_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(22));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b13_i0, b13_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b13_s0, b13_s1]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2)))
                }

                fn string_int_1_resume_23(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b14_i0, b14_s0,) = (values.ints[0], values.strings[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b14_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b14_s0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(23));
                    }
                    *budget -= 1;
                    {
                        let (b15_i0, b15_s0,) = (b14_i0, b14_s0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b15_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b15_s0]);
                        CompiledResume::Next(24)
                    }
                }

                fn string_int_1_resume_24(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b15_i0, b15_s0,) = (values.ints[0], values.strings[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b15_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b15_s0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(24));
                    }
                    *budget -= 1;
                    if values.bytes(b15_s0).starts_with("λ".as_bytes()) {
                        let (b16_i0,) = (b15_i0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b16_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        CompiledResume::Next(25)
                    } else {
                        let (b17_i0,) = (b15_i0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b17_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        CompiledResume::Next(27)
                    }
                }

                fn string_int_1_resume_25(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b16_i0,) = (values.ints[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b16_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(25));
                    }
                    *budget -= 1;
                    let b16_i1 = b16_i0 + 8_i128;
                    if b16_i1 < i128::from(i64::MIN) || b16_i1 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b16_i0, b16_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(26));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b16_i0, b16_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(26)
                }

                fn string_int_1_resume_26(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b16_i0, b16_i1,) = (values.ints[0], values.ints[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b16_i0, b16_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(26));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b16_i0, b16_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(3)))
                }

                fn string_int_1_resume_27(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b17_i0,) = (values.ints[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b17_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(27));
                    }
                    *budget -= 1;
                    let b17_i1 = b17_i0 + 16_i128;
                    if b17_i1 < i128::from(i64::MIN) || b17_i1 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b17_i0, b17_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(28));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b17_i0, b17_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(28)
                }

                fn string_int_1_resume_28(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b17_i0, b17_i1,) = (values.ints[0], values.ints[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b17_i0, b17_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(28));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b17_i0, b17_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(4)))
                }

                fn string_int_1_resume_29(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b18_i0, b18_s0,) = (values.ints[0], values.strings[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b18_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b18_s0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(29));
                    }
                    *budget -= 1;
                    {
                        let (b15_i0, b15_s0,) = (b18_i0, b18_s0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b15_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b15_s0]);
                        CompiledResume::Next(24)
                    }
                }

                fn string_int_1_resume_30(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b19_i0, b19_s0, b19_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b19_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b19_s0, b19_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(30));
                    }
                    *budget -= 1;
                    {
                        let (b10_i0, b10_s0, b10_s1,) = (b19_i0, b19_s0, b19_s1,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b10_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b10_s0, b10_s1]);
                        CompiledResume::Next(16)
                    }
                }

                fn string_int_1_resume_31(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b20_i0, b20_s0, b20_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b20_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b20_s0, b20_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(31));
                    }
                    *budget -= 1;
                    {
                        let (b11_i0, b11_s0, b11_s1,) = (b20_i0, b20_s0, b20_s1,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b11_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b11_s0, b11_s1]);
                        CompiledResume::Next(17)
                    }
                }

                fn string_int_2(
                    point: usize,
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {

                    const RESUME: [
                        fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                        12
                    ] = [
                        |values, budget| CompiledResume::Exit(string_int_2_entry((values.ints[0], values.strings[0],), values, budget)),
                        string_int_2_resume_1,
                        string_int_2_resume_2,
                        string_int_2_resume_3,
                        string_int_2_resume_4,
                        string_int_2_resume_5,
                        string_int_2_resume_6,
                        string_int_2_resume_7,
                        string_int_2_resume_8,
                        string_int_2_resume_9,
                        string_int_2_resume_10,
                        string_int_2_resume_11,
                    ];

                    let mut point = point;
                    loop {
                        match RESUME[point](values, budget) {
                            CompiledResume::Next(next) => point = next,
                            CompiledResume::Exit(progress) => return progress,
                        }
                    }
                }

                fn string_int_2_entry(
                    inputs: (i128, data::compiled::string::StringRange,),
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {
                    let (b0_i0, b0_s0,) = inputs;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b0_s0]);
                        return data::compiled::CompiledProgress::Yield(0);
                    }
                    *budget -= 1;
                    if values.bytes(b0_s0).starts_with("λ".as_bytes()) {
                        let m2 = b0_s0;
                        let m0 = data::compiled::string::StringRange::literal("λ");
                        let m1 = b0_s0.drop_prefix(2);
                        let (b1_i0, b1_s0, b1_s1, b1_s2,) = (b0_i0, m0, m1, m2,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                            return data::compiled::CompiledProgress::Yield(1);
                        }
                        *budget -= 1;
                        if values.bytes(b1_s1) != values.bytes(b1_s2) {
                            let (b2_i0, b2_s0,) = (b1_i0, b1_s0,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b2_s0]);
                                return data::compiled::CompiledProgress::Yield(2);
                            }
                            *budget -= 1;
                            let b2_s1 = data::compiled::string::StringRange::literal("λ");
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                                return data::compiled::CompiledProgress::Yield(3);
                            }
                            *budget -= 1;
                            if values.bytes(b2_s0) == values.bytes(b2_s1) {
                                let (b3_i0,) = (b2_i0,);
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);

                                    values.strings.clear();
                                    values.strings.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Yield(4);
                                }
                                *budget -= 1;
                                let b3_i1 = b3_i0 + 1_i128;
                                if b3_i1 < i128::from(i64::MIN) || b3_i1 > i128::from(i64::MAX) {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);

                                    values.strings.clear();
                                    values.strings.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Interpreted(5);
                                }
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);

                                    values.strings.clear();
                                    values.strings.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Yield(5);
                                }
                                *budget -= 1;

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                            } else {
                                let () = ();
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);

                                    values.strings.clear();
                                    values.strings.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Yield(6);
                                }
                                *budget -= 1;
                                let b4_i0 = -1_i128;
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b4_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);

                                    values.strings.clear();
                                    values.strings.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Yield(7);
                                }
                                *budget -= 1;

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b4_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                            }
                        } else {
                            let () = ();
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(8);
                            }
                            *budget -= 1;
                            let b5_i0 = -1_i128;
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b5_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(9);
                            }
                            *budget -= 1;

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b5_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2))
                        }
                    } else {
                        let (b6_s0,) = (b0_s0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b6_s0]);
                            return data::compiled::CompiledProgress::Yield(10);
                        }
                        *budget -= 1;
                        let b6_s1 = data::compiled::string::StringRange::literal("lambda required");
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b6_s0, b6_s1]);
                            return data::compiled::CompiledProgress::Yield(11);
                        }

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b6_s0, b6_s1]);
                        data::compiled::CompiledProgress::Interpreted(11)
                    }
                }

                fn string_int_2_resume_1(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_i0, b1_s0, b1_s1, b1_s2,) = (values.ints[0], values.strings[0], values.strings[1], values.strings[2],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                    }
                    *budget -= 1;
                    if values.bytes(b1_s1) != values.bytes(b1_s2) {
                        let (b2_i0, b2_s0,) = (b1_i0, b1_s0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b2_s0]);
                        CompiledResume::Next(2)
                    } else {
                        let () = ();

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        CompiledResume::Next(8)
                    }
                }

                fn string_int_2_resume_2(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b2_i0, b2_s0,) = (values.ints[0], values.strings[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b2_s0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                    }
                    *budget -= 1;
                    let b2_s1 = data::compiled::string::StringRange::literal("λ");

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                    CompiledResume::Next(3)
                }

                fn string_int_2_resume_3(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b2_i0, b2_s0, b2_s1,) = (values.ints[0], values.strings[0], values.strings[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                    }
                    *budget -= 1;
                    if values.bytes(b2_s0) == values.bytes(b2_s1) {
                        let (b3_i0,) = (b2_i0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        CompiledResume::Next(4)
                    } else {
                        let () = ();

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        CompiledResume::Next(6)
                    }
                }

                fn string_int_2_resume_4(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b3_i0,) = (values.ints[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                    }
                    *budget -= 1;
                    let b3_i1 = b3_i0 + 1_i128;
                    if b3_i1 < i128::from(i64::MIN) || b3_i1 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(5));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(5)
                }

                fn string_int_2_resume_5(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b3_i0, b3_i1,) = (values.ints[0], values.ints[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
                }

                fn string_int_2_resume_6(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                    }
                    *budget -= 1;
                    let b4_i0 = -1_i128;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(7)
                }

                fn string_int_2_resume_7(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b4_i0,) = (values.ints[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
                }

                fn string_int_2_resume_8(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                    }
                    *budget -= 1;
                    let b5_i0 = -1_i128;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(9)
                }

                fn string_int_2_resume_9(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b5_i0,) = (values.ints[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b5_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(9));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2)))
                }

                fn string_int_2_resume_10(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b6_s0,) = (values.strings[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b6_s0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(10));
                    }
                    *budget -= 1;
                    let b6_s1 = data::compiled::string::StringRange::literal("lambda required");

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b6_s0, b6_s1]);
                    CompiledResume::Next(11)
                }

                fn string_int_2_resume_11(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b6_s0, b6_s1,) = (values.strings[0], values.strings[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b6_s0, b6_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(11));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b6_s0, b6_s1]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(11))
                }

                fn string_int_4(
                    point: usize,
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {

                    const RESUME: [
                        fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                        4
                    ] = [
                        |values, budget| CompiledResume::Exit(string_int_4_entry((values.strings[0],), values, budget)),
                        string_int_4_resume_1,
                        string_int_4_resume_2,
                        string_int_4_resume_3,
                    ];

                    let mut point = point;
                    loop {
                        match RESUME[point](values, budget) {
                            CompiledResume::Next(next) => point = next,
                            CompiledResume::Exit(progress) => return progress,
                        }
                    }
                }

                fn string_int_4_entry(
                    inputs: (data::compiled::string::StringRange,),
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {
                    let (mut b0_s0,) = inputs;
                    'repeat: loop {
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b0_s0]);
                            return data::compiled::CompiledProgress::Yield(0);
                        }
                        *budget -= 1;
                        if values.bytes(b0_s0).starts_with("λ".as_bytes()) {
                            let (b1_s0,) = (b0_s0,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b1_s0]);
                                return data::compiled::CompiledProgress::Yield(1);
                            }
                            *budget -= 1;
                            {
                                (b0_s0,) = (b1_s0,);
                                continue 'repeat;
                            }
                        } else {
                            let () = ();
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(2);
                            }
                            *budget -= 1;
                            let b2_i0 = 0_i128;
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(3);
                            }
                            *budget -= 1;

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0));
                        }
                    }
                }

                fn string_int_4_resume_1(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_s0,) = (values.strings[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                    }
                    *budget -= 1;
                    {
                        let (b0_s0,) = (b1_s0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b0_s0]);
                        CompiledResume::Next(0)
                    }
                }

                fn string_int_4_resume_2(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                    }
                    *budget -= 1;
                    let b2_i0 = 0_i128;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(3)
                }

                fn string_int_4_resume_3(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b2_i0,) = (values.ints[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
                }

                fn string_bool_0(
                    point: usize,
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {

                    const RESUME: [
                        fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                        7
                    ] = [
                        |values, budget| CompiledResume::Exit(string_bool_0_entry((values.bools[0], values.strings[0], values.strings[1],), values, budget)),
                        string_bool_0_resume_1,
                        string_bool_0_resume_2,
                        string_bool_0_resume_3,
                        string_bool_0_resume_4,
                        string_bool_0_resume_5,
                        string_bool_0_resume_6,
                    ];

                    let mut point = point;
                    loop {
                        match RESUME[point](values, budget) {
                            CompiledResume::Next(next) => point = next,
                            CompiledResume::Exit(progress) => return progress,
                        }
                    }
                }

                fn string_bool_0_entry(
                    inputs: (bool, data::compiled::string::StringRange, data::compiled::string::StringRange,),
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {
                    let (b0_v0, b0_s0, b0_s1,) = inputs;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b0_v0]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                        return data::compiled::CompiledProgress::Yield(0);
                    }
                    *budget -= 1;
                    let b0_v1 = values.bytes(b0_s0) == values.bytes(b0_s1);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b0_v0, b0_v1]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                        return data::compiled::CompiledProgress::Yield(1);
                    }
                    *budget -= 1;
                    let b0_v2 = values.bytes(b0_s0) != values.bytes(b0_s1);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b0_v0, b0_v1, b0_v2]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                        return data::compiled::CompiledProgress::Yield(2);
                    }
                    *budget -= 1;
                    if b0_v1 == b0_v0 {
                        let (b1_v0, b1_v1,) = (b0_v0, b0_v2,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b1_v0, b1_v1]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(3);
                        }
                        *budget -= 1;
                        let b1_v2 = b1_v1 != b1_v0;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b1_v0, b1_v1, b1_v2]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(4);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0, b1_v1, b1_v2]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                    } else {
                        let () = ();
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(5);
                        }
                        *budget -= 1;
                        let b2_v0 = false;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b2_v0]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(6);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b2_v0]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                    }
                }

                fn string_bool_0_resume_1(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b0_v0, b0_v1, b0_s0, b0_s1,) = (values.bools[0], values.bools[1], values.strings[0], values.strings[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b0_v0, b0_v1]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                    }
                    *budget -= 1;
                    let b0_v2 = values.bytes(b0_s0) != values.bytes(b0_s1);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0, b0_v1, b0_v2]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                    CompiledResume::Next(2)
                }

                fn string_bool_0_resume_2(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b0_v0, b0_v1, b0_v2, b0_s0, b0_s1,) = (values.bools[0], values.bools[1], values.bools[2], values.strings[0], values.strings[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b0_v0, b0_v1, b0_v2]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                    }
                    *budget -= 1;
                    if b0_v1 == b0_v0 {
                        let (b1_v0, b1_v1,) = (b0_v0, b0_v2,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0, b1_v1]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        CompiledResume::Next(3)
                    } else {
                        let () = ();

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        CompiledResume::Next(5)
                    }
                }

                fn string_bool_0_resume_3(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_v0, b1_v1,) = (values.bools[0], values.bools[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0, b1_v1]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                    }
                    *budget -= 1;
                    let b1_v2 = b1_v1 != b1_v0;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0, b1_v1, b1_v2]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(4)
                }

                fn string_bool_0_resume_4(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_v0, b1_v1, b1_v2,) = (values.bools[0], values.bools[1], values.bools[2],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0, b1_v1, b1_v2]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0, b1_v1, b1_v2]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
                }

                fn string_bool_0_resume_5(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                    }
                    *budget -= 1;
                    let b2_v0 = false;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b2_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(6)
                }

                fn string_bool_0_resume_6(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b2_v0,) = (values.bools[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b2_v0]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b2_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
                }

                fn string_bool_1(
                    point: usize,
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {

                    const RESUME: [
                        fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                        11
                    ] = [
                        |values, budget| CompiledResume::Exit(string_bool_1_entry((values.strings[0],), values, budget)),
                        string_bool_1_resume_1,
                        string_bool_1_resume_2,
                        string_bool_1_resume_3,
                        string_bool_1_resume_4,
                        string_bool_1_resume_5,
                        string_bool_1_resume_6,
                        string_bool_1_resume_7,
                        string_bool_1_resume_8,
                        string_bool_1_resume_9,
                        string_bool_1_resume_10,
                    ];

                    let mut point = point;
                    loop {
                        match RESUME[point](values, budget) {
                            CompiledResume::Next(next) => point = next,
                            CompiledResume::Exit(progress) => return progress,
                        }
                    }
                }

                fn string_bool_1_entry(
                    inputs: (data::compiled::string::StringRange,),
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {
                    let (b0_s0,) = inputs;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b0_s0]);
                        return data::compiled::CompiledProgress::Yield(0);
                    }
                    *budget -= 1;
                    if values.bytes(b0_s0).starts_with("".as_bytes()) {
                        let (b1_s0,) = (b0_s0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b1_s0]);
                            return data::compiled::CompiledProgress::Yield(1);
                        }
                        *budget -= 1;
                        let b1_s1 = data::compiled::string::StringRange::literal("");
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                            return data::compiled::CompiledProgress::Yield(2);
                        }
                        *budget -= 1;
                        let b1_s2 = b1_s0.drop_prefix(0);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                            return data::compiled::CompiledProgress::Yield(3);
                        }
                        *budget -= 1;
                        let b1_s3 = data::compiled::string::StringRange::literal("");
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2, b1_s3]);
                            return data::compiled::CompiledProgress::Yield(4);
                        }
                        *budget -= 1;
                        if values.bytes(b1_s1) == values.bytes(b1_s3) {
                            let (b2_s0, b2_s1,) = (b1_s0, b1_s2,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                                return data::compiled::CompiledProgress::Yield(5);
                            }
                            *budget -= 1;
                            let b2_v0 = values.bytes(b2_s1) == values.bytes(b2_s0);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[b2_v0]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                                return data::compiled::CompiledProgress::Yield(6);
                            }
                            *budget -= 1;

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b2_v0]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                            data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                        } else {
                            let () = ();
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(7);
                            }
                            *budget -= 1;
                            let b3_v0 = false;
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[b3_v0]);

                                values.strings.clear();
                                values.strings.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(8);
                            }
                            *budget -= 1;

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b3_v0]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                        }
                    } else {
                        let () = ();
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(9);
                        }
                        *budget -= 1;
                        let b4_v0 = false;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b4_v0]);

                            values.strings.clear();
                            values.strings.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(10);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b4_v0]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2))
                    }
                }

                fn string_bool_1_resume_1(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_s0,) = (values.strings[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                    }
                    *budget -= 1;
                    let b1_s1 = data::compiled::string::StringRange::literal("");

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                    CompiledResume::Next(2)
                }

                fn string_bool_1_resume_2(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_s0, b1_s1,) = (values.strings[0], values.strings[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0, b1_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                    }
                    *budget -= 1;
                    let b1_s2 = b1_s0.drop_prefix(0);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                    CompiledResume::Next(3)
                }

                fn string_bool_1_resume_3(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_s0, b1_s1, b1_s2,) = (values.strings[0], values.strings[1], values.strings[2],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                    }
                    *budget -= 1;
                    let b1_s3 = data::compiled::string::StringRange::literal("");

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2, b1_s3]);
                    CompiledResume::Next(4)
                }

                fn string_bool_1_resume_4(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_s0, b1_s1, b1_s2, b1_s3,) = (values.strings[0], values.strings[1], values.strings[2], values.strings[3],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b1_s0, b1_s1, b1_s2, b1_s3]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                    }
                    *budget -= 1;
                    if values.bytes(b1_s1) == values.bytes(b1_s3) {
                        let (b2_s0, b2_s1,) = (b1_s0, b1_s2,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                        CompiledResume::Next(5)
                    } else {
                        let () = ();

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        CompiledResume::Next(7)
                    }
                }

                fn string_bool_1_resume_5(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b2_s0, b2_s1,) = (values.strings[0], values.strings[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                    }
                    *budget -= 1;
                    let b2_v0 = values.bytes(b2_s1) == values.bytes(b2_s0);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b2_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                    CompiledResume::Next(6)
                }

                fn string_bool_1_resume_6(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b2_v0, b2_s0, b2_s1,) = (values.bools[0], values.strings[0], values.strings[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b2_v0]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b2_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b2_s0, b2_s1]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
                }

                fn string_bool_1_resume_7(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                    }
                    *budget -= 1;
                    let b3_v0 = false;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b3_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(8)
                }

                fn string_bool_1_resume_8(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b3_v0,) = (values.bools[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b3_v0]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b3_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
                }

                fn string_bool_1_resume_9(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(9));
                    }
                    *budget -= 1;
                    let b4_v0 = false;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b4_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Next(10)
                }

                fn string_bool_1_resume_10(
                    values: &mut data::compiled::string::StringValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b4_v0,) = (values.bools[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b4_v0]);

                        values.strings.clear();
                        values.strings.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(10));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b4_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2)))
                }
                data::compiled::CompiledFunctions {
                    ints: data::Storage::Static(&[
                        data::compiled::CompiledFunction {
                            function: data::function::IntFunctionId(0),
                            implementation: data::compiled::CompiledImplementation::String(data::compiled::StringImplementation {
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 1,
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
                                        strings: 1,
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
                                        strings: 2,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 2,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
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
                                        strings: 1,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 1,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
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
                                        block: data::graph::BlockId(4),
                                        instruction: 1,
                                        ints: 0,
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
                                run: string_int_0,
                            }),
                        },
                        data::compiled::CompiledFunction {
                            function: data::function::IntFunctionId(1),
                            implementation: data::compiled::CompiledImplementation::String(data::compiled::StringImplementation {
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 0,
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
                                        strings: 2,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 1,
                                        ints: 1,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 3,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
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
                                        strings: 1,
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
                                        strings: 2,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(3),
                                        instruction: 2,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 3,
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
                                        strings: 1,
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
                                        int_lists: 0,
                                        strings: 2,
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
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(5),
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
                                        block: data::graph::BlockId(6),
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
                                        block: data::graph::BlockId(7),
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
                                        block: data::graph::BlockId(7),
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
                                        block: data::graph::BlockId(8),
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
                                        block: data::graph::BlockId(9),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(10),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(11),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(12),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(12),
                                        instruction: 1,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 3,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(13),
                                        instruction: 0,
                                        ints: 1,
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
                                        block: data::graph::BlockId(13),
                                        instruction: 1,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(13),
                                        instruction: 2,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(14),
                                        instruction: 0,
                                        ints: 1,
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
                                        block: data::graph::BlockId(15),
                                        instruction: 0,
                                        ints: 1,
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
                                        block: data::graph::BlockId(16),
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
                                        block: data::graph::BlockId(16),
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
                                        block: data::graph::BlockId(17),
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
                                        block: data::graph::BlockId(17),
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
                                        block: data::graph::BlockId(18),
                                        instruction: 0,
                                        ints: 1,
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
                                        block: data::graph::BlockId(19),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(20),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                run: string_int_1,
                            }),
                        },
                        data::compiled::CompiledFunction {
                            function: data::function::IntFunctionId(2),
                            implementation: data::compiled::CompiledImplementation::String(data::compiled::StringImplementation {
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 1,
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
                                        strings: 3,
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
                                        strings: 1,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 1,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
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
                                        block: data::graph::BlockId(3),
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
                                        block: data::graph::BlockId(4),
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
                                        block: data::graph::BlockId(4),
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
                                        block: data::graph::BlockId(5),
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
                                        block: data::graph::BlockId(5),
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
                                        block: data::graph::BlockId(6),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(6),
                                        instruction: 1,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                run: string_int_2,
                            }),
                        },
                        data::compiled::CompiledFunction {
                            function: data::function::IntFunctionId(4),
                            implementation: data::compiled::CompiledImplementation::String(data::compiled::StringImplementation {
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 0,
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
                                        ints: 0,
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
                                run: string_int_4,
                            }),
                        },
                    ]),
                    bools: data::Storage::Static(&[
                        data::compiled::CompiledFunction {
                            function: data::function::BoolFunctionId(0),
                            implementation: data::compiled::CompiledImplementation::String(data::compiled::StringImplementation {
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
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
                                        strings: 2,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 0,
                                        bools: 3,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 0,
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
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 1,
                                        ints: 0,
                                        bools: 3,
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
                                run: string_bool_0,
                            }),
                        },
                        data::compiled::CompiledFunction {
                            function: data::function::BoolFunctionId(1),
                            implementation: data::compiled::CompiledImplementation::String(data::compiled::StringImplementation {
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 0,
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
                                        ints: 0,
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
                                        instruction: 1,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 2,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 2,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 3,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 3,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 4,
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
                                        strings: 2,
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
                                        strings: 2,
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
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(3),
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
                                        block: data::graph::BlockId(4),
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
                                        block: data::graph::BlockId(4),
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
                                run: string_bool_1,
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
                    5..7,
                    0..0,
                    7..8,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
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
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 2..6,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(2),
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 6..8,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 8..9,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 9..10,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 10..13,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(2),
                        ]),
                        return_: data::type_::ValueShapeId(2),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 13..14,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(2),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 14..16,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(4),
                        captures: data::Storage::Static(&[]),
                    },
                ]),
                parameters: data::Storage::Static(&[
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
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
                    data::type_::ValueShapeDescriptor::String,
                    data::type_::ValueShapeDescriptor::Int,
                    data::type_::ValueShapeDescriptor::Bool,
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(1)),
                    data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(1),
                        data::type_::ValueShapeId(3),
                        data::type_::ValueShapeId(2),
                    ])),
                ]),
                shape_types: data::Storage::Static(&[
                    data::type_::ValueType::String,
                    data::type_::ValueType::Int,
                    data::type_::ValueType::Bool,
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::String,
                        data::type_::ValueType::Int,
                        data::type_::ValueType::List(data::type_::ListTypeId(0)),
                        data::type_::ValueType::Bool,
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
            ]),
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
                name: data::Text::Static("count"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("aliases"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                        data::type_::TypeMetadata::String,
                        data::type_::TypeMetadata::Bool,
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 1,
            },
            data::Export {
                name: data::Text::Static("same"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                        data::type_::TypeMetadata::String,
                        data::type_::TypeMetadata::Bool,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("empty_prefix"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
                },
                slot: 1,
            },
            data::Export {
                name: data::Text::Static("asserted"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 2,
            },
            data::Export {
                name: data::Text::Static("caller"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Tuple(data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                        data::type_::TypeMetadata::Bool,
                    ]))),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("running"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::String,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 3,
            },
        ]),
    },
    value_functions: data::Storage::Static(&[]),
    never_functions: data::Storage::Static(&[]),
    callables: data::Storage::Static(&[]),
}
