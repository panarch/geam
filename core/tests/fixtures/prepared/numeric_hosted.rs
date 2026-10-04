data::HostedModuleArtifact {
    module: data::ModuleArtifact {
        format: 15,
        program: data::ProgramTables {
            root: data::source::module_id(0),
            modules: data::Storage::Static(&[
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("example"),
                    source_context: Some(data::source::SourceContext::from_static_block("src/example.gleam", r#"
pub fn arithmetic(remaining: Int, total: Int) -> Int {
  case remaining {
    0 -> total
    n ->
      case n % 2 == 0 {
        True -> arithmetic(n - 1, total + n * 3 + 1)
        False -> arithmetic(n - 1, total + n * 2 - 1)
      }
  }
}

pub fn shuffle(remaining: Int, left: Int, flag: Bool, right: Int) -> Int {
  case remaining <= 0 {
    True ->
      case flag {
        True -> left - right
        False -> right - left
      }
    False -> shuffle(remaining - 1, right, !flag, left)
  }
}

pub fn choice(value: Int, flag: Bool) -> Bool {
  case value < 0 {
    True -> !flag
    False ->
      case value >= 10 {
        True -> flag
        False -> False
      }
  }
}

pub fn switch(value: Int) -> Int {
  let selected = case value {
    0 -> -3
    1 -> 2
    _ -> value
  }
  case selected > 0 {
    True -> selected + 42
    False -> -selected
  }
}

pub fn quotient(left: Int, right: Int, negate: Bool) -> Int {
  let divided = left / right
  case negate {
    True -> -divided
    False -> divided
  }
}

pub fn operators(left: Int, right: Int, flag: Bool) -> Int {
  let product = left * right + left - right
  let value = -product
  case flag {
    True -> value / right
    False -> value % right
  }
}

pub fn divmod(left: Int, right: Int, flag: Bool) -> Int {
  let value = left / right + left % right
  case flag {
    True -> value
    False -> -value
  }
}

pub fn product(left: Int, right: Int) -> Int {
  case left != right {
    True -> left * right
    False -> left
  }
}

pub fn discarded(value: Int, flag: Bool) -> Int {
  let _discarded = value + 1 + 2
  case flag {
    True -> value
    False -> -value
  }
}

pub fn captured(value: Int, offset: Int) -> Int {
  let calculate = fn(input) {
    case input >= 0 {
      True -> input + offset
      False -> offset - input
    }
  }
  calculate(value)
}

pub fn caller(
  value: Int,
  remaining: Int,
  text: String,
) -> #(String, Int, List(Int), Int) {
  let values = [value, remaining]
  let first = arithmetic(remaining, value)
  #(text, first, values, captured(value, remaining))
}

pub fn main() -> Int {
  arithmetic(12, 0) + shuffle(3, 7, True, 11) + switch(1)
}

pub fn running() -> Int {
  echo "entered"
  arithmetic(-1, 0)
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
                                            terminator: data::graph::Terminator::IntSwitch(data::graph::IntSwitch {
                                                subject: data::graph::IntLocalId(0),
                                                clauses: data::Storage::Static(&[
                                                    (data::graph::IntegerLiteral {
                                                        sign: data::Sign::NoSign,
                                                        digits: data::Storage::Static(&[]),
                                                    }, data::graph::Edge {
                                                        target: data::graph::BlockId(1),
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
                                                            ]),
                                                        },
                                                    }),
                                                ]),
                                                fallback: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::EqualInt {
                                                    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
                                                    right: data::graph::IntegerOperand::Immediate(0),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(3),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 2,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(4),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 2,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 5..7,
                                            instructions: 1..2,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
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
                                        data::graph::BlockHeader {
                                            params: 7..9,
                                            instructions: 2..3,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
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
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Remainder {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(2),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::IntegerRegion(data::graph::ArithmeticRegion {
                                            inputs: data::Storage::Static(&[
                                                data::graph::IntLocalId(0),
                                                data::graph::IntLocalId(1),
                                            ]),
                                            nodes: data::Storage::Static(&[
                                                data::graph::ArithmeticNode::Subtract(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Immediate(1)),
                                                data::graph::ArithmeticNode::Multiply(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Immediate(3)),
                                                data::graph::ArithmeticNode::Add(data::graph::ArithmeticOperand::Input(1), data::graph::ArithmeticOperand::Value(1)),
                                                data::graph::ArithmeticNode::Add(data::graph::ArithmeticOperand::Value(2), data::graph::ArithmeticOperand::Immediate(1)),
                                            ]),
                                            outputs: data::Storage::Static(&[
                                                data::graph::ArithmeticOutput {
                                                    value: 0,
                                                    slot: data::graph::ParamSlot {
                                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                        shape: data::type_::ValueShapeId(0),
                                                    },
                                                },
                                                data::graph::ArithmeticOutput {
                                                    value: 3,
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
                                                data::graph::IntLocalId(1),
                                            ]),
                                            nodes: data::Storage::Static(&[
                                                data::graph::ArithmeticNode::Subtract(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Immediate(1)),
                                                data::graph::ArithmeticNode::Multiply(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Immediate(2)),
                                                data::graph::ArithmeticNode::Add(data::graph::ArithmeticOperand::Input(1), data::graph::ArithmeticOperand::Value(1)),
                                                data::graph::ArithmeticNode::Subtract(data::graph::ArithmeticOperand::Value(2), data::graph::ArithmeticOperand::Immediate(1)),
                                            ]),
                                            outputs: data::Storage::Static(&[
                                                data::graph::ArithmeticOutput {
                                                    value: 0,
                                                    slot: data::graph::ParamSlot {
                                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                        shape: data::type_::ValueShapeId(0),
                                                    },
                                                },
                                                data::graph::ArithmeticOutput {
                                                    value: 3,
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
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                ]),
                            },
                        })),
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
                                            instructions: 0..12,
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
                                                    12,
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
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2101, 2118)),
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
                                                    3,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
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
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    11,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(6)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(4),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2121, 2144)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(6)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
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
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(9)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(5),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2147, 2156)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(10)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(7)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(9)),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(10)),
                                ]),
                            },
                        })),
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
                                                site: data::source::EchoSite::from_static("example", "running", data::source::SourceSpan::new(2188, 2202)),
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
                                            instructions: 1..3,
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
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("entered"))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(0),
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
                                            function: data::function::IntFunctionId(0),
                                            site: data::source::HostCallSite::from_static("example", "running", data::source::SourceSpan::new(2205, 2222)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                parameter_count: 2,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 0..2,
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
                                                local: data::graph::ParamLocal::IntFunction {
                                                    local: data::graph::IntFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(5),
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
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(6)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::Int {
                                                            target: data::graph::IntLocalId(1),
                                                            source: data::graph::IntLocalId(1),
                                                        },
                                                    ]),
                                                },
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
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "captured", data::source::SourceSpan::new(1822, 1838)),
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
                                                test: data::graph::BoolTest::LtEqInt {
                                                    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                    right: data::graph::IntegerOperand::Immediate(0),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                                    target: data::graph::BlockId(4),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 4..7,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                                subject: data::graph::BoolLocalId(0),
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                                    target: data::graph::BlockId(3),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 7..9,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 9..11,
                                            instructions: 1..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 11..15,
                                            instructions: 2..4,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 3,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 3,
                                                                        destination: 0,
                                                                    },
                                                                    data::graph::TransferStep {
                                                                        source: 2,
                                                                        destination: 1,
                                                                    },
                                                                ]),
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
                                                        ]),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
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
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            shape: data::type_::ValueShapeId(0),
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
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            }),
                                        }),
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
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Not(data::graph::BoolLocalId(0)))),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(2)),
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(2)),
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
                                            params: 1..1,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
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
                                            params: 1..2,
                                            instructions: 1..1,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::GtInt {
                                                    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                    right: data::graph::IntegerOperand::Immediate(0),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(3),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
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
                                            params: 2..3,
                                            instructions: 1..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 3..4,
                                            instructions: 2..3,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 4..4,
                                            instructions: 3..4,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
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
                                            params: 4..5,
                                            instructions: 4..4,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
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
                                                sign: data::Sign::Minus,
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
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(42),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Negate(data::graph::IntLocalId(0))),
                                        }),
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
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
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
                                            params: 0..2,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::GtEqInt {
                                                    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                    right: data::graph::IntegerOperand::Immediate(0),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                            instructions: 1..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
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
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(2)),
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(2)),
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
                                                test: data::graph::BoolTest::LtInt {
                                                    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                    right: data::graph::IntegerOperand::Immediate(0),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
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
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..3,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 3..5,
                                            instructions: 1..1,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::GtEqInt {
                                                    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                    right: data::graph::IntegerOperand::Immediate(10),
                                                },
                                                true_: data::graph::Edge {
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
                                            params: 5..6,
                                            instructions: 1..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 6..6,
                                            instructions: 1..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Not(data::graph::BoolLocalId(0)))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(false)),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::BoolLocalId(1)),
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
                                parameter_count: 3,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..3,
                                            instructions: 0..4,
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
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
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
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "caller", data::source::SourceSpan::new(1991, 2019)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "caller", data::source::SourceSpan::new(2045, 2071)),
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
                                                shape: data::type_::ValueShapeId(4),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                                data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
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

                fn numeric_int_0(
                    point: usize,
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {

                    const RESUME: [
                        fn(&mut data::compiled::numeric::NumericValues, &mut usize) -> CompiledResume;
                        8
                    ] = [
                        |values, budget| CompiledResume::Exit(numeric_int_0_entry((values.ints[0], values.ints[1],), values, budget)),
                        numeric_int_0_resume_1,
                        numeric_int_0_resume_2,
                        numeric_int_0_resume_3,
                        numeric_int_0_resume_4,
                        numeric_int_0_resume_5,
                        numeric_int_0_resume_6,
                        numeric_int_0_resume_7,
                    ];

                    let mut point = point;
                    loop {
                        match RESUME[point](values, budget) {
                            CompiledResume::Next(next) => point = next,
                            CompiledResume::Exit(progress) => return progress,
                        }
                    }
                }

                fn numeric_int_0_entry(
                    inputs: (i128, i128,),
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {
                    let (mut b0_i0, mut b0_i1,) = inputs;
                    'repeat: loop {
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(0);
                        }
                        *budget -= 1;
                        if b0_i0 == 0_i128 {
                            let (b1_i0,) = (b0_i1,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b1_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(1);
                            }
                            *budget -= 1;

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0));
                        } else {
                            let (b2_i0, b2_i1,) = (b0_i0, b0_i1,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(2);
                            }
                            *budget -= 1;
                            let b2_i2 = if 2_i128 == 0 { 0_i128 } else { b2_i0 % 2_i128 };
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(3);
                            }
                            *budget -= 1;
                            if b2_i2 == 0_i128 {
                                let (b3_i0, b3_i1,) = (b2_i0, b2_i1,);
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Yield(4);
                                }
                                *budget -= 1;
                                let _r0_n0 = b3_i0 - 1_i128;
                                let _r0_n1 = b3_i0 * 3_i128;
                                let _r0_n2 = b3_i1 + _r0_n1;
                                let _r0_n3 = _r0_n2 + 1_i128;
                                let b3_i2 = _r0_n0;
                                let b3_i3 = _r0_n3;
                                if b3_i2 < i128::from(i64::MIN) || b3_i2 > i128::from(i64::MAX) || b3_i3 < i128::from(i64::MIN) || b3_i3 > i128::from(i64::MAX) {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Interpreted(5);
                                }
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Yield(5);
                                }
                                *budget -= 1;
                                {
                                    (b0_i0, b0_i1,) = (b3_i2, b3_i3,);
                                    continue 'repeat;
                                }
                            } else {
                                let (b4_i0, b4_i1,) = (b2_i0, b2_i1,);
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Yield(6);
                                }
                                *budget -= 1;
                                let _r0_n0 = b4_i0 - 1_i128;
                                let _r0_n1 = b4_i0 * 2_i128;
                                let _r0_n2 = b4_i1 + _r0_n1;
                                let _r0_n3 = _r0_n2 - 1_i128;
                                let b4_i2 = _r0_n0;
                                let b4_i3 = _r0_n3;
                                if b4_i2 < i128::from(i64::MIN) || b4_i2 > i128::from(i64::MAX) || b4_i3 < i128::from(i64::MIN) || b4_i3 > i128::from(i64::MAX) {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Interpreted(7);
                                }
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Yield(7);
                                }
                                *budget -= 1;
                                {
                                    (b0_i0, b0_i1,) = (b4_i2, b4_i3,);
                                    continue 'repeat;
                                }
                            }
                        }
                    }
                }

                fn numeric_int_0_resume_1(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_i0,) = (values.ints[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
                }

                fn numeric_int_0_resume_2(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b2_i0, b2_i1,) = (values.ints[0], values.ints[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                    }
                    *budget -= 1;
                    let b2_i2 = if 2_i128 == 0 { 0_i128 } else { b2_i0 % 2_i128 };

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(3)
                }

                fn numeric_int_0_resume_3(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b2_i0, b2_i1, b2_i2,) = (values.ints[0], values.ints[1], values.ints[2],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                    }
                    *budget -= 1;
                    if b2_i2 == 0_i128 {
                        let (b3_i0, b3_i1,) = (b2_i0, b2_i1,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        CompiledResume::Next(4)
                    } else {
                        let (b4_i0, b4_i1,) = (b2_i0, b2_i1,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        CompiledResume::Next(6)
                    }
                }

                fn numeric_int_0_resume_4(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b3_i0, b3_i1,) = (values.ints[0], values.ints[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                    }
                    *budget -= 1;
                    let _r0_n0 = b3_i0 - 1_i128;
                    let _r0_n1 = b3_i0 * 3_i128;
                    let _r0_n2 = b3_i1 + _r0_n1;
                    let _r0_n3 = _r0_n2 + 1_i128;
                    let b3_i2 = _r0_n0;
                    let b3_i3 = _r0_n3;
                    if b3_i2 < i128::from(i64::MIN) || b3_i2 > i128::from(i64::MAX) || b3_i3 < i128::from(i64::MIN) || b3_i3 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(5));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(5)
                }

                fn numeric_int_0_resume_5(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b3_i0, b3_i1, b3_i2, b3_i3,) = (values.ints[0], values.ints[1], values.ints[2], values.ints[3],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2, b3_i3]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                    }
                    *budget -= 1;
                    {
                        let (b0_i0, b0_i1,) = (b3_i2, b3_i3,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        CompiledResume::Next(0)
                    }
                }

                fn numeric_int_0_resume_6(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b4_i0, b4_i1,) = (values.ints[0], values.ints[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                    }
                    *budget -= 1;
                    let _r0_n0 = b4_i0 - 1_i128;
                    let _r0_n1 = b4_i0 * 2_i128;
                    let _r0_n2 = b4_i1 + _r0_n1;
                    let _r0_n3 = _r0_n2 - 1_i128;
                    let b4_i2 = _r0_n0;
                    let b4_i3 = _r0_n3;
                    if b4_i2 < i128::from(i64::MIN) || b4_i2 > i128::from(i64::MAX) || b4_i3 < i128::from(i64::MIN) || b4_i3 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(7));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(7)
                }

                fn numeric_int_0_resume_7(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b4_i0, b4_i1, b4_i2, b4_i3,) = (values.ints[0], values.ints[1], values.ints[2], values.ints[3],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                    }
                    *budget -= 1;
                    {
                        let (b0_i0, b0_i1,) = (b4_i2, b4_i3,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        CompiledResume::Next(0)
                    }
                }

                fn numeric_int_4(
                    point: usize,
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {

                    const RESUME: [
                        fn(&mut data::compiled::numeric::NumericValues, &mut usize) -> CompiledResume;
                        9
                    ] = [
                        |values, budget| CompiledResume::Exit(numeric_int_4_entry((values.ints[0], values.ints[1], values.ints[2], values.bools[0],), values, budget)),
                        numeric_int_4_resume_1,
                        numeric_int_4_resume_2,
                        numeric_int_4_resume_3,
                        numeric_int_4_resume_4,
                        numeric_int_4_resume_5,
                        numeric_int_4_resume_6,
                        numeric_int_4_resume_7,
                        numeric_int_4_resume_8,
                    ];

                    let mut point = point;
                    loop {
                        match RESUME[point](values, budget) {
                            CompiledResume::Next(next) => point = next,
                            CompiledResume::Exit(progress) => return progress,
                        }
                    }
                }

                fn numeric_int_4_entry(
                    inputs: (i128, i128, i128, bool,),
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {
                    let (mut b0_i0, mut b0_i1, mut b0_i2, mut b0_v0,) = inputs;
                    'repeat: loop {
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b0_v0]);
                            return data::compiled::CompiledProgress::Yield(0);
                        }
                        *budget -= 1;
                        if b0_i0 <= 0_i128 {
                            let (b1_i0, b1_i1, b1_v0,) = (b0_i1, b0_i2, b0_v0,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[b1_v0]);
                                return data::compiled::CompiledProgress::Yield(1);
                            }
                            *budget -= 1;
                            if b1_v0 {
                                let (b2_i0, b2_i1,) = (b1_i0, b1_i1,);
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Yield(2);
                                }
                                *budget -= 1;
                                let b2_i2 = b2_i0 - b2_i1;
                                if b2_i2 < i128::from(i64::MIN) || b2_i2 > i128::from(i64::MAX) {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Interpreted(3);
                                }
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Yield(3);
                                }
                                *budget -= 1;

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0));
                            } else {
                                let (b3_i0, b3_i1,) = (b1_i0, b1_i1,);
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Yield(4);
                                }
                                *budget -= 1;
                                let b3_i2 = b3_i1 - b3_i0;
                                if b3_i2 < i128::from(i64::MIN) || b3_i2 > i128::from(i64::MAX) {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Interpreted(5);
                                }
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    return data::compiled::CompiledProgress::Yield(5);
                                }
                                *budget -= 1;

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1));
                            }
                        } else {
                            let (b4_i0, b4_i1, b4_i2, b4_v0,) = (b0_i0, b0_i1, b0_i2, b0_v0,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[b4_v0]);
                                return data::compiled::CompiledProgress::Yield(6);
                            }
                            *budget -= 1;
                            let b4_i3 = b4_i0 - 1_i128;
                            if b4_i3 < i128::from(i64::MIN) || b4_i3 > i128::from(i64::MAX) {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[b4_v0]);
                                return data::compiled::CompiledProgress::Interpreted(7);
                            }
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[b4_v0]);
                                return data::compiled::CompiledProgress::Yield(7);
                            }
                            *budget -= 1;
                            let b4_v1 = !b4_v0;
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[b4_v0, b4_v1]);
                                return data::compiled::CompiledProgress::Yield(8);
                            }
                            *budget -= 1;
                            {
                                (b0_i0, b0_i1, b0_i2, b0_v0,) = (b4_i3, b4_i2, b4_i1, b4_v1,);
                                continue 'repeat;
                            }
                        }
                    }
                }

                fn numeric_int_4_resume_1(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_i0, b1_i1, b1_v0,) = (values.ints[0], values.ints[1], values.bools[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                    }
                    *budget -= 1;
                    if b1_v0 {
                        let (b2_i0, b2_i1,) = (b1_i0, b1_i1,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        CompiledResume::Next(2)
                    } else {
                        let (b3_i0, b3_i1,) = (b1_i0, b1_i1,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        CompiledResume::Next(4)
                    }
                }

                fn numeric_int_4_resume_2(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b2_i0, b2_i1,) = (values.ints[0], values.ints[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                    }
                    *budget -= 1;
                    let b2_i2 = b2_i0 - b2_i1;
                    if b2_i2 < i128::from(i64::MIN) || b2_i2 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(3));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(3)
                }

                fn numeric_int_4_resume_3(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b2_i0, b2_i1, b2_i2,) = (values.ints[0], values.ints[1], values.ints[2],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
                }

                fn numeric_int_4_resume_4(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b3_i0, b3_i1,) = (values.ints[0], values.ints[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                    }
                    *budget -= 1;
                    let b3_i2 = b3_i1 - b3_i0;
                    if b3_i2 < i128::from(i64::MIN) || b3_i2 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(5));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(5)
                }

                fn numeric_int_4_resume_5(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b3_i0, b3_i1, b3_i2,) = (values.ints[0], values.ints[1], values.ints[2],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
                }

                fn numeric_int_4_resume_6(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b4_i0, b4_i1, b4_i2, b4_v0,) = (values.ints[0], values.ints[1], values.ints[2], values.bools[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b4_v0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                    }
                    *budget -= 1;
                    let b4_i3 = b4_i0 - 1_i128;
                    if b4_i3 < i128::from(i64::MIN) || b4_i3 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b4_v0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(7));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b4_v0]);
                    CompiledResume::Next(7)
                }

                fn numeric_int_4_resume_7(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b4_i0, b4_i1, b4_i2, b4_i3, b4_v0,) = (values.ints[0], values.ints[1], values.ints[2], values.ints[3], values.bools[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b4_v0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                    }
                    *budget -= 1;
                    let b4_v1 = !b4_v0;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b4_v0, b4_v1]);
                    CompiledResume::Next(8)
                }

                fn numeric_int_4_resume_8(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b4_i0, b4_i1, b4_i2, b4_i3, b4_v0, b4_v1,) = (values.ints[0], values.ints[1], values.ints[2], values.ints[3], values.bools[0], values.bools[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0, b4_i1, b4_i2, b4_i3]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b4_v0, b4_v1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                    }
                    *budget -= 1;
                    {
                        let (b0_i0, b0_i1, b0_i2, b0_v0,) = (b4_i3, b4_i2, b4_i1, b4_v1,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b0_v0]);
                        CompiledResume::Next(0)
                    }
                }

                fn numeric_int_5(
                    point: usize,
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {

                    const RESUME: [
                        fn(&mut data::compiled::numeric::NumericValues, &mut usize) -> CompiledResume;
                        11
                    ] = [
                        |values, budget| CompiledResume::Exit(numeric_int_5_entry((values.ints[0],), values, budget)),
                        numeric_int_5_resume_1,
                        numeric_int_5_resume_2,
                        numeric_int_5_resume_3,
                        numeric_int_5_resume_4,
                        numeric_int_5_resume_5,
                        numeric_int_5_resume_6,
                        numeric_int_5_resume_7,
                        numeric_int_5_resume_8,
                        numeric_int_5_resume_9,
                        numeric_int_5_resume_10,
                    ];

                    let mut point = point;
                    loop {
                        match RESUME[point](values, budget) {
                            CompiledResume::Next(next) => point = next,
                            CompiledResume::Exit(progress) => return progress,
                        }
                    }
                }

                fn numeric_int_5_entry(
                    inputs: (i128,),
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {
                    let (b0_i0,) = inputs;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(0);
                    }
                    *budget -= 1;
                    let (b2_i0,) = if b0_i0 == 0_i128 {
                        let () = ();
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(1);
                        }
                        *budget -= 1;
                        let b1_i0 = -3_i128;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(2);
                        }
                        *budget -= 1;
                        let (b2_i0,) = {
                            (b1_i0,)
                        };
                        (b2_i0,)
                    } else if b0_i0 == 1_i128 {
                        let () = ();
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(8);
                        }
                        *budget -= 1;
                        let b5_i0 = 2_i128;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b5_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(9);
                        }
                        *budget -= 1;
                        let (b2_i0,) = {
                            (b5_i0,)
                        };
                        (b2_i0,)
                    } else {
                        let (b6_i0,) = (b0_i0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b6_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(10);
                        }
                        *budget -= 1;
                        let (b2_i0,) = {
                            (b6_i0,)
                        };
                        (b2_i0,)
                    };
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(3);
                    }
                    *budget -= 1;
                    if b2_i0 > 0_i128 {
                        let (b3_i0,) = (b2_i0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b3_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(4);
                        }
                        *budget -= 1;
                        let b3_i1 = b3_i0 + 42_i128;
                        if b3_i1 < i128::from(i64::MIN) || b3_i1 > i128::from(i64::MAX) {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Interpreted(5);
                        }
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(5);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                    } else {
                        let (b4_i0,) = (b2_i0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b4_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(6);
                        }
                        *budget -= 1;
                        let b4_i1 = -b4_i0;
                        if b4_i1 < i128::from(i64::MIN) || b4_i1 > i128::from(i64::MAX) {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Interpreted(7);
                        }
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(7);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                    }
                }

                fn numeric_int_5_resume_1(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                    }
                    *budget -= 1;
                    let b1_i0 = -3_i128;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(2)
                }

                fn numeric_int_5_resume_2(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_i0,) = (values.ints[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                    }
                    *budget -= 1;
                    {
                        let (b2_i0,) = (b1_i0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        CompiledResume::Next(3)
                    }
                }

                fn numeric_int_5_resume_3(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b2_i0,) = (values.ints[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                    }
                    *budget -= 1;
                    if b2_i0 > 0_i128 {
                        let (b3_i0,) = (b2_i0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        CompiledResume::Next(4)
                    } else {
                        let (b4_i0,) = (b2_i0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        CompiledResume::Next(6)
                    }
                }

                fn numeric_int_5_resume_4(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b3_i0,) = (values.ints[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                    }
                    *budget -= 1;
                    let b3_i1 = b3_i0 + 42_i128;
                    if b3_i1 < i128::from(i64::MIN) || b3_i1 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(5));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(5)
                }

                fn numeric_int_5_resume_5(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b3_i0, b3_i1,) = (values.ints[0], values.ints[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
                }

                fn numeric_int_5_resume_6(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b4_i0,) = (values.ints[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                    }
                    *budget -= 1;
                    let b4_i1 = -b4_i0;
                    if b4_i1 < i128::from(i64::MIN) || b4_i1 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(7));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(7)
                }

                fn numeric_int_5_resume_7(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b4_i0, b4_i1,) = (values.ints[0], values.ints[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(7));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0, b4_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
                }

                fn numeric_int_5_resume_8(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(8));
                    }
                    *budget -= 1;
                    let b5_i0 = 2_i128;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(9)
                }

                fn numeric_int_5_resume_9(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b5_i0,) = (values.ints[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b5_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(9));
                    }
                    *budget -= 1;
                    {
                        let (b2_i0,) = (b5_i0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        CompiledResume::Next(3)
                    }
                }

                fn numeric_int_5_resume_10(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b6_i0,) = (values.ints[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b6_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(10));
                    }
                    *budget -= 1;
                    {
                        let (b2_i0,) = (b6_i0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        CompiledResume::Next(3)
                    }
                }

                fn numeric_int_6(
                    point: usize,
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {

                    const RESUME: [
                        fn(&mut data::compiled::numeric::NumericValues, &mut usize) -> CompiledResume;
                        5
                    ] = [
                        |values, budget| CompiledResume::Exit(numeric_int_6_entry((values.ints[0], values.ints[1],), values, budget)),
                        numeric_int_6_resume_1,
                        numeric_int_6_resume_2,
                        numeric_int_6_resume_3,
                        numeric_int_6_resume_4,
                    ];

                    let mut point = point;
                    loop {
                        match RESUME[point](values, budget) {
                            CompiledResume::Next(next) => point = next,
                            CompiledResume::Exit(progress) => return progress,
                        }
                    }
                }

                fn numeric_int_6_entry(
                    inputs: (i128, i128,),
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {
                    let (b0_i0, b0_i1,) = inputs;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return data::compiled::CompiledProgress::Yield(0);
                    }
                    *budget -= 1;
                    if b0_i0 >= 0_i128 {
                        let (b1_i0, b1_i1,) = (b0_i0, b0_i1,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(1);
                        }
                        *budget -= 1;
                        let b1_i2 = b1_i0 + b1_i1;
                        if b1_i2 < i128::from(i64::MIN) || b1_i2 > i128::from(i64::MAX) {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Interpreted(2);
                        }
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(2);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                    } else {
                        let (b2_i0, b2_i1,) = (b0_i0, b0_i1,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(3);
                        }
                        *budget -= 1;
                        let b2_i2 = b2_i1 - b2_i0;
                        if b2_i2 < i128::from(i64::MIN) || b2_i2 > i128::from(i64::MAX) {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Interpreted(4);
                        }
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(4);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                    }
                }

                fn numeric_int_6_resume_1(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_i0, b1_i1,) = (values.ints[0], values.ints[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                    }
                    *budget -= 1;
                    let b1_i2 = b1_i0 + b1_i1;
                    if b1_i2 < i128::from(i64::MIN) || b1_i2 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(2));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(2)
                }

                fn numeric_int_6_resume_2(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_i0, b1_i1, b1_i2,) = (values.ints[0], values.ints[1], values.ints[2],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1, b1_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
                }

                fn numeric_int_6_resume_3(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b2_i0, b2_i1,) = (values.ints[0], values.ints[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                    }
                    *budget -= 1;
                    let b2_i2 = b2_i1 - b2_i0;
                    if b2_i2 < i128::from(i64::MIN) || b2_i2 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(4));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(4)
                }

                fn numeric_int_6_resume_4(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b2_i0, b2_i1, b2_i2,) = (values.ints[0], values.ints[1], values.ints[2],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1, b2_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
                }

                fn numeric_bool_0(
                    point: usize,
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {

                    const RESUME: [
                        fn(&mut data::compiled::numeric::NumericValues, &mut usize) -> CompiledResume;
                        7
                    ] = [
                        |values, budget| CompiledResume::Exit(numeric_bool_0_entry((values.ints[0], values.bools[0],), values, budget)),
                        numeric_bool_0_resume_1,
                        numeric_bool_0_resume_2,
                        numeric_bool_0_resume_3,
                        numeric_bool_0_resume_4,
                        numeric_bool_0_resume_5,
                        numeric_bool_0_resume_6,
                    ];

                    let mut point = point;
                    loop {
                        match RESUME[point](values, budget) {
                            CompiledResume::Next(next) => point = next,
                            CompiledResume::Exit(progress) => return progress,
                        }
                    }
                }

                fn numeric_bool_0_entry(
                    inputs: (i128, bool,),
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {
                    let (b0_i0, b0_v0,) = inputs;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b0_v0]);
                        return data::compiled::CompiledProgress::Yield(0);
                    }
                    *budget -= 1;
                    if b0_i0 < 0_i128 {
                        let (b1_v0,) = (b0_v0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b1_v0]);
                            return data::compiled::CompiledProgress::Yield(1);
                        }
                        *budget -= 1;
                        let b1_v1 = !b1_v0;
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b1_v0, b1_v1]);
                            return data::compiled::CompiledProgress::Yield(2);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0, b1_v1]);
                        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                    } else {
                        let (b2_i0, b2_v0,) = (b0_i0, b0_v0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b2_v0]);
                            return data::compiled::CompiledProgress::Yield(3);
                        }
                        *budget -= 1;
                        if b2_i0 >= 10_i128 {
                            let (b3_v0,) = (b2_v0,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[b3_v0]);
                                return data::compiled::CompiledProgress::Yield(4);
                            }
                            *budget -= 1;

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b3_v0]);
                            data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                        } else {
                            let () = ();
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                return data::compiled::CompiledProgress::Yield(5);
                            }
                            *budget -= 1;
                            let b4_v0 = false;
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[b4_v0]);
                                return data::compiled::CompiledProgress::Yield(6);
                            }
                            *budget -= 1;

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b4_v0]);
                            data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2))
                        }
                    }
                }

                fn numeric_bool_0_resume_1(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_v0,) = (values.bools[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                    }
                    *budget -= 1;
                    let b1_v1 = !b1_v0;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0, b1_v1]);
                    CompiledResume::Next(2)
                }

                fn numeric_bool_0_resume_2(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_v0, b1_v1,) = (values.bools[0], values.bools[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b1_v0, b1_v1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b1_v0, b1_v1]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
                }

                fn numeric_bool_0_resume_3(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b2_i0, b2_v0,) = (values.ints[0], values.bools[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b2_v0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                    }
                    *budget -= 1;
                    if b2_i0 >= 10_i128 {
                        let (b3_v0,) = (b2_v0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b3_v0]);
                        CompiledResume::Next(4)
                    } else {
                        let () = ();

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        CompiledResume::Next(5)
                    }
                }

                fn numeric_bool_0_resume_4(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b3_v0,) = (values.bools[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b3_v0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b3_v0]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
                }

                fn numeric_bool_0_resume_5(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                    }
                    *budget -= 1;
                    let b4_v0 = false;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b4_v0]);
                    CompiledResume::Next(6)
                }

                fn numeric_bool_0_resume_6(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b4_v0,) = (values.bools[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b4_v0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b4_v0]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2)))
                }
                data::compiled::CompiledFunctions {
                    ints: data::Storage::Static(&[
                        data::compiled::CompiledFunction {
                            function: data::function::IntFunctionId(0),
                            implementation: data::compiled::CompiledImplementation::Numeric(data::compiled::NumericImplementation {
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 0,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 1,
                                        ints: 3,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(3),
                                        instruction: 0,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(3),
                                        instruction: 1,
                                        ints: 4,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(4),
                                        instruction: 0,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(4),
                                        instruction: 1,
                                        ints: 4,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                ]),
                                run: numeric_int_0,
                            }),
                        },
                        data::compiled::CompiledFunction {
                            function: data::function::IntFunctionId(4),
                            implementation: data::compiled::CompiledImplementation::Numeric(data::compiled::NumericImplementation {
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 3,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 0,
                                        ints: 2,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 0,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 1,
                                        ints: 3,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(3),
                                        instruction: 0,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(3),
                                        instruction: 1,
                                        ints: 3,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(4),
                                        instruction: 0,
                                        ints: 3,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(4),
                                        instruction: 1,
                                        ints: 4,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(4),
                                        instruction: 2,
                                        ints: 4,
                                        bools: 2,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                ]),
                                run: numeric_int_4,
                            }),
                        },
                        data::compiled::CompiledFunction {
                            function: data::function::IntFunctionId(5),
                            implementation: data::compiled::CompiledImplementation::Numeric(data::compiled::NumericImplementation {
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 1,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(3),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(3),
                                        instruction: 1,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(4),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(4),
                                        instruction: 1,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(5),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(5),
                                        instruction: 1,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(6),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                ]),
                                run: numeric_int_5,
                            }),
                        },
                        data::compiled::CompiledFunction {
                            function: data::function::IntFunctionId(6),
                            implementation: data::compiled::CompiledImplementation::Numeric(data::compiled::NumericImplementation {
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 0,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 1,
                                        ints: 3,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 0,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 1,
                                        ints: 3,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                ]),
                                run: numeric_int_6,
                            }),
                        },
                    ]),
                    bools: data::Storage::Static(&[
                        data::compiled::CompiledFunction {
                            function: data::function::BoolFunctionId(0),
                            implementation: data::compiled::CompiledImplementation::Numeric(data::compiled::NumericImplementation {
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 1,
                                        ints: 0,
                                        bools: 2,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(3),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(4),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(4),
                                        instruction: 1,
                                        ints: 0,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                    },
                                ]),
                                run: numeric_bool_0,
                            }),
                        },
                    ]),
                    customs: data::Storage::Static(&[
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
                    0..7,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    7..8,
                    0..0,
                    8..9,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
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
                        parameters: 2..2,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 2..2,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 2..4,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 4..8,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(1),
                            data::type_::ValueShapeId(0),
                        ]),
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
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                shape: data::type_::ValueShapeId(0),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 10..12,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 12..15,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(2),
                        ]),
                        return_: data::type_::ValueShapeId(4),
                        captures: data::Storage::Static(&[]),
                    },
                ]),
                parameters: data::Storage::Static(&[
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                    data::type_::ValueShapeDescriptor::Bool,
                    data::type_::ValueShapeDescriptor::String,
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(0)),
                    data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                        data::type_::ValueShapeId(2),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(3),
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
                    data::type_::ValueType::Bool,
                    data::type_::ValueType::String,
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
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
                name: data::Text::Static("arithmetic"),
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
                name: data::Text::Static("choice"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::Bool,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("caller"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
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
                name: data::Text::Static("main"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 1,
            },
            data::Export {
                name: data::Text::Static("running"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 2,
            },
        ]),
    },
    value_functions: data::Storage::Static(&[]),
    never_functions: data::Storage::Static(&[]),
    callables: data::Storage::Static(&[]),
}
