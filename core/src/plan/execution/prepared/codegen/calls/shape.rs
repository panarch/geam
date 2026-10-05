use super::super::shape::{
    CompiledBoolean, CompiledInstruction, CompiledShape, CompiledTest, KernelKind,
    NumericComparison, NumericInteger,
};
use crate::plan::HostCallSite;
use crate::plan::execution::compiled::{
    CallContract, CallContractTarget, CallTarget, CompiledCheckpoint, CreationContract,
    ReturnContract, TailContract,
};
use crate::plan::execution::function::{
    BoolFunctionFunctionId, BoolFunctionId, ExecutionFunctionEntry, ExecutionFunctionRef,
    ExecutionGraphProfile, ExecutionProfile, FunctionBodyOwner, FunctionExit, FunctionTables,
    IntFunctionFunctionId, IntFunctionId, ProfiledFunctionBody, ProfiledFunctionFunctionId,
};
use crate::plan::execution::graph::{
    ArithmeticRegion, BlockGraphExitId, BlockId, BoolFunctionLocalId, BoolInstruction, BoolLocalId,
    BoolTest, Edge, FunctionCapture, FunctionInstructionKind, FunctionTarget, IntFunctionLocalId,
    IntInstruction, IntLocalId, IntegerLiteral, IntegerOperand, ParamLocal, ProfiledInstruction,
    ProfiledInstructionKind, Terminator,
};
use crate::plan::execution::type_::FunctionType;
use std::collections::BTreeMap;

pub(in crate::plan::execution::prepared) struct CallProgram<'graph, Graph: ExecutionGraphProfile> {
    pub(super) functions: Vec<CallFunction<'graph, Graph>>,
}

pub(super) struct CallFunction<'graph, Graph: ExecutionGraphProfile> {
    pub(super) target: CallTarget,
    pub(super) shape: CallShape<'graph>,
    pub(super) numeric: Option<CompiledShape<'graph, Graph>>,
    pub(super) numeric_returns: Vec<CallLocal>,
}

impl<'graph, Graph: ExecutionGraphProfile> CallProgram<'graph, Graph> {
    pub(in crate::plan::execution::prepared) fn inspect<
        Profile: ExecutionProfile<Graph = Graph>,
    >(
        functions: &'graph FunctionTables<Profile>,
    ) -> Self {
        let mut selected = Vec::new();
        for (index, entry) in functions.value_returns.int_functions.iter().enumerate() {
            if let ExecutionFunctionRef::Graph(function) = entry.as_ref()
                && let Some(shape) = CallShape::inspect(
                    function.body(),
                    function.entry().parameter_count,
                    false,
                    |local| CallLocal::Int(*local),
                    |target| CallTarget::Int(*target.function()),
                    |target| target.site().clone(),
                )
            {
                selected.push(CallFunction {
                    target: CallTarget::Int(IntFunctionId(index)),
                    numeric: CompiledShape::inspect(function.body()).filter(|numeric| {
                        numeric.kind == KernelKind::Numeric
                            && function
                                .body()
                                .exits
                                .iter()
                                .all(|exit| matches!(exit, FunctionExit::Return(_)))
                    }),
                    numeric_returns: function
                        .body()
                        .exits
                        .iter()
                        .filter_map(|exit| match exit {
                            FunctionExit::Return(local) => Some(CallLocal::Int(*local)),
                            FunctionExit::TailCall { .. } => None,
                        })
                        .collect(),
                    shape,
                });
            }
        }
        for (index, entry) in functions.value_returns.bool_functions.iter().enumerate() {
            if let ExecutionFunctionRef::Graph(function) = entry.as_ref()
                && let Some(shape) = CallShape::inspect(
                    function.body(),
                    function.entry().parameter_count,
                    false,
                    |local| CallLocal::Bool(*local),
                    |target| CallTarget::Bool(*target.function()),
                    |target| target.site().clone(),
                )
            {
                selected.push(CallFunction {
                    target: CallTarget::Bool(BoolFunctionId(index)),
                    numeric: CompiledShape::inspect(function.body()).filter(|numeric| {
                        numeric.kind == KernelKind::Numeric
                            && function
                                .body()
                                .exits
                                .iter()
                                .all(|exit| matches!(exit, FunctionExit::Return(_)))
                    }),
                    numeric_returns: function
                        .body()
                        .exits
                        .iter()
                        .filter_map(|exit| match exit {
                            FunctionExit::Return(local) => Some(CallLocal::Bool(*local)),
                            FunctionExit::TailCall { .. } => None,
                        })
                        .collect(),
                    shape,
                });
            }
        }
        for (index, entry) in functions
            .function_returns
            .int_function_functions
            .iter()
            .enumerate()
        {
            if let ExecutionFunctionRef::Graph(function) = entry.as_ref()
                && let Some(shape) = CallShape::inspect(
                    FunctionBodyOwner::function_body(function.body()),
                    function.entry().parameter_count,
                    true,
                    |local| CallLocal::IntFunction {
                        local: *local,
                        type_: function.body()._shape.type_.clone(),
                    },
                    |target| CallTarget::IntFunction(*target.function()),
                    |target| target.site().clone(),
                )
            {
                selected.push(CallFunction {
                    target: CallTarget::IntFunction(IntFunctionFunctionId(index)),
                    shape,
                    numeric: None,
                    numeric_returns: Vec::new(),
                });
            }
        }
        for (index, entry) in functions
            .function_returns
            .bool_function_functions
            .iter()
            .enumerate()
        {
            if let ExecutionFunctionRef::Graph(function) = entry.as_ref()
                && let Some(shape) = CallShape::inspect(
                    FunctionBodyOwner::function_body(function.body()),
                    function.entry().parameter_count,
                    true,
                    |local| CallLocal::BoolFunction {
                        local: *local,
                        type_: function.body()._shape.type_.clone(),
                    },
                    |target| CallTarget::BoolFunction(*target.function()),
                    |target| target.site().clone(),
                )
            {
                selected.push(CallFunction {
                    target: CallTarget::BoolFunction(BoolFunctionFunctionId(index)),
                    shape,
                    numeric: None,
                    numeric_returns: Vec::new(),
                });
            }
        }
        let roots = selected
            .iter()
            .map(|function| {
                function.shape.root
                    && match function.shape.points[function.shape.entry()] {
                        CallPoint::Call(index) => {
                            let call = &function.shape.calls[index];
                            matches!(call.target, CallContractTarget::Static(_))
                                || selected.iter().any(|callee| callee.accepts_call(call))
                        }
                        _ => true,
                    }
            })
            .collect::<Vec<_>>();
        for (function, root) in selected.iter_mut().zip(roots) {
            function.shape.root = root;
        }
        if !selected.iter().any(|function| function.shape.root) {
            selected.clear();
        }
        Self {
            functions: selected,
        }
    }

