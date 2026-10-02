data::HostedModuleArtifact {
    module: data::ModuleArtifact {
        format: 12,
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
                                                site: data::source::HostCallSite::from_static("main", "bit_range", data::source::SourceSpan::new(1818, 1824)),
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
                                            site: data::source::HostCallSite::from_static("main", "bit_range", data::source::SourceSpan::new(1681, 1700)),
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
                                            site: data::source::HostCallSite::from_static("main", "bit_range", data::source::SourceSpan::new(1808, 1825)),
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
                                                site: data::source::PanicSite::from_static("main", "bit_tail", data::source::SourceSpan::new(1890, 1900)),
                                                pattern_span: data::source::SourceSpan::new(1901, 1919),
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
                                            site: data::source::HostCallSite::from_static("main", "bit_tail", data::source::SourceSpan::new(1930, 1945)),
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
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(2)),
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
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(3235, 3245)),
                                                pattern_span: data::source::SourceSpan::new(3246, 3250),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 41..42,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(3197, 3207)),
                                                pattern_span: data::source::SourceSpan::new(3208, 3213),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 42..43,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(3160, 3170)),
                                                pattern_span: data::source::SourceSpan::new(3171, 3176),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 43..44,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(3127, 3137)),
                                                pattern_span: data::source::SourceSpan::new(3138, 3142),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 44..45,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(3091, 3101)),
                                                pattern_span: data::source::SourceSpan::new(3102, 3106),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 45..46,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(3053, 3063)),
                                                pattern_span: data::source::SourceSpan::new(3064, 3069),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 46..47,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(3016, 3026)),
                                                pattern_span: data::source::SourceSpan::new(3027, 3032),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 47..48,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2975, 2985)),
                                                pattern_span: data::source::SourceSpan::new(2986, 2990),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 48..49,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2939, 2949)),
                                                pattern_span: data::source::SourceSpan::new(2950, 2954),
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
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2884, 2894)),
                                                pattern_span: data::source::SourceSpan::new(2895, 2909),
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
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2830, 2840)),
                                                pattern_span: data::source::SourceSpan::new(2841, 2855),
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
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2780, 2790)),
                                                pattern_span: data::source::SourceSpan::new(2791, 2805),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 52..53,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2589, 2599)),
                                                pattern_span: data::source::SourceSpan::new(2600, 2604),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 53..54,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2532, 2542)),
                                                pattern_span: data::source::SourceSpan::new(2543, 2547),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 54..55,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2444, 2454)),
                                                pattern_span: data::source::SourceSpan::new(2455, 2505),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 55..56,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2415, 2425)),
                                                pattern_span: data::source::SourceSpan::new(2426, 2427),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 56..57,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2386, 2396)),
                                                pattern_span: data::source::SourceSpan::new(2397, 2398),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 57..58,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2361, 2371)),
                                                pattern_span: data::source::SourceSpan::new(2372, 2374),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 58..59,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2336, 2346)),
                                                pattern_span: data::source::SourceSpan::new(2347, 2349),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 59..60,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2306, 2316)),
                                                pattern_span: data::source::SourceSpan::new(2317, 2318),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 60..61,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2233, 2243)),
                                                pattern_span: data::source::SourceSpan::new(2244, 2269),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 61..62,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2179, 2189)),
                                                pattern_span: data::source::SourceSpan::new(2190, 2216),
                                            }),
                                        },
                                        data::graph::BlockHeader {
                                            params: 62..63,
                                            instructions: 33..33,
                                            terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                message: None,
                                                site: data::source::PanicSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2126, 2136)),
                                                pattern_span: data::source::SourceSpan::new(2137, 2162),
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
                                            shape: data::type_::ValueShapeId(28),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Bool,
                                                    data::type_::ValueType::Bool,
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(28),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Tuple {
                                                local: data::graph::TupleLocalId(0),
                                                type_: data::Storage::Static(&[
                                                    data::type_::ValueType::Bool,
                                                    data::type_::ValueType::Bool,
                                                ]),
                                            },
                                            shape: data::type_::ValueShapeId(28),
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
                                                shape: data::type_::ValueShapeId(28),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Call {
                                                function: data::function::TupleFunctionId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2808, 2827)),
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
                                                shape: data::type_::ValueShapeId(28),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Call {
                                                function: data::function::TupleFunctionId(2),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2858, 2881)),
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
                                                shape: data::type_::ValueShapeId(28),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Tuple(data::graph::TupleInstruction::Call {
                                                function: data::function::TupleFunctionId(3),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("main", "integer_comparisons", data::source::SourceSpan::new(2912, 2936)),
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
                                                        list_type: data::type_::ListTypeId(0),
                                                        item_type: data::type_::CustomTypeId(0),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(8),
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
                                                            list_type: data::type_::ListTypeId(0),
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
                                                            list_type: data::type_::ListTypeId(0),
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
                                                        list_type: data::type_::ListTypeId(1),
                                                        item_type: data::type_::CustomTypeId(1),
                                                    },
                                                }),
                                                shape: data::type_::ValueShapeId(11),
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
                                                            list_type: data::type_::ListTypeId(1),
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
                                                            list_type: data::type_::ListTypeId(1),
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
                                                },
                                            ]))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(0)),
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
                                                site: data::source::PanicSite::from_static("main", "substring", data::source::SourceSpan::new(1418, 1428)),
                                                pattern_span: data::source::SourceSpan::new(1429, 1446),
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
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::FunctionCall {
                                                function: data::graph::StringFunctionLocalId(0),
                                                args: data::Storage::Static(&[]),
                                                site: data::source::HostCallSite::from_static("main", "substring", data::source::SourceSpan::new(1518, 1524)),
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
                                                site: data::source::HostCallSite::from_static("main", "substring", data::source::SourceSpan::new(1527, 1533)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
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
                                                site: data::source::HostCallSite::from_static("main", "substring", data::source::SourceSpan::new(1486, 1536)),
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
                                                site: data::source::HostCallSite::from_static("main", "substring", data::source::SourceSpan::new(1538, 1544)),
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
                                                shape: data::type_::ValueShapeId(28),
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
                                                shape: data::type_::ValueShapeId(28),
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
            numeric: {
enum NumericResume {
Next(usize),
Exit(data::numeric::NumericProgress),
}
fn numeric_int_0(point: usize, values: &mut data::numeric::NumericValues, budget: &mut usize) -> data::numeric::NumericProgress {
const RESUME: [fn(&mut data::numeric::NumericValues, &mut usize) -> NumericResume; 5] = [
|values, budget| NumericResume::Exit(numeric_int_0_entry((values.ints[0],), values, budget)),
numeric_int_0_resume_1,
numeric_int_0_resume_2,
numeric_int_0_resume_3,
numeric_int_0_resume_4,
];
let mut point = point;
loop {
match RESUME[point](values, budget) {
NumericResume::Next(next) => point = next,
NumericResume::Exit(progress) => return progress,
}
}
}
fn numeric_int_0_entry(inputs: (i128,), values: &mut data::numeric::NumericValues, budget: &mut usize) -> data::numeric::NumericProgress {
let (b0_i0,) = inputs;
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[b0_i0]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return data::numeric::NumericProgress::Yield(0);
}
*budget -= 1;
if b0_i0 >= 0_i128 {
let (b1_i0,) = (b0_i0,);
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[b1_i0]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return data::numeric::NumericProgress::Yield(1);
}
*budget -= 1;
let b1_i1 = b1_i0 + 1_i128;
if b1_i1 < i128::from(i64::MIN) || b1_i1 > i128::from(i64::MAX) {
values.ints.clear();
values.ints.extend_from_slice(&[b1_i0, b1_i1]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return data::numeric::NumericProgress::Interpreted(2);
}
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[b1_i0, b1_i1]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return data::numeric::NumericProgress::Yield(2);
}
*budget -= 1;
values.ints.clear();
values.ints.extend_from_slice(&[b1_i0, b1_i1]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
data::numeric::NumericProgress::Complete(data::graph::BlockGraphExitId(0))
} else {
let (b2_i0,) = (b0_i0,);
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[b2_i0]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return data::numeric::NumericProgress::Yield(3);
}
*budget -= 1;
let b2_i1 = b2_i0 - 1_i128;
if b2_i1 < i128::from(i64::MIN) || b2_i1 > i128::from(i64::MAX) {
values.ints.clear();
values.ints.extend_from_slice(&[b2_i0, b2_i1]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return data::numeric::NumericProgress::Interpreted(4);
}
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[b2_i0, b2_i1]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return data::numeric::NumericProgress::Yield(4);
}
*budget -= 1;
values.ints.clear();
values.ints.extend_from_slice(&[b2_i0, b2_i1]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
data::numeric::NumericProgress::Complete(data::graph::BlockGraphExitId(1))
}
}
fn numeric_int_0_resume_1(values: &mut data::numeric::NumericValues, budget: &mut usize) -> NumericResume {
let (b1_i0,) = (values.ints[0],);
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[b1_i0]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return NumericResume::Exit(data::numeric::NumericProgress::Yield(1));
}
*budget -= 1;
let b1_i1 = b1_i0 + 1_i128;
if b1_i1 < i128::from(i64::MIN) || b1_i1 > i128::from(i64::MAX) {
values.ints.clear();
values.ints.extend_from_slice(&[b1_i0, b1_i1]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return NumericResume::Exit(data::numeric::NumericProgress::Interpreted(2));
}
values.ints.clear();
values.ints.extend_from_slice(&[b1_i0, b1_i1]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
NumericResume::Next(2)
}
fn numeric_int_0_resume_2(values: &mut data::numeric::NumericValues, budget: &mut usize) -> NumericResume {
let (b1_i0, b1_i1,) = (values.ints[0], values.ints[1],);
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[b1_i0, b1_i1]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return NumericResume::Exit(data::numeric::NumericProgress::Yield(2));
}
*budget -= 1;
values.ints.clear();
values.ints.extend_from_slice(&[b1_i0, b1_i1]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
NumericResume::Exit(data::numeric::NumericProgress::Complete(data::graph::BlockGraphExitId(0)))
}
fn numeric_int_0_resume_3(values: &mut data::numeric::NumericValues, budget: &mut usize) -> NumericResume {
let (b2_i0,) = (values.ints[0],);
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[b2_i0]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return NumericResume::Exit(data::numeric::NumericProgress::Yield(3));
}
*budget -= 1;
let b2_i1 = b2_i0 - 1_i128;
if b2_i1 < i128::from(i64::MIN) || b2_i1 > i128::from(i64::MAX) {
values.ints.clear();
values.ints.extend_from_slice(&[b2_i0, b2_i1]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return NumericResume::Exit(data::numeric::NumericProgress::Interpreted(4));
}
values.ints.clear();
values.ints.extend_from_slice(&[b2_i0, b2_i1]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
NumericResume::Next(4)
}
fn numeric_int_0_resume_4(values: &mut data::numeric::NumericValues, budget: &mut usize) -> NumericResume {
let (b2_i0, b2_i1,) = (values.ints[0], values.ints[1],);
if *budget == 0 {
values.ints.clear();
values.ints.extend_from_slice(&[b2_i0, b2_i1]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
return NumericResume::Exit(data::numeric::NumericProgress::Yield(4));
}
*budget -= 1;
values.ints.clear();
values.ints.extend_from_slice(&[b2_i0, b2_i1]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
NumericResume::Exit(data::numeric::NumericProgress::Complete(data::graph::BlockGraphExitId(1)))
}
data::numeric::NumericFunctions {
ints: data::Storage::Static(&[
data::numeric::NumericFunction { function: data::function::IntFunctionId(0), implementation: data::numeric::NumericImplementation { entry: 0, checkpoints: data::Storage::Static(&[
    data::numeric::NumericCheckpoint {
        block: data::graph::BlockId(0),
        instruction: 0,
        ints: 1,
        bools: 0,
    },
    data::numeric::NumericCheckpoint {
        block: data::graph::BlockId(1),
        instruction: 0,
        ints: 1,
        bools: 0,
    },
    data::numeric::NumericCheckpoint {
        block: data::graph::BlockId(1),
        instruction: 1,
        ints: 2,
        bools: 0,
    },
    data::numeric::NumericCheckpoint {
        block: data::graph::BlockId(2),
        instruction: 0,
        ints: 1,
        bools: 0,
    },
    data::numeric::NumericCheckpoint {
        block: data::graph::BlockId(2),
        instruction: 1,
        ints: 2,
        bools: 0,
    },
]), run: numeric_int_0 } },
]),
bools: data::Storage::Static(&[
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
                    1..5,
                    0..0,
                    5..6,
                    6..10,
                    0..0,
                    10..15,
                    0..0,
                    15..20,
                    0..0,
                    20..24,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
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
                        return_: data::type_::ValueShapeId(17),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 3..3,
                        parameter_shapes: data::Storage::Static(&[]),
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
                        parameters: 3..6,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                            data::type_::ValueShapeId(17),
                            data::type_::ValueShapeId(17),
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
                        parameters: 8..8,
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
                        parameters: 8..9,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(17),
                        ]),
                        return_: data::type_::ValueShapeId(24),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 9..10,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(24),
                        ]),
                        return_: data::type_::ValueShapeId(24),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 10..11,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(25),
                        ]),
                        return_: data::type_::ValueShapeId(24),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 11..12,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(26),
                        ]),
                        return_: data::type_::ValueShapeId(27),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 12..13,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(27),
                        ]),
                        return_: data::type_::ValueShapeId(27),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 13..13,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(6),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 13..13,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(6),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 13..15,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(9),
                            data::type_::ValueShapeId(12),
                        ]),
                        return_: data::type_::ValueShapeId(6),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 15..17,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(14),
                            data::type_::ValueShapeId(16),
                        ]),
                        return_: data::type_::ValueShapeId(6),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 17..19,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(16),
                            data::type_::ValueShapeId(16),
                        ]),
                        return_: data::type_::ValueShapeId(6),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 19..19,
                        parameter_shapes: data::Storage::Static(&[]),
                        return_: data::type_::ValueShapeId(19),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 19..20,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(3),
                        ]),
                        return_: data::type_::ValueShapeId(21),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 20..22,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(17),
                            data::type_::ValueShapeId(17),
                        ]),
                        return_: data::type_::ValueShapeId(28),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 22..24,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(3),
                            data::type_::ValueShapeId(3),
                        ]),
                        return_: data::type_::ValueShapeId(28),
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
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
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
                    data::type_::ValueShapeDescriptor::Tuple(data::Storage::Static(&[
                        data::type_::ValueShapeId(6),
                        data::type_::ValueShapeId(6),
                    ])),
                ]),
                shape_types: data::Storage::Static(&[
                    data::type_::ValueType::BitArray,
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                    data::type_::ValueType::String,
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                    data::type_::ValueType::List(data::type_::ListTypeId(1)),
                    data::type_::ValueType::Bool,
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::List(data::type_::ListTypeId(0)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(0)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
                    data::type_::ValueType::List(data::type_::ListTypeId(1)),
                    data::type_::ValueType::Custom(data::type_::CustomTypeId(1)),
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
            ints: data::Storage::Static(&[]),
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
                            shared: false,
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
                        shared: false,
                    },
                ]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: Some(data::Storage::Static(&[])),
            }),
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
                callables: data::Storage::Static(&[]),
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
                            shared: false,
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
                        shared: false,
                    },
                ]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: Some(data::Storage::Static(&[])),
            }),
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
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
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
                callables: data::Storage::Static(&[]),
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
                            shared: false,
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
                        shared: false,
                    },
                ]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: Some(data::Storage::Static(&[])),
            }),
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
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
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
                        shared: false,
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
                        shared: false,
                    },
                ]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
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
                        shared: false,
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
                        shared: false,
                    },
                ]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
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
                        shared: false,
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
                        shared: false,
                    },
                ]),
                external_schemas: data::Storage::Static(&[]),
                constructions: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
    ]),
    never_functions: data::Storage::Static(&[]),
    callables: data::Storage::Static(&[]),
}
