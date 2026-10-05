data::HostedModuleArtifact {
    module: data::ModuleArtifact {
        format: 21,
        program: data::ProgramTables {
            root: data::source::module_id(0),
            modules: data::Storage::Static(&[
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("example"),
                    source_context: Some(data::source::SourceContext::from_static_block("src/custom_loop_boundaries.gleam", r#"
type Item {
  Item(Int)
}

fn walk(
  items: List(Item),
  total: Int,
  fail: Bool,
  apply: fn(Int, Item) -> Int,
) {
  case items {
    [] ->
      case fail {
        True -> panic
        False -> {
          echo total
          let assert [result] = [total]
          result
        }
      }
    [head, ..tail] -> walk(tail, apply(total + 1, head), fail, apply)
  }
}

fn add(total: Int, item: Item) {
  let Item(value) = item
  total + value
}

fn relay(total: Int, item: Item) {
  add(total, item)
}

pub fn integer(count: Int, fail: Bool, connected: Bool, initial: Int) {
  let apply = case connected {
    True -> add
    False -> relay
  }
  walk(items(count, []), initial, fail, apply)
}

fn any(items: List(Item), seen: Bool, apply: fn(Bool, Item) -> Bool) {
  case items {
    [] -> seen
    [head, ..tail] -> any(tail, apply(seen, head), apply)
  }
}

pub fn boolean(bias: Int) {
  any([Item(1)], False, fn(_seen, item) {
    let Item(value) = item
    value + bias > 0
  })
}

pub fn main() {
  integer(1, False, True, 3)
}

fn items(count: Int, result: List(Item)) {
  case count {
    0 -> result
    _ -> items(count - 1, [Item(2), ..result])
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
                                parameter_count: 4,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..4,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                                subject: data::graph::BoolLocalId(1),
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(3),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 4..7,
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
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 7..11,
                                            instructions: 1..3,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 11..14,
                                            instructions: 3..4,
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
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
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
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
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
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
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
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
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
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
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
                                                            data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
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
                                                        data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                                family: data::function::FunctionReturnFamily::Int,
                                                kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(1))),
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
                                                shape: data::type_::ValueShapeId(4),
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
                                                shape: data::type_::ValueShapeId(4),
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
                                                    data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                        local: data::graph::CustomListLocalId(0),
                                                        type_id: data::type_::CustomListTypeId {
                                                            list_type: data::type_::ListTypeId(0),
                                                            item_type: data::type_::CustomTypeId(0),
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "integer", data::source::SourceSpan::new(660, 676)),
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
                                                shape: data::type_::ValueShapeId(3),
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
                                                kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(2))),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::IntFunctionId(3),
                                            site: data::source::HostCallSite::from_static("example", "integer", data::source::SourceSpan::new(655, 699)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(1),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::CustomTypeId(0),
                                                },
                                            }),
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(2),
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
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                },
                                                index: 0,
                                            }),
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
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::IntFunctionId(1),
                                            site: data::source::HostCallSite::from_static("example", "relay", data::source::SourceSpan::new(491, 507)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
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
                                                                family: data::graph::StorageFamily::IntFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(7),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                            local: data::graph::CustomListLocalId(0),
                                                            type_id: data::type_::CustomListTypeId {
                                                                list_type: data::type_::ListTypeId(0),
                                                                item_type: data::type_::CustomTypeId(0),
                                                            },
                                                        }),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                            params: 4..6,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                                subject: data::graph::BoolLocalId(0),
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
                                            params: 6..6,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::SourceStop(data::graph::SourceStop {
                                                kind: data::graph::SourceStopKind::Panic,
                                                message: None,
                                                site: data::source::PanicSite::from_static("example", "walk", data::source::SourceSpan::new(179, 184)),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 6..7,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Echo(data::graph::Echo {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                message: None,
                                                site: data::source::EchoSite::from_static("example", "walk", data::source::SourceSpan::new(214, 224)),
                                                next: data::graph::Edge {
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
                                            params: 7..8,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(1),
                                                    },
                                                }),
                                                pattern: data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                    elements: data::Storage::Static(&[
                                                        data::graph::MatchPattern::Bind(data::graph::MatchPatternBinding {
                                                            index: 0,
                                                        }),
                                                    ]),
                                                    tail: None,
                                                }),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(5),
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
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(6),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                            local: data::graph::IntListLocalId(0),
                                                            type_id: data::type_::IntListTypeId {
                                                                list_type: data::type_::ListTypeId(1),
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
                                            params: 8..9,
                                            instructions: 1..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 9..10,
                                            instructions: 1..1,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(1),
                                                    },
                                                }),
                                                message: None,
                                                site: data::source::PanicSite::from_static("example", "walk", data::source::SourceSpan::new(235, 245)),
                                                pattern_span: data::source::SourceSpan::new(246, 254),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 10..14,
                                            instructions: 1..5,
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
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                            shape: data::type_::ValueShapeId(4),
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
                                            shape: data::type_::ValueShapeId(3),
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
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(1),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(8),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(0),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::CustomTypeId(0),
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
                                            shape: data::type_::ValueShapeId(1),
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
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(1),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(8),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Int(data::type_::IntListTypeId {
                                                list_type: data::type_::ListTypeId(1),
                                            }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                                data::graph::IntLocalId(0),
                                            ])))),
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
                                                shape: data::type_::ValueShapeId(4),
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
                                                            shape_id: data::type_::CustomValueShapeId(0),
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "walk", data::source::SourceSpan::new(333, 355)),
                                            }),
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
                                                        shape_id: data::type_::CustomValueShapeId(1),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(5),
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
                                                        shape_id: data::type_::CustomValueShapeId(1),
                                                    },
                                                },
                                            ])))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(false)),
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
                                                shape: data::type_::ValueShapeId(7),
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
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::Int {
                                                            target: data::graph::IntLocalId(0),
                                                            source: data::graph::IntLocalId(0),
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
                                            function: data::function::BoolFunctionId(2),
                                            site: data::source::HostCallSite::from_static("example", "boolean", data::source::SourceSpan::new(899, 991)),
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
                                            instructions: 0..3,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(0),
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(2),
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
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                },
                                                index: 0,
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
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::GtInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
                                                right: data::graph::IntegerOperand::Immediate(0),
                                            })),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::BoolLocalId(1)),
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
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
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
                                            shape: data::type_::ValueShapeId(7),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(0),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::CustomTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
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
                                                        shape_id: data::type_::CustomValueShapeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(2),
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
                                                shape: data::type_::ValueShapeId(4),
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
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::FunctionCall {
                                                function: data::graph::BoolFunctionLocalId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(0),
                                                            shape_id: data::type_::CustomValueShapeId(0),
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "any", data::source::SourceSpan::new(836, 853)),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
                                ]),
                            },
                        })),
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
                                            terminator: data::graph::Terminator::IntSwitch(data::graph::IntSwitch {
                                                subject: data::graph::IntLocalId(0),
                                                clauses: data::Storage::Static(&[
                                                    (data::graph::IntegerLiteral {
                                                        sign: data::Sign::NoSign,
                                                        digits: data::Storage::Static(&[]),
                                                    }, data::graph::Edge {
                                                        target: data::graph::BlockId(1),
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
                                                    }),
                                                ]),
                                                fallback: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..3,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 3..5,
                                            instructions: 0..4,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
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
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                local: data::graph::CustomListLocalId(0),
                                                type_id: data::type_::CustomListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                    item_type: data::type_::CustomTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(4),
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
                                            shape: data::type_::ValueShapeId(4),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
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
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(1),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(5),
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
                                                    local: data::graph::CustomListLocalId(1),
                                                    type_id: data::type_::CustomListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                        item_type: data::type_::CustomTypeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(4),
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
                                                            shape_id: data::type_::CustomValueShapeId(1),
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
                        }))),
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

                fn callback_int_1_call(
                    inputs: &data::compiled::custom_loop::CallbackInputs<'_>,
                    values: &mut data::compiled::custom::CustomValues,
                    budget: &mut usize,
                ) -> data::compiled::custom_loop::CallbackProgress<i128> {
                    let Some(_integer0) = inputs.integer(0) else {
                        return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Entry);
                    };
                    callback_int_1_entry((_integer0, inputs.custom(0),), values, budget)
                }

                fn callback_int_1_entry(
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
                    let b0_i1 = match b0_c0.integer(0) {
                        Some(value) => value,
                        None => {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b0_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.customs.clear();
                            values.customs.extend([b0_c0]);
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
                        values.customs.extend([b0_c0]);
                        return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(1));
                    }
                    *budget -= 1;
                    let b0_i2 = b0_i0 + b0_i1;
                    if b0_i2 < i128::from(i64::MIN) || b0_i2 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([b0_c0]);
                        return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(2));
                    }
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.customs.clear();
                        values.customs.extend([b0_c0]);
                        return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(2));
                    }
                    *budget -= 1;
                    data::compiled::custom_loop::CallbackProgress::Complete(b0_i2)
                }

                fn callback_bool_1_call(
                    inputs: &data::compiled::custom_loop::CallbackInputs<'_>,
                    values: &mut data::compiled::custom::CustomValues,
                    budget: &mut usize,
                ) -> data::compiled::custom_loop::CallbackProgress<bool> {
                    let Some(_integer0) = inputs.integer(0) else {
                        return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Entry);
                    };
                    callback_bool_1_entry((_integer0, inputs.boolean(0), inputs.custom(0),), values, budget)
                }

                fn callback_bool_1_entry(
                    inputs: (i128, bool, data::compiled::custom::CustomInput,),
                    values: &mut data::compiled::custom::CustomValues,
                    budget: &mut usize,
                ) -> data::compiled::custom_loop::CallbackProgress<bool> {
                    let (b0_i0, b0_v0, b0_c0,) = inputs;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b0_v0]);
                        values.customs.clear();
                        values.customs.extend([b0_c0]);
                        return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(0));
                    }
                    let b0_i1 = match b0_c0.integer(0) {
                        Some(value) => value,
                        None => {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b0_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b0_v0]);
                            values.customs.clear();
                            values.customs.extend([b0_c0]);
                            return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(0));
                        }
                    };
                    *budget -= 1;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0, b0_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b0_v0]);
                        values.customs.clear();
                        values.customs.extend([b0_c0]);
                        return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(1));
                    }
                    *budget -= 1;
                    let b0_i2 = b0_i1 + b0_i0;
                    if b0_i2 < i128::from(i64::MIN) || b0_i2 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b0_v0]);
                        values.customs.clear();
                        values.customs.extend([b0_c0]);
                        return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(2));
                    }
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b0_v0]);
                        values.customs.clear();
                        values.customs.extend([b0_c0]);
                        return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(2));
                    }
                    *budget -= 1;
                    let b0_v1 = b0_i2 > 0_i128;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0, b0_i1, b0_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b0_v0, b0_v1]);
                        values.customs.clear();
                        values.customs.extend([b0_c0]);
                        return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Yield(3));
                    }
                    *budget -= 1;
                    data::compiled::custom_loop::CallbackProgress::Complete(b0_v1)
                }
                use data::compiled::calls::{BoolCallable, CallExecution, CallInputs, CallInteger, CallOps, CallProgress, CallStorage, CallValues, IntCallable};
                enum FunctionState {
                    Int0Point0 { int0: i128, bool0: bool, bool1: bool, int1: i128 },
                    Int0Point1 { int0: i128, bool0: bool, int1: i128 },
                    Int0Point2 { int0: i128, bool0: bool, int1: i128, int_function0: IntCallable },
                    Int0Point3 { int_function0: IntCallable, int0: i128, bool0: bool, int1: i128 },
                    Int0Point4 { int0: i128, bool0: bool, int1: i128 },
                    Int0Point5 { int0: i128, bool0: bool, int1: i128, int_function0: IntCallable },
                    Bool0Point0 { int0: i128 },
                    Bool0Point1 { int0: i128, int1: i128 },
                }
                enum IntReturn {
                }
                impl IntReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                        }
                    }
                    fn small(self, result: i128) -> FunctionState {
                        let _ = result;
                        match self {
                        }
                    }
                    fn resume(self, result: CallInteger) -> FunctionState {
                        if let Some(result) = result.small() {
                            return self.small(result);
                        }
                        match self {
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
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)) => calls_bool_0_state(point, values),
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
                        FunctionState::Int0Point0 { int0, bool0, bool1, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point0 { int0, bool0, bool1, int1 }); }
                            *budget -= 1;
                            if bool1 { FunctionStep::Next(FunctionState::Int0Point1 { int0, bool0, int1 }) } else { FunctionStep::Next(FunctionState::Int0Point4 { int0, bool0, int1 }) }
                        },
                        FunctionState::Int0Point1 { int0, bool0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point1 { int0, bool0, int1 }); }
                            *budget -= 1;
                            let int_function0 = ops.int_reference(data::function::IntFunctionId(1), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            });
                            FunctionStep::Next(FunctionState::Int0Point2 { int0, bool0, int1, int_function0 })
                        },
                        FunctionState::Int0Point2 { int0, bool0, int1, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point2 { int0, bool0, int1, int_function0 }); }
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Int0Point3 { int_function0: int_function0.clone(), int0, bool0, int1 })
                        },
                        FunctionState::Int0Point3 { int_function0, int0, bool0, int1 } => {
                            FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(2),
                                instruction: 0,
                                ints: 2,
                                bools: 1,
                                bit_arrays: 0,
                                int_lists: 0,
                                strings: 0,
                                customs: 0,
                                custom_lists: 0,
                                int_functions: 1,
                                bool_functions: 0,
                            }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![bool0], int_functions: vec![int_function0], bool_functions: vec![] } }
                        },
                        FunctionState::Int0Point4 { int0, bool0, int1 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point4 { int0, bool0, int1 }); }
                            *budget -= 1;
                            let int_function0 = ops.int_reference(data::function::IntFunctionId(2), data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            });
                            FunctionStep::Next(FunctionState::Int0Point5 { int0, bool0, int1, int_function0 })
                        },
                        FunctionState::Int0Point5 { int0, bool0, int1, int_function0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point5 { int0, bool0, int1, int_function0 }); }
                            *budget -= 1;
                            FunctionStep::Next(FunctionState::Int0Point3 { int_function0: int_function0.clone(), int0, bool0, int1 })
                        },
                        FunctionState::Bool0Point0 { int0 } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point0 { int0 }); }
                            *budget -= 1;
                            let int1 = 1_i128;
                            if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_functions: vec![], bool_functions: vec![] } }; }
                            FunctionStep::Next(FunctionState::Bool0Point1 { int0, int1 })
                        },
                        FunctionState::Bool0Point1 { int0, int1 } => {
                            FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: CallValues { ints: vec![int0.into(), int1.into()], bools: vec![], int_functions: vec![], bool_functions: vec![] } }
                        },
                    }
                }
                fn calls_int_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Int0Point0 { int0: values.int(0)?, bool0: values.bool(0)?, bool1: values.bool(1)?, int1: values.int(1)? },
                        1 => FunctionState::Int0Point1 { int0: values.int(0)?, bool0: values.bool(0)?, int1: values.int(1)? },
                        2 => FunctionState::Int0Point2 { int0: values.int(0)?, bool0: values.bool(0)?, int1: values.int(1)?, int_function0: values.int_function(0)? },
                        3 => FunctionState::Int0Point3 { int_function0: values.int_function(0)?, int0: values.int(0)?, bool0: values.bool(0)?, int1: values.int(1)? },
                        4 => FunctionState::Int0Point4 { int0: values.int(0)?, bool0: values.bool(0)?, int1: values.int(1)? },
                        5 => FunctionState::Int0Point5 { int0: values.int(0)?, bool0: values.bool(0)?, int1: values.int(1)?, int_function0: values.int_function(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_int_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point, values) { return Some(execution); }
                    let active = calls_int_0_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_bool_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool0Point0 { int0: values.int(0)? },
                        1 => FunctionState::Bool0Point1 { int0: values.int(0)?, int1: values.int(1)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point, values) { return Some(execution); }
                    let active = calls_bool_0_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }

                fn custom_loop_int_3(
                    point: usize,
                    values: &mut data::compiled::custom_loop::CustomLoopValues,
                    _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                    budget: &mut usize,
                ) -> data::compiled::custom_loop::CustomLoopProgress {

                    const RESUME: [
                        fn(&mut data::compiled::custom_loop::CustomLoopValues, &data::compiled::custom_loop::CustomListOps<'_>, &mut usize) -> CustomLoopResume;
                        11
                    ] = [
                        |values, _lists, budget| {
                            let _custom_list0 = values.custom_lists.remove(0);
                            let _int_function0 = values.int_functions.remove(0);
                            CustomLoopResume::Exit(custom_loop_int_3_entry((values.ints[0], values.bools[0], _custom_list0, _int_function0,), values, _lists, budget))
                        },
                        custom_loop_int_3_resume_1,
                        custom_loop_int_3_resume_2,
                        custom_loop_int_3_resume_3,
                        custom_loop_int_3_resume_4,
                        custom_loop_int_3_resume_5,
                        custom_loop_int_3_resume_6,
                        custom_loop_int_3_resume_7,
                        custom_loop_int_3_resume_8,
                        custom_loop_int_3_resume_9,
                        custom_loop_int_3_resume_10,
                    ];

                    let mut point = point;
                    loop {
                        match RESUME[point](values, _lists, budget) {
                            CustomLoopResume::Next(next) => point = next,
                            CustomLoopResume::Exit(progress) => return progress,
                        }
                    }
                }

                fn custom_loop_int_3_entry(
                    inputs: (i128, bool, data::compiled::custom_loop::CustomList, data::compiled::custom_loop::IntCallback,),
                    values: &mut data::compiled::custom_loop::CustomLoopValues,
                    _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                    budget: &mut usize,
                ) -> data::compiled::custom_loop::CustomLoopProgress {
                    let (mut b0_i0, mut b0_v0, mut b0_l0, mut b0_f0,) = inputs;
                    'repeat: loop {
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b0_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b0_v0]);
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
                            let _next = (b0_i0, b0_v0,);
                            drop(b0_l0);
                            drop(b0_f0);
                            let (b1_i0, b1_v0,) = _next;
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b1_i0]);
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
                            if b1_v0 {
                                let () = ();
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[]);
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
                                    return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(2));
                                }

                                values.ints.clear();
                                values.ints.extend_from_slice(&[]);
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
                                return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Interpreted(2));
                            } else {
                                let (b3_i0,) = (b1_i0,);
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0]);
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
                                    return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(3));
                                }

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0]);
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
                                return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Interpreted(3));
                            }
                        } else {
                            let (b7_i0, b7_v0, b7_l0, b7_f0,) = (b0_i0, b0_v0, b0_l0, b0_f0,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b7_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[b7_v0]);
                                values.customs.clear();
                                values.customs.extend([]);
                                values.custom_lists.clear();
                                values.custom_lists.extend([b7_l0]);
                                values.int_functions.clear();
                                values.int_functions.extend([b7_f0]);
                                values.bool_functions.clear();
                                values.bool_functions.extend([]);
                                return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(6));
                            }
                            let b7_c0 = match _lists.index(&b7_l0, 0) {
                                Some(value) => value,
                                None => {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b7_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[b7_v0]);
                                    values.customs.clear();
                                    values.customs.extend([]);
                                    values.custom_lists.clear();
                                    values.custom_lists.extend([b7_l0]);
                                    values.int_functions.clear();
                                    values.int_functions.extend([b7_f0]);
                                    values.bool_functions.clear();
                                    values.bool_functions.extend([]);
                                    return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Interpreted(6));
                                }
                            };
                            *budget -= 1;
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b7_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[b7_v0]);
                                values.customs.clear();
                                values.customs.extend([b7_c0]);
                                values.custom_lists.clear();
                                values.custom_lists.extend([b7_l0]);
                                values.int_functions.clear();
                                values.int_functions.extend([b7_f0]);
                                values.bool_functions.clear();
                                values.bool_functions.extend([]);
                                return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(7));
                            }
                            *budget -= 1;
                            let b7_l1 = _lists.tail(&b7_l0, data::type_::CustomListTypeId {
                                list_type: data::type_::ListTypeId(0),
                                item_type: data::type_::CustomTypeId(0),
                            }, 1);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b7_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[b7_v0]);
                                values.customs.clear();
                                values.customs.extend([b7_c0]);
                                values.custom_lists.clear();
                                values.custom_lists.extend([b7_l0, b7_l1]);
                                values.int_functions.clear();
                                values.int_functions.extend([b7_f0]);
                                values.bool_functions.clear();
                                values.bool_functions.extend([]);
                                return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(8));
                            }
                            *budget -= 1;
                            let b7_i1 = b7_i0 + 1_i128;
                            if b7_i1 < i128::from(i64::MIN) || b7_i1 > i128::from(i64::MAX) {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b7_i0, b7_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[b7_v0]);
                                values.customs.clear();
                                values.customs.extend([b7_c0]);
                                values.custom_lists.clear();
                                values.custom_lists.extend([b7_l0, b7_l1]);
                                values.int_functions.clear();
                                values.int_functions.extend([b7_f0]);
                                values.bool_functions.clear();
                                values.bool_functions.extend([]);
                                return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Interpreted(9));
                            }
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b7_i0, b7_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[b7_v0]);
                                values.customs.clear();
                                values.customs.extend([b7_c0]);
                                values.custom_lists.clear();
                                values.custom_lists.extend([b7_l0, b7_l1]);
                                values.int_functions.clear();
                                values.int_functions.extend([b7_f0]);
                                values.bool_functions.clear();
                                values.bool_functions.extend([]);
                                return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(9));
                            }
                            *budget -= 1;
                            let b7_i2 = match b7_f0.call(
                                &data::compiled::custom_loop::CallbackArguments {
                                    ints: &[b7_i1],
                                    bools: &[],
                                    customs: &[&b7_c0],
                                },
                                &mut values.callee, budget,
                                ) {
                                data::compiled::custom_loop::CallbackProgress::Complete(value) => value,
                                data::compiled::custom_loop::CallbackProgress::Stopped(progress) => {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b7_i0, b7_i1]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[b7_v0]);
                                    values.customs.clear();
                                    values.customs.extend([b7_c0]);
                                    values.custom_lists.clear();
                                    values.custom_lists.extend([b7_l0, b7_l1]);
                                    values.int_functions.clear();
                                    values.int_functions.extend([b7_f0]);
                                    values.bool_functions.clear();
                                    values.bool_functions.extend([]);
                                    return data::compiled::custom_loop::CustomLoopProgress::Call { point: 9, progress };
                                }
                            };
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b7_i0, b7_i1, b7_i2]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[b7_v0]);
                                values.customs.clear();
                                values.customs.extend([b7_c0]);
                                values.custom_lists.clear();
                                values.custom_lists.extend([b7_l0, b7_l1]);
                                values.int_functions.clear();
                                values.int_functions.extend([b7_f0]);
                                values.bool_functions.clear();
                                values.bool_functions.extend([]);
                                return data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(10));
                            }
                            *budget -= 1;
                            {
                                let _next = (b7_i2, b7_v0, b7_l1, b7_f0,);
                                drop(b7_c0);
                                drop(b7_l0);
                                (b0_i0, b0_v0, b0_l0, b0_f0,) = _next;
                                continue 'repeat;
                            }
                        }
                    }
                }

                fn custom_loop_int_3_resume_1(
                    values: &mut data::compiled::custom_loop::CustomLoopValues,
                    _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                    budget: &mut usize,
                ) -> CustomLoopResume {
                    let (b1_i0, b1_v0,) = (values.ints[0], values.bools[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0]);
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
                    if b1_v0 {
                        let () = ();

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
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
                        CustomLoopResume::Next(2)
                    } else {
                        let (b3_i0,) = (b1_i0,);

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
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
                        CustomLoopResume::Next(3)
                    }
                }

                fn custom_loop_int_3_resume_2(
                    values: &mut data::compiled::custom_loop::CustomLoopValues,
                    _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                    budget: &mut usize,
                ) -> CustomLoopResume {
                    let () = ();
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[]);
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
                        return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(2)));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
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
                    CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Interpreted(2)))
                }

                fn custom_loop_int_3_resume_3(
                    values: &mut data::compiled::custom_loop::CustomLoopValues,
                    _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                    budget: &mut usize,
                ) -> CustomLoopResume {
                    let (b3_i0,) = (values.ints[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0]);
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
                        return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(3)));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0]);
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
                    CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Interpreted(3)))
                }

                fn custom_loop_int_3_resume_4(
                    values: &mut data::compiled::custom_loop::CustomLoopValues,
                    _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                    budget: &mut usize,
                ) -> CustomLoopResume {
                    let (b4_i0,) = (values.ints[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b4_i0]);
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
                        return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(4)));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b4_i0]);
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
                    CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Interpreted(4)))
                }

                fn custom_loop_int_3_resume_5(
                    values: &mut data::compiled::custom_loop::CustomLoopValues,
                    _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                    budget: &mut usize,
                ) -> CustomLoopResume {
                    let (b5_i0,) = (values.ints[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b5_i0]);
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
                        return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(5)));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b5_i0]);
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

                fn custom_loop_int_3_resume_6(
                    values: &mut data::compiled::custom_loop::CustomLoopValues,
                    _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                    budget: &mut usize,
                ) -> CustomLoopResume {
                    let _custom_list0 = values.custom_lists.remove(0);
                    let _int_function0 = values.int_functions.remove(0);
                    let (b7_i0, b7_v0, b7_l0, b7_f0,) = (values.ints[0], values.bools[0], _custom_list0, _int_function0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b7_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b7_v0]);
                        values.customs.clear();
                        values.customs.extend([]);
                        values.custom_lists.clear();
                        values.custom_lists.extend([b7_l0]);
                        values.int_functions.clear();
                        values.int_functions.extend([b7_f0]);
                        values.bool_functions.clear();
                        values.bool_functions.extend([]);
                        return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(6)));
                    }
                    let b7_c0 = match _lists.index(&b7_l0, 0) {
                        Some(value) => value,
                        None => {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b7_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b7_v0]);
                            values.customs.clear();
                            values.customs.extend([]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([b7_l0]);
                            values.int_functions.clear();
                            values.int_functions.extend([b7_f0]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([]);
                            return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Interpreted(6)));
                        }
                    };
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b7_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b7_v0]);
                    values.customs.clear();
                    values.customs.extend([b7_c0]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b7_l0]);
                    values.int_functions.clear();
                    values.int_functions.extend([b7_f0]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    CustomLoopResume::Next(7)
                }

                fn custom_loop_int_3_resume_7(
                    values: &mut data::compiled::custom_loop::CustomLoopValues,
                    _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                    budget: &mut usize,
                ) -> CustomLoopResume {
                    let _custom0 = values.customs.remove(0);
                    let _custom_list0 = values.custom_lists.remove(0);
                    let _int_function0 = values.int_functions.remove(0);
                    let (b7_i0, b7_v0, b7_c0, b7_l0, b7_f0,) = (values.ints[0], values.bools[0], _custom0, _custom_list0, _int_function0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b7_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b7_v0]);
                        values.customs.clear();
                        values.customs.extend([b7_c0]);
                        values.custom_lists.clear();
                        values.custom_lists.extend([b7_l0]);
                        values.int_functions.clear();
                        values.int_functions.extend([b7_f0]);
                        values.bool_functions.clear();
                        values.bool_functions.extend([]);
                        return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(7)));
                    }
                    *budget -= 1;
                    let b7_l1 = _lists.tail(&b7_l0, data::type_::CustomListTypeId {
                        list_type: data::type_::ListTypeId(0),
                        item_type: data::type_::CustomTypeId(0),
                    }, 1);

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b7_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b7_v0]);
                    values.customs.clear();
                    values.customs.extend([b7_c0]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b7_l0, b7_l1]);
                    values.int_functions.clear();
                    values.int_functions.extend([b7_f0]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    CustomLoopResume::Next(8)
                }

                fn custom_loop_int_3_resume_8(
                    values: &mut data::compiled::custom_loop::CustomLoopValues,
                    _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                    budget: &mut usize,
                ) -> CustomLoopResume {
                    let _custom0 = values.customs.remove(0);
                    let _custom_list1 = values.custom_lists.remove(1);
                    let _custom_list0 = values.custom_lists.remove(0);
                    let _int_function0 = values.int_functions.remove(0);
                    let (b7_i0, b7_v0, b7_c0, b7_l0, b7_l1, b7_f0,) = (values.ints[0], values.bools[0], _custom0, _custom_list0, _custom_list1, _int_function0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b7_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b7_v0]);
                        values.customs.clear();
                        values.customs.extend([b7_c0]);
                        values.custom_lists.clear();
                        values.custom_lists.extend([b7_l0, b7_l1]);
                        values.int_functions.clear();
                        values.int_functions.extend([b7_f0]);
                        values.bool_functions.clear();
                        values.bool_functions.extend([]);
                        return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(8)));
                    }
                    *budget -= 1;
                    let b7_i1 = b7_i0 + 1_i128;
                    if b7_i1 < i128::from(i64::MIN) || b7_i1 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b7_i0, b7_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b7_v0]);
                        values.customs.clear();
                        values.customs.extend([b7_c0]);
                        values.custom_lists.clear();
                        values.custom_lists.extend([b7_l0, b7_l1]);
                        values.int_functions.clear();
                        values.int_functions.extend([b7_f0]);
                        values.bool_functions.clear();
                        values.bool_functions.extend([]);
                        return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Interpreted(9)));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b7_i0, b7_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b7_v0]);
                    values.customs.clear();
                    values.customs.extend([b7_c0]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b7_l0, b7_l1]);
                    values.int_functions.clear();
                    values.int_functions.extend([b7_f0]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    CustomLoopResume::Next(9)
                }

                fn custom_loop_int_3_resume_9(
                    values: &mut data::compiled::custom_loop::CustomLoopValues,
                    _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                    budget: &mut usize,
                ) -> CustomLoopResume {
                    let _custom0 = values.customs.remove(0);
                    let _custom_list1 = values.custom_lists.remove(1);
                    let _custom_list0 = values.custom_lists.remove(0);
                    let _int_function0 = values.int_functions.remove(0);
                    let (b7_i0, b7_i1, b7_v0, b7_c0, b7_l0, b7_l1, b7_f0,) = (values.ints[0], values.ints[1], values.bools[0], _custom0, _custom_list0, _custom_list1, _int_function0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b7_i0, b7_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b7_v0]);
                        values.customs.clear();
                        values.customs.extend([b7_c0]);
                        values.custom_lists.clear();
                        values.custom_lists.extend([b7_l0, b7_l1]);
                        values.int_functions.clear();
                        values.int_functions.extend([b7_f0]);
                        values.bool_functions.clear();
                        values.bool_functions.extend([]);
                        return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(9)));
                    }
                    *budget -= 1;
                    let b7_i2 = match b7_f0.call(
                        &data::compiled::custom_loop::CallbackArguments {
                            ints: &[b7_i1],
                            bools: &[],
                            customs: &[&b7_c0],
                        },
                        &mut values.callee, budget,
                        ) {
                        data::compiled::custom_loop::CallbackProgress::Complete(value) => value,
                        data::compiled::custom_loop::CallbackProgress::Stopped(progress) => {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b7_i0, b7_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[b7_v0]);
                            values.customs.clear();
                            values.customs.extend([b7_c0]);
                            values.custom_lists.clear();
                            values.custom_lists.extend([b7_l0, b7_l1]);
                            values.int_functions.clear();
                            values.int_functions.extend([b7_f0]);
                            values.bool_functions.clear();
                            values.bool_functions.extend([]);
                            return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Call { point: 9, progress });
                        }
                    };

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b7_i0, b7_i1, b7_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b7_v0]);
                    values.customs.clear();
                    values.customs.extend([b7_c0]);
                    values.custom_lists.clear();
                    values.custom_lists.extend([b7_l0, b7_l1]);
                    values.int_functions.clear();
                    values.int_functions.extend([b7_f0]);
                    values.bool_functions.clear();
                    values.bool_functions.extend([]);
                    CustomLoopResume::Next(10)
                }

                fn custom_loop_int_3_resume_10(
                    values: &mut data::compiled::custom_loop::CustomLoopValues,
                    _lists: &data::compiled::custom_loop::CustomListOps<'_>,
                    budget: &mut usize,
                ) -> CustomLoopResume {
                    let _custom0 = values.customs.remove(0);
                    let _custom_list1 = values.custom_lists.remove(1);
                    let _custom_list0 = values.custom_lists.remove(0);
                    let _int_function0 = values.int_functions.remove(0);
                    let (b7_i0, b7_i1, b7_i2, b7_v0, b7_c0, b7_l0, b7_l1, b7_f0,) = (values.ints[0], values.ints[1], values.ints[2], values.bools[0], _custom0, _custom_list0, _custom_list1, _int_function0,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b7_i0, b7_i1, b7_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b7_v0]);
                        values.customs.clear();
                        values.customs.extend([b7_c0]);
                        values.custom_lists.clear();
                        values.custom_lists.extend([b7_l0, b7_l1]);
                        values.int_functions.clear();
                        values.int_functions.extend([b7_f0]);
                        values.bool_functions.clear();
                        values.bool_functions.extend([]);
                        return CustomLoopResume::Exit(data::compiled::custom_loop::CustomLoopProgress::Caller(data::compiled::CompiledProgress::Yield(10)));
                    }
                    *budget -= 1;
                    {
                        let _next = (b7_i2, b7_v0, b7_l1, b7_f0,);
                        drop(b7_c0);
                        drop(b7_l0);
                        let (b0_i0, b0_v0, b0_l0, b0_f0,) = _next;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[b0_v0]);
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
                            function: data::function::IntFunctionId(3),
                            implementation: data::compiled::CompiledImplementation::CustomLoop(data::Storage::Static(&data::compiled::CustomLoopImplementation {
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
                                        custom_lists: 1,
                                        int_functions: 1,
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
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(7),
                                        instruction: 0,
                                        ints: 1,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 1,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(7),
                                        instruction: 1,
                                        ints: 1,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 1,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(7),
                                        instruction: 2,
                                        ints: 1,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 2,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(7),
                                        instruction: 3,
                                        ints: 2,
                                        bools: 1,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 2,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(7),
                                        instruction: 4,
                                        ints: 3,
                                        bools: 1,
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
                                        point: 9,
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
                                                    shape_id: data::type_::CustomValueShapeId(0),
                                                },
                                            }),
                                        ]),
                                        output: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                    },
                                ]),
                                run: custom_loop_int_3,
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
                                                    shape_id: data::type_::CustomValueShapeId(0),
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
                                function: data::function::IntFunctionId(1),
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
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
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
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
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
                                ]),
                                returns: data::Storage::Static(&[
                                    data::graph::IntLocalId(2),
                                ]),
                                run: callback_int_1_call,
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
                                        ints: 1,
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
                                        block: data::graph::BlockId(0),
                                        instruction: 1,
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
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 3,
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
                                        block: data::graph::BlockId(0),
                                        instruction: 3,
                                        ints: 3,
                                        bools: 2,
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
                                    data::graph::BoolLocalId(1),
                                ]),
                                run: callback_bool_1_call,
                            },
                        ]),
                    },
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
                                        block: data::graph::BlockId(1),
                                        instruction: 1,
                                        ints: 2,
                                        bools: 1,
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
                                        ints: 2,
                                        bools: 1,
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
                                        block: data::graph::BlockId(3),
                                        instruction: 1,
                                        ints: 2,
                                        bools: 1,
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
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(1)),
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
                                        point: 4,
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
                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(2)),
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
                                start: calls_int_0_start,
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)),
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
                                start: calls_bool_0_start,
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
                    4..7,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
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
                ],
                functions: data::Storage::Static(&[
                    data::function::FunctionContract {
                        parameters: 0..4,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(1),
                            data::type_::ValueShapeId(1),
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 4..6,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(2),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 6..8,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(2),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 8..12,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(4),
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(1),
                            data::type_::ValueShapeId(3),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 12..13,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 13..15,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                            data::type_::ValueShapeId(2),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                shape: data::type_::ValueShapeId(0),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 15..18,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(4),
                            data::type_::ValueShapeId(1),
                            data::type_::ValueShapeId(7),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 18..20,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(4),
                        ]),
                        return_: data::type_::ValueShapeId(4),
                        captures: data::Storage::Static(&[]),
                    },
                ]),
                parameters: data::Storage::Static(&[
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(0),
                        },
                    }),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(0),
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
                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(0),
                            shape_id: data::type_::CustomValueShapeId(0),
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
                    data::type_::ListStorageTypeId::Int(data::type_::IntListTypeId {
                        list_type: data::type_::ListTypeId(1),
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
                        constructor_count: 1,
                        constructors: data::Storage::Static(&[
                            data::type_::CustomConstructorDescriptor {
                                id: data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(0),
                                    index: 0,
                                },
                                name: data::Text::Static("Item"),
                                native_tag: data::Text::Static("item"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: None,
                                        type_: data::type_::ValueType::Int,
                                        shape: data::type_::ValueShapeId(0),
                                        refinement: data::type_::FieldRefinement::Value,
                                    },
                                ]),
                            },
                        ]),
                    },
                ]),
                definitions: data::Storage::Static(&[
                    data::type_::CustomDefinition {
                        package: data::Text::Static("example"),
                        module: data::Text::Static("example"),
                        name: data::Text::Static("Item"),
                        publicity: data::type_::CustomTypePublicity::Private,
                        opaque: false,
                        parameters: 0,
                        constructors: data::Storage::Static(&[
                            data::type_::ConstructorDefinition {
                                name: data::Text::Static("Item"),
                                fields: data::Storage::Static(&[
                                    data::type_::FieldDefinition {
                                        label: None,
                                        type_: data::type_::TypeMetadata::Int,
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
                    data::type_::ValueShapeDescriptor::Int,
                    data::type_::ValueShapeDescriptor::Bool,
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(0)),
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(2),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                    },
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(2)),
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(1)),
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(5)),
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                            data::type_::ValueShapeId(2),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                    },
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(0)),
                ]),
                shape_types: data::Storage::Static(&[
                    data::type_::ValueType::Int,
                    data::type_::ValueType::Bool,
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                            data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    }),
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Bool,
                            data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Bool),
                    }),
                    data::type_::ValueType::List(data::type_::ListTypeId(1)),
                ]),
                custom_shapes: data::Storage::Static(&[
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(0),
                        arguments: data::Storage::Static(&[]),
                        constructor: data::type_::CustomConstructorRefinement::Any,
                    },
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(0),
                        arguments: data::Storage::Static(&[]),
                        constructor: data::type_::CustomConstructorRefinement::Exact(0),
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
                name: data::Text::Static("integer"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::Bool,
                        data::type_::TypeMetadata::Bool,
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("boolean"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
                },
                slot: 0,
            },
        ]),
    },
    value_functions: data::Storage::Static(&[]),
    never_functions: data::Storage::Static(&[]),
    callables: data::Storage::Static(&[]),
}