    pub(in crate::plan::execution::prepared) fn shapes(
        &self,
    ) -> impl Iterator<Item = (CallTarget, &CallShape<'graph>)> {
        self.functions
            .iter()
            .map(|function| (function.target, &function.shape))
    }
}

impl<Graph: ExecutionGraphProfile> CallFunction<'_, Graph> {
    pub(super) fn accepts_call(&self, call: &CallInvocation) -> bool {
        matches!(
            (&call.output, self.target),
            (CallLocal::Int(_), CallTarget::Int(_))
                | (CallLocal::Bool(_), CallTarget::Bool(_))
                | (CallLocal::IntFunction { .. }, CallTarget::IntFunction(_))
                | (CallLocal::BoolFunction { .. }, CallTarget::BoolFunction(_))
        ) && self.matches_parameters(&call.args)
    }

    pub(super) fn matches_parameters(&self, args: &[CallLocal]) -> bool {
        let entry = &self.shape.locals[self.shape.entry()];
        self.shape.parameter_count == args.len()
            && entry
                .iter()
                .take(self.shape.parameter_count)
                .zip(args)
                .all(|(parameter, argument)| parameter.same_type(argument))
    }
}

/// Only supported locals enter the renderer. Canonical metadata is materialized
/// from these preparation-local views, never interpreted as a second graph.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum CallLocal {
    Int(IntLocalId),
    Bool(BoolLocalId),
    IntFunction {
        local: IntFunctionLocalId,
        type_: FunctionType,
    },
    BoolFunction {
        local: BoolFunctionLocalId,
        type_: FunctionType,
    },
}

impl CallLocal {
    pub(super) fn same_type(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(_), Self::Int(_)) | (Self::Bool(_), Self::Bool(_)) => true,
            (Self::IntFunction { type_: left, .. }, Self::IntFunction { type_: right, .. })
            | (Self::BoolFunction { type_: left, .. }, Self::BoolFunction { type_: right, .. }) => {
                left == right
            }
            _ => false,
        }
    }

    pub(super) fn inspect(local: &ParamLocal) -> Option<Self> {
        Some(match local {
            ParamLocal::Int(local) => Self::Int(*local),
            ParamLocal::Bool(local) => Self::Bool(*local),
            ParamLocal::IntFunction { local, type_ } => Self::IntFunction {
                local: *local,
                type_: type_.clone(),
            },
            ParamLocal::BoolFunction { local, type_ } => Self::BoolFunction {
                local: *local,
                type_: type_.clone(),
            },
            _ => return None,
        })
    }

    pub(super) fn canonical(&self) -> ParamLocal {
        match self {
            Self::Int(local) => ParamLocal::Int(*local),
            Self::Bool(local) => ParamLocal::Bool(*local),
            Self::IntFunction { local, type_ } => ParamLocal::IntFunction {
                local: *local,
                type_: type_.clone(),
            },
            Self::BoolFunction { local, type_ } => ParamLocal::BoolFunction {
                local: *local,
                type_: type_.clone(),
            },
        }
    }
}

pub(super) struct CallInvocation {
    pub point: usize,
    pub output: CallLocal,
    pub target: CallContractTarget,
    pub args: Vec<CallLocal>,
    pub site: HostCallSite,
}

impl CallInvocation {
    pub(super) fn contract(&self) -> CallContract {
        CallContract {
            point: self.point,
            output: self.output.canonical(),
            target: self.target.clone(),
            args: self.args.iter().map(CallLocal::canonical).collect(),
            site: self.site.clone(),
        }
    }
}

pub(super) enum CallableTarget {
    Int(IntFunctionId),
    Bool(BoolFunctionId),
}

pub(super) enum Capture {
    Int {
        target: IntLocalId,
        source: IntLocalId,
    },
    Bool {
        target: BoolLocalId,
        source: BoolLocalId,
    },
    IntFunction {
        target: IntFunctionLocalId,
        source: IntFunctionLocalId,
    },
    BoolFunction {
        target: BoolFunctionLocalId,
        source: BoolFunctionLocalId,
    },
}

impl Capture {
    fn inspect(capture: &FunctionCapture) -> Option<Self> {
        Some(match capture {
            FunctionCapture::Int { target, source } => Self::Int {
                target: *target,
                source: *source,
            },
            FunctionCapture::Bool { target, source } => Self::Bool {
                target: *target,
                source: *source,
            },
            FunctionCapture::IntFunction { target, source } => Self::IntFunction {
                target: *target,
                source: *source,
            },
            FunctionCapture::BoolFunction { target, source } => Self::BoolFunction {
                target: *target,
                source: *source,
            },
            _ => return None,
        })
    }

