data::HostedModuleArtifact {
    module: data::ModuleArtifact {
        format: 28,
        program: data::ProgramTables {
            root: data::source::module_id(0),
            modules: data::Storage::Static(&[
                data::program::ExecutionModuleContext {
                    module: data::Text::Static("example"),
                    source_context: Some(data::source::SourceContext::from_static_block("src/call_boundaries.gleam", r#"
fn keep(value: String) -> String {
  value
}

fn spin(flag: Bool) -> Bool {
  spin(flag)
}

pub fn choose(flag: Bool) -> String {
  let value = keep("kept")
  case flag {
    True -> {
      let _ = spin(True)
      value
    }
    False -> value
  }
}

fn identity(value: Int) -> Int {
  value
}

pub fn wide() -> Int {
  let calculate = identity
  let value = calculate(1_099_511_627_776)
  value * value * value * value
}
"#)),
                },
            ]),
            main: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::String(data::function::StringFunctionId(0))),
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
                                            instructions: 0..4,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[]),
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
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                },
                                                family: data::function::FunctionReturnFamily::Int,
                                                kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(1))),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    0,
                                                    256,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                shape: data::type_::ValueShapeId(2),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::FunctionCall {
                                                function: data::graph::IntFunctionLocalId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "wide", data::source::SourceSpan::new(362, 390)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::IntegerRegion(data::graph::ArithmeticRegion {
                                            inputs: data::Storage::Static(&[
                                                data::graph::IntLocalId(1),
                                            ]),
                                            nodes: data::Storage::Static(&[
                                                data::graph::ArithmeticNode::Multiply(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Input(0)),
                                                data::graph::ArithmeticNode::Multiply(data::graph::ArithmeticOperand::Value(0), data::graph::ArithmeticOperand::Input(0)),
                                                data::graph::ArithmeticNode::Multiply(data::graph::ArithmeticOperand::Value(1), data::graph::ArithmeticOperand::Input(0)),
                                            ]),
                                            outputs: data::Storage::Static(&[
                                                data::graph::ArithmeticOutput {
                                                    value: 2,
                                                    slot: data::graph::ParamSlot {
                                                        local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                                                        shape: data::type_::ValueShapeId(2),
                                                    },
                                                },
                                            ]),
                                            native: false,
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
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::IntLocalId(0)),
                                ]),
                            },
                        })),
                    ]),
                    float_functions: data::Storage::Static(&[]),
                    string_functions: data::Storage::Static(&[
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
                                            terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                                subject: data::graph::BoolLocalId(0),
                                                true_: data::graph::Edge {
                                                    target: data::graph::BlockId(1),
                                                    args: data::Storage::Static(&[
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
                                                            data::graph::FamilyTransfer {
                                                                family: data::graph::StorageFamily::Bool,
                                                                length: 0,
                                                                steps: data::Storage::Static(&[]),
                                                            },
                                                        ]),
                                                    },
                                                },
                                                false_: data::graph::Edge {
                                                    target: data::graph::BlockId(2),
                                                    args: data::Storage::Static(&[
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
                                            params: 1..2,
                                            instructions: 2..4,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                        data::graph::BlockHeader {
                                            params: 2..3,
                                            instructions: 4..4,
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(1)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("kept"))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                                shape: data::type_::ValueShapeId(1),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Call {
                                                function: data::function::StringFunctionId(1),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "choose", data::source::SourceSpan::new(144, 156)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                                function: data::function::BoolFunctionId(0),
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("example", "choose", data::source::SourceSpan::new(199, 209)),
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::Return(data::graph::StringLocalId(0)),
                                    data::function::FunctionExit::Return(data::graph::StringLocalId(0)),
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
                                            terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                        },
                                    ]),
                                    params: data::Storage::Static(&[
                                        data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
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
                                            instructions: 0..0,
                                            terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                                edge: data::graph::Edge {
                                                    target: data::graph::BlockId(0),
                                                    args: data::Storage::Static(&[
                                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                    ]),
                                    instructions: data::Storage::Static(&[]),
                                },
                                exits: data::Storage::Static(&[]),
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
                const CALL_GROUP_0: [data::compiled::calls::CallStart; 2] = {
                    use data::compiled::calls::{CallArguments, CallCaptureInputs, CallExecution, CallInputs, CallInteger, CallOps, CallOutput, CallProgress, CallStorage, CallValues, IntCallable};
                    enum FunctionState {
                        Int0Point0 {  },
                        Int0Point1 { int_function0: IntCallable },
                        Int0Point2 { int_function0: IntCallable, int0: i128 },
                        Int0Point3 { int_function0: IntCallable, int0: i128, int1: i128 },
                        Int1Point0 { int0: i128 },
                        Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                    }
                    enum IntReturn {
                        Int0Call2 { int_function0: IntCallable, int0: i128 },
                    }
                    impl IntReturn {
                        fn site(&self) -> data::source::HostCallSite {
                            match *self {
                                Self::Int0Call2 { .. } => data::source::HostCallSite::from_static("example", "wide", data::source::SourceSpan::new(362, 390)),
                            }
                        }
                        fn small(self, result: i128) -> FunctionState {
                            match self {
                                Self::Int0Call2 { int_function0, int0 } => {
                                    let int1 = result;
                                    FunctionState::Int0Point3 { int_function0, int0, int1 }
                                },
                            }
                        }
                        fn resume(self, result: CallInteger) -> FunctionState {
                            if let Some(result) = result.small() {
                                return self.small(result);
                            }
                            match self {
                                Self::Int0Call2 { int_function0, int0 } => {
                                    let int1 = result;
                                    FunctionState::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 3,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { ints: vec![int0.into(), int1], int_functions: vec![int_function0], ..CallValues::default() }) }
                                },
                            }
                        }
                    }
                    #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                    enum FunctionStep {
                        Yield(FunctionState),
                        Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                        IntCall { callee: FunctionState, caller: IntReturn },
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
                                data::compiled::CallTarget::Int(data::function::IntFunctionId(0)) => calls_int_0_state(point, values),
                                data::compiled::CallTarget::Int(data::function::IntFunctionId(1)) => calls_int_1_state(point, values),
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
                                    FunctionStep::IntCall { callee, caller } => {
                                        self.integer_returns.push(caller);
                                        active = callee;
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
                            FunctionState::Int0Point0 {  } => calls_int_0_run(Int0State::Point0 {  }, ops, budget),
                            FunctionState::Int0Point1 { int_function0 } => calls_int_0_run(Int0State::Point1 { int_function0 }, ops, budget),
                            FunctionState::Int0Point2 { int_function0, int0 } => calls_int_0_run(Int0State::Point2 { int_function0, int0 }, ops, budget),
                            FunctionState::Int0Point3 { int_function0, int0, int1 } => calls_int_0_run(Int0State::Point3 { int_function0, int0, int1 }, ops, budget),
                            FunctionState::Int1Point0 { int0 } => calls_int_1_run(Int1State::Point0 { int0 }, ops, budget),
                        }
                    }
                    enum Int0State {
                        Point0 {  },
                        Point1 { int_function0: IntCallable },
                        Point2 { int_function0: IntCallable, int0: i128 },
                        Point3 { int_function0: IntCallable, int0: i128, int1: i128 },
                    }
                    fn calls_int_0_run(mut active: Int0State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        loop {
                            match active {
                                Int0State::Point0 {  } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point0 {  }); }
                                    *budget -= 1;
                                    let int_function0 = ops.int_reference(data::function::IntFunctionId(1), data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    });
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point1 { int_function0 }); }
                                    *budget -= 1;
                                    let int0 = 1099511627776_i128;
                                    if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { ints: vec![int0.into()], int_functions: vec![int_function0], ..CallValues::default() }) }; }
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point2 { int_function0, int0 }); }
                                    *budget -= 1;
                                    return {
                                        let callable = &int_function0;
                                        let captures = callable.captures();
                                        let target = callable.target();
                                        if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (int0,)) {
                                            return FunctionStep::IntCall { callee, caller: IntReturn::Int0Call2 { int_function0, int0 } };
                                        }
                                        FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("example", "wide", data::source::SourceSpan::new(362, 390)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int0.into()], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: IntReturn::Int0Call2 { int_function0, int0 } }
                                    };
                                },
                                Int0State::Point1 { int_function0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point1 { int_function0 }); }
                                    *budget -= 1;
                                    let int0 = 1099511627776_i128;
                                    if int0 < i128::from(i64::MIN) || int0 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 1,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { ints: vec![int0.into()], int_functions: vec![int_function0], ..CallValues::default() }) }; }
                                    active = Int0State::Point2 { int_function0, int0 };
                                    continue;
                                },
                                Int0State::Point2 { int_function0, int0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point2 { int_function0, int0 }); }
                                    *budget -= 1;
                                    return {
                                        let callable = &int_function0;
                                        let captures = callable.captures();
                                        let target = callable.target();
                                        if ops.belongs_to_execution(&captures) && let Some(callee) = calls_entry_0(target, &captures, (int0,)) {
                                            return FunctionStep::IntCall { callee, caller: IntReturn::Int0Call2 { int_function0, int0 } };
                                        }
                                        FunctionStep::IntBridge { function: target, site: data::source::HostCallSite::from_static("example", "wide", data::source::SourceSpan::new(362, 390)), arguments: CallArguments { values: Box::new(CallValues { ints: vec![int0.into()], ..CallValues::default() }), captures: Some(captures.retain()) }, caller: IntReturn::Int0Call2 { int_function0, int0 } }
                                    };
                                },
                                Int0State::Point3 { int_function0, int0, int1 } => {
                                    return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                        block: data::graph::BlockId(0),
                                        instruction: 3,
                                        ints: 2,
                                        bools: 0,
                                        bit_arrays: 0,
                                        int_lists: 0,
                                        strings: 0,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 1,
                                        bool_functions: 0,
                                    }, values: Box::new(CallValues { ints: vec![int0.into(), int1.into()], int_functions: vec![int_function0], ..CallValues::default() }) };
                                },
                            }
                        }
                    }
                    enum Int1State {
                        Point0 { int0: i128 },
                    }
                    fn calls_int_1_run(active: Int1State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            Int1State::Point0 { int0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::Int1Point0 { int0 }); }
                                *budget -= 1;
                                {
                                    FunctionStep::Int { value: int0 }
                                }
                            },
                        }
                    }
                    fn calls_entry_0(target: data::function::IntFunctionId, _captures: &CallCaptureInputs<'_>, inputs: (i128,)) -> Option<FunctionState> {
                        let (argument0,) = inputs;
                        match target.0 {
                            1 => Some(FunctionState::Int1Point0 { int0: argument0 }),
                            _ => None,
                        }
                    }
                    fn calls_int_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                        let active = match point {
                            0 => FunctionState::Int0Point0 {  },
                            1 => FunctionState::Int0Point1 { int_function0: values.int_function(0)? },
                            2 => {
                                if !matches!(values.int_function_target(0)?, data::function::IntFunctionId(1)) { return None; }
                                FunctionState::Int0Point2 { int_function0: values.int_function(0)?, int0: values.int(0)? }
                            },
                            3 => FunctionState::Int0Point3 { int_function0: values.int_function(0)?, int0: values.int(0)?, int1: values.int(1)? },
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
                            0 => FunctionState::Int1Point0 { int0: values.int(0)? },
                            _ => return None,
                        };
                        Some(active)
                    }
                    fn calls_int_1_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                        if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(1)), point, values) { return Some(execution); }
                        let active = calls_int_1_state(point, values)?;
                        Some(Box::new(FunctionExecution::new(active)))
                    }
                    [calls_int_0_start, calls_int_1_start]
                };
                const CALL_GROUP_1: [data::compiled::calls::CallStart; 1] = {
                    use data::compiled::calls::{CallExecution, CallInputs, CallOps, CallProgress, CallStorage};
                    enum FunctionState {
                        Bool0Point0 { bool0: bool },
                    }
                    enum BoolReturn {
                    }
                    #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                    enum FunctionStep {
                        Yield(FunctionState),
                    }
                    struct FunctionExecution {
                        active: Option<FunctionState>,
                        boolean_returns: Vec<BoolReturn>,
                    }
                    impl FunctionExecution {
                        fn new(active: FunctionState) -> Self {
                            Self {
                                active: Some(active),
                                boolean_returns: Vec::new(),
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
                            std::mem::size_of::<Self>() + self.boolean_returns.capacity() * std::mem::size_of::<BoolReturn>()
                        }
                        fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                            let Some(active) = self.active.take() else { return CallProgress::Yield(self); };
                            match function_step(active, ops, budget) {
                                FunctionStep::Yield(active) => {
                                    self.active = Some(active);
                                    CallProgress::Yield(self)
                                },
                            }
                        }
                    }
                    fn function_step(active: FunctionState, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            FunctionState::Bool0Point0 { bool0 } => calls_bool_0_run(Bool0State::Point0 { bool0 }, ops, budget),
                        }
                    }
                    enum Bool0State {
                        Point0 { bool0: bool },
                    }
                    fn calls_bool_0_run(mut active: Bool0State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        loop {
                            match active {
                                Bool0State::Point0 { bool0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point0 { bool0 }); }
                                    *budget -= 1;
                                    active = {
                                        Bool0State::Point0 { bool0 }
                                    };
                                    continue;
                                },
                            }
                        }
                    }
                    fn calls_bool_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                        let active = match point {
                            0 => FunctionState::Bool0Point0 { bool0: values.bool(0)? },
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
                        Bool0Point0 { bool0: bool },
                        String0Point0 { bool0: bool },
                        String0Point1 { bool0: bool, string0: StringValue },
                        String0Point2 { bool0: bool, string0: StringValue, string1: StringValue },
                        String0Point3 { string0: StringValue },
                        String0Point4 { string0: StringValue, bool0: bool },
                        String0Point5 { string0: StringValue, bool0: bool, bool1: bool },
                        String0Point6 { string0: StringValue },
                        String1Point0 { string0: StringValue },
                    }
                    #[allow(dead_code, reason = "Caller locals stay owned until the non-returning callee is cancelled.")]
                    enum BoolReturn {
                        String0Call4 { string0: StringValue, bool0: bool },
                    }
                    enum StringReturn {
                        String0Call1 { bool0: bool, string0: StringValue },
                    }
                    impl StringReturn {
                        fn small(self, result: StringValue) -> FunctionState {
                            match self {
                                Self::String0Call1 { bool0, string0 } => {
                                    let string1 = result;
                                    FunctionState::String0Point2 { bool0, string0, string1 }
                                },
                            }
                        }
                    }
                    #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                    enum FunctionStep {
                        Yield(FunctionState),
                        BoolCall { callee: FunctionState, caller: BoolReturn },
                        StringCall { callee: FunctionState, caller: StringReturn },
                        String { value: StringValue },
                    }
                    struct FunctionExecution {
                        active: Option<FunctionState>,
                        boolean_returns: Vec<BoolReturn>,
                        string_returns: Vec<StringReturn>,
                    }
                    impl FunctionExecution {
                        fn new(active: FunctionState) -> Self {
                            Self {
                                active: Some(active),
                                boolean_returns: Vec::new(),
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
                            std::mem::size_of::<Self>() + self.boolean_returns.capacity() * std::mem::size_of::<BoolReturn>() + self.string_returns.capacity() * std::mem::size_of::<StringReturn>()
                        }
                        fn advance(mut self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
                            let Some(mut active) = self.active.take() else { return CallProgress::Yield(self); };
                            loop {
                                match function_step(active, ops, budget) {
                                    FunctionStep::Yield(active) => {
                                        self.active = Some(active);
                                        return CallProgress::Yield(self);
                                    },
                                    FunctionStep::BoolCall { callee, caller } => {
                                        self.boolean_returns.push(caller);
                                        active = callee;
                                    },
                                    FunctionStep::StringCall { callee, caller } => {
                                        self.string_returns.push(caller);
                                        active = callee;
                                    },
                                    FunctionStep::String { value } => {
                                        if let Some(caller) = self.string_returns.pop() {
                                            active = caller.small(value);
                                        } else {
                                            self.boolean_returns.clear();
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
                            FunctionState::Bool0Point0 { bool0 } => calls_bool_0_run(Bool0State::Point0 { bool0 }, ops, budget),
                            FunctionState::String0Point0 { bool0 } => calls_string_0_run(String0State::Point0 { bool0 }, ops, budget),
                            FunctionState::String0Point1 { bool0, string0 } => calls_string_0_run(String0State::Point1 { bool0, string0 }, ops, budget),
                            FunctionState::String0Point2 { bool0, string0, string1 } => calls_string_0_run(String0State::Point2 { bool0, string0, string1 }, ops, budget),
                            FunctionState::String0Point3 { string0 } => calls_string_0_run(String0State::Point3 { string0 }, ops, budget),
                            FunctionState::String0Point4 { string0, bool0 } => calls_string_0_run(String0State::Point4 { string0, bool0 }, ops, budget),
                            FunctionState::String0Point5 { string0, bool0, bool1 } => calls_string_0_run(String0State::Point5 { string0, bool0, bool1 }, ops, budget),
                            FunctionState::String0Point6 { string0 } => calls_string_0_run(String0State::Point6 { string0 }, ops, budget),
                            FunctionState::String1Point0 { string0 } => calls_string_1_run(String1State::Point0 { string0 }, ops, budget),
                        }
                    }
                    enum Bool0State {
                        Point0 { bool0: bool },
                    }
                    fn calls_bool_0_run(mut active: Bool0State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        loop {
                            match active {
                                Bool0State::Point0 { bool0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point0 { bool0 }); }
                                    *budget -= 1;
                                    active = {
                                        Bool0State::Point0 { bool0 }
                                    };
                                    continue;
                                },
                            }
                        }
                    }
                    enum String0State {
                        Point0 { bool0: bool },
                        Point1 { bool0: bool, string0: StringValue },
                        Point2 { bool0: bool, string0: StringValue, string1: StringValue },
                        Point3 { string0: StringValue },
                        Point4 { string0: StringValue, bool0: bool },
                        Point5 { string0: StringValue, bool0: bool, bool1: bool },
                        Point6 { string0: StringValue },
                    }
                    fn calls_string_0_run(mut active: String0State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        loop {
                            match active {
                                String0State::Point0 { bool0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String0Point0 { bool0 }); }
                                    *budget -= 1;
                                    let string0 = StringValue::from("kept");
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String0Point1 { bool0, string0 }); }
                                    *budget -= 1;
                                    return {
                                        FunctionStep::StringCall { callee: FunctionState::String1Point0 { string0: string0.clone() }, caller: StringReturn::String0Call1 { bool0, string0 } }
                                    };
                                },
                                String0State::Point1 { bool0, string0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String0Point1 { bool0, string0 }); }
                                    *budget -= 1;
                                    return {
                                        FunctionStep::StringCall { callee: FunctionState::String1Point0 { string0: string0.clone() }, caller: StringReturn::String0Call1 { bool0, string0 } }
                                    };
                                },
                                String0State::Point2 { bool0, string0, string1 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String0Point2 { bool0, string0, string1 }); }
                                    *budget -= 1;
                                    active = {
                                        if bool0 { String0State::Point3 { string0: string1.clone() } } else { String0State::Point6 { string0: string1.clone() } }
                                    };
                                    continue;
                                },
                                String0State::Point3 { string0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String0Point3 { string0 }); }
                                    *budget -= 1;
                                    let bool0 = true;
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String0Point4 { string0, bool0 }); }
                                    *budget -= 1;
                                    return {
                                        FunctionStep::BoolCall { callee: FunctionState::Bool0Point0 { bool0 }, caller: BoolReturn::String0Call4 { string0, bool0 } }
                                    };
                                },
                                String0State::Point4 { string0, bool0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String0Point4 { string0, bool0 }); }
                                    *budget -= 1;
                                    return {
                                        FunctionStep::BoolCall { callee: FunctionState::Bool0Point0 { bool0 }, caller: BoolReturn::String0Call4 { string0, bool0 } }
                                    };
                                },
                                String0State::Point5 { string0, bool0, bool1 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String0Point5 { string0, bool0, bool1 }); }
                                    *budget -= 1;
                                    return {
                                        FunctionStep::String { value: string0 }
                                    };
                                },
                                String0State::Point6 { string0 } => {
                                    if *budget == 0 { return FunctionStep::Yield(FunctionState::String0Point6 { string0 }); }
                                    *budget -= 1;
                                    return {
                                        FunctionStep::String { value: string0 }
                                    };
                                },
                            }
                        }
                    }
                    enum String1State {
                        Point0 { string0: StringValue },
                    }
                    fn calls_string_1_run(active: String1State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            String1State::Point0 { string0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::String1Point0 { string0 }); }
                                *budget -= 1;
                                {
                                    FunctionStep::String { value: string0 }
                                }
                            },
                        }
                    }
                    fn calls_string_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                        let active = match point {
                            0 => FunctionState::String0Point0 { bool0: values.bool(0)? },
                            1 => FunctionState::String0Point1 { bool0: values.bool(0)?, string0: values.string(0)? },
                            2 => FunctionState::String0Point2 { bool0: values.bool(0)?, string0: values.string(0)?, string1: values.string(1)? },
                            3 => FunctionState::String0Point3 { string0: values.string(0)? },
                            4 => FunctionState::String0Point4 { string0: values.string(0)?, bool0: values.bool(0)? },
                            5 => FunctionState::String0Point5 { string0: values.string(0)?, bool0: values.bool(0)?, bool1: values.bool(1)? },
                            6 => FunctionState::String0Point6 { string0: values.string(0)? },
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
                    use data::compiled::calls::{CallExecution, CallInputs, CallOps, CallOutput, CallProgress, CallStorage, StringValue};
                    enum FunctionState {
                        String1Point0 { string0: StringValue },
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
                                data::compiled::CallTarget::String(data::function::StringFunctionId(1)) => calls_string_1_state(point, values),
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
                            FunctionState::String1Point0 { string0 } => calls_string_1_run(String1State::Point0 { string0 }, ops, budget),
                        }
                    }
                    enum String1State {
                        Point0 { string0: StringValue },
                    }
                    fn calls_string_1_run(active: String1State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                        match active {
                            String1State::Point0 { string0 } => {
                                if *budget == 0 { return FunctionStep::Yield(FunctionState::String1Point0 { string0 }); }
                                *budget -= 1;
                                {
                                    FunctionStep::String { value: string0 }
                                }
                            },
                        }
                    }
                    fn calls_string_1_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                        let active = match point {
                            0 => FunctionState::String1Point0 { string0: values.string(0)? },
                            _ => return None,
                        };
                        Some(active)
                    }
                    fn calls_string_1_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                        if let Some(execution) = storage.reuse(data::compiled::CallTarget::String(data::function::StringFunctionId(1)), point, values) { return Some(execution); }
                        let active = calls_string_1_state(point, values)?;
                        Some(Box::new(FunctionExecution::new(active)))
                    }
                    [calls_string_1_start]
                };
                data::compiled::CompiledFunctions {
                    ints: data::Storage::Static(&[
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
                                        block: data::graph::BlockId(0),
                                        instruction: 2,
                                        ints: 1,
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
                                        instruction: 3,
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
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[]),
                                    data::Storage::Static(&[
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
                                    data::Storage::Static(&[
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
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 2,
                                        output: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                        target: data::compiled::CallContractTarget::IntValue(data::graph::IntFunctionLocalId(0)),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("example", "wide", data::source::SourceSpan::new(362, 390)),
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
                                        target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(1)),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                        reference: true,
                                        captures: data::Storage::Static(&[]),
                                    },
                                ]),
                                returns: data::Storage::Static(&[]),
                                tails: data::Storage::Static(&[]),
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
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 0,
                                        value: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: CALL_GROUP_0[1],
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
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[]),
                                tails: data::Storage::Static(&[]),
                                start: CALL_GROUP_1[0],
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::String(data::function::StringFunctionId(0)),
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
                                        bools: 1,
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
                                        bools: 1,
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
                                        instruction: 2,
                                        ints: 0,
                                        bools: 2,
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
                                        strings: 1,
                                        customs: 0,
                                        custom_lists: 0,
                                        int_functions: 0,
                                        bool_functions: 0,
                                    },
                                ]),
                                locals: data::Storage::Static(&[
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    ]),
                                    data::Storage::Static(&[
                                        data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                    ]),
                                ]),
                                calls: data::Storage::Static(&[
                                    data::compiled::CallContract {
                                        point: 1,
                                        output: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::String(data::function::StringFunctionId(1))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("example", "choose", data::source::SourceSpan::new(144, 156)),
                                    },
                                    data::compiled::CallContract {
                                        point: 4,
                                        output: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                        target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0))),
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        ]),
                                        site: data::source::HostCallSite::from_static("example", "choose", data::source::SourceSpan::new(199, 209)),
                                    },
                                ]),
                                creations: data::Storage::Static(&[]),
                                returns: data::Storage::Static(&[
                                    data::compiled::ReturnContract {
                                        point: 5,
                                        value: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                    },
                                    data::compiled::ReturnContract {
                                        point: 6,
                                        value: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                    },
                                ]),
                                tails: data::Storage::Static(&[]),
                                start: CALL_GROUP_2[0],
                            })),
                        },
                        data::compiled::CompiledFunction {
                            function: data::compiled::CallTarget::String(data::function::StringFunctionId(1)),
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
                    0..0,
                    0..2,
                    0..0,
                    2..4,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    4..5,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
                    0..0,
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
                        return_: data::type_::ValueShapeId(2),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 0..1,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(2),
                        ]),
                        return_: data::type_::ValueShapeId(2),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 1..2,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 2..3,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(1),
                        ]),
                        return_: data::type_::ValueShapeId(1),
                        captures: data::Storage::Static(&[]),
                    },
                    data::function::FunctionContract {
                        parameters: 3..4,
                        parameter_shapes: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::ValueShapeId(0),
                        captures: data::Storage::Static(&[]),
                    },
                ]),
                parameters: data::Storage::Static(&[
                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                ]),
            },
            list_types: data::type_::ListTypeTable {
                types: data::Storage::Static(&[]),
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
                    data::type_::ValueShapeDescriptor::Bool,
                    data::type_::ValueShapeDescriptor::String,
                    data::type_::ValueShapeDescriptor::Int,
                    data::type_::ValueShapeDescriptor::Function {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(2),
                        ]),
                        return_: data::type_::ValueShapeId(2),
                    },
                ]),
                shape_types: data::Storage::Static(&[
                    data::type_::ValueType::Bool,
                    data::type_::ValueType::String,
                    data::type_::ValueType::Int,
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
            strings: data::Storage::Static(&[
                data::program::LibraryFunctionEntry {
                    function: data::function::StringFunctionId(0),
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
        exports: data::Storage::Static(&[
            data::Export {
                name: data::Text::Static("choose"),
                signature: data::type_::FunctionMetadata {
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Bool,
                    ]),
                    return_: data::Storage::Static(&data::type_::TypeMetadata::String),
                },
                slot: 0,
            },
            data::Export {
                name: data::Text::Static("wide"),
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
