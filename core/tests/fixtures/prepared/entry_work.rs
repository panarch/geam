data::HostedEntryArtifact {
    format: 21,
    program: data::ProgramTables {
        root: data::source::module_id(3),
        modules: data::Storage::Static(&[
            data::program::ExecutionModuleContext {
                module: data::Text::Static("fixture/work"),
                source_context: Some(data::source::SourceContext::from_static_block("src/fixture/work.gleam", r#"
pub type Work(value)

@external(erlang, "fixture", "ready")
pub fn ready(value: value) -> Work(value)

@external(erlang, "fixture", "map")
pub fn map(value: Work(a), callback: fn(a) -> b) -> Work(b)

@external(erlang, "fixture", "flatten")
pub fn flatten(value: Work(Work(a))) -> Work(a)

@external(erlang, "fixture", "all")
pub fn all(values: List(Work(a))) -> Work(List(a))

pub fn then(value: Work(a), callback: fn(a) -> Work(b)) -> Work(b) {
  flatten(map(value, callback))
}
"#)),
            },
            data::program::ExecutionModuleContext {
                module: data::Text::Static("entry"),
                source_context: Some(data::source::SourceContext::from_static_block("src/entry.gleam", r#"
pub fn main() {
  echo second([0, 42])
  fn(value) { value }
}

fn second(items) {
  case items {
    [] -> 0
    [item] -> item
    [_, item, ..] -> item
  }
}
"#)),
            },
            data::program::ExecutionModuleContext {
                module: data::Text::Static("entry_failure"),
                source_context: Some(data::source::SourceContext::from_static_block("src/entry_failure.gleam", r#"
pub fn main() {
  echo "before failure"
  panic as "prepared main failed"
}
"#)),
            },
            data::program::ExecutionModuleContext {
                module: data::Text::Static("entry_work"),
                source_context: Some(data::source::SourceContext::from_static_block("src/entry_work.gleam", r#"
import fixture/work

pub fn main() {
  echo "main"
  work.map(work.ready(41), fn(value) {
    echo value + 1
    fn(value: Int) { value + 1 }
  })
}
"#)),
            },
        ]),
        main: data::function::ProfiledRuntimeFunctionId::External(data::function::ExternalFunctionId {
            index: 0,
            return_type: data::type_::ExternalTypeId(0),
        }),
        functions: data::function::FunctionTables {
            value_returns: data::function::ValueFunctionTables {
                never_functions: data::Storage::Static(&[]),
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
                external_functions: data::Storage::Static(&[
                    data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 0,
                        },
                        body: data::function::ProfiledExternalFunctionBody {
                            _signature_type: data::type_::ExternalTypeId(0),
                            _body_type: data::type_::ExternalTypeId(0),
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
                                                site: data::source::EchoSite::from_static("entry_work", "main", data::source::SourceSpan::new(39, 50)),
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
                                            instructions: 1..4,
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
                                            kind: data::graph::ProfiledInstructionKind::String(data::graph::StringInstruction::Value(data::Text::Static("main"))),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                shape: data::type_::ValueShapeId(0),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Value(data::graph::IntegerLiteral {
                                                sign: data::Sign::Plus,
                                                digits: data::Storage::Static(&[
                                                    41,
                                                ]),
                                            })),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                    id: data::graph::ExternalLocalId(0),
                                                    type_id: data::type_::ExternalTypeId(1),
                                                }),
                                                shape: data::type_::ValueShapeId(3),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::External(data::graph::ExternalInstruction::Call {
                                                function: data::function::ExternalFunctionId {
                                                    index: 1,
                                                    return_type: data::type_::ExternalTypeId(1),
                                                },
                                                args: data::Storage::Static(&[
                                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                                ]),
                                                site: data::source::HostCallSite::from_static("entry_work", "main", data::source::SourceSpan::new(62, 76)),
                                            }),
                                        }),
                                        data::graph::ProfiledInstruction::Value(data::graph::ProfiledValueInstruction {
                                            output: data::graph::ParamSlot {
                                                local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                                                    id: data::graph::CoreFunctionFunctionLocalId(0),
                                                    type_: data::type_::FunctionFunctionType {
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                            })),
                                                        },
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueShapeId(0),
                                                        ]),
                                                        return_: data::type_::FunctionShape {
                                                            shape_id: data::type_::ValueShapeId(1),
                                                            type_: data::type_::FunctionType {
                                                                arguments: data::Storage::Static(&[
                                                                    data::type_::ValueType::Int,
                                                                ]),
                                                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                            },
                                                        },
                                                    },
                                                })),
                                                shape: data::type_::ValueShapeId(4),
                                            },
                                            kind: data::graph::ProfiledInstructionKind::Function(data::graph::FunctionInstruction {
                                                type_: data::type_::FunctionType {
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueType::Int,
                                                    ]),
                                                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                    })),
                                                },
                                                family: data::function::FunctionReturnFamily::Function,
                                                kind: data::graph::FunctionInstructionKind::Closure {
                                                    target: data::graph::FunctionTarget::Function(data::function::ProfiledFunctionFunctionId::Int(data::function::IntFunctionFunctionId(0))),
                                                    captures: data::Storage::Static(&[]),
                                                },
                                            }),
                                        }),
                                    ]),
                                },
                                exits: data::Storage::Static(&[
                                    data::function::FunctionExit::TailCall {
                                        function: data::source::FunctionCallTarget {
                                            function: 2,
                                            site: data::source::HostCallSite::from_static("entry_work", "main", data::source::SourceSpan::new(53, 146)),
                                        },
                                        args: data::Storage::Static(&[
                                            data::graph::ParamLocal::External(data::graph::ExternalLocal {
                                                id: data::graph::ExternalLocalId(0),
                                                type_id: data::type_::ExternalTypeId(1),
                                            }),
                                            data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                                                id: data::graph::CoreFunctionFunctionLocalId(0),
                                                type_: data::type_::FunctionFunctionType {
                                                    type_: data::type_::FunctionType {
                                                        arguments: data::Storage::Static(&[
                                                            data::type_::ValueType::Int,
                                                        ]),
                                                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        })),
                                                    },
                                                    arguments: data::Storage::Static(&[
                                                        data::type_::ValueShapeId(0),
                                                    ]),
                                                    return_: data::type_::FunctionShape {
                                                        shape_id: data::type_::ValueShapeId(1),
                                                        type_: data::type_::FunctionType {
                                                            arguments: data::Storage::Static(&[
                                                                data::type_::ValueType::Int,
                                                            ]),
                                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                                        },
                                                    },
                                                },
                                            })),
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
                                ]),
                            },
                        },
                    })),
                    data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                        index: 0,
                        return_: data::graph::ExternalLocal {
                            id: data::graph::ExternalLocalId(0),
                            type_id: data::type_::ExternalTypeId(1),
                        },
                        body: ::core::marker::PhantomData,
                    })),
                    data::function::ValueFunctionEntry::Host(data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
                        index: 1,
                        return_: data::graph::ExternalLocal {
                            id: data::graph::ExternalLocalId(0),
                            type_id: data::type_::ExternalTypeId(0),
                        },
                        body: ::core::marker::PhantomData,
                    })),
                ]),
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
                int_function_functions: data::Storage::Static(&[
                    data::function::ValueFunctionEntry::Graph(data::Storage::Static(&data::function::ExecutableFunction {
                        entry: data::function::FunctionEntry {
                            parameter_count: 1,
                        },
                        body: data::function::TypedFunctionBody {
                            _shape: data::type_::FunctionShape {
                                shape_id: data::type_::ValueShapeId(1),
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                                },
                            },
                            body: data::function::ProfiledFunctionBody {
                                block_graph: data::graph::ProfiledBlockGraph {
                                    entry: data::graph::BlockId(0),
                                    blocks: data::Storage::Static(&[
                                        data::graph::BlockHeader {
                                            params: 0..1,
                                            instructions: 0..1,
                                            terminator: data::graph::Terminator::Echo(data::graph::Echo {
                                                subject: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                                message: None,
                                                site: data::source::EchoSite::from_static("entry_work", "<anonymous:0>", data::source::SourceSpan::new(94, 108)),
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
                                            instructions: 1..2,
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
                                                shape: data::type_::ValueShapeId(1),
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
            use data::compiled::calls::{BoolCallable, CallExecution, CallInputs, CallInteger, CallOps, CallOutput, CallProgress, CallStorage, CallValues, IntCallable};
            enum FunctionState {
                Int0Point0 { int0: i128 },
                Int0Point1 { int0: i128, int1: i128 },
                IntFunction0Point0 { int0: i128 },
                IntFunction0Point1 { int0: i128, int1: i128 },
                IntFunction0Point2 {  },
                IntFunction0Point3 { int_function0: IntCallable },
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
                Int { value: i128, exit: data::graph::BlockGraphExitId },
                IntFunction { value: IntCallable, exit: data::graph::BlockGraphExitId },
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
                        data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(0)) => calls_intfunction_0_state(point, values),
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
                    FunctionState::Int0Point0 { int0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point0 { int0 }); }
                        *budget -= 1;
                        let int1 = int0 + 1_i128;
                        if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point: data::compiled::CompiledCheckpoint {
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
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point1 { int0, int1 }); }
                        *budget -= 1;
                        FunctionStep::Int { value: int1, exit: data::graph::BlockGraphExitId(0) }
                    },
                    FunctionState::Int0Point1 { int0, int1 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::Int0Point1 { int0, int1 }); }
                        *budget -= 1;
                        FunctionStep::Int { value: int1, exit: data::graph::BlockGraphExitId(0) }
                    },
                    FunctionState::IntFunction0Point0 { int0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction0Point0 { int0 }); }
                        *budget -= 1;
                        let int1 = int0 + 1_i128;
                        if int1 < i128::from(i64::MIN) || int1 > i128::from(i64::MAX) { return FunctionStep::Canonical { target: data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(0)), point: data::compiled::CompiledCheckpoint {
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
                        FunctionStep::Next(FunctionState::IntFunction0Point1 { int0, int1 })
                    },
                    FunctionState::IntFunction0Point1 { int0, int1 } => {
                        FunctionStep::Canonical { target: data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(0)), point: data::compiled::CompiledCheckpoint {
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
                    FunctionState::IntFunction0Point2 {  } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction0Point2 {  }); }
                        *budget -= 1;
                        let int_function0 = ops.int_closure(data::function::IntFunctionId(0), data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        }, vec![]);
                        FunctionStep::Next(FunctionState::IntFunction0Point3 { int_function0 })
                    },
                    FunctionState::IntFunction0Point3 { int_function0 } => {
                        if *budget == 0 { return FunctionStep::Yield(FunctionState::IntFunction0Point3 { int_function0 }); }
                        *budget -= 1;
                        FunctionStep::IntFunction { value: int_function0, exit: data::graph::BlockGraphExitId(0) }
                    },
                }
            }
            fn calls_int_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                let active = match point {
                    0 => FunctionState::Int0Point0 { int0: values.int(0)? },
                    1 => FunctionState::Int0Point1 { int0: values.int(0)?, int1: values.int(1)? },
                    _ => return None,
                };
                Some(active)
            }
            fn calls_int_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                if let Some(execution) = storage.reuse(data::compiled::CallTarget::Int(data::function::IntFunctionId(0)), point, values) { return Some(execution); }
                let active = calls_int_0_state(point, values)?;
                Some(Box::new(FunctionExecution::new(active)))
            }
            fn calls_intfunction_0_state(point: usize, values: CallInputs<'_>) -> Option<FunctionState> {
                let active = match point {
                    0 => FunctionState::IntFunction0Point0 { int0: values.int(0)? },
                    1 => FunctionState::IntFunction0Point1 { int0: values.int(0)?, int1: values.int(1)? },
                    2 => FunctionState::IntFunction0Point2 {  },
                    3 => FunctionState::IntFunction0Point3 { int_function0: values.int_function(0)? },
                    _ => return None,
                };
                Some(active)
            }
            fn calls_intfunction_0_start(point: usize, values: CallInputs<'_>, storage: &mut CallStorage) -> Option<Box<dyn CallExecution>> {
                if let Some(execution) = storage.reuse(data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(0)), point, values) { return Some(execution); }
                let active = calls_intfunction_0_state(point, values)?;
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
                        function: data::compiled::CallTarget::Int(data::function::IntFunctionId(0)),
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
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 1,
                                    value: data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                },
                            ]),
                            tails: data::Storage::Static(&[]),
                            start: calls_int_0_start,
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
                            ]),
                            locals: data::Storage::Static(&[
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                ]),
                                data::Storage::Static(&[
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                                    data::graph::ParamLocal::Int(data::graph::IntLocalId(1)),
                                ]),
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
                            ]),
                            calls: data::Storage::Static(&[]),
                            creations: data::Storage::Static(&[
                                data::compiled::CreationContract {
                                    point: 2,
                                    output: data::graph::ParamLocal::IntFunction {
                                        local: data::graph::IntFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
                                            ]),
                                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                                        },
                                    },
                                    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(0)),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    },
                                    reference: false,
                                    captures: data::Storage::Static(&[]),
                                },
                            ]),
                            returns: data::Storage::Static(&[
                                data::compiled::ReturnContract {
                                    point: 3,
                                    value: data::graph::ParamLocal::IntFunction {
                                        local: data::graph::IntFunctionLocalId(0),
                                        type_: data::type_::FunctionType {
                                            arguments: data::Storage::Static(&[
                                                data::type_::ValueType::Int,
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
                0..1,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                1..4,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
                0..0,
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
            ],
            functions: data::Storage::Static(&[
                data::function::FunctionContract {
                    parameters: 0..1,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 1..1,
                    parameter_shapes: data::Storage::Static(&[]),
                    return_: data::type_::ValueShapeId(5),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 1..2,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(3),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 2..4,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(3),
                        data::type_::ValueShapeId(4),
                    ]),
                    return_: data::type_::ValueShapeId(5),
                    captures: data::Storage::Static(&[]),
                },
                data::function::FunctionContract {
                    parameters: 4..5,
                    parameter_shapes: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                    captures: data::Storage::Static(&[]),
                },
            ]),
            parameters: data::Storage::Static(&[
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
                data::graph::ParamLocal::External(data::graph::ExternalLocal {
                    id: data::graph::ExternalLocalId(0),
                    type_id: data::type_::ExternalTypeId(1),
                }),
                data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                    id: data::graph::CoreFunctionFunctionLocalId(0),
                    type_: data::type_::FunctionFunctionType {
                        type_: data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            })),
                        },
                        arguments: data::Storage::Static(&[
                            data::type_::ValueShapeId(0),
                        ]),
                        return_: data::type_::FunctionShape {
                            shape_id: data::type_::ValueShapeId(1),
                            type_: data::type_::FunctionType {
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueType::Int,
                                ]),
                                return_: data::Storage::Static(&data::type_::ValueType::Int),
                            },
                        },
                    },
                })),
                data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
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
            types: data::Storage::Static(&[
                data::type_::NominalTypeMetadata {
                    package: data::Text::Static("work_fixture"),
                    module: data::Text::Static("fixture/work"),
                    name: data::Text::Static("Work"),
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                        }),
                    ]),
                },
                data::type_::NominalTypeMetadata {
                    package: data::Text::Static("work_fixture"),
                    module: data::Text::Static("fixture/work"),
                    name: data::Text::Static("Work"),
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                    ]),
                },
            ]),
        },
        value_shapes: data::type_::ValueShapeTable {
            shapes: data::Storage::Static(&[
                data::type_::ValueShapeDescriptor::Int,
                data::type_::ValueShapeDescriptor::Function {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(0),
                },
                data::type_::ValueShapeDescriptor::String,
                data::type_::ValueShapeDescriptor::External(data::type_::ExternalTypeId(1)),
                data::type_::ValueShapeDescriptor::Function {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueShapeId(0),
                    ]),
                    return_: data::type_::ValueShapeId(1),
                },
                data::type_::ValueShapeDescriptor::External(data::type_::ExternalTypeId(0)),
            ]),
            shape_types: data::Storage::Static(&[
                data::type_::ValueType::Int,
                data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                }),
                data::type_::ValueType::String,
                data::type_::ValueType::External(data::type_::ExternalTypeId(1)),
                data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[
                        data::type_::ValueType::Int,
                    ]),
                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                    })),
                }),
                data::type_::ValueType::External(data::type_::ExternalTypeId(0)),
            ]),
            custom_shapes: data::Storage::Static(&[]),
        },
    },
    value_functions: data::Storage::Static(&[
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("work_fixture"),
            site: data::source::HostCallSite::from_static("fixture/work", "ready", data::source::SourceSpan::new(60, 86)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::Int,
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                    package: data::Text::Static("work_fixture"),
                    module: data::Text::Static("fixture/work"),
                    name: data::Text::Static("Work"),
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Int,
                    ]),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Int,
                    shape: data::type_::ValueShapeId(0),
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
                    entries: data::Storage::Static(&[]),
                },
                externals: data::host::ConstructionIndex {
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("work_fixture"),
                            module: data::Text::Static("fixture/work"),
                            name: data::Text::Static("Work"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                        }), data::type_::ExternalTypeId(1)),
                    ]),
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
                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(1))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 1,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::Parameter(0),
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::External {
                    schema: data::host::ExternalSchema {
                        package: data::Text::Static("work_fixture"),
                        module: data::Text::Static("fixture/work"),
                        name: data::Text::Static("Work"),
                        parameter_count: 1,
                    },
                    arguments: data::Storage::Static(&[
                        data::host::RegistrationType::Parameter(0),
                    ]),
                },
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::Value(0),
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("work_fixture"),
                        module: data::Text::Static("fixture/work"),
                        name: data::Text::Static("Work"),
                        parameter_count: 1,
                    },
                ]),
                constructions: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
        data::host::HostedFunctionMetadata {
            completion: data::host::HostFunctionCompletion::Value,
            callable_entry: None,
            package: data::Text::Static("work_fixture"),
            site: data::source::HostCallSite::from_static("fixture/work", "map", data::source::SourceSpan::new(139, 187)),
            signature: data::type_::FunctionMetadata {
                arguments: data::Storage::Static(&[
                    data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                        package: data::Text::Static("work_fixture"),
                        module: data::Text::Static("fixture/work"),
                        name: data::Text::Static("Work"),
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                    }),
                    data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                        })),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                    package: data::Text::Static("work_fixture"),
                    module: data::Text::Static("fixture/work"),
                    name: data::Text::Static("Work"),
                    arguments: data::Storage::Static(&[
                        data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                        }),
                    ]),
                })),
            },
            type_arguments: data::Storage::Static(&[
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                        arguments: data::Storage::Static(&[
                            data::type_::TypeMetadata::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                    }),
                    shape: data::type_::ValueShapeId(1),
                },
                data::host::HostTypeArgument {
                    type_: data::type_::TypeMetadata::Int,
                    shape: data::type_::ValueShapeId(0),
                },
            ]),
            parameters: data::host::HostedFunctionParameters {
                call: data::Storage::Static(&[
                    data::host::HostCallParameter::External(data::graph::ParamLocal::External(data::graph::ExternalLocal {
                        id: data::graph::ExternalLocalId(0),
                        type_id: data::type_::ExternalTypeId(1),
                    })),
                    data::host::HostCallParameter::Function {
                        local: data::graph::ParamLocal::FunctionFunction(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
                            id: data::graph::CoreFunctionFunctionLocalId(0),
                            type_: data::type_::FunctionFunctionType {
                                type_: data::type_::FunctionType {
                                    arguments: data::Storage::Static(&[
                                        data::type_::ValueType::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    })),
                                },
                                arguments: data::Storage::Static(&[
                                    data::type_::ValueShapeId(0),
                                ]),
                                return_: data::type_::FunctionShape {
                                    shape_id: data::type_::ValueShapeId(1),
                                    type_: data::type_::FunctionType {
                                        arguments: data::Storage::Static(&[
                                            data::type_::ValueType::Int,
                                        ]),
                                        return_: data::Storage::Static(&data::type_::ValueType::Int),
                                    },
                                },
                            },
                        })),
                        arity: 1,
                    },
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
                    entries: data::Storage::Static(&[
                        (data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("work_fixture"),
                            module: data::Text::Static("fixture/work"),
                            name: data::Text::Static("Work"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Int,
                            ]),
                        }), data::type_::ExternalTypeId(1)),
                        (data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {
                            package: data::Text::Static("work_fixture"),
                            module: data::Text::Static("fixture/work"),
                            name: data::Text::Static("Work"),
                            arguments: data::Storage::Static(&[
                                data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {
                                    arguments: data::Storage::Static(&[
                                        data::type_::TypeMetadata::Int,
                                    ]),
                                    return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
                                }),
                            ]),
                        }), data::type_::ExternalTypeId(0)),
                    ]),
                },
                natives: data::host::NativeConversions {
                    roots: data::Storage::Static(&[]),
                    nodes: data::Storage::Static(&[]),
                },
                callables: data::Storage::Static(&[]),
            },
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[
                    data::type_::ValueType::External(data::type_::ExternalTypeId(1)),
                    data::type_::ValueType::Function(data::type_::FunctionType {
                        arguments: data::Storage::Static(&[
                            data::type_::ValueType::Int,
                        ]),
                        return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                            arguments: data::Storage::Static(&[
                                data::type_::ValueType::Int,
                            ]),
                            return_: data::Storage::Static(&data::type_::ValueType::Int),
                        })),
                    }),
                ]),
                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(0))),
            },
            registration: data::Storage::Static(&data::host::RegistrationContract {
                parameter_count: 2,
                parameters: data::Storage::Static(&[
                    data::host::RegistrationType::External {
                        schema: data::host::ExternalSchema {
                            package: data::Text::Static("work_fixture"),
                            module: data::Text::Static("fixture/work"),
                            name: data::Text::Static("Work"),
                            parameter_count: 1,
                        },
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Parameter(1),
                        ]),
                    },
                    data::host::RegistrationType::Function {
                        arguments: data::Storage::Static(&[
                            data::host::RegistrationType::Parameter(1),
                        ]),
                        return_: data::Storage::Static(&data::host::RegistrationType::Parameter(0)),
                    },
                ]),
                captures: data::Storage::Static(&[]),
                callable: false,
                callable_constructions: data::Storage::Static(&[]),
                return_: data::host::RegistrationType::External {
                    schema: data::host::ExternalSchema {
                        package: data::Text::Static("work_fixture"),
                        module: data::Text::Static("fixture/work"),
                        name: data::Text::Static("Work"),
                        parameter_count: 1,
                    },
                    arguments: data::Storage::Static(&[
                        data::host::RegistrationType::Parameter(0),
                    ]),
                },
                layout: data::Storage::Static(&[
                    data::host::RegistrationParameter::External(0),
                    data::host::RegistrationParameter::Function {
                        slot: 0,
                        arity: 1,
                    },
                ]),
                custom_schemas: data::Storage::Static(&[]),
                external_schemas: data::Storage::Static(&[
                    data::host::ExternalSchema {
                        package: data::Text::Static("work_fixture"),
                        module: data::Text::Static("fixture/work"),
                        name: data::Text::Static("Work"),
                        parameter_count: 1,
                    },
                ]),
                constructions: data::Storage::Static(&[]),
                construction_customs: data::Storage::Static(&[]),
                construction_externals: data::Storage::Static(&[]),
                native_rules: None,
            }),
        },
    ]),
    never_functions: data::Storage::Static(&[]),
}