    fn canonical(&self) -> FunctionCapture {
        match self {
            Self::Int { target, source } => FunctionCapture::Int {
                target: *target,
                source: *source,
            },
            Self::Bool { target, source } => FunctionCapture::Bool {
                target: *target,
                source: *source,
            },
            Self::IntFunction { target, source } => FunctionCapture::IntFunction {
                target: *target,
                source: *source,
            },
            Self::BoolFunction { target, source } => FunctionCapture::BoolFunction {
                target: *target,
                source: *source,
            },
        }
    }
}

pub(super) struct CallCreation {
    pub point: usize,
    pub output: CallLocal,
    pub target: CallableTarget,
    pub type_: FunctionType,
    pub reference: bool,
    pub captures: Vec<Capture>,
}

impl CallCreation {
    pub(super) fn contract(&self) -> CreationContract {
        CreationContract {
            point: self.point,
            output: self.output.canonical(),
            target: match self.target {
                CallableTarget::Int(id) => FunctionTarget::Int(id),
                CallableTarget::Bool(id) => FunctionTarget::Bool(id),
            },
            type_: self.type_.clone(),
            reference: self.reference,
            captures: self.captures.iter().map(Capture::canonical).collect(),
        }
    }
}

pub(super) struct CallReturn {
    pub point: usize,
    pub value: CallLocal,
    pub exit: BlockGraphExitId,
}

impl CallReturn {
    pub(super) fn contract(&self) -> ReturnContract {
        ReturnContract {
            point: self.point,
            value: self.value.canonical(),
        }
    }
}

pub(super) struct CallTail {
    pub point: usize,
    pub target: CallTarget,
    pub args: Vec<CallLocal>,
    pub site: HostCallSite,
}

impl CallTail {
    pub(super) fn contract(&self) -> TailContract {
        TailContract {
            point: self.point,
            target: self.target,
            args: self.args.iter().map(CallLocal::canonical).collect(),
            site: self.site.clone(),
        }
    }
}

/// Phase-local views of the frozen graph. The executable sidecar stores only
/// its admission contracts; generated Rust owns the actual typed call states.
pub(in crate::plan::execution::prepared) struct CallShape<'graph> {
    pub root: bool,
    pub parameter_count: usize,
    entry: usize,
    pub checkpoints: Vec<CompiledCheckpoint>,
    pub(super) locals: Vec<Vec<CallLocal>>,
    pub(super) calls: Vec<CallInvocation>,
    pub(super) creations: Vec<CallCreation>,
    pub(super) returns: Vec<CallReturn>,
    pub(super) tails: Vec<CallTail>,
    pub(super) points: Vec<CallPoint<'graph>>,
    pub(super) starts: BTreeMap<usize, usize>,
}

