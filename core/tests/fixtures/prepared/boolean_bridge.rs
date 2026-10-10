data::ModuleArtifact {
    format: 30,
    program: data::ProgramTables {
        root: data::source::module_id(0),
        modules: data::Storage::Static(&[
            data::program::ExecutionModuleContext {
                module: data::Text::Static("example"),
                source_context: None,
            },
        ]),
        main: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Bool(data::function::BoolFunctionId(0))),
        functions: data::function::FunctionTables {
            value_returns: data::function::ValueFunctionTables {
                never_functions: data::Storage::Static(&[]),
                int_functions: data::Storage::Static(&[]),
                float_functions: data::Storage::Static(&[]),
                string_functions: data::Storage::Static(&[]),
                bit_array_functions: data::Storage::Static(&[]),
                utf_codepoint_functions: data::Storage::Static(&[]),
                custom_functions: data::Storage::Static(&[]),
                external_functions: data::Storage::Static(&[]),
                bool_functions: data::Storage::Static(&[
                    data::function::ExecutableFunction {
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
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 0..1,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::LetAssertPanic(data::graph::LetAssertPanic {
                                            subject: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            message: None,
                                            site: data::source::PanicSite::from_static("example", "verify", data::source::SourceSpan::new(28, 38)),
                                            pattern_span: data::source::SourceSpan::new(39, 43),
                                        }),
                                    },
                                ]),
                                params: data::Storage::Static(&[
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(0),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Call {
                                            function: data::function::BoolFunctionId(1),
                                            args: data::Storage::Static(&[]),
                                            site: data::source::HostCallSite::from_static("example", "verify", data::source::SourceSpan::new(46, 56)),
                                        }),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::TailCall {
                                    function: data::source::FunctionCallTarget {
                                        function: data::function::BoolFunctionId(1),
                                        site: data::source::HostCallSite::from_static("example", "verify", data::source::SourceSpan::new(59, 69)),
                                    },
                                    args: data::Storage::Static(&[]),
                                    transfer: data::graph::Transfer {
                                        families: data::Storage::Static(&[]),
                                    },
                                },
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
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
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("ok"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                            shape: data::type_::ValueShapeId(1),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("ok"))),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(0),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Equal {
                                            left: data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                            right: data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                        })),
                                    }),
                                ]),
                            },
                            exits: data::Storage::Static(&[
                                data::function::FunctionExit::Return(data::graph::BoolLocalId(0)),
                            ]),
                        },
                    },
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

            enum CompiledResume {
                Next(usize),
                Exit(data::compiled::CompiledProgress),
            }
            const CALL_GROUP_0: [data::compiled::calls::CallStart; 2] = {
                use data::compiled::calls::{CallExecution, CallInputs, CallOps, CallOutput, CallProgress, CallStorage, CallValues, StringValue};
                enum FunctionState {
                    Bool0Point0 {  },
                    Bool0Point1 { bool0: bool },
                    Bool0Point2 {  },
                    Bool0Point3 { bool0: bool },
                    Bool1Point0 {  },
                    Bool1Point1 { string0: StringValue },
                    Bool1Point2 { string0: StringValue, string1: StringValue },
                    Bool1Point3 { string0: StringValue, string1: StringValue, bool0: bool },
                    Bool1Kernel { point: usize, values: Box<data::compiled::string::StringValues> },
                }
                enum BoolReturn {
                    Bool0Call0 {  },
                }
                impl BoolReturn {
                    fn site(&self) -> data::source::HostCallSite {
                        match *self {
                            Self::Bool0Call0 { .. } => data::source::HostCallSite::from_static("example", "verify", data::source::SourceSpan::new(46, 56)),
                        }
                    }
                    fn small(self, result: bool) -> FunctionState {
                        match self {
                            Self::Bool0Call0 {  } => {
                                let bool0 = result;
                                FunctionState::Bool0Point1 { bool0 }
                            },
                        }
                    }
                    fn resume(self, result: bool) -> FunctionState { self.small(result) }
                }
                #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
                enum FunctionStep {
                    Yield(FunctionState),
                    Canonical { target: data::compiled::CallTarget, point: data::compiled::CompiledCheckpoint, values: Box<CallValues> },
                    BoolCall { callee: FunctionState, caller: BoolReturn },
                    BoolTail { callee: FunctionState },
                    Bool { value: bool },
                }
                struct FunctionExecution {
                    active: Option<FunctionState>,
                    pending_entry: bool,
                    boolean_returns: Vec<BoolReturn>,
                }
                impl FunctionExecution {
                    fn new(active: FunctionState) -> Self {
                        Self {
                            active: Some(active),
                            pending_entry: false,
                            boolean_returns: Vec::new(),
                        }
                    }
                }
                impl CallExecution for FunctionExecution {
                    fn restart(&mut self, target: data::compiled::CallTarget, point: usize, values: CallInputs<'_>) -> bool {
                        if self.active.is_some() { return false; }
                        let active = match target {
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)) => calls_bool_0_state(point, values),
                            data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)) => calls_bool_1_state(point, values),
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
                        let Some(mut active) = self.active.take() else { return CallProgress::Yield(self); };
                        loop {
                            if self.pending_entry {
                                if *budget == 0 { self.active = Some(active); return CallProgress::Yield(self); }
                                *budget -= 1;
                                self.pending_entry = false;
                            }
                            match function_step(active, ops, budget) {
                                FunctionStep::Yield(active) => {
                                    self.active = Some(active);
                                    return CallProgress::Yield(self);
                                },
                                FunctionStep::BoolCall { callee, caller } => {
                                    self.boolean_returns.push(caller);
                                    active = callee;
                                },
                                FunctionStep::BoolTail { callee } => {
                                    *budget -= 1;
                                    self.pending_entry = self.boolean_returns.is_empty() && ops.root_tail_entry();
                                    active = callee;
                                },
                                FunctionStep::Bool { value } => {
                                    if let Some(caller) = self.boolean_returns.pop() {
                                        active = caller.small(value);
                                    } else {
                                        self.boolean_returns.clear();
                                        return CallProgress::Complete { output: CallOutput::Bool(value), execution: self };
                                    }
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
                                        _ => return CallProgress::Interpreted { target, point, values },
                                    }
                                },
                            }
                        }
                    }
                }
                fn function_step(active: FunctionState, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        FunctionState::Bool0Point0 {  } => calls_bool_0_run(Bool0State::Point0 {  }, ops, budget),
                        FunctionState::Bool0Point1 { bool0 } => calls_bool_0_run(Bool0State::Point1 { bool0 }, ops, budget),
                        FunctionState::Bool0Point2 {  } => calls_bool_0_run(Bool0State::Point2 {  }, ops, budget),
                        FunctionState::Bool0Point3 { bool0 } => calls_bool_0_run(Bool0State::Point3 { bool0 }, ops, budget),
                        FunctionState::Bool1Point0 {  } => calls_bool_1_run(Bool1State::Point0 {  }, ops, budget),
                        FunctionState::Bool1Point1 { string0 } => calls_bool_1_run(Bool1State::Point1 { string0 }, ops, budget),
                        FunctionState::Bool1Point2 { string0, string1 } => calls_bool_1_run(Bool1State::Point2 { string0, string1 }, ops, budget),
                        FunctionState::Bool1Point3 { string0, string1, bool0 } => calls_bool_1_run(Bool1State::Point3 { string0, string1, bool0 }, ops, budget),
                        FunctionState::Bool1Kernel { point, mut values } => {
                            let progress = string_bool_1(point, &mut values, budget);
                            calls_bool_1_kernel(progress, values, ops)
                        },
                    }
                }
                enum Bool0State {
                    Point0 {  },
                    Point1 { bool0: bool },
                    Point2 {  },
                    Point3 { bool0: bool },
                }
                fn calls_bool_0_run(active: Bool0State, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Bool0State::Point0 {  } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point0 {  }); }
                            *budget -= 1;
                            {
                                FunctionStep::BoolCall { callee: FunctionState::Bool1Point0 {  }, caller: BoolReturn::Bool0Call0 {  } }
                            }
                        },
                        Bool0State::Point1 { bool0 } => {
                            FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
                                block: data::graph::BlockId(0),
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
                            }, values: Box::new(CallValues { bools: vec![bool0], ..CallValues::default() }) }
                        },
                        Bool0State::Point2 {  } => {
                            if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point2 {  }); }
                            {
                                FunctionStep::BoolTail { callee: FunctionState::Bool1Point0 {  } }
                            }
                        },
                        Bool0State::Point3 { bool0 } => {
                            FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point: data::compiled::CompiledCheckpoint {
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
                            }, values: Box::new(CallValues { bools: vec![bool0], ..CallValues::default() }) }
                        },
                    }
                }
                enum Bool1State {
                    Point0 {  },
                    Point1 { string0: StringValue },
                    Point2 { string0: StringValue, string1: StringValue },
                    Point3 { string0: StringValue, string1: StringValue, bool0: bool },
                }
                fn calls_bool_1_run(active: Bool1State, ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                    match active {
                        Bool1State::Point0 {  } => {
                            {
                                let mut values = ops.strings(&[], &[], []);
                                let progress = string_bool_1(0, &mut values, budget);
                                calls_bool_1_kernel(progress, values, ops)
                            }
                        },
                        Bool1State::Point1 { string0 } => {
                            {
                                let mut values = ops.strings(&[], &[], [string0]);
                                let progress = string_bool_1(1, &mut values, budget);
                                calls_bool_1_kernel(progress, values, ops)
                            }
                        },
                        Bool1State::Point2 { string0, string1 } => {
                            {
                                let mut values = ops.strings(&[], &[], [string0, string1]);
                                let progress = string_bool_1(2, &mut values, budget);
                                calls_bool_1_kernel(progress, values, ops)
                            }
                        },
                        Bool1State::Point3 { string0, string1, bool0 } => {
                            {
                                let mut values = ops.strings(&[], &[bool0], [string0, string1]);
                                let progress = string_bool_1(3, &mut values, budget);
                                calls_bool_1_kernel(progress, values, ops)
                            }
                        },
                    }
                }
                fn calls_bool_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool0Point0 {  },
                        1 => FunctionState::Bool0Point1 { bool0: values.bool(0)? },
                        2 => FunctionState::Bool0Point2 {  },
                        3 => FunctionState::Bool0Point3 { bool0: values.bool(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)), point, values) { return Some(execution); }
                    let active = calls_bool_0_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                fn calls_bool_1_kernel(progress: data::compiled::CompiledProgress, mut values: Box<data::compiled::string::StringValues>, ops: &mut CallOps<'_>) -> FunctionStep {
                    const RETURNS: [fn(&mut data::compiled::string::StringValues) -> FunctionStep; 1] = [
                        |values| {
                            let value = values.bools[0];
                            values.release_inputs();
                            FunctionStep::Bool { value }
                        },
                    ];
                    match progress {
                        data::compiled::CompiledProgress::Yield(point) => FunctionStep::Yield(FunctionState::Bool1Kernel { point, values }),
                        data::compiled::CompiledProgress::Interpreted(point) => {
                            const POINTS: [data::compiled::CompiledCheckpoint; 4] = [
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
                                    block: data::graph::BlockId(0),
                                    instruction: 3,
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
                            ];
                            let strings = values.take_strings();
                            let step = FunctionStep::Canonical { target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)), point: POINTS[point], values: Box::new(CallValues { ints: values.ints.iter().copied().map(Into::into).collect(), bools: values.bools.clone(), strings, ..CallValues::default() }) };
                            ops.recycle_strings(values);
                            step
                        },
                        data::compiled::CompiledProgress::Complete(exit) => {
                            let step = RETURNS[exit.0](&mut values);
                            ops.recycle_strings(values);
                            step
                        },
                    }
                }
                fn calls_bool_1_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                    let active = match point {
                        0 => FunctionState::Bool1Point0 {  },
                        1 => FunctionState::Bool1Point1 { string0: values.string(0)? },
                        2 => FunctionState::Bool1Point2 { string0: values.string(0)?, string1: values.string(1)? },
                        3 => FunctionState::Bool1Point3 { string0: values.string(0)?, string1: values.string(1)?, bool0: values.bool(0)? },
                        _ => return None,
                    };
                    Some(active)
                }
                fn calls_bool_1_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                    if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)), point, values) { return Some(execution); }
                    let active = calls_bool_1_state(point, values)?;
                    Some(Box::new(FunctionExecution::new(active)))
                }
                [calls_bool_0_start, calls_bool_1_start]
            };

            fn string_bool_1(
                point: usize,
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> data::compiled::CompiledProgress {

                const RESUME: [
                    fn(&mut data::compiled::string::StringValues, &mut usize) -> CompiledResume;
                    4
                ] = [
                    |values, budget| CompiledResume::Exit(string_bool_1_entry((), values, budget)),
                    string_bool_1_resume_1,
                    string_bool_1_resume_2,
                    string_bool_1_resume_3,
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
                let b0_s0 = data::compiled::string::StringRange::literal("ok");
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
                let b0_s1 = data::compiled::string::StringRange::literal("ok");
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                    return data::compiled::CompiledProgress::Yield(2);
                }
                *budget -= 1;
                let b0_v0 = values.bytes(b0_s0) == values.bytes(b0_s1);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                    return data::compiled::CompiledProgress::Yield(3);
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b0_v0]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
            }

            fn string_bool_1_resume_1(
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
                let b0_s1 = data::compiled::string::StringRange::literal("ok");

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                CompiledResume::Next(2)
            }

            fn string_bool_1_resume_2(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b0_s0, b0_s1,) = (values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(2));
                }
                *budget -= 1;
                let b0_v0 = values.bytes(b0_s0) == values.bytes(b0_s1);

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b0_v0]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                CompiledResume::Next(3)
            }

            fn string_bool_1_resume_3(
                values: &mut data::compiled::string::StringValues,
                budget: &mut usize,
            ) -> CompiledResume {
                let (b0_v0, b0_s0, b0_s1,) = (values.bools[0], values.strings[0], values.strings[1],);
                if *budget == 0 {

                    values.ints.clear();
                    values.ints.extend_from_slice(&[]);
                    values.bools.clear();
                    values.bools.extend_from_slice(&[b0_v0]);

                    values.strings.clear();
                    values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                    return CompiledResume::Exit(data::compiled::CompiledProgress::Yield(3));
                }
                *budget -= 1;

                values.ints.clear();
                values.ints.extend_from_slice(&[]);
                values.bools.clear();
                values.bools.extend_from_slice(&[b0_v0]);

                values.strings.clear();
                values.strings.extend_from_slice(&[b0_s0, b0_s1]);
                CompiledResume::Exit(data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0)))
            }
            data::compiled::CompiledFunctions {
                ints: data::Storage::Static(&[
                ]),
                bools: data::Storage::Static(&[
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
                                    block: data::graph::BlockId(0),
                                    instruction: 3,
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
                                data::Storage::Static(&[]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[
                                data::compiled::CallContract {
                                    point: 0,
                                    output: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1))),
                                    args: data::Storage::Static(&[]),
                                    site: data::source::HostCallSite::from_static("example", "verify", data::source::SourceSpan::new(46, 56)),
                                },
                            ]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[]),
                            tails: data::Storage::Static(&[
                                data::compiled::TailContract {
                                    point: 2,
                                    target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)),
                                    args: data::Storage::Static(&[]),
                                    site: data::source::HostCallSite::from_static("example", "verify", data::source::SourceSpan::new(59, 69)),
                                },
                            ]),
                            start: CALL_GROUP_0[0],
                        })),
                    },
                    data::compiled::CompiledFunction {
                        function: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)),
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
                                    block: data::graph::BlockId(0),
                                    instruction: 3,
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
                            ]),
                            locals: data::Storage::Static(&[
                                data::Storage::Static(&[]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::String(data::graph::StringLocalId(0)),
                                    data::graph::ParamLocal::String(data::graph::StringLocalId(1)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 3,
                                    value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: CALL_GROUP_0[1],
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
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
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
            ],
            functions: data::Storage::Static(&[
                data::function::FunctionContract {
                    parameters: 0..0,
                    parameter_shapes: data::Storage::Static(&[]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 0..0,
                    parameter_shapes: data::Storage::Static(&[]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
            ]),
            parameters: data::Storage::Static(&[]),
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
                data::type_::ValueShapeDescriptor::Bool,
                data::type_::ValueShapeDescriptor::String,
            ]),
            shape_types: data::Storage::Static(&[
                data::type_::ValueType::Bool,
                data::type_::ValueType::String,
            ]),
            custom_shapes: data::Storage::Static(&[]),
        },
    },
    entries: data::program::LibraryFunctionEntries {
        ints: data::Storage::Static(&[]),
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
            name: data::Text::Static("verify"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            slot: 0,
        },
    ]),
}
