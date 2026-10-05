data::HostedModuleArtifact {
    module: data::ModuleArtifact {
        format: 22,
        program: data::ProgramTables {
            root: data::source::module_id(0),
            modules: data::Storage::Static(&[
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("example"),
                    source_context: Some(data::source::SourceContext::from_static_block("src/int_list_calls.gleam", r#"
fn fold(values: List(Int), total: Int, calculate: fn(Int, Int) -> Int) -> Int {
  case values {
    [] -> total
    [first, ..rest] -> fold(rest, calculate(total, first), calculate)
  }
}

pub fn capturing_fold(values: List(Int), factor: Int, bias: Int) -> Int {
  let transform = fn(value) { value * factor + bias }
  fold(values, 0, fn(total, value) { total + transform(value) })
}

fn sum(values: List(Int), initial: Int) -> Int {
  fold(values, initial, fn(total, value) { total + value })
}

pub fn verify(values: List(Int), expected: Int) -> Bool {
  sum(values, 0) == expected
}

pub fn make_sum(prefix: List(Int), offset: Int) -> fn(List(Int)) -> Int {
  fn(values) { sum([offset, ..prefix], sum(values, 0)) }
}

pub fn make_check(prefix: List(Int)) -> fn(List(Int)) -> Bool {
  fn(values) {
    let same = prefix == values
    let different = prefix != values
    let total = sum(values, 0)
    case same {
      True -> total > 0
      False -> !different
    }
  }
}

pub fn selected(values: List(Int), other: List(Int), choose: Bool) -> Int {
  let calculate = make_sum(values, 7)
  let selected = case choose {
    True -> [1, 2, ..other]
    False -> []
  }
  calculate(selected)
}

fn echo_value(value: Int) -> Int {
  echo value
  value + 1
}

pub fn canonical(values: List(Int)) -> Int {
  fold(values, 0, fn(total, value) { total + echo_value(value) })
}

fn stopped(value: Int) -> Int {
  echo value
  panic as "list callback stopped"
}

pub fn failure(values: List(Int)) -> Int {
  fold(values, 0, fn(total, value) { total + stopped(value) })
}

fn identity_list(values: List(Int)) -> List(Int) {
  values
}

pub fn list_return(values: List(Int)) -> Int {
  sum(identity_list(values), 5)
}

pub fn non_tail(values: List(Int)) -> Int {
  case values {
    [] -> 0
    [first, ..rest] -> non_tail(rest) + first
  }
}

pub fn main() -> Int {
  capturing_fold([1, 2, 3], 2, 1)
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
                                                shape: data::type_::ValueShapeId(2),
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
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(7)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::Int {
                                                            target: data::graph::IntLocalId(1),
                                                            source: data::graph::IntLocalId(0),
                                                        },
                                                        data::graph::FunctionCapture::Int {
                                                            target: data::graph::IntLocalId(2),
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
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::NoSign,
                                                digits: data::Storage::Static(&[]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::IntFunction {
                                                    local: data::graph::IntFunctionLocalId(1),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                                family: data::function::FunctionReturnFamily::Int,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(8)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::IntFunction {
                                                            target: data::graph::IntFunctionLocalId(0),
                                                            source: data::graph::IntFunctionLocalId(0),
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
                                            function: data::function::IntFunctionId(9),
                                            site: data::source::HostCallSite::from_static("example", "capturing_fold", data::source::SourceSpan::new(319, 381)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(1),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                        data::type_::ValueType::Int,
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
                                                    family: data::graph::StorageFamily::IntFunction,
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
                                parameter_count: 3,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..3,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                                subject: data::graph::BoolLocalId(0),
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                            local: data::graph::IntListLocalId(1),
                                                            type_id: data::type_::IntListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                            },
                                                        }),
                                                        data::graph::ParamLocal::IntFunction {
                                                            local: data::graph::IntFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
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
                                                                family: data::graph::StorageFamily::Bool,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
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
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(3),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::IntFunction {
                                                            local: data::graph::IntFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
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
                                                                family: data::graph::StorageFamily::Bool,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntList,
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
                                            instructions: 2..5,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                            local: data::graph::IntListLocalId(1),
                                                            type_id: data::type_::IntListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                            },
                                                        }),
                                                        data::graph::ParamLocal::IntFunction {
                                                            local: data::graph::IntFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
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
                                            params: 5..7,
                                            instructions: 5..6,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 7..8,
                                            instructions: 6..7,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                            local: data::graph::IntListLocalId(0),
                                                            type_id: data::type_::IntListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                            },
                                                        }),
                                                        data::graph::ParamLocal::IntFunction {
                                                            local: data::graph::IntFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
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
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(4),
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
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(5),
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
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(5),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(5),
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
                                                    7,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::IntFunction {
                                                    local: data::graph::IntFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(5),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                                family: data::function::FunctionReturnFamily::Int,
                                                kind: data::graph::FunctionInstructionKind::Call {
                                                    function: data::function::ProfiledFunctionFunctionId::Int(data::function::IntFunctionFunctionId(0)),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                            local: data::graph::IntListLocalId(0),
                                                            type_id: data::type_::IntListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                            },
                                                        }),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    site: data::source::HostCallSite::from_static("example", "selected", data::source::SourceSpan::new(1073, 1092)),
                                                },
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
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    2,
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
                                            }, data::graph::TypedListInstruction::Spread {
                                                elements: data::Storage::Static(&[
                                                    data::graph::IntLocalId(0),
                                                    data::graph::IntLocalId(1),
                                                ]),
                                                tail: data::graph::IntListLocalId(0),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::FunctionCall {
                                                function: data::graph::IntFunctionLocalId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(0),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "selected", data::source::SourceSpan::new(1174, 1193)),
                                            }),
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
                                            }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))),
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
                                            params: 0..1,
                                            instructions: 0..2,
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
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                                family: data::function::FunctionReturnFamily::Int,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(12)),
                                                    captures: data::Storage::Static(&[]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::IntFunctionId(9),
                                            site: data::source::HostCallSite::from_static("example", "canonical", data::source::SourceSpan::new(1307, 1370)),
                                        },
                                        args: data::Storage::Static(&[
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
                                                        data::type_::ValueType::Int,
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
                                            instructions: 0..2,
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
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                                family: data::function::FunctionReturnFamily::Int,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(13)),
                                                    captures: data::Storage::Static(&[]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::IntFunctionId(9),
                                            site: data::source::HostCallSite::from_static("example", "failure", data::source::SourceSpan::new(1502, 1562)),
                                        },
                                        args: data::Storage::Static(&[
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
                                                        data::type_::ValueType::Int,
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
                                            instructions: 0..2,
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
                                            }, data::graph::TypedListInstruction::Call {
                                                function: data::function::IntListFunctionId {
                                                    index: 0,
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(0),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "list_return", data::source::SourceSpan::new(1682, 1703)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    5,
                                                ]),
                                            })),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::IntFunctionId(10),
                                            site: data::source::HostCallSite::from_static("example", "list_return", data::source::SourceSpan::new(1678, 1707)),
                                        },
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
                                                    args: data::Storage::Static(&[]),
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
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 1..2,
                                            instructions: 1..5,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
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
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::NoSign,
                                                digits: data::Storage::Static(&[]),
                                            })),
                                        }),
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
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(5),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(1),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "non_tail", data::source::SourceSpan::new(1806, 1820)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(2)),
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
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    2,
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
                                                    3,
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
                                                    2,
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
                                                    1,
                                                ]),
                                            })),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::IntFunctionId(0),
                                            site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(1861, 1892)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::Int,
                                                    length: 2,
                                                    steps: data::Storage::Static(&[
                                                        data::graph::TransferStep {
                                                            source: 3,
                                                            destination: 0,
                                                        },
                                                        data::graph::TransferStep {
                                                            source: 4,
                                                            destination: 1,
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
                                parameter_count: 1,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..3,
                                            instructions: 0..1,
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
                                        data::graph::ProfiledInstruction::IntegerRegion(data::graph::ArithmeticRegion {
                                            inputs: data::Storage::Static(&[
                                                data::graph::IntLocalId(0),
                                                data::graph::IntLocalId(1),
                                                data::graph::IntLocalId(2),
                                            ]),
                                            nodes: data::Storage::Static(&[
                                                data::graph::ArithmeticNode::Multiply(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Input(1)),
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
                                            params: 0..3,
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
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
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
                                                site: data::source::HostCallSite::from_static("example", "<anonymous:1>", data::source::SourceSpan::new(362, 378)),
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
                                                                    data::type_::ValueType::Int,
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
                                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                            local: data::graph::IntListLocalId(1),
                                                            type_id: data::type_::IntListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                            },
                                                        }),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                        data::graph::ParamLocal::IntFunction {
                                                            local: data::graph::IntFunctionLocalId(0),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                    data::type_::ValueType::Int,
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
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(3),
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
                                            local: data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                            shape: data::type_::ValueShapeId(3),
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
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::FunctionCall {
                                                function: data::graph::IntFunctionLocalId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "fold", data::source::SourceSpan::new(146, 169)),
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
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                                family: data::function::FunctionReturnFamily::Int,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(14)),
                                                    captures: data::Storage::Static(&[]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::IntFunctionId(9),
                                            site: data::source::HostCallSite::from_static("example", "sum", data::source::SourceSpan::new(436, 493)),
                                        },
                                        args: data::Storage::Static(&[
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
                                                        data::type_::ValueType::Int,
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
                                                tail: data::graph::IntListLocalId(1),
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
                                                function: data::function::IntFunctionId(10),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(0),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "<anonymous:3>", data::source::SourceSpan::new(700, 714)),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::IntFunctionId(10),
                                            site: data::source::HostCallSite::from_static("example", "<anonymous:3>", data::source::SourceSpan::new(676, 715)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(2),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
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
                                                    family: data::graph::StorageFamily::IntList,
                                                    length: 1,
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
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(15),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "<anonymous:5>", data::source::SourceSpan::new(1350, 1367)),
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
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(16),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "<anonymous:6>", data::source::SourceSpan::new(1545, 1559)),
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
                                            instructions: 0..1,
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
                                            terminator: data::graph::Terminator::Echo(data::graph::Echo {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                message: None,
                                                site: data::source::EchoSite::from_static("example", "echo_value", data::source::SourceSpan::new(1234, 1244)),
                                                next: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
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
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
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
                                            terminator: data::graph::Terminator::Echo(data::graph::Echo {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                message: None,
                                                site: data::source::EchoSite::from_static("example", "stopped", data::source::SourceSpan::new(1408, 1418)),
                                                next: data::graph::Edge {
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
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 1..1,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::SourceStop(data::graph::SourceStop {
                                                kind: data::graph::SourceStopKind::Panic,
                                                message: Some(data::graph::StringLocalId(0)),
                                                site: data::source::PanicSite::from_static("example", "stopped", data::source::SourceSpan::new(1421, 1453)),
                                            }),
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
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                shape: data::type_::ValueShapeId(7),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("list callback stopped"))),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[]),
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
                                                function: data::function::IntFunctionId(10),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(0),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "verify", data::source::SourceSpan::new(557, 571)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(4),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::EqualInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            })),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
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
                                            params: 0..2,
                                            instructions: 0..4,
                                            terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                                subject: data::graph::BoolLocalId(0),
                                                true_: data::graph::Edge {
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
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
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
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
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
                                                                family: data::graph::StorageFamily::IntList,
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
                                            instructions: 4..5,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 3..4,
                                            instructions: 5..6,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
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
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(4),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Equal {
                                                left: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(1),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                                right: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                shape: data::type_::ValueShapeId(4),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::NotEqual {
                                                left: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(1),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                                right: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::NoSign,
                                                digits: data::Storage::Static(&[]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(10),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                        local: data::graph::IntListLocalId(0),
                                                        type_id: data::type_::IntListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                        },
                                                    }),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "<anonymous:4>", data::source::SourceSpan::new(885, 899)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(4),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::GtInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(0),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                shape: data::type_::ValueShapeId(4),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Not(data::graph::BoolLocalId(0)))),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
                                    data::function::FunctionExit::Return(data::graph::BoolLocalId(1)),
                                ]),
                            },
                        })),
                    ]),
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
                                parameter_count: 1,
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
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
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
                    int_function_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 2,
                            },
                            body: data::function::TypedFunctionBody {
                                _shape: data::type_::FunctionShape {
                                    shape_id: data::type_::ValueShapeId(5),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    },
                                },
                                body: data::function::ProfiledFunctionBody {
                                    block_graph: data::graph::ProfiledBlockGraph {
                                        entry: data::graph::BlockId(0),
                                        blocks: data::Storage::Static(&[
                                            data::graph::BlockHeader {
                                                params: 0..2,
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
                                                                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        },
                                                    },
                                                    shape: data::type_::ValueShapeId(5),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                    family: data::function::FunctionReturnFamily::Int,
                                                    kind: data::graph::FunctionInstructionKind::Closure {
                                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(11)),
                                                        captures: data::Storage::Static(&[
                                                            data::graph::FunctionCapture::Int {
                                                                target: data::graph::IntLocalId(0),
                                                                source: data::graph::IntLocalId(0),
                                                            },
                                                            data::graph::FunctionCapture::IntList {
                                                                target: data::graph::IntListLocalId(1),
                                                                source: data::graph::IntListLocalId(0),
                                                            },
                                                        ]),
                                                    },
                                                }),
                                            }),
                                        ]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::Return(data::graph::IntFunctionLocalId(0)),
                                    ]),
                                },
                            },
                        })),
                    ]),
                    float_function_functions: data::Storage::Static(&[]),
                    string_function_functions: data::Storage::Static(&[]),
                    bit_array_function_functions: data::Storage::Static(&[]),
                    utf_codepoint_function_functions: data::Storage::Static(&[]),
                    custom_function_functions: data::Storage::Static(&[]),
                    external_function_functions: data::Storage::Static(&[]),
                    bool_function_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 1,
                            },
                            body: data::function::TypedFunctionBody {
                                _shape: data::type_::FunctionShape {
                                    shape_id: data::type_::ValueShapeId(6),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                    },
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
                                                    local: data::graph::ParamLocal::BoolFunction {
                                                        local: data::graph::BoolFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                        },
                                                    },
                                                    shape: data::type_::ValueShapeId(6),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                                    },
                                                    family: data::function::FunctionReturnFamily::Bool,
                                                    kind: data::graph::FunctionInstructionKind::Closure {
                                                        target: data::graph::FunctionTarget::Bool(data::function::BoolFunctionId(1)),
                                                        captures: data::Storage::Static(&[
                                                            data::graph::FunctionCapture::IntList {
                                                                target: data::graph::IntListLocalId(1),
                                                                source: data::graph::IntListLocalId(0),
                                                            },
                                                        ]),
                                                    },
                                                }),
                                            }),
                                        ]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::Return(data::graph::BoolFunctionLocalId(0)),
                                    ]),
                                },
                            },
                        })),
                    ]),
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
                use data::compiled::calls::{BoolCallable, CallArguments, CallCapture, CallCaptureInputs, CallExecution, CallInputs, CallInteger, CallOps, CallOutput, CallProgress, CallStorage, CallValues, IntCallable};
                use data::compiled::int_list::IntList;
                enum FunctionState {
                    Int0Point0 { int_list0: IntList, int0: i128, int1: i128 },
                    Int0Point1 { int_list0: IntList, int0: i128, int1: i128, int_function0: IntCallable },
                    Int0Point2 { int_list0: IntList, int0: i128, int1: i128, int_function0: IntCallable, int2: i128 },
                    Int0Point3 { int_list0: IntList, int0: i128, int1: i128, int_function0: IntCallable, int2: i128, int_function1: IntCallable },
                    Int1Point0 { int_list0: IntList, int_list1: IntList, bool0: bool },
                    Int1Point1 { int_list0: IntList, int_list1: IntList, bool0: bool, int0: i128 },
                    Int1Point2 { int_list0: IntList, int_list1: IntList, bool0: bool, int0: i128, int_function0: IntCallable },
                    Int1Point3 { int_list0: IntList, int_function0: IntCallable },
                    Int1Point4 { int_list0: IntList, int_function0: IntCallable, int0: i128 },
                    Int1Point5 { int_list0: IntList, int_function0: IntCallable, int0: i128, int1: i128 },
                    Int1Point6 { int_list0: IntList, int_function0: IntCallable, int0: i128, int1: i128, int_list1: IntList },
                    Int1Point7 { int_list0: IntList, int_function0: IntCallable },
                    Int1Point8 { int_list0: IntList, int_function0: IntCallable, int0: i128 },
                    Int1Point9 { int_function0: IntCallable },
                    Int1Point10 { int_function0: IntCallable, int_list0: IntList },
                    Int2Point0 { int_list0: IntList },
                    Int2Point1 { int_list0: IntList, int0: i128 },
                    Int2Point2 { int_list0: IntList, int0: i128, int_function0: IntCallable },
                    Int3Point0 { int_list0: IntList },
                    Int3Point1 { int_list0: IntList, int0: i128 },
                    Int3Point2 { int_list0: IntList, int0: i128, int_function0: IntCallable },
                    Int5Point0 { int_list0: IntList },
                    Int5Point1 {  },
                    Int5Point2 { int0: i128 },
                    Int5Point3 { int_list0: IntList },
                    Int5Point4 { int_list0: IntList, int0: i128 },
                    Int5Point5 { int_list0: IntList, int0: i128, int_list1: IntList },
                    Int5Point6 { int_list0: IntList, int0: i128, int_list1: IntList, int1: i128 },
                    Int5Point7 { int_list0: IntList, int0: i128, int_list1: IntList, int1: i128, int2: i128 },
                    Int7Point0 { int0: i128, int1: i128, int2: i128 },
                    Int7Point1 { int0: i128, int1: i128, int2: i128, int3: i128 },
                    Int8Point0 { int0: i128, int1: i128, int_function0: IntCallable },
                    Int8Point1 { int0: i128, int1: i128, int_function0: IntCallable, int2: i128 },
                    Int8Point2 { int0: i128, int1: i128, int_function0: IntCallable, int2: i128, int3: i128 },
                    Int9Point0 { int_list0: IntList, int0: i128, int_function0: IntCallable },
                    Int9Point1 { int0: i128 },
                    Int9Point2 { int_list0: IntList, int0: i128, int_function0: IntCallable },
                    Int9Point3 { int_list0: IntList, int0: i128, int_function0: IntCallable, int1: i128 },
                    Int9Point4 { int_list0: IntList, int0: i128, int_function0: IntCallable, int1: i128, int_list1: IntList },
                    Int9Point5 { int_list0: IntList, int0: i128, int_function0: IntCallable, int1: i128, int_list1: IntList, int2: i128 },
                    Int10Point0 { int_list0: IntList, int0: i128 },
                    Int10Point1 { int_list0: IntList, int0: i128, int_function0: IntCallable },
                    Int11Point0 { int_list0: IntList, int0: i128, int_list1: IntList },
                    Int11Point1 { int_list0: IntList, int0: i128, int_list1: IntList, int_list2: IntList },
                    Int11Point2 { int_list0: IntList, int0: i128, int_list1: IntList, int_list2: IntList, int1: i128 },
                    Int11Point3 { int_list0: IntList, int0: i128, int_list1: IntList, int_list2: IntList, int1: i128, int2: i128 },
                    Int12Point0 { int0: i128, int1: i128 },
                    Int12Point1 { int0: i128, int1: i128, int2: i128 },
                    Int12Point2 { int0: i128, int1: i128, int2: i128, int3: i128 },
                    Int13Point0 { int0: i128, int1: i128 },
                    Int13Point1 { int0: i128, int1: i128, int2: i128 },
                    Int13Point2 { int0: i128, int1: i128, int2: i128, int3: i128 },
                    Int14Point0 { int0: i128, int1: i128 },
                    Int14Point1 { int0: i128, int1: i128, int2: i128 },
                    Bool0Point0 { int_list0: IntList, int0: i128 },
                    Bool0Point1 { int_list0: IntList, int0: i128, int1: i128 },
                    Bool0Point2 { int_list0: IntList, int0: i128, int1: i128, int2: i128 },
                    Bool0Point3 { int_list0: IntList, int0: i128, int1: i128, int2: i128, bool0: bool },
                    Bool1Point0 { int_list0: IntList, int_list1: IntList },
                    Bool1Point1 { int_list0: IntList, int_list1: IntList, bool0: bool },
                    Bool1Point2 { int_list0: IntList, int_list1: IntList, bool0: bool, bool1: bool },
                    Bool1Point3 { int_list0: IntList, int_list1: IntList, bool0: bool, bool1: bool, int0: i128 },
                    Bool1Point4 { int_list0: IntList, int_list1: IntList, bool0: bool, bool1: bool, int0: i128, int1: i128 },
                    Bool1Point5 { int0: i128 },
                    Bool1Point6 { int0: i128, bool0: bool },
                    Bool1Point7 { bool0: bool },
                    Bool1Point8 { bool0: bool, bool1: bool },
                    IntFunction0Point0 { int_list0: IntList, int0: i128 },
                    IntFunction0Point1 { int_list0: IntList, int0: i128, int_function0: IntCallable },
                    BoolFunction0Point0 { int_list0: IntList },
                    BoolFunction0Point1 { int_list0: IntList, bool_function0: BoolCallable },
                    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: CallValues },
                }
                impl FunctionState {
                    fn values(self) -> CallValues {
                        match self {
                            Self::Int0Point0 { int_list0, int0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int0Point1 { int_list0, int0, int1, int_function0 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int0Point2 { int_list0, int0, int1, int_function0, int2 } => {
                                CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int0Point3 { int_list0, int0, int1, int_function0, int2, int_function1 } => {
                                CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![int_function0, int_function1], bool_functions: vec![] }
                            },
                            Self::Int1Point0 { int_list0, int_list1, bool0 } => {
                                CallValues { ints: vec![], bools: vec![bool0], int_lists: vec![int_list0, int_list1], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int1Point1 { int_list0, int_list1, bool0, int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![bool0], int_lists: vec![int_list0, int_list1], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int1Point2 { int_list0, int_list1, bool0, int0, int_function0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![bool0], int_lists: vec![int_list0, int_list1], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int1Point3 { int_list0, int_function0 } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![int_list0], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int1Point4 { int_list0, int_function0, int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int1Point5 { int_list0, int_function0, int0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int1Point6 { int_list0, int_function0, int0, int1, int_list1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![int_list0, int_list1], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int1Point7 { int_list0, int_function0 } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![int_list0], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int1Point8 { int_list0, int_function0, int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int1Point9 { int_function0 } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int1Point10 { int_function0, int_list0 } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![int_list0], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int2Point0 { int_list0 } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int2Point1 { int_list0, int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int2Point2 { int_list0, int0, int_function0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int3Point0 { int_list0 } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int3Point1 { int_list0, int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int3Point2 { int_list0, int0, int_function0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int5Point0 { int_list0 } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int5Point1 {  } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int5Point2 { int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int5Point3 { int_list0 } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int5Point4 { int_list0, int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int5Point5 { int_list0, int0, int_list1 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![int_list0, int_list1], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int5Point6 { int_list0, int0, int_list1, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![int_list0, int_list1], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int5Point7 { int_list0, int0, int_list1, int1, int2 } => {
                                CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![], int_lists: vec![int_list0, int_list1], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int7Point0 { int0, int1, int2 } => {
                                CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int7Point1 { int0, int1, int2, int3 } => {
                                CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int8Point0 { int0, int1, int_function0 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int8Point1 { int0, int1, int_function0, int2 } => {
                                CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int8Point2 { int0, int1, int_function0, int2, int3 } => {
                                CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int9Point0 { int_list0, int0, int_function0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int9Point1 { int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int9Point2 { int_list0, int0, int_function0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int9Point3 { int_list0, int0, int_function0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int9Point4 { int_list0, int0, int_function0, int1, int_list1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![int_list0, int_list1], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int9Point5 { int_list0, int0, int_function0, int1, int_list1, int2 } => {
                                CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![], int_lists: vec![int_list0, int_list1], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int10Point0 { int_list0, int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int10Point1 { int_list0, int0, int_function0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::Int11Point0 { int_list0, int0, int_list1 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![int_list0, int_list1], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int11Point1 { int_list0, int0, int_list1, int_list2 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![int_list0, int_list1, int_list2], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int11Point2 { int_list0, int0, int_list1, int_list2, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![int_list0, int_list1, int_list2], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int11Point3 { int_list0, int0, int_list1, int_list2, int1, int2 } => {
                                CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![], int_lists: vec![int_list0, int_list1, int_list2], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int12Point0 { int0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int12Point1 { int0, int1, int2 } => {
                                CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int12Point2 { int0, int1, int2, int3 } => {
                                CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int13Point0 { int0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int13Point1 { int0, int1, int2 } => {
                                CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int13Point2 { int0, int1, int2, int3 } => {
                                CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int14Point0 { int0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Int14Point1 { int0, int1, int2 } => {
                                CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point0 { int_list0, int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point1 { int_list0, int0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point2 { int_list0, int0, int1, int2 } => {
                                CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool0Point3 { int_list0, int0, int1, int2, bool0 } => {
                                CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![bool0], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool1Point0 { int_list0, int_list1 } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![int_list0, int_list1], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool1Point1 { int_list0, int_list1, bool0 } => {
                                CallValues { ints: vec![], bools: vec![bool0], int_lists: vec![int_list0, int_list1], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool1Point2 { int_list0, int_list1, bool0, bool1 } => {
                                CallValues { ints: vec![], bools: vec![bool0, bool1], int_lists: vec![int_list0, int_list1], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool1Point3 { int_list0, int_list1, bool0, bool1, int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![bool0, bool1], int_lists: vec![int_list0, int_list1], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool1Point4 { int_list0, int_list1, bool0, bool1, int0, int1 } => {
                                CallValues { ints: vec![int0.into(), int1.into()], bools: vec![bool0, bool1], int_lists: vec![int_list0, int_list1], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool1Point5 { int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool1Point6 { int0, bool0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool1Point7 { bool0 } => {
                                CallValues { ints: vec![], bools: vec![bool0], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::Bool1Point8 { bool0, bool1 } => {
                                CallValues { ints: vec![], bools: vec![bool0, bool1], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::IntFunction0Point0 { int_list0, int0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::IntFunction0Point1 { int_list0, int0, int_function0 } => {
                                CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![int_function0], bool_functions: vec![] }
                            },
                            Self::BoolFunction0Point0 { int_list0 } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] }
                            },
                            Self::BoolFunction0Point1 { int_list0, bool_function0 } => {
                                CallValues { ints: vec![], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![bool_function0] }
                            },
                            Self::Canonical { values, .. } => values,
                        }
                    }
                }
                enum IntReturn {
                    Int1Call7 { int_list0: IntList, int_function0: IntCallable },
                    Int5Call5 { int_list0: IntList, int0: i128, int_list1: IntList },
                    Int8Call0 { int0: i128, int1: i128, int_function0: IntCallable },
                    Int9Call4 { int_list0: IntList, int0: i128, int_function0: IntCallable, int1: i128, int_list1: IntList },
                    Int11Call2 { int_list0: IntList, int0: i128, int_list1: IntList, int_list2: IntList, int1: i128 },
                    Int12Call0 { int0: i128, int1: i128 },
                    Int13Call0 { int0: i128, int1: i128 },
                    Bool0Call1 { int_list0: IntList, int0: i128, int1: i128 },
                    Bool1Call3 { int_list0: IntList, int_list1: IntList, bool0: bool, bool1: bool, int0: i128 },
                }
                impl IntReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                            Self::Int1Call7 { .. } => data::source::HostCallSite::from_static("example", "selected", data::source::SourceSpan::new(1174, 1193)),
                            Self::Int5Call5 { .. } => data::source::HostCallSite::from_static("example", "non_tail", data::source::SourceSpan::new(1806, 1820)),
                            Self::Int8Call0 { .. } => data::source::HostCallSite::from_static("example", "<anonymous:1>", data::source::SourceSpan::new(362, 378)),
                            Self::Int9Call4 { .. } => data::source::HostCallSite::from_static("example", "fold", data::source::SourceSpan::new(146, 169)),
                            Self::Int11Call2 { .. } => data::source::HostCallSite::from_static("example", "<anonymous:3>", data::source::SourceSpan::new(700, 714)),
                            Self::Int12Call0 { .. } => data::source::HostCallSite::from_static("example", "<anonymous:5>", data::source::SourceSpan::new(1350, 1367)),
                            Self::Int13Call0 { .. } => data::source::HostCallSite::from_static("example", "<anonymous:6>", data::source::SourceSpan::new(1545, 1559)),
                            Self::Bool0Call1 { .. } => data::source::HostCallSite::from_static("example", "verify", data::source::SourceSpan::new(557, 571)),
                            Self::Bool1Call3 { .. } => data::source::HostCallSite::from_static("example", "<anonymous:4>", data::source::SourceSpan::new(885, 899)),
                        }
                    }
                    fn small(self, result: i128) -> FunctionState {
                        match self {
                            Self::Int1Call7 { int_list0, int_function0 } => {
                                let int0 = result;
                                FunctionState::Int1Point8 { int_list0, int_function0, int0 }
                            },
                            Self::Int5Call5 { int_list0, int0, int_list1 } => {
                                let int1 = result;
                                FunctionState::Int5Point6 { int_list0, int0, int_list1, int1 }
                            },
                            Self::Int8Call0 { int0, int1, int_function0 } => {
                                let int2 = result;
                                FunctionState::Int8Point1 { int0, int1, int_function0, int2 }
                            },
                            Self::Int9Call4 { int_list0, int0, int_function0, int1, int_list1 } => {
                                let int2 = result;
                                FunctionState::Int9Point5 { int_list0, int0, int_function0, int1, int_list1, int2 }
                            },
                            Self::Int11Call2 { int_list0, int0, int_list1, int_list2, int1 } => {
                                let int2 = result;
                                FunctionState::Int11Point3 { int_list0, int0, int_list1, int_list2, int1, int2 }
                            },
                            Self::Int12Call0 { int0, int1 } => {
                                let int2 = result;
                                FunctionState::Int12Point1 { int0, int1, int2 }
                            },
                            Self::Int13Call0 { int0, int1 } => {
                                let int2 = result;
                                FunctionState::Int13Point1 { int0, int1, int2 }
                            },
                            Self::Bool0Call1 { int_list0, int0, int1 } => {
                                let int2 = result;
                                FunctionState::Bool0Point2 { int_list0, int0, int1, int2 }
                            },
                            Self::Bool1Call3 { int_list0, int_list1, bool0, bool1, int0 } => {
                                let int1 = result;
                                FunctionState::Bool1Point4 { int_list0, int_list1, bool0, bool1, int0, int1 }
                            },
                        }
                    }
                    fn resume(self, result: CallInteger) -> FunctionState {
                        if let Some(result) = result.small() {
                            return self.small(result);
                        }
                        match self {
                            Self::Int1Call7 { int_list0, int_function0 } => {
                                let int0 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(1)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
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
                                }, values: CallValues { ints: vec![int0], bools: vec![], int_lists: vec![int_list0], int_functions: vec![int_function0], bool_functions: vec![] } }
                            },
                            Self::Int5Call5 { int_list0, int0, int_list1 } => {
                                let int1 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(5)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 3,
                                    ints: 2,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 2,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: CallValues { ints: vec![int0.into(), int1], bools: vec![], int_lists: vec![int_list0, int_list1], int_functions: vec![], bool_functions: vec![] } }
                            },
                            Self::Int8Call0 { int0, int1, int_function0 } => {
                                let int2 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(8)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 1,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 0,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 1,
                                    bool_functions: 0,
                                }, values: CallValues { ints: vec![int0.into(), int1.into(), int2], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] } }
                            },
                            Self::Int9Call4 { int_list0, int0, int_function0, int1, int_list1 } => {
                                let int2 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(9)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 3,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 2,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 1,
                                    bool_functions: 0,
                                }, values: CallValues { ints: vec![int0.into(), int1.into(), int2], bools: vec![], int_lists: vec![int_list0, int_list1], int_functions: vec![int_function0], bool_functions: vec![] } }
                            },
                            Self::Int11Call2 { int_list0, int0, int_list1, int_list2, int1 } => {
                                let int2 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 3,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 3,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: CallValues { ints: vec![int0.into(), int1.into(), int2], bools: vec![], int_lists: vec![int_list0, int_list1, int_list2], int_functions: vec![], bool_functions: vec![] } }
                            },
                            Self::Int12Call0 { int0, int1 } => {
                                let int2 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(12)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
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
                                }, values: CallValues { ints: vec![int0.into(), int1.into(), int2], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }
                            },
                            Self::Int13Call0 { int0, int1 } => {
                                let int2 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(13)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
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
                                }, values: CallValues { ints: vec![int0.into(), int1.into(), int2], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }
                            },
                            Self::Bool0Call1 { int_list0, int0, int1 } => {
                                let int2 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 2,
                                    ints: 3,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 1,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: CallValues { ints: vec![int0.into(), int1.into(), int2], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] } }
                            },
                            Self::Bool1Call3 { int_list0, int_list1, bool0, bool1, int0 } => {
                                let int1 = result;
                                FunctionState::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(0),
                                    instruction: 4,
                                    ints: 2,
                                    bools: 2,
                                    bit_arrays: 0,
                                    int_lists: 2,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 0,
                                    bool_functions: 0,
                                }, values: CallValues { ints: vec![int0.into(), int1], bools: vec![bool0, bool1], int_lists: vec![int_list0, int_list1], int_functions: vec![], bool_functions: vec![] } }
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
                    Int1Call1 { int_list0: IntList, int_list1: IntList, bool0: bool, int0: i128 },
                }
                impl IntFunctionReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                            Self::Int1Call1 { .. } => data::source::HostCallSite::from_static("example", "selected", data::source::SourceSpan::new(1073, 1092)),
                        }
                    }
                    fn small(self, result: IntCallable) -> FunctionState {
                        match self {
                            Self::Int1Call1 { int_list0, int_list1, bool0, int0 } => {
                                let int_function0 = result.with_type(data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                });
                                FunctionState::Int1Point2 { int_list0, int_list1, bool0, int0, int_function0 }
                            },
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
                    IntTail { callee: FunctionState, completed: FunctionState, point: data::compiled::CompiledCheckpoint },
                    Int { value: i128, exit: data::graph::BlockGraphExitId },
                    IntBridge { function: data::function::IntFunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: IntReturn },
                    Bool { value: bool, exit: data::graph::BlockGraphExitId },
                    IntFunctionCall { callee: FunctionState, caller: IntFunctionReturn },
                    IntFunction { value: IntCallable, exit: data::graph::BlockGraphExitId },
                    BoolFunction { value: BoolCallable, exit: data::graph::BlockGraphExitId },
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
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(1)) => calls_int_1_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(2)) => calls_int_2_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(3)) => calls_int_3_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(5)) => calls_int_5_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(7)) => calls_int_7_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(8)) => calls_int_8_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(9)) => calls_int_9_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(10)) => calls_int_10_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(11)) => calls_int_11_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(12)) => calls_int_12_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(13)) => calls_int_13_state(point, values),
                            data::compiled::CallTarget::Int(data::function::IntFunctionId(14)) => calls_int_14_state(point, values),
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)) => calls_bool_0_state(point, values),
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)) => calls_bool_1_state(point, values),
                            data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(0)) => calls_intfunction_0_state(point, values),
                            data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(0)) => calls_boolfunction_0_state(point, values),
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
                                FunctionStep::IntCall { callee, caller } => {
                                    self.integer_returns.push(caller);
                                    active = callee;
                                },
                                FunctionStep::IntTail { callee, completed, point } => {
                                    if self.integer_returns.is_empty() {
                                        return CallProgress::Interpreted { point, values: completed.values() };
                                    }
                                    *budget -= 1;
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
                                        self.active = Some(caller.resume(value));
                                        self
                                    }),
                                },
                                FunctionStep::Bool { value, exit } => {
                                    if let Some(caller) = self.boolean_returns.pop() {
                                        active = caller.small(value);
                                    } else {
                                        self.integer_returns.clear();
                                        self.boolean_returns.clear();
                                        self.integer_function_returns.clear();
                                        self.boolean_function_returns.clear();
                                        return CallProgress::Complete { exit, output: CallOutput::Bool(value), execution: self };
                                    }
                                },
                                FunctionStep::IntFunctionCall { callee, caller } => {
                                    self.integer_function_returns.push(caller);
                                    active = callee;
                                },
                                FunctionStep::IntFunction { value, exit } => {
                                    if let Some(caller) = self.integer_function_returns.pop() {
                                        active = caller.small(value);
                                    } else {
                                        self.integer_returns.clear();
                                        self.boolean_returns.clear();
                                        self.integer_function_returns.clear();
                                        self.boolean_function_returns.clear();
                                        return CallProgress::Complete { exit, output: CallOutput::IntFunction(value), execution: self };
                                    }
                                },
                                FunctionStep::BoolFunction { value, exit } => {
                                    if let Some(caller) = self.boolean_function_returns.pop() {
                                        active = caller.small(value);
                                    } else {
                                        self.integer_returns.clear();
                                        self.boolean_returns.clear();
                                        self.integer_function_returns.clear();
                                        self.boolean_function_returns.clear();
                                        return CallProgress::Complete { exit, output: CallOutput::BoolFunction(value), execution: self };
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
                        FunctionState::Int0Point0 { int_list0, int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point0 { int_list0, int0, int1 }); }
                            *budget -= 1;
                            let int_function0 = ops.int_closure(data::function::IntFunctionId(7), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            }, vec![CallCapture::int(data::graph::IntLocalId(1), int0), CallCapture::int(data::graph::IntLocalId(2), int1)]);
                            FunctionStep::Next(FunctionState::Int0Point1 { int_list0, int0, int1, int_function0 })
                        },
                        FunctionState::Int0Point1 { int_list0, int0, int1, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point1 { int_list0, int0, int1, int_function0 }); }
                            *budget -= 1;
                            let int2 = 0_i128;
                            if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 2,
                                ints: 3,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 1,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 1,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![int_function0], bool_functions: vec![] } }; }
                            FunctionStep::Next(FunctionState::Int0Point2 { int_list0, int0, int1, int_function0, int2 })
                        },
                        FunctionState::Int0Point2 { int_list0, int0, int1, int_function0, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point2 { int_list0, int0, int1, int_function0, int2 }); }
                            *budget -= 1;
                            let int_function1 = ops.int_closure(data::function::IntFunctionId(8), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            }, vec![CallCapture::int_function(data::graph::IntFunctionLocalId(0), int_function0.clone())]);
                            FunctionStep::Next(FunctionState::Int0Point3 { int_list0, int0, int1, int_function0, int2, int_function1 })
                        },
                        FunctionState::Int0Point3 { int_list0, int0, int1, int_function0, int2, int_function1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point3 { int_list0, int0, int1, int_function0, int2, int_function1 }); }
                            FunctionStep::IntTail { callee: FunctionState::Int9Point0 { int_list0: int_list0.clone(), int0: int2, int_function0: int_function1.clone() }, completed: FunctionState::Int0Point3 { int_list0, int0, int1, int_function0, int2, int_function1 }, point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 3,
                                ints: 3,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 1,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 2,
                                bool_functions: 0,
                            } }
                        },
                        FunctionState::Int1Point0 { int_list0, int_list1, bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point0 { int_list0, int_list1, bool0 }); }
                            *budget -= 1;
                            let int0 = 7_i128;
                            if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(1)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 1,
                                ints: 1,
                                bools: 1,
                                bit_arrays: 0,
                                int_lists: 2,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into()], bools: vec![bool0], int_lists: vec![int_list0, int_list1], int_functions: vec![], bool_functions: vec![] } }; }
                            FunctionStep::Next(FunctionState::Int1Point1 { int_list0, int_list1, bool0, int0 })
                        },
                        FunctionState::Int1Point1 { int_list0, int_list1, bool0, int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point1 { int_list0, int_list1, bool0, int0 }); }
                            *budget -= 1;
                            FunctionStep::IntFunctionCall { callee: FunctionState::IntFunction0Point0 { int_list0: int_list0.clone(), int0 }, caller: IntFunctionReturn::Int1Call1 { int_list0, int_list1, bool0, int0 } }
                        },
                        FunctionState::Int1Point2 { int_list0, int_list1, bool0, int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point2 { int_list0, int_list1, bool0, int0, int_function0 }); }
                            *budget -= 1;
                            if bool0 { FunctionStep::Next(FunctionState::Int1Point3 { int_list0: int_list1.clone(), int_function0: int_function0.clone() }) } else { FunctionStep::Next(FunctionState::Int1Point9 { int_function0: int_function0.clone() }) }
                        },
                        FunctionState::Int1Point3 { int_list0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point3 { int_list0, int_function0 }); }
                            *budget -= 1;
                            let int0 = 1_i128;
                            if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(1)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(1),
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
                            }, values: CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![int_function0], bool_functions: vec![] } }; }
                            FunctionStep::Next(FunctionState::Int1Point4 { int_list0, int_function0, int0 })
                        },
                        FunctionState::Int1Point4 { int_list0, int_function0, int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point4 { int_list0, int_function0, int0 }); }
                            *budget -= 1;
                            let int1 = 2_i128;
                            if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(1)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(1),
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
                            }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![int_function0], bool_functions: vec![] } }; }
                            FunctionStep::Next(FunctionState::Int1Point5 { int_list0, int_function0, int0, int1 })
                        },
                        FunctionState::Int1Point5 { int_list0, int_function0, int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point5 { int_list0, int_function0, int0, int1 }); }
                            *budget -= 1;
                            let int_list1 = ops.lists().prepend(data::type_::IntListTypeId {
                                list_type: data::type_::ListTypeId(0),
                            }, &[int0 as i64, int1 as i64], &int_list0);
                            FunctionStep::Next(FunctionState::Int1Point6 { int_list0, int_function0, int0, int1, int_list1 })
                        },
                        FunctionState::Int1Point6 { int_list0, int_function0, int0, int1, int_list1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point6 { int_list0, int_function0, int0, int1, int_list1 }); }
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Int1Point7 { int_list0: int_list1.clone(), int_function0: int_function0.clone() })
                        },
                        FunctionState::Int1Point7 { int_list0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point7 { int_list0, int_function0 }); }
                            *budget -= 1;
                            let callable = &int_function0;
                            let captures = callable.captures();
                            let target = callable.target();
                            if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (int_list0.clone(),)) {
                                return FunctionStep::IntCall { callee, caller: IntReturn::Int1Call7 { int_list0, int_function0 } };
                            }
                            FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("example", "selected", data::source::SourceSpan::new(1174, 1193)), arguments: CallArguments { values: CallValues { ints: vec![], bools: vec![], int_lists: vec![int_list0.clone()], int_functions: vec![], bool_functions: vec![] }, captures: Some(captures.retain()) }, caller: IntReturn::Int1Call7 { int_list0, int_function0 } }
                        },
                        FunctionState::Int1Point8 { int_list0, int_function0, int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point8 { int_list0, int_function0, int0 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int1Point9 { int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point9 { int_function0 }); }
                            *budget -= 1;
                            let int_list0 = ops.lists().value(data::type_::IntListTypeId {
                                list_type: data::type_::ListTypeId(0),
                            }, &[]);
                            FunctionStep::Next(FunctionState::Int1Point10 { int_function0, int_list0 })
                        },
                        FunctionState::Int1Point10 { int_function0, int_list0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point10 { int_function0, int_list0 }); }
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Int1Point7 { int_list0: int_list0.clone(), int_function0: int_function0.clone() })
                        },
                        FunctionState::Int2Point0 { int_list0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point0 { int_list0 }); }
                            *budget -= 1;
                            let int0 = 0_i128;
                            if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 1,
                                ints: 1,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 1,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] } }; }
                            FunctionStep::Next(FunctionState::Int2Point1 { int_list0, int0 })
                        },
                        FunctionState::Int2Point1 { int_list0, int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point1 { int_list0, int0 }); }
                            *budget -= 1;
                            let int_function0 = ops.int_closure(data::function::IntFunctionId(12), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            }, vec![]);
                            FunctionStep::Next(FunctionState::Int2Point2 { int_list0, int0, int_function0 })
                        },
                        FunctionState::Int2Point2 { int_list0, int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int2Point2 { int_list0, int0, int_function0 }); }
                            FunctionStep::IntTail { callee: FunctionState::Int9Point0 { int_list0: int_list0.clone(), int0, int_function0: int_function0.clone() }, completed: FunctionState::Int2Point2 { int_list0, int0, int_function0 }, point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 2,
                                ints: 1,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 1,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 1,
                                bool_functions: 0,
                            } }
                        },
                        FunctionState::Int3Point0 { int_list0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point0 { int_list0 }); }
                            *budget -= 1;
                            let int0 = 0_i128;
                            if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 1,
                                ints: 1,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 1,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] } }; }
                            FunctionStep::Next(FunctionState::Int3Point1 { int_list0, int0 })
                        },
                        FunctionState::Int3Point1 { int_list0, int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point1 { int_list0, int0 }); }
                            *budget -= 1;
                            let int_function0 = ops.int_closure(data::function::IntFunctionId(13), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            }, vec![]);
                            FunctionStep::Next(FunctionState::Int3Point2 { int_list0, int0, int_function0 })
                        },
                        FunctionState::Int3Point2 { int_list0, int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point2 { int_list0, int0, int_function0 }); }
                            FunctionStep::IntTail { callee: FunctionState::Int9Point0 { int_list0: int_list0.clone(), int0, int_function0: int_function0.clone() }, completed: FunctionState::Int3Point2 { int_list0, int0, int_function0 }, point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 2,
                                ints: 1,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 1,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 1,
                                bool_functions: 0,
                            } }
                        },
                        FunctionState::Int5Point0 { int_list0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int5Point0 { int_list0 }); }
                            *budget -= 1;
                            if int_list0.is_empty() { FunctionStep::Next(FunctionState::Int5Point1 {  }) } else { FunctionStep::Next(FunctionState::Int5Point3 { int_list0: int_list0.clone() }) }
                        },
                        FunctionState::Int5Point1 {  } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int5Point1 {  }); }
                            *budget -= 1;
                            let int0 = 0_i128;
                            if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(5)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(1),
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
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int5Point2 { int0 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int5Point2 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int5Point2 { int0 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int5Point3 { int_list0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int5Point3 { int_list0 }); }
                            let int0 = match ops.lists().index(&int_list0, 0) {
                                Some(value) => value,
                                None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(5)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
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
                                }, values: CallValues { ints: vec![], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] } },
                            };
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Int5Point4 { int_list0, int0 })
                        },
                        FunctionState::Int5Point4 { int_list0, int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int5Point4 { int_list0, int0 }); }
                            *budget -= 1;
                            let int_list1 = ops.lists().tail(&int_list0, data::type_::IntListTypeId {
                                list_type: data::type_::ListTypeId(0),
                            }, 1);
                            FunctionStep::Next(FunctionState::Int5Point5 { int_list0, int0, int_list1 })
                        },
                        FunctionState::Int5Point5 { int_list0, int0, int_list1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int5Point5 { int_list0, int0, int_list1 }); }
                            *budget -= 1;
                            FunctionStep::IntCall { callee: FunctionState::Int5Point0 { int_list0: int_list1.clone() }, caller: IntReturn::Int5Call5 { int_list0, int0, int_list1 } }
                        },
                        FunctionState::Int5Point6 { int_list0, int0, int_list1, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int5Point6 { int_list0, int0, int_list1, int1 }); }
                            *budget -= 1;
                            let int2 = int1 + int0;
                            if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(5)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(2),
                                instruction: 4,
                                ints: 3,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 2,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![], int_lists: vec![int_list0, int_list1], int_functions: vec![], bool_functions: vec![] } }; }
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int5Point7 { int_list0, int0, int_list1, int1, int2 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int2, exit: data::graph::BlockGraphExitId(1) }
                        },
                        FunctionState::Int5Point7 { int_list0, int0, int_list1, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int5Point7 { int_list0, int0, int_list1, int1, int2 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int2, exit: data::graph::BlockGraphExitId(1) }
                        },
                        FunctionState::Int7Point0 { int0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int7Point0 { int0, int1, int2 }); }
                            *budget -= 1;
                            let region0 = int0 * int1;
                            let region1 = region0 + int2;
                            let int3 = region1;
                            if int3 < i128::from(i64::MIN) || int3 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(7)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
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
                            }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int7Point1 { int0, int1, int2, int3 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int3, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int7Point1 { int0, int1, int2, int3 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int7Point1 { int0, int1, int2, int3 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int3, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int8Point0 { int0, int1, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point0 { int0, int1, int_function0 }); }
                            *budget -= 1;
                            let callable = &int_function0;
                            let captures = callable.captures();
                            let target = callable.target();
                            if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_1(target, &captures, (int1,)) {
                                return FunctionStep::IntCall { callee, caller: IntReturn::Int8Call0 { int0, int1, int_function0 } };
                            }
                            FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("example", "<anonymous:1>", data::source::SourceSpan::new(362, 378)), arguments: CallArguments { values: CallValues { ints: vec![int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }, captures: Some(captures.retain()) }, caller: IntReturn::Int8Call0 { int0, int1, int_function0 } }
                        },
                        FunctionState::Int8Point1 { int0, int1, int_function0, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point1 { int0, int1, int_function0, int2 }); }
                            *budget -= 1;
                            let int3 = int0 + int2;
                            if int3 < i128::from(i64::MIN) || int3 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(8)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 2,
                                ints: 4,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 1,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into(), int3.into()], bools: vec![], int_lists: vec![], int_functions: vec![int_function0], bool_functions: vec![] } }; }
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point2 { int0, int1, int_function0, int2, int3 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int3, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int8Point2 { int0, int1, int_function0, int2, int3 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int8Point2 { int0, int1, int_function0, int2, int3 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int3, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int9Point0 { int_list0, int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point0 { int_list0, int0, int_function0 }); }
                            *budget -= 1;
                            if int_list0.is_empty() { FunctionStep::Next(FunctionState::Int9Point1 { int0 }) } else { FunctionStep::Next(FunctionState::Int9Point2 { int_list0: int_list0.clone(), int0, int_function0: int_function0.clone() }) }
                        },
                        FunctionState::Int9Point1 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point1 { int0 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int9Point2 { int_list0, int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point2 { int_list0, int0, int_function0 }); }
                            let int1 = match ops.lists().index(&int_list0, 0) {
                                Some(value) => value,
                                None => return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(9)), point: data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(2),
                                    instruction: 0,
                                    ints: 1,
                                    bools: 0,
                                    bit_arrays: 0,
                                    int_lists: 1,
                                    strings: 0,
                                    customs: 0,
                                    custom_lists: 0,
                                    int_functions: 1,
                                    bool_functions: 0,
                                }, values: CallValues { ints: vec![int0.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![int_function0], bool_functions: vec![] } },
                            };
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Int9Point3 { int_list0, int0, int_function0, int1 })
                        },
                        FunctionState::Int9Point3 { int_list0, int0, int_function0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point3 { int_list0, int0, int_function0, int1 }); }
                            *budget -= 1;
                            let int_list1 = ops.lists().tail(&int_list0, data::type_::IntListTypeId {
                                list_type: data::type_::ListTypeId(0),
                            }, 1);
                            FunctionStep::Next(FunctionState::Int9Point4 { int_list0, int0, int_function0, int1, int_list1 })
                        },
                        FunctionState::Int9Point4 { int_list0, int0, int_function0, int1, int_list1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point4 { int_list0, int0, int_function0, int1, int_list1 }); }
                            *budget -= 1;
                            let callable = &int_function0;
                            let captures = callable.captures();
                            let target = callable.target();
                            if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_2(target, &captures, (int0, int1,)) {
                                return FunctionStep::IntCall { callee, caller: IntReturn::Int9Call4 { int_list0, int0, int_function0, int1, int_list1 } };
                            }
                            FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("example", "fold", data::source::SourceSpan::new(146, 169)), arguments: CallArguments { values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }, captures: Some(captures.retain()) }, caller: IntReturn::Int9Call4 { int_list0, int0, int_function0, int1, int_list1 } }
                        },
                        FunctionState::Int9Point5 { int_list0, int0, int_function0, int1, int_list1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int9Point5 { int_list0, int0, int_function0, int1, int_list1, int2 }); }
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Int9Point0 { int_list0: int_list1.clone(), int0: int2, int_function0: int_function0.clone() })
                        },
                        FunctionState::Int10Point0 { int_list0, int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int10Point0 { int_list0, int0 }); }
                            *budget -= 1;
                            let int_function0 = ops.int_closure(data::function::IntFunctionId(14), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            }, vec![]);
                            FunctionStep::Next(FunctionState::Int10Point1 { int_list0, int0, int_function0 })
                        },
                        FunctionState::Int10Point1 { int_list0, int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int10Point1 { int_list0, int0, int_function0 }); }
                            FunctionStep::IntTail { callee: FunctionState::Int9Point0 { int_list0: int_list0.clone(), int0, int_function0: int_function0.clone() }, completed: FunctionState::Int10Point1 { int_list0, int0, int_function0 }, point: data::compiled::CompiledCheckpoint {
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
                            } }
                        },
                        FunctionState::Int11Point0 { int_list0, int0, int_list1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point0 { int_list0, int0, int_list1 }); }
                            *budget -= 1;
                            let int_list2 = ops.lists().prepend(data::type_::IntListTypeId {
                                list_type: data::type_::ListTypeId(0),
                            }, &[int0 as i64], &int_list1);
                            FunctionStep::Next(FunctionState::Int11Point1 { int_list0, int0, int_list1, int_list2 })
                        },
                        FunctionState::Int11Point1 { int_list0, int0, int_list1, int_list2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point1 { int_list0, int0, int_list1, int_list2 }); }
                            *budget -= 1;
                            let int1 = 0_i128;
                            if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
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
                            }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![int_list0, int_list1, int_list2], int_functions: vec![], bool_functions: vec![] } }; }
                            FunctionStep::Next(FunctionState::Int11Point2 { int_list0, int0, int_list1, int_list2, int1 })
                        },
                        FunctionState::Int11Point2 { int_list0, int0, int_list1, int_list2, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point2 { int_list0, int0, int_list1, int_list2, int1 }); }
                            *budget -= 1;
                            FunctionStep::IntCall { callee: FunctionState::Int10Point0 { int_list0: int_list0.clone(), int0: int1 }, caller: IntReturn::Int11Call2 { int_list0, int0, int_list1, int_list2, int1 } }
                        },
                        FunctionState::Int11Point3 { int_list0, int0, int_list1, int_list2, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int11Point3 { int_list0, int0, int_list1, int_list2, int1, int2 }); }
                            FunctionStep::IntTail { callee: FunctionState::Int10Point0 { int_list0: int_list2.clone(), int0: int2 }, completed: FunctionState::Int11Point3 { int_list0, int0, int_list1, int_list2, int1, int2 }, point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 3,
                                ints: 3,
                                bools: 0,
                                bit_arrays: 0,
                                int_lists: 3,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 0,
                            } }
                        },
                        FunctionState::Int12Point0 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int12Point0 { int0, int1 }); }
                            *budget -= 1;
                            FunctionStep::IntBridge { function: data::function::IntFunctionId(15), site: data::source::HostCallSite::from_static("example", "<anonymous:5>", data::source::SourceSpan::new(1350, 1367)), arguments: CallArguments { values: CallValues { ints: vec![int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }, captures: None }, caller: IntReturn::Int12Call0 { int0, int1 } }
                        },
                        FunctionState::Int12Point1 { int0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int12Point1 { int0, int1, int2 }); }
                            *budget -= 1;
                            let int3 = int0 + int2;
                            if int3 < i128::from(i64::MIN) || int3 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(12)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 2,
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
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int12Point2 { int0, int1, int2, int3 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int3, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int12Point2 { int0, int1, int2, int3 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int12Point2 { int0, int1, int2, int3 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int3, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int13Point0 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point0 { int0, int1 }); }
                            *budget -= 1;
                            FunctionStep::IntBridge { function: data::function::IntFunctionId(16), site: data::source::HostCallSite::from_static("example", "<anonymous:6>", data::source::SourceSpan::new(1545, 1559)), arguments: CallArguments { values: CallValues { ints: vec![int1.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] }, captures: None }, caller: IntReturn::Int13Call0 { int0, int1 } }
                        },
                        FunctionState::Int13Point1 { int0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point1 { int0, int1, int2 }); }
                            *budget -= 1;
                            let int3 = int0 + int2;
                            if int3 < i128::from(i64::MIN) || int3 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(13)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 2,
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
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point2 { int0, int1, int2, int3 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int3, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int13Point2 { int0, int1, int2, int3 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int13Point2 { int0, int1, int2, int3 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int3, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int14Point0 { int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int14Point0 { int0, int1 }); }
                            *budget -= 1;
                            let int2 = int0 + int1;
                            if int2 < i128::from(i64::MIN) || int2 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(14)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
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
                            }, values: CallValues { ints: vec![int0.into(), int1.into(), int2.into()], bools: vec![], int_lists: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int14Point1 { int0, int1, int2 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int2, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Int14Point1 { int0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int14Point1 { int0, int1, int2 }); }
                            *budget -= 1;
                            FunctionStep::Int { value: int2, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Bool0Point0 { int_list0, int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point0 { int_list0, int0 }); }
                            *budget -= 1;
                            let int1 = 0_i128;
                            if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
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
                            }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_lists: vec![int_list0], int_functions: vec![], bool_functions: vec![] } }; }
                            FunctionStep::Next(FunctionState::Bool0Point1 { int_list0, int0, int1 })
                        },
                        FunctionState::Bool0Point1 { int_list0, int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point1 { int_list0, int0, int1 }); }
                            *budget -= 1;
                            FunctionStep::IntCall { callee: FunctionState::Int10Point0 { int_list0: int_list0.clone(), int0: int1 }, caller: IntReturn::Bool0Call1 { int_list0, int0, int1 } }
                        },
                        FunctionState::Bool0Point2 { int_list0, int0, int1, int2 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point2 { int_list0, int0, int1, int2 }); }
                            *budget -= 1;
                            let bool0 = int2 == int0;
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point3 { int_list0, int0, int1, int2, bool0 }); }
                            *budget -= 1;
                            FunctionStep::Bool { value: bool0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Bool0Point3 { int_list0, int0, int1, int2, bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point3 { int_list0, int0, int1, int2, bool0 }); }
                            *budget -= 1;
                            FunctionStep::Bool { value: bool0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Bool1Point0 { int_list0, int_list1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point0 { int_list0, int_list1 }); }
                            *budget -= 1;
                            let bool0 = ops.lists().equal(&int_list1, &int_list0);
                            FunctionStep::Next(FunctionState::Bool1Point1 { int_list0, int_list1, bool0 })
                        },
                        FunctionState::Bool1Point1 { int_list0, int_list1, bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point1 { int_list0, int_list1, bool0 }); }
                            *budget -= 1;
                            let bool1 = !ops.lists().equal(&int_list1, &int_list0);
                            FunctionStep::Next(FunctionState::Bool1Point2 { int_list0, int_list1, bool0, bool1 })
                        },
                        FunctionState::Bool1Point2 { int_list0, int_list1, bool0, bool1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point2 { int_list0, int_list1, bool0, bool1 }); }
                            *budget -= 1;
                            let int0 = 0_i128;
                            if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
                                instruction: 3,
                                ints: 1,
                                bools: 2,
                                bit_arrays: 0,
                                int_lists: 2,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 0,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into()], bools: vec![bool0, bool1], int_lists: vec![int_list0, int_list1], int_functions: vec![], bool_functions: vec![] } }; }
                            FunctionStep::Next(FunctionState::Bool1Point3 { int_list0, int_list1, bool0, bool1, int0 })
                        },
                        FunctionState::Bool1Point3 { int_list0, int_list1, bool0, bool1, int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point3 { int_list0, int_list1, bool0, bool1, int0 }); }
                            *budget -= 1;
                            FunctionStep::IntCall { callee: FunctionState::Int10Point0 { int_list0: int_list0.clone(), int0 }, caller: IntReturn::Bool1Call3 { int_list0, int_list1, bool0, bool1, int0 } }
                        },
                        FunctionState::Bool1Point4 { int_list0, int_list1, bool0, bool1, int0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point4 { int_list0, int_list1, bool0, bool1, int0, int1 }); }
                            *budget -= 1;
                            if bool0 { FunctionStep::Next(FunctionState::Bool1Point5 { int0: int1 }) } else { FunctionStep::Next(FunctionState::Bool1Point7 { bool0: bool1 }) }
                        },
                        FunctionState::Bool1Point5 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point5 { int0 }); }
                            *budget -= 1;
                            let bool0 = int0 > 0_i128;
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point6 { int0, bool0 }); }
                            *budget -= 1;
                            FunctionStep::Bool { value: bool0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Bool1Point6 { int0, bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point6 { int0, bool0 }); }
                            *budget -= 1;
                            FunctionStep::Bool { value: bool0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::Bool1Point7 { bool0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point7 { bool0 }); }
                            *budget -= 1;
                            let bool1 = !bool0;
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point8 { bool0, bool1 }); }
                            *budget -= 1;
                            FunctionStep::Bool { value: bool1, exit: data::graph::BlockGraphExitId(1) }
                        },
                        FunctionState::Bool1Point8 { bool0, bool1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point8 { bool0, bool1 }); }
                            *budget -= 1;
                            FunctionStep::Bool { value: bool1, exit: data::graph::BlockGraphExitId(1) }
                        },
                        FunctionState::IntFunction0Point0 { int_list0, int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction0Point0 { int_list0, int0 }); }
                            *budget -= 1;
                            let int_function0 = ops.int_closure(data::function::IntFunctionId(11), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            }, vec![CallCapture::int(data::graph::IntLocalId(0), int0), CallCapture::int_list(data::graph::IntListLocalId(1), int_list0.clone())]);
                            FunctionStep::Next(FunctionState::IntFunction0Point1 { int_list0, int0, int_function0 })
                        },
                        FunctionState::IntFunction0Point1 { int_list0, int0, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction0Point1 { int_list0, int0, int_function0 }); }
                            *budget -= 1;
                            FunctionStep::IntFunction { value: int_function0, exit: data::graph::BlockGraphExitId(0) }
                        },
                        FunctionState::BoolFunction0Point0 { int_list0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction0Point0 { int_list0 }); }
                            *budget -= 1;
                            let bool_function0 = ops.bool_closure(data::function::BoolFunctionId(1), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                            }, vec![CallCapture::int_list(data::graph::IntListLocalId(1), int_list0.clone())]);
                            FunctionStep::Next(FunctionState::BoolFunction0Point1 { int_list0, bool_function0 })
                        },
                        FunctionState::BoolFunction0Point1 { int_list0, bool_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::BoolFunction0Point1 { int_list0, bool_function0 }); }
                            *budget -= 1;
                            FunctionStep::BoolFunction { value: bool_function0, exit: data::graph::BlockGraphExitId(0) }
                        },
                    }
                }
                fn calls_entry_0(target: data::function::IntFunctionId, captures: &CallCaptureInputs<'_>, inputs: (IntList,)) -> Option<FunctionState> {
                    let (argument0,) = inputs;
                    match target.0 {
                        2 => Some(FunctionState::Int2Point0 { int_list0: argument0 }),
                        3 => Some(FunctionState::Int3Point0 { int_list0: argument0 }),
                        5 => Some(FunctionState::Int5Point0 { int_list0: argument0 }),
                        11 => Some(FunctionState::Int11Point0 { int_list0: argument0, int0: captures.int(data::graph::IntLocalId(0))?, int_list1: captures.int_list(data::graph::IntListLocalId(1))? }),
                        _ => None,
                    }
                }
                fn calls_entry_1(target: data::function::IntFunctionId, captures: &CallCaptureInputs<'_>, inputs: (i128,)) -> Option<FunctionState> {
                    let (argument0,) = inputs;
                    match target.0 {
                        7 => Some(FunctionState::Int7Point0 { int0: argument0, int1: captures.int(data::graph::IntLocalId(1))?, int2: captures.int(data::graph::IntLocalId(2))? }),
                        _ => None,
                    }
                }
                fn calls_entry_2(target: data::function::IntFunctionId, captures: &CallCaptureInputs<'_>, inputs: (i128, i128,)) -> Option<FunctionState> {
                    let (argument0, argument1,) = inputs;
                    match target.0 {
                        8 => Some(FunctionState::Int8Point0 { int0: argument0, int1: argument1, int_function0: captures.int_function(data::graph::IntFunctionLocalId(0))? }),
                        12 => Some(FunctionState::Int12Point0 { int0: argument0, int1: argument1 }),
                        13 => Some(FunctionState::Int13Point0 { int0: argument0, int1: argument1 }),
                        14 => Some(FunctionState::Int14Point0 { int0: argument0, int1: argument1 }),
                        _ => None,
                    }
                }
                fn calls_int_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int0Point0 { int_list0: values.int_list(0)?, int0: values.int(0)?, int1: values.int(1)? },
                        1 => FunctionState::Int0Point1 { int_list0: values.int_list(0)?, int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)? },
                        2 => FunctionState::Int0Point2 { int_list0: values.int_list(0)?, int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)?, int2: values.int(2)? },
                        3 => FunctionState::Int0Point3 { int_list0: values.int_list(0)?, int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)?, int2: values.int(2)?, int_function1: values.int_function(1)? },
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
                        0 => FunctionState::Int1Point0 { int_list0: values.int_list(0)?, int_list1: values.int_list(1)?, bool0: values.bool(0)? },
                        1 => FunctionState::Int1Point1 { int_list0: values.int_list(0)?, int_list1: values.int_list(1)?, bool0: values.bool(0)?, int0: values.int(0)? },
                        2 => FunctionState::Int1Point2 { int_list0: values.int_list(0)?, int_list1: values.int_list(1)?, bool0: values.bool(0)?, int0: values.int(0)?, int_function0: values.int_function(0)? },
                        3 => FunctionState::Int1Point3 { int_list0: values.int_list(0)?, int_function0: values.int_function(0)? },
                        4 => FunctionState::Int1Point4 { int_list0: values.int_list(0)?, int_function0: values.int_function(0)?, int0: values.int(0)? },
                        5 => FunctionState::Int1Point5 { int_list0: values.int_list(0)?, int_function0: values.int_function(0)?, int0: values.int(0)?, int1: values.int(1)? },
                        6 => FunctionState::Int1Point6 { int_list0: values.int_list(0)?, int_function0: values.int_function(0)?, int0: values.int(0)?, int1: values.int(1)?, int_list1: values.int_list(1)? },
                        7 => {
                            if !matches!(values.int_function_target(0)?, data::function::IntFunctionId(2) | data::function::IntFunctionId(3) | data::function::IntFunctionId(5) | data::function::IntFunctionId(11)) { return None; }
                            FunctionState::Int1Point7 { int_list0: values.int_list(0)?, int_function0: values.int_function(0)? }
                        },
                        8 => FunctionState::Int1Point8 { int_list0: values.int_list(0)?, int_function0: values.int_function(0)?, int0: values.int(0)? },
                        9 => FunctionState::Int1Point9 { int_function0: values.int_function(0)? },
                        10 => FunctionState::Int1Point10 { int_function0: values.int_function(0)?, int_list0: values.int_list(0)? },
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
                        0 => FunctionState::Int2Point0 { int_list0: values.int_list(0)? },
                        1 => FunctionState::Int2Point1 { int_list0: values.int_list(0)?, int0: values.int(0)? },
                        2 => FunctionState::Int2Point2 { int_list0: values.int_list(0)?, int0: values.int(0)?, int_function0: values.int_function(0)? },
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
                        0 => FunctionState::Int3Point0 { int_list0: values.int_list(0)? },
                        1 => FunctionState::Int3Point1 { int_list0: values.int_list(0)?, int0: values.int(0)? },
                        2 => FunctionState::Int3Point2 { int_list0: values.int_list(0)?, int0: values.int(0)?, int_function0: values.int_function(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_3_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point, values) { return Some(execution); }
                    let active = calls_int_3_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_5_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int5Point0 { int_list0: values.int_list(0)? },
                        1 => FunctionState::Int5Point1 {  },
                        2 => FunctionState::Int5Point2 { int0: values.int(0)? },
                        3 => FunctionState::Int5Point3 { int_list0: values.int_list(0)? },
                        4 => FunctionState::Int5Point4 { int_list0: values.int_list(0)?, int0: values.int(0)? },
                        5 => FunctionState::Int5Point5 { int_list0: values.int_list(0)?, int0: values.int(0)?, int_list1: values.int_list(1)? },
                        6 => FunctionState::Int5Point6 { int_list0: values.int_list(0)?, int0: values.int(0)?, int_list1: values.int_list(1)?, int1: values.int(1)? },
                        7 => FunctionState::Int5Point7 { int_list0: values.int_list(0)?, int0: values.int(0)?, int_list1: values.int_list(1)?, int1: values.int(1)?, int2: values.int(2)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_5_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(5)), point, values) { return Some(execution); }
                    let active = calls_int_5_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_7_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int7Point0 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        1 => FunctionState::Int7Point1 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_7_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(7)), point, values) { return Some(execution); }
                    let active = calls_int_7_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_8_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => {
                            if !matches!(values.int_function_target(0)?, data::function::IntFunctionId(7)) { return None; }
                            FunctionState::Int8Point0 { int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)? }
                        },
                        1 => FunctionState::Int8Point1 { int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)?, int2: values.int(2)? },
                        2 => FunctionState::Int8Point2 { int0: values.int(0)?, int1: values.int(1)?, int_function0: values.int_function(0)?, int2: values.int(2)?, int3: values.int(3)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_8_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(8)), point, values) { return Some(execution); }
                    let active = calls_int_8_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_9_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int9Point0 { int_list0: values.int_list(0)?, int0: values.int(0)?, int_function0: values.int_function(0)? },
                        1 => FunctionState::Int9Point1 { int0: values.int(0)? },
                        2 => FunctionState::Int9Point2 { int_list0: values.int_list(0)?, int0: values.int(0)?, int_function0: values.int_function(0)? },
                        3 => FunctionState::Int9Point3 { int_list0: values.int_list(0)?, int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)? },
                        4 => {
                            if !matches!(values.int_function_target(0)?, data::function::IntFunctionId(8) | data::function::IntFunctionId(12) | data::function::IntFunctionId(13) | data::function::IntFunctionId(14)) { return None; }
                            FunctionState::Int9Point4 { int_list0: values.int_list(0)?, int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)?, int_list1: values.int_list(1)? }
                        },
                        5 => FunctionState::Int9Point5 { int_list0: values.int_list(0)?, int0: values.int(0)?, int_function0: values.int_function(0)?, int1: values.int(1)?, int_list1: values.int_list(1)?, int2: values.int(2)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_9_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(9)), point, values) { return Some(execution); }
                    let active = calls_int_9_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_10_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int10Point0 { int_list0: values.int_list(0)?, int0: values.int(0)? },
                        1 => FunctionState::Int10Point1 { int_list0: values.int_list(0)?, int0: values.int(0)?, int_function0: values.int_function(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_10_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(10)), point, values) { return Some(execution); }
                    let active = calls_int_10_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_11_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int11Point0 { int_list0: values.int_list(0)?, int0: values.int(0)?, int_list1: values.int_list(1)? },
                        1 => FunctionState::Int11Point1 { int_list0: values.int_list(0)?, int0: values.int(0)?, int_list1: values.int_list(1)?, int_list2: values.int_list(2)? },
                        2 => FunctionState::Int11Point2 { int_list0: values.int_list(0)?, int0: values.int(0)?, int_list1: values.int_list(1)?, int_list2: values.int_list(2)?, int1: values.int(1)? },
                        3 => FunctionState::Int11Point3 { int_list0: values.int_list(0)?, int0: values.int(0)?, int_list1: values.int_list(1)?, int_list2: values.int_list(2)?, int1: values.int(1)?, int2: values.int(2)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_11_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(11)), point, values) { return Some(execution); }
                    let active = calls_int_11_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_12_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int12Point0 { int0: values.int(0)?, int1: values.int(1)? },
                        1 => FunctionState::Int12Point1 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        2 => FunctionState::Int12Point2 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_12_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(12)), point, values) { return Some(execution); }
                    let active = calls_int_12_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_13_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int13Point0 { int0: values.int(0)?, int1: values.int(1)? },
                        1 => FunctionState::Int13Point1 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        2 => FunctionState::Int13Point2 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, int3: values.int(3)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_13_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(13)), point, values) { return Some(execution); }
                    let active = calls_int_13_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_int_14_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int14Point0 { int0: values.int(0)?, int1: values.int(1)? },
                        1 => FunctionState::Int14Point1 { int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_14_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(14)), point, values) { return Some(execution); }
                    let active = calls_int_14_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_bool_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool0Point0 { int_list0: values.int_list(0)?, int0: values.int(0)? },
                        1 => FunctionState::Bool0Point1 { int_list0: values.int_list(0)?, int0: values.int(0)?, int1: values.int(1)? },
                        2 => FunctionState::Bool0Point2 { int_list0: values.int_list(0)?, int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)? },
                        3 => FunctionState::Bool0Point3 { int_list0: values.int_list(0)?, int0: values.int(0)?, int1: values.int(1)?, int2: values.int(2)?, bool0: values.bool(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point, values) { return Some(execution); }
                    let active = calls_bool_0_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_bool_1_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool1Point0 { int_list0: values.int_list(0)?, int_list1: values.int_list(1)? },
                        1 => FunctionState::Bool1Point1 { int_list0: values.int_list(0)?, int_list1: values.int_list(1)?, bool0: values.bool(0)? },
                        2 => FunctionState::Bool1Point2 { int_list0: values.int_list(0)?, int_list1: values.int_list(1)?, bool0: values.bool(0)?, bool1: values.bool(1)? },
                        3 => FunctionState::Bool1Point3 { int_list0: values.int_list(0)?, int_list1: values.int_list(1)?, bool0: values.bool(0)?, bool1: values.bool(1)?, int0: values.int(0)? },
                        4 => FunctionState::Bool1Point4 { int_list0: values.int_list(0)?, int_list1: values.int_list(1)?, bool0: values.bool(0)?, bool1: values.bool(1)?, int0: values.int(0)?, int1: values.int(1)? },
                        5 => FunctionState::Bool1Point5 { int0: values.int(0)? },
                        6 => FunctionState::Bool1Point6 { int0: values.int(0)?, bool0: values.bool(0)? },
                        7 => FunctionState::Bool1Point7 { bool0: values.bool(0)? },
                        8 => FunctionState::Bool1Point8 { bool0: values.bool(0)?, bool1: values.bool(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_1_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)), point, values) { return Some(execution); }
                    let active = calls_bool_1_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_intfunction_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::IntFunction0Point0 { int_list0: values.int_list(0)?, int0: values.int(0)? },
                        1 => FunctionState::IntFunction0Point1 { int_list0: values.int_list(0)?, int0: values.int(0)?, int_function0: values.int_function(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_intfunction_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(0)), point, values) { return Some(execution); }
                    let active = calls_intfunction_0_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_boolfunction_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::BoolFunction0Point0 { int_list0: values.int_list(0)? },
                        1 => FunctionState::BoolFunction0Point1 { int_list0: values.int_list(0)?, bool_function0: values.bool_function(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_boolfunction_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(0)), point, values) { return Some(execution); }
                    let active = calls_boolfunction_0_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }

                fn int_list_int_6(
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
                            CompiledResume::Exit(int_list_int_6_entry((), values, _lists, budget))
                        },
                        int_list_int_6_resume_1,
                        int_list_int_6_resume_2,
                        int_list_int_6_resume_3,
                        int_list_int_6_resume_4,
                        int_list_int_6_resume_5,
                        int_list_int_6_resume_6,
                    ];

                    let mut point = point;
                    loop {
                        match RESUME[point](values, _lists, budget) {
                            CompiledResume::Next(next) => point = next,
                            CompiledResume::Exit(progress) => return progress,
                        }
                    }
                }

                fn int_list_int_6_entry(
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
                    let b0_i0 = 1_i128;
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
                    let b0_i1 = 2_i128;
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
                    let b0_i2 = 3_i128;
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
                    let b0_l0 = _lists.value(data::type_::IntListTypeId {
                        list_type: data::type_::ListTypeId(0),
                    }, &[b0_i0 as i64, b0_i1 as i64, b0_i2 as i64]);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b0_l0]);
                        return data::compiled::CompiledProgress::Yield(4);
                    }
                    *budget -= 1;
                    let b0_i3 = 2_i128;
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
                    let b0_i4 = 1_i128;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2, b0_i3, b0_i4]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b0_l0]);
                        return data::compiled::CompiledProgress::Yield(6);
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2, b0_i3, b0_i4]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b0_l0]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                }

                fn int_list_int_6_resume_1(
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
                    let b0_i1 = 2_i128;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([]);
                    CompiledResume::Next(2)
                }

                fn int_list_int_6_resume_2(
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
                    let b0_i2 = 3_i128;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([]);
                    CompiledResume::Next(3)
                }

                fn int_list_int_6_resume_3(
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
                    let b0_l0 = _lists.value(data::type_::IntListTypeId {
                        list_type: data::type_::ListTypeId(0),
                    }, &[b0_i0 as i64, b0_i1 as i64, b0_i2 as i64]);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b0_l0]);
                    CompiledResume::Next(4)
                }

                fn int_list_int_6_resume_4(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let _list0 = values.int_lists.remove(0);
                    let (b0_i0, b0_i1, b0_i2, b0_l0,) = (values.ints[0], values.ints[1], values.ints[2], _list0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b0_l0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                    }
                    *budget -= 1;
                    let b0_i3 = 2_i128;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2, b0_i3]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b0_l0]);
                    CompiledResume::Next(5)
                }

                fn int_list_int_6_resume_5(
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
                    let b0_i4 = 1_i128;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2, b0_i3, b0_i4]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b0_l0]);
                    CompiledResume::Next(6)
                }

                fn int_list_int_6_resume_6(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let _list0 = values.int_lists.remove(0);
                    let (b0_i0, b0_i1, b0_i2, b0_i3, b0_i4, b0_l0,) = (values.ints[0], values.ints[1], values.ints[2], values.ints[3], values.ints[4], _list0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2, b0_i3, b0_i4]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b0_l0]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(6));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2, b0_i3, b0_i4]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b0_l0]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
                }

                fn int_list_int_list_0(
                    point: usize,
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {

                    const RESUME: [
                        fn(&mut data::compiled::int_list::IntListValues, &data::compiled::int_list::IntListOps<'_>, &mut usize) -> CompiledResume;
                        1
                    ] = [
                        |values, _lists, budget| {
                            let _list0 = values.int_lists.remove(0);
                            CompiledResume::Exit(int_list_int_list_0_entry((_list0,), values, _lists, budget))
                        },
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

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b0_l0]);
                    data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                }
                data::compiled::CompiledFunctions {
                    ints: data::Storage::Static(&[
                        data::compiled::CompiledFunction {
                            function: data::function::IntFunctionId(6),
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
                                ]),
                                run: int_list_int_6,
                            }),
                        },
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
                                        int_lists: 1,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                run: int_list_int_list_0,
                            }),
                        },
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
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
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
                                        instruction: 2,
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
                                        int_functions: 2,
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
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(1),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
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
                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(7)),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                        reference: false,
                                        captures: data::Storage::Static(&[
                                            data::graph::FunctionCapture::Int {
                                                target: data::graph::IntLocalId(1),
                                                source: data::graph::IntLocalId(0),
                                            },
                                            data::graph::FunctionCapture::Int {
                                                target: data::graph::IntLocalId(2),
                                                source: data::graph::IntLocalId(1),
                                            },
                                        ]),
                                    },
                                    data::compiled::CreationContract {
                                        point: 2,
                                        output: data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(1),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(8)),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                        reference: false,
                                        captures: data::Storage::Static(&[
                                            data::graph::FunctionCapture::IntFunction {
                                                target: data::graph::IntFunctionLocalId(0),
                                                source: data::graph::IntFunctionLocalId(0),
                                            },
                                        ]),
                                    },
                                ]),
                                returns: data::Storage::Static(&[]),
                                tails: data::Storage::Static(&[
                                    data::compiled::TailContract {
                                        point: 3,
                                        target: data::compiled::CallTarget::Int(data::function::IntFunctionId(9)),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                            data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(1),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                        ]),
                                        site: data::source::HostCallSite::from_static("example", "capturing_fold", data::source::SourceSpan::new(319, 381)),
                                    },
                                ]),
                                start: calls_int_0_start,
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
                                        ints: 0,
                                        bools: 1,
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
                                        instruction: 1,
                                        ints: 1,
                                        bools: 1,
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
                                        instruction: 2,
                                        ints: 1,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 2,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(1),
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
                                        block: data::graph::BlockId(1),
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
                                        block: data::graph::BlockId(1),
                                        instruction: 3,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 2,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(2),
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
                                        block: data::graph::BlockId(3),
                                        instruction: 0,
                                        ints: 0,
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
                                        instruction: 1,
                                        ints: 0,
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
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
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
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
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
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
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
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
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
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
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
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
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
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
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
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 1,
                                        output: data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(0))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("example", "selected", data::source::SourceSpan::new(1073, 1092)),
                                    },
                                    data::compiled::CallContract {
                                        point: 7,
                                        output: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        target: data::compiled::CallContractTarget::IntValue(data::graph::IntFunctionLocalId(0)),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                        ]),
                                        site: data::source::HostCallSite::from_static("example", "selected", data::source::SourceSpan::new(1174, 1193)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 8,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_int_1_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)),
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
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
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
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                    ]),
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
                                                    data::type_::ValueType::Int,
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
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(12)),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                        reference: false,
                                        captures: data::Storage::Static(&[]),
                                    },
                                ]),
                                returns: data::Storage::Static(&[]),
                                tails: data::Storage::Static(&[
                                    data::compiled::TailContract {
                                        point: 2,
                                        target: data::compiled::CallTarget::Int(data::function::IntFunctionId(9)),
                                        args: data::Storage::Static(&[
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
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                        ]),
                                        site: data::source::HostCallSite::from_static("example", "canonical", data::source::SourceSpan::new(1307, 1370)),
                                    },
                                ]),
                                start: calls_int_2_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)),
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
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
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
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                    ]),
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
                                                    data::type_::ValueType::Int,
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
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(13)),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                        reference: false,
                                        captures: data::Storage::Static(&[]),
                                    },
                                ]),
                                returns: data::Storage::Static(&[]),
                                tails: data::Storage::Static(&[
                                    data::compiled::TailContract {
                                        point: 2,
                                        target: data::compiled::CallTarget::Int(data::function::IntFunctionId(9)),
                                        args: data::Storage::Static(&[
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
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                        ]),
                                        site: data::source::HostCallSite::from_static("example", "failure", data::source::SourceSpan::new(1502, 1562)),
                                    },
                                ]),
                                start: calls_int_3_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(5)),
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
                                        block: data::graph::BlockId(1),
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
                                        block: data::graph::BlockId(2),
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
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 1,
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
                                        instruction: 2,
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
                                        block: data::graph::BlockId(2),
                                        instruction: 3,
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
                                        block: data::graph::BlockId(2),
                                        instruction: 4,
                                        ints: 3,
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
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                    ]),
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
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
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
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 5,
                                        output: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(5))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(1),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                        ]),
                                        site: data::source::HostCallSite::from_static("example", "non_tail", data::source::SourceSpan::new(1806, 1820)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 2,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    },
                                    data::compiled::ReturnContract {
                                        point: 7,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_int_5_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(7)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: false,
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
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
                                        block: data::graph::BlockId(0),
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
                                ]),
                                locals: data::Storage::Static(&[
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
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 1,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_int_7_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(8)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: true,
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
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
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 3,
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
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 4,
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
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 0,
                                        output: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        target: data::compiled::CallContractTarget::IntValue(data::graph::IntFunctionLocalId(0)),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("example", "<anonymous:1>", data::source::SourceSpan::new(362, 378)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 2,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_int_8_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(9)),
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
                                        int_lists: 1,
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
                                        int_lists: 1,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 2,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 2,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(2),
                                        instruction: 3,
                                        ints: 3,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 2,
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
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                    data::Storage::Static(&[
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
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
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
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 4,
                                        output: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        target: data::compiled::CallContractTarget::IntValue(data::graph::IntFunctionLocalId(0)),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("example", "fold", data::source::SourceSpan::new(146, 169)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 1,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_int_9_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(10)),
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
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[
                                    data::compiled::CreationContract {
                                        point: 0,
                                        output: data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::Int,
                                                    data::type_::ValueType::Int,
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(14)),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                        reference: false,
                                        captures: data::Storage::Static(&[]),
                                    },
                                ]),
                                returns: data::Storage::Static(&[]),
                                tails: data::Storage::Static(&[
                                    data::compiled::TailContract {
                                        point: 1,
                                        target: data::compiled::CallTarget::Int(data::function::IntFunctionId(9)),
                                        args: data::Storage::Static(&[
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
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                        ]),
                                        site: data::source::HostCallSite::from_static("example", "sum", data::source::SourceSpan::new(436, 493)),
                                    },
                                ]),
                                start: calls_int_10_start,
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
                                        block: data::graph::BlockId(0),
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
                                        block: data::graph::BlockId(0),
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
                                        block: data::graph::BlockId(0),
                                        instruction: 3,
                                        ints: 3,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 3,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
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
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(1),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 2,
                                        output: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(10))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("example", "<anonymous:3>", data::source::SourceSpan::new(700, 714)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[]),
                                tails: data::Storage::Static(&[
                                    data::compiled::TailContract {
                                        point: 3,
                                        target: data::compiled::CallTarget::Int(data::function::IntFunctionId(10)),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(2),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("example", "<anonymous:3>", data::source::SourceSpan::new(676, 715)),
                                    },
                                ]),
                                start: calls_int_11_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(12)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: true,
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
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
                                        block: data::graph::BlockId(0),
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
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
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
                                ]),
                                locals: data::Storage::Static(&[
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
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 0,
                                        output: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(15))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("example", "<anonymous:5>", data::source::SourceSpan::new(1350, 1367)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 2,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_int_12_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(13)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: true,
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
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
                                        block: data::graph::BlockId(0),
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
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
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
                                ]),
                                locals: data::Storage::Static(&[
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
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 0,
                                        output: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(16))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("example", "<anonymous:6>", data::source::SourceSpan::new(1545, 1559)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 2,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_int_13_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(14)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: false,
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
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
                                        block: data::graph::BlockId(0),
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
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 1,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_int_14_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)),
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
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
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
                                        instruction: 3,
                                        ints: 3,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 1,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
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
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 1,
                                        output: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(10))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("example", "verify", data::source::SourceSpan::new(557, 571)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 3,
                                        value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_bool_0_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)),
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
                                        int_lists: 2,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
                                        ints: 0,
                                        bools: 1,
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
                                        instruction: 2,
                                        ints: 0,
                                        bools: 2,
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
                                        instruction: 3,
                                        ints: 1,
                                        bools: 2,
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
                                        instruction: 4,
                                        ints: 2,
                                        bools: 2,
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
                                        block: data::graph::BlockId(1),
                                        instruction: 1,
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
                                        instruction: 1,
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
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
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
                                    data::Storage::Static(&[
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
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
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
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
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
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
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
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 3,
                                        output: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Int(data::function::IntFunctionId(10))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("example", "<anonymous:4>", data::source::SourceSpan::new(885, 899)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 6,
                                        value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    },
                                    data::compiled::ReturnContract {
                                        point: 8,
                                        value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_bool_1_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(0)),
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
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[
                                    data::compiled::CreationContract {
                                        point: 0,
                                        output: data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(11)),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                        reference: false,
                                        captures: data::Storage::Static(&[
                                            data::graph::FunctionCapture::Int {
                                                target: data::graph::IntLocalId(0),
                                                source: data::graph::IntLocalId(0),
                                            },
                                            data::graph::FunctionCapture::IntList {
                                                target: data::graph::IntListLocalId(1),
                                                source: data::graph::IntListLocalId(0),
                                            },
                                        ]),
                                    },
                                ]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 1,
                                        value: data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_intfunction_0_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(0)),
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
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 1,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 1,
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
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                            local: data::graph::IntListLocalId(0),
                                            type_id: data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(0),
                                            },
                                        }),
                                        data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[
                                    data::compiled::CreationContract {
                                        point: 0,
                                        output: data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                        target: data::graph::FunctionTarget::Bool(data::function::BoolFunctionId(1)),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                        },
                                        reference: false,
                                        captures: data::Storage::Static(&[
                                            data::graph::FunctionCapture::IntList {
                                                target: data::graph::IntListLocalId(1),
                                                source: data::graph::IntListLocalId(0),
                                            },
                                        ]),
                                    },
                                ]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 1,
                                        value: data::graph::ParamLocal::BoolFunction {
                                            local: data::graph::BoolFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[
                                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                                ]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                                            },
                                        },
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: calls_boolfunction_0_start,
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
                    0..17,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    17..19,
                    0..0,
                    0..0,
                    0..0,
                    19..20,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    20..21,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    21..22,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
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
                            data::type_::ValueShapeId(1),
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 3..6,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                            data::type_::ValueShapeId(1),
                            data::type_::ValueShapeId(4),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 6..7,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 7..8,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 8..9,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 9..10,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 10..10,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 10..11,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                shape: data::type_::ValueShapeId(0),
                            },
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                shape: data::type_::ValueShapeId(0),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 11..13,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::IntFunction {
                                    local: data::graph::IntFunctionLocalId(0),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    },
                                },
                                shape: data::type_::ValueShapeId(2),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 13..16,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(3),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 16..18,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 18..19,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[
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
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 19..21,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 21..23,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 23..25,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 25..26,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 26..27,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 27..29,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(4),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 29..30,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(4),
                        captures: data::Storage::Static(&[
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
                    },
                    data::function::FunctionContract {
                        parameters: 30..31,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 31..33,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(5),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 33..34,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(6),
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
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        },
                    },
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
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                        local: data::graph::IntListLocalId(0),
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
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                    },
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                    },
                    data::type_::ValueShapeDescriptor::Bool,
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                    },
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(4),
                    },
                    data::type_::ValueShapeDescriptor::String,
                ]),
                shape_types: data::Storage::Static(&[
                    data::type_::ValueType::Int,
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    }),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    }),
                    data::type_::ValueType::Bool,
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::List(data::type_::ListTypeId(0)),
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    }),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::List(data::type_::ListTypeId(0)),
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                    }),
                    data::type_::ValueType::String,
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
                data::program::LibraryFunctionEntry {
                    function: data::function::IntFunctionId(1),
                    inputs: data::program::LibraryInputConstructions {
                        variants: data::Storage::Static(&[]),
                        lists: data::program::LibraryListConstructions {
                            ints: data::Storage::Static(&[
                                data::type_::IntListTypeId {
                                    list_type: data::type_::ListTypeId(0),
                                },
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
                data::program::LibraryFunctionEntry {
                    function: data::function::IntFunctionId(2),
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
                data::program::LibraryFunctionEntry {
                    function: data::function::IntFunctionId(3),
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
                data::program::LibraryFunctionEntry {
                    function: data::function::IntFunctionId(4),
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
                data::program::LibraryFunctionEntry {
                    function: data::function::IntFunctionId(5),
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
            nils: data::Storage::Static(&[]),
            tuples: data::Storage::Static(&[]),
            lists: data::Storage::Static(&[]),
            functions: data::Storage::Static(&[
                data::program::LibraryFunctionEntry {
                    function: data::program::LibraryCallableEntry {
                        function: data::function::RuntimeFunctionFunctionTarget::Core(data::function::ProfiledFunctionFunctionId::Int(data::function::IntFunctionFunctionId(0))),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        },
                    },
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
                    callables: data::Storage::Static(&[
                        data::program::LibraryCallable {
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            },
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
                },
                data::program::LibraryFunctionEntry {
                    function: data::program::LibraryCallableEntry {
                        function: data::function::RuntimeFunctionFunctionTarget::Core(data::function::ProfiledFunctionFunctionId::Bool(data::function::BoolFunctionFunctionId(0))),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Bool),
                        },
                    },
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
                    callables: data::Storage::Static(&[
                        data::program::LibraryCallable {
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Bool),
                            },
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
                },
            ]),
        },
        exports: data::Storage::Static(&[
            data::Export {
                name: data::Text::Static("capturing_fold"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("verify"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("make_sum"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                    })),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("make_check"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
                    })),
                },
                slot: 1,
            },
            data::Export {
                name: data::Text::Static("selected"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                        data::type_::TypeMetadata::Bool,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 1,
            },
            data::Export {
                name: data::Text::Static("canonical"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 2,
            },
            data::Export {
                name: data::Text::Static("failure"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 3,
            },
            data::Export {
                name: data::Text::Static("list_return"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 4,
            },
            data::Export {
                name: data::Text::Static("non_tail"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 5,
            },
            data::Export {
                name: data::Text::Static("main"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 6,
            },
        ]),
    },
    value_functions: data::Storage::Static(&[]),
    never_functions: data::Storage::Static(&[]),
    callables: data::Storage::Static(&[]),
}