pub(super) enum CallPoint<'graph> {
    Scalar(CallScalar<'graph>),
    Call(usize),
    Create(usize),
    Terminator(CallTerminator<'graph>),
    Return(usize),
    Tail(usize),
    Interpreted,
}

impl<'graph> CallShape<'graph> {
    fn inspect<Return, Tail, Graph: ExecutionGraphProfile>(
        body: &'graph ProfiledFunctionBody<Return, Tail, Graph>,
        parameter_count: usize,
        returning_callable: bool,
        return_local: impl Fn(&Return) -> CallLocal,
        tail_target: impl Fn(&Tail) -> CallTarget,
        tail_site: impl Fn(&Tail) -> HostCallSite,
    ) -> Option<Self> {
        let graph = body.block_graph().as_view();
        let mut shape = Self {
            root: returning_callable,
            parameter_count,
            entry: 0,
            checkpoints: Vec::new(),
            locals: Vec::new(),
            calls: Vec::new(),
            creations: Vec::new(),
            returns: Vec::new(),
            tails: Vec::new(),
            points: Vec::new(),
            starts: BTreeMap::new(),
        };
        for (index, block) in graph.blocks().enumerate() {
            let Some(mut locals) = block
                .params()
                .iter()
                .map(|slot| CallLocal::inspect(slot.local()))
                .collect::<Option<Vec<_>>>()
            else {
                continue;
            };
            shape.root |= locals.iter().any(callable_local);
            shape.starts.insert(index, shape.points.len());
            let mut complete = true;
            for (instruction_index, instruction) in block.instructions().iter().enumerate() {
                let point = shape.push_point(BlockId(index), instruction_index, &locals);
                let Some(instruction) = inspect_instruction(instruction, point, &mut shape) else {
                    shape.points.push(CallPoint::Interpreted);
                    complete = false;
                    break;
                };
                shape.points.push(instruction);
                locals.extend(
                    block.instructions()[instruction_index]
                        .outputs()
                        .filter_map(|slot| CallLocal::inspect(slot.local())),
                );
            }
            if complete {
                let point = shape.push_point(BlockId(index), block.instructions().len(), &locals);
                let terminator = match block.terminator() {
                    Terminator::Exit(exit) => match body.exit(*exit) {
                        FunctionExit::Return(value) => {
                            let value = return_local(value);
                            let index = shape.returns.len();
                            shape.returns.push(CallReturn {
                                point,
                                value,
                                exit: *exit,
                            });
                            CallPoint::Return(index)
                        }
                        FunctionExit::TailCall { function, args, .. } => {
                            let index = shape.tails.len();
                            shape.tails.push(CallTail {
                                point,
                                target: tail_target(function),
                                args: args.iter().filter_map(CallLocal::inspect).collect(),
                                site: tail_site(function),
                            });
                            CallPoint::Tail(index)
                        }
                    },
                    terminator => CallTerminator::inspect(terminator)
                        .map(CallPoint::Terminator)
                        .unwrap_or(CallPoint::Interpreted),
                };
                shape.points.push(terminator);
            }
        }
        let entry = *shape.starts.get(&graph.entry().index())?;
        if matches!(shape.points[entry], CallPoint::Interpreted) {
            return None;
        }
        shape.entry = entry;
        Some(shape)
    }

    pub(in crate::plan::execution::prepared) fn entry(&self) -> usize {
        self.entry
    }

    pub(in crate::plan::execution::prepared) fn local_contracts(&self) -> Vec<Vec<ParamLocal>> {
        self.locals
            .iter()
            .map(|locals| locals.iter().map(CallLocal::canonical).collect())
            .collect()
    }
    pub(in crate::plan::execution::prepared) fn call_contracts(&self) -> Vec<CallContract> {
        self.calls.iter().map(CallInvocation::contract).collect()
    }
    pub(in crate::plan::execution::prepared) fn creation_contracts(&self) -> Vec<CreationContract> {
        self.creations.iter().map(CallCreation::contract).collect()
    }
    pub(in crate::plan::execution::prepared) fn return_contracts(&self) -> Vec<ReturnContract> {
        self.returns.iter().map(CallReturn::contract).collect()
    }
    pub(in crate::plan::execution::prepared) fn tail_contracts(&self) -> Vec<TailContract> {
        self.tails.iter().map(CallTail::contract).collect()
    }

    fn push_point(&mut self, block: BlockId, instruction: usize, locals: &[CallLocal]) -> usize {
        let point = self.checkpoints.len();
        self.checkpoints.push(CompiledCheckpoint {
            block,
            instruction,
            ints: locals
                .iter()
                .filter(|local| matches!(local, CallLocal::Int(_)))
                .count(),
            bools: locals
                .iter()
                .filter(|local| matches!(local, CallLocal::Bool(_)))
                .count(),
            bit_arrays: 0,
            int_lists: 0,
            strings: 0,
            customs: 0,
            custom_lists: 0,
            int_functions: locals
                .iter()
                .filter(|local| matches!(local, CallLocal::IntFunction { .. }))
                .count(),
            bool_functions: locals
                .iter()
                .filter(|local| matches!(local, CallLocal::BoolFunction { .. }))
                .count(),
        });
        self.locals.push(locals.to_vec());
        point
    }
}

pub(super) enum CallScalar<'graph> {
    Integer(IntLocalId, NumericInteger<'graph>),
    Boolean(BoolLocalId, CallBoolean),
    Region {
        region: &'graph ArithmeticRegion,
        outputs: Vec<IntLocalId>,
    },
}

pub(super) enum CallBoolean {
    Value(bool),
    Test(CallTest),
}

pub(super) enum CallTest {
    Not(BoolLocalId),
    Compare(NumericComparison, IntegerOperand, IntegerOperand),
}

impl CallTest {
    fn from_compiled(test: CompiledTest<'_>) -> Option<Self> {
        match test {
            CompiledTest::Not(local) => Some(Self::Not(local)),
            CompiledTest::Compare(comparison, left, right) => {
                Some(Self::Compare(comparison, left, right))
            }
            CompiledTest::IntList(_) | CompiledTest::String(_) | CompiledTest::BoolEqual { .. } => {
                None
            }
            CompiledTest::CustomListLength { .. } => None,
        }
    }

    fn inspect(test: &BoolTest) -> Option<Self> {
        Self::from_compiled(CompiledTest::inspect(test, KernelKind::Numeric)?)
    }
}

impl<'graph> CallScalar<'graph> {
    fn inspect<Graph: ExecutionGraphProfile>(
        instruction: &'graph ProfiledInstruction<Graph>,
    ) -> Option<Self> {
        match CompiledInstruction::inspect(instruction, KernelKind::Numeric)? {
            CompiledInstruction::Integer(output, value) => Some(Self::Integer(output, value)),
            CompiledInstruction::Boolean(output, CompiledBoolean::Value(value)) => {
                Some(Self::Boolean(output, CallBoolean::Value(value)))
            }
            CompiledInstruction::Boolean(output, CompiledBoolean::Test(test)) => Some(
                Self::Boolean(output, CallBoolean::Test(CallTest::from_compiled(test)?)),
            ),
            CompiledInstruction::Region { region, outputs } => {
                Some(Self::Region { region, outputs })
            }
            CompiledInstruction::IntList(_)
            | CompiledInstruction::String(_, _)
            | CompiledInstruction::CustomField(_)
            | CompiledInstruction::CustomLoop(_) => None,
        }
    }
}

pub(super) enum CallTerminator<'graph> {
    Jump(&'graph Edge),
    Boolean {
        subject: BoolLocalId,
        true_: &'graph Edge,
        false_: &'graph Edge,
    },
    Test {
        test: CallTest,
        true_: &'graph Edge,
        false_: &'graph Edge,
    },
    Switch {
        subject: IntLocalId,
        clauses: &'graph [(IntegerLiteral, Edge)],
        fallback: &'graph Edge,
    },
}

impl<'graph> CallTerminator<'graph> {
    fn inspect(terminator: &'graph Terminator) -> Option<Self> {
        Some(match terminator {
            Terminator::Jump(jump) => Self::Jump(&jump.edge),
            Terminator::BoolBranch(branch) => Self::Boolean {
                subject: branch.subject,
                true_: &branch.true_,
                false_: &branch.false_,
            },
            Terminator::TestBranch(branch) => Self::Test {
                test: CallTest::inspect(&branch.test)?,
                true_: &branch.true_,
                false_: &branch.false_,
            },
            Terminator::IntSwitch(switch)
                if switch
                    .clauses
                    .iter()
                    .all(|(literal, _)| small_literal(literal)) =>
            {
                Self::Switch {
                    subject: switch.subject,
                    clauses: &switch.clauses,
                    fallback: &switch.fallback,
                }
            }
            _ => return None,
        })
    }
}

fn small_literal(literal: &IntegerLiteral) -> bool {
    i64::try_from(literal.materialize()).is_ok()
}

pub(super) fn supported_local(local: &ParamLocal) -> bool {
    matches!(
        local,
        ParamLocal::Int(_)
            | ParamLocal::Bool(_)
            | ParamLocal::IntFunction { .. }
            | ParamLocal::BoolFunction { .. }
    )
}

fn callable_local(local: &CallLocal) -> bool {
    matches!(
        local,
        CallLocal::IntFunction { .. } | CallLocal::BoolFunction { .. }
    )
}

fn inspect_instruction<'graph, Graph: ExecutionGraphProfile>(
    instruction: &'graph ProfiledInstruction<Graph>,
    point: usize,
    shape: &mut CallShape<'graph>,
) -> Option<CallPoint<'graph>> {
    // Scalar classification already establishes its typed outputs. Other
    // instructions have one output, inspected once below.
    if let Some(scalar) = CallScalar::inspect(instruction) {
        return Some(CallPoint::Scalar(scalar));
    }
    let instruction = instruction.value()?;
    let output = CallLocal::inspect(instruction.output().local())?;
    let (target, args, site) = match instruction.kind() {
        ProfiledInstructionKind::Int(IntInstruction::Call {
            function,
            args,
            site,
        }) => (
            CallContractTarget::Static(CallTarget::Int(*function)),
            args,
            site,
        ),
        ProfiledInstructionKind::Int(IntInstruction::FunctionCall {
            function,
            args,
            site,
        }) => (CallContractTarget::IntValue(*function), args, site),
        ProfiledInstructionKind::Bool(BoolInstruction::Call {
            function,
            args,
            site,
        }) => (
            CallContractTarget::Static(CallTarget::Bool(*function)),
            args,
            site,
        ),
        ProfiledInstructionKind::Bool(BoolInstruction::FunctionCall {
            function,
            args,
            site,
        }) => (CallContractTarget::BoolValue(*function), args, site),
        ProfiledInstructionKind::Function(function) => match function.kind() {
            FunctionInstructionKind::Call {
                function: ProfiledFunctionFunctionId::Int(function),
                args,
                site,
            } => (
                CallContractTarget::Static(CallTarget::IntFunction(*function)),
                args,
                site,
            ),
            FunctionInstructionKind::Call {
                function: ProfiledFunctionFunctionId::Bool(function),
                args,
                site,
            } => (
                CallContractTarget::Static(CallTarget::BoolFunction(*function)),
                args,
                site,
            ),
            kind => {
                let target = match kind {
                    FunctionInstructionKind::Reference(FunctionTarget::Int(id))
                    | FunctionInstructionKind::Closure {
                        target: FunctionTarget::Int(id),
                        ..
                    } => CallableTarget::Int(*id),
                    FunctionInstructionKind::Reference(FunctionTarget::Bool(id))
                    | FunctionInstructionKind::Closure {
                        target: FunctionTarget::Bool(id),
                        ..
                    } => CallableTarget::Bool(*id),
                    _ => return None,
                };
                let captures = match function.kind() {
                    FunctionInstructionKind::Closure { captures, .. } => captures
                        .iter()
                        .map(Capture::inspect)
                        .collect::<Option<Vec<_>>>()?,
                    _ => Vec::new(),
                };
                let index = shape.creations.len();
                shape.creations.push(CallCreation {
                    point,
                    output: output.clone(),
                    target,
                    type_: function.type_().clone(),
                    reference: matches!(function.kind(), FunctionInstructionKind::Reference(_)),
                    captures,
                });
                shape.root = true;
                return Some(CallPoint::Create(index));
            }
        },
        _ => return None,
    };
    if !args.iter().all(supported_local) {
        return None;
    }
    let index = shape.calls.len();
    shape.calls.push(CallInvocation {
        point,
        output: output.clone(),
        target,
        args: args.iter().filter_map(CallLocal::inspect).collect(),
        site: site.clone(),
    });
    shape.root = true;
    Some(CallPoint::Call(index))
}

