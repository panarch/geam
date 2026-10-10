data::HostedEntryArtifact {
    format: 30,
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
                            parameter_count: 0,
                        },
                        body: data::function::ProfiledFunctionBody {
                            block_graph: data::graph::ProfiledBlockGraph {
                                entry: data::graph::BlockId(0),
                                blocks: data::Storage::Static(&[
                                    data::graph::BlockHeader {
                                        params: 0..0,
                                        instructions: 0..2,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                ]),
                                params: data::Storage::Static(&[]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("λλλ"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                            sign: data::Sign::Plus,
                                            digits: data::Storage::Static(&[
                                                4,
                                            ]),
                                        })),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::IntFunctionId(1),
                                        site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(2203, 2221)),
                                    },
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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

            fn string_int_0(
                point: usize,
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                    3
                ] = [
                    |values, budget| CompiledResume::Exit(string_int_0_entry((), values, budget)),
                    string_int_0_resume_1,
                    string_int_0_resume_2,
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
                inputs: (),
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {
                let () = inputs;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[]);
                    return data::compiled::CompiledProgress::Yield(0);
                }
                *budget -= 1;
                let b0_s0 = data::compiled::string::StringRange::literal("λλλ");
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0]);
                    return data::compiled::CompiledProgress::Yield(1);
                }
                *budget -= 1;
                let b0_i0 = 4_i128;
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0]);
                    return data::compiled::CompiledProgress::Yield(2);
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b0_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b0_s0]);
                data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
            }

            fn string_int_0_resume_1(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b0_s0,) = (values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(1));
                }
                *budget -= 1;
                let b0_i0 = 4_i128;

                values.ints.clear();
                values.ints.extend_from_slice(&[b0_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b0_s0]);
                CompiledResume::Next(2)
            }

            fn string_int_0_resume_2(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b0_i0, b0_s0,) = (values.ints[0], values.strings[0],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[b0_i0]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[b0_i0]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b0_s0]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }

            fn string_int_1(
                point: usize,
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                    9
                ] = [
                    |values, budget| CompiledResume::Exit(string_int_1_entry((values.ints[0], values.strings[0],), values, budget)),
                    string_int_1_resume_1,
                    string_int_1_resume_2,
                    string_int_1_resume_3,
                    string_int_1_resume_4,
                    string_int_1_resume_5,
                    string_int_1_resume_6,
                    string_int_1_resume_7,
                    string_int_1_resume_8,
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

            fn string_int_1_resume_1(
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

            fn string_int_1_resume_2(
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

            fn string_int_1_resume_3(
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

            fn string_int_1_resume_4(
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

            fn string_int_1_resume_5(
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

            fn string_int_1_resume_6(
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

            fn string_int_1_resume_7(
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

            fn string_int_1_resume_8(
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
                                    block: data::graph::BlockId(0),
                                    instruction: 2,
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
                            run: string_int_1,
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
                0..2,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
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
                        data::type_::ValueShapeId(0),
                        data::type_::ValueShapeId(1),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
            ]),
            parameters: data::Storage::Static(&[
                data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
            ]),
        },
        list_types: data::type_::ListTypeTable {
            types: data::Storage::Static(&[]),
            tuple_items: data::Storage::Static(&[]),
            function_items: data::Storage::Static(&[]),
            lifetimes: data::Storage::Static(&[]),
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
                data::type_::ValueShapeDescriptor::String,
                data::type_::ValueShapeDescriptor::Int,
            ]),
            shape_types: data::Storage::Static(&[
                data::type_::ValueType::String,
                data::type_::ValueType::Int,
            ]),
            custom_shapes: data::Storage::Static(&[]),
        },
    },
    value_functions: data::Storage::Static(&[]),
    never_functions: data::Storage::Static(&[]),
}
