data::ModuleArtifact {
    format: 25,
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
                                                target: data::graph::BlockId(6),
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
                                        }),
                                    },
                                    data::graph::BlockHeader {
                                        params: 1..1,
                                        instructions: 0..1,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
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
                                        params: 1..2,
                                        instructions: 1..1,
                                        terminator: data::graph::Terminator::BoolBranch(data::graph::BoolBranch {
                                            subject: data::graph::BoolLocalId(0),
                                            true_: data::graph::Edge {
                                                target: data::graph::BlockId(3),
                                                args: data::Storage::Static(&[]),
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
                                                target: data::graph::BlockId(5),
                                                args: data::Storage::Static(&[]),
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
                                        params: 2..2,
                                        instructions: 1..2,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(4),
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
                                        params: 2..3,
                                        instructions: 2..4,
                                        terminator: data::graph::Terminator::Exit(data::graph::BlockGraphExitId(0)),
                                    },
                                    data::graph::BlockHeader {
                                        params: 3..3,
                                        instructions: 4..5,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
                                                target: data::graph::BlockId(4),
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
                                        params: 3..3,
                                        instructions: 5..6,
                                        terminator: data::graph::Terminator::Jump(data::graph::Jump {
                                            edge: data::graph::Edge {
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
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                    data::graph::ParamSlot {
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Value(true)),
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
                                            function: data::function::BoolFunctionId(1),
                                            args: data::Storage::Static(&[
                                                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                            ]),
                                            site: data::source::HostCallSite::from_static("example", "nonempty", data::source::SourceSpan::new(168, 183)),
                                        }),
                                    }),
                                    data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                        output: data::graph::ParamSlot {
                                            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                            shape: data::type_::ValueShapeId(2),
                                        },
                                        kind: data::graph::ProfiledInstructionKind::Bool(data::graph::BoolInstruction::Test(data::graph::BoolTest::Not(data::graph::BoolLocalId(1)))),
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
                                data::function::FunctionExit::Return(data::graph::BoolLocalId(2)),
                            ]),
                        },
                    },
                    data::function::ExecutableFunction {
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
                                        local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                        shape: data::type_::ValueShapeId(2),
                                    },
                                ]),
                                instructions: data::Storage::Static(&[]),
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
            use data::compiled::calls::{CallExecution, CallInputs, CallOps, CallOutput, CallProgress, CallStorage};
            use data::compiled::int_list::IntList;
            enum FunctionState {
                Bool0Point0 { int_list0: IntList },
                Bool0Point1 {  },
                Bool0Point2 { bool0: bool },
                Bool0Point3 { bool0: bool },
                Bool0Point4 {  },
                Bool0Point5 { bool0: bool },
                Bool0Point6 { bool0: bool },
                Bool0Point7 { bool0: bool, bool1: bool },
                Bool0Point8 { bool0: bool, bool1: bool, bool2: bool },
                Bool0Point9 {  },
                Bool0Point10 { bool0: bool },
                Bool0Point11 {  },
                Bool0Point12 { bool0: bool },
                Bool1Point0 { bool0: bool },
            }
            enum IntReturn {
            }
            enum BoolReturn {
                Bool0Call6 { bool0: bool },
            }
            impl BoolReturn {
                fn small(self, result: bool) -> FunctionState {
                    match self {
                        Self::Bool0Call6 { bool0 } => {
                            let bool1 = result;
                            FunctionState::Bool0Point7 { bool0, bool1 }
                        },
                    }
                }
            }
            enum IntFunctionReturn {
            }
            enum BoolFunctionReturn {
            }
            #[allow(clippy::large_enum_variant, reason = "Typed locals stay inline to avoid allocating at each generated step.")]
            enum FunctionStep {
                Next(FunctionState),
                Yield(FunctionState),
                BoolCall { callee: FunctionState, caller: BoolReturn },
                Bool { value: bool, exit: data::graph::BlockGraphExitId },
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
                        data::compiled::CallTarget::Bool(data::function::BoolFunctionId(0)) => calls_bool_0_state(point, values),
                        data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)) => calls_bool_1_state(point, values),
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
                            FunctionStep::BoolCall { callee, caller } => {
                                self.boolean_returns.push(caller);
                                active = callee;
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
                        }
                    }
                }
            }
            fn function_step(active: FunctionState, _ops: &mut CallOps<'_>, budget: &mut usize) -> FunctionStep {
                match active {
                    FunctionState::Bool0Point0 { int_list0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point0 { int_list0 }); }
                        *budget -= 1;
                        if int_list0.is_empty() { FunctionStep::Next(FunctionState::Bool0Point1 {  }) } else { FunctionStep::Next(FunctionState::Bool0Point11 {  }) }
                    },
                    FunctionState::Bool0Point1 {  } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point1 {  }); }
                        *budget -= 1;
                        let bool0 = true;
                        FunctionStep::Next(FunctionState::Bool0Point2 { bool0 })
                    },
                    FunctionState::Bool0Point2 { bool0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point2 { bool0 }); }
                        *budget -= 1;
                        FunctionStep::Next(FunctionState::Bool0Point3 { bool0 })
                    },
                    FunctionState::Bool0Point3 { bool0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point3 { bool0 }); }
                        *budget -= 1;
                        if bool0 { FunctionStep::Next(FunctionState::Bool0Point4 {  }) } else { FunctionStep::Next(FunctionState::Bool0Point9 {  }) }
                    },
                    FunctionState::Bool0Point4 {  } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point4 {  }); }
                        *budget -= 1;
                        let bool0 = true;
                        FunctionStep::Next(FunctionState::Bool0Point5 { bool0 })
                    },
                    FunctionState::Bool0Point5 { bool0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point5 { bool0 }); }
                        *budget -= 1;
                        FunctionStep::Next(FunctionState::Bool0Point6 { bool0 })
                    },
                    FunctionState::Bool0Point6 { bool0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point6 { bool0 }); }
                        *budget -= 1;
                        FunctionStep::BoolCall { callee: FunctionState::Bool1Point0 { bool0 }, caller: BoolReturn::Bool0Call6 { bool0 } }
                    },
                    FunctionState::Bool0Point7 { bool0, bool1 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point7 { bool0, bool1 }); }
                        *budget -= 1;
                        let bool2 = !bool1;
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point8 { bool0, bool1, bool2 }); }
                        *budget -= 1;
                        FunctionStep::Bool { value: bool2, exit: data::graph::BlockGraphExitId(0) }
                    },
                    FunctionState::Bool0Point8 { bool0, bool1, bool2 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point8 { bool0, bool1, bool2 }); }
                        *budget -= 1;
                        FunctionStep::Bool { value: bool2, exit: data::graph::BlockGraphExitId(0) }
                    },
                    FunctionState::Bool0Point9 {  } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point9 {  }); }
                        *budget -= 1;
                        let bool0 = false;
                        FunctionStep::Next(FunctionState::Bool0Point10 { bool0 })
                    },
                    FunctionState::Bool0Point10 { bool0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point10 { bool0 }); }
                        *budget -= 1;
                        FunctionStep::Next(FunctionState::Bool0Point6 { bool0 })
                    },
                    FunctionState::Bool0Point11 {  } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point11 {  }); }
                        *budget -= 1;
                        let bool0 = false;
                        FunctionStep::Next(FunctionState::Bool0Point12 { bool0 })
                    },
                    FunctionState::Bool0Point12 { bool0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool0Point12 { bool0 }); }
                        *budget -= 1;
                        FunctionStep::Next(FunctionState::Bool0Point3 { bool0 })
                    },
                    FunctionState::Bool1Point0 { bool0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Bool1Point0 { bool0 }); }
                        *budget -= 1;
                        FunctionStep::Bool { value: bool0, exit: data::graph::BlockGraphExitId(0) }
                    },
                }
            }
            fn calls_bool_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                let active = match point {
                    0 => FunctionState::Bool0Point0 { int_list0: values.int_list(0)? },
                    1 => FunctionState::Bool0Point1 {  },
                    2 => FunctionState::Bool0Point2 { bool0: values.bool(0)? },
                    3 => FunctionState::Bool0Point3 { bool0: values.bool(0)? },
                    4 => FunctionState::Bool0Point4 {  },
                    5 => FunctionState::Bool0Point5 { bool0: values.bool(0)? },
                    6 => FunctionState::Bool0Point6 { bool0: values.bool(0)? },
                    7 => FunctionState::Bool0Point7 { bool0: values.bool(0)?, bool1: values.bool(1)? },
                    8 => FunctionState::Bool0Point8 { bool0: values.bool(0)?, bool1: values.bool(1)?, bool2: values.bool(2)? },
                    9 => FunctionState::Bool0Point9 {  },
                    10 => FunctionState::Bool0Point10 { bool0: values.bool(0)? },
                    11 => FunctionState::Bool0Point11 {  },
                    12 => FunctionState::Bool0Point12 { bool0: values.bool(0)? },
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
                    0 => FunctionState::Bool1Point0 { bool0: values.bool(0)? },
                    _ => return None,
                };
                Some(active)
            }
            fn calls_bool_1_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                if let Some(execution) = storage.reuse(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1)), point, values) { return Some(execution); }
                let active = calls_bool_1_state(point, values)?;
                Some(Box::new(FunctionExecution::new(active)))
            }
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
                                data::compiled::CompiledCheckpoint {
                                    block: data::graph::BlockId(4),
                                    instruction: 2,
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
                                    block: data::graph::BlockId(6),
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
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                ]),
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
                                    point: 6,
                                    output: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(1)),
                                    target: data::compiled::CallContractTarget::Static(data::compiled::CallTarget::Bool(data::function::BoolFunctionId(1))),
                                    args: data::Storage::Static(&[
                                        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                    ]),
                                    site: data::source::HostCallSite::from_static("example", "nonempty", data::source::SourceSpan::new(168, 183)),
                                },
                            ]),
                            creations: data::Storage::Static(&[]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 8,
                                    value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: calls_bool_0_start,
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
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 0,
                                    value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: calls_bool_1_start,
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
                    parameters: 0..1,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(1),
                    ]),
                    return_: data::type_::ValueShapeId(2),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 1..2,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(2),
                    ]),
                    return_: data::type_::ValueShapeId(2),
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
                data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
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
            lifetimes: data::Storage::Static(&[
                data::host::HostValueLifetime::LoadedOwner,
            ]),
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
                data::type_::ValueShapeDescriptor::Int,
                data::type_::ValueShapeDescriptor::List(data::type_::ValueShapeId(0)),
                data::type_::ValueShapeDescriptor::Bool,
            ]),
            shape_types: data::Storage::Static(&[
                data::type_::ValueType::Int,
                data::type_::ValueType::List(data::type_::ListTypeId(0)),
                data::type_::ValueType::Bool,
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
        functions: data::Storage::Static(&[]),
    },
    exports: data::Storage::Static(&[
        data::Export {
            name: data::Text::Static("nonempty"),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Int)),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),
            },
            slot: 0,
        },
    ]),
}