#[cfg(test)]
mod tests {
    use super::{
        CallLocal, CallProgram, CallScalar, CallTarget, CallTest, Capture, CompiledBoolean,
        CompiledInstruction, CompiledTest, KernelKind, inspect_instruction, supported_local,
    };
    use crate::plan::execution::compiled::{CallContractTarget, CompiledCheckpoint};
    use crate::plan::execution::function::{
        BoolFunctionId, ExecutionFunctionEntry, ExecutionFunctionRef, IntFunctionFunctionId,
        IntFunctionId,
    };
    use crate::plan::execution::graph::{
        BlockId, BoolFunctionLocalId, BoolInstruction, BoolLocalId, BoolTest, CustomListLocalId,
        FunctionCapture, FunctionInstructionKind, IntFunctionLocalId, IntInstruction,
        IntListLocalId, IntLocalId, ListLocal, ParamLocal, ParamSlot, ProfiledInstruction,
        ProfiledInstructionKind, StringLocalId,
    };
    use crate::plan::execution::prepared::codegen::int_list::IntListTest;
    use crate::plan::execution::prepared::codegen::string::StringTest;
    use crate::plan::execution::type_::{
        CustomListTypeId, CustomTypeId, FunctionType, ListTypeId, ValueShapeId, ValueType,
    };
    use num_bigint::BigInt;
    use std::convert::Infallible;

