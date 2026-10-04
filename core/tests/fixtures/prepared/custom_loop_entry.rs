data::HostedEntryArtifact {
    format: 18,
    program: data::ProgramTables {
        root: data::source::module_id(0),
        modules: data::Storage::Static(&[
            data::program::ExecutionModuleContext {
                module: data::Text::Static("example"),
                source_context: Some(data::source::SourceContext::from_static_block("src/custom_loop.gleam", r#"
pub type Item {
  Add(Int)
  Subtract(Int)
  Skip
}

fn fold(items: List(Item), total: Int, apply: fn(Int, Item) -> Int) -> Int {
  case items {
    [] -> total
    [head, ..tail] -> fold(tail, apply(total, head), apply)
  }
}

fn adjust(total: Int, item: Item) -> Int {
  case item {
    Add(value) -> total + value
    Subtract(value) -> total - value
    Skip -> total
  }
}

fn double(total: Int, item: Item) -> Int {
  case item {
    Add(value) -> total + value * 2
    Subtract(value) -> total - value * 2
    Skip -> total
  }
}

pub fn run(seed: Int, value: Int) -> Int {
  fold([Add(value), Subtract(value), Add(7), Skip], seed, adjust)
}

pub fn chosen(seed: Int, doubled: Bool) -> Int {
  let apply = case doubled {
    True -> double
    False -> adjust
  }
  fold([Add(7), Subtract(2), Skip], seed, apply)
}

pub fn captured(seed: Int, bias: Int, enabled: Bool) -> Int {
  fold([Add(7), Subtract(2), Skip], seed, fn(total, item) {
    case enabled {
      True ->
        case item {
          Add(value) -> total + value + bias
          Subtract(value) -> total - value - bias
          Skip -> total + bias
        }
      False -> total
    }
  })
}

fn any(items: List(Item), seen: Bool, apply: fn(Bool, Item) -> Bool) -> Bool {
  case items {
    [] -> seen
    [head, ..tail] -> any(tail, apply(seen, head), apply)
  }
}

pub fn boolean(initial: Bool, value: Int) -> Bool {
  any([Skip, Add(value), Subtract(0)], initial, fn(seen, item) {
    case item {
      Add(n) ->
        case n > 0 {
          True -> True
          False -> seen
        }
      _ -> seen
    }
  })
}

fn asserted(total: Int, item: Item) -> Int {
  let assert Add(value) = item
  total + value
}

pub fn assertion(value: Int) -> Int {
  fold([Add(1), Subtract(value)], 0, asserted)
}

fn stop(total: Int, item: Item) -> Int {
  case item {
    Add(_) -> panic as "loop callback stop"
    _ -> total
  }
}

pub fn empty() -> Int {
  fold([], 19, stop)
}

pub fn panic_case(value: Int) -> Int {
  fold([Skip, Add(value)], 0, stop)
}

fn extra(value: Int) -> Int {
  value + 1
}

fn non_leaf(total: Int, item: Item) -> Int {
  case item {
    Add(value) -> total + extra(value)
    _ -> total
  }
}

pub fn unsupported(value: Int) -> Int {
  fold([Add(value), Skip], 0, non_leaf)
}

pub fn custom_capture(seed: Int, bias: Int) -> Int {
  let modifier = Add(bias)
  fold([Add(7), Subtract(2), Skip], seed, fn(total, item) {
    case modifier {
      Add(bias) ->
        case item {
          Add(value) -> total + value + bias
          Subtract(value) -> total - value
          Skip -> total
        }
      _ -> total
    }
  })
}

fn repeat_items(count: Int, value: Int, items: List(Item)) -> List(Item) {
  case count > 0 {
    True -> repeat_items(count - 1, value, [Add(value), ..items])
    False -> items
  }
}

pub fn repeated(count: Int, value: Int, seed: Int) -> Int {
  fold(repeat_items(count, value, []), seed, adjust)
}

fn fold_adjusted(
  items: List(Item),
  total: Int,
  apply: fn(Int, Item) -> Int,
) -> Int {
  case items {
    [] -> total
    [head, ..tail] -> fold_adjusted(tail, apply(total + 1, head), apply)
  }
}

pub fn caller_overflow(seed: Int) -> Int {
  fold_adjusted([Subtract(2), Add(3)], seed, adjust)
}

pub type Decision {
  Decision(Int, Bool)
}

fn fold_decisions(
  items: List(Decision),
  total: Int,
  apply: fn(Int, Decision) -> Int,
) -> Int {
  case items {
    [] -> total
    [head, ..tail] -> fold_decisions(tail, apply(total, head), apply)
  }
}

fn gated(total: Int, item: Decision) -> Int {
  case item {
    Decision(amount, True) as original if amount > 0 -> {
      let Decision(other, _) = original
      total + amount + other
    }
    _ -> total
  }
}

pub fn guarded(seed: Int, value: Int, enabled: Bool) -> Int {
  fold_decisions([Decision(value, enabled), Decision(3, True)], seed, gated)
}

pub fn main() -> Int {
  run(2, 9) + chosen(1, True) + captured(0, 3, True) + empty() + unsupported(2)
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
                                            function: data::function::IntFunctionId(1),
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
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(2),
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
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(8)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                            function: data::function::IntFunctionId(3),
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
                                            function: data::function::IntFunctionId(5),
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
                                            shape: data::type_::ValueShapeId(2),
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
                                            shape: data::type_::ValueShapeId(3),
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
                                            shape: data::type_::ValueShapeId(2),
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
                                            shape: data::type_::ValueShapeId(4),
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
                                            shape: data::type_::ValueShapeId(6),
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
                                            shape: data::type_::ValueShapeId(7),
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
                                            kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(6))),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(7),
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
                                        shape: data::type_::ValueShapeId(1),
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
                                        shape: data::type_::ValueShapeId(7),
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
                                            shape: data::type_::ValueShapeId(7),
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
                                            kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(8))),
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
                                            shape: data::type_::ValueShapeId(2),
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
                                            shape: data::type_::ValueShapeId(3),
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
                                            shape: data::type_::ValueShapeId(4),
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
                                            shape: data::type_::ValueShapeId(6),
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
                                            shape: data::type_::ValueShapeId(7),
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
                                            kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(6))),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(7),
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
                                        shape: data::type_::ValueShapeId(1),
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
                                            shape: data::type_::ValueShapeId(2),
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
                                            shape: data::type_::ValueShapeId(3),
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
                                            shape: data::type_::ValueShapeId(4),
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
                                            shape: data::type_::ValueShapeId(6),
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
                                            shape: data::type_::ValueShapeId(7),
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
                                                target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(9)),
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
                                        function: data::function::IntFunctionId(7),
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
                                            shape: data::type_::ValueShapeId(6),
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
                                            shape: data::type_::ValueShapeId(7),
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
                                            kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(10))),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(7),
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
                                            shape: data::type_::ValueShapeId(2),
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
                                            shape: data::type_::ValueShapeId(4),
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
                                            shape: data::type_::ValueShapeId(6),
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
                                            shape: data::type_::ValueShapeId(7),
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
                                            kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(11))),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(7),
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
                                        shape: data::type_::ValueShapeId(6),
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
                                        shape: data::type_::ValueShapeId(7),
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
                                        shape: data::type_::ValueShapeId(6),
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
                                        shape: data::type_::ValueShapeId(7),
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
                                            shape: data::type_::ValueShapeId(5),
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
                                            shape: data::type_::ValueShapeId(6),
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
                                        shape: data::type_::ValueShapeId(5),
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
                                        local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(0),
                                                shape_id: data::type_::CustomValueShapeId(3),
                                            },
                                        }),
                                        shape: data::type_::ValueShapeId(5),
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
                                        shape: data::type_::ValueShapeId(5),
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
                                        shape: data::type_::ValueShapeId(5),
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
                                            shape: data::type_::ValueShapeId(8),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("loop callback stop"))),
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
                                            function: data::function::IntFunctionId(12),
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

            enum CustomLoopResume {
                Next(usize),
                Exit(data::compiled::custom_loop::CustomLoopProgress),
            }

            fn callback_int_6_call(
                inputs: &data::compiled::custom_loop::CallbackInputs<'_>,
                values: &mut data::compiled::custom::CustomValues,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CallbackProgress<i128> {
                let Some(_integer0) = inputs.integer(0) else {
                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Entry);
                };
                callback_int_6_entry((_integer0, inputs.custom(0),), values, budget)
            }

            fn callback_int_6_entry(
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

            fn callback_int_8_call(
                inputs: &data::compiled::custom_loop::CallbackInputs<'_>,
                values: &mut data::compiled::custom::CustomValues,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CallbackProgress<i128> {
                let Some(_integer0) = inputs.integer(0) else {
                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Entry);
                };
                callback_int_8_entry((_integer0, inputs.custom(0),), values, budget)
            }

            fn callback_int_8_entry(
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

            fn callback_int_9_call(
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
                callback_int_9_entry((_integer0, _integer1, inputs.boolean(0), inputs.custom(0),), values, budget)
            }

            fn callback_int_9_entry(
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

            fn callback_int_10_call(
                inputs: &data::compiled::custom_loop::CallbackInputs<'_>,
                values: &mut data::compiled::custom::CustomValues,
                budget: &mut usize,
            ) -> data::compiled::custom_loop::CallbackProgress<i128> {
                let Some(_integer0) = inputs.integer(0) else {
                    return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Entry);
                };
                callback_int_10_entry((_integer0, inputs.custom(0),), values, budget)
            }

            fn callback_int_10_entry(
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

            fn custom_loop_int_7(
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
                        CustomLoopResume::Exit(custom_loop_int_7_entry((values.ints[0], _custom_list0, _int_function0,), values, _lists, budget))
                    },
                    custom_loop_int_7_resume_1,
                    custom_loop_int_7_resume_2,
                    custom_loop_int_7_resume_3,
                    custom_loop_int_7_resume_4,
                    custom_loop_int_7_resume_5,
                ];

                let mut point = point;
                loop {
                    match RESUME[point](values, _lists, budget) {
                        CustomLoopResume::Next(next) => point = next,
                        CustomLoopResume::Exit(progress) => return progress,
                    }
                }
            }

            fn custom_loop_int_7_entry(
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

            fn custom_loop_int_7_resume_1(
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

            fn custom_loop_int_7_resume_2(
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

            fn custom_loop_int_7_resume_3(
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

            fn custom_loop_int_7_resume_4(
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

            fn custom_loop_int_7_resume_5(
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
            data::compiled::CompiledFunctions {
                ints: data::Storage::Static(&[
                    data::compiled::CompiledFunction {
                        function: data::function::IntFunctionId(7),
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
                            run: custom_loop_int_7,
                        })),
                    },
                ]),
                bools: data::Storage::Static(&[
                ]),
                customs: data::Storage::Static(&[
                ]),
                int_lists: data::Storage::Static(&[
                ]),
                callbacks: data::compiled::CompiledCallbacks {
                    ints: data::Storage::Static(&[
                        data::compiled::CompiledCallback {
                            function: data::function::IntFunctionId(6),
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
                            run: callback_int_6_call,
                        },
                        data::compiled::CompiledCallback {
                            function: data::function::IntFunctionId(8),
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
                            run: callback_int_8_call,
                        },
                        data::compiled::CompiledCallback {
                            function: data::function::IntFunctionId(9),
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
                            run: callback_int_9_call,
                        },
                        data::compiled::CompiledCallback {
                            function: data::function::IntFunctionId(10),
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
                            run: callback_int_10_call,
                        },
                    ]),
                    bools: data::Storage::Static(&[
                    ]),
                },
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
                0..13,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
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
                        data::type_::ValueShapeId(1),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 4..7,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(1),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 7..7,
                    parameter_shapes: data::Storage::Static(&[]),
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
                    parameters: 8..10,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(5),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 10..13,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(6),
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(7),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 13..15,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(5),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 15..17,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(5),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[
                        data::graph::ParamSlot {
                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                            shape: data::type_::ValueShapeId(1),
                        },
                        data::graph::ParamSlot {
                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                            shape: data::type_::ValueShapeId(0),
                        },
                    ]),
                },
                data::function::FunctionContract {
                    parameters: 17..19,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(5),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 19..21,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(5),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 21..22,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
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
            ]),
        },
        list_types: data::type_::ListTypeTable {
            types: data::Storage::Static(&[
                data::type_::ListStorageTypeId::Custom(data::type_::CustomListTypeId {
                    list_type: data::type_::ListTypeId(0),
                    item_type: data::type_::CustomTypeId(0),
                }),
            ]),
            tuple_items: data::Storage::Static(&[]),
            function_items: data::Storage::Static(&[]),
        },
        custom_types: data::type_::CustomTypeTable {
            types: data::Storage::Static(&[
                data::type_::CustomTypeDescriptor {
                    type_: data::type_::NominalTypeMetadata {
                        package: data::Text::Static("example"),
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
            ]),
            definitions: data::Storage::Static(&[
                data::type_::CustomDefinition {
                    package: data::Text::Static("example"),
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
                    package: data::Text::Static("example"),
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
                data::type_::ValueShapeDescriptor::Bool,
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(0)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(1)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(2)),
                data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(3)),
                data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(5)),
                data::type_::ValueShapeDescriptor::Function {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(5),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                },
                data::type_::ValueShapeDescriptor::String,
            ]),
            shape_types: data::Storage::Static(&[
                data::type_::ValueType::Int,
                data::type_::ValueType::Bool,
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
                data::type_::ValueType::String,
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
            ]),
        },
    },
    value_functions: data::Storage::Static(&[]),
    never_functions: data::Storage::Static(&[]),
}
