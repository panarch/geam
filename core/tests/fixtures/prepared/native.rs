data::HostedModuleArtifact {
    module: data::ModuleArtifact {
        format: 30,
        program: data::ProgramTables {
            root: data::source::module_id(0),
            modules: data::Storage::Static(&[
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("main"),
                    source_context: Some(data::source::SourceContext::from_static_block("src/main.gleam", r#"
pub type Tree(a) {
  Leaf(a)
  Branch(List(Tree(a)))
}

@external(erlang, "native", "equal_native")
fn equal_native(value: a, target: b) -> Bool

@external(erlang, "native", "fold")
fn fold(callback: fn(Int) -> Int, initial: Int) -> Int

@external(erlang, "native", "keep_bits")
fn keep_bits(value: BitArray) -> BitArray

@external(erlang, "native", "success")
fn success(value: a) -> Result(a, String)

@external(erlang, "native", "failure")
fn failure(callback: fn() -> a) -> Result(a, String)

fn rebuild(result: Result(a, String)) -> Result(a, String) {
  case result {
    Ok(value) -> Ok(value)
    Error(reason) -> Error(reason)
  }
}

pub fn generic_results() {
  let assert Ok(5) = rebuild(success(5))
  let assert Error("caught") = rebuild(failure(fn() { 5 }))
  let assert Error("caught") = rebuild(failure(fn() { panic }))
  let assert Error("caught") = rebuild(failure(fn() -> Int { panic }))
  True
}

pub fn run() {
  let assert True = integer_comparisons()
  let source = Branch([Leaf(<<"one":utf8>>), Branch([Leaf(<<"two":utf8>>)])])
  let expected = Branch([Leaf("one"), Branch([Leaf("two")])])
  #(
    equal_native(source, expected),
    equal_native(#(<<"one":utf8>>, [<<"two":utf8>>]), #("one", ["two"])),
    fold(
      fn(value) {
        case value >= 0 {
          True -> value + 1
          False -> value - 1
        }
      },
      40,
    ),
  )
}

pub fn list_callback(values: List(Int), initial: Int) -> Int {
  let calculate = fn(input) {
    let assert [head, ..tail] as original = values
    case tail == original {
      True -> input
      False -> head + input
    }
  }
  fold(calculate, initial)
}

pub fn substring(value: String) {
  let assert "prefix:" <> rest = value
  let read = fn() { rest }
  #(equal_native(#(rest, [rest]), #(read(), [read()])), read())
}

pub fn bit_range(value: BitArray, start: Int, size: Int) {
  case value {
    <<selected:bits-size(size), _:bits>> if start == 0 -> keep_bits(selected)
    <<_:bits-size(start), selected:bits-size(size), _:bits>> -> {
      let read = fn() { selected }
      keep_bits(read())
    }
    _ -> <<>>
  }
}

pub fn bit_tail(value: BitArray) {
  let assert <<_:8, rest:bits>> = value
  keep_bits(rest)
}

fn compare(left, right) {
  #(left == right, left != right)
}

fn integer_comparisons() {
  let minimum = -9_223_372_036_854_775_808
  let maximum = 9_223_372_036_854_775_807
  let assert 9_223_372_036_854_775_808 = maximum + 1
  let assert -9_223_372_036_854_775_809 = minimum - 1
  let assert 9_223_372_036_854_775_808 = -9_223_372_036_854_775_808 / -1
  let assert 0 = minimum % -1
  let assert -2 = -7 / 3
  let assert -1 = -7 % 3
  let assert 0 = minimum / 0
  let assert 0 = maximum % 0
  let assert 85_070_591_730_234_615_865_843_651_857_942_052_864 =
    minimum * minimum
  let assert True = -9_223_372_036_854_775_809 < minimum
  let assert True = maximum < 9_223_372_036_854_775_808
  let wide = 340_282_366_920_938_463_463_374_607_431_768_211_456
  let negative = -340_282_366_920_938_463_463_374_607_431_768_211_456
  let assert #(True, False) = compare(wide, wide)
  let assert #(False, True) = compare(negative, wide)
  let assert #(False, True) = compare("left", "right")
  let assert True = negative < wide
  let assert True = negative <= negative
  let assert False = wide < negative
  let assert False = wide <= negative
  let assert True = wide > negative
  let assert True = wide >= wide
  let assert False = negative > wide
  let assert False = negative >= wide
  let assert True = wide == 340_282_366_920_938_463_463_374_607_431_768_211_456
  True
}
"#)),
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
                    never_functions: data::Storage::Static(&[
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
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::SourceStop(data::graph::SourceStop {
                                                kind: data::graph::SourceStopKind::Panic,
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "<anonymous:1>", data::source::SourceSpan::new(825, 830)),
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[]),
                            },
                        })),
                    ]),
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
                                            shape: data::type_::ValueShapeId(28),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
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
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(5)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::IntList {
                                                            target: data::graph::IntListLocalId(0),
                                                            source: data::graph::IntListLocalId(0),
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
                                            function: data::function::IntFunctionId(2),
                                            site: data::source::HostCallSite::from_static("main", "list_callback", data::source::SourceSpan::new(1614, 1638)),
                                        },
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
                                                test: data::graph::BoolTest::GtEqInt {
                                                    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                    right: data::graph::IntegerOperand::Immediate(0),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
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
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..3,
                                            instructions: 1..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(1)),
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 2,
                            return_: data::graph::IntLocalId(0),
                            body: ::core::marker::PhantomData,
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
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(17),
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
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::SourceStop(data::graph::SourceStop {
                                                kind: data::graph::SourceStopKind::Panic,
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "<anonymous:2>", data::source::SourceSpan::new(896, 901)),
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[]),
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
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                                pattern: data::graph::MatchPattern::Alias {
                                                    pattern: data::Storage::Static(&data::graph::MatchPattern::List(data::graph::MatchPatternList {
                                                        elements: data::Storage::Static(&[
                                                            data::graph::MatchPattern::Bind(data::graph::MatchPatternBinding {
                                                                index: 0,
                                                            }),
                                                        ]),
                                                        tail: Some(data::graph::MatchPatternListTail::Bind(data::graph::MatchPatternBinding {
                                                            index: 1,
                                                        })),
                                                    })),
                                                    binding: data::graph::MatchPatternBinding {
                                                        index: 2,
                                                    },
                                                },
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Binding(1),
                                                        data::graph::MatchEdgeArgument::Binding(2),
                                                        data::graph::MatchEdgeArgument::Binding(0),
                                                    ]),
                                                    bindings: data::Storage::Static(&[
                                                        0,
                                                        1,
                                                        2,
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
                                                                    data::graph::TransferStep {
                                                                        source: 2,
                                                                        destination: 1,
                                                                    },
                                                                ]),
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
                                            params: 2..6,
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
                                                    target: data::graph::BlockId(2),
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
                                                                family: data::graph::StorageFamily::IntList,
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
                                            params: 6..7,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 7..9,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 9..10,
                                            instructions: 1..1,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                    local: data::graph::IntListLocalId(0),
                                                    type_id: data::type_::IntListTypeId {
                                                        list_type: data::type_::ListTypeId(0),
                                                    },
                                                }),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "<anonymous:4>", data::source::SourceSpan::new(1479, 1489)),
                                                pattern_span: data::source::SourceSpan::new(1490, 1516),
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(28),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(28),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(1),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(28),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                                local: data::graph::IntListLocalId(0),
                                                type_id: data::type_::IntListTypeId {
                                                    list_type: data::type_::ListTypeId(0),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(28),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(17),
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
                    bit_array_functions: data::Storage::Static(&[
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
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                pattern: data::graph::MatchPattern::BitArray(data::graph::BitArrayPattern {
                                                    segments: data::Storage::Static(&[
                                                        data::graph::BitArrayPatternSegment::Bits {
                                                            pattern: data::graph::BitArrayBindingPattern::Bind(data::graph::MatchPatternBinding {
                                                                index: 0,
                                                            }),
                                                            size: Some(data::graph::BitArrayPatternSize::Dynamic {
                                                                value: data::graph::BitArrayPatternSizeExpr::Local(data::graph::IntLocalId(1)),
                                                                unit: 1,
                                                            }),
                                                            unit: 1,
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
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Binding(0),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[
                                                        0,
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(7),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
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
                                            params: 3..7,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::TestBranch(data::graph::TestBranch {
                                                test: data::graph::BoolTest::EqualInt {
                                                    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                    right: data::graph::IntegerOperand::Immediate(0),
                                                },
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
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
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(3),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::BitArray,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 7..8,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 8..11,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(4),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
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
                                            params: 11..14,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                pattern: data::graph::MatchPattern::BitArray(data::graph::BitArrayPattern {
                                                    segments: data::Storage::Static(&[
                                                        data::graph::BitArrayPatternSegment::Bits {
                                                            pattern: data::graph::BitArrayBindingPattern::Discard,
                                                            size: Some(data::graph::BitArrayPatternSize::Dynamic {
                                                                value: data::graph::BitArrayPatternSizeExpr::Local(data::graph::IntLocalId(0)),
                                                                unit: 1,
                                                            }),
                                                            unit: 1,
                                                        },
                                                        data::graph::BitArrayPatternSegment::Bits {
                                                            pattern: data::graph::BitArrayBindingPattern::Bind(data::graph::MatchPatternBinding {
                                                                index: 0,
                                                            }),
                                                            size: Some(data::graph::BitArrayPatternSize::Dynamic {
                                                                value: data::graph::BitArrayPatternSizeExpr::Local(data::graph::IntLocalId(1)),
                                                                unit: 1,
                                                            }),
                                                            unit: 1,
                                                        },
                                                        data::graph::BitArrayPatternSegment::Bits {
                                                            pattern: data::graph::BitArrayBindingPattern::Discard,
                                                            size: None,
                                                            unit: 1,
                                                        },
                                                    ]),
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
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
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
                                            params: 14..15,
                                            instructions: 0..2,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 15..15,
                                            instructions: 2..3,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(2)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 15..18,
                                            instructions: 3..3,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(4),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BitArrayFunction {
                                                    local: data::graph::BitArrayFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(22),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                                                },
                                                family: data::function::FunctionReturnFamily::BitArray,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::BitArray(data::function::BitArrayFunctionId(3)),
                                                    captures: data::Storage::Static(&[
                                                        data::graph::FunctionCapture::BitArray {
                                                            target: data::graph::BitArrayLocalId(0),
                                                            source: data::graph::BitArrayLocalId(0),
                                                        },
                                                    ]),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::BitArray(data::graph::BitArrayInstruction::FunctionCall {
                                                function: data::graph::BitArrayFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("main", "bit_range", data::source::SourceSpan::new(2078, 2084)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::BitArray(data::graph::BitArrayInstruction::Value(data::Storage::Static(&[]))),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::BitArrayFunctionId(2),
                                            site: data::source::HostCallSite::from_static("main", "bit_range", data::source::SourceSpan::new(1941, 1960)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[]),
                                        },
                                    },
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::BitArrayFunctionId(2),
                                            site: data::source::HostCallSite::from_static("main", "bit_range", data::source::SourceSpan::new(2068, 2085)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[
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
                                                data::graph::FamilyTransfer {
                                                    family: data::graph::StorageFamily::BitArrayFunction,
                                                    length: 0,
                                                    steps: data::Storage::Static(&[]),
                                                },
                                            ]),
                                        },
                                    },
                                    data::function::FunctionExit::Return(data::graph::BitArrayLocalId(0)),
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
                                                subject: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                pattern: data::graph::MatchPattern::BitArray(data::graph::BitArrayPattern {
                                                    segments: data::Storage::Static(&[
                                                        data::graph::BitArrayPatternSegment::Int {
                                                            pattern: data::graph::BitArrayPatternValue::Discard,
                                                            size: data::graph::BitArrayPatternSize::Fixed(8),
                                                            endianness: data::graph::Endianness::Big,
                                                            signedness: data::graph::Signedness::Unsigned,
                                                        },
                                                        data::graph::BitArrayPatternSegment::Bits {
                                                            pattern: data::graph::BitArrayBindingPattern::Bind(data::graph::MatchPatternBinding {
                                                                index: 0,
                                                            }),
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
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 1..2,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..3,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "bit_tail", data::source::SourceSpan::new(2150, 2160)),
                                                pattern_span: data::source::SourceSpan::new(2161, 2179),
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: data::function::BitArrayFunctionId(2),
                                            site: data::source::HostCallSite::from_static("main", "bit_tail", data::source::SourceSpan::new(2190, 2205)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                        ]),
                                        transfer: data::graph::Transfer {
                                            families: data::Storage::Static(&[]),
                                        },
                                    },
                                ]),
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 4,
                            return_: data::graph::BitArrayLocalId(0),
                            body: ::core::marker::PhantomData,
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
                                            params: 0..1,
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::BitArrayLocalId(0)),
                                ]),
                            },
                        })),
                    ]),
                    utf_codepoint_functions: data::Storage::Static(&[]),
                    custom_functions: data::Storage::Static(&[
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 5,
                            return_: data::graph::CustomLocal {
                                id: data::graph::CustomLocalId(0),
                                shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(2),
                                    shape_id: data::type_::CustomValueShapeId(6),
                                },
                            },
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 1,
                            },
                            body: data::function::ProfiledCustomFunctionBody {
                                _signature_shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(2),
                                    shape_id: data::type_::CustomValueShapeId(6),
                                },
                                _body_shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(2),
                                    shape_id: data::type_::CustomValueShapeId(6),
                                },
                                body: data::function::ProfiledFunctionBody {
                                    block_graph: data::graph::ProfiledBlockGraph {
                                        entry: data::graph::BlockId(0),
                                        blocks: data::Storage::Static(&[
                                            data::graph::BlockHeader {
                                                params: 0..1,
                                                instructions: 0..0,
                                                terminator: data::graph::Terminator::Match(data::graph::Match {
                                                    subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(2),
                                                            shape_id: data::type_::CustomValueShapeId(6),
                                                        },
                                                    }),
                                                    pattern: data::graph::MatchPattern::Custom {
                                                        constructor: data::type_::CustomConstructorId {
                                                            type_id: data::type_::CustomTypeId(2),
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
                                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                                id: data::graph::CustomLocalId(0),
                                                                shape: data::type_::CustomValueShape {
                                                                    type_id: data::type_::CustomTypeId(2),
                                                                    shape_id: data::type_::CustomValueShapeId(6),
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
                                                params: 1..2,
                                                instructions: 0..1,
                                                terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                            },
                                            data::graph::BlockHeader {
                                                params: 2..3,
                                                instructions: 1..3,
                                                terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                            },
                                        ]),
                                        params: data::Storage::Static(&[
                                            data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(24),
                                            },
                                            data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(24),
                                            },
                                        ]),
                                        instructions: data::Storage::Static(&[
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(2),
                                                            shape_id: data::type_::CustomValueShapeId(6),
                                                        },
                                                    }),
                                                    shape: data::type_::ValueShapeId(24),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                    constructor: data::type_::CustomConstructorId {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        index: 0,
                                                    },
                                                    fields: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    ]),
                                                }),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    shape: data::type_::ValueShapeId(3),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::CustomField {
                                                    source: data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(2),
                                                            shape_id: data::type_::CustomValueShapeId(6),
                                                        },
                                                    },
                                                    index: 0,
                                                }),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(1),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(2),
                                                            shape_id: data::type_::CustomValueShapeId(6),
                                                        },
                                                    }),
                                                    shape: data::type_::ValueShapeId(24),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                    constructor: data::type_::CustomConstructorId {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        index: 1,
                                                    },
                                                    fields: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    ]),
                                                }),
                                            }),
                                        ]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::Return(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }),
                                        data::function::FunctionExit::Return(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }),
                                    ]),
                                },
                            },
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 6,
                            return_: data::graph::CustomLocal {
                                id: data::graph::CustomLocalId(0),
                                shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(2),
                                    shape_id: data::type_::CustomValueShapeId(6),
                                },
                            },
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                            index: 7,
                            return_: data::graph::CustomLocal {
                                id: data::graph::CustomLocalId(0),
                                shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(3),
                                    shape_id: data::type_::CustomValueShapeId(7),
                                },
                            },
                            body: ::core::marker::PhantomData,
                        })),
                        data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                            entry: data::function::FunctionEntry {
                                parameter_count: 1,
                            },
                            body: data::function::ProfiledCustomFunctionBody {
                                _signature_shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(3),
                                    shape_id: data::type_::CustomValueShapeId(7),
                                },
                                _body_shape: data::type_::CustomValueShape {
                                    type_id: data::type_::CustomTypeId(3),
                                    shape_id: data::type_::CustomValueShapeId(7),
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
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        shape_id: data::type_::CustomValueShapeId(7),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(27),
                                            },
                                        ]),
                                        instructions: data::Storage::Static(&[
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    shape: data::type_::ValueShapeId(3),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::CustomField {
                                                    source: data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(3),
                                                            shape_id: data::type_::CustomValueShapeId(7),
                                                        },
                                                    },
                                                    index: 0,
                                                }),
                                            }),
                                            data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                                output: data::graph::ParamSlot {
                                                    local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(1),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(3),
                                                            shape_id: data::type_::CustomValueShapeId(7),
                                                        },
                                                    }),
                                                    shape: data::type_::ValueShapeId(27),
                                                },
                                                kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Construct {
                                                    constructor: data::type_::CustomConstructorId {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        index: 1,
                                                    },
                                                    fields: data::Storage::Static(&[
                                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    ]),
                                                }),
                                            }),
                                        ]),
                                    },
                                    exits: data::Storage::Static(&[
                                        data::function::FunctionExit::Return(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(3),
                                                shape_id: data::type_::CustomValueShapeId(7),
                                            },
                                        }),
                                    ]),
                                },
                            },
                        })),
                    ]),
                    external_functions: data::Storage::Static(&[]),
                    bool_functions: data::Storage::Static(&[
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
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                }),
                                                pattern: data::graph::MatchPattern::Custom {
                                                    constructor: data::type_::CustomConstructorId {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        index: 0,
                                                    },
                                                    fields: data::Storage::Static(&[
                                                        data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                            sign: data::Sign::Plus,
                                                            digits: data::Storage::Static(&[
                                                                5,
                                                            ]),
                                                        }),
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
                                                    target: data::graph::BlockId(8),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                            id: data::graph::CustomLocalId(1),
                                                            shape: data::type_::CustomValueShape {
                                                                type_id: data::type_::CustomTypeId(2),
                                                                shape_id: data::type_::CustomValueShapeId(6),
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
                                                                family: data::graph::StorageFamily::Custom,
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
                                            params: 0..0,
                                            instructions: 3..6,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                }),
                                                pattern: data::graph::MatchPattern::Custom {
                                                    constructor: data::type_::CustomConstructorId {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        index: 1,
                                                    },
                                                    fields: data::Storage::Static(&[
                                                        data::graph::MatchPattern::String(data::Text::Static("caught")),
                                                    ]),
                                                },
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[]),
                                                    bindings: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Custom,
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
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(7),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                            id: data::graph::CustomLocalId(1),
                                                            shape: data::type_::CustomValueShape {
                                                                type_id: data::type_::CustomTypeId(2),
                                                                shape_id: data::type_::CustomValueShapeId(6),
                                                            },
                                                        }),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Custom,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 1,
                                                                        destination: 0,
                                                                    },
                                                                ]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
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
                                            instructions: 6..9,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        shape_id: data::type_::CustomValueShapeId(7),
                                                    },
                                                }),
                                                pattern: data::graph::MatchPattern::Custom {
                                                    constructor: data::type_::CustomConstructorId {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        index: 1,
                                                    },
                                                    fields: data::Storage::Static(&[
                                                        data::graph::MatchPattern::String(data::Text::Static("caught")),
                                                    ]),
                                                },
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(3),
                                                    args: data::Storage::Static(&[]),
                                                    bindings: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Custom,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::NeverFunction,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(6),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                            id: data::graph::CustomLocalId(1),
                                                            shape: data::type_::CustomValueShape {
                                                                type_id: data::type_::CustomTypeId(3),
                                                                shape_id: data::type_::CustomValueShapeId(7),
                                                            },
                                                        }),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Custom,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 1,
                                                                        destination: 0,
                                                                    },
                                                                ]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::NeverFunction,
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
                                            instructions: 9..12,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                }),
                                                pattern: data::graph::MatchPattern::Custom {
                                                    constructor: data::type_::CustomConstructorId {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        index: 1,
                                                    },
                                                    fields: data::Storage::Static(&[
                                                        data::graph::MatchPattern::String(data::Text::Static("caught")),
                                                    ]),
                                                },
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(4),
                                                    args: data::Storage::Static(&[]),
                                                    bindings: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Custom,
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
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(5),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                            id: data::graph::CustomLocalId(1),
                                                            shape: data::type_::CustomValueShape {
                                                                type_id: data::type_::CustomTypeId(2),
                                                                shape_id: data::type_::CustomValueShapeId(6),
                                                            },
                                                        }),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Custom,
                                                                length: 1,
                                                                steps: data::Storage::Static(&[
                                                                    data::graph::TransferStep {
                                                                        source: 1,
                                                                        destination: 0,
                                                                    },
                                                                ]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::IntFunction,
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
                                            instructions: 12..13,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 13..13,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                }),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "generic_results", data::source::SourceSpan::new(837, 847)),
                                                pattern_span: data::source::SourceSpan::new(848, 863),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 1..2,
                                            instructions: 13..13,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        shape_id: data::type_::CustomValueShapeId(7),
                                                    },
                                                }),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "generic_results", data::source::SourceSpan::new(773, 783)),
                                                pattern_span: data::source::SourceSpan::new(784, 799),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..3,
                                            instructions: 13..13,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                }),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "generic_results", data::source::SourceSpan::new(713, 723)),
                                                pattern_span: data::source::SourceSpan::new(724, 739),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 3..4,
                                            instructions: 13..13,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                }),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "generic_results", data::source::SourceSpan::new(672, 682)),
                                                pattern_span: data::source::SourceSpan::new(683, 688),
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(6),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(24),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(3),
                                                    shape_id: data::type_::CustomValueShapeId(7),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(27),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(6),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(24),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(6),
                                                },
                                            }),
                                            shape: data::type_::ValueShapeId(24),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(17),
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
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(24),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Call {
                                                function: data::function::CustomFunctionId {
                                                    index: 0,
                                                    return_shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(699, 709)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(24),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Call {
                                                function: data::function::CustomFunctionId {
                                                    index: 1,
                                                    return_shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(2),
                                                            shape_id: data::type_::CustomValueShapeId(6),
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(691, 710)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::IntFunction {
                                                    local: data::graph::IntFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(25),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                                family: data::function::FunctionReturnFamily::Int,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(3)),
                                                    captures: data::Storage::Static(&[]),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(24),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Call {
                                                function: data::function::CustomFunctionId {
                                                    index: 2,
                                                    return_shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::IntFunction {
                                                        local: data::graph::IntFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        },
                                                    },
                                                ]),
                                                site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(750, 769)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(24),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Call {
                                                function: data::function::CustomFunctionId {
                                                    index: 1,
                                                    return_shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(2),
                                                            shape_id: data::type_::CustomValueShapeId(6),
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(742, 770)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::NeverFunction(data::graph::NeverFunctionLocal {
                                                    id: data::graph::NeverFunctionLocalId(0),
                                                    type_: data::type_::GenericFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                        },
                                                        shape: data::type_::FunctionShape {
                                                            shape_id: data::type_::ValueShapeId(26),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                            },
                                                        },
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(26),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                },
                                                family: data::function::FunctionReturnFamily::Never,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Never(data::function::NeverFunctionId(0)),
                                                    captures: data::Storage::Static(&[]),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        shape_id: data::type_::CustomValueShapeId(7),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(27),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Call {
                                                function: data::function::CustomFunctionId {
                                                    index: 3,
                                                    return_shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        shape_id: data::type_::CustomValueShapeId(7),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::NeverFunction(data::graph::NeverFunctionLocal {
                                                        id: data::graph::NeverFunctionLocalId(0),
                                                        type_: data::type_::GenericFunctionType {
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                            },
                                                            shape: data::type_::FunctionShape {
                                                                shape_id: data::type_::ValueShapeId(26),
                                                                type_: data::type_::FunctionType {
                                                                    arguments: data::Storage::Static(&[]),
                                                                    return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                                                },
                                                            },
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(810, 833)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        shape_id: data::type_::CustomValueShapeId(7),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(27),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Call {
                                                function: data::function::CustomFunctionId {
                                                    index: 4,
                                                    return_shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(3),
                                                        shape_id: data::type_::CustomValueShapeId(7),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(3),
                                                            shape_id: data::type_::CustomValueShapeId(7),
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(802, 834)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::IntFunction {
                                                    local: data::graph::IntFunctionLocalId(0),
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    },
                                                },
                                                shape: data::type_::ValueShapeId(25),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                                family: data::function::FunctionReturnFamily::Int,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(4)),
                                                    captures: data::Storage::Static(&[]),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(24),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Call {
                                                function: data::function::CustomFunctionId {
                                                    index: 2,
                                                    return_shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::IntFunction {
                                                        local: data::graph::IntFunctionLocalId(0),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        },
                                                    },
                                                ]),
                                                site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(874, 904)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(1),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(24),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Custom(data::graph::CustomInstruction::Call {
                                                function: data::function::CustomFunctionId {
                                                    index: 1,
                                                    return_shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(2),
                                                        shape_id: data::type_::CustomValueShapeId(6),
                                                    },
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                        id: data::graph::CustomLocalId(0),
                                                        shape: data::type_::CustomValueShape {
                                                            type_id: data::type_::CustomTypeId(2),
                                                            shape_id: data::type_::CustomValueShapeId(6),
                                                        },
                                                    }),
                                                ]),
                                                site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(866, 905)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
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
                                parameter_count: 0,
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..0,
                                            instructions: 0..3,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                pattern: data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                    sign: data::Sign::Plus,
                                                    digits: data::Storage::Static(&[
                                                        0,
                                                        2147483648,
                                                    ]),
                                                }),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
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
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(46),
                                                    args: data::Storage::Static(&[
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
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 0..2,
                                            instructions: 3..4,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                pattern: data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                    sign: data::Sign::Minus,
                                                    digits: data::Storage::Static(&[
                                                        1,
                                                        2147483648,
                                                    ]),
                                                }),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
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
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(45),
                                                    args: data::Storage::Static(&[
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
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..4,
                                            instructions: 4..5,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                pattern: data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                    sign: data::Sign::Plus,
                                                    digits: data::Storage::Static(&[
                                                        0,
                                                        2147483648,
                                                    ]),
                                                }),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(3),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
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
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(44),
                                                    args: data::Storage::Static(&[
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
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 4..6,
                                            instructions: 5..6,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                pattern: data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                    sign: data::Sign::NoSign,
                                                    digits: data::Storage::Static(&[]),
                                                }),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(4),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
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
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(43),
                                                    args: data::Storage::Static(&[
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
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 6..8,
                                            instructions: 6..7,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                pattern: data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                    sign: data::Sign::Minus,
                                                    digits: data::Storage::Static(&[
                                                        2,
                                                    ]),
                                                }),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(5),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
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
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(42),
                                                    args: data::Storage::Static(&[
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
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 8..10,
                                            instructions: 7..8,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                pattern: data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                    sign: data::Sign::Minus,
                                                    digits: data::Storage::Static(&[
                                                        1,
                                                    ]),
                                                }),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(6),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
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
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(41),
                                                    args: data::Storage::Static(&[
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
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 10..12,
                                            instructions: 8..9,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                pattern: data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                    sign: data::Sign::NoSign,
                                                    digits: data::Storage::Static(&[]),
                                                }),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(7),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
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
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(40),
                                                    args: data::Storage::Static(&[
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
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 12..14,
                                            instructions: 9..10,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                pattern: data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                    sign: data::Sign::NoSign,
                                                    digits: data::Storage::Static(&[]),
                                                }),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(8),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
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
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(39),
                                                    args: data::Storage::Static(&[
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
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 14..16,
                                            instructions: 10..11,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                pattern: data::graph::MatchPattern::Int(data::graph::IntegerLiteral {
                                                    sign: data::Sign::Plus,
                                                    digits: data::Storage::Static(&[
                                                        0,
                                                        0,
                                                        0,
                                                        1073741824,
                                                    ]),
                                                }),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(9),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
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
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(38),
                                                    args: data::Storage::Static(&[
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
                                                        ]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 16..18,
                                            instructions: 11..13,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                pattern: data::graph::MatchPattern::Bool(true),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(10),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
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
                                                        ]),
                                                    },
                                                },
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(37),
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
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 18..19,
                                            instructions: 13..15,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                pattern: data::graph::MatchPattern::Bool(true),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(11),
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
                                                                family: data::graph::StorageFamily::Bool,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(36),
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
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 19..19,
                                            instructions: 15..18,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::Bool,
                                                    ]),
                                                },
                                                pattern: data::graph::MatchPattern::Tuple(data::Storage::Static(&[
                                                    data::graph::MatchPattern::Bool(true),
                                                    data::graph::MatchPattern::Bool(false),
                                                ])),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(12),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Tuple,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(35),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Tuple {
                                                            local: data::graph::TupleLocalId(0),
                                                            type_: data::Storage::Static(&[
                                                                data::type_::ValueType::Bool,
                                                                data::type_::ValueType::Bool,
                                                            ]),
                                                        },
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
                                            params: 19..21,
                                            instructions: 18..19,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::Bool,
                                                    ]),
                                                },
                                                pattern: data::graph::MatchPattern::Tuple(data::Storage::Static(&[
                                                    data::graph::MatchPattern::Bool(false),
                                                    data::graph::MatchPattern::Bool(true),
                                                ])),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(13),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Tuple,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(34),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Tuple {
                                                            local: data::graph::TupleLocalId(0),
                                                            type_: data::Storage::Static(&[
                                                                data::type_::ValueType::Bool,
                                                                data::type_::ValueType::Bool,
                                                            ]),
                                                        },
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
                                            params: 21..23,
                                            instructions: 19..22,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::Bool,
                                                    ]),
                                                },
                                                pattern: data::graph::MatchPattern::Tuple(data::Storage::Static(&[
                                                    data::graph::MatchPattern::Bool(false),
                                                    data::graph::MatchPattern::Bool(true),
                                                ])),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(14),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::String,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Tuple,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(33),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Tuple {
                                                            local: data::graph::TupleLocalId(0),
                                                            type_: data::Storage::Static(&[
                                                                data::type_::ValueType::Bool,
                                                                data::type_::ValueType::Bool,
                                                            ]),
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
                                            params: 23..25,
                                            instructions: 22..23,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                pattern: data::graph::MatchPattern::Bool(true),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(15),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
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
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(32),
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
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 25..27,
                                            instructions: 23..24,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                pattern: data::graph::MatchPattern::Bool(true),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(16),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
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
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(31),
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
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 27..29,
                                            instructions: 24..25,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                pattern: data::graph::MatchPattern::Bool(false),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(17),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
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
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(30),
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
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 29..31,
                                            instructions: 25..26,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                pattern: data::graph::MatchPattern::Bool(false),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(18),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
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
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(29),
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
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 31..33,
                                            instructions: 26..27,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                pattern: data::graph::MatchPattern::Bool(true),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(19),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
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
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(28),
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
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 33..35,
                                            instructions: 27..28,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                pattern: data::graph::MatchPattern::Bool(true),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(20),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
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
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(27),
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
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 35..37,
                                            instructions: 28..29,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                pattern: data::graph::MatchPattern::Bool(false),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(21),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(1))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
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
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(26),
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
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 37..39,
                                            instructions: 29..30,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                pattern: data::graph::MatchPattern::Bool(false),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(22),
                                                    args: data::Storage::Static(&[
                                                        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                                                    ]),
                                                    bindings: data::Storage::Static(&[]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Int,
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
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(25),
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
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 39..40,
                                            instructions: 30..32,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                pattern: data::graph::MatchPattern::Bool(true),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(23),
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
                                                                family: data::graph::StorageFamily::Bool,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(24),
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
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 40..40,
                                            instructions: 32..33,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 40..41,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(3495, 3505)),
                                                pattern_span: data::source::SourceSpan::new(3506, 3510),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 41..42,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(3457, 3467)),
                                                pattern_span: data::source::SourceSpan::new(3468, 3473),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 42..43,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(3420, 3430)),
                                                pattern_span: data::source::SourceSpan::new(3431, 3436),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 43..44,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(3387, 3397)),
                                                pattern_span: data::source::SourceSpan::new(3398, 3402),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 44..45,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(3351, 3361)),
                                                pattern_span: data::source::SourceSpan::new(3362, 3366),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 45..46,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(3313, 3323)),
                                                pattern_span: data::source::SourceSpan::new(3324, 3329),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 46..47,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(3276, 3286)),
                                                pattern_span: data::source::SourceSpan::new(3287, 3292),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 47..48,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(3235, 3245)),
                                                pattern_span: data::source::SourceSpan::new(3246, 3250),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 48..49,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(3199, 3209)),
                                                pattern_span: data::source::SourceSpan::new(3210, 3214),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 49..50,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::Bool,
                                                    ]),
                                                },
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(3144, 3154)),
                                                pattern_span: data::source::SourceSpan::new(3155, 3169),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 50..51,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::Bool,
                                                    ]),
                                                },
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(3090, 3100)),
                                                pattern_span: data::source::SourceSpan::new(3101, 3115),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 51..52,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::Bool,
                                                    ]),
                                                },
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(3040, 3050)),
                                                pattern_span: data::source::SourceSpan::new(3051, 3065),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 52..53,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2849, 2859)),
                                                pattern_span: data::source::SourceSpan::new(2860, 2864),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 53..54,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2792, 2802)),
                                                pattern_span: data::source::SourceSpan::new(2803, 2807),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 54..55,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2704, 2714)),
                                                pattern_span: data::source::SourceSpan::new(2715, 2765),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 55..56,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2675, 2685)),
                                                pattern_span: data::source::SourceSpan::new(2686, 2687),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 56..57,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2646, 2656)),
                                                pattern_span: data::source::SourceSpan::new(2657, 2658),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 57..58,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2621, 2631)),
                                                pattern_span: data::source::SourceSpan::new(2632, 2634),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 58..59,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2596, 2606)),
                                                pattern_span: data::source::SourceSpan::new(2607, 2609),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 59..60,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2566, 2576)),
                                                pattern_span: data::source::SourceSpan::new(2577, 2578),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 60..61,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2493, 2503)),
                                                pattern_span: data::source::SourceSpan::new(2504, 2529),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 61..62,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2439, 2449)),
                                                pattern_span: data::source::SourceSpan::new(2450, 2476),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 62..63,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2386, 2396)),
                                                pattern_span: data::source::SourceSpan::new(2397, 2422),
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Bool,
                                                    data::type_::ValueType::Bool,
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(29),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Bool,
                                                    data::type_::ValueType::Bool,
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(29),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Bool,
                                                    data::type_::ValueType::Bool,
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(29),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Minus,
                                                digits: data::Storage::Static(&[
                                                    0,
                                                    2147483648,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    4294967295,
                                                    2147483647,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {
                                                left: data::graph::IntegerOperand::Immediate(9223372036854775807),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Sub {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(1),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Div {
                                                left: data::graph::IntegerOperand::Immediate(-9223372036854775808),
                                                right: data::graph::IntegerOperand::Immediate(-1),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Remainder {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(-1),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Div {
                                                left: data::graph::IntegerOperand::Immediate(-7),
                                                right: data::graph::IntegerOperand::Immediate(3),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Remainder {
                                                left: data::graph::IntegerOperand::Immediate(-7),
                                                right: data::graph::IntegerOperand::Immediate(3),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Div {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Immediate(0),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Remainder {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                                right: data::graph::IntegerOperand::Immediate(0),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Mult {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Minus,
                                                digits: data::Storage::Static(&[
                                                    1,
                                                    2147483648,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::LtInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    0,
                                                    2147483648,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::LtInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    0,
                                                    0,
                                                    0,
                                                    0,
                                                    1,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Minus,
                                                digits: data::Storage::Static(&[
                                                    0,
                                                    0,
                                                    0,
                                                    0,
                                                    1,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::Bool,
                                                    ]),
                                                },
                                                shape: data::type_::ValueShapeId(29),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Call {
                                                function: data::function::TupleFunctionId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(3068, 3087)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::Bool,
                                                    ]),
                                                },
                                                shape: data::type_::ValueShapeId(29),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Call {
                                                function: data::function::TupleFunctionId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(3118, 3141)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("left"))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("right"))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::Bool,
                                                    ]),
                                                },
                                                shape: data::type_::ValueShapeId(29),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Call {
                                                function: data::function::TupleFunctionId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(3172, 3196)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::LtInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::LtEqInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::LtInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::LtEqInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::GtInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::GtEqInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::GtInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::GtEqInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    0,
                                                    0,
                                                    0,
                                                    0,
                                                    1,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::EqualInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
                                ]),
                            },
                        })),
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
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Match(data::graph::Match {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                pattern: data::graph::MatchPattern::Bool(true),
                                                success: data::graph::MatchEdge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[]),
                                                    bindings: data::Storage::Static(&[]),
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
                                                failure: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                    ]),
                                                    transfer: data::graph::Transfer {
                                                        families: data::Storage::Static(&[]),
                                                    },
                                                },
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 0..0,
                                            instructions: 1..35,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 35..35,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "run", data::source::SourceSpan::new(933, 943)),
                                                pattern_span: data::source::SourceSpan::new(944, 948),
                                            }),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(6),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                                function: data::function::BoolFunctionId(1),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("main", "run", data::source::SourceSpan::new(951, 972)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("one"))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::BitArray(data::graph::BitArrayInstruction::Value(data::Storage::Static(&[
                                                data::graph::BitArraySegment::String {
                                                    value: data::graph::StringLocalId(0),
                                                    encoding: data::graph::StringEncoding::Utf8,
                                                    site: data::source::PanicSite::from_static("main", "run", data::source::SourceSpan::new(1003, 1013)),
                                                },
                                            ]))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(0),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(2),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(7),
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
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("two"))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::BitArray(data::graph::BitArrayInstruction::Value(data::Storage::Static(&[
                                                data::graph::BitArraySegment::String {
                                                    value: data::graph::StringLocalId(1),
                                                    encoding: data::graph::StringEncoding::Utf8,
                                                    site: data::source::PanicSite::from_static("main", "run", data::source::SourceSpan::new(1033, 1043)),
                                                },
                                            ]))),
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
                                                shape: data::type_::ValueShapeId(7),
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
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                    local: data::graph::CustomListLocalId(0),
                                                    type_id: data::type_::CustomListTypeId {
                                                        list_type: data::type_::ListTypeId(1),
                                                        item_type: data::type_::CustomTypeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(8),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(1),
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
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(2),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(3),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(9),
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
                                                            list_type: data::type_::ListTypeId(1),
                                                            item_type: data::type_::CustomTypeId(0),
                                                        },
                                                    }),
                                                ]),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                    local: data::graph::CustomListLocalId(1),
                                                    type_id: data::type_::CustomListTypeId {
                                                        list_type: data::type_::ListTypeId(1),
                                                        item_type: data::type_::CustomTypeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(1),
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
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(3),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(0),
                                                        shape_id: data::type_::CustomValueShapeId(3),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(9),
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
                                                            list_type: data::type_::ListTypeId(1),
                                                            item_type: data::type_::CustomTypeId(0),
                                                        },
                                                    }),
                                                ]),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("one"))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(4),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(4),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(10),
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
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(3)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("two"))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(5),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(4),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(10),
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
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                    local: data::graph::CustomListLocalId(2),
                                                    type_id: data::type_::CustomListTypeId {
                                                        list_type: data::type_::ListTypeId(2),
                                                        item_type: data::type_::CustomTypeId(1),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(11),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(2),
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
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(6),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(5),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(12),
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
                                                            list_type: data::type_::ListTypeId(2),
                                                            item_type: data::type_::CustomTypeId(1),
                                                        },
                                                    }),
                                                ]),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::Custom {
                                                    local: data::graph::CustomListLocalId(3),
                                                    type_id: data::type_::CustomListTypeId {
                                                        list_type: data::type_::ListTypeId(2),
                                                        item_type: data::type_::CustomTypeId(1),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(5),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
                                                list_type: data::type_::ListTypeId(2),
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
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                    id: data::graph::CustomLocalId(7),
                                                    shape: data::type_::CustomValueShape {
                                                        type_id: data::type_::CustomTypeId(1),
                                                        shape_id: data::type_::CustomValueShapeId(5),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(12),
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
                                                            list_type: data::type_::ListTypeId(2),
                                                            item_type: data::type_::CustomTypeId(1),
                                                        },
                                                    }),
                                                ]),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                                function: data::function::BoolFunctionId(2),
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
                                                site: data::source::HostCallSite::from_static("main", "run", data::source::SourceSpan::new(1122, 1152)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(4)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("one"))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(2)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::BitArray(data::graph::BitArrayInstruction::Value(data::Storage::Static(&[
                                                data::graph::BitArraySegment::String {
                                                    value: data::graph::StringLocalId(4),
                                                    encoding: data::graph::StringEncoding::Utf8,
                                                    site: data::source::PanicSite::from_static("main", "run", data::source::SourceSpan::new(1175, 1185)),
                                                },
                                            ]))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(5)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("two"))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(3)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::BitArray(data::graph::BitArrayInstruction::Value(data::Storage::Static(&[
                                                data::graph::BitArraySegment::String {
                                                    value: data::graph::StringLocalId(5),
                                                    encoding: data::graph::StringEncoding::Utf8,
                                                    site: data::source::PanicSite::from_static("main", "run", data::source::SourceSpan::new(1192, 1202)),
                                                },
                                            ]))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::BitArray {
                                                    local: data::graph::BitArrayListLocalId(0),
                                                    type_id: data::type_::BitArrayListTypeId {
                                                        list_type: data::type_::ListTypeId(3),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(13),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::BitArray(data::type_::BitArrayListTypeId {
                                                list_type: data::type_::ListTypeId(3),
                                            }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                                data::graph::BitArrayLocalId(3),
                                            ])))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::BitArray,
                                                        data::type_::ValueType::List(data::type_::ListTypeId(3)),
                                                    ]),
                                                },
                                                shape: data::type_::ValueShapeId(14),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                                data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(2)),
                                                data::graph::ParamLocal::List(data::graph::ListLocal::BitArray {
                                                    local: data::graph::BitArrayListLocalId(0),
                                                    type_id: data::type_::BitArrayListTypeId {
                                                        list_type: data::type_::ListTypeId(3),
                                                    },
                                                }),
                                            ]))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(6)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("one"))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(7)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("two"))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                    local: data::graph::StringListLocalId(0),
                                                    type_id: data::type_::StringListTypeId {
                                                        list_type: data::type_::ListTypeId(4),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(15),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::String(data::type_::StringListTypeId {
                                                list_type: data::type_::ListTypeId(4),
                                            }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                                data::graph::StringLocalId(7),
                                            ])))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(1),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                        data::type_::ValueType::List(data::type_::ListTypeId(4)),
                                                    ]),
                                                },
                                                shape: data::type_::ValueShapeId(16),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(6)),
                                                data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                    local: data::graph::StringListLocalId(0),
                                                    type_id: data::type_::StringListTypeId {
                                                        list_type: data::type_::ListTypeId(4),
                                                    },
                                                }),
                                            ]))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                                function: data::function::BoolFunctionId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Tuple {
                                                        local: data::graph::TupleLocalId(0),
                                                        type_: data::Storage::Static(&[
                                                            data::type_::ValueType::BitArray,
                                                            data::type_::ValueType::List(data::type_::ListTypeId(3)),
                                                        ]),
                                                    },
                                                    data::graph::ParamLocal::Tuple {
                                                        local: data::graph::TupleLocalId(1),
                                                        type_: data::Storage::Static(&[
                                                            data::type_::ValueType::String,
                                                            data::type_::ValueType::List(data::type_::ListTypeId(4)),
                                                        ]),
                                                    },
                                                ]),
                                                site: data::source::HostCallSite::from_static("main", "run", data::source::SourceSpan::new(1158, 1226)),
                                            }),
                                        }),
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
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(1)),
                                                    captures: data::Storage::Static(&[]),
                                                },
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(17),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Call {
                                                function: data::function::IntFunctionId(2),
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
                                                site: data::source::HostCallSite::from_static("main", "run", data::source::SourceSpan::new(1232, 1373)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                        }),
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
                                                site: data::source::PanicSite::from_static("main", "substring", data::source::SourceSpan::new(1678, 1688)),
                                                pattern_span: data::source::SourceSpan::new(1689, 1706),
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
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                    local: data::graph::StringListLocalId(0),
                                                    type_id: data::type_::StringListTypeId {
                                                        list_type: data::type_::ListTypeId(4),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(15),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::String(data::type_::StringListTypeId {
                                                list_type: data::type_::ListTypeId(4),
                                            }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                                data::graph::StringLocalId(0),
                                            ])))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                        data::type_::ValueType::List(data::type_::ListTypeId(4)),
                                                    ]),
                                                },
                                                shape: data::type_::ValueShapeId(16),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                    local: data::graph::StringListLocalId(0),
                                                    type_id: data::type_::StringListTypeId {
                                                        list_type: data::type_::ListTypeId(4),
                                                    },
                                                }),
                                            ]))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::FunctionCall {
                                                function: data::graph::StringFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("main", "substring", data::source::SourceSpan::new(1778, 1784)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(2)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::FunctionCall {
                                                function: data::graph::StringFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("main", "substring", data::source::SourceSpan::new(1787, 1793)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                    local: data::graph::StringListLocalId(1),
                                                    type_id: data::type_::StringListTypeId {
                                                        list_type: data::type_::ListTypeId(4),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(15),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::List(data::graph::ListInstruction::String(data::type_::StringListTypeId {
                                                list_type: data::type_::ListTypeId(4),
                                            }, data::graph::TypedListInstruction::Value(data::Storage::Static(&[
                                                data::graph::StringLocalId(2),
                                            ])))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(1),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::String,
                                                        data::type_::ValueType::List(data::type_::ListTypeId(4)),
                                                    ]),
                                                },
                                                shape: data::type_::ValueShapeId(16),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                                data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                data::graph::ParamLocal::List(data::graph::ListLocal::String {
                                                    local: data::graph::StringListLocalId(1),
                                                    type_id: data::type_::StringListTypeId {
                                                        list_type: data::type_::ListTypeId(4),
                                                    },
                                                }),
                                            ]))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                                function: data::function::BoolFunctionId(4),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Tuple {
                                                        local: data::graph::TupleLocalId(0),
                                                        type_: data::Storage::Static(&[
                                                            data::type_::ValueType::String,
                                                            data::type_::ValueType::List(data::type_::ListTypeId(4)),
                                                        ]),
                                                    },
                                                    data::graph::ParamLocal::Tuple {
                                                        local: data::graph::TupleLocalId(1),
                                                        type_: data::Storage::Static(&[
                                                            data::type_::ValueType::String,
                                                            data::type_::ValueType::List(data::type_::ListTypeId(4)),
                                                        ]),
                                                    },
                                                ]),
                                                site: data::source::HostCallSite::from_static("main", "substring", data::source::SourceSpan::new(1746, 1796)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(3)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::FunctionCall {
                                                function: data::graph::StringFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("main", "substring", data::source::SourceSpan::new(1798, 1804)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::TupleLocalId(2)),
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
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                            shape: data::type_::ValueShapeId(17),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::EqualInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::NotEqualInt {
                                                left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(0)),
                                                right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(1)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::Bool,
                                                    ]),
                                                },
                                                shape: data::type_::ValueShapeId(29),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(3),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Equal {
                                                left: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                right: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                shape: data::type_::ValueShapeId(6),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::NotEqual {
                                                left: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                right: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Tuple {
                                                    local: data::graph::TupleLocalId(0),
                                                    type_: data::Storage::Static(&[
                                                        data::type_::ValueType::Bool,
                                                        data::type_::ValueType::Bool,
                                                    ]),
                                                },
                                                shape: data::type_::ValueShapeId(29),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Value(data::Storage::Static(&[
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                const CALL_GROUP_0: [data::compiled::calls::CallStart; 3] = {
                    use data::compiled::calls::{CallCapture, CallExecution, CallInputs, CallInteger, CallOps, CallOutput, CallProgress, CallStorage, CallValues, IntCallable};
                    use data::compiled::int_list::IntList;
                    enum FunctionState {
                        Int0Point0 { int_list0: IntList, int0: i128 },
                        Int0Point1 { int_list0: IntList, int0: i128, int_function0: IntCallable },
                        Int1Point0 { int0: i128 },
                        Int1Point1 { int0: i128 },
                        Int1Point2 { int0: i128, int1: i128 },
                        Int1Point3 { int0: i128 },
                        Int1Point4 { int0: i128, int1: i128 },
                        Int3Point0 {  },
                        Int3Point1 { int0: i128 },
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
                    #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                    enum FunctionStep {
                        Yield(FunctionState),
                        Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                        Int { value: i128 },
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
                                data::compiled::CallTarget::Int(data::function::IntFunctionId(0)) => calls_int_0_state(point, values),
                                data::compiled::CallTarget::Int(data::function::IntFunctionId(1)) => calls_int_1_state(point, values),
                                data::compiled::CallTarget::Int(data::function::IntFunctionId(3)) => calls_int_3_state(point, values),
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
                            FunctionState::Int0Point0 { int_list0, int0 } => calls_int_0_run(Int0State::Point0 { int_list0, int0 }, ops, budget),
                            FunctionState::Int0Point1 { int_list0, int0, int_function0 } => calls_int_0_run(Int0State::Point1 { int_list0, int0, int_function0 }, ops, budget),
                            FunctionState::Int1Point0 { int0 } => calls_int_1_run(Int1State::Point0 { int0 }, ops, budget),
                            FunctionState::Int1Point1 { int0 } => calls_int_1_run(Int1State::Point1 { int0 }, ops, budget),
                            FunctionState::Int1Point2 { int0, int1 } => calls_int_1_run(Int1State::Point2 { int0, int1 }, ops, budget),
                            FunctionState::Int1Point3 { int0 } => calls_int_1_run(Int1State::Point3 { int0 }, ops, budget),
                            FunctionState::Int1Point4 { int0, int1 } => calls_int_1_run(Int1State::Point4 { int0, int1 }, ops, budget),
                            FunctionState::Int3Point0 {  } => calls_int_3_run(Int3State::Point0 {  }, ops, budget),
                            FunctionState::Int3Point1 { int0 } => calls_int_3_run(Int3State::Point1 { int0 }, ops, budget),
                        }
                    }
                    enum Int0State {
                        Point0 { int_list0: IntList, int0: i128 },
                        Point1 { int_list0: IntList, int0: i128, int_function0: IntCallable },
                    }
                    fn calls_int_0_run(active: Int0State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            Int0State::Point0 { int_list0, int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point0 { int_list0, int0 }); }
                                *budget -= 1;
                                let int_function0 = ops.int_closure(data::function::IntFunctionId(5), data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                }, vec![CallCapture::int_list(data::graph::IntListLocalId(0), int_list0.clone())]);
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point1 { int_list0, int0, int_function0 }); }
                                {
                                    FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
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
                                    }, values: Box::new(CallValues { ints: vec![int0.into()], int_lists: vec![int_list0], int_functions: vec![int_function0], ..CallValues::default() }) }
                                }
                            },
                            Int0State::Point1 { int_list0, int0, int_function0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point1 { int_list0, int0, int_function0 }); }
                                {
                                    FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
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
                                    }, values: Box::new(CallValues { ints: vec![int0.into()], int_lists: vec![int_list0], int_functions: vec![int_function0], ..CallValues::default() }) }
                                }
                            },
                        }
                    }
                    enum Int1State {
                        Point0 { int0: i128 },
                        Point1 { int0: i128 },
                        Point2 { int0: i128, int1: i128 },
                        Point3 { int0: i128 },
                        Point4 { int0: i128, int1: i128 },
                    }
                    fn calls_int_1_run(active: Int1State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            Int1State::Point0 { int0 } => {
                                {
                                    let values = ops.numeric();
                                    let progress = numeric_int_1_entry((int0,), values, budget);
                                    calls_int_1_numeric(progress, values)
                                }
                            },
                            Int1State::Point1 { int0 } => {
                                {
                                    let values = ops.numeric();
                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[int0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    let progress = numeric_int_1(1, values, budget);
                                    calls_int_1_numeric(progress, values)
                                }
                            },
                            Int1State::Point2 { int0, int1 } => {
                                {
                                    let values = ops.numeric();
                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[int0, int1]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    let progress = numeric_int_1(2, values, budget);
                                    calls_int_1_numeric(progress, values)
                                }
                            },
                            Int1State::Point3 { int0 } => {
                                {
                                    let values = ops.numeric();
                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[int0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    let progress = numeric_int_1(3, values, budget);
                                    calls_int_1_numeric(progress, values)
                                }
                            },
                            Int1State::Point4 { int0, int1 } => {
                                {
                                    let values = ops.numeric();
                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[int0, int1]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    let progress = numeric_int_1(4, values, budget);
                                    calls_int_1_numeric(progress, values)
                                }
                            },
                        }
                    }
                    enum Int3State {
                        Point0 {  },
                        Point1 { int0: i128 },
                    }
                    fn calls_int_3_run(active: Int3State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            Int3State::Point0 {  } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point0 {  }); }
                                *budget -= 1;
                                let int0 = 5_i128;
                                if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point: data::compiled::CompiledCheckpoint {
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
                                }, values: Box::new(CallValues { ints: vec![int0.into()], ..CallValues::default() }) }; }
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point1 { int0 }); }
                                *budget -= 1;
                                {
                                    FunctionStep::Int { value: int0 }
                                }
                            },
                            Int3State::Point1 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int3Point1 { int0 }); }
                                *budget -= 1;
                                {
                                    FunctionStep::Int { value: int0 }
                                }
                            },
                        }
                    }
                    fn calls_int_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                        let active = match point {
                            0 => FunctionState::Int0Point0 { int_list0: values.int_list(0)?, int0: values.int(0)? },
                            1 => FunctionState::Int0Point1 { int_list0: values.int_list(0)?, int0: values.int(0)?, int_function0: values.int_function(0)? },
                            _ => return None,
                        };
                        Some(active)
                    }
                    fn calls_int_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                        if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point, values) { return Some(execution); }
                        let active = calls_int_0_state(point, values)?;
                        Some(Box::new(FunctionExecution::new(active)))
                    }
                    fn calls_int_1_numeric(progress: data::compiled::CompiledProgress, values: &data::compiled::numeric::NumericValues) -> FunctionStep {
                        const STATES: [fn(&data::compiled::numeric::NumericValues) -> FunctionState; 5] = [
                            |values| FunctionState::Int1Point0 { int0: values.ints[0] },
                            |values| FunctionState::Int1Point1 { int0: values.ints[0] },
                            |values| FunctionState::Int1Point2 { int0: values.ints[0], int1: values.ints[1] },
                            |values| FunctionState::Int1Point3 { int0: values.ints[0] },
                            |values| FunctionState::Int1Point4 { int0: values.ints[0], int1: values.ints[1] },
                        ];
                        const RETURNS: [fn(&data::compiled::numeric::NumericValues) -> FunctionStep; 2] = [
                            |values| FunctionStep::Int { value: values.ints[1] },
                            |values| FunctionStep::Int { value: values.ints[1] },
                        ];
                        match progress {
                            data::compiled::CompiledProgress::Yield(point) => FunctionStep::Yield(STATES[point](values)),
                            data::compiled::CompiledProgress::Interpreted(point) => {
                                const POINTS: [data::compiled::CompiledCheckpoint; 5] = [
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
                                ];
                                FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(1)), point: POINTS[point], values: Box::new(CallValues { ints: values.ints.iter().copied().map(Into::into).collect(), bools: values.bools.clone(), ..CallValues::default() }) }
                            },
                            data::compiled::CompiledProgress::Complete(exit) => RETURNS[exit.0](values),
                        }
                    }
                    fn calls_int_1_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                        let active = match point {
                            0 => FunctionState::Int1Point0 { int0: values.int(0)? },
                            1 => FunctionState::Int1Point1 { int0: values.int(0)? },
                            2 => FunctionState::Int1Point2 { int0: values.int(0)?, int1: values.int(1)? },
                            3 => FunctionState::Int1Point3 { int0: values.int(0)? },
                            4 => FunctionState::Int1Point4 { int0: values.int(0)?, int1: values.int(1)? },
                            _ => return None,
                        };
                        Some(active)
                    }
                    fn calls_int_1_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                        if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(1)), point, values) { return Some(execution); }
                        let active = calls_int_1_state(point, values)?;
                        Some(Box::new(FunctionExecution::new(active)))
                    }
                    fn calls_int_3_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                        let active = match point {
                            0 => FunctionState::Int3Point0 {  },
                            1 => FunctionState::Int3Point1 { int0: values.int(0)? },
                            _ => return None,
                        };
                        Some(active)
                    }
                    fn calls_int_3_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                        if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(3)), point, values) { return Some(execution); }
                        let active = calls_int_3_state(point, values)?;
                        Some(Box::new(FunctionExecution::new(active)))
                    }
                    [calls_int_0_start, calls_int_1_start, calls_int_3_start]
                };
                const CALL_GROUP_1: [data::compiled::calls::CallStart; 1] = {
                    use data::compiled::calls::{CallArguments, CallCustom, CallExecution, CallInputs, CallOps, CallOutput, CallProgress, CallStorage, CallValues, CustomNativeExecution, CustomNativeRequest, IntCallable};
                    enum FunctionState {
                        Bool0Point0 {  },
                        Bool0Point1 { int0: i128 },
                        Bool0Point2 { int0: i128, custom0: CallCustom },
                        Bool0Point3 { int0: i128, custom0: CallCustom, custom1: CallCustom },
                        Bool0Point4 {  },
                        Bool0Point5 { int_function0: IntCallable },
                        Bool0Point6 { int_function0: IntCallable, custom0: CallCustom },
                        Bool0Point7 { int_function0: IntCallable, custom0: CallCustom, custom1: CallCustom },
                        Bool0Point8 {  },
                        Bool0Point9 {  },
                        Bool0Point10 { int_function0: IntCallable },
                        Bool0Point11 { int_function0: IntCallable, custom0: CallCustom },
                        Bool0Point12 { int_function0: IntCallable, custom0: CallCustom, custom1: CallCustom },
                        Bool0Point13 {  },
                        Bool0Point14 { bool0: bool },
                        Bool0Point15 { custom0: CallCustom },
                        Bool0Point16 { custom0: CallCustom },
                        Bool0Point17 { custom0: CallCustom },
                        Bool0Point18 { custom0: CallCustom },
                        CustomNativeComplete { value: CallCustom },
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
                    enum CustomReturn {
                        Bool0Call1 { int0: i128 },
                        Bool0Call2 { int0: i128, custom0: CallCustom },
                        Bool0Call5 { int_function0: IntCallable },
                        Bool0Call6 { int_function0: IntCallable, custom0: CallCustom },
                        Bool0Call10 { int_function0: IntCallable },
                        Bool0Call11 { int_function0: IntCallable, custom0: CallCustom },
                    }
                    impl CustomReturn {
                        fn site(&self) -> data::source::HostCallSite {
                            match *self {
                                Self::Bool0Call1 { .. } => data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(699, 709)),
                                Self::Bool0Call2 { .. } => data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(691, 710)),
                                Self::Bool0Call5 { .. } => data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(750, 769)),
                                Self::Bool0Call6 { .. } => data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(742, 770)),
                                Self::Bool0Call10 { .. } => data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(874, 904)),
                                Self::Bool0Call11 { .. } => data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(866, 905)),
                            }
                        }
                        fn small(self, result: CallCustom) -> FunctionState {
                            match self {
                                Self::Bool0Call1 { int0 } => {
                                    let custom0 = result;
                                    FunctionState::Bool0Point2 { int0, custom0 }
                                },
                                Self::Bool0Call2 { int0, custom0 } => {
                                    let custom1 = result;
                                    FunctionState::Bool0Point3 { int0, custom0, custom1 }
                                },
                                Self::Bool0Call5 { int_function0 } => {
                                    let custom0 = result;
                                    FunctionState::Bool0Point6 { int_function0, custom0 }
                                },
                                Self::Bool0Call6 { int_function0, custom0 } => {
                                    let custom1 = result;
                                    FunctionState::Bool0Point7 { int_function0, custom0, custom1 }
                                },
                                Self::Bool0Call10 { int_function0 } => {
                                    let custom0 = result;
                                    FunctionState::Bool0Point11 { int_function0, custom0 }
                                },
                                Self::Bool0Call11 { int_function0, custom0 } => {
                                    let custom1 = result;
                                    FunctionState::Bool0Point12 { int_function0, custom0, custom1 }
                                },
                            }
                        }
                        fn resume(self, result: CallCustom) -> FunctionState { self.small(result) }
                    }
                    #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                    enum FunctionStep {
                        Yield(FunctionState),
                        CustomNative { function: data::function::CustomFunctionId, site: data::source::HostCallSite, arguments: Box<CallValues>, caller: Option<CustomReturn> },
                        CustomNativeComplete { value: CallCustom },
                        Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                        Bool { value: bool },
                        CustomBridge { function: data::function::CustomFunctionId, site: data::source::HostCallSite, arguments: CallArguments, caller: CustomReturn },
                    }
                    struct FunctionExecution {
                        active: Option<FunctionState>,
                        custom_native_caller: Option<CustomReturn>,
                        boolean_returns: Vec<BoolReturn>,
                        custom_returns: Vec<CustomReturn>,
                    }
                    impl FunctionExecution {
                        fn new(active: FunctionState) -> Self {
                            Self {
                                active: Some(active),
                                custom_native_caller: None,
                                boolean_returns: Vec::new(),
                                custom_returns: Vec::new(),
                            }
                        }
                    }
                    impl CallExecution for FunctionExecution {
                        fn restart(&mut self, target: data::compiled::CallTarget, point: usize, values: CallInputs<'_>) -> bool {
                            if self.active.is_some() { return false; }
                            let active = match target {
                                data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)) => calls_bool_0_state(point, values),
                                _ => None,
                            };
                            let Some(active) = active else { return false; };
                            self.active = Some(active);
                            true
                        }
                        fn retained_bytes(&self) -> usize {
                            std::mem::size_of::<Self>() + self.boolean_returns.capacity() * std::mem::size_of::<BoolReturn>() + self.custom_returns.capacity() * std::mem::size_of::<CustomReturn>()
                        }
                        fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                            let Some(mut active) = self.active.take() else { return CallProgress::Yield(self); };
                            loop {
                                match function_step(active, ops, budget) {
                                    FunctionStep::CustomNative { function, site, arguments, caller } => {
                                        let root_tail = caller.is_none() && self.custom_returns.is_empty() && ops.root_tail_entry();
                                        self.custom_native_caller = caller;
                                        return CallProgress::CustomNative(CustomNativeRequest { function, site, arguments, root_tail, execution: self });
                                    },
                                    FunctionStep::CustomNativeComplete { value } => {
                                        self.boolean_returns.clear();
                                        self.custom_returns.clear();
                                        return CallProgress::Complete { output: CallOutput::Custom(value), execution: self };
                                    },
                                    FunctionStep::Yield(active) => {
                                        self.active = Some(active);
                                        return CallProgress::Yield(self);
                                    },
                                    FunctionStep::Bool { value } => {
                                        if let Some(caller) = self.boolean_returns.pop() {
                                            active = caller.small(value);
                                        } else {
                                            self.boolean_returns.clear();
                                            self.custom_returns.clear();
                                            return CallProgress::Complete { output: CallOutput::Bool(value), execution: self };
                                        }
                                    },
                                    FunctionStep::CustomBridge { function, site, arguments, caller } => return CallProgress::Custom {
                                        function, site, arguments,
                                        resume: Box::new(move |value| {
                                            self.active = Some(caller.resume(value));
                                            self
                                        }),
                                    },
                                    FunctionStep::Canonical { target, point, values } => {
                                        match target {
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
                                                return CallProgress::Interpreted { target, point, values };
                                            },
                                            data::compiled::CallTarget::Custom(function) => {
                                                if let Some(caller) = self.custom_returns.pop() {
                                                    let site = caller.site();
                                                    return CallProgress::InterpretedCustom {
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
                    impl CustomNativeExecution for FunctionExecution {
                        fn resume_native(mut self: Box<Self>, value: CallCustom) -> Box<dyn CallExecution> {
                            let active = if let Some(caller) = self.custom_native_caller.take().or_else(|| self.custom_returns.pop()) {
                                caller.small(value)
                            } else {
                                FunctionState::CustomNativeComplete { value }
                            };
                            self.active = Some(active);
                            self
                        }
                    }
                    fn function_step(active: FunctionState, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            FunctionState::CustomNativeComplete { value } => FunctionStep::CustomNativeComplete { value },
                            FunctionState::Bool0Point0 {  } => calls_bool_0_run(Bool0State::Point0 {  }, ops, budget),
                            FunctionState::Bool0Point1 { int0 } => calls_bool_0_run(Bool0State::Point1 { int0 }, ops, budget),
                            FunctionState::Bool0Point2 { int0, custom0 } => calls_bool_0_run(Bool0State::Point2 { int0, custom0 }, ops, budget),
                            FunctionState::Bool0Point3 { int0, custom0, custom1 } => calls_bool_0_run(Bool0State::Point3 { int0, custom0, custom1 }, ops, budget),
                            FunctionState::Bool0Point4 {  } => calls_bool_0_run(Bool0State::Point4 {  }, ops, budget),
                            FunctionState::Bool0Point5 { int_function0 } => calls_bool_0_run(Bool0State::Point5 { int_function0 }, ops, budget),
                            FunctionState::Bool0Point6 { int_function0, custom0 } => calls_bool_0_run(Bool0State::Point6 { int_function0, custom0 }, ops, budget),
                            FunctionState::Bool0Point7 { int_function0, custom0, custom1 } => calls_bool_0_run(Bool0State::Point7 { int_function0, custom0, custom1 }, ops, budget),
                            FunctionState::Bool0Point8 {  } => calls_bool_0_run(Bool0State::Point8 {  }, ops, budget),
                            FunctionState::Bool0Point9 {  } => calls_bool_0_run(Bool0State::Point9 {  }, ops, budget),
                            FunctionState::Bool0Point10 { int_function0 } => calls_bool_0_run(Bool0State::Point10 { int_function0 }, ops, budget),
                            FunctionState::Bool0Point11 { int_function0, custom0 } => calls_bool_0_run(Bool0State::Point11 { int_function0, custom0 }, ops, budget),
                            FunctionState::Bool0Point12 { int_function0, custom0, custom1 } => calls_bool_0_run(Bool0State::Point12 { int_function0, custom0, custom1 }, ops, budget),
                            FunctionState::Bool0Point13 {  } => calls_bool_0_run(Bool0State::Point13 {  }, ops, budget),
                            FunctionState::Bool0Point14 { bool0 } => calls_bool_0_run(Bool0State::Point14 { bool0 }, ops, budget),
                            FunctionState::Bool0Point15 { custom0 } => calls_bool_0_run(Bool0State::Point15 { custom0 }, ops, budget),
                            FunctionState::Bool0Point16 { custom0 } => calls_bool_0_run(Bool0State::Point16 { custom0 }, ops, budget),
                            FunctionState::Bool0Point17 { custom0 } => calls_bool_0_run(Bool0State::Point17 { custom0 }, ops, budget),
                            FunctionState::Bool0Point18 { custom0 } => calls_bool_0_run(Bool0State::Point18 { custom0 }, ops, budget),
                        }
                    }
                    enum Bool0State {
                        Point0 {  },
                        Point1 { int0: i128 },
                        Point2 { int0: i128, custom0: CallCustom },
                        Point3 { int0: i128, custom0: CallCustom, custom1: CallCustom },
                        Point4 {  },
                        Point5 { int_function0: IntCallable },
                        Point6 { int_function0: IntCallable, custom0: CallCustom },
                        Point7 { int_function0: IntCallable, custom0: CallCustom, custom1: CallCustom },
                        Point8 {  },
                        Point9 {  },
                        Point10 { int_function0: IntCallable },
                        Point11 { int_function0: IntCallable, custom0: CallCustom },
                        Point12 { int_function0: IntCallable, custom0: CallCustom, custom1: CallCustom },
                        Point13 {  },
                        Point14 { bool0: bool },
                        Point15 { custom0: CallCustom },
                        Point16 { custom0: CallCustom },
                        Point17 { custom0: CallCustom },
                        Point18 { custom0: CallCustom },
                    }
                    fn calls_bool_0_run(mut active: Bool0State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        loop {
                            match active {
                                Bool0State::Point0 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point0 {  }); }
                                    *budget -= 1;
                                    let int0 = 5_i128;
                                    if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
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
                                    }, values: Box::new(CallValues { ints: vec![int0.into()], ..CallValues::default() }) }; }
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point1 { int0 }); }
                                    *budget -= 1;
                                    return {
                                        if ops.supports_custom_native(data::function::CustomFunctionId {
                                            index: 0,
                                            return_shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }) {
                                            return FunctionStep::CustomNative { function: data::function::CustomFunctionId {
                                                index: 0,
                                                return_shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(6),
                                                },
                                            }, site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(699, 709)), arguments: Box::new(CallValues { ints: vec![int0.into()], ..CallValues::default() }), caller: Some(CustomReturn::Bool0Call1 { int0 }) };
                                        }
                                        FunctionStep::CustomBridge { function: data::function::CustomFunctionId {
                                            index: 0,
                                            return_shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }, site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(699, 709)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int0.into()], ..CallValues::default() }), captures: None }, caller: CustomReturn::Bool0Call1 { int0 } }
                                    };
                                },
                                Bool0State::Point1 { int0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point1 { int0 }); }
                                    *budget -= 1;
                                    return {
                                        if ops.supports_custom_native(data::function::CustomFunctionId {
                                            index: 0,
                                            return_shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }) {
                                            return FunctionStep::CustomNative { function: data::function::CustomFunctionId {
                                                index: 0,
                                                return_shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(6),
                                                },
                                            }, site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(699, 709)), arguments: Box::new(CallValues { ints: vec![int0.into()], ..CallValues::default() }), caller: Some(CustomReturn::Bool0Call1 { int0 }) };
                                        }
                                        FunctionStep::CustomBridge { function: data::function::CustomFunctionId {
                                            index: 0,
                                            return_shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }, site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(699, 709)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int0.into()], ..CallValues::default() }), captures: None }, caller: CustomReturn::Bool0Call1 { int0 } }
                                    };
                                },
                                Bool0State::Point2 { int0, custom0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point2 { int0, custom0 }); }
                                    *budget -= 1;
                                    return {
                                        FunctionStep::CustomBridge { function: data::function::CustomFunctionId {
                                            index: 1,
                                            return_shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }, site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(691, 710)), arguments: CallArguments { values: Box::new(CallValues { customs: vec![custom0.clone()], ..CallValues::default() }), captures: None }, caller: CustomReturn::Bool0Call2 { int0, custom0 } }
                                    };
                                },
                                Bool0State::Point3 { int0, custom0, custom1 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point3 { int0, custom0, custom1 }); }
                                    active = {
                                        let matched = (|| -> Option<Option<()>> {
                                            if !custom1.matches_constructor(data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(2),
                                                index: 0,
                                            }) { return Some(None); }
                                            let field0 = custom1.field(0)?;
                                            if !field0.matches_type(&data::type_::ValueType::Int) { return None; }
                                            if field0.integer()? != 5_i128 { return Some(None); }
                                            Some(Some(()))
                                        })();
                                        let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                            block: data::graph::BlockId(0),
                                            instruction: 3,
                                            ints: 1,
                                            bools: 0,
                                            bit_arrays: 0,
                                            int_lists: 0,
                                            strings: 0,
                                            customs: 2,
                                            custom_lists: 0,
                                            int_functions: 0,
                                            bool_functions: 0,
                                        }, values: Box::new(CallValues { customs: vec![custom0, custom1], ints: vec![int0.into()], ..CallValues::default() }) }; };
                                        *budget -= 1;
                                        match matched {
                                            Some(()) => Bool0State::Point4 {  },
                                            None => Bool0State::Point18 { custom0: custom1.clone() },
                                        }
                                    };
                                    continue;
                                },
                                Bool0State::Point4 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point4 {  }); }
                                    *budget -= 1;
                                    let int_function0 = ops.int_closure(data::function::IntFunctionId(3), data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    }, vec![]);
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point5 { int_function0 }); }
                                    *budget -= 1;
                                    return {
                                        FunctionStep::CustomBridge { function: data::function::CustomFunctionId {
                                            index: 2,
                                            return_shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }, site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(750, 769)), arguments: CallArguments { values: Box::new(CallValues { int_functions: vec![int_function0.clone()], ..CallValues::default() }), captures: None }, caller: CustomReturn::Bool0Call5 { int_function0 } }
                                    };
                                },
                                Bool0State::Point5 { int_function0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point5 { int_function0 }); }
                                    *budget -= 1;
                                    return {
                                        FunctionStep::CustomBridge { function: data::function::CustomFunctionId {
                                            index: 2,
                                            return_shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }, site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(750, 769)), arguments: CallArguments { values: Box::new(CallValues { int_functions: vec![int_function0.clone()], ..CallValues::default() }), captures: None }, caller: CustomReturn::Bool0Call5 { int_function0 } }
                                    };
                                },
                                Bool0State::Point6 { int_function0, custom0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point6 { int_function0, custom0 }); }
                                    *budget -= 1;
                                    return {
                                        FunctionStep::CustomBridge { function: data::function::CustomFunctionId {
                                            index: 1,
                                            return_shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }, site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(742, 770)), arguments: CallArguments { values: Box::new(CallValues { customs: vec![custom0.clone()], ..CallValues::default() }), captures: None }, caller: CustomReturn::Bool0Call6 { int_function0, custom0 } }
                                    };
                                },
                                Bool0State::Point7 { int_function0, custom0, custom1 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point7 { int_function0, custom0, custom1 }); }
                                    active = {
                                        let matched = (|| -> Option<Option<()>> {
                                            if !custom1.matches_constructor(data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(2),
                                                index: 1,
                                            }) { return Some(None); }
                                            let field0 = custom1.field(0)?;
                                            if !field0.matches_type(&data::type_::ValueType::String) { return None; }
                                            if field0.string()?.as_bytes() != "caught".as_bytes() { return Some(None); }
                                            Some(Some(()))
                                        })();
                                        let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                            block: data::graph::BlockId(1),
                                            instruction: 3,
                                            ints: 0,
                                            bools: 0,
                                            bit_arrays: 0,
                                            int_lists: 0,
                                            strings: 0,
                                            customs: 2,
                                            custom_lists: 0,
                                            int_functions: 1,
                                            bool_functions: 0,
                                        }, values: Box::new(CallValues { customs: vec![custom0, custom1], int_functions: vec![int_function0], ..CallValues::default() }) }; };
                                        *budget -= 1;
                                        match matched {
                                            Some(()) => Bool0State::Point8 {  },
                                            None => Bool0State::Point17 { custom0: custom1.clone() },
                                        }
                                    };
                                    continue;
                                },
                                Bool0State::Point8 {  } => {
                                    return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
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
                                    }, values: Box::new(CallValues::default()) };
                                },
                                Bool0State::Point9 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point9 {  }); }
                                    *budget -= 1;
                                    let int_function0 = ops.int_closure(data::function::IntFunctionId(4), data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    }, vec![]);
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point10 { int_function0 }); }
                                    *budget -= 1;
                                    return {
                                        FunctionStep::CustomBridge { function: data::function::CustomFunctionId {
                                            index: 2,
                                            return_shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }, site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(874, 904)), arguments: CallArguments { values: Box::new(CallValues { int_functions: vec![int_function0.clone()], ..CallValues::default() }), captures: None }, caller: CustomReturn::Bool0Call10 { int_function0 } }
                                    };
                                },
                                Bool0State::Point10 { int_function0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point10 { int_function0 }); }
                                    *budget -= 1;
                                    return {
                                        FunctionStep::CustomBridge { function: data::function::CustomFunctionId {
                                            index: 2,
                                            return_shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }, site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(874, 904)), arguments: CallArguments { values: Box::new(CallValues { int_functions: vec![int_function0.clone()], ..CallValues::default() }), captures: None }, caller: CustomReturn::Bool0Call10 { int_function0 } }
                                    };
                                },
                                Bool0State::Point11 { int_function0, custom0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point11 { int_function0, custom0 }); }
                                    *budget -= 1;
                                    return {
                                        FunctionStep::CustomBridge { function: data::function::CustomFunctionId {
                                            index: 1,
                                            return_shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }, site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(866, 905)), arguments: CallArguments { values: Box::new(CallValues { customs: vec![custom0.clone()], ..CallValues::default() }), captures: None }, caller: CustomReturn::Bool0Call11 { int_function0, custom0 } }
                                    };
                                },
                                Bool0State::Point12 { int_function0, custom0, custom1 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point12 { int_function0, custom0, custom1 }); }
                                    active = {
                                        let matched = (|| -> Option<Option<()>> {
                                            if !custom1.matches_constructor(data::type_::CustomConstructorId {
                                                type_id: data::type_::CustomTypeId(2),
                                                index: 1,
                                            }) { return Some(None); }
                                            let field0 = custom1.field(0)?;
                                            if !field0.matches_type(&data::type_::ValueType::String) { return None; }
                                            if field0.string()?.as_bytes() != "caught".as_bytes() { return Some(None); }
                                            Some(Some(()))
                                        })();
                                        let Some(matched) = matched else { return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                            block: data::graph::BlockId(3),
                                            instruction: 3,
                                            ints: 0,
                                            bools: 0,
                                            bit_arrays: 0,
                                            int_lists: 0,
                                            strings: 0,
                                            customs: 2,
                                            custom_lists: 0,
                                            int_functions: 1,
                                            bool_functions: 0,
                                        }, values: Box::new(CallValues { customs: vec![custom0, custom1], int_functions: vec![int_function0], ..CallValues::default() }) }; };
                                        *budget -= 1;
                                        match matched {
                                            Some(()) => Bool0State::Point13 {  },
                                            None => Bool0State::Point15 { custom0: custom1.clone() },
                                        }
                                    };
                                    continue;
                                },
                                Bool0State::Point13 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point13 {  }); }
                                    *budget -= 1;
                                    let bool0 = true;
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point14 { bool0 }); }
                                    *budget -= 1;
                                    return {
                                        FunctionStep::Bool { value: bool0 }
                                    };
                                },
                                Bool0State::Point14 { bool0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point14 { bool0 }); }
                                    *budget -= 1;
                                    return {
                                        FunctionStep::Bool { value: bool0 }
                                    };
                                },
                                Bool0State::Point15 { custom0 } => {
                                    return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(5),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) };
                                },
                                Bool0State::Point16 { custom0 } => {
                                    return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(6),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) };
                                },
                                Bool0State::Point17 { custom0 } => {
                                    return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(7),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) };
                                },
                                Bool0State::Point18 { custom0 } => {
                                    return FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(8),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { customs: vec![custom0], ..CallValues::default() }) };
                                },
                            }
                        }
                    }
                    fn calls_bool_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                        let active = match point {
                            0 => FunctionState::Bool0Point0 {  },
                            1 => FunctionState::Bool0Point1 { int0: values.int(0)? },
                            2 => FunctionState::Bool0Point2 { int0: values.int(0)?, custom0: values.custom(0)? },
                            3 => FunctionState::Bool0Point3 { int0: values.int(0)?, custom0: values.custom(0)?, custom1: values.custom(1)? },
                            4 => FunctionState::Bool0Point4 {  },
                            5 => FunctionState::Bool0Point5 { int_function0: values.int_function(0)? },
                            6 => FunctionState::Bool0Point6 { int_function0: values.int_function(0)?, custom0: values.custom(0)? },
                            7 => FunctionState::Bool0Point7 { int_function0: values.int_function(0)?, custom0: values.custom(0)?, custom1: values.custom(1)? },
                            8 => FunctionState::Bool0Point8 {  },
                            9 => FunctionState::Bool0Point9 {  },
                            10 => FunctionState::Bool0Point10 { int_function0: values.int_function(0)? },
                            11 => FunctionState::Bool0Point11 { int_function0: values.int_function(0)?, custom0: values.custom(0)? },
                            12 => FunctionState::Bool0Point12 { int_function0: values.int_function(0)?, custom0: values.custom(0)?, custom1: values.custom(1)? },
                            13 => FunctionState::Bool0Point13 {  },
                            14 => FunctionState::Bool0Point14 { bool0: values.bool(0)? },
                            15 => FunctionState::Bool0Point15 { custom0: values.custom(0)? },
                            16 => FunctionState::Bool0Point16 { custom0: values.custom(0)? },
                            17 => FunctionState::Bool0Point17 { custom0: values.custom(0)? },
                            18 => FunctionState::Bool0Point18 { custom0: values.custom(0)? },
                            _ => return None,
                        };
                        Some(active)
                    }
                    fn calls_bool_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                        if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point, values) { return Some(execution); }
                        let active = calls_bool_0_state(point, values)?;
                        Some(Box::new(FunctionExecution::new(active)))
                    }
                    [calls_bool_0_start]
                };
                const CALL_GROUP_2: [data::compiled::calls::CallStart; 1] = {
                    use data::compiled::calls::{CallExecution, CallInputs, CallOps, CallOutput, CallProgress, CallStorage, StringValue};
                    enum FunctionState {
                        String0Point0 { string0: StringValue },
                    }
                    enum StringReturn {
                    }
                    impl StringReturn {
                        fn small(self, result: StringValue) -> FunctionState {
                            let _ = result;
                            match self {
                            }
                        }
                    }
                    #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                    enum FunctionStep {
                        Yield(FunctionState),
                        String { value: StringValue },
                    }
                    struct FunctionExecution {
                        active: Option<FunctionState>,
                        string_returns: Vec<StringReturn>,
                    }
                    impl FunctionExecution {
                        fn new(active: FunctionState) -> Self {
                            Self {
                                active: Some(active),
                                string_returns: Vec::new(),
                            }
                        }
                    }
                    impl CallExecution for FunctionExecution {
                        fn restart(&mut self, target: data::compiled::CallTarget, point: usize, values: CallInputs<'_>) -> bool {
                            if self.active.is_some() { return false; }
                            let active = match target {
                                data::compiled::CallTarget::String(data::function::StringFunctionId(0)) => calls_string_0_state(point, values),
                                _ => None,
                            };
                            let Some(active) = active else { return false; };
                            self.active = Some(active);
                            true
                        }
                        fn retained_bytes(&self) -> usize {
                            std::mem::size_of::<Self>() + self.string_returns.capacity() * std::mem::size_of::<StringReturn>()
                        }
                        fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                            let Some(mut active) = self.active.take() else { return CallProgress::Yield(self); };
                            loop {
                                match function_step(active, ops, budget) {
                                    FunctionStep::Yield(active) => {
                                        self.active = Some(active);
                                        return CallProgress::Yield(self);
                                    },
                                    FunctionStep::String { value } => {
                                        if let Some(caller) = self.string_returns.pop() {
                                            active = caller.small(value);
                                        } else {
                                            self.string_returns.clear();
                                            return CallProgress::Complete { output: CallOutput::String(value), execution: self };
                                        }
                                    },
                                }
                            }
                        }
                    }
                    fn function_step(active: FunctionState, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            FunctionState::String0Point0 { string0 } => calls_string_0_run(String0State::Point0 { string0 }, ops, budget),
                        }
                    }
                    enum String0State {
                        Point0 { string0: StringValue },
                    }
                    fn calls_string_0_run(active: String0State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            String0State::Point0 { string0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::String0Point0 { string0 }); }
                                *budget -= 1;
                                {
                                    FunctionStep::String { value: string0 }
                                }
                            },
                        }
                    }
                    fn calls_string_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                        let active = match point {
                            0 => FunctionState::String0Point0 { string0: values.string(0)? },
                            _ => return None,
                        };
                        Some(active)
                    }
                    fn calls_string_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                        if let Some(execution) = storage.reuse(data::compiled::CallTarget::String(data::function::StringFunctionId(0)), point, values) { return Some(execution); }
                        let active = calls_string_0_state(point, values)?;
                        Some(Box::new(FunctionExecution::new(active)))
                    }
                    [calls_string_0_start]
                };
                const CALL_GROUP_3: [data::compiled::calls::CallStart; 1] = {
                    use data::compiled::calls::{CallBitArray, CallExecution, CallInputs, CallOps, CallOutput, CallProgress, CallStorage};
                    enum FunctionState {
                        BitArray3Point0 { bit_array0: CallBitArray },
                    }
                    enum BitArrayReturn {
                    }
                    impl BitArrayReturn {
                        fn small(self, result: CallBitArray) -> FunctionState {
                            let _ = result;
                            match self {
                            }
                        }
                    }
                    #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                    enum FunctionStep {
                        Yield(FunctionState),
                        BitArray { value: CallBitArray },
                    }
                    struct FunctionExecution {
                        active: Option<FunctionState>,
                        bit_array_returns: Vec<BitArrayReturn>,
                    }
                    impl FunctionExecution {
                        fn new(active: FunctionState) -> Self {
                            Self {
                                active: Some(active),
                                bit_array_returns: Vec::new(),
                            }
                        }
                    }
                    impl CallExecution for FunctionExecution {
                        fn restart(&mut self, target: data::compiled::CallTarget, point: usize, values: CallInputs<'_>) -> bool {
                            if self.active.is_some() { return false; }
                            let active = match target {
                                data::compiled::CallTarget::BitArray(data::function::BitArrayFunctionId(3)) => calls_bitarray_3_state(point, values),
                                _ => None,
                            };
                            let Some(active) = active else { return false; };
                            self.active = Some(active);
                            true
                        }
                        fn retained_bytes(&self) -> usize {
                            std::mem::size_of::<Self>() + self.bit_array_returns.capacity() * std::mem::size_of::<BitArrayReturn>()
                        }
                        fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                            let Some(mut active) = self.active.take() else { return CallProgress::Yield(self); };
                            loop {
                                match function_step(active, ops, budget) {
                                    FunctionStep::Yield(active) => {
                                        self.active = Some(active);
                                        return CallProgress::Yield(self);
                                    },
                                    FunctionStep::BitArray { value } => {
                                        if let Some(caller) = self.bit_array_returns.pop() {
                                            active = caller.small(value);
                                        } else {
                                            self.bit_array_returns.clear();
                                            return CallProgress::Complete { output: CallOutput::BitArray(value), execution: self };
                                        }
                                    },
                                }
                            }
                        }
                    }
                    fn function_step(active: FunctionState, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            FunctionState::BitArray3Point0 { bit_array0 } => calls_bitarray_3_run(BitArray3State::Point0 { bit_array0 }, ops, budget),
                        }
                    }
                    enum BitArray3State {
                        Point0 { bit_array0: CallBitArray },
                    }
                    fn calls_bitarray_3_run(active: BitArray3State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            BitArray3State::Point0 { bit_array0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::BitArray3Point0 { bit_array0 }); }
                                *budget -= 1;
                                {
                                    FunctionStep::BitArray { value: bit_array0 }
                                }
                            },
                        }
                    }
                    fn calls_bitarray_3_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                        let active = match point {
                            0 => FunctionState::BitArray3Point0 { bit_array0: values.bit_array(0)? },
                            _ => return None,
                        };
                        Some(active)
                    }
                    fn calls_bitarray_3_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                        if let Some(execution) = storage.reuse(data::compiled::CallTarget::BitArray(data::function::BitArrayFunctionId(3)), point, values) { return Some(execution); }
                        let active = calls_bitarray_3_state(point, values)?;
                        Some(Box::new(FunctionExecution::new(active)))
                    }
                    [calls_bitarray_3_start]
                };

                fn numeric_int_1(
                    point: usize,
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {

                    const RESUME: [
                        fn(&mut data::compiled::numeric::NumericValues, &mut usize) -> CompiledResume;
                        5
                    ] = [
                        |values, budget| CompiledResume::Exit(numeric_int_1_entry((values.ints[0],), values, budget)),
                        numeric_int_1_resume_1,
                        numeric_int_1_resume_2,
                        numeric_int_1_resume_3,
                        numeric_int_1_resume_4,
                    ];

                    let mut point = point;
                    loop {
                        match RESUME[point](values, budget) {
                            CompiledResume::Next(next) => point = next,
                            CompiledResume::Exit(progress) => return progress,
                        }
                    }
                }

                fn numeric_int_1_entry(
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
                    if b0_i0 >= 0_i128 {
                        let (b1_i0,) = (b0_i0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(1);
                        }
                        *budget -= 1;
                        let b1_i1 = b1_i0 + 1_i128;
                        if b1_i1 < i128::from(i64::MIN) || b1_i1 > i128::from(i64::MAX) {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Interpreted(2);
                        }
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(2);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                    } else {
                        let (b2_i0,) = (b0_i0,);
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(3);
                        }
                        *budget -= 1;
                        let b2_i1 = b2_i0 - 1_i128;
                        if b2_i1 < i128::from(i64::MIN) || b2_i1 > i128::from(i64::MAX) {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Interpreted(4);
                        }
                        if *budget == 0 {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            return data::compiled::CompiledProgress::Yield(4);
                        }
                        *budget -= 1;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
                    }
                }

                fn numeric_int_1_resume_1(
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
                    let b1_i1 = b1_i0 + 1_i128;
                    if b1_i1 < i128::from(i64::MIN) || b1_i1 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(2));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(2)
                }

                fn numeric_int_1_resume_2(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b1_i0, b1_i1,) = (values.ints[0], values.ints[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
                }

                fn numeric_int_1_resume_3(
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
                    let b2_i1 = b2_i0 - 1_i128;
                    if b2_i1 < i128::from(i64::MIN) || b2_i1 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(4));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Next(4)
                }

                fn numeric_int_1_resume_4(
                    values: &mut data::compiled::numeric::NumericValues,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b2_i0, b2_i1,) = (values.ints[0], values.ints[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(4));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0, b2_i1]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
                }

                fn int_list_int_5(
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
                            let _list0 = values.int_lists.remove(0);
                            CompiledResume::Exit(int_list_int_5_entry((values.ints[0], _list0,), values, _lists, budget))
                        },
                        int_list_int_5_resume_1,
                        int_list_int_5_resume_2,
                        int_list_int_5_resume_3,
                        int_list_int_5_resume_4,
                        int_list_int_5_resume_5,
                    ];

                    let mut point = point;
                    loop {
                        match RESUME[point](values, _lists, budget) {
                            CompiledResume::Next(next) => point = next,
                            CompiledResume::Exit(progress) => return progress,
                        }
                    }
                }

                fn int_list_int_5_entry(
                    inputs: (i128, data::compiled::int_list::IntList,),
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> data::compiled::CompiledProgress {
                    let (b0_i0, b0_l0,) = inputs;
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b0_i0]);
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
                            let m1 = _lists.tail(&b0_l0, data::type_::IntListTypeId {
                                list_type: data::type_::ListTypeId(0),
                            }, 1);
                            let (b1_i0, b1_i1, b1_l0, b1_l1,) = (b0_i0, m0, m1, b0_l0,);
                            if *budget == 0 {

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([b1_l0, b1_l1]);
                                return data::compiled::CompiledProgress::Yield(1);
                            }
                            *budget -= 1;
                            if _lists.equal(&b1_l0, &b1_l1) {
                                let _next = (b1_i0,);
                                drop(b1_l0);
                                drop(b1_l1);
                                let (b2_i0,) = _next;
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b2_i0]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.int_lists.clear();
                                    values.int_lists.extend([]);
                                    return data::compiled::CompiledProgress::Yield(2);
                                }
                                *budget -= 1;

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b2_i0]);
                                values.bools.clear();
                                values.bools.extend_from_slice(&[]);
                                values.int_lists.clear();
                                values.int_lists.extend([]);
                                data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
                            } else {
                                let _next = (b1_i0, b1_i1,);
                                drop(b1_l0);
                                drop(b1_l1);
                                let (b3_i0, b3_i1,) = _next;
                                if *budget == 0 {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.int_lists.clear();
                                    values.int_lists.extend([]);
                                    return data::compiled::CompiledProgress::Yield(3);
                                }
                                *budget -= 1;
                                let b3_i2 = b3_i1 + b3_i0;
                                if b3_i2 < i128::from(i64::MIN) || b3_i2 > i128::from(i64::MAX) {

                                    values.ints.clear();
                                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                                    values.bools.clear();
                                    values.bools.extend_from_slice(&[]);
                                    values.int_lists.clear();
                                    values.int_lists.extend([]);
                                    return data::compiled::CompiledProgress::Interpreted(4);
                                }
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

                                values.ints.clear();
                                values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
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
                                return data::compiled::CompiledProgress::Yield(5);
                            }

                            values.ints.clear();
                            values.ints.extend_from_slice(&[]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.int_lists.clear();
                            values.int_lists.extend([b4_l0]);
                            data::compiled::CompiledProgress::Interpreted(5)
                        },
                        Err(()) => {

                            values.ints.clear();
                            values.ints.extend_from_slice(&[b0_i0]);
                            values.bools.clear();
                            values.bools.extend_from_slice(&[]);
                            values.int_lists.clear();
                            values.int_lists.extend([b0_l0]);
                            data::compiled::CompiledProgress::Interpreted(0)
                        }
                    }
                }

                fn int_list_int_5_resume_1(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let _list1 = values.int_lists.remove(1);
                    let _list0 = values.int_lists.remove(0);
                    let (b1_i0, b1_i1, b1_l0, b1_l1,) = (values.ints[0], values.ints[1], _list0, _list1,);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b1_i0, b1_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([b1_l0, b1_l1]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                    }
                    *budget -= 1;
                    if _lists.equal(&b1_l0, &b1_l1) {
                        let _next = (b1_i0,);
                        drop(b1_l0);
                        drop(b1_l1);
                        let (b2_i0,) = _next;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([]);
                        CompiledResume::Next(2)
                    } else {
                        let _next = (b1_i0, b1_i1,);
                        drop(b1_l0);
                        drop(b1_l1);
                        let (b3_i0, b3_i1,) = _next;

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([]);
                        CompiledResume::Next(3)
                    }
                }

                fn int_list_int_5_resume_2(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b2_i0,) = (values.ints[0],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b2_i0]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                    }
                    *budget -= 1;

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b2_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
                }

                fn int_list_int_5_resume_3(
                    values: &mut data::compiled::int_list::IntListValues,
                    _lists: &data::compiled::int_list::IntListOps<'_>,
                    budget: &mut usize,
                ) -> CompiledResume {
                    let (b3_i0, b3_i1,) = (values.ints[0], values.ints[1],);
                    if *budget == 0 {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                    }
                    *budget -= 1;
                    let b3_i2 = b3_i1 + b3_i0;
                    if b3_i2 < i128::from(i64::MIN) || b3_i2 > i128::from(i64::MAX) {

                        values.ints.clear();
                        values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                        values.bools.clear();
                        values.bools.extend_from_slice(&[]);
                        values.int_lists.clear();
                        values.int_lists.extend([]);
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(4));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([]);
                    CompiledResume::Next(4)
                }

                fn int_list_int_5_resume_4(
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

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1)))
                }

                fn int_list_int_5_resume_5(
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
                        return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(5));
                    }

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);
                    values.int_lists.clear();
                    values.int_lists.extend([b4_l0]);
                    CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(5))
                }
                data::compiled::CompiledFunctions {
                    ints: data::Storage::Static(&[
                        data::compiled::CompiledFunction {
                            function: data::function::IntFunctionId(1),
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
                                run: numeric_int_1,
                            }),
                        },
                        data::compiled::CompiledFunction {
                            function: data::function::IntFunctionId(5),
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
                                run: int_list_int_5,
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
                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(5)),
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
                                        ]),
                                    },
                                ]),
                                returns: data::Storage::Static(&[]),
                                tails: data::Storage::Static(&[
                                    data::compiled::TailContract {
                                        point: 1,
                                        target: data::compiled::CallTarget::Int(data::function::IntFunctionId(2)),
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
                                        site: data::source::HostCallSite::from_static("main", "list_callback", data::source::SourceSpan::new(1614, 1638)),
                                    },
                                ]),
                                start: CALL_GROUP_0[0],
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(1)),
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
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
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
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 2,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    },
                                    data::compiled::ReturnContract {
                                        point: 4,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: CALL_GROUP_0[1],
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::Int(data::function::IntFunctionId(3)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: false,
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
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 1,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: CALL_GROUP_0[2],
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
                                        instruction: 3,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 2,
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
                                        block: data::graph::BlockId(1),
                                        instruction: 2,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(1),
                                        instruction: 3,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 2,
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
                                        instruction: 2,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 1,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    },
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(3),
                                        instruction: 3,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 2,
                                        custom_lists: 0,
                                        int_functions: 1,
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
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(5),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(6),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(7),
                                        instruction: 0,
                                        ints: 0,
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
                                        block: data::graph::BlockId(8),
                                        instruction: 0,
                                        ints: 0,
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
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }),
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(3),
                                                shape_id: data::type_::CustomValueShapeId(7),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 1,
                                        output: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Custom(data::function::CustomFunctionId {
                                            index: 0,
                                            return_shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        })),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(699, 709)),
                                    },
                                    data::compiled::CallContract {
                                        point: 2,
                                        output: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Custom(data::function::CustomFunctionId {
                                            index: 1,
                                            return_shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        })),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(6),
                                                },
                                            }),
                                        ]),
                                        site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(691, 710)),
                                    },
                                    data::compiled::CallContract {
                                        point: 5,
                                        output: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Custom(data::function::CustomFunctionId {
                                            index: 2,
                                            return_shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        })),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                        ]),
                                        site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(750, 769)),
                                    },
                                    data::compiled::CallContract {
                                        point: 6,
                                        output: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Custom(data::function::CustomFunctionId {
                                            index: 1,
                                            return_shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        })),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(6),
                                                },
                                            }),
                                        ]),
                                        site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(742, 770)),
                                    },
                                    data::compiled::CallContract {
                                        point: 10,
                                        output: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(0),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Custom(data::function::CustomFunctionId {
                                            index: 2,
                                            return_shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        })),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::IntFunction {
                                                local: data::graph::IntFunctionLocalId(0),
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                            },
                                        ]),
                                        site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(874, 904)),
                                    },
                                    data::compiled::CallContract {
                                        point: 11,
                                        output: data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                            id: data::graph::CustomLocalId(1),
                                            shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        }),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Custom(data::function::CustomFunctionId {
                                            index: 1,
                                            return_shape: data::type_::CustomValueShape {
                                                type_id: data::type_::CustomTypeId(2),
                                                shape_id: data::type_::CustomValueShapeId(6),
                                            },
                                        })),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                                                id: data::graph::CustomLocalId(0),
                                                shape: data::type_::CustomValueShape {
                                                    type_id: data::type_::CustomTypeId(2),
                                                    shape_id: data::type_::CustomValueShapeId(6),
                                                },
                                            }),
                                        ]),
                                        site: data::source::HostCallSite::from_static("main", "generic_results", data::source::SourceSpan::new(866, 905)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[
                                    data::compiled::CreationContract {
                                        point: 4,
                                        output: data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(3)),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                        reference: false,
                                        captures: data::Storage::Static(&[]),
                                    },
                                    data::compiled::CreationContract {
                                        point: 9,
                                        output: data::graph::ParamLocal::IntFunction {
                                            local: data::graph::IntFunctionLocalId(0),
                                            type_: data::type_::FunctionType {
                                                arguments: data::Storage::Static(&[]),
                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                            },
                                        },
                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(4)),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                        reference: false,
                                        captures: data::Storage::Static(&[]),
                                    },
                                ]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 14,
                                        value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: CALL_GROUP_1[0],
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::String(data::function::StringFunctionId(0)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: false,
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
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 0,
                                        value: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: CALL_GROUP_2[0],
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::BitArray(data::function::BitArrayFunctionId(3)),
                            implementation: data::compiled::CompiledImplementation::FunctionCalls(data::Storage::Static(&data::compiled::FunctionCallsImplementation {
                                root: false,
                                entry: 0,
                                checkpoints: data::Storage::Static(&[
                                    data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 0,
                                        ints: 0,
                                        bools: 0,
                                        bit_arrays: 1,
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
                                        data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 0,
                                        value: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: CALL_GROUP_3[0],
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
                    0..1,
                    1..7,
                    0..0,
                    7..8,
                    8..12,
                    0..0,
                    12..17,
                    0..0,
                    17..22,
                    0..0,
                    22..26,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
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
                        return_: data::type_::ValueShapeId(23),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 0..2,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(28),
                            data::type_::ValueShapeId(17),
                        ]),
                        return_: data::type_::ValueShapeId(17),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 2..3,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(17),
                        ]),
                        return_: data::type_::ValueShapeId(17),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 3..5,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(18),
                            data::type_::ValueShapeId(17),
                        ]),
                        return_: data::type_::ValueShapeId(17),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 5..5,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(17),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 5..5,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(17),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 5..6,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(17),
                        ]),
                        return_: data::type_::ValueShapeId(17),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::List(data::graph::ListLocal::Int {
                                    local: data::graph::IntListLocalId(0),
                                    type_id: data::type_::IntListTypeId {
                                        list_type: data::type_::ListTypeId(0),
                                    },
                                }),
                                shape: data::type_::ValueShapeId(28),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 6..6,
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
                        parameters: 6..9,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(17),
                            data::type_::ValueShapeId(17),
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
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 10..11,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 11..11,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[
                            data::graph::ParamSlot {
                                local: data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                                shape: data::type_::ValueShapeId(0),
                            },
                        ]),
                    },
                    data::function::FunctionContract {
                        parameters: 11..12,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(17),
                        ]),
                        return_: data::type_::ValueShapeId(24),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 12..13,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(24),
                        ]),
                        return_: data::type_::ValueShapeId(24),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 13..14,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(25),
                        ]),
                        return_: data::type_::ValueShapeId(24),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 14..15,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(26),
                        ]),
                        return_: data::type_::ValueShapeId(27),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 15..16,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(27),
                        ]),
                        return_: data::type_::ValueShapeId(27),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 16..16,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(6),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 16..16,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(6),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 16..18,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(9),
                            data::type_::ValueShapeId(12),
                        ]),
                        return_: data::type_::ValueShapeId(6),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 18..20,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(14),
                            data::type_::ValueShapeId(16),
                        ]),
                        return_: data::type_::ValueShapeId(6),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 20..22,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(16),
                            data::type_::ValueShapeId(16),
                        ]),
                        return_: data::type_::ValueShapeId(6),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 22..22,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(19),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 22..23,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(3),
                        ]),
                        return_: data::type_::ValueShapeId(21),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 23..25,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(17),
                            data::type_::ValueShapeId(17),
                        ]),
                        return_: data::type_::ValueShapeId(29),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 25..27,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(3),
                            data::type_::ValueShapeId(3),
                        ]),
                        return_: data::type_::ValueShapeId(29),
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
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                    data::graph::ParamLocal::BitArray(data::graph::BitArrayLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(2),
                            shape_id: data::type_::CustomValueShapeId(6),
                        },
                    }),
                    data::graph::ParamLocal::IntFunction {
                        local: data::graph::IntFunctionLocalId(0),
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        },
                    },
                    data::graph::ParamLocal::NeverFunction(data::graph::NeverFunctionLocal {
                        id: data::graph::NeverFunctionLocalId(0),
                        type_: data::type_::GenericFunctionType {
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[]),
                                return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                            },
                            shape: data::type_::FunctionShape {
                                shape_id: data::type_::ValueShapeId(26),
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                },
                            },
                        },
                    }),
                    data::graph::ParamLocal::Custom(data::graph::CustomLocal {
                        id: data::graph::CustomLocalId(0),
                        shape: data::type_::CustomValueShape {
                            type_id: data::type_::CustomTypeId(3),
                            shape_id: data::type_::CustomValueShapeId(7),
                        },
                    }),
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
                            data::type_::ValueType::List(data::type_::ListTypeId(3)),
                        ]),
                    },
                    data::graph::ParamLocal::Tuple {
                        local: data::graph::TupleLocalId(1),
                        type_: data::Storage::Static(&[
                            data::type_::ValueType::String,
                            data::type_::ValueType::List(data::type_::ListTypeId(4)),
                        ]),
                    },
                    data::graph::ParamLocal::Tuple {
                        local: data::graph::TupleLocalId(0),
                        type_: data::Storage::Static(&[
                            data::type_::ValueType::String,
                            data::type_::ValueType::List(data::type_::ListTypeId(4)),
                        ]),
                    },
                    data::graph::ParamLocal::Tuple {
                        local: data::graph::TupleLocalId(1),
                        type_: data::Storage::Static(&[
                            data::type_::ValueType::String,
                            data::type_::ValueType::List(data::type_::ListTypeId(4)),
                        ]),
                    },
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                ]),
            },
            list_types: data::type_::ListTypeTable {
                types: data::Storage::Static(&[
                    data::type_::ListStorageTypeId::Int(data::type_::IntListTypeId {
                        list_type: data::type_::ListTypeId(0),
                    }),
                    data::type_::ListStorageTypeId::Custom(data::type_::CustomListTypeId {
                        list_type: data::type_::ListTypeId(1),
                        item_type: data::type_::CustomTypeId(0),
                    }),
                    data::type_::ListStorageTypeId::Custom(data::type_::CustomListTypeId {
                        list_type: data::type_::ListTypeId(2),
                        item_type: data::type_::CustomTypeId(1),
                    }),
                    data::type_::ListStorageTypeId::BitArray(data::type_::BitArrayListTypeId {
                        list_type: data::type_::ListTypeId(3),
                    }),
                    data::type_::ListStorageTypeId::String(data::type_::StringListTypeId {
                        list_type: data::type_::ListTypeId(4),
                    }),
                ]),
                tuple_items: data::Storage::Static(&[]),
                function_items: data::Storage::Static(&[]),
                lifetimes: data::Storage::Static(&[
                    data::host::HostValueLifetime::LoadedOwner,
                    data::host::HostValueLifetime::LoadedOwner,
                    data::host::HostValueLifetime::LoadedOwner,
                    data::host::HostValueLifetime::LoadedOwner,
                    data::host::HostValueLifetime::LoadedOwner,
                ]),
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
                        native_visible: true,
                        lifetime: data::host::HostValueLifetime::LoadedOwner,
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
                                        type_: data::type_::ValueType::List(data::type_::ListTypeId(1)),
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
                        native_visible: true,
                        lifetime: data::host::HostValueLifetime::LoadedOwner,
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
                                        type_: data::type_::ValueType::List(data::type_::ListTypeId(2)),
                                        shape: data::type_::ValueShapeId(5),
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
                            package: data::Text::Static(""),
                            module: data::Text::Static("gleam"),
                            name: data::Text::Static("Result"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                                data::type_::TypeMetadata::String,
                            ]),
                        },
                        native_visible: true,
                        lifetime: data::host::HostValueLifetime::LoadedOwner,
                        constructor_count: 2,
                        constructors: data::Storage::Static(&[
                            data::type_::CustomConstructorDescriptor {
                                id: data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(2),
                                    index: 0,
                                },
                                name: data::Text::Static("Ok"),
                                native_tag: data::Text::Static("ok"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: None,
                                        type_: data::type_::ValueType::Int,
                                        shape: data::type_::ValueShapeId(17),
                                        refinement: data::type_::FieldRefinement::Argument(0),
                                    },
                                ]),
                            },
                            data::type_::CustomConstructorDescriptor {
                                id: data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(2),
                                    index: 1,
                                },
                                name: data::Text::Static("Error"),
                                native_tag: data::Text::Static("error"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: None,
                                        type_: data::type_::ValueType::String,
                                        shape: data::type_::ValueShapeId(3),
                                        refinement: data::type_::FieldRefinement::Argument(1),
                                    },
                                ]),
                            },
                        ]),
                    },
                    data::type_::CustomTypeDescriptor {
                        type_: data::type_::NominalTypeMetadata {
                            package: data::Text::Static(""),
                            module: data::Text::Static("gleam"),
                            name: data::Text::Static("Result"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                                data::type_::TypeMetadata::String,
                            ]),
                        },
                        native_visible: true,
                        lifetime: data::host::HostValueLifetime::LoadedOwner,
                        constructor_count: 2,
                        constructors: data::Storage::Static(&[
                            data::type_::CustomConstructorDescriptor {
                                id: data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(3),
                                    index: 0,
                                },
                                name: data::Text::Static("Ok"),
                                native_tag: data::Text::Static("ok"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: None,
                                        type_: data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                                        shape: data::type_::ValueShapeId(23),
                                        refinement: data::type_::FieldRefinement::Argument(0),
                                    },
                                ]),
                            },
                            data::type_::CustomConstructorDescriptor {
                                id: data::type_::CustomConstructorId {
                                    type_id: data::type_::CustomTypeId(3),
                                    index: 1,
                                },
                                name: data::Text::Static("Error"),
                                native_tag: data::Text::Static("error"),
                                fields: data::Storage::Static(&[
                                    data::type_::CustomFieldDescriptor {
                                        label: None,
                                        type_: data::type_::ValueType::String,
                                        shape: data::type_::ValueShapeId(3),
                                        refinement: data::type_::FieldRefinement::Argument(1),
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
                        native_access: None,
                        retention_lifetime: data::host::HostValueLifetime::LoadedOwner,
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
                lifetimes: data::Storage::Static(&[]),
                definitions: data::Storage::Static(&[]),
            },
            value_shapes: data::type_::ValueShapeTable {
                shapes: data::Storage::Static(&[
                    data::type_::ValueShapeDescriptor::BitArray,
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(0)),
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(1)),
                    data::type_::ValueShapeDescriptor::String,
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(1)),
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(4)),
                    data::type_::ValueShapeDescriptor::Bool,
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(2)),
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(7)),
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(3)),
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(4)),
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(10)),
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(5)),
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
                        data::type_::ValueShapeId(6),
                        data::type_::ValueShapeId(6),
                        data::type_::ValueShapeId(17),
                    ])),
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(3),
                    },
                    data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                        data::type_::ValueShapeId(6),
                        data::type_::ValueShapeId(3),
                    ])),
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(0),
                    },
                    data::type_::ValueShapeDescriptor::Parameter(data::type_::parameter_id(0)),
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(6)),
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(17),
                    },
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(23),
                    },
                    data::type_::ValueShapeDescriptor::Custom(data::type_::CustomValueShapeId(7)),
                    data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(17)),
                    data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                        data::type_::ValueShapeId(6),
                        data::type_::ValueShapeId(6),
                    ])),
                ]),
                shape_types: data::Storage::Static(&[
                    data::type_::ValueType::BitArray,
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::List(data::type_::ListTypeId(1)),
                    data::type_::ValueType::String,
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                    data::type_::ValueType::List(data::type_::ListTypeId(2)),
                    data::type_::ValueType::Bool,
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::List(data::type_::ListTypeId(1)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                    data::type_::ValueType::List(data::type_::ListTypeId(2)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                    data::type_::ValueType::List(data::type_::ListTypeId(3)),
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::BitArray,
                        data::type_::ValueType::List(data::type_::ListTypeId(3)),
                    ])),
                    data::type_::ValueType::List(data::type_::ListTypeId(4)),
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::String,
                        data::type_::ValueType::List(data::type_::ListTypeId(4)),
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
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::ValueType::BitArray),
                    }),
                    data::type_::ValueType::Parameter(data::type_::parameter_id(0)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(2)),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    }),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                    }),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(3)),
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::Bool,
                        data::type_::ValueType::Bool,
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
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(2),
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(17),
                            data::type_::ValueShapeId(3),
                        ]),
                        constructor: data::type_::CustomConstructorRefinement::Any,
                    },
                    data::type_::CustomValueShapeDescriptor {
                        type_id: data::type_::CustomTypeId(3),
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(23),
                            data::type_::ValueShapeId(3),
                        ]),
                        constructor: data::type_::CustomConstructorRefinement::Any,
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
            floats: data::Storage::Static(&[]),
            strings: data::Storage::Static(&[]),
            bit_arrays: data::Storage::Static(&[
                data::program::LibraryFunctionEntry {
                    function: data::function::BitArrayFunctionId(0),
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
                    function: data::function::BitArrayFunctionId(1),
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
            data::Export {
                name: data::Text::Static("bit_range"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::BitArray,
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::BitArray),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("bit_tail"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::BitArray,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::BitArray),
                },
                slot: 1,
            },
            data::Export {
                name: data::Text::Static("generic_results"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("list_callback"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                        data::type_::TypeMetadata::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                },
                slot: 0,
            },
        ]),
    },
    value_functions: data::Storage::Static(&[
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("main", "equal_native", data::source::SourceSpan::new(100, 136)),
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
                    shape: data::type_::ValueShapeId(9),
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
                    shape: data::type_::ValueShapeId(12),
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
                captures: data::Storage::Static(&[]),
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
                        }))), data::type_::ListTypeId(2)),
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
                                storage: data::type_::ListTypeId(2),
                                item: data::host::NativeConversionId(0),
                            },
                        },
                    ]),
                },
                callables: data::Storage::Static(&[]),
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
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
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
                            lifetime: data::host::HostValueLifetime::LoadedOwner,
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
                            access: data::host::HostCustomAccess::Declared,
                        },
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::String,
                        ]),
                    },
                ]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[
                    data::host::CustomSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("main"),
                        name: data::Text::Static("Tree"),
                        parameter_count: 1,
                        lifetime: data::host::HostValueLifetime::LoadedOwner,
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
                        access: data::host::HostCustomAccess::Declared,
                    },
                ]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: Some(data::Storage::Static(&[])),
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("main", "equal_native", data::source::SourceSpan::new(100, 136)),
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
                            data::type_::ValueType::List(data::type_::ListTypeId(3)),
                        ]),
                    }),
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::Tuple {
                        local: data::graph::TupleLocalId(1),
                        type_: data::Storage::Static(&[
                            data::type_::ValueType::String,
                            data::type_::ValueType::List(data::type_::ListTypeId(4)),
                        ]),
                    }),
                ]),
                captures: data::Storage::Static(&[]),
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
                        }))), data::type_::ListTypeId(2)),
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
                                storage: data::type_::ListTypeId(4),
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
                                storage: data::type_::ListTypeId(2),
                                item: data::host::NativeConversionId(1),
                            },
                        },
                    ]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::BitArray,
                        data::type_::ValueType::List(data::type_::ListTypeId(3)),
                    ])),
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::String,
                        data::type_::ValueType::List(data::type_::ListTypeId(4)),
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
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
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
                            lifetime: data::host::HostValueLifetime::LoadedOwner,
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
                            access: data::host::HostCustomAccess::Declared,
                        },
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::String,
                        ]),
                    },
                ]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[
                    data::host::CustomSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("main"),
                        name: data::Text::Static("Tree"),
                        parameter_count: 1,
                        lifetime: data::host::HostValueLifetime::LoadedOwner,
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
                        access: data::host::HostCustomAccess::Declared,
                    },
                ]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: Some(data::Storage::Static(&[])),
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("main", "fold", data::source::SourceSpan::new(182, 229)),
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
                captures: data::Storage::Static(&[]),
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
                callables: data::Storage::Static(&[]),
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
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
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
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("main", "equal_native", data::source::SourceSpan::new(100, 136)),
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
                            data::type_::ValueType::List(data::type_::ListTypeId(4)),
                        ]),
                    }),
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::Tuple {
                        local: data::graph::TupleLocalId(1),
                        type_: data::Storage::Static(&[
                            data::type_::ValueType::String,
                            data::type_::ValueType::List(data::type_::ListTypeId(4)),
                        ]),
                    }),
                ]),
                captures: data::Storage::Static(&[]),
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
                        }))), data::type_::ListTypeId(2)),
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
                                storage: data::type_::ListTypeId(4),
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
                                storage: data::type_::ListTypeId(2),
                                item: data::host::NativeConversionId(1),
                            },
                        },
                    ]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::String,
                        data::type_::ValueType::List(data::type_::ListTypeId(4)),
                    ])),
                    data::type_::ValueType::Tuple(data::Storage::Static(&[
                        data::type_::ValueType::String,
                        data::type_::ValueType::List(data::type_::ListTypeId(4)),
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
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
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
                            lifetime: data::host::HostValueLifetime::LoadedOwner,
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
                            access: data::host::HostCustomAccess::Declared,
                        },
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::String,
                        ]),
                    },
                ]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[
                    data::host::CustomSchema {
                        package: data::Text::Static("application"),
                        module: data::Text::Static("main"),
                        name: data::Text::Static("Tree"),
                        parameter_count: 1,
                        lifetime: data::host::HostValueLifetime::LoadedOwner,
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
                        access: data::host::HostCustomAccess::Declared,
                    },
                ]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: Some(data::Storage::Static(&[])),
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("main", "keep_bits", data::source::SourceSpan::new(279, 308)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::BitArray,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::BitArray),
            },
            type_arguments: data::Storage::Static(&[]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::BitArray(data::graph::BitArrayLocalId(0)),
                ]),
                captures: data::Storage::Static(&[]),
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
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::BitArray,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::BitArray),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 0,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::BitArray,
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::BitArray,
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::BitArray(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("main", "success", data::source::SourceSpan::new(361, 381)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                    package: data::Text::Static(""),
                    module: data::Text::Static("gleam"),
                    name: data::Text::Static("Result"),
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::String,
                    ]),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Int,
                    shape: data::type_::ValueShapeId(17),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static(""),
                            module: data::Text::Static("gleam"),
                            name: data::Text::Static("Result"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                                data::type_::TypeMetadata::String,
                            ]),
                        }), data::type_::CustomTypeId(2)),
                    ]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Int,
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(2))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Custom {
                    schema: data::host::CustomSchema {
                        package: data::Text::Static(""),
                        module: data::Text::Static("gleam"),
                        name: data::Text::Static("Result"),
                        parameter_count: 2,
                        lifetime: data::host::HostValueLifetime::LoadedOwner,
                        constructors: data::Storage::Static(&[
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Ok"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(0),
                                    },
                                ]),
                            },
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Error"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(1),
                                    },
                                ]),
                            },
                        ]),
                        access: data::host::HostCustomAccess::Declared,
                    },
                    arguments: data::Storage::Static(&[
                        data::host::RegistrationType::Parameter(0),
                        data::host::RegistrationType::String,
                    ]),
                },
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[
                    data::host::CustomSchema {
                        package: data::Text::Static(""),
                        module: data::Text::Static("gleam"),
                        name: data::Text::Static("Result"),
                        parameter_count: 2,
                        lifetime: data::host::HostValueLifetime::LoadedOwner,
                        constructors: data::Storage::Static(&[
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Ok"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(0),
                                    },
                                ]),
                            },
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Error"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(1),
                                    },
                                ]),
                            },
                        ]),
                        access: data::host::HostCustomAccess::Declared,
                    },
                ]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("main", "failure", data::source::SourceSpan::new(443, 474)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                    package: data::Text::Static(""),
                    module: data::Text::Static("gleam"),
                    name: data::Text::Static("Result"),
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                        data::type_::TypeMetadata::String,
                    ]),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Int,
                    shape: data::type_::ValueShapeId(17),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Function {
                        local: data::graph::ParamLocal::IntFunction {
                            local: data::graph::IntFunctionLocalId(0),
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            },
                        },
                        arity: 0,
                    },
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static(""),
                            module: data::Text::Static("gleam"),
                            name: data::Text::Static("Result"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                                data::type_::TypeMetadata::String,
                            ]),
                        }), data::type_::CustomTypeId(2)),
                    ]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(2))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::host::RegistrationType::Parameter(0)),
                    },
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Custom {
                    schema: data::host::CustomSchema {
                        package: data::Text::Static(""),
                        module: data::Text::Static("gleam"),
                        name: data::Text::Static("Result"),
                        parameter_count: 2,
                        lifetime: data::host::HostValueLifetime::LoadedOwner,
                        constructors: data::Storage::Static(&[
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Ok"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(0),
                                    },
                                ]),
                            },
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Error"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(1),
                                    },
                                ]),
                            },
                        ]),
                        access: data::host::HostCustomAccess::Declared,
                    },
                    arguments: data::Storage::Static(&[
                        data::host::RegistrationType::Parameter(0),
                        data::host::RegistrationType::String,
                    ]),
                },
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Function {
                        slot: 0,
                        arity: 0,
                    },
                ]),
                custom_schemas: data::Storage::Static(&[
                    data::host::CustomSchema {
                        package: data::Text::Static(""),
                        module: data::Text::Static("gleam"),
                        name: data::Text::Static("Result"),
                        parameter_count: 2,
                        lifetime: data::host::HostValueLifetime::LoadedOwner,
                        constructors: data::Storage::Static(&[
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Ok"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(0),
                                    },
                                ]),
                            },
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Error"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(1),
                                    },
                                ]),
                            },
                        ]),
                        access: data::host::HostCustomAccess::Declared,
                    },
                ]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("application"),
            site: data::source::HostCallSite::from_static("main", "failure", data::source::SourceSpan::new(443, 474)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0))),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                    package: data::Text::Static(""),
                    module: data::Text::Static("gleam"),
                    name: data::Text::Static("Result"),
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                        data::type_::TypeMetadata::String,
                    ]),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                    shape: data::type_::ValueShapeId(23),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::Function {
                        local: data::graph::ParamLocal::NeverFunction(data::graph::NeverFunctionLocal {
                            id: data::graph::NeverFunctionLocalId(0),
                            type_: data::type_::GenericFunctionType {
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                },
                                shape: data::type_::FunctionShape {
                                    shape_id: data::type_::ValueShapeId(26),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                                    },
                                },
                            },
                        }),
                        arity: 0,
                    },
                ]),
                captures: data::Storage::Static(&[]),
            },
            constructions: data::host::HostConstructionTypes {
                lists: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                customs: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {
                            package: data::Text::Static(""),
                            module: data::Text::Static("gleam"),
                            name: data::Text::Static("Result"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Parameter(data::type_::parameter_id(0)),
                                data::type_::TypeMetadata::String,
                            ]),
                        }), data::type_::CustomTypeId(3)),
                    ]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(3))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Function {
                        arguments: data::Storage::Static(&[]),
                        return_: data::Storage::Static(&data::host::RegistrationType::Parameter(0)),
                    },
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::Custom {
                    schema: data::host::CustomSchema {
                        package: data::Text::Static(""),
                        module: data::Text::Static("gleam"),
                        name: data::Text::Static("Result"),
                        parameter_count: 2,
                        lifetime: data::host::HostValueLifetime::LoadedOwner,
                        constructors: data::Storage::Static(&[
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Ok"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(0),
                                    },
                                ]),
                            },
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Error"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(1),
                                    },
                                ]),
                            },
                        ]),
                        access: data::host::HostCustomAccess::Declared,
                    },
                    arguments: data::Storage::Static(&[
                        data::host::RegistrationType::Parameter(0),
                        data::host::RegistrationType::String,
                    ]),
                },
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Function {
                        slot: 0,
                        arity: 0,
                    },
                ]),
                custom_schemas: data::Storage::Static(&[
                    data::host::CustomSchema {
                        package: data::Text::Static(""),
                        module: data::Text::Static("gleam"),
                        name: data::Text::Static("Result"),
                        parameter_count: 2,
                        lifetime: data::host::HostValueLifetime::LoadedOwner,
                        constructors: data::Storage::Static(&[
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Ok"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(0),
                                    },
                                ]),
                            },
                            data::host::ConstructorSchema {
                                name: data::Text::Static("Error"),
                                fields: data::Storage::Static(&[
                                    data::host::FieldSchema {
                                        label: None,
                                        type_: data::host::SchemaType::Parameter(1),
                                    },
                                ]),
                            },
                        ]),
                        access: data::host::HostCustomAccess::Declared,
                    },
                ]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                restorations: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
                native_sources: data::Storage::Static(&[]),
            }),
            native_view: None,
        },
    ]),
    never_functions: data::Storage::Static(&[]),
    callables: data::Storage::Static(&[]),
}