    #[test]
    fn a_wide_arithmetic_region_after_a_call_keeps_its_canonical_big_integer_result() {
        let source = r#"
fn identity(value: Int) { value }
pub fn main() {
  let calculate = identity
  let value = calculate(1099511627776)
  value * value * value * value
}
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let main = match plan.program.functions.value_returns.int_functions[0].as_ref() {
            ExecutionFunctionRef::Graph(body) => body,
            ExecutionFunctionRef::Host(never) => match *never {},
        };
        let graph = main.body().block_graph().as_view();
        let (region, arithmetic) = graph
            .blocks()
            .flat_map(|block| block.instructions())
            .find_map(|instruction| match instruction {
                ProfiledInstruction::IntegerRegion(arithmetic) => Some((instruction, arithmetic)),
                _ => None,
            })
            .unwrap();
        assert!(!arithmetic.native);
        let mut program = CallProgram::inspect(&plan.program.functions);
        let main = program
            .functions
            .iter_mut()
            .find(|function| function.target == CallTarget::Int(IntFunctionId(0)))
            .unwrap();
        assert!(inspect_instruction(region, 0, &mut main.shape).is_none());
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Int(BigInt::from(1) << 160_usize)
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn a_native_arithmetic_region_after_a_call_preserves_its_integer_output() {
        let source = r#"
fn identity(value: Int) { value }
pub fn main() {
  let calculate = identity
  let value = calculate(4)
  value * 2 + 3
}
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let main = match plan.program.functions.value_returns.int_functions[0].as_ref() {
            ExecutionFunctionRef::Graph(body) => body,
            ExecutionFunctionRef::Host(never) => match *never {},
        };
        let graph = main.body().block_graph().as_view();
        let (instruction, arithmetic) = graph
            .blocks()
            .flat_map(|block| block.instructions())
            .find_map(|instruction| match instruction {
                ProfiledInstruction::IntegerRegion(arithmetic) => Some((instruction, arithmetic)),
                _ => None,
            })
            .unwrap();
        assert!(arithmetic.native);
        assert_eq!(arithmetic.inputs.as_ref(), &[IntLocalId(1)]);
        assert!(matches!(
            CallScalar::inspect(instruction),
            Some(CallScalar::Region { outputs, .. }) if outputs.as_slice() == [IntLocalId(2)]
        ));
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Int(11.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn each_supported_capture_projection_preserves_its_family_and_both_local_ids() {
        for capture in [
            FunctionCapture::Int {
                target: IntLocalId(2),
                source: IntLocalId(7),
            },
            FunctionCapture::Bool {
                target: BoolLocalId(3),
                source: BoolLocalId(8),
            },
            FunctionCapture::IntFunction {
                target: IntFunctionLocalId(4),
                source: IntFunctionLocalId(9),
            },
            FunctionCapture::BoolFunction {
                target: BoolFunctionLocalId(5),
                source: BoolFunctionLocalId(10),
            },
        ] {
            assert!(Capture::inspect(&capture).unwrap().canonical() == capture);
        }
    }

    #[test]
    fn a_dynamic_function_producer_remains_canonical_without_changing_its_supported_template() {
        let source = r#"
fn identity(value: Int) { value }
fn factory() { identity }
pub fn main() { let make = factory let calculate = make() calculate(7) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let main = match plan.program.functions.value_returns.int_functions[0].as_ref() {
            ExecutionFunctionRef::Graph(body) => body,
            ExecutionFunctionRef::Host(never) => match *never {},
        };
        let graph = main.body().block_graph().as_view();
        let block = graph.blocks().next().unwrap();
        let mut program = CallProgram::inspect(&plan.program.functions);
        let factory = program
            .functions
            .iter_mut()
            .find(|function| function.target == CallTarget::IntFunction(IntFunctionFunctionId(0)))
            .unwrap();
        let creation_count = factory.shape.creations.len();
        let invocation = block
            .instructions()
            .iter()
            .find(|instruction| {
                matches!(
                    instruction.value().map(|value| value.kind()),
                    Some(ProfiledInstructionKind::Function(function))
                        if matches!(function.kind(), FunctionInstructionKind::FunctionCall { .. })
                )
            })
            .unwrap();
        assert!(inspect_instruction(invocation, 0, &mut factory.shape).is_none());
        assert_eq!(factory.shape.creations.len(), creation_count);
        assert!(factory.shape.calls.is_empty());
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Int(7.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn a_string_parameter_keeps_each_function_producer_canonical() {
        for (source, expected) in [
            (
                r#"
fn identity(value: Int) { value }
fn factory(label: String) { fn(value: Int) { case label { "x" -> identity(value) _ -> 0 } } }
pub fn main() {
  let forward = identity
  let value = forward(7)
  let calculate = factory("x")
  calculate(value)
}
"#,
                crate::Value::Int(7.into()),
            ),
            (
                r#"
fn predicate(flag: Bool) { flag }
fn negative(value: Int) { value < 0 }
fn negate(flag: Bool) { let selected = predicate(flag) !selected }
fn factory(label: String) { fn(flag: Bool) { case label { "x" -> predicate(flag) _ -> False } } }
pub fn main() {
  let compare = negative
  let less = compare(-1)
  let calculate = negate
  let flag = calculate(less)
  let selected = factory("x")
  selected(flag)
}
"#,
                crate::Value::Bool(false),
            ),
        ] {
            let typed =
                crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
            let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
            let program = CallProgram::inspect(&plan.program.functions);
            assert!(!program.functions.is_empty());
            assert!(program.shapes().all(|(target, _)| !matches!(
                target,
                CallTarget::IntFunction(_) | CallTarget::BoolFunction(_)
            )));
            let mut echo = Vec::new();
            assert_eq!(crate::run_main(&plan, &mut echo).unwrap(), expected);
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn a_string_capture_declines_creation_before_publishing_a_call_contract() {
        let source = r#"
fn identity(value: Int) { value }
pub fn main() {
  let calculate = identity
  let value = calculate(7)
  let label = "x"
  let captured = fn(value) { case label { "x" -> value _ -> 0 } }
  captured(value)
}
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let main = match plan.program.functions.value_returns.int_functions[0].as_ref() {
            ExecutionFunctionRef::Graph(body) => body,
            ExecutionFunctionRef::Host(never) => match *never {},
        };
        let graph = main.body().block_graph().as_view();
        let closure = graph.blocks().flat_map(|block| block.instructions()).find(|instruction| matches!(instruction.value().map(|value| value.kind()), Some(ProfiledInstructionKind::Function(function)) if matches!(function.kind(), FunctionInstructionKind::Closure { .. }))).unwrap();
        let mut program = CallProgram::inspect(&plan.program.functions);
        let main = program
            .functions
            .iter_mut()
            .find(|function| function.target == CallTarget::Int(IntFunctionId(0)))
            .unwrap();
        let creation_count = main.shape.creations.len();
        assert!(inspect_instruction(closure, 0, &mut main.shape).is_none());
        assert_eq!(main.shape.creations.len(), creation_count);
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Int(7.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn scalar_calls_leave_a_string_argument_to_its_canonical_owner() {
        let source = r#"
fn identity(value: Int) { value }
fn labelled(label: String) { case label { "x" -> 7 _ -> 0 } }
pub fn main() { let calculate = identity let value = calculate(1) labelled("x") + value }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let main = match plan.program.functions.value_returns.int_functions[0].as_ref() {
            ExecutionFunctionRef::Graph(body) => body,
            ExecutionFunctionRef::Host(never) => match *never {},
        };
        let graph = main.body().block_graph().as_view();
        let call = graph.blocks().flat_map(|block| block.instructions()).find(|instruction| matches!(instruction.value().map(|value| value.kind()), Some(ProfiledInstructionKind::Int(IntInstruction::Call { args, .. })) if args.iter().any(|local| matches!(local, ParamLocal::String(_))))).unwrap();
        let mut program = CallProgram::inspect(&plan.program.functions);
        let main = program
            .functions
            .iter_mut()
            .find(|function| function.target == CallTarget::Int(IntFunctionId(0)))
            .unwrap();
        let call_count = main.shape.calls.len();
        assert!(inspect_instruction(call, 0, &mut main.shape).is_none());
        assert_eq!(main.shape.calls.len(), call_count);
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Int(8.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn custom_list_length_values_remain_canonical_in_scalar_calls() {
        let list = ListLocal::Custom {
            local: CustomListLocalId(7),
            type_id: CustomListTypeId {
                list_type: ListTypeId(11),
                item_type: CustomTypeId(4),
            },
        };
        for (test, at_least) in [
            (
                BoolTest::ListLengthEquals {
                    value: list.clone(),
                    length: 3,
                },
                false,
            ),
            (
                BoolTest::ListLengthAtLeast {
                    value: list.clone(),
                    length: 3,
                },
                true,
            ),
        ] {
            let instruction = ProfiledInstruction::<Infallible>::new(
                ParamSlot::new(ParamLocal::Bool(BoolLocalId(2)), ValueShapeId(3)),
                ProfiledInstructionKind::Bool(BoolInstruction::Test(test)),
            );
            let compiled = CompiledInstruction::inspect(&instruction, KernelKind::Numeric).unwrap();
            assert!(matches!(
                compiled,
                CompiledInstruction::Boolean(
                    BoolLocalId(2),
                    CompiledBoolean::Test(CompiledTest::CustomListLength {
                        list: CustomListLocalId(7),
                        length: 3,
                        at_least: projected_at_least,
                    }),
                ) if projected_at_least == at_least
            ));
            assert!(CallScalar::inspect(&instruction).is_none());
        }
    }

    #[test]
    fn scalar_boolean_projection_leaves_list_comparison_to_its_canonical_owner() {
        let source = r#"
fn identity(value: Int) { value }
fn same(left: List(Int), right: List(Int)) { left == right }
pub fn main() { let calculate = identity let value = calculate(1) same([value], [2]) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let same = match plan.program.functions.value_returns.bool_functions[1].as_ref() {
            ExecutionFunctionRef::Graph(body) => body,
            ExecutionFunctionRef::Host(never) => match *never {},
        };
        let graph = same.body().block_graph().as_view();
        let instruction = &graph.blocks().next().unwrap().instructions()[0];
        assert!(CallScalar::inspect(instruction).is_none());
        let mut program = CallProgram::inspect(&plan.program.functions);
        let main = program
            .functions
            .iter_mut()
            .find(|function| function.target == CallTarget::Bool(BoolFunctionId(0)))
            .unwrap();
        let call_count = main.shape.calls.len();
        assert!(inspect_instruction(instruction, 0, &mut main.shape).is_none());
        assert_eq!(main.shape.calls.len(), call_count);
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Bool(false)
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn scalar_call_projection_leaves_string_captures_and_other_compiled_tests_canonical() {
        let capture = FunctionCapture::String {
            target: StringLocalId(0),
            source: StringLocalId(1),
        };
        assert!(Capture::inspect(&capture).is_none());
        assert!(!supported_local(&ParamLocal::String(StringLocalId(1))));
        for test in [
            CompiledTest::IntList(IntListTest::Length {
                list: IntListLocalId(0),
                length: 2,
                at_least: false,
            }),
            CompiledTest::String(StringTest::Prefix {
                value: StringLocalId(0),
                prefix: "prefix",
            }),
            CompiledTest::BoolEqual {
                left: BoolLocalId(0),
                right: BoolLocalId(1),
                negate: false,
            },
            CompiledTest::CustomListLength {
                list: CustomListLocalId(0),
                length: 2,
                at_least: true,
            },
        ] {
            assert!(CallTest::from_compiled(test).is_none());
        }
    }

    #[test]
    fn closure_and_nested_call_views_record_exact_canonical_locals_and_destinations() {
        let source = r#"
fn identity(value: Int) { value }
fn make(offset: Int) { fn(value) { identity(value) + offset } }
pub fn main() {
  let calculate = make(7)
  calculate(3) + 1
}
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let program = CallProgram::inspect(&plan.program.functions);
        let main = program
            .shapes()
            .find(|(target, _)| *target == CallTarget::Int(IntFunctionId(0)))
            .unwrap()
            .1;
        assert!(main.root);
        assert_eq!(main.entry(), 0);
        assert_eq!(main.parameter_count, 0);
        assert_eq!(
            main.checkpoints,
            (0..=5)
                .zip([0, 1, 1, 2, 3, 4])
                .map(|(instruction, ints)| CompiledCheckpoint {
                    block: BlockId(0),
                    instruction,
                    ints,
                    bools: 0,
                    bit_arrays: 0,
                    int_lists: 0,
                    strings: 0,
                    customs: 0,
                    custom_lists: 0,
                    int_functions: usize::from(instruction >= 2),
                    bool_functions: 0,
                })
                .collect::<Vec<_>>()
        );
        let calls = main.call_contracts();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].point, 1);
        assert!(
            calls[0].target
                == CallContractTarget::Static(CallTarget::IntFunction(IntFunctionFunctionId(0)))
        );
        assert_eq!(calls[0].args.as_ref(), &[ParamLocal::Int(IntLocalId(0))]);
        assert_eq!(
            calls[0].output,
            ParamLocal::IntFunction {
                local: IntFunctionLocalId(0),
                type_: FunctionType::new(vec![ValueType::Int], ValueType::Int),
            }
        );
        assert_eq!(calls[1].point, 3);
        assert!(calls[1].target == CallContractTarget::IntValue(IntFunctionLocalId(0)));
        assert_eq!(calls[1].output, ParamLocal::Int(IntLocalId(2)));
        assert_eq!(calls[1].args.as_ref(), &[ParamLocal::Int(IntLocalId(1))]);
        assert_eq!(main.return_contracts()[0].point, 5);
        assert_eq!(
            main.return_contracts()[0].value,
            ParamLocal::Int(IntLocalId(3))
        );
        let maker = program
            .shapes()
            .find(|(target, _)| *target == CallTarget::IntFunction(IntFunctionFunctionId(0)))
            .unwrap()
            .1;
        assert!(maker.root);
        assert!(
            maker.creation_contracts()[0].captures.as_ref()
                == [FunctionCapture::Int {
                    target: IntLocalId(1),
                    source: IntLocalId(0)
                }]
        );
        assert!(!maker.creation_contracts()[0].reference);
        let anonymous = program
            .shapes()
            .find(|(_, shape)| {
                shape.parameter_count == 1
                    && shape.locals[0]
                        == [CallLocal::Int(IntLocalId(0)), CallLocal::Int(IntLocalId(1))]
            })
            .unwrap()
            .1;
        assert!(anonymous.root);
        assert!(
            anonymous.call_contracts()[0].target
                == CallContractTarget::Static(CallTarget::Int(IntFunctionId(2)))
        );
        assert_eq!(
            anonymous.call_contracts()[0].output,
            ParamLocal::Int(IntLocalId(2))
        );
        assert_eq!(
            anonymous.return_contracts()[0].value,
            ParamLocal::Int(IntLocalId(3))
        );
        let identity = program
            .shapes()
            .find(|(target, _)| *target == CallTarget::Int(IntFunctionId(2)))
            .unwrap()
            .1;
        assert!(!identity.root);
        assert_eq!(identity.points.len(), 1);
        assert_eq!(identity.returns[0].point, 0);
        assert_eq!(identity.returns[0].value, CallLocal::Int(IntLocalId(0)));
    }

    #[test]
    fn unsupported_dynamic_entry_keeps_its_shared_body_without_selecting_a_root() {
        let source = r#"
fn apply(calculate: fn(Int) -> Int, value: Int) { calculate(value) }
pub fn main() {
  let calculate = fn(value) {
    case [value] {
      [first, ..] -> first
      [] -> 0
    }
  }
  apply(calculate, 7)
}
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let program = CallProgram::inspect(&plan.program.functions);
        let main = program
            .functions
            .iter()
            .find(|function| function.target == CallTarget::Int(IntFunctionId(0)))
            .unwrap();
        assert!(main.shape.root);
        let apply = program
            .functions
            .iter()
            .find(|function| function.target == main.shape.tails[0].target)
            .unwrap();
        assert!(!apply.shape.root);
        assert_eq!(apply.shape.parameter_count, 2);
        assert_eq!(apply.shape.calls.len(), 1);
        let call = &apply.shape.calls[0];
        assert!(call.target == CallContractTarget::IntValue(IntFunctionLocalId(0)));
        assert_eq!(call.args, [CallLocal::Int(IntLocalId(0))]);
        assert!(
            !program
                .functions
                .iter()
                .any(|callee| callee.accepts_call(call))
        );
    }

    #[test]
    fn unsupported_suffix_keeps_its_exact_prefix_and_scalar_only_programs_need_no_call_sidecar() {
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            "pub fn main() { let add = fn(value) { value + 1 } echo add(7) add(8) }",
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let program = CallProgram::inspect(&plan.program.functions);
        let main = program
            .shapes()
            .find(|(target, _)| *target == CallTarget::Int(IntFunctionId(0)))
            .unwrap()
            .1;
        assert!(main.root);
        // Echo ends this supported prefix. Later canonical blocks may still
        // have their own supported entries; they do not move this boundary.
        assert_eq!(main.checkpoints[3].block, BlockId(0));
        assert_eq!(main.checkpoints[3].instruction, 3);
        assert_eq!(main.checkpoints[3].ints, 2);
        let typed =
            crate::compile_typed_module("example", "src/example.gleam", "pub fn main() { 7 }")
                .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        assert_eq!(
            CallProgram::inspect(&plan.program.functions)
                .shapes()
                .count(),
            0
        );
    }
}
